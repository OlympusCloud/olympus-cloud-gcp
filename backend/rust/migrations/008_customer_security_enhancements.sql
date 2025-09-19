-- ============================================================================
-- OLYMPUS CLOUD - CUSTOMER DATA SECURITY ENHANCEMENTS
-- ============================================================================
-- Migration: 008_customer_security_enhancements.sql
-- Description: Enhanced security and privacy controls for customer data
-- Author: Claude Code Agent
-- Date: 2025-01-19
-- ============================================================================

-- Add security and privacy columns to customers table
ALTER TABLE customers
ADD COLUMN IF NOT EXISTS email_verified BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS email_verification_token VARCHAR(255),
ADD COLUMN IF NOT EXISTS email_verification_expires_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS password_reset_token VARCHAR(255),
ADD COLUMN IF NOT EXISTS password_reset_expires_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS login_attempts INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS last_login_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS last_login_ip INET,
ADD COLUMN IF NOT EXISTS privacy_consent_given BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS privacy_consent_date TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS marketing_consent_given BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS marketing_consent_date TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS gdpr_deletion_requested BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS gdpr_deletion_requested_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS data_retention_expires_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS encrypted_pii_data JSONB DEFAULT '{}',
ADD COLUMN IF NOT EXISTS data_classification VARCHAR(50) DEFAULT 'public';

-- Add data classification constraint
ALTER TABLE customers
ADD CONSTRAINT valid_data_classification
CHECK (data_classification IN ('public', 'internal', 'confidential', 'restricted'));

-- Create encrypted PII storage table for sensitive customer data
CREATE TABLE customer_encrypted_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    data_type VARCHAR(100) NOT NULL,
    encrypted_value TEXT NOT NULL,
    encryption_key_version INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    accessed_at TIMESTAMPTZ,
    access_count INTEGER DEFAULT 0,

    UNIQUE(customer_id, data_type)
);

-- Create customer audit log table for compliance
CREATE TABLE customer_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL,
    entity_type VARCHAR(50) NOT NULL,
    entity_id UUID,
    old_values JSONB,
    new_values JSONB,
    performed_by UUID, -- User or system that performed the action
    performed_by_type VARCHAR(20) DEFAULT 'user', -- 'user', 'system', 'api'
    ip_address INET,
    user_agent TEXT,
    session_id VARCHAR(255),
    compliance_reason VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create data access log for GDPR compliance
CREATE TABLE customer_data_access_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    accessed_by UUID, -- User who accessed the data
    access_type VARCHAR(50) NOT NULL, -- 'read', 'export', 'modify', 'delete'
    data_fields TEXT[] NOT NULL, -- Which fields were accessed
    purpose VARCHAR(255), -- Purpose of access
    legal_basis VARCHAR(100), -- GDPR legal basis
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create customer consent tracking table
CREATE TABLE customer_consent (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    consent_type VARCHAR(100) NOT NULL, -- 'marketing', 'analytics', 'cookies', etc.
    status BOOLEAN NOT NULL,
    consent_method VARCHAR(50) NOT NULL, -- 'explicit', 'implicit', 'opt_in', 'opt_out'
    consent_text TEXT, -- The exact consent text shown to user
    ip_address INET,
    user_agent TEXT,
    valid_from TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Row Level Security Policies for customers table
CREATE POLICY customers_tenant_isolation ON customers
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- More restrictive policy for sensitive customer operations
CREATE POLICY customers_admin_only ON customers
    FOR UPDATE USING (
        tenant_id = current_setting('app.tenant_id', true)::UUID AND
        current_setting('app.user_role', true) IN ('admin', 'customer_service')
    );

-- RLS for encrypted data table
ALTER TABLE customer_encrypted_data ENABLE ROW LEVEL SECURITY;
CREATE POLICY encrypted_data_tenant_isolation ON customer_encrypted_data
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- RLS for audit log
ALTER TABLE customer_audit_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY audit_log_tenant_isolation ON customer_audit_log
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- RLS for data access log
ALTER TABLE customer_data_access_log ENABLE ROW LEVEL SECURITY;
CREATE POLICY data_access_log_tenant_isolation ON customer_data_access_log
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- RLS for consent tracking
ALTER TABLE customer_consent ENABLE ROW LEVEL SECURITY;
CREATE POLICY consent_tenant_isolation ON customer_consent
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- Indexes for performance and security
CREATE INDEX idx_customers_email_verified ON customers(email_verified);
CREATE INDEX idx_customers_locked_until ON customers(locked_until) WHERE locked_until IS NOT NULL;
CREATE INDEX idx_customers_login_attempts ON customers(login_attempts) WHERE login_attempts > 0;
CREATE INDEX idx_customers_gdpr_deletion ON customers(gdpr_deletion_requested) WHERE gdpr_deletion_requested = true;
CREATE INDEX idx_customers_data_retention ON customers(data_retention_expires_at) WHERE data_retention_expires_at IS NOT NULL;

CREATE INDEX idx_encrypted_data_customer ON customer_encrypted_data(customer_id);
CREATE INDEX idx_encrypted_data_type ON customer_encrypted_data(data_type);
CREATE INDEX idx_encrypted_data_accessed ON customer_encrypted_data(accessed_at);

CREATE INDEX idx_audit_log_customer ON customer_audit_log(customer_id);
CREATE INDEX idx_audit_log_action ON customer_audit_log(action);
CREATE INDEX idx_audit_log_created ON customer_audit_log(created_at);
CREATE INDEX idx_audit_log_performed_by ON customer_audit_log(performed_by);

CREATE INDEX idx_data_access_customer ON customer_data_access_log(customer_id);
CREATE INDEX idx_data_access_type ON customer_data_access_log(access_type);
CREATE INDEX idx_data_access_created ON customer_data_access_log(created_at);

CREATE INDEX idx_consent_customer ON customer_consent(customer_id);
CREATE INDEX idx_consent_type ON customer_consent(consent_type);
CREATE INDEX idx_consent_status ON customer_consent(status);
CREATE INDEX idx_consent_valid ON customer_consent(valid_from, valid_until);

-- Functions for customer security operations

-- Function to encrypt sensitive customer data
CREATE OR REPLACE FUNCTION encrypt_customer_data(
    p_customer_id UUID,
    p_tenant_id UUID,
    p_data_type VARCHAR(100),
    p_value TEXT
) RETURNS UUID AS $$
DECLARE
    v_encrypted_id UUID;
BEGIN
    -- Insert or update encrypted data
    INSERT INTO customer_encrypted_data (customer_id, tenant_id, data_type, encrypted_value, accessed_at, access_count)
    VALUES (p_customer_id, p_tenant_id, p_data_type, p_value, NOW(), 1)
    ON CONFLICT (customer_id, data_type)
    DO UPDATE SET
        encrypted_value = EXCLUDED.encrypted_value,
        updated_at = NOW(),
        accessed_at = NOW(),
        access_count = customer_encrypted_data.access_count + 1
    RETURNING id INTO v_encrypted_id;

    RETURN v_encrypted_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to log customer data access
CREATE OR REPLACE FUNCTION log_customer_data_access(
    p_customer_id UUID,
    p_tenant_id UUID,
    p_accessed_by UUID,
    p_access_type VARCHAR(50),
    p_data_fields TEXT[],
    p_purpose VARCHAR(255) DEFAULT NULL,
    p_legal_basis VARCHAR(100) DEFAULT NULL,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_log_id UUID;
BEGIN
    INSERT INTO customer_data_access_log (
        customer_id, tenant_id, accessed_by, access_type, data_fields,
        purpose, legal_basis, ip_address, user_agent
    ) VALUES (
        p_customer_id, p_tenant_id, p_accessed_by, p_access_type, p_data_fields,
        p_purpose, p_legal_basis, p_ip_address, p_user_agent
    ) RETURNING id INTO v_log_id;

    RETURN v_log_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to track customer consent
CREATE OR REPLACE FUNCTION record_customer_consent(
    p_customer_id UUID,
    p_tenant_id UUID,
    p_consent_type VARCHAR(100),
    p_status BOOLEAN,
    p_consent_method VARCHAR(50),
    p_consent_text TEXT DEFAULT NULL,
    p_ip_address INET DEFAULT NULL,
    p_user_agent TEXT DEFAULT NULL,
    p_valid_until TIMESTAMPTZ DEFAULT NULL
) RETURNS UUID AS $$
DECLARE
    v_consent_id UUID;
BEGIN
    -- Revoke previous consent of the same type
    UPDATE customer_consent
    SET revoked_at = NOW()
    WHERE customer_id = p_customer_id
      AND consent_type = p_consent_type
      AND revoked_at IS NULL;

    -- Insert new consent record
    INSERT INTO customer_consent (
        customer_id, tenant_id, consent_type, status, consent_method,
        consent_text, ip_address, user_agent, valid_until
    ) VALUES (
        p_customer_id, p_tenant_id, p_consent_type, p_status, p_consent_method,
        p_consent_text, p_ip_address, p_user_agent, p_valid_until
    ) RETURNING id INTO v_consent_id;

    RETURN v_consent_id;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function for GDPR data export
CREATE OR REPLACE FUNCTION export_customer_gdpr_data(
    p_customer_id UUID,
    p_tenant_id UUID
) RETURNS JSONB AS $$
DECLARE
    v_customer_data JSONB;
    v_order_data JSONB;
    v_consent_data JSONB;
    v_result JSONB;
BEGIN
    -- Get basic customer data
    SELECT jsonb_build_object(
        'id', id,
        'email', email,
        'first_name', first_name,
        'last_name', last_name,
        'phone', phone,
        'company', company,
        'addresses', addresses,
        'created_at', created_at,
        'updated_at', updated_at,
        'total_spent', total_spent,
        'order_count', order_count,
        'last_order_at', last_order_at
    ) INTO v_customer_data
    FROM customers
    WHERE id = p_customer_id AND tenant_id = p_tenant_id;

    -- Get order data
    SELECT jsonb_agg(
        jsonb_build_object(
            'id', id,
            'status', status,
            'total_amount', total_amount,
            'created_at', created_at,
            'shipped_at', shipped_at
        )
    ) INTO v_order_data
    FROM orders
    WHERE customer_id = p_customer_id AND tenant_id = p_tenant_id;

    -- Get consent data
    SELECT jsonb_agg(
        jsonb_build_object(
            'consent_type', consent_type,
            'status', status,
            'created_at', created_at,
            'revoked_at', revoked_at
        )
    ) INTO v_consent_data
    FROM customer_consent
    WHERE customer_id = p_customer_id AND tenant_id = p_tenant_id;

    -- Combine all data
    v_result := jsonb_build_object(
        'customer', v_customer_data,
        'orders', COALESCE(v_order_data, '[]'::jsonb),
        'consents', COALESCE(v_consent_data, '[]'::jsonb),
        'exported_at', NOW()
    );

    -- Log the data export
    PERFORM log_customer_data_access(
        p_customer_id, p_tenant_id, NULL, 'export',
        ARRAY['customer_data', 'orders', 'consents'],
        'GDPR data export', 'legitimate_interest'
    );

    RETURN v_result;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to anonymize customer data (GDPR right to be forgotten)
CREATE OR REPLACE FUNCTION anonymize_customer_data(
    p_customer_id UUID,
    p_tenant_id UUID,
    p_performed_by UUID DEFAULT NULL
) RETURNS BOOLEAN AS $$
BEGIN
    -- Update customer record with anonymized data
    UPDATE customers SET
        email = 'anonymized_' || id::text || '@deleted.local',
        first_name = 'Deleted',
        last_name = 'User',
        phone = NULL,
        company = NULL,
        addresses = '[]'::jsonb,
        notes = 'Customer data anonymized per GDPR request',
        tags = '{}',
        metadata = '{"anonymized": true, "anonymized_at": "' || NOW()::text || '"}'::jsonb,
        gdpr_deletion_requested = true,
        gdpr_deletion_requested_at = NOW(),
        updated_at = NOW()
    WHERE id = p_customer_id AND tenant_id = p_tenant_id;

    -- Delete encrypted personal data
    DELETE FROM customer_encrypted_data
    WHERE customer_id = p_customer_id AND tenant_id = p_tenant_id;

    -- Log the anonymization
    INSERT INTO customer_audit_log (
        customer_id, tenant_id, action, entity_type, entity_id,
        performed_by, performed_by_type, compliance_reason
    ) VALUES (
        p_customer_id, p_tenant_id, 'anonymize', 'customer', p_customer_id,
        p_performed_by, 'user', 'GDPR right to be forgotten'
    );

    RETURN true;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Triggers for automatic audit logging
CREATE OR REPLACE FUNCTION audit_customer_changes()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'UPDATE' THEN
        INSERT INTO customer_audit_log (
            customer_id, tenant_id, action, entity_type, entity_id,
            old_values, new_values, performed_by_type
        ) VALUES (
            NEW.id, NEW.tenant_id, 'update', 'customer', NEW.id,
            to_jsonb(OLD), to_jsonb(NEW), 'system'
        );
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO customer_audit_log (
            customer_id, tenant_id, action, entity_type, entity_id,
            old_values, performed_by_type
        ) VALUES (
            OLD.id, OLD.tenant_id, 'delete', 'customer', OLD.id,
            to_jsonb(OLD), 'system'
        );
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Apply audit trigger to customers table
CREATE TRIGGER customers_audit_trigger
    AFTER UPDATE OR DELETE ON customers
    FOR EACH ROW
    EXECUTE FUNCTION audit_customer_changes();

-- Automatic timestamp updates
CREATE TRIGGER customer_encrypted_data_update_timestamp
    BEFORE UPDATE ON customer_encrypted_data
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Grant permissions
GRANT SELECT, INSERT, UPDATE ON customer_encrypted_data TO olympus_app;
GRANT SELECT, INSERT ON customer_audit_log TO olympus_app;
GRANT SELECT, INSERT ON customer_data_access_log TO olympus_app;
GRANT SELECT, INSERT, UPDATE ON customer_consent TO olympus_app;

GRANT EXECUTE ON FUNCTION encrypt_customer_data(UUID, UUID, VARCHAR, TEXT) TO olympus_app;
GRANT EXECUTE ON FUNCTION log_customer_data_access(UUID, UUID, UUID, VARCHAR, TEXT[], VARCHAR, VARCHAR, INET, TEXT) TO olympus_app;
GRANT EXECUTE ON FUNCTION record_customer_consent(UUID, UUID, VARCHAR, BOOLEAN, VARCHAR, TEXT, INET, TEXT, TIMESTAMPTZ) TO olympus_app;
GRANT EXECUTE ON FUNCTION export_customer_gdpr_data(UUID, UUID) TO olympus_app;
GRANT EXECUTE ON FUNCTION anonymize_customer_data(UUID, UUID, UUID) TO olympus_app;

-- Comments for documentation
COMMENT ON TABLE customer_encrypted_data IS 'Encrypted storage for sensitive customer PII data';
COMMENT ON TABLE customer_audit_log IS 'Comprehensive audit log for all customer data changes';
COMMENT ON TABLE customer_data_access_log IS 'Access log for GDPR compliance and data protection';
COMMENT ON TABLE customer_consent IS 'Customer consent tracking for GDPR and privacy compliance';

COMMENT ON FUNCTION encrypt_customer_data(UUID, UUID, VARCHAR, TEXT) IS 'Encrypt and store sensitive customer data';
COMMENT ON FUNCTION log_customer_data_access(UUID, UUID, UUID, VARCHAR, TEXT[], VARCHAR, VARCHAR, INET, TEXT) IS 'Log customer data access for compliance';
COMMENT ON FUNCTION record_customer_consent(UUID, UUID, VARCHAR, BOOLEAN, VARCHAR, TEXT, INET, TEXT, TIMESTAMPTZ) IS 'Record customer consent for GDPR compliance';
COMMENT ON FUNCTION export_customer_gdpr_data(UUID, UUID) IS 'Export all customer data for GDPR data portability';
COMMENT ON FUNCTION anonymize_customer_data(UUID, UUID, UUID) IS 'Anonymize customer data for GDPR right to be forgotten';