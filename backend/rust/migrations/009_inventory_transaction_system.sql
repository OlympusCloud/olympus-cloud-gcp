-- ============================================================================
-- OLYMPUS CLOUD - INVENTORY TRANSACTION SYSTEM ENHANCEMENT
-- ============================================================================
-- Migration: 009_inventory_transaction_system.sql
-- Description: Enhanced inventory management with ACID transactions and comprehensive stock tracking
-- Author: Claude Code Agent
-- Date: 2025-01-19
-- ============================================================================

-- Extend adjustment types for comprehensive inventory management
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'SALE';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'PURCHASE';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'RETURN';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'DAMAGE';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'THEFT';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'EXPIRED';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'TRANSFER_IN';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'TRANSFER_OUT';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'PRODUCTION';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'CONSUMPTION';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'CYCLE_COUNT';
ALTER TYPE adjustment_type ADD VALUE IF NOT EXISTS 'REVALUATION';

-- Create new enum for stock movement types
CREATE TYPE stock_movement_type AS ENUM (
    'INBOUND',      -- Receiving inventory
    'OUTBOUND',     -- Shipping/selling inventory
    'TRANSFER',     -- Moving between locations
    'ADJUSTMENT',   -- Manual adjustments
    'RESERVATION',  -- Reserving stock for orders
    'RELEASE',      -- Releasing reserved stock
    'ALLOCATION',   -- Allocating stock to specific orders
    'DEALLOCATION'  -- Removing allocations
);

-- Create new enum for transaction status
CREATE TYPE inventory_transaction_status AS ENUM (
    'PENDING',
    'IN_PROGRESS',
    'COMPLETED',
    'CANCELLED',
    'FAILED',
    'ROLLED_BACK'
);

-- Create enhanced inventory transaction table for ACID compliance
CREATE TABLE inventory_transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_number VARCHAR(50) NOT NULL,
    transaction_type stock_movement_type NOT NULL,
    status inventory_transaction_status NOT NULL DEFAULT 'PENDING',
    reference_type VARCHAR(50), -- 'order', 'purchase_order', 'transfer', 'adjustment'
    reference_id UUID,
    source_location_id UUID REFERENCES locations(id),
    destination_location_id UUID REFERENCES locations(id),
    user_id UUID REFERENCES users(id),
    notes TEXT,

    -- Transaction control fields
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    rollback_reason TEXT,

    -- Audit fields
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, transaction_number)
);

-- Create inventory transaction items (the actual stock movements)
CREATE TABLE inventory_transaction_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES inventory_transactions(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventory(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    variant_id UUID REFERENCES product_variants(id),

    -- Quantity fields
    quantity INTEGER NOT NULL,
    quantity_processed INTEGER DEFAULT 0,
    unit_cost DECIMAL(19,4),
    total_cost DECIMAL(19,4),

    -- Stock levels before/after for audit
    quantity_before INTEGER NOT NULL,
    reserved_before INTEGER NOT NULL,
    quantity_after INTEGER,
    reserved_after INTEGER,

    -- Item status
    status inventory_transaction_status NOT NULL DEFAULT 'PENDING',
    processed_at TIMESTAMPTZ,
    error_message TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create stock reservations table for order management
CREATE TABLE stock_reservations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventory(id) ON DELETE CASCADE,
    reference_type VARCHAR(50) NOT NULL, -- 'order', 'cart', 'quote'
    reference_id UUID NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    reserved_until TIMESTAMPTZ NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_reservation_status CHECK (status IN ('ACTIVE', 'EXPIRED', 'RELEASED', 'ALLOCATED'))
);

-- Create stock movements audit table
CREATE TABLE stock_movements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventory(id) ON DELETE CASCADE,
    transaction_id UUID REFERENCES inventory_transactions(id),
    movement_type stock_movement_type NOT NULL,

    -- Movement details
    quantity_change INTEGER NOT NULL,
    quantity_before INTEGER NOT NULL,
    quantity_after INTEGER NOT NULL,
    reserved_change INTEGER DEFAULT 0,
    reserved_before INTEGER NOT NULL,
    reserved_after INTEGER NOT NULL,

    -- Cost tracking
    unit_cost DECIMAL(19,4),
    total_value_change DECIMAL(19,4),
    running_value DECIMAL(19,4),

    -- Reference information
    reference_type VARCHAR(50),
    reference_id UUID,
    reason TEXT,

    -- Audit information
    performed_by UUID REFERENCES users(id),
    performed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    location_id UUID REFERENCES locations(id),

    -- Additional metadata
    batch_number VARCHAR(100),
    expiry_date DATE,
    lot_number VARCHAR(100),
    metadata JSONB DEFAULT '{}',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create inventory valuation table for cost tracking
CREATE TABLE inventory_valuations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventory(id) ON DELETE CASCADE,
    valuation_method VARCHAR(20) NOT NULL DEFAULT 'FIFO', -- FIFO, LIFO, AVERAGE, SPECIFIC

    -- Cost information
    unit_cost DECIMAL(19,4) NOT NULL,
    total_quantity INTEGER NOT NULL,
    total_value DECIMAL(19,4) NOT NULL,
    average_cost DECIMAL(19,4),

    -- Timestamp for valuation
    valued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ,

    -- Audit
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_valuation_method CHECK (valuation_method IN ('FIFO', 'LIFO', 'AVERAGE', 'SPECIFIC'))
);

-- Create inventory lots table for batch/lot tracking
CREATE TABLE inventory_lots (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventory(id) ON DELETE CASCADE,
    lot_number VARCHAR(100) NOT NULL,
    batch_number VARCHAR(100),

    -- Quantity tracking
    quantity_received INTEGER NOT NULL,
    quantity_available INTEGER NOT NULL,
    quantity_allocated INTEGER DEFAULT 0,

    -- Cost and valuation
    unit_cost DECIMAL(19,4) NOT NULL,
    total_cost DECIMAL(19,4) NOT NULL,

    -- Dates
    received_date DATE NOT NULL,
    expiry_date DATE,
    manufacture_date DATE,

    -- Status
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',

    -- Supplier information
    supplier_id UUID,
    purchase_order_reference VARCHAR(100),

    -- Quality control
    quality_status VARCHAR(20) DEFAULT 'APPROVED',
    quality_notes TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_lot_status CHECK (status IN ('ACTIVE', 'EXPIRED', 'QUARANTINE', 'DISPOSED')),
    CONSTRAINT valid_quality_status CHECK (quality_status IN ('PENDING', 'APPROVED', 'REJECTED', 'ON_HOLD'))
);

-- Add fields to inventory table for enhanced tracking
ALTER TABLE inventory
ADD COLUMN IF NOT EXISTS average_cost DECIMAL(19,4) DEFAULT 0.00,
ADD COLUMN IF NOT EXISTS total_value DECIMAL(19,4) DEFAULT 0.00,
ADD COLUMN IF NOT EXISTS last_movement_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS minimum_stock_level INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS maximum_stock_level INTEGER,
ADD COLUMN IF NOT EXISTS optimal_stock_level INTEGER,
ADD COLUMN IF NOT EXISTS safety_stock INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS lead_time_days INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS abc_classification CHAR(1) DEFAULT 'C',
ADD COLUMN IF NOT EXISTS velocity_score DECIMAL(5,2) DEFAULT 0.00,
ADD COLUMN IF NOT EXISTS last_sale_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS is_serialized BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS is_lot_tracked BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS is_expired_tracking BOOLEAN DEFAULT false;

-- Create comprehensive indexes for performance
CREATE INDEX idx_inventory_transactions_tenant ON inventory_transactions(tenant_id);
CREATE INDEX idx_inventory_transactions_status ON inventory_transactions(status);
CREATE INDEX idx_inventory_transactions_type ON inventory_transactions(transaction_type);
CREATE INDEX idx_inventory_transactions_reference ON inventory_transactions(reference_type, reference_id);
CREATE INDEX idx_inventory_transactions_started ON inventory_transactions(started_at);

CREATE INDEX idx_transaction_items_transaction ON inventory_transaction_items(transaction_id);
CREATE INDEX idx_transaction_items_inventory ON inventory_transaction_items(inventory_id);
CREATE INDEX idx_transaction_items_status ON inventory_transaction_items(status);

CREATE INDEX idx_stock_reservations_tenant ON stock_reservations(tenant_id);
CREATE INDEX idx_stock_reservations_inventory ON stock_reservations(inventory_id);
CREATE INDEX idx_stock_reservations_reference ON stock_reservations(reference_type, reference_id);
CREATE INDEX idx_stock_reservations_status ON stock_reservations(status);
CREATE INDEX idx_stock_reservations_expires ON stock_reservations(reserved_until);

CREATE INDEX idx_stock_movements_tenant ON stock_movements(tenant_id);
CREATE INDEX idx_stock_movements_inventory ON stock_movements(inventory_id);
CREATE INDEX idx_stock_movements_transaction ON stock_movements(transaction_id);
CREATE INDEX idx_stock_movements_type ON stock_movements(movement_type);
CREATE INDEX idx_stock_movements_performed ON stock_movements(performed_at);
CREATE INDEX idx_stock_movements_reference ON stock_movements(reference_type, reference_id);

CREATE INDEX idx_inventory_valuations_inventory ON inventory_valuations(inventory_id);
CREATE INDEX idx_inventory_valuations_valid ON inventory_valuations(valid_from, valid_until);

CREATE INDEX idx_inventory_lots_inventory ON inventory_lots(inventory_id);
CREATE INDEX idx_inventory_lots_lot_number ON inventory_lots(lot_number);
CREATE INDEX idx_inventory_lots_expiry ON inventory_lots(expiry_date);
CREATE INDEX idx_inventory_lots_status ON inventory_lots(status);

-- Row Level Security
ALTER TABLE inventory_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory_transaction_items ENABLE ROW LEVEL SECURITY;
ALTER TABLE stock_reservations ENABLE ROW LEVEL SECURITY;
ALTER TABLE stock_movements ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory_valuations ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory_lots ENABLE ROW LEVEL SECURITY;

-- RLS Policies
CREATE POLICY inventory_transactions_tenant_policy ON inventory_transactions
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY transaction_items_tenant_policy ON inventory_transaction_items
    FOR ALL USING (
        transaction_id IN (
            SELECT id FROM inventory_transactions
            WHERE tenant_id = current_setting('app.tenant_id', true)::UUID
        )
    );

CREATE POLICY stock_reservations_tenant_policy ON stock_reservations
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY stock_movements_tenant_policy ON stock_movements
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY inventory_valuations_tenant_policy ON inventory_valuations
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY inventory_lots_tenant_policy ON inventory_lots
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- Triggers for automatic timestamp updates
CREATE TRIGGER inventory_transactions_update_timestamp
    BEFORE UPDATE ON inventory_transactions
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER stock_reservations_update_timestamp
    BEFORE UPDATE ON stock_reservations
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER inventory_lots_update_timestamp
    BEFORE UPDATE ON inventory_lots
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Functions for inventory transaction management

-- Function to start an inventory transaction
CREATE OR REPLACE FUNCTION start_inventory_transaction(
    p_tenant_id UUID,
    p_transaction_type stock_movement_type,
    p_reference_type VARCHAR DEFAULT NULL,
    p_reference_id UUID DEFAULT NULL,
    p_source_location_id UUID DEFAULT NULL,
    p_destination_location_id UUID DEFAULT NULL,
    p_user_id UUID DEFAULT NULL,
    p_notes TEXT DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_transaction_id UUID;
    v_transaction_number VARCHAR(50);
BEGIN
    -- Generate transaction number
    v_transaction_number := 'TXN-' || TO_CHAR(NOW(), 'YYYYMMDD') || '-' ||
                           LPAD(EXTRACT(EPOCH FROM NOW())::TEXT, 10, '0');

    -- Create transaction
    INSERT INTO inventory_transactions (
        tenant_id, transaction_number, transaction_type, reference_type, reference_id,
        source_location_id, destination_location_id, user_id, notes
    ) VALUES (
        p_tenant_id, v_transaction_number, p_transaction_type, p_reference_type, p_reference_id,
        p_source_location_id, p_destination_location_id, p_user_id, p_notes
    ) RETURNING id INTO v_transaction_id;

    RETURN v_transaction_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to add item to transaction
CREATE OR REPLACE FUNCTION add_transaction_item(
    p_transaction_id UUID,
    p_inventory_id UUID,
    p_quantity INTEGER,
    p_unit_cost DECIMAL DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_item_id UUID;
    v_current_quantity INTEGER;
    v_current_reserved INTEGER;
    v_product_id UUID;
    v_variant_id UUID;
BEGIN
    -- Get current inventory levels
    SELECT quantity_on_hand, quantity_reserved, product_id, variant_id
    INTO v_current_quantity, v_current_reserved, v_product_id, v_variant_id
    FROM inventory
    WHERE id = p_inventory_id;

    -- Create transaction item
    INSERT INTO inventory_transaction_items (
        transaction_id, inventory_id, product_id, variant_id, quantity,
        unit_cost, total_cost, quantity_before, reserved_before
    ) VALUES (
        p_transaction_id, p_inventory_id, v_product_id, v_variant_id, p_quantity,
        p_unit_cost, p_unit_cost * p_quantity, v_current_quantity, v_current_reserved
    ) RETURNING id INTO v_item_id;

    RETURN v_item_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to reserve stock
CREATE OR REPLACE FUNCTION reserve_stock(
    p_tenant_id UUID,
    p_inventory_id UUID,
    p_reference_type VARCHAR,
    p_reference_id UUID,
    p_quantity INTEGER,
    p_reserved_until TIMESTAMPTZ DEFAULT NULL,
    p_created_by UUID DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_reservation_id UUID;
    v_available_quantity INTEGER;
BEGIN
    -- Check available quantity
    SELECT quantity_available INTO v_available_quantity
    FROM inventory
    WHERE id = p_inventory_id AND tenant_id = p_tenant_id;

    IF v_available_quantity IS NULL THEN
        RAISE EXCEPTION 'Inventory item not found';
    END IF;

    IF v_available_quantity < p_quantity THEN
        RAISE EXCEPTION 'Insufficient inventory. Available: %, Requested: %', v_available_quantity, p_quantity;
    END IF;

    -- Create reservation
    INSERT INTO stock_reservations (
        tenant_id, inventory_id, reference_type, reference_id, quantity,
        reserved_until, created_by
    ) VALUES (
        p_tenant_id, p_inventory_id, p_reference_type, p_reference_id, p_quantity,
        COALESCE(p_reserved_until, NOW() + INTERVAL '24 hours'), p_created_by
    ) RETURNING id INTO v_reservation_id;

    -- Update inventory reserved quantity
    UPDATE inventory
    SET quantity_reserved = quantity_reserved + p_quantity,
        updated_at = NOW()
    WHERE id = p_inventory_id;

    RETURN v_reservation_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to release stock reservation
CREATE OR REPLACE FUNCTION release_stock_reservation(
    p_reservation_id UUID
) RETURNS BOOLEAN AS $$
DECLARE
    v_inventory_id UUID;
    v_quantity INTEGER;
BEGIN
    -- Get reservation details
    SELECT inventory_id, quantity INTO v_inventory_id, v_quantity
    FROM stock_reservations
    WHERE id = p_reservation_id AND status = 'ACTIVE';

    IF NOT FOUND THEN
        RETURN FALSE;
    END IF;

    -- Update reservation status
    UPDATE stock_reservations
    SET status = 'RELEASED', updated_at = NOW()
    WHERE id = p_reservation_id;

    -- Update inventory
    UPDATE inventory
    SET quantity_reserved = quantity_reserved - v_quantity,
        updated_at = NOW()
    WHERE id = v_inventory_id;

    RETURN TRUE;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to process inventory transaction (ACID)
CREATE OR REPLACE FUNCTION process_inventory_transaction(
    p_transaction_id UUID
) RETURNS BOOLEAN AS $$
DECLARE
    v_item RECORD;
    v_transaction_type stock_movement_type;
    v_tenant_id UUID;
BEGIN
    -- Get transaction details
    SELECT transaction_type, tenant_id INTO v_transaction_type, v_tenant_id
    FROM inventory_transactions
    WHERE id = p_transaction_id AND status = 'PENDING';

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Transaction not found or not in pending status';
    END IF;

    -- Update transaction status
    UPDATE inventory_transactions
    SET status = 'IN_PROGRESS', updated_at = NOW()
    WHERE id = p_transaction_id;

    -- Process each transaction item
    FOR v_item IN
        SELECT * FROM inventory_transaction_items
        WHERE transaction_id = p_transaction_id AND status = 'PENDING'
    LOOP
        -- Update inventory based on transaction type
        IF v_transaction_type IN ('INBOUND', 'PURCHASE', 'TRANSFER_IN', 'RETURN') THEN
            -- Add to inventory
            UPDATE inventory
            SET quantity_on_hand = quantity_on_hand + v_item.quantity,
                last_movement_at = NOW(),
                updated_at = NOW()
            WHERE id = v_item.inventory_id;

        ELSIF v_transaction_type IN ('OUTBOUND', 'SALE', 'TRANSFER_OUT', 'DAMAGE', 'THEFT') THEN
            -- Remove from inventory
            UPDATE inventory
            SET quantity_on_hand = quantity_on_hand - v_item.quantity,
                last_movement_at = NOW(),
                updated_at = NOW()
            WHERE id = v_item.inventory_id;

        END IF;

        -- Update item status
        UPDATE inventory_transaction_items
        SET status = 'COMPLETED',
            quantity_processed = v_item.quantity,
            processed_at = NOW()
        WHERE id = v_item.id;

        -- Record stock movement
        INSERT INTO stock_movements (
            tenant_id, inventory_id, transaction_id, movement_type,
            quantity_change, quantity_before, quantity_after,
            reserved_before, reserved_after, reference_type, reference_id
        ) SELECT
            v_tenant_id, v_item.inventory_id, p_transaction_id, v_transaction_type,
            CASE
                WHEN v_transaction_type IN ('INBOUND', 'PURCHASE', 'TRANSFER_IN', 'RETURN')
                THEN v_item.quantity
                ELSE -v_item.quantity
            END,
            v_item.quantity_before,
            i.quantity_on_hand,
            v_item.reserved_before,
            i.quantity_reserved
        FROM inventory i
        WHERE i.id = v_item.inventory_id;
    END LOOP;

    -- Complete transaction
    UPDATE inventory_transactions
    SET status = 'COMPLETED', completed_at = NOW(), updated_at = NOW()
    WHERE id = p_transaction_id;

    RETURN TRUE;
EXCEPTION
    WHEN OTHERS THEN
        -- Rollback transaction
        UPDATE inventory_transactions
        SET status = 'FAILED', rollback_reason = SQLERRM, updated_at = NOW()
        WHERE id = p_transaction_id;

        RAISE;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Grant permissions
GRANT SELECT, INSERT, UPDATE ON inventory_transactions TO olympus_app;
GRANT SELECT, INSERT, UPDATE ON inventory_transaction_items TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON stock_reservations TO olympus_app;
GRANT SELECT, INSERT ON stock_movements TO olympus_app;
GRANT SELECT, INSERT, UPDATE ON inventory_valuations TO olympus_app;
GRANT SELECT, INSERT, UPDATE ON inventory_lots TO olympus_app;

GRANT EXECUTE ON FUNCTION start_inventory_transaction(UUID, stock_movement_type, VARCHAR, UUID, UUID, UUID, UUID, TEXT) TO olympus_app;
GRANT EXECUTE ON FUNCTION add_transaction_item(UUID, UUID, INTEGER, DECIMAL) TO olympus_app;
GRANT EXECUTE ON FUNCTION reserve_stock(UUID, UUID, VARCHAR, UUID, INTEGER, TIMESTAMPTZ, UUID) TO olympus_app;
GRANT EXECUTE ON FUNCTION release_stock_reservation(UUID) TO olympus_app;
GRANT EXECUTE ON FUNCTION process_inventory_transaction(UUID) TO olympus_app;

-- Comments for documentation
COMMENT ON TABLE inventory_transactions IS 'ACID-compliant inventory transactions for stock management';
COMMENT ON TABLE inventory_transaction_items IS 'Individual items within inventory transactions';
COMMENT ON TABLE stock_reservations IS 'Stock reservations for orders and allocations';
COMMENT ON TABLE stock_movements IS 'Comprehensive audit trail of all stock movements';
COMMENT ON TABLE inventory_valuations IS 'Inventory valuation history and cost tracking';
COMMENT ON TABLE inventory_lots IS 'Batch and lot tracking for inventory items';

COMMENT ON FUNCTION start_inventory_transaction(UUID, stock_movement_type, VARCHAR, UUID, UUID, UUID, UUID, TEXT) IS 'Start a new inventory transaction';
COMMENT ON FUNCTION process_inventory_transaction(UUID) IS 'Process an inventory transaction with ACID compliance';
COMMENT ON FUNCTION reserve_stock(UUID, UUID, VARCHAR, UUID, INTEGER, TIMESTAMPTZ, UUID) IS 'Reserve stock for orders or allocations';
COMMENT ON FUNCTION release_stock_reservation(UUID) IS 'Release a stock reservation';