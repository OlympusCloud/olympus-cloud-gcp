-- ============================================================================
-- OLYMPUS CLOUD - RESTAURANT SYSTEM MIGRATION
-- ============================================================================
-- Migration: 007_restaurant_system.sql
-- Description: Restaurant-specific tables for table management, orders, and kitchen display
-- Author: Claude Code Agent
-- Date: 2025-01-19
-- ============================================================================

-- Restaurant Tables Management
CREATE TABLE commerce.restaurant_tables (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL,
    table_number VARCHAR(50) NOT NULL,
    name VARCHAR(255),
    capacity INTEGER NOT NULL CHECK (capacity > 0),
    status VARCHAR(50) NOT NULL DEFAULT 'Available',
    section VARCHAR(100),
    position_x DECIMAL(10,2),
    position_y DECIMAL(10,2),
    current_order_id UUID,
    server_id UUID,
    last_cleaned_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(tenant_id, location_id, table_number),
    CONSTRAINT valid_table_status CHECK (status IN ('Available', 'Occupied', 'Reserved', 'Cleaning', 'OutOfOrder'))
);

-- Restaurant Orders
CREATE TABLE commerce.restaurant_orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL,
    order_number VARCHAR(50) NOT NULL,
    table_id UUID REFERENCES commerce.restaurant_tables(id),
    server_id UUID,
    customer_name VARCHAR(255),
    guest_count INTEGER,
    order_type VARCHAR(50) NOT NULL DEFAULT 'DineIn',
    status VARCHAR(50) NOT NULL DEFAULT 'Open',
    subtotal DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    tax_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    tip_amount DECIMAL(15,2),
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    payment_status VARCHAR(50) NOT NULL DEFAULT 'Pending',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    seat_time TIMESTAMPTZ,
    order_time TIMESTAMPTZ,
    kitchen_time TIMESTAMPTZ,
    served_time TIMESTAMPTZ,
    check_closed_at TIMESTAMPTZ,
    UNIQUE(tenant_id, order_number),
    CONSTRAINT valid_order_type CHECK (order_type IN ('DineIn', 'Takeout', 'Delivery', 'Pickup')),
    CONSTRAINT valid_order_status CHECK (status IN ('Open', 'Fired', 'InProgress', 'Ready', 'Served', 'Closed', 'Cancelled')),
    CONSTRAINT valid_payment_status CHECK (payment_status IN ('Pending', 'PartiallyPaid', 'Paid', 'Refunded', 'Failed'))
);

-- Restaurant Order Items
CREATE TABLE commerce.restaurant_order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES commerce.restaurant_orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    unit_price DECIMAL(15,2) NOT NULL,
    special_instructions TEXT,
    kitchen_status VARCHAR(50) NOT NULL DEFAULT 'Pending',
    fired_at TIMESTAMPTZ,
    ready_at TIMESTAMPTZ,
    served_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_kitchen_status CHECK (kitchen_status IN ('Pending', 'InPreparation', 'Ready', 'Served', 'Cancelled'))
);

-- Order Item Modifiers
CREATE TABLE commerce.order_item_modifiers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_item_id UUID NOT NULL REFERENCES commerce.restaurant_order_items(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    price_adjustment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    modifier_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT valid_modifier_type CHECK (modifier_type IN ('Addition', 'Removal', 'Substitution', 'Size', 'Preparation'))
);

-- Update restaurant_tables to reference orders
ALTER TABLE commerce.restaurant_tables
ADD CONSTRAINT fk_restaurant_tables_current_order
FOREIGN KEY (current_order_id) REFERENCES commerce.restaurant_orders(id);

-- Indexes for performance
CREATE INDEX idx_restaurant_tables_tenant_location ON commerce.restaurant_tables(tenant_id, location_id);
CREATE INDEX idx_restaurant_tables_status ON commerce.restaurant_tables(status);
CREATE INDEX idx_restaurant_tables_server ON commerce.restaurant_tables(server_id);

CREATE INDEX idx_restaurant_orders_tenant_location ON commerce.restaurant_orders(tenant_id, location_id);
CREATE INDEX idx_restaurant_orders_status ON commerce.restaurant_orders(status);
CREATE INDEX idx_restaurant_orders_table ON commerce.restaurant_orders(table_id);
CREATE INDEX idx_restaurant_orders_server ON commerce.restaurant_orders(server_id);
CREATE INDEX idx_restaurant_orders_created_at ON commerce.restaurant_orders(created_at);
CREATE INDEX idx_restaurant_orders_payment_status ON commerce.restaurant_orders(payment_status);

CREATE INDEX idx_restaurant_order_items_order ON commerce.restaurant_order_items(order_id);
CREATE INDEX idx_restaurant_order_items_kitchen_status ON commerce.restaurant_order_items(kitchen_status);
CREATE INDEX idx_restaurant_order_items_product ON commerce.restaurant_order_items(product_id);

CREATE INDEX idx_order_item_modifiers_item ON commerce.order_item_modifiers(order_item_id);

-- Row Level Security for multi-tenancy
ALTER TABLE commerce.restaurant_tables ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.restaurant_orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.restaurant_order_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.order_item_modifiers ENABLE ROW LEVEL SECURITY;

-- RLS Policies for restaurant_tables
CREATE POLICY restaurant_tables_tenant_isolation ON commerce.restaurant_tables
    FOR ALL USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- RLS Policies for restaurant_orders
CREATE POLICY restaurant_orders_tenant_isolation ON commerce.restaurant_orders
    FOR ALL USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- RLS Policies for restaurant_order_items (inherit from orders)
CREATE POLICY restaurant_order_items_tenant_isolation ON commerce.restaurant_order_items
    FOR ALL USING (
        order_id IN (
            SELECT id FROM commerce.restaurant_orders
            WHERE tenant_id = current_setting('app.tenant_id')::UUID
        )
    );

-- RLS Policies for order_item_modifiers (inherit from order items)
CREATE POLICY order_item_modifiers_tenant_isolation ON commerce.order_item_modifiers
    FOR ALL USING (
        order_item_id IN (
            SELECT oi.id FROM commerce.restaurant_order_items oi
            JOIN commerce.restaurant_orders o ON oi.order_id = o.id
            WHERE o.tenant_id = current_setting('app.tenant_id')::UUID
        )
    );

-- Functions for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_restaurant_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers for automatic timestamp updates
CREATE TRIGGER restaurant_tables_update_timestamp
    BEFORE UPDATE ON commerce.restaurant_tables
    FOR EACH ROW
    EXECUTE FUNCTION update_restaurant_updated_at();

CREATE TRIGGER restaurant_orders_update_timestamp
    BEFORE UPDATE ON commerce.restaurant_orders
    FOR EACH ROW
    EXECUTE FUNCTION update_restaurant_updated_at();

CREATE TRIGGER restaurant_order_items_update_timestamp
    BEFORE UPDATE ON commerce.restaurant_order_items
    FOR EACH ROW
    EXECUTE FUNCTION update_restaurant_updated_at();

-- Sample data for development
INSERT INTO commerce.restaurant_tables (tenant_id, location_id, table_number, name, capacity, section, position_x, position_y) VALUES
    ('550e8400-e29b-41d4-a716-446655440000', '550e8400-e29b-41d4-a716-446655440001', '1', 'Table 1', 4, 'Main Dining', 100.0, 100.0),
    ('550e8400-e29b-41d4-a716-446655440000', '550e8400-e29b-41d4-a716-446655440001', '2', 'Table 2', 6, 'Main Dining', 200.0, 100.0),
    ('550e8400-e29b-41d4-a716-446655440000', '550e8400-e29b-41d4-a716-446655440001', '3', 'Table 3', 2, 'Bar Area', 300.0, 200.0),
    ('550e8400-e29b-41d4-a716-446655440000', '550e8400-e29b-41d4-a716-446655440001', '4', 'Table 4', 8, 'Private Room', 150.0, 300.0);

-- Comments for documentation
COMMENT ON TABLE commerce.restaurant_tables IS 'Physical tables in restaurant locations with status and position tracking';
COMMENT ON TABLE commerce.restaurant_orders IS 'Customer orders with table assignments and service timing';
COMMENT ON TABLE commerce.restaurant_order_items IS 'Individual items within orders with kitchen workflow tracking';
COMMENT ON TABLE commerce.order_item_modifiers IS 'Customizations and modifications to order items (extra cheese, no onions, etc.)';

COMMENT ON COLUMN commerce.restaurant_tables.position_x IS 'X coordinate for table positioning in floor plan (optional)';
COMMENT ON COLUMN commerce.restaurant_tables.position_y IS 'Y coordinate for table positioning in floor plan (optional)';
COMMENT ON COLUMN commerce.restaurant_orders.seat_time IS 'When customers were seated at the table';
COMMENT ON COLUMN commerce.restaurant_orders.order_time IS 'When the order was placed';
COMMENT ON COLUMN commerce.restaurant_orders.kitchen_time IS 'When order was fired to kitchen';
COMMENT ON COLUMN commerce.restaurant_orders.served_time IS 'When order was served to table';
COMMENT ON COLUMN commerce.restaurant_orders.check_closed_at IS 'When payment was completed and table cleared';