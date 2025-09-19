-- ============================================================================
-- OLYMPUS CLOUD - AUTHENTICATION SYSTEM MIGRATION
-- ============================================================================
-- Migration: 002_auth_system.sql
-- Description: Enhanced authentication tables and security features
-- Author: Claude Code Agent
-- Date: 2025-01-18
-- ============================================================================

-- Additional authentication-specific types
CREATE TYPE auth.session_status AS ENUM (
    'active',
    'expired',
    'revoked'
);

CREATE TYPE auth.token_type AS ENUM (
    'access',
    'refresh',
    'email_verification',
    'password_reset',
    'mfa'
);

CREATE TYPE auth.mfa_type AS ENUM (
    'totp',
    'sms',
    'email',
    'backup_code'
);

-- ============================================================================
-- ENHANCED AUTHENTICATION TABLES
-- ============================================================================

-- Email verification tokens
CREATE TABLE auth.email_verification_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Ensure email matches user's email or is a new email being verified
    CONSTRAINT valid_email_format CHECK (email ~ '^[^@]+@[^@]+\.[^@]+$')
);

-- Password reset tokens
CREATE TABLE auth.password_reset_tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Enhanced sessions table with additional security features
CREATE TABLE auth.user_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL UNIQUE,
    access_token_jti TEXT, -- JWT ID for access token tracking
    device_fingerprint TEXT,
    device_name VARCHAR(255),
    device_type VARCHAR(50), -- mobile, desktop, tablet, etc.
    ip_address INET,
    location JSONB, -- Geolocation data
    user_agent TEXT,
    status auth.session_status NOT NULL DEFAULT 'active',
    expires_at TIMESTAMPTZ NOT NULL,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revoked_at TIMESTAMPTZ,
    revoked_by UUID REFERENCES users(id),
    revoke_reason VARCHAR(255)
);

-- Multi-factor authentication settings
CREATE TABLE auth.user_mfa (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    mfa_type auth.mfa_type NOT NULL,
    secret_key TEXT, -- For TOTP
    phone_number VARCHAR(50), -- For SMS
    backup_codes TEXT[], -- Array of backup codes
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Ensure only one MFA method of each type per user
    UNIQUE(user_id, mfa_type)
);

-- Login attempts tracking for security
CREATE TABLE auth.login_attempts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) NOT NULL,
    tenant_slug VARCHAR(100),
    ip_address INET NOT NULL,
    user_agent TEXT,
    success BOOLEAN NOT NULL,
    failure_reason VARCHAR(255),
    user_id UUID REFERENCES users(id), -- NULL if user not found
    location JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- API keys for service-to-service authentication
CREATE TABLE auth.api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    key_hash TEXT NOT NULL UNIQUE,
    key_prefix VARCHAR(20) NOT NULL, -- First few chars for identification
    permissions TEXT[] DEFAULT '{}',
    scopes TEXT[] DEFAULT '{}',
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- User permissions cache (for performance)
CREATE TABLE auth.user_permissions (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission VARCHAR(255) NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    granted_by UUID REFERENCES users(id),

    PRIMARY KEY (user_id, permission)
);

-- Security events log
CREATE TABLE auth.security_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    tenant_id UUID REFERENCES tenants(id),
    event_type VARCHAR(100) NOT NULL,
    severity VARCHAR(20) NOT NULL DEFAULT 'info', -- info, warning, critical
    description TEXT NOT NULL,
    ip_address INET,
    user_agent TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Email verification tokens indexes
CREATE INDEX idx_email_verification_user ON auth.email_verification_tokens(user_id);
CREATE INDEX idx_email_verification_token ON auth.email_verification_tokens(token);
CREATE INDEX idx_email_verification_expires ON auth.email_verification_tokens(expires_at);

-- Password reset tokens indexes
CREATE INDEX idx_password_reset_user ON auth.password_reset_tokens(user_id);
CREATE INDEX idx_password_reset_token ON auth.password_reset_tokens(token);
CREATE INDEX idx_password_reset_expires ON auth.password_reset_tokens(expires_at);

-- User sessions indexes
CREATE INDEX idx_user_sessions_user ON auth.user_sessions(user_id);
CREATE INDEX idx_user_sessions_token ON auth.user_sessions(refresh_token_hash);
CREATE INDEX idx_user_sessions_status ON auth.user_sessions(status) WHERE status = 'active';
CREATE INDEX idx_user_sessions_expires ON auth.user_sessions(expires_at);
CREATE INDEX idx_user_sessions_activity ON auth.user_sessions(last_activity_at);

-- MFA indexes
CREATE INDEX idx_user_mfa_user ON auth.user_mfa(user_id);
CREATE INDEX idx_user_mfa_enabled ON auth.user_mfa(user_id, is_enabled) WHERE is_enabled = true;

-- Login attempts indexes
CREATE INDEX idx_login_attempts_email ON auth.login_attempts(email);
CREATE INDEX idx_login_attempts_ip ON auth.login_attempts(ip_address);
CREATE INDEX idx_login_attempts_time ON auth.login_attempts(created_at);
CREATE INDEX idx_login_attempts_success ON auth.login_attempts(success, created_at);

-- API keys indexes
CREATE INDEX idx_api_keys_user ON auth.api_keys(user_id);
CREATE INDEX idx_api_keys_hash ON auth.api_keys(key_hash);
CREATE INDEX idx_api_keys_active ON auth.api_keys(is_active) WHERE is_active = true;
CREATE INDEX idx_api_keys_expires ON auth.api_keys(expires_at);

-- User permissions indexes
CREATE INDEX idx_user_permissions_user ON auth.user_permissions(user_id);
CREATE INDEX idx_user_permissions_permission ON auth.user_permissions(permission);

-- Security events indexes
CREATE INDEX idx_security_events_user ON auth.security_events(user_id);
CREATE INDEX idx_security_events_tenant ON auth.security_events(tenant_id);
CREATE INDEX idx_security_events_type ON auth.security_events(event_type);
CREATE INDEX idx_security_events_severity ON auth.security_events(severity);
CREATE INDEX idx_security_events_time ON auth.security_events(created_at);

-- ============================================================================
-- ROW LEVEL SECURITY POLICIES
-- ============================================================================

-- Enable RLS on all auth tables
ALTER TABLE auth.email_verification_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.password_reset_tokens ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.user_sessions ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.user_mfa ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.login_attempts ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.api_keys ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.user_permissions ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.security_events ENABLE ROW LEVEL SECURITY;

-- Email verification tokens policies
CREATE POLICY email_verification_user_policy ON auth.email_verification_tokens
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Password reset tokens policies
CREATE POLICY password_reset_user_policy ON auth.password_reset_tokens
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- User sessions policies
CREATE POLICY user_sessions_policy ON auth.user_sessions
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- User MFA policies
CREATE POLICY user_mfa_policy ON auth.user_mfa
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Login attempts policies (tenant-based)
CREATE POLICY login_attempts_tenant_policy ON auth.login_attempts
    FOR ALL
    USING (
        tenant_slug = current_setting('app.current_tenant_slug', true)
        OR user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- API keys policies
CREATE POLICY api_keys_user_policy ON auth.api_keys
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- User permissions policies
CREATE POLICY user_permissions_policy ON auth.user_permissions
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Security events policies
CREATE POLICY security_events_tenant_policy ON auth.security_events
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

-- ============================================================================
-- UPDATED_AT TRIGGERS
-- ============================================================================

-- Apply updated_at triggers to tables that need them
CREATE TRIGGER update_user_mfa_updated_at
    BEFORE UPDATE ON auth.user_mfa
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_api_keys_updated_at
    BEFORE UPDATE ON auth.api_keys
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

-- ============================================================================
-- SECURITY FUNCTIONS
-- ============================================================================

-- Function to clean up expired tokens
CREATE OR REPLACE FUNCTION auth.cleanup_expired_tokens()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER := 0;
BEGIN
    -- Delete expired email verification tokens
    DELETE FROM auth.email_verification_tokens
    WHERE expires_at < CURRENT_TIMESTAMP AND used_at IS NULL;

    GET DIAGNOSTICS deleted_count = ROW_COUNT;

    -- Delete expired password reset tokens
    DELETE FROM auth.password_reset_tokens
    WHERE expires_at < CURRENT_TIMESTAMP AND used_at IS NULL;

    GET DIAGNOSTICS deleted_count = deleted_count + ROW_COUNT;

    -- Mark expired sessions as expired
    UPDATE auth.user_sessions
    SET status = 'expired'
    WHERE expires_at < CURRENT_TIMESTAMP AND status = 'active';

    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Function to revoke all user sessions
CREATE OR REPLACE FUNCTION auth.revoke_user_sessions(target_user_id UUID, revoked_by_user_id UUID DEFAULT NULL)
RETURNS INTEGER AS $$
DECLARE
    updated_count INTEGER := 0;
BEGIN
    UPDATE auth.user_sessions
    SET
        status = 'revoked',
        revoked_at = CURRENT_TIMESTAMP,
        revoked_by = revoked_by_user_id,
        revoke_reason = 'manual_revocation'
    WHERE user_id = target_user_id AND status = 'active';

    GET DIAGNOSTICS updated_count = ROW_COUNT;

    RETURN updated_count;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- GRANTS AND PERMISSIONS
-- ============================================================================

-- Grant permissions on auth schema tables
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.email_verification_tokens TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.password_reset_tokens TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.user_sessions TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.user_mfa TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.login_attempts TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.api_keys TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.user_permissions TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON auth.security_events TO olympus_app;

-- Grant execute permissions on auth functions
GRANT EXECUTE ON FUNCTION auth.cleanup_expired_tokens() TO olympus_app;
GRANT EXECUTE ON FUNCTION auth.revoke_user_sessions(UUID, UUID) TO olympus_app;