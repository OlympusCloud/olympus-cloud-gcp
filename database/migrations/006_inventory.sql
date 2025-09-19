-- ============================================================================
-- OLYMPUS CLOUD - INVENTORY MANAGEMENT TABLES
-- ============================================================================
-- Migration: 006_inventory.sql
-- Description: Inventory tracking, stock management, and transfer systems
-- Author: Claude Code Agent
-- Date: 2025-01-19
-- ============================================================================

-- Create inventory enums
CREATE TYPE inventory_adjustment_type AS ENUM (
    'increase',
    'decrease',
    'sale',
    'return',
    'damage',
    'loss',
    'transfer',
    'recount'
);

CREATE TYPE transfer_status AS ENUM (
    'draft',
    'pending',
    'shipped',
    'received',
    'cancelled'
);

-- ============================================================================
-- INVENTORY ITEMS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.inventory_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES commerce.products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES commerce.product_variants(id) ON DELETE CASCADE,
    location_id UUID REFERENCES platform.locations(id) ON DELETE CASCADE,
    sku VARCHAR(100) NOT NULL,

    -- Stock quantities
    quantity_available INTEGER NOT NULL DEFAULT 0 CHECK (quantity_available >= 0),
    quantity_reserved INTEGER NOT NULL DEFAULT 0 CHECK (quantity_reserved >= 0),
    quantity_on_hand INTEGER NOT NULL DEFAULT 0 CHECK (quantity_on_hand >= 0),

    -- Reorder management
    low_stock_threshold INTEGER CHECK (low_stock_threshold >= 0),
    reorder_point INTEGER CHECK (reorder_point >= 0),
    reorder_quantity INTEGER CHECK (reorder_quantity > 0),

    -- Costing
    cost_per_unit DECIMAL(10, 4) CHECK (cost_per_unit >= 0),
    last_cost_update_at TIMESTAMPTZ,

    -- Tracking
    last_counted_at TIMESTAMPTZ,
    last_movement_at TIMESTAMPTZ,

    -- Metadata
    notes TEXT,
    metadata JSONB DEFAULT '{}',

    -- Audit fields
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES auth.users(id),
    updated_by UUID REFERENCES auth.users(id),

    -- Indexes
    INDEX idx_inventory_items_tenant (tenant_id),
    INDEX idx_inventory_items_product (product_id),
    INDEX idx_inventory_items_variant (variant_id),
    INDEX idx_inventory_items_location (location_id),
    INDEX idx_inventory_items_sku (tenant_id, sku),
    INDEX idx_inventory_items_low_stock (tenant_id, quantity_available, low_stock_threshold)
        WHERE low_stock_threshold IS NOT NULL AND quantity_available <= low_stock_threshold,
    INDEX idx_inventory_items_out_of_stock (tenant_id) WHERE quantity_available = 0,
    INDEX idx_inventory_items_composite (tenant_id, product_id, variant_id, location_id),

    -- Constraints
    UNIQUE(tenant_id, product_id, variant_id, location_id),
    CONSTRAINT inventory_quantities_valid CHECK (quantity_on_hand = quantity_available + quantity_reserved)
);

-- ============================================================================
-- INVENTORY ADJUSTMENTS TABLE (Audit Log)
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.inventory_adjustments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    inventory_item_id UUID NOT NULL REFERENCES commerce.inventory_items(id) ON DELETE CASCADE,
    adjustment_type inventory_adjustment_type NOT NULL,
    quantity_change INTEGER NOT NULL,

    -- Context
    reason TEXT,
    reference_id UUID, -- order_id, transfer_id, etc.
    reference_type VARCHAR(50), -- 'order', 'transfer', 'manual', etc.

    -- Cost tracking
    cost_per_unit DECIMAL(10, 4),
    total_cost_impact DECIMAL(12, 2),

    -- Audit
    adjusted_by UUID NOT NULL REFERENCES auth.users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_inventory_adjustments_tenant (tenant_id),
    INDEX idx_inventory_adjustments_item (inventory_item_id),
    INDEX idx_inventory_adjustments_type (adjustment_type),
    INDEX idx_inventory_adjustments_reference (reference_id),
    INDEX idx_inventory_adjustments_created (created_at),
    INDEX idx_inventory_adjustments_user (adjusted_by)
);

-- ============================================================================
-- STOCK TRANSFERS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.stock_transfers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    transfer_number VARCHAR(50) UNIQUE NOT NULL,

    -- Locations
    from_location_id UUID NOT NULL REFERENCES platform.locations(id),
    to_location_id UUID NOT NULL REFERENCES platform.locations(id),

    -- Status tracking
    status transfer_status NOT NULL DEFAULT 'draft',

    -- Metadata
    notes TEXT,
    internal_notes TEXT,
    metadata JSONB DEFAULT '{}',

    -- Tracking
    shipped_at TIMESTAMPTZ,
    received_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,

    -- Audit
    created_by UUID NOT NULL REFERENCES auth.users(id),
    updated_by UUID REFERENCES auth.users(id),
    shipped_by UUID REFERENCES auth.users(id),
    received_by UUID REFERENCES auth.users(id),
    cancelled_by UUID REFERENCES auth.users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_stock_transfers_tenant (tenant_id),
    INDEX idx_stock_transfers_from_location (from_location_id),
    INDEX idx_stock_transfers_to_location (to_location_id),
    INDEX idx_stock_transfers_status (status),
    INDEX idx_stock_transfers_number (transfer_number),
    INDEX idx_stock_transfers_created (created_at),

    -- Constraints
    CONSTRAINT transfer_different_locations CHECK (from_location_id != to_location_id)
);

-- ============================================================================
-- STOCK TRANSFER ITEMS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.stock_transfer_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    transfer_id UUID NOT NULL REFERENCES commerce.stock_transfers(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES commerce.products(id),
    variant_id UUID REFERENCES commerce.product_variants(id),

    -- Quantities
    quantity_requested INTEGER NOT NULL CHECK (quantity_requested > 0),
    quantity_shipped INTEGER CHECK (quantity_shipped >= 0 AND quantity_shipped <= quantity_requested),
    quantity_received INTEGER CHECK (quantity_received >= 0 AND quantity_received <= quantity_shipped),

    -- Cost tracking
    cost_per_unit DECIMAL(10, 4),
    total_cost DECIMAL(12, 2),

    -- Notes
    notes TEXT,

    -- Audit
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_transfer_items_transfer (transfer_id),
    INDEX idx_transfer_items_product (product_id),
    INDEX idx_transfer_items_variant (variant_id)
);

-- ============================================================================
-- INVENTORY COUNTS TABLE (Physical Inventory)
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.inventory_counts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    count_number VARCHAR(50) UNIQUE NOT NULL,
    location_id UUID REFERENCES platform.locations(id),

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'draft', -- draft, in_progress, completed, cancelled

    -- Count details
    count_date DATE NOT NULL,
    count_type VARCHAR(20) NOT NULL DEFAULT 'full', -- full, partial, cycle

    -- Results
    total_items_counted INTEGER DEFAULT 0,
    total_discrepancies INTEGER DEFAULT 0,
    total_value_adjustment DECIMAL(12, 2) DEFAULT 0,

    -- Metadata
    notes TEXT,
    metadata JSONB DEFAULT '{}',

    -- Audit
    started_by UUID NOT NULL REFERENCES auth.users(id),
    completed_by UUID REFERENCES auth.users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    -- Indexes
    INDEX idx_inventory_counts_tenant (tenant_id),
    INDEX idx_inventory_counts_location (location_id),
    INDEX idx_inventory_counts_status (status),
    INDEX idx_inventory_counts_date (count_date),
    INDEX idx_inventory_counts_number (count_number)
);

-- ============================================================================
-- INVENTORY COUNT ITEMS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.inventory_count_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    count_id UUID NOT NULL REFERENCES commerce.inventory_counts(id) ON DELETE CASCADE,
    inventory_item_id UUID NOT NULL REFERENCES commerce.inventory_items(id),

    -- Count data
    system_quantity INTEGER NOT NULL,
    counted_quantity INTEGER,
    variance INTEGER GENERATED ALWAYS AS (counted_quantity - system_quantity) STORED,

    -- Cost impact
    cost_per_unit DECIMAL(10, 4),
    variance_value DECIMAL(12, 2) GENERATED ALWAYS AS (variance * cost_per_unit) STORED,

    -- Status
    is_counted BOOLEAN DEFAULT false,
    notes TEXT,

    -- Audit
    counted_by UUID REFERENCES auth.users(id),
    counted_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_count_items_count (count_id),
    INDEX idx_count_items_inventory (inventory_item_id),
    INDEX idx_count_items_variance (variance) WHERE variance != 0,
    INDEX idx_count_items_counted (is_counted)
);

-- ============================================================================
-- INVENTORY RESERVATIONS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.inventory_reservations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    inventory_item_id UUID NOT NULL REFERENCES commerce.inventory_items(id) ON DELETE CASCADE,

    -- Reservation details
    reference_id UUID NOT NULL, -- Usually order_id
    reference_type VARCHAR(50) NOT NULL DEFAULT 'order',
    quantity_reserved INTEGER NOT NULL CHECK (quantity_reserved > 0),

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, fulfilled, released, expired

    -- Timing
    expires_at TIMESTAMPTZ,
    fulfilled_at TIMESTAMPTZ,
    released_at TIMESTAMPTZ,

    -- Audit
    created_by UUID NOT NULL REFERENCES auth.users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_reservations_tenant (tenant_id),
    INDEX idx_reservations_item (inventory_item_id),
    INDEX idx_reservations_reference (reference_id),
    INDEX idx_reservations_status (status),
    INDEX idx_reservations_expires (expires_at) WHERE expires_at IS NOT NULL,

    -- Constraints
    UNIQUE(inventory_item_id, reference_id, reference_type)
);

-- ============================================================================
-- REORDER RULES TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.reorder_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES commerce.products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES commerce.product_variants(id) ON DELETE CASCADE,
    location_id UUID REFERENCES platform.locations(id) ON DELETE CASCADE,

    -- Reorder settings
    reorder_point INTEGER NOT NULL CHECK (reorder_point >= 0),
    reorder_quantity INTEGER NOT NULL CHECK (reorder_quantity > 0),
    max_stock_level INTEGER CHECK (max_stock_level > reorder_quantity),

    -- Supplier information
    supplier_id UUID, -- References to be added when supplier module exists
    supplier_sku VARCHAR(100),
    lead_time_days INTEGER DEFAULT 7,

    -- Rule settings
    is_active BOOLEAN DEFAULT true,
    auto_reorder BOOLEAN DEFAULT false,

    -- Metadata
    notes TEXT,

    -- Audit
    created_by UUID NOT NULL REFERENCES auth.users(id),
    updated_by UUID REFERENCES auth.users(id),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_reorder_rules_tenant (tenant_id),
    INDEX idx_reorder_rules_product (product_id),
    INDEX idx_reorder_rules_variant (variant_id),
    INDEX idx_reorder_rules_location (location_id),
    INDEX idx_reorder_rules_active (tenant_id, is_active) WHERE is_active = true,

    -- Constraints
    UNIQUE(tenant_id, product_id, variant_id, location_id)
);

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Function to update inventory item updated_at timestamp
CREATE OR REPLACE FUNCTION update_inventory_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    NEW.last_movement_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to generate transfer numbers
CREATE OR REPLACE FUNCTION generate_transfer_number()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.transfer_number IS NULL THEN
        NEW.transfer_number := 'TXR-' || TO_CHAR(NOW(), 'YYYYMMDD') || '-' ||
                              LPAD(EXTRACT(EPOCH FROM NOW())::TEXT, 10, '0');
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to generate count numbers
CREATE OR REPLACE FUNCTION generate_count_number()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.count_number IS NULL THEN
        NEW.count_number := 'CNT-' || TO_CHAR(NOW(), 'YYYYMMDD') || '-' ||
                           LPAD(EXTRACT(EPOCH FROM NOW())::TEXT, 10, '0');
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Update timestamp triggers
CREATE TRIGGER update_inventory_items_updated_at
    BEFORE UPDATE ON commerce.inventory_items
    FOR EACH ROW
    EXECUTE FUNCTION update_inventory_updated_at();

CREATE TRIGGER update_stock_transfers_updated_at
    BEFORE UPDATE ON commerce.stock_transfers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transfer_items_updated_at
    BEFORE UPDATE ON commerce.stock_transfer_items
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_inventory_counts_updated_at
    BEFORE UPDATE ON commerce.inventory_counts
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reservations_updated_at
    BEFORE UPDATE ON commerce.inventory_reservations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_reorder_rules_updated_at
    BEFORE UPDATE ON commerce.reorder_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Auto-generate numbers
CREATE TRIGGER generate_stock_transfer_number
    BEFORE INSERT ON commerce.stock_transfers
    FOR EACH ROW
    EXECUTE FUNCTION generate_transfer_number();

CREATE TRIGGER generate_inventory_count_number
    BEFORE INSERT ON commerce.inventory_counts
    FOR EACH ROW
    EXECUTE FUNCTION generate_count_number();

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

-- Enable RLS on all inventory tables
ALTER TABLE commerce.inventory_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.inventory_adjustments ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.stock_transfers ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.stock_transfer_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.inventory_counts ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.inventory_count_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.inventory_reservations ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.reorder_rules ENABLE ROW LEVEL SECURITY;

-- Inventory Items Policies
CREATE POLICY tenant_isolation_inventory_items ON commerce.inventory_items
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Inventory Adjustments Policies
CREATE POLICY tenant_isolation_inventory_adjustments ON commerce.inventory_adjustments
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Stock Transfers Policies
CREATE POLICY tenant_isolation_stock_transfers ON commerce.stock_transfers
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Transfer Items Policies (inherit from transfer)
CREATE POLICY tenant_isolation_transfer_items ON commerce.stock_transfer_items
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM commerce.stock_transfers
            WHERE id = transfer_id
            AND tenant_id = current_setting('app.tenant_id')::UUID
        )
    );

-- Inventory Counts Policies
CREATE POLICY tenant_isolation_inventory_counts ON commerce.inventory_counts
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Count Items Policies (inherit from count)
CREATE POLICY tenant_isolation_count_items ON commerce.inventory_count_items
    FOR ALL
    USING (
        EXISTS (
            SELECT 1 FROM commerce.inventory_counts
            WHERE id = count_id
            AND tenant_id = current_setting('app.tenant_id')::UUID
        )
    );

-- Reservations Policies
CREATE POLICY tenant_isolation_inventory_reservations ON commerce.inventory_reservations
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Reorder Rules Policies
CREATE POLICY tenant_isolation_reorder_rules ON commerce.reorder_rules
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- ============================================================================
-- GRANTS
-- ============================================================================

-- Grant permissions to application role
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA commerce TO app_user;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA commerce TO app_user;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA commerce TO app_user;

-- ============================================================================
-- VIEWS
-- ============================================================================

-- View for current stock levels with product information
CREATE VIEW commerce.current_stock_levels AS
SELECT
    ii.id,
    ii.tenant_id,
    ii.product_id,
    ii.variant_id,
    ii.location_id,
    ii.sku,
    p.name as product_name,
    pv.name as variant_name,
    l.name as location_name,
    ii.quantity_available,
    ii.quantity_reserved,
    ii.quantity_on_hand,
    ii.low_stock_threshold,
    ii.reorder_point,
    ii.cost_per_unit,
    p.base_price,
    (ii.quantity_on_hand * COALESCE(ii.cost_per_unit, 0)) as total_cost_value,
    (ii.quantity_on_hand * p.base_price) as total_retail_value,
    CASE
        WHEN ii.quantity_available = 0 THEN 'out_of_stock'
        WHEN ii.low_stock_threshold IS NOT NULL AND ii.quantity_available <= ii.low_stock_threshold THEN 'low_stock'
        ELSE 'in_stock'
    END as stock_status,
    ii.last_movement_at,
    ii.last_counted_at
FROM commerce.inventory_items ii
JOIN commerce.products p ON ii.product_id = p.id
LEFT JOIN commerce.product_variants pv ON ii.variant_id = pv.id
LEFT JOIN platform.locations l ON ii.location_id = l.id;

-- View for low stock alerts
CREATE VIEW commerce.low_stock_alerts AS
SELECT
    csl.*,
    COALESCE(csl.reorder_quantity, rr.reorder_quantity) as suggested_reorder_quantity,
    rr.supplier_id,
    rr.lead_time_days
FROM commerce.current_stock_levels csl
LEFT JOIN commerce.reorder_rules rr ON (
    csl.tenant_id = rr.tenant_id
    AND csl.product_id = rr.product_id
    AND (csl.variant_id = rr.variant_id OR (csl.variant_id IS NULL AND rr.variant_id IS NULL))
    AND (csl.location_id = rr.location_id OR (csl.location_id IS NULL AND rr.location_id IS NULL))
    AND rr.is_active = true
)
WHERE csl.stock_status = 'low_stock' OR csl.stock_status = 'out_of_stock';

-- ============================================================================
-- INDEXES FOR VIEWS
-- ============================================================================

-- Additional indexes to support views and common queries
CREATE INDEX IF NOT EXISTS idx_products_base_price ON commerce.products(base_price);
CREATE INDEX IF NOT EXISTS idx_inventory_items_movement_date ON commerce.inventory_items(last_movement_at DESC);
CREATE INDEX IF NOT EXISTS idx_inventory_adjustments_date_type ON commerce.inventory_adjustments(created_at DESC, adjustment_type);

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE commerce.inventory_items IS 'Core inventory tracking with stock levels per product/variant/location';
COMMENT ON TABLE commerce.inventory_adjustments IS 'Audit trail of all inventory movements and adjustments';
COMMENT ON TABLE commerce.stock_transfers IS 'Inter-location inventory transfers';
COMMENT ON TABLE commerce.inventory_counts IS 'Physical inventory count sessions';
COMMENT ON TABLE commerce.inventory_reservations IS 'Stock reservations for orders and other purposes';
COMMENT ON TABLE commerce.reorder_rules IS 'Automatic reorder point configuration per product/location';

COMMENT ON VIEW commerce.current_stock_levels IS 'Current stock levels with product details and status';
COMMENT ON VIEW commerce.low_stock_alerts IS 'Items requiring reorder attention';