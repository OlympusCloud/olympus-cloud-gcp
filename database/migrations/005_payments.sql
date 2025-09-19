-- ============================================================================
-- OLYMPUS CLOUD - PAYMENT PROCESSING TABLES
-- ============================================================================
-- Migration: 005_payments.sql
-- Description: Payment processing, payment methods, and refunds
-- Author: Claude Code Agent
-- Date: 2025-01-19
-- ============================================================================

-- Create payment enums
CREATE TYPE payment_gateway AS ENUM (
    'stripe',
    'square',
    'paypal',
    'manual',
    'cash',
    'card'
);

CREATE TYPE payment_transaction_status AS ENUM (
    'pending',
    'processing',
    'authorized',
    'captured',
    'completed',
    'failed',
    'cancelled',
    'refunded',
    'partially_refunded'
);

CREATE TYPE payment_type AS ENUM (
    'sale',
    'authorization',
    'capture',
    'refund',
    'partial_refund',
    'void'
);

CREATE TYPE payment_method_type AS ENUM (
    'card',
    'bank_account',
    'cash',
    'check',
    'gift_card',
    'wallet',
    'other'
);

CREATE TYPE refund_status AS ENUM (
    'pending',
    'processing',
    'completed',
    'failed',
    'cancelled'
);

-- ============================================================================
-- PAYMENT METHODS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.payment_methods (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customer.customers(id) ON DELETE CASCADE,
    gateway payment_gateway NOT NULL,
    gateway_method_id VARCHAR(255),
    method_type payment_method_type NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    last_four VARCHAR(4),
    brand VARCHAR(50),
    exp_month INTEGER CHECK (exp_month >= 1 AND exp_month <= 12),
    exp_year INTEGER CHECK (exp_year >= 2020 AND exp_year <= 2100),
    is_default BOOLEAN DEFAULT false,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_payment_methods_tenant (tenant_id),
    INDEX idx_payment_methods_customer (customer_id),
    INDEX idx_payment_methods_gateway (gateway),
    INDEX idx_payment_methods_default (tenant_id, customer_id, is_default) WHERE is_default = true
);

-- ============================================================================
-- PAYMENTS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES commerce.orders(id) ON DELETE CASCADE,
    payment_method_id UUID REFERENCES commerce.payment_methods(id),
    gateway payment_gateway NOT NULL,
    gateway_payment_id VARCHAR(255),
    gateway_customer_id VARCHAR(255),
    amount DECIMAL(10, 2) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status payment_transaction_status NOT NULL DEFAULT 'pending',
    payment_type payment_type NOT NULL,
    metadata JSONB,
    error_message TEXT,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES auth.users(id),
    updated_by UUID REFERENCES auth.users(id),

    -- Indexes
    INDEX idx_payments_tenant (tenant_id),
    INDEX idx_payments_order (order_id),
    INDEX idx_payments_status (status),
    INDEX idx_payments_gateway (gateway),
    INDEX idx_payments_gateway_payment_id (gateway_payment_id),
    INDEX idx_payments_created_at (created_at),
    INDEX idx_payments_tenant_status_created (tenant_id, status, created_at DESC)
);

-- ============================================================================
-- REFUNDS TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.refunds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    payment_id UUID NOT NULL REFERENCES commerce.payments(id) ON DELETE CASCADE,
    gateway_refund_id VARCHAR(255),
    amount DECIMAL(10, 2) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status refund_status NOT NULL DEFAULT 'pending',
    reason TEXT NOT NULL,
    metadata JSONB,
    error_message TEXT,
    processed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    created_by UUID REFERENCES auth.users(id),
    updated_by UUID REFERENCES auth.users(id),

    -- Indexes
    INDEX idx_refunds_tenant (tenant_id),
    INDEX idx_refunds_payment (payment_id),
    INDEX idx_refunds_status (status),
    INDEX idx_refunds_created_at (created_at),

    -- Constraints
    CONSTRAINT refund_amount_valid CHECK (amount > 0)
);

-- ============================================================================
-- PAYMENT TRANSACTIONS TABLE (Audit Log)
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.payment_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    payment_id UUID NOT NULL REFERENCES commerce.payments(id) ON DELETE CASCADE,
    transaction_type VARCHAR(50) NOT NULL,
    amount DECIMAL(10, 2),
    status VARCHAR(50) NOT NULL,
    gateway_response JSONB,
    error_details JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_payment_transactions_payment (payment_id),
    INDEX idx_payment_transactions_created (created_at),
    INDEX idx_payment_transactions_type (transaction_type)
);

-- ============================================================================
-- PAYMENT RECONCILIATION TABLE
-- ============================================================================
CREATE TABLE IF NOT EXISTS commerce.payment_reconciliation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    payment_id UUID NOT NULL REFERENCES commerce.payments(id) ON DELETE CASCADE,
    gateway payment_gateway NOT NULL,
    gateway_transaction_id VARCHAR(255),
    gateway_amount DECIMAL(10, 2),
    gateway_fee DECIMAL(10, 2),
    net_amount DECIMAL(10, 2),
    reconciliation_date DATE,
    reconciliation_status VARCHAR(50) NOT NULL DEFAULT 'pending',
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),

    -- Indexes
    INDEX idx_payment_reconciliation_tenant (tenant_id),
    INDEX idx_payment_reconciliation_payment (payment_id),
    INDEX idx_payment_reconciliation_date (reconciliation_date),
    INDEX idx_payment_reconciliation_status (reconciliation_status)
);

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Update timestamp trigger for payment_methods
CREATE TRIGGER update_payment_methods_updated_at
    BEFORE UPDATE ON commerce.payment_methods
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Update timestamp trigger for payments
CREATE TRIGGER update_payments_updated_at
    BEFORE UPDATE ON commerce.payments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Update timestamp trigger for refunds
CREATE TRIGGER update_refunds_updated_at
    BEFORE UPDATE ON commerce.refunds
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Update timestamp trigger for payment_reconciliation
CREATE TRIGGER update_payment_reconciliation_updated_at
    BEFORE UPDATE ON commerce.payment_reconciliation
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

-- Enable RLS on all payment tables
ALTER TABLE commerce.payment_methods ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.payments ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.refunds ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.payment_transactions ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.payment_reconciliation ENABLE ROW LEVEL SECURITY;

-- Payment Methods Policies
CREATE POLICY tenant_isolation_payment_methods ON commerce.payment_methods
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Payments Policies
CREATE POLICY tenant_isolation_payments ON commerce.payments
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Refunds Policies
CREATE POLICY tenant_isolation_refunds ON commerce.refunds
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Payment Transactions Policies
CREATE POLICY tenant_isolation_payment_transactions ON commerce.payment_transactions
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- Payment Reconciliation Policies
CREATE POLICY tenant_isolation_payment_reconciliation ON commerce.payment_reconciliation
    FOR ALL
    USING (tenant_id = current_setting('app.tenant_id')::UUID);

-- ============================================================================
-- GRANTS
-- ============================================================================

-- Grant permissions to application role
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA commerce TO app_user;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA commerce TO app_user;

-- ============================================================================
-- SEED DATA (Optional)
-- ============================================================================

-- Add sample payment methods for development
-- INSERT INTO commerce.payment_methods (tenant_id, display_name, gateway, method_type, is_default)
-- VALUES
--     ((SELECT id FROM platform.tenants LIMIT 1), 'Cash', 'cash', 'cash', true),
--     ((SELECT id FROM platform.tenants LIMIT 1), 'Credit Card', 'stripe', 'card', false);