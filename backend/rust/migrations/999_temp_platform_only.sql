-- Temporary migration to create only platform tables for testing
-- This bypasses the complex migration issues for now

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create platform schema
CREATE SCHEMA IF NOT EXISTS platform;

-- Basic tenant table (minimal for foreign keys)
CREATE TABLE IF NOT EXISTS tenants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Basic users table (minimal for foreign keys)
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

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

-- Create basic indexes
CREATE INDEX idx_feature_flags_tenant ON platform.feature_flags(tenant_id);
CREATE INDEX idx_feature_flags_key ON platform.feature_flags(key);
CREATE INDEX idx_configurations_tenant ON platform.configurations(tenant_id);
CREATE INDEX idx_configurations_key ON platform.configurations(key);

-- Insert test tenant and user
INSERT INTO tenants (id, slug, name) VALUES
    ('00000000-0000-0000-0000-000000000001', 'test-tenant', 'Test Tenant')
ON CONFLICT (slug) DO NOTHING;

INSERT INTO users (id, tenant_id, email) VALUES
    ('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'test@example.com')
ON CONFLICT DO NOTHING;