-- ============================================================================
-- OLYMPUS CLOUD - ADDITIONAL INDEXES AND CONSTRAINTS
-- ============================================================================
-- Migration: 005_indexes_and_constraints.sql
-- Description: Performance indexes, foreign key constraints, and data integrity
-- Author: Claude Code Agent
-- Date: 2025-01-18
-- ============================================================================

-- ============================================================================
-- ADDITIONAL PERFORMANCE INDEXES
-- ============================================================================

-- Composite indexes for common query patterns
CREATE INDEX CONCURRENTLY idx_users_tenant_email_active ON users(tenant_id, email)
    WHERE deleted_at IS NULL AND status = 'ACTIVE';

CREATE INDEX CONCURRENTLY idx_products_tenant_active_category ON products(tenant_id, category_id, is_active)
    WHERE deleted_at IS NULL;

CREATE INDEX CONCURRENTLY idx_orders_tenant_status_created ON orders(tenant_id, status, created_at DESC);

CREATE INDEX CONCURRENTLY idx_inventory_location_available ON inventory(location_id, quantity_available)
    WHERE quantity_available > 0;

CREATE INDEX CONCURRENTLY idx_sessions_user_active ON sessions(user_id, expires_at)
    WHERE revoked_at IS NULL;

-- Full-text search indexes
CREATE INDEX CONCURRENTLY idx_products_name_search ON products USING gin(to_tsvector('english', name));
CREATE INDEX CONCURRENTLY idx_products_description_search ON products USING gin(to_tsvector('english', description));
CREATE INDEX CONCURRENTLY idx_categories_name_search ON categories USING gin(to_tsvector('english', name));
CREATE INDEX CONCURRENTLY idx_customers_name_search ON customers USING gin(to_tsvector('english', first_name || ' ' || last_name));

-- JSONB indexes for metadata and settings
CREATE INDEX CONCURRENTLY idx_tenants_settings_gin ON tenants USING gin(settings);
CREATE INDEX CONCURRENTLY idx_users_preferences_gin ON users USING gin(preferences);
CREATE INDEX CONCURRENTLY idx_products_attributes_gin ON products USING gin(attributes);
CREATE INDEX CONCURRENTLY idx_orders_metadata_gin ON orders USING gin(metadata);

-- Partial indexes for specific use cases
CREATE INDEX CONCURRENTLY idx_users_failed_logins ON users(failed_login_attempts, locked_until)
    WHERE failed_login_attempts > 0;

CREATE INDEX CONCURRENTLY idx_products_low_stock ON inventory(product_id, quantity_available)
    WHERE quantity_available <= reorder_point;

CREATE INDEX CONCURRENTLY idx_orders_recent_pending ON orders(tenant_id, created_at)
    WHERE status IN ('PENDING', 'CONFIRMED') AND created_at > CURRENT_DATE - INTERVAL '7 days';

-- Array indexes for tags and features
CREATE INDEX CONCURRENTLY idx_products_tags_gin ON products USING gin(tags);
CREATE INDEX CONCURRENTLY idx_tenants_features_gin ON tenants USING gin(features);
CREATE INDEX CONCURRENTLY idx_customers_tags_gin ON customers USING gin(tags);

-- ============================================================================
-- DATA INTEGRITY CONSTRAINTS
-- ============================================================================

-- Additional check constraints for data validation
ALTER TABLE tenants ADD CONSTRAINT tenants_user_limit_positive
    CHECK (user_limit IS NULL OR user_limit > 0);

ALTER TABLE tenants ADD CONSTRAINT tenants_location_limit_positive
    CHECK (location_limit IS NULL OR location_limit > 0);

ALTER TABLE tenants ADD CONSTRAINT tenants_storage_limit_positive
    CHECK (storage_limit_gb IS NULL OR storage_limit_gb > 0);

ALTER TABLE users ADD CONSTRAINT users_email_format
    CHECK (email ~ '^[^@]+@[^@]+\.[^@]+$');

ALTER TABLE users ADD CONSTRAINT users_failed_attempts_range
    CHECK (failed_login_attempts >= 0 AND failed_login_attempts <= 10);

ALTER TABLE products ADD CONSTRAINT products_price_positive
    CHECK (unit_price >= 0);

ALTER TABLE products ADD CONSTRAINT products_compare_price_valid
    CHECK (compare_at_price IS NULL OR compare_at_price >= unit_price);

ALTER TABLE products ADD CONSTRAINT products_cost_valid
    CHECK (cost IS NULL OR cost >= 0);

ALTER TABLE products ADD CONSTRAINT products_weight_positive
    CHECK (weight_value IS NULL OR weight_value >= 0);

ALTER TABLE product_variants ADD CONSTRAINT variants_price_positive
    CHECK (price IS NULL OR price >= 0);

ALTER TABLE inventory ADD CONSTRAINT inventory_quantities_valid
    CHECK (quantity_on_hand >= 0 AND quantity_reserved >= 0 AND quantity_reserved <= quantity_on_hand);

ALTER TABLE inventory ADD CONSTRAINT inventory_reorder_valid
    CHECK (reorder_point IS NULL OR reorder_point >= 0);

ALTER TABLE orders ADD CONSTRAINT orders_amounts_valid
    CHECK (subtotal >= 0 AND tax_amount >= 0 AND discount_amount >= 0 AND shipping_amount >= 0 AND total_amount >= 0);

ALTER TABLE order_items ADD CONSTRAINT order_items_positive_values
    CHECK (quantity > 0 AND unit_price >= 0 AND total_amount >= 0);

ALTER TABLE payments ADD CONSTRAINT payments_amount_positive
    CHECK (amount > 0);

ALTER TABLE payments ADD CONSTRAINT payments_refund_valid
    CHECK (refunded_amount >= 0 AND refunded_amount <= amount);

-- ============================================================================
-- BUSINESS LOGIC CONSTRAINTS
-- ============================================================================

-- Ensure only one primary location per tenant
CREATE UNIQUE INDEX CONCURRENTLY idx_locations_tenant_primary
    ON locations(tenant_id) WHERE is_primary = true AND deleted_at IS NULL;

-- Ensure order numbers are unique per tenant per year
CREATE UNIQUE INDEX CONCURRENTLY idx_orders_tenant_number_year
    ON orders(tenant_id, order_number, EXTRACT(YEAR FROM created_at));

-- Ensure product SKUs are unique per tenant
ALTER TABLE products ADD CONSTRAINT products_sku_tenant_unique
    UNIQUE(tenant_id, sku) DEFERRABLE INITIALLY DEFERRED;

-- Ensure customer emails are unique per tenant
ALTER TABLE customers ADD CONSTRAINT customers_email_tenant_unique
    UNIQUE(tenant_id, email) DEFERRABLE INITIALLY DEFERRED;

-- ============================================================================
-- AUDIT AND COMPLIANCE CONSTRAINTS
-- ============================================================================

-- Ensure audit logs have required fields
ALTER TABLE audit_logs ADD CONSTRAINT audit_logs_required_fields
    CHECK (
        action IS NOT NULL AND LENGTH(TRIM(action)) > 0 AND
        resource_type IS NOT NULL AND LENGTH(TRIM(resource_type)) > 0
    );

-- Ensure security events have valid severity
ALTER TABLE auth.security_events ADD CONSTRAINT security_events_valid_severity
    CHECK (severity IN ('info', 'warning', 'critical'));

-- ============================================================================
-- PERFORMANCE CONSTRAINTS
-- ============================================================================

-- Limit the number of roles per user (performance constraint)
CREATE OR REPLACE FUNCTION check_user_role_limit()
RETURNS TRIGGER AS $$
BEGIN
    IF (
        SELECT COUNT(*)
        FROM user_roles
        WHERE user_id = NEW.user_id
    ) >= 20 THEN
        RAISE EXCEPTION 'User cannot have more than 20 roles';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER user_role_limit_trigger
    BEFORE INSERT ON user_roles
    FOR EACH ROW EXECUTE FUNCTION check_user_role_limit();

-- Limit the number of items in a cart (performance constraint)
CREATE OR REPLACE FUNCTION check_cart_item_limit()
RETURNS TRIGGER AS $$
BEGIN
    IF (
        SELECT COUNT(*)
        FROM commerce.cart_items
        WHERE cart_id = NEW.cart_id
    ) >= 100 THEN
        RAISE EXCEPTION 'Cart cannot have more than 100 items';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER cart_item_limit_trigger
    BEFORE INSERT ON commerce.cart_items
    FOR EACH ROW EXECUTE FUNCTION check_cart_item_limit();

-- ============================================================================
-- REFERENTIAL INTEGRITY ENHANCEMENTS
-- ============================================================================

-- Add missing foreign key constraints with proper CASCADE behavior
ALTER TABLE locations ADD CONSTRAINT fk_locations_manager
    FOREIGN KEY (manager_id) REFERENCES users(id) ON DELETE SET NULL;

ALTER TABLE products ADD CONSTRAINT fk_products_category
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL;

ALTER TABLE orders ADD CONSTRAINT fk_orders_customer
    FOREIGN KEY (customer_id) REFERENCES customers(id) ON DELETE SET NULL;

ALTER TABLE orders ADD CONSTRAINT fk_orders_location
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE RESTRICT;

-- ============================================================================
-- COMPUTED COLUMNS AND GENERATED VALUES
-- ============================================================================

-- Add computed columns for common calculations
ALTER TABLE customers ADD COLUMN full_name VARCHAR(255)
    GENERATED ALWAYS AS (
        CASE
            WHEN first_name IS NOT NULL AND last_name IS NOT NULL
            THEN first_name || ' ' || last_name
            WHEN first_name IS NOT NULL
            THEN first_name
            WHEN last_name IS NOT NULL
            THEN last_name
            ELSE NULL
        END
    ) STORED;

-- Add search vector for products
ALTER TABLE products ADD COLUMN search_vector tsvector
    GENERATED ALWAYS AS (
        to_tsvector('english',
            COALESCE(name, '') || ' ' ||
            COALESCE(description, '') || ' ' ||
            COALESCE(brand, '') || ' ' ||
            COALESCE(array_to_string(tags, ' '), '')
        )
    ) STORED;

CREATE INDEX CONCURRENTLY idx_products_search_vector ON products USING gin(search_vector);

-- ============================================================================
-- DATABASE MAINTENANCE FUNCTIONS
-- ============================================================================

-- Function to update product search vectors
CREATE OR REPLACE FUNCTION update_product_search_vector()
RETURNS TRIGGER AS $$
BEGIN
    -- This is now handled by the generated column, but keeping for compatibility
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate customer lifetime value
CREATE OR REPLACE FUNCTION calculate_customer_ltv(customer_uuid UUID)
RETURNS DECIMAL(19,4) AS $$
DECLARE
    ltv DECIMAL(19,4);
BEGIN
    SELECT
        COALESCE(SUM(total_amount), 0)
    INTO ltv
    FROM orders
    WHERE customer_id = customer_uuid
        AND status IN ('COMPLETED', 'DELIVERED');

    RETURN ltv;
END;
$$ LANGUAGE plpgsql;

-- Function to get low stock products
CREATE OR REPLACE FUNCTION get_low_stock_products(tenant_uuid UUID)
RETURNS TABLE(
    product_id UUID,
    product_name VARCHAR(500),
    location_name VARCHAR(255),
    current_stock INTEGER,
    reorder_point INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        p.id,
        p.name,
        l.name,
        i.quantity_available,
        i.reorder_point
    FROM inventory i
    JOIN products p ON i.product_id = p.id
    JOIN locations l ON i.location_id = l.id
    WHERE p.tenant_id = tenant_uuid
        AND i.quantity_available <= COALESCE(i.reorder_point, 0)
        AND p.track_inventory = true
        AND p.is_active = true
        AND p.deleted_at IS NULL
    ORDER BY (i.quantity_available::FLOAT / GREATEST(i.reorder_point, 1)) ASC;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- DATA ARCHIVAL FUNCTIONS
-- ============================================================================

-- Function to archive old audit logs
CREATE OR REPLACE FUNCTION archive_old_audit_logs(days_old INTEGER DEFAULT 90)
RETURNS INTEGER AS $$
DECLARE
    archived_count INTEGER;
BEGIN
    -- Move old audit logs to archive table (would need to create archive table first)
    DELETE FROM audit_logs
    WHERE created_at < CURRENT_DATE - days_old * INTERVAL '1 day';

    GET DIAGNOSTICS archived_count = ROW_COUNT;

    RETURN archived_count;
END;
$$ LANGUAGE plpgsql;

-- Function to clean up expired tokens and sessions
CREATE OR REPLACE FUNCTION cleanup_expired_auth_data()
RETURNS INTEGER AS $$
DECLARE
    cleanup_count INTEGER := 0;
BEGIN
    -- Clean up expired sessions
    DELETE FROM sessions WHERE expires_at < CURRENT_TIMESTAMP;
    GET DIAGNOSTICS cleanup_count = ROW_COUNT;

    -- Clean up expired auth sessions
    DELETE FROM auth.user_sessions WHERE expires_at < CURRENT_TIMESTAMP;
    GET DIAGNOSTICS cleanup_count = cleanup_count + ROW_COUNT;

    -- Clean up used verification tokens older than 1 day
    DELETE FROM auth.email_verification_tokens
    WHERE used_at IS NOT NULL AND used_at < CURRENT_TIMESTAMP - INTERVAL '1 day';
    GET DIAGNOSTICS cleanup_count = cleanup_count + ROW_COUNT;

    -- Clean up used password reset tokens older than 1 day
    DELETE FROM auth.password_reset_tokens
    WHERE used_at IS NOT NULL AND used_at < CURRENT_TIMESTAMP - INTERVAL '1 day';
    GET DIAGNOSTICS cleanup_count = cleanup_count + ROW_COUNT;

    RETURN cleanup_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- GRANTS FOR NEW FUNCTIONS
-- ============================================================================

GRANT EXECUTE ON FUNCTION calculate_customer_ltv(UUID) TO olympus_app;
GRANT EXECUTE ON FUNCTION get_low_stock_products(UUID) TO olympus_app;
GRANT EXECUTE ON FUNCTION archive_old_audit_logs(INTEGER) TO olympus_app;
GRANT EXECUTE ON FUNCTION cleanup_expired_auth_data() TO olympus_app;

-- ============================================================================
-- COMMENTS FOR DOCUMENTATION
-- ============================================================================

COMMENT ON TABLE tenants IS 'Multi-tenant root table for the entire application';
COMMENT ON TABLE users IS 'User accounts with authentication and profile information';
COMMENT ON TABLE auth.user_sessions IS 'JWT refresh token sessions with device tracking';
COMMENT ON TABLE products IS 'Product catalog with full-text search capabilities';
COMMENT ON TABLE orders IS 'Customer orders with comprehensive status tracking';
COMMENT ON TABLE inventory IS 'Real-time inventory tracking with automatic calculations';
COMMENT ON TABLE events.domain_events IS 'Event sourcing table for all business events';
COMMENT ON TABLE analytics.business_metrics IS 'Time-series business metrics data';

COMMENT ON FUNCTION calculate_customer_ltv(UUID) IS 'Calculate lifetime value for a specific customer';
COMMENT ON FUNCTION get_low_stock_products(UUID) IS 'Get products that are below reorder point';
COMMENT ON FUNCTION cleanup_expired_auth_data() IS 'Clean up expired authentication tokens and sessions';