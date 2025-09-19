-- ============================================================================
-- OLYMPUS CLOUD - PLATFORM SYSTEM MIGRATION
-- ============================================================================
-- Migration: 006_platform_system.sql
-- Description: Platform-specific tables for feature flags and configuration
-- Author: Claude Code Agent
-- Date: 2025-01-18
-- ============================================================================

-- Create platform schema
CREATE SCHEMA IF NOT EXISTS platform;

-- Platform-specific types
CREATE TYPE platform.feature_flag_type AS ENUM (
    'boolean',
    'string',
    'number',
    'json'
);

CREATE TYPE platform.feature_flag_status AS ENUM (
    'active',
    'inactive',
    'archived'
);

CREATE TYPE platform.rollout_strategy AS ENUM (
    'percentage',
    'user_list',
    'segment',
    'conditional'
);

-- ============================================================================
-- PLATFORM TABLES
-- ============================================================================

-- Feature flags table
CREATE TABLE platform.feature_flags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    key VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    flag_type platform.feature_flag_type NOT NULL DEFAULT 'boolean',
    status platform.feature_flag_status NOT NULL DEFAULT 'active',
    default_value JSONB NOT NULL DEFAULT 'false',
    rollout_strategy platform.rollout_strategy NOT NULL DEFAULT 'percentage',
    rollout_percentage DECIMAL(5,2) DEFAULT 0.00,
    user_segments TEXT[] DEFAULT '{}',
    conditions JSONB DEFAULT '{}',
    variants JSONB DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    is_global BOOLEAN DEFAULT false,
    starts_at TIMESTAMPTZ,
    ends_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),

    -- Ensure unique keys per tenant (global flags have tenant_id = NULL)
    CONSTRAINT unique_feature_flag_key UNIQUE(tenant_id, key),
    CONSTRAINT valid_rollout_percentage CHECK (rollout_percentage >= 0 AND rollout_percentage <= 100),
    CONSTRAINT valid_flag_key CHECK (key ~ '^[a-z0-9_-]+$')
);

-- Configuration table
CREATE TABLE platform.configurations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    key VARCHAR(255) NOT NULL,
    value JSONB NOT NULL,
    value_type VARCHAR(50) NOT NULL DEFAULT 'string',
    description TEXT,
    is_sensitive BOOLEAN DEFAULT false,
    is_readonly BOOLEAN DEFAULT false,
    validation_rules JSONB DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    category VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),

    -- Ensure unique keys per tenant
    CONSTRAINT unique_config_key UNIQUE(tenant_id, key),
    CONSTRAINT valid_config_key CHECK (key ~ '^[a-z0-9_.-]+$'),
    CONSTRAINT valid_value_type CHECK (value_type IN ('string', 'number', 'boolean', 'json', 'array'))
);

-- User feature flag overrides
CREATE TABLE platform.user_feature_flags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feature_flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    value JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),

    -- Ensure one override per user per flag
    UNIQUE(user_id, feature_flag_id)
);

-- Feature flag evaluation logs (for analytics)
CREATE TABLE platform.feature_flag_evaluations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    feature_flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    evaluated_value JSONB NOT NULL,
    context JSONB DEFAULT '{}',
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Feature flags indexes
CREATE INDEX idx_feature_flags_tenant ON platform.feature_flags(tenant_id);
CREATE INDEX idx_feature_flags_key ON platform.feature_flags(key);
CREATE INDEX idx_feature_flags_status ON platform.feature_flags(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_feature_flags_global ON platform.feature_flags(is_global) WHERE is_global = true;
CREATE INDEX idx_feature_flags_active_period ON platform.feature_flags(starts_at, ends_at) WHERE status = 'active';

-- Configuration indexes
CREATE INDEX idx_configurations_tenant ON platform.configurations(tenant_id);
CREATE INDEX idx_configurations_key ON platform.configurations(key);
CREATE INDEX idx_configurations_category ON platform.configurations(category);
CREATE INDEX idx_configurations_sensitive ON platform.configurations(is_sensitive);

-- User feature flag overrides indexes
CREATE INDEX idx_user_feature_flags_user ON platform.user_feature_flags(user_id);
CREATE INDEX idx_user_feature_flags_flag ON platform.user_feature_flags(feature_flag_id);

-- Feature flag evaluations indexes
CREATE INDEX idx_flag_evaluations_flag ON platform.feature_flag_evaluations(feature_flag_id);
CREATE INDEX idx_flag_evaluations_user ON platform.feature_flag_evaluations(user_id);
CREATE INDEX idx_flag_evaluations_tenant ON platform.feature_flag_evaluations(tenant_id);
CREATE INDEX idx_flag_evaluations_created ON platform.feature_flag_evaluations(created_at);

-- ============================================================================
-- ROW LEVEL SECURITY POLICIES
-- ============================================================================

-- Enable RLS on all platform tables
ALTER TABLE platform.feature_flags ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform.configurations ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform.user_feature_flags ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform.feature_flag_evaluations ENABLE ROW LEVEL SECURITY;

-- Feature flags policies (include global flags)
CREATE POLICY feature_flags_tenant_policy ON platform.feature_flags
    FOR ALL
    USING (
        tenant_id = current_setting('app.current_tenant_id', true)::UUID
        OR tenant_id IS NULL  -- Global flags
    );

-- Configuration policies
CREATE POLICY configurations_tenant_policy ON platform.configurations
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

-- User feature flag overrides policies
CREATE POLICY user_feature_flags_policy ON platform.user_feature_flags
    FOR ALL
    USING (
        user_id IN (
            SELECT id FROM users
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- Feature flag evaluations policies
CREATE POLICY feature_flag_evaluations_tenant_policy ON platform.feature_flag_evaluations
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Create platform schema functions first
CREATE OR REPLACE FUNCTION platform.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers
CREATE TRIGGER update_feature_flags_updated_at
    BEFORE UPDATE ON platform.feature_flags
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_configurations_updated_at
    BEFORE UPDATE ON platform.configurations
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

-- ============================================================================
-- GRANTS AND PERMISSIONS
-- ============================================================================

-- Create application role if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'olympus_app') THEN
        CREATE ROLE olympus_app;
    END IF;
END $$;

-- Grant permissions on platform schema tables
GRANT USAGE ON SCHEMA platform TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON platform.feature_flags TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON platform.configurations TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON platform.user_feature_flags TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON platform.feature_flag_evaluations TO olympus_app;

-- Grant execute permissions on platform functions
GRANT EXECUTE ON FUNCTION platform.update_updated_at_column() TO olympus_app;