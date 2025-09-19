-- ============================================================================
-- OLYMPUS CLOUD - COMMERCE SYSTEM ENHANCEMENTS
-- ============================================================================
-- Migration: 003_commerce_system.sql
-- Description: Enhanced commerce tables for orders, payments, and fulfillment
-- Author: Claude Code Agent
-- Date: 2025-01-18
-- ============================================================================

-- Additional commerce-specific types
CREATE TYPE commerce.discount_type AS ENUM (
    'percentage',
    'fixed_amount',
    'buy_x_get_y',
    'free_shipping'
);

CREATE TYPE commerce.shipping_status AS ENUM (
    'pending',
    'ready_to_ship',
    'shipped',
    'in_transit',
    'delivered',
    'failed_delivery',
    'returned'
);

CREATE TYPE commerce.refund_reason AS ENUM (
    'customer_request',
    'damaged_product',
    'wrong_item',
    'quality_issue',
    'cancelled_order',
    'fraud',
    'other'
);

-- ============================================================================
-- ENHANCED COMMERCE TABLES
-- ============================================================================

-- Discount codes and promotions
CREATE TABLE commerce.discount_codes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    code VARCHAR(100) NOT NULL,
    description TEXT,
    discount_type commerce.discount_type NOT NULL,
    value DECIMAL(19,4) NOT NULL,
    minimum_order_amount DECIMAL(19,4),
    maximum_discount_amount DECIMAL(19,4),
    usage_limit INTEGER,
    usage_limit_per_customer INTEGER,
    used_count INTEGER NOT NULL DEFAULT 0,
    starts_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    applicable_products UUID[] DEFAULT '{}', -- Array of product IDs
    applicable_categories UUID[] DEFAULT '{}', -- Array of category IDs
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT unique_discount_code UNIQUE(tenant_id, code),
    CONSTRAINT valid_discount_value CHECK (value > 0),
    CONSTRAINT valid_usage_limits CHECK (usage_limit IS NULL OR usage_limit > 0),
    CONSTRAINT valid_date_range CHECK (starts_at IS NULL OR ends_at IS NULL OR starts_at < ends_at)
);

-- Order discounts applied
CREATE TABLE commerce.order_discounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    discount_code_id UUID REFERENCES commerce.discount_codes(id),
    discount_type commerce.discount_type NOT NULL,
    description VARCHAR(255) NOT NULL,
    value DECIMAL(19,4) NOT NULL,
    amount DECIMAL(19,4) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Shipping methods and rates
CREATE TABLE commerce.shipping_methods (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    carrier VARCHAR(100),
    estimated_delivery_days INTEGER,
    base_rate DECIMAL(19,4) NOT NULL DEFAULT 0,
    rate_per_weight DECIMAL(19,4) DEFAULT 0,
    rate_per_distance DECIMAL(19,4) DEFAULT 0,
    free_shipping_threshold DECIMAL(19,4),
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Order fulfillments
CREATE TABLE commerce.order_fulfillments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    location_id UUID NOT NULL REFERENCES locations(id),
    shipping_method_id UUID REFERENCES commerce.shipping_methods(id),
    tracking_number VARCHAR(255),
    carrier VARCHAR(100),
    shipping_status commerce.shipping_status NOT NULL DEFAULT 'pending',
    shipped_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    estimated_delivery_at TIMESTAMPTZ,
    shipping_address JSONB NOT NULL,
    notes TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Fulfillment items (which items are in each shipment)
CREATE TABLE commerce.fulfillment_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fulfillment_id UUID NOT NULL REFERENCES commerce.order_fulfillments(id) ON DELETE CASCADE,
    order_item_id UUID NOT NULL REFERENCES order_items(id),
    quantity INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT positive_quantity CHECK (quantity > 0)
);

-- Returns and refunds
CREATE TABLE commerce.order_returns (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    return_number VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'requested',
    reason commerce.refund_reason NOT NULL,
    customer_note TEXT,
    admin_note TEXT,
    return_address JSONB,
    refund_amount DECIMAL(19,4) NOT NULL DEFAULT 0,
    restocking_fee DECIMAL(19,4) DEFAULT 0,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT unique_return_number UNIQUE(return_number)
);

-- Return items
CREATE TABLE commerce.return_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    return_id UUID NOT NULL REFERENCES commerce.order_returns(id) ON DELETE CASCADE,
    order_item_id UUID NOT NULL REFERENCES order_items(id),
    quantity INTEGER NOT NULL,
    reason commerce.refund_reason NOT NULL,
    condition VARCHAR(100), -- new, used, damaged, etc.
    refund_amount DECIMAL(19,4) NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT positive_return_quantity CHECK (quantity > 0)
);

-- Shopping carts for draft orders
CREATE TABLE commerce.shopping_carts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id) ON DELETE CASCADE,
    session_id VARCHAR(255), -- For guest carts
    currency JSONB NOT NULL DEFAULT '{"code": "USD", "symbol": "$"}',
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (CURRENT_TIMESTAMP + INTERVAL '7 days'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Either customer_id or session_id must be present
    CONSTRAINT cart_ownership CHECK (customer_id IS NOT NULL OR session_id IS NOT NULL)
);

-- Shopping cart items
CREATE TABLE commerce.cart_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    cart_id UUID NOT NULL REFERENCES commerce.shopping_carts(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    variant_id UUID REFERENCES product_variants(id),
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(19,4) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT positive_cart_quantity CHECK (quantity > 0),
    CONSTRAINT unique_cart_product UNIQUE(cart_id, product_id, variant_id)
);

-- Product reviews and ratings
CREATE TABLE commerce.product_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders(id), -- Optional link to purchase
    rating INTEGER NOT NULL,
    title VARCHAR(255),
    review_text TEXT,
    is_verified_purchase BOOLEAN NOT NULL DEFAULT false,
    is_approved BOOLEAN NOT NULL DEFAULT false,
    approved_at TIMESTAMPTZ,
    approved_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT valid_rating CHECK (rating >= 1 AND rating <= 5),
    CONSTRAINT unique_customer_product_review UNIQUE(product_id, customer_id)
);

-- Wishlist functionality
CREATE TABLE commerce.wishlists (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL DEFAULT 'My Wishlist',
    is_public BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Wishlist items
CREATE TABLE commerce.wishlist_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wishlist_id UUID NOT NULL REFERENCES commerce.wishlists(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    added_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT unique_wishlist_product UNIQUE(wishlist_id, product_id, variant_id)
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Discount codes indexes
CREATE INDEX idx_discount_codes_tenant ON commerce.discount_codes(tenant_id);
CREATE INDEX idx_discount_codes_code ON commerce.discount_codes(tenant_id, code);
CREATE INDEX idx_discount_codes_active ON commerce.discount_codes(is_active, starts_at, ends_at);

-- Order discounts indexes
CREATE INDEX idx_order_discounts_order ON commerce.order_discounts(order_id);
CREATE INDEX idx_order_discounts_code ON commerce.order_discounts(discount_code_id);

-- Shipping methods indexes
CREATE INDEX idx_shipping_methods_tenant ON commerce.shipping_methods(tenant_id);
CREATE INDEX idx_shipping_methods_active ON commerce.shipping_methods(is_active);

-- Order fulfillments indexes
CREATE INDEX idx_fulfillments_order ON commerce.order_fulfillments(order_id);
CREATE INDEX idx_fulfillments_location ON commerce.order_fulfillments(location_id);
CREATE INDEX idx_fulfillments_status ON commerce.order_fulfillments(shipping_status);
CREATE INDEX idx_fulfillments_tracking ON commerce.order_fulfillments(tracking_number);

-- Fulfillment items indexes
CREATE INDEX idx_fulfillment_items_fulfillment ON commerce.fulfillment_items(fulfillment_id);
CREATE INDEX idx_fulfillment_items_order_item ON commerce.fulfillment_items(order_item_id);

-- Order returns indexes
CREATE INDEX idx_returns_order ON commerce.order_returns(order_id);
CREATE INDEX idx_returns_number ON commerce.order_returns(return_number);
CREATE INDEX idx_returns_status ON commerce.order_returns(status);

-- Return items indexes
CREATE INDEX idx_return_items_return ON commerce.return_items(return_id);
CREATE INDEX idx_return_items_order_item ON commerce.return_items(order_item_id);

-- Shopping carts indexes
CREATE INDEX idx_carts_tenant ON commerce.shopping_carts(tenant_id);
CREATE INDEX idx_carts_customer ON commerce.shopping_carts(customer_id);
CREATE INDEX idx_carts_session ON commerce.shopping_carts(session_id);
CREATE INDEX idx_carts_expires ON commerce.shopping_carts(expires_at);

-- Cart items indexes
CREATE INDEX idx_cart_items_cart ON commerce.cart_items(cart_id);
CREATE INDEX idx_cart_items_product ON commerce.cart_items(product_id);

-- Product reviews indexes
CREATE INDEX idx_reviews_product ON commerce.product_reviews(product_id);
CREATE INDEX idx_reviews_customer ON commerce.product_reviews(customer_id);
CREATE INDEX idx_reviews_approved ON commerce.product_reviews(is_approved, created_at);
CREATE INDEX idx_reviews_rating ON commerce.product_reviews(product_id, rating);

-- Wishlist indexes
CREATE INDEX idx_wishlists_customer ON commerce.wishlists(customer_id);
CREATE INDEX idx_wishlist_items_wishlist ON commerce.wishlist_items(wishlist_id);
CREATE INDEX idx_wishlist_items_product ON commerce.wishlist_items(product_id);

-- ============================================================================
-- ROW LEVEL SECURITY POLICIES
-- ============================================================================

-- Enable RLS on all commerce tables
ALTER TABLE commerce.discount_codes ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.order_discounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.shipping_methods ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.order_fulfillments ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.fulfillment_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.order_returns ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.return_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.shopping_carts ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.cart_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.product_reviews ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.wishlists ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.wishlist_items ENABLE ROW LEVEL SECURITY;

-- Tenant-based policies for most tables
CREATE POLICY discount_codes_tenant_policy ON commerce.discount_codes
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY shipping_methods_tenant_policy ON commerce.shipping_methods
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY shopping_carts_tenant_policy ON commerce.shopping_carts
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

-- Order-based policies (inherit tenant through order)
CREATE POLICY order_discounts_tenant_policy ON commerce.order_discounts
    FOR ALL USING (
        order_id IN (
            SELECT id FROM orders
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

CREATE POLICY order_fulfillments_tenant_policy ON commerce.order_fulfillments
    FOR ALL USING (
        order_id IN (
            SELECT id FROM orders
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

CREATE POLICY order_returns_tenant_policy ON commerce.order_returns
    FOR ALL USING (
        order_id IN (
            SELECT id FROM orders
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Related item policies
CREATE POLICY fulfillment_items_tenant_policy ON commerce.fulfillment_items
    FOR ALL USING (
        fulfillment_id IN (
            SELECT id FROM commerce.order_fulfillments
            WHERE order_id IN (
                SELECT id FROM orders
                WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
            )
        )
    );

CREATE POLICY return_items_tenant_policy ON commerce.return_items
    FOR ALL USING (
        return_id IN (
            SELECT id FROM commerce.order_returns
            WHERE order_id IN (
                SELECT id FROM orders
                WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
            )
        )
    );

CREATE POLICY cart_items_tenant_policy ON commerce.cart_items
    FOR ALL USING (
        cart_id IN (
            SELECT id FROM commerce.shopping_carts
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Product-based policies (inherit tenant through product)
CREATE POLICY product_reviews_tenant_policy ON commerce.product_reviews
    FOR ALL USING (
        product_id IN (
            SELECT id FROM products
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Customer-based policies (inherit tenant through customer)
CREATE POLICY wishlists_tenant_policy ON commerce.wishlists
    FOR ALL USING (
        customer_id IN (
            SELECT id FROM customers
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

CREATE POLICY wishlist_items_tenant_policy ON commerce.wishlist_items
    FOR ALL USING (
        wishlist_id IN (
            SELECT id FROM commerce.wishlists
            WHERE customer_id IN (
                SELECT id FROM customers
                WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
            )
        )
    );

-- ============================================================================
-- UPDATED_AT TRIGGERS
-- ============================================================================

-- Apply updated_at triggers to tables that need them
CREATE TRIGGER update_discount_codes_updated_at
    BEFORE UPDATE ON commerce.discount_codes
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_shipping_methods_updated_at
    BEFORE UPDATE ON commerce.shipping_methods
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_order_fulfillments_updated_at
    BEFORE UPDATE ON commerce.order_fulfillments
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_order_returns_updated_at
    BEFORE UPDATE ON commerce.order_returns
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_shopping_carts_updated_at
    BEFORE UPDATE ON commerce.shopping_carts
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_cart_items_updated_at
    BEFORE UPDATE ON commerce.cart_items
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_product_reviews_updated_at
    BEFORE UPDATE ON commerce.product_reviews
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_wishlists_updated_at
    BEFORE UPDATE ON commerce.wishlists
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

-- ============================================================================
-- GRANTS AND PERMISSIONS
-- ============================================================================

-- Grant permissions on commerce schema tables
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.discount_codes TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.order_discounts TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.shipping_methods TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.order_fulfillments TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.fulfillment_items TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.order_returns TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.return_items TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.shopping_carts TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.cart_items TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.product_reviews TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.wishlists TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON commerce.wishlist_items TO olympus_app;