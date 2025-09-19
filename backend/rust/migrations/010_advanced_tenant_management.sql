-- ============================================================================
-- OLYMPUS CLOUD - ADVANCED TENANT MANAGEMENT
-- ============================================================================
-- Migration: 010_advanced_tenant_management.sql
-- Description: Advanced tenant management, isolation, quotas, and monitoring
-- Author: Claude Code Agent
-- Date: 2025-01-19
-- ============================================================================

-- Add new enum types for advanced tenant management
CREATE TYPE tenant_status AS ENUM ('active', 'suspended', 'inactive', 'pending_setup', 'migrating', 'archived');
CREATE TYPE subscription_tier AS ENUM ('trial', 'basic', 'professional', 'enterprise', 'custom');
CREATE TYPE billing_cycle AS ENUM ('monthly', 'quarterly', 'yearly', 'custom');

-- Extend tenants table with advanced features
ALTER TABLE tenants
ADD COLUMN IF NOT EXISTS display_name VARCHAR(200),
ADD COLUMN IF NOT EXISTS status tenant_status DEFAULT 'pending_setup',
ADD COLUMN IF NOT EXISTS subscription_tier subscription_tier DEFAULT 'trial',
ADD COLUMN IF NOT EXISTS billing_cycle billing_cycle DEFAULT 'monthly',
ADD COLUMN IF NOT EXISTS domain VARCHAR(255),
ADD COLUMN IF NOT EXISTS logo_url VARCHAR(500),
ADD COLUMN IF NOT EXISTS primary_color VARCHAR(7),
ADD COLUMN IF NOT EXISTS secondary_color VARCHAR(7),
ADD COLUMN IF NOT EXISTS timezone VARCHAR(50) DEFAULT 'UTC',
ADD COLUMN IF NOT EXISTS locale VARCHAR(10) DEFAULT 'en_US',
ADD COLUMN IF NOT EXISTS currency VARCHAR(3) DEFAULT 'USD',
ADD COLUMN IF NOT EXISTS billing_email VARCHAR(255),
ADD COLUMN IF NOT EXISTS technical_contact_email VARCHAR(255),
ADD COLUMN IF NOT EXISTS data_region VARCHAR(20) DEFAULT 'us-east-1',
ADD COLUMN IF NOT EXISTS compliance_requirements TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS trial_ends_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS subscription_starts_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS subscription_ends_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS next_billing_date TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS feature_flags JSONB DEFAULT '{}',
ADD COLUMN IF NOT EXISTS custom_features TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS onboarding_completed BOOLEAN DEFAULT false,
ADD COLUMN IF NOT EXISTS onboarding_step VARCHAR(50),
ADD COLUMN IF NOT EXISTS last_activity_at TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS health_score DECIMAL(5,2),
ADD COLUMN IF NOT EXISTS created_by UUID REFERENCES users(id),
ADD COLUMN IF NOT EXISTS updated_by UUID REFERENCES users(id);

-- Create tenant resources table
CREATE TABLE tenant_resources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    resource_type VARCHAR(100) NOT NULL,
    resource_name VARCHAR(200) NOT NULL,
    resource_config JSONB DEFAULT '{}',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, resource_type, resource_name)
);

-- Create tenant quotas table
CREATE TABLE tenant_quotas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    quota_type VARCHAR(100) NOT NULL,
    limit_value BIGINT NOT NULL,
    current_usage BIGINT DEFAULT 0,
    reset_interval VARCHAR(20) DEFAULT 'monthly', -- daily, weekly, monthly, yearly
    last_reset TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    next_reset TIMESTAMPTZ NOT NULL,
    is_hard_limit BOOLEAN DEFAULT true,
    warning_threshold DECIMAL(5,2) DEFAULT 80.0, -- Percentage
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, quota_type),
    CONSTRAINT valid_warning_threshold CHECK (warning_threshold >= 0 AND warning_threshold <= 100)
);

-- Create tenant isolation table
CREATE TABLE tenant_isolation (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    database_schema VARCHAR(100) NOT NULL,
    file_storage_prefix VARCHAR(200) NOT NULL,
    cache_namespace VARCHAR(100) NOT NULL,
    encryption_key_id VARCHAR(200) NOT NULL,
    network_isolation_config JSONB DEFAULT '{}',
    backup_config JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id),
    UNIQUE(database_schema),
    UNIQUE(file_storage_prefix),
    UNIQUE(cache_namespace)
);

-- Create tenant health checks table
CREATE TABLE tenant_health_checks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    check_name VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL, -- healthy, warning, critical
    last_check TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    response_time_ms INTEGER,
    error_count INTEGER DEFAULT 0,
    details JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_status CHECK (status IN ('healthy', 'warning', 'critical'))
);

-- Create tenant analytics table
CREATE TABLE tenant_analytics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    active_users INTEGER DEFAULT 0,
    total_requests BIGINT DEFAULT 0,
    error_rate DECIMAL(5,4) DEFAULT 0.0000,
    avg_response_time DECIMAL(8,2) DEFAULT 0.00,
    feature_usage JSONB DEFAULT '{}',
    revenue_generated DECIMAL(12,2),
    storage_consumed DECIMAL(10,3) DEFAULT 0.000, -- GB
    bandwidth_consumed DECIMAL(10,3) DEFAULT 0.000, -- GB
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, period_start, period_end),
    CONSTRAINT valid_period CHECK (period_end > period_start),
    CONSTRAINT valid_error_rate CHECK (error_rate >= 0 AND error_rate <= 1)
);

-- ============================================================================
-- ADVANCED FEATURE FLAGS TABLES
-- ============================================================================

-- Add feature flag dependency types
CREATE TYPE dependency_type AS ENUM ('requires', 'conflicts', 'prerequisite', 'mutex');
CREATE TYPE ab_test_status AS ENUM ('draft', 'running', 'paused', 'completed', 'cancelled');

-- Feature flag dependencies
CREATE TABLE feature_flag_dependencies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    depends_on_flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    dependency_type dependency_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(flag_id, depends_on_flag_id),
    CONSTRAINT no_self_dependency CHECK (flag_id != depends_on_flag_id)
);

-- A/B Testing for feature flags
CREATE TABLE feature_flag_ab_tests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    test_name VARCHAR(200) NOT NULL,
    hypothesis TEXT NOT NULL,
    variants JSONB NOT NULL DEFAULT '{}',
    traffic_allocation JSONB NOT NULL DEFAULT '{}',
    success_metrics TEXT[] DEFAULT '{}',
    statistical_significance DECIMAL(5,4),
    confidence_interval DECIMAL(5,4),
    test_duration_days INTEGER NOT NULL,
    started_at TIMESTAMPTZ,
    ended_at TIMESTAMPTZ,
    results JSONB,
    status ab_test_status DEFAULT 'draft',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_duration CHECK (test_duration_days > 0 AND test_duration_days <= 365),
    CONSTRAINT valid_test_period CHECK (ended_at IS NULL OR ended_at > started_at)
);

-- User segments for targeted feature flags
CREATE TABLE feature_flag_segments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    conditions JSONB NOT NULL DEFAULT '{}',
    user_count BIGINT,
    is_dynamic BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, name)
);

-- Feature flag rules for complex targeting
CREATE TABLE feature_flag_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    rule_name VARCHAR(100) NOT NULL,
    conditions JSONB NOT NULL DEFAULT '{}',
    value JSONB NOT NULL,
    rollout_percentage DECIMAL(5,2) DEFAULT 100.0,
    target_segments UUID[] DEFAULT '{}',
    priority INTEGER DEFAULT 1000,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT valid_rollout_percentage CHECK (rollout_percentage >= 0 AND rollout_percentage <= 100)
);

-- Feature flag change history
CREATE TABLE feature_flag_history (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    changed_by UUID NOT NULL REFERENCES users(id),
    change_type VARCHAR(50) NOT NULL,
    old_value JSONB,
    new_value JSONB,
    reason TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Feature flag environments for deployment pipelines
CREATE TABLE feature_flag_environments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    is_production BOOLEAN DEFAULT false,
    requires_approval BOOLEAN DEFAULT false,
    approval_workflow JSONB,
    promotion_rules JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(tenant_id, name)
);

-- Feature flag environment configurations
CREATE TABLE feature_flag_environment_configs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    flag_id UUID NOT NULL REFERENCES platform.feature_flags(id) ON DELETE CASCADE,
    environment_id UUID NOT NULL REFERENCES feature_flag_environments(id) ON DELETE CASCADE,
    status platform.feature_flag_status DEFAULT 'inactive',
    value JSONB NOT NULL,
    rollout_strategy platform.rollout_strategy DEFAULT 'all_users',
    rollout_percentage DECIMAL(5,2) DEFAULT 0.0,
    target_rules UUID[] DEFAULT '{}',
    last_modified_by UUID NOT NULL REFERENCES users(id),
    last_modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(flag_id, environment_id)
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Tenant indexes
CREATE INDEX idx_tenants_status ON tenants(status) WHERE status != 'active';
CREATE INDEX idx_tenants_subscription_tier ON tenants(subscription_tier);
CREATE INDEX idx_tenants_domain ON tenants(domain) WHERE domain IS NOT NULL;
CREATE INDEX idx_tenants_trial_ends ON tenants(trial_ends_at) WHERE trial_ends_at IS NOT NULL;
CREATE INDEX idx_tenants_health_score ON tenants(health_score) WHERE health_score IS NOT NULL;
CREATE INDEX idx_tenants_last_activity ON tenants(last_activity_at) WHERE last_activity_at IS NOT NULL;

-- Tenant resources indexes
CREATE INDEX idx_tenant_resources_tenant ON tenant_resources(tenant_id);
CREATE INDEX idx_tenant_resources_type ON tenant_resources(resource_type);
CREATE INDEX idx_tenant_resources_active ON tenant_resources(is_active) WHERE is_active = true;

-- Tenant quotas indexes
CREATE INDEX idx_tenant_quotas_tenant ON tenant_quotas(tenant_id);
CREATE INDEX idx_tenant_quotas_type ON tenant_quotas(quota_type);
CREATE INDEX idx_tenant_quotas_usage ON tenant_quotas(current_usage, limit_value) WHERE current_usage > 0;
CREATE INDEX idx_tenant_quotas_next_reset ON tenant_quotas(next_reset) WHERE next_reset <= NOW() + INTERVAL '1 day';

-- Tenant isolation indexes
CREATE INDEX idx_tenant_isolation_tenant ON tenant_isolation(tenant_id);
CREATE INDEX idx_tenant_isolation_schema ON tenant_isolation(database_schema);

-- Health checks indexes
CREATE INDEX idx_health_checks_tenant ON tenant_health_checks(tenant_id);
CREATE INDEX idx_health_checks_status ON tenant_health_checks(status) WHERE status != 'healthy';
CREATE INDEX idx_health_checks_last_check ON tenant_health_checks(last_check);

-- Analytics indexes
CREATE INDEX idx_tenant_analytics_tenant ON tenant_analytics(tenant_id);
CREATE INDEX idx_tenant_analytics_period ON tenant_analytics(period_start, period_end);
CREATE INDEX idx_tenant_analytics_created ON tenant_analytics(created_at);

-- Feature flag advanced indexes
CREATE INDEX idx_flag_dependencies_flag ON feature_flag_dependencies(flag_id);
CREATE INDEX idx_flag_dependencies_depends_on ON feature_flag_dependencies(depends_on_flag_id);
CREATE INDEX idx_flag_dependencies_type ON feature_flag_dependencies(dependency_type);

CREATE INDEX idx_flag_ab_tests_flag ON feature_flag_ab_tests(flag_id);
CREATE INDEX idx_flag_ab_tests_status ON feature_flag_ab_tests(status);
CREATE INDEX idx_flag_ab_tests_dates ON feature_flag_ab_tests(started_at, ended_at);

CREATE INDEX idx_flag_segments_tenant ON feature_flag_segments(tenant_id);
CREATE INDEX idx_flag_segments_dynamic ON feature_flag_segments(is_dynamic) WHERE is_dynamic = true;

CREATE INDEX idx_flag_rules_flag ON feature_flag_rules(flag_id);
CREATE INDEX idx_flag_rules_priority ON feature_flag_rules(priority);
CREATE INDEX idx_flag_rules_active ON feature_flag_rules(is_active) WHERE is_active = true;

CREATE INDEX idx_flag_history_flag ON feature_flag_history(flag_id);
CREATE INDEX idx_flag_history_changed_by ON feature_flag_history(changed_by);
CREATE INDEX idx_flag_history_created ON feature_flag_history(created_at);

CREATE INDEX idx_flag_environments_tenant ON feature_flag_environments(tenant_id);
CREATE INDEX idx_flag_environments_production ON feature_flag_environments(is_production) WHERE is_production = true;

CREATE INDEX idx_flag_env_configs_flag ON feature_flag_environment_configs(flag_id);
CREATE INDEX idx_flag_env_configs_environment ON feature_flag_environment_configs(environment_id);

-- ============================================================================
-- ROW LEVEL SECURITY POLICIES
-- ============================================================================

-- Enable RLS on new tables
ALTER TABLE tenant_resources ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_quotas ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_isolation ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_health_checks ENABLE ROW LEVEL SECURITY;
ALTER TABLE tenant_analytics ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_dependencies ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_ab_tests ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_segments ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_rules ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_history ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_environments ENABLE ROW LEVEL SECURITY;
ALTER TABLE feature_flag_environment_configs ENABLE ROW LEVEL SECURITY;

-- Tenant isolation policies
CREATE POLICY tenant_resources_isolation ON tenant_resources
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY tenant_quotas_isolation ON tenant_quotas
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY tenant_isolation_isolation ON tenant_isolation
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY tenant_health_checks_isolation ON tenant_health_checks
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY tenant_analytics_isolation ON tenant_analytics
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- Feature flag policies (these work with platform schema flags)
CREATE POLICY flag_segments_isolation ON feature_flag_segments
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

CREATE POLICY flag_environments_isolation ON feature_flag_environments
    FOR ALL USING (tenant_id = current_setting('app.tenant_id', true)::UUID);

-- Admin access policies for feature flag management
CREATE POLICY flag_dependencies_admin ON feature_flag_dependencies
    FOR ALL USING (current_setting('app.user_role', true) IN ('admin', 'feature_manager'));

CREATE POLICY flag_ab_tests_admin ON feature_flag_ab_tests
    FOR ALL USING (current_setting('app.user_role', true) IN ('admin', 'feature_manager'));

CREATE POLICY flag_rules_admin ON feature_flag_rules
    FOR ALL USING (current_setting('app.user_role', true) IN ('admin', 'feature_manager'));

CREATE POLICY flag_history_read_only ON feature_flag_history
    FOR SELECT USING (true);

CREATE POLICY flag_env_configs_admin ON feature_flag_environment_configs
    FOR ALL USING (current_setting('app.user_role', true) IN ('admin', 'feature_manager'));

-- ============================================================================
-- FUNCTIONS FOR TENANT MANAGEMENT
-- ============================================================================

-- Function to update tenant usage metrics
CREATE OR REPLACE FUNCTION update_tenant_usage(
    p_tenant_id UUID,
    p_quota_type VARCHAR(100),
    p_amount BIGINT
) RETURNS BOOLEAN AS $$
DECLARE
    v_current_usage BIGINT;
    v_limit_value BIGINT;
    v_is_hard_limit BOOLEAN;
BEGIN
    -- Get current quota information
    SELECT current_usage, limit_value, is_hard_limit
    INTO v_current_usage, v_limit_value, v_is_hard_limit
    FROM tenant_quotas
    WHERE tenant_id = p_tenant_id AND quota_type = p_quota_type;

    -- If quota doesn't exist, create it with default values
    IF NOT FOUND THEN
        INSERT INTO tenant_quotas (tenant_id, quota_type, limit_value, current_usage, next_reset)
        VALUES (p_tenant_id, p_quota_type, 1000000, p_amount, NOW() + INTERVAL '1 month');
        RETURN true;
    END IF;

    -- Check if update would exceed hard limit
    IF v_is_hard_limit AND (v_current_usage + p_amount) > v_limit_value THEN
        RETURN false;
    END IF;

    -- Update usage
    UPDATE tenant_quotas
    SET current_usage = current_usage + p_amount,
        updated_at = NOW()
    WHERE tenant_id = p_tenant_id AND quota_type = p_quota_type;

    RETURN true;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to reset quota usage
CREATE OR REPLACE FUNCTION reset_tenant_quota(
    p_tenant_id UUID,
    p_quota_type VARCHAR(100)
) RETURNS VOID AS $$
BEGIN
    UPDATE tenant_quotas
    SET current_usage = 0,
        last_reset = NOW(),
        next_reset = CASE
            WHEN reset_interval = 'daily' THEN NOW() + INTERVAL '1 day'
            WHEN reset_interval = 'weekly' THEN NOW() + INTERVAL '1 week'
            WHEN reset_interval = 'monthly' THEN NOW() + INTERVAL '1 month'
            WHEN reset_interval = 'yearly' THEN NOW() + INTERVAL '1 year'
            ELSE NOW() + INTERVAL '1 month'
        END,
        updated_at = NOW()
    WHERE tenant_id = p_tenant_id AND quota_type = p_quota_type;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Function to calculate tenant health score
CREATE OR REPLACE FUNCTION calculate_tenant_health_score(
    p_tenant_id UUID
) RETURNS DECIMAL(5,2) AS $$
DECLARE
    v_score DECIMAL(5,2) := 100.0;
    v_critical_count INTEGER;
    v_warning_count INTEGER;
    v_avg_response_time DECIMAL(8,2);
BEGIN
    -- Count health check issues
    SELECT
        COUNT(*) FILTER (WHERE status = 'critical'),
        COUNT(*) FILTER (WHERE status = 'warning'),
        AVG(response_time_ms)
    INTO v_critical_count, v_warning_count, v_avg_response_time
    FROM tenant_health_checks
    WHERE tenant_id = p_tenant_id
      AND last_check > NOW() - INTERVAL '1 hour';

    -- Deduct points for health issues
    v_score := v_score - (v_critical_count * 30) - (v_warning_count * 10);

    -- Deduct points for slow response times
    IF v_avg_response_time > 1000 THEN
        v_score := v_score - 20;
    ELSIF v_avg_response_time > 500 THEN
        v_score := v_score - 10;
    ELSIF v_avg_response_time > 100 THEN
        v_score := v_score - 5;
    END IF;

    -- Ensure score doesn't go below 0
    v_score := GREATEST(v_score, 0.0);

    -- Update tenant health score
    UPDATE tenants
    SET health_score = v_score,
        updated_at = NOW()
    WHERE id = p_tenant_id;

    RETURN v_score;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Auto-update timestamps
CREATE TRIGGER tenant_resources_update_timestamp
    BEFORE UPDATE ON tenant_resources
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER tenant_quotas_update_timestamp
    BEFORE UPDATE ON tenant_quotas
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER tenant_isolation_update_timestamp
    BEFORE UPDATE ON tenant_isolation
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER feature_flag_ab_tests_update_timestamp
    BEFORE UPDATE ON feature_flag_ab_tests
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER feature_flag_segments_update_timestamp
    BEFORE UPDATE ON feature_flag_segments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER feature_flag_rules_update_timestamp
    BEFORE UPDATE ON feature_flag_rules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER feature_flag_environments_update_timestamp
    BEFORE UPDATE ON feature_flag_environments
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- ============================================================================
-- PERMISSIONS
-- ============================================================================

-- Grant permissions to application user
GRANT SELECT, INSERT, UPDATE, DELETE ON tenant_resources TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON tenant_quotas TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON tenant_isolation TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON tenant_health_checks TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON tenant_analytics TO olympus_app;

GRANT SELECT, INSERT, UPDATE, DELETE ON feature_flag_dependencies TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON feature_flag_ab_tests TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON feature_flag_segments TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON feature_flag_rules TO olympus_app;
GRANT SELECT, INSERT ON feature_flag_history TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON feature_flag_environments TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON feature_flag_environment_configs TO olympus_app;

-- Grant function permissions
GRANT EXECUTE ON FUNCTION update_tenant_usage(UUID, VARCHAR, BIGINT) TO olympus_app;
GRANT EXECUTE ON FUNCTION reset_tenant_quota(UUID, VARCHAR) TO olympus_app;
GRANT EXECUTE ON FUNCTION calculate_tenant_health_score(UUID) TO olympus_app;

-- ============================================================================
-- COMMENTS
-- ============================================================================

COMMENT ON TABLE tenant_resources IS 'Tenant-specific resource configurations and allocations';
COMMENT ON TABLE tenant_quotas IS 'Usage quotas and limits for each tenant';
COMMENT ON TABLE tenant_isolation IS 'Data isolation and security configurations per tenant';
COMMENT ON TABLE tenant_health_checks IS 'Health monitoring results for each tenant';
COMMENT ON TABLE tenant_analytics IS 'Analytics and usage metrics aggregated per tenant';

COMMENT ON TABLE feature_flag_dependencies IS 'Dependencies and relationships between feature flags';
COMMENT ON TABLE feature_flag_ab_tests IS 'A/B testing configurations and results for feature flags';
COMMENT ON TABLE feature_flag_segments IS 'User segments for targeted feature flag rollouts';
COMMENT ON TABLE feature_flag_rules IS 'Complex targeting rules for feature flag evaluation';
COMMENT ON TABLE feature_flag_history IS 'Audit trail of all feature flag changes';
COMMENT ON TABLE feature_flag_environments IS 'Environment configurations for feature flag deployment';
COMMENT ON TABLE feature_flag_environment_configs IS 'Feature flag configurations per environment';

COMMENT ON FUNCTION update_tenant_usage(UUID, VARCHAR, BIGINT) IS 'Update tenant quota usage and enforce limits';
COMMENT ON FUNCTION reset_tenant_quota(UUID, VARCHAR) IS 'Reset tenant quota usage based on interval';
COMMENT ON FUNCTION calculate_tenant_health_score(UUID) IS 'Calculate overall health score for a tenant';