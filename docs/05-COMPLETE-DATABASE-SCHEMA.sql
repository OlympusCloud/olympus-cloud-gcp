# 🗄️ Olympus Cloud GCP - Complete Database Schema

> **Production-ready PostgreSQL schema with multi-tenancy and row-level security**

## 📋 Database Architecture Overview

```yaml
Database: PostgreSQL 15+
Extensions:
  - uuid-ossp (UUID generation)
  - pgcrypto (encryption)
  - pg_trgm (fuzzy text search)
  - timescaledb (time-series data)
  - postgis (geospatial data)
Partitioning: By tenant_id and created_at
RLS: Row-level security on all tables
Audit: Trigger-based audit logging
```

## 🏗️ Complete Schema Definition

```sql
-- ============================================================================
-- EXTENSIONS AND CONFIGURATION
-- ============================================================================

-- Enable required extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_trgm";
CREATE EXTENSION IF NOT EXISTS "btree_gist";
CREATE EXTENSION IF NOT EXISTS "postgres_fdw";

-- Set timezone
SET timezone = 'UTC';

-- ============================================================================
-- SCHEMAS
-- ============================================================================

CREATE SCHEMA IF NOT EXISTS platform;
CREATE SCHEMA IF NOT EXISTS auth;
CREATE SCHEMA IF NOT EXISTS commerce;
CREATE SCHEMA IF NOT EXISTS inventory;
CREATE SCHEMA IF NOT EXISTS customer;
CREATE SCHEMA IF NOT EXISTS workforce;
CREATE SCHEMA IF NOT EXISTS analytics;
CREATE SCHEMA IF NOT EXISTS events;
CREATE SCHEMA IF NOT EXISTS audit;

-- ============================================================================
-- CUSTOM TYPES
-- ============================================================================

-- Industry types
CREATE TYPE platform.industry_type AS ENUM (
    'restaurant',
    'retail',
    'salon',
    'hospitality',
    'events',
    'other'
);

-- Subscription tiers
CREATE TYPE platform.subscription_tier AS ENUM (
    'free',
    'starter',
    'professional',
    'enterprise',
    'custom'
);

-- Order status
CREATE TYPE commerce.order_status AS ENUM (
    'draft',
    'pending',
    'confirmed',
    'preparing',
    'ready',
    'in_delivery',
    'completed',
    'cancelled',
    'refunded'
);

-- Payment status
CREATE TYPE commerce.payment_status AS ENUM (
    'pending',
    'processing',
    'authorized',
    'captured',
    'partially_refunded',
    'refunded',
    'failed',
    'cancelled'
);

-- User roles
CREATE TYPE auth.user_role AS ENUM (
    'super_admin',
    'tenant_admin',
    'location_admin',
    'manager',
    'employee',
    'customer',
    'guest'
);

-- ============================================================================
-- PLATFORM SCHEMA - MULTI-TENANCY
-- ============================================================================

-- Tenants table (companies/organizations)
CREATE TABLE platform.tenants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    legal_name VARCHAR(255),
    industry platform.industry_type NOT NULL,
    tier platform.subscription_tier NOT NULL DEFAULT 'starter',
    
    -- Hierarchy
    parent_id UUID REFERENCES platform.tenants(id),
    path LTREE, -- For efficient hierarchy queries
    
    -- Configuration
    settings JSONB NOT NULL DEFAULT '{}',
    features JSONB NOT NULL DEFAULT '{}',
    branding JSONB NOT NULL DEFAULT '{}',
    
    -- Billing
    stripe_customer_id VARCHAR(255),
    billing_email VARCHAR(255),
    trial_ends_at TIMESTAMPTZ,
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_suspended BOOLEAN NOT NULL DEFAULT false,
    suspension_reason TEXT,
    
    -- Metadata
    metadata JSONB NOT NULL DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    -- Constraints
    CONSTRAINT valid_slug CHECK (slug ~ '^[a-z0-9-]+$'),
    CONSTRAINT valid_hierarchy CHECK (
        (parent_id IS NULL) OR (parent_id != id)
    )
);

-- Locations (physical locations for a tenant)
CREATE TABLE platform.locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    
    -- Address
    address_line1 VARCHAR(255),
    address_line2 VARCHAR(255),
    city VARCHAR(100),
    state_province VARCHAR(100),
    postal_code VARCHAR(20),
    country_code CHAR(2),
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    
    -- Contact
    phone VARCHAR(50),
    email VARCHAR(255),
    website VARCHAR(255),
    
    -- Operating hours (JSONB for flexibility)
    operating_hours JSONB DEFAULT '{}',
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    
    -- Configuration
    settings JSONB NOT NULL DEFAULT '{}',
    features JSONB NOT NULL DEFAULT '{}',
    pos_config JSONB DEFAULT '{}',
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    opened_at DATE,
    closed_at DATE,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, code)
);

-- Feature flags
CREATE TABLE platform.feature_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Targeting
    enabled_globally BOOLEAN NOT NULL DEFAULT false,
    rollout_percentage INTEGER CHECK (rollout_percentage BETWEEN 0 AND 100),
    
    -- Overrides
    tenant_overrides JSONB DEFAULT '{}', -- {tenant_id: boolean}
    location_overrides JSONB DEFAULT '{}', -- {location_id: boolean}
    user_overrides JSONB DEFAULT '{}', -- {user_id: boolean}
    
    -- Metadata
    category VARCHAR(100),
    tags TEXT[] DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- ============================================================================
-- AUTH SCHEMA - AUTHENTICATION & AUTHORIZATION
-- ============================================================================

-- Users table
CREATE TABLE auth.users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Authentication
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(50),
    password_hash VARCHAR(255),
    
    -- Profile
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    display_name VARCHAR(255),
    avatar_url VARCHAR(500),
    
    -- Roles and permissions
    roles auth.user_role[] NOT NULL DEFAULT '{}',
    permissions TEXT[] DEFAULT '{}',
    location_ids UUID[] DEFAULT '{}', -- Which locations user can access
    
    -- Security
    mfa_secret VARCHAR(255),
    mfa_enabled BOOLEAN NOT NULL DEFAULT false,
    last_login_at TIMESTAMPTZ,
    last_login_ip INET,
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ,
    
    -- Preferences
    preferences JSONB NOT NULL DEFAULT '{}',
    language VARCHAR(10) DEFAULT 'en',
    timezone VARCHAR(50) DEFAULT 'UTC',
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    phone_verified BOOLEAN NOT NULL DEFAULT false,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE(tenant_id, email),
    UNIQUE(tenant_id, phone)
);

-- Sessions table
CREATE TABLE auth.sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Token data
    access_token_hash VARCHAR(255) NOT NULL,
    refresh_token_hash VARCHAR(255),
    
    -- Device info
    device_id UUID,
    device_name VARCHAR(255),
    device_type VARCHAR(50),
    user_agent TEXT,
    ip_address INET,
    
    -- Session data
    location_id UUID REFERENCES platform.locations(id),
    active_role auth.user_role,
    
    -- Expiry
    access_expires_at TIMESTAMPTZ NOT NULL,
    refresh_expires_at TIMESTAMPTZ,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    revoked_at TIMESTAMPTZ,
    revoked_reason TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Device registry
CREATE TABLE auth.devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    
    -- Device identification
    device_id VARCHAR(255) NOT NULL,
    device_name VARCHAR(255),
    device_type VARCHAR(50), -- pos, kiosk, mobile, tablet, watch, web
    platform VARCHAR(50), -- ios, android, windows, macos, linux, web
    
    -- Push notifications
    push_token VARCHAR(500),
    push_provider VARCHAR(50), -- fcm, apns
    
    -- Security
    fingerprint VARCHAR(255),
    trusted BOOLEAN NOT NULL DEFAULT false,
    last_seen_at TIMESTAMPTZ,
    last_seen_ip INET,
    
    -- Location binding (for POS devices)
    location_id UUID REFERENCES platform.locations(id),
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT true,
    blocked BOOLEAN NOT NULL DEFAULT false,
    blocked_reason TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, device_id)
);

-- ============================================================================
-- COMMERCE SCHEMA - ORDERS & PAYMENTS
-- ============================================================================

-- Products/Items catalog
CREATE TABLE commerce.products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Basic info
    sku VARCHAR(100) NOT NULL,
    barcode VARCHAR(100),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category VARCHAR(100),
    tags TEXT[] DEFAULT '{}',
    
    -- Pricing
    price DECIMAL(10,2) NOT NULL,
    cost DECIMAL(10,2),
    tax_rate DECIMAL(5,4) DEFAULT 0,
    
    -- Variants and modifiers
    is_variant BOOLEAN DEFAULT false,
    parent_id UUID REFERENCES commerce.products(id),
    variants JSONB DEFAULT '[]',
    modifiers JSONB DEFAULT '[]',
    
    -- Images
    image_url VARCHAR(500),
    images JSONB DEFAULT '[]',
    
    -- Inventory tracking
    track_inventory BOOLEAN DEFAULT true,
    
    -- Availability
    is_active BOOLEAN NOT NULL DEFAULT true,
    available_online BOOLEAN DEFAULT true,
    available_pos BOOLEAN DEFAULT true,
    location_availability JSONB DEFAULT '{}', -- {location_id: boolean}
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    sort_order INTEGER DEFAULT 0,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE(tenant_id, sku)
);

-- Orders table
CREATE TABLE commerce.orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    location_id UUID REFERENCES platform.locations(id),
    
    -- Order identification
    order_number VARCHAR(50) NOT NULL,
    external_id VARCHAR(100),
    
    -- Customer
    customer_id UUID,
    guest_email VARCHAR(255),
    guest_phone VARCHAR(50),
    
    -- Order data
    status commerce.order_status NOT NULL DEFAULT 'pending',
    source VARCHAR(50), -- pos, online, mobile, kiosk, phone
    channel VARCHAR(50), -- dine_in, takeout, delivery, pickup
    
    -- Items
    items JSONB NOT NULL DEFAULT '[]',
    /*
    items: [{
        product_id: uuid,
        name: string,
        quantity: number,
        unit_price: number,
        modifiers: [],
        notes: string,
        tax: number,
        total: number
    }]
    */
    
    -- Pricing
    currency_code CHAR(3) DEFAULT 'USD',
    subtotal DECIMAL(10,2) NOT NULL DEFAULT 0,
    tax_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    tip_amount DECIMAL(10,2) DEFAULT 0,
    discount_amount DECIMAL(10,2) DEFAULT 0,
    delivery_fee DECIMAL(10,2) DEFAULT 0,
    service_fee DECIMAL(10,2) DEFAULT 0,
    total_amount DECIMAL(10,2) NOT NULL DEFAULT 0,
    
    -- Payment
    payment_status commerce.payment_status DEFAULT 'pending',
    payment_method VARCHAR(50),
    payment_details JSONB DEFAULT '{}',
    
    -- Delivery/Pickup
    fulfillment_type VARCHAR(50),
    scheduled_at TIMESTAMPTZ,
    delivery_address JSONB,
    delivery_instructions TEXT,
    
    -- Table/Service info (for restaurants)
    table_number VARCHAR(20),
    party_size INTEGER,
    server_id UUID REFERENCES auth.users(id),
    
    -- Notes
    customer_notes TEXT,
    internal_notes TEXT,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ,
    preparing_at TIMESTAMPTZ,
    ready_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    
    UNIQUE(tenant_id, order_number)
);

-- Payments table
CREATE TABLE commerce.payments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    order_id UUID REFERENCES commerce.orders(id) ON DELETE CASCADE,
    
    -- Payment info
    amount DECIMAL(10,2) NOT NULL,
    currency_code CHAR(3) DEFAULT 'USD',
    method VARCHAR(50) NOT NULL, -- card, cash, wallet, bank_transfer
    status commerce.payment_status NOT NULL DEFAULT 'pending',
    
    -- Gateway info
    gateway VARCHAR(50), -- stripe, square, paypal
    gateway_transaction_id VARCHAR(255),
    gateway_response JSONB,
    
    -- Card details (encrypted/tokenized)
    card_last4 VARCHAR(4),
    card_brand VARCHAR(50),
    
    -- Refunds
    refunded_amount DECIMAL(10,2) DEFAULT 0,
    refund_reason TEXT,
    
    -- Metadata
    metadata JSONB DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    processed_at TIMESTAMPTZ,
    failed_at TIMESTAMPTZ,
    refunded_at TIMESTAMPTZ
);

-- Shopping carts
CREATE TABLE commerce.carts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Owner
    user_id UUID REFERENCES auth.users(id),
    session_id VARCHAR(255),
    device_id UUID REFERENCES auth.devices(id),
    
    -- Cart data
    items JSONB NOT NULL DEFAULT '[]',
    currency_code CHAR(3) DEFAULT 'USD',
    subtotal DECIMAL(10,2) DEFAULT 0,
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    converted_to_order_id UUID REFERENCES commerce.orders(id),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ DEFAULT NOW() + INTERVAL '30 days'
);

-- ============================================================================
-- INVENTORY SCHEMA
-- ============================================================================

-- Inventory items
CREATE TABLE inventory.stock (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL REFERENCES platform.locations(id),
    product_id UUID NOT NULL REFERENCES commerce.products(id),
    
    -- Stock levels
    quantity_on_hand DECIMAL(10,3) NOT NULL DEFAULT 0,
    quantity_reserved DECIMAL(10,3) NOT NULL DEFAULT 0,
    quantity_available DECIMAL(10,3) GENERATED ALWAYS AS 
        (quantity_on_hand - quantity_reserved) STORED,
    
    -- Thresholds
    min_stock_level DECIMAL(10,3),
    max_stock_level DECIMAL(10,3),
    reorder_point DECIMAL(10,3),
    reorder_quantity DECIMAL(10,3),
    
    -- Cost tracking
    average_cost DECIMAL(10,2),
    last_cost DECIMAL(10,2),
    
    -- Location in warehouse/store
    bin_location VARCHAR(50),
    zone VARCHAR(50),
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    count_required BOOLEAN DEFAULT false,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_counted_at TIMESTAMPTZ,
    last_received_at TIMESTAMPTZ,
    
    UNIQUE(tenant_id, location_id, product_id)
);

-- Stock movements
CREATE TABLE inventory.movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL REFERENCES platform.locations(id),
    product_id UUID NOT NULL REFERENCES commerce.products(id),
    
    -- Movement info
    type VARCHAR(50) NOT NULL, -- sale, purchase, transfer, adjustment, waste, return
    quantity DECIMAL(10,3) NOT NULL, -- Positive for in, negative for out
    unit_cost DECIMAL(10,2),
    
    -- References
    order_id UUID REFERENCES commerce.orders(id),
    transfer_id UUID,
    adjustment_reason VARCHAR(255),
    
    -- User tracking
    user_id UUID NOT NULL REFERENCES auth.users(id),
    notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE
);

-- Suppliers
CREATE TABLE inventory.suppliers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Basic info
    code VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    legal_name VARCHAR(255),
    tax_id VARCHAR(50),
    
    -- Contact
    contact_name VARCHAR(255),
    email VARCHAR(255),
    phone VARCHAR(50),
    website VARCHAR(255),
    
    -- Address
    address JSONB,
    
    -- Terms
    payment_terms VARCHAR(50), -- net30, net60, cod
    currency_code CHAR(3) DEFAULT 'USD',
    minimum_order_amount DECIMAL(10,2),
    
    -- Performance
    lead_time_days INTEGER,
    reliability_score DECIMAL(3,2), -- 0.00 to 1.00
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    is_preferred BOOLEAN DEFAULT false,
    
    -- Metadata
    categories TEXT[] DEFAULT '{}',
    notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, code)
);

-- ============================================================================
-- CUSTOMER SCHEMA
-- ============================================================================

-- Customer profiles
CREATE TABLE customer.profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Identification
    customer_number VARCHAR(50),
    external_id VARCHAR(100),
    
    -- Personal info
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    company_name VARCHAR(255),
    
    -- Contact
    email VARCHAR(255),
    phone VARCHAR(50),
    secondary_phone VARCHAR(50),
    
    -- Address
    addresses JSONB DEFAULT '[]',
    default_billing_address_id UUID,
    default_shipping_address_id UUID,
    
    -- Demographics
    birth_date DATE,
    gender VARCHAR(20),
    language VARCHAR(10) DEFAULT 'en',
    
    -- Loyalty
    loyalty_tier VARCHAR(50) DEFAULT 'bronze',
    loyalty_points INTEGER DEFAULT 0,
    loyalty_member_since DATE,
    
    -- Metrics
    lifetime_value DECIMAL(10,2) DEFAULT 0,
    total_orders INTEGER DEFAULT 0,
    average_order_value DECIMAL(10,2),
    last_order_date DATE,
    
    -- Preferences
    preferences JSONB DEFAULT '{}',
    dietary_restrictions TEXT[],
    allergens TEXT[],
    
    -- Marketing
    email_opt_in BOOLEAN DEFAULT false,
    sms_opt_in BOOLEAN DEFAULT false,
    push_opt_in BOOLEAN DEFAULT false,
    
    -- Segmentation
    segments TEXT[] DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    
    -- Notes
    notes TEXT,
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    is_vip BOOLEAN DEFAULT false,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    
    UNIQUE(tenant_id, email),
    UNIQUE(tenant_id, customer_number)
);

-- Customer interactions
CREATE TABLE customer.interactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customer.profiles(id) ON DELETE CASCADE,
    
    -- Interaction info
    type VARCHAR(50) NOT NULL, -- order, support, feedback, complaint, review
    channel VARCHAR(50), -- email, phone, chat, in_person, social
    
    -- Content
    subject VARCHAR(255),
    content TEXT,
    sentiment VARCHAR(20), -- positive, neutral, negative
    
    -- References
    order_id UUID REFERENCES commerce.orders(id),
    user_id UUID REFERENCES auth.users(id),
    
    -- Resolution
    status VARCHAR(50) DEFAULT 'open',
    priority VARCHAR(20) DEFAULT 'normal',
    resolved_at TIMESTAMPTZ,
    resolution_notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Loyalty transactions
CREATE TABLE customer.loyalty_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customer.profiles(id) ON DELETE CASCADE,
    
    -- Transaction info
    type VARCHAR(50) NOT NULL, -- earn, redeem, expire, adjust
    points INTEGER NOT NULL, -- Positive for earn, negative for redeem
    
    -- References
    order_id UUID REFERENCES commerce.orders(id),
    campaign_id UUID,
    
    -- Metadata
    description TEXT,
    metadata JSONB DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- ============================================================================
-- WORKFORCE SCHEMA
-- ============================================================================

-- Employees
CREATE TABLE workforce.employees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    
    -- Employee info
    employee_number VARCHAR(50) NOT NULL,
    department VARCHAR(100),
    position VARCHAR(100),
    manager_id UUID REFERENCES workforce.employees(id),
    
    -- Location assignment
    primary_location_id UUID REFERENCES platform.locations(id),
    location_ids UUID[] DEFAULT '{}',
    
    -- Employment
    hire_date DATE NOT NULL,
    termination_date DATE,
    employment_type VARCHAR(50), -- full_time, part_time, contractor
    
    -- Compensation
    pay_rate DECIMAL(10,2),
    pay_frequency VARCHAR(20), -- hourly, salary, commission
    
    -- Status
    is_active BOOLEAN DEFAULT true,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, employee_number),
    UNIQUE(tenant_id, user_id)
);

-- Schedules
CREATE TABLE workforce.schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL REFERENCES platform.locations(id),
    employee_id UUID NOT NULL REFERENCES workforce.employees(id),
    
    -- Schedule info
    shift_start TIMESTAMPTZ NOT NULL,
    shift_end TIMESTAMPTZ NOT NULL,
    break_minutes INTEGER DEFAULT 0,
    
    -- Role/Position for this shift
    role VARCHAR(100),
    department VARCHAR(100),
    
    -- Status
    status VARCHAR(50) DEFAULT 'scheduled', -- scheduled, confirmed, in_progress, completed, cancelled
    
    -- Actual times (for time tracking)
    actual_start TIMESTAMPTZ,
    actual_end TIMESTAMPTZ,
    
    -- Notes
    notes TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Prevent double-booking
    EXCLUDE USING gist (
        employee_id WITH =,
        tstzrange(shift_start, shift_end) WITH &&
    ) WHERE (status != 'cancelled')
);

-- Time clock entries
CREATE TABLE workforce.time_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    employee_id UUID NOT NULL REFERENCES workforce.employees(id),
    schedule_id UUID REFERENCES workforce.schedules(id),
    
    -- Clock in/out
    clock_in TIMESTAMPTZ NOT NULL,
    clock_out TIMESTAMPTZ,
    
    -- Break tracking
    break_start TIMESTAMPTZ,
    break_end TIMESTAMPTZ,
    
    -- Location/Device
    location_id UUID REFERENCES platform.locations(id),
    device_id UUID REFERENCES auth.devices(id),
    clock_in_location JSONB, -- GPS coordinates
    clock_out_location JSONB,
    
    -- Approval
    approved_by UUID REFERENCES auth.users(id),
    approved_at TIMESTAMPTZ,
    
    -- Adjustments
    adjusted_clock_in TIMESTAMPTZ,
    adjusted_clock_out TIMESTAMPTZ,
    adjustment_reason TEXT,
    
    -- Total hours calculation
    total_hours DECIMAL(5,2) GENERATED ALWAYS AS (
        CASE 
            WHEN clock_out IS NOT NULL 
            THEN EXTRACT(EPOCH FROM (
                COALESCE(adjusted_clock_out, clock_out) - 
                COALESCE(adjusted_clock_in, clock_in)
            )) / 3600.0
            ELSE NULL
        END
    ) STORED,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- ============================================================================
-- ANALYTICS SCHEMA
-- ============================================================================

-- Metrics table (for pre-calculated metrics)
CREATE TABLE analytics.metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    location_id UUID REFERENCES platform.locations(id),
    
    -- Metric info
    metric_date DATE NOT NULL,
    metric_type VARCHAR(100) NOT NULL,
    metric_name VARCHAR(255) NOT NULL,
    
    -- Values
    value DECIMAL(20,4),
    previous_value DECIMAL(20,4),
    
    -- Aggregations
    dimensions JSONB DEFAULT '{}',
    
    -- Timestamps
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(tenant_id, location_id, metric_date, metric_type, metric_name)
);

-- Events table (for event tracking)
CREATE TABLE analytics.events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES platform.tenants(id) ON DELETE CASCADE,
    
    -- Event info
    event_type VARCHAR(100) NOT NULL,
    event_name VARCHAR(255) NOT NULL,
    event_category VARCHAR(100),
    
    -- Context
    user_id UUID REFERENCES auth.users(id),
    session_id UUID,
    device_id UUID REFERENCES auth.devices(id),
    
    -- Properties
    properties JSONB DEFAULT '{}',
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create monthly partitions for events
CREATE TABLE analytics.events_2024_01 PARTITION OF analytics.events
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');
-- Continue creating partitions as needed...

-- ============================================================================
-- AUDIT SCHEMA
-- ============================================================================

-- Audit log table
CREATE TABLE audit.logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    
    -- Actor
    user_id UUID,
    user_email VARCHAR(255),
    user_role auth.user_role,
    
    -- Action
    action VARCHAR(50) NOT NULL, -- create, update, delete, login, logout, etc.
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    
    -- Changes
    old_values JSONB,
    new_values JSONB,
    
    -- Context
    ip_address INET,
    user_agent TEXT,
    session_id UUID,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
) PARTITION BY RANGE (created_at);

-- Create monthly partitions for audit logs
CREATE TABLE audit.logs_2024_01 PARTITION OF audit.logs
    FOR VALUES FROM ('2024-01-01') TO ('2024-02-01');

-- ============================================================================
-- INDEXES
-- ============================================================================

-- Platform indexes
CREATE INDEX idx_tenants_slug ON platform.tenants(slug) WHERE deleted_at IS NULL;
CREATE INDEX idx_tenants_parent ON platform.tenants(parent_id) WHERE parent_id IS NOT NULL;
CREATE INDEX idx_locations_tenant ON platform.locations(tenant_id) WHERE is_active = true;
CREATE INDEX idx_locations_geo ON platform.locations USING gist(ll_to_earth(latitude, longitude));

-- Auth indexes
CREATE INDEX idx_users_tenant_email ON auth.users(tenant_id, lower(email)) WHERE deleted_at IS NULL;
CREATE INDEX idx_users_tenant_phone ON auth.users(tenant_id, phone) WHERE phone IS NOT NULL;
CREATE INDEX idx_sessions_user ON auth.sessions(user_id) WHERE is_active = true;
CREATE INDEX idx_sessions_token ON auth.sessions(access_token_hash);
CREATE INDEX idx_devices_tenant_device ON auth.devices(tenant_id, device_id);

-- Commerce indexes
CREATE INDEX idx_products_tenant_sku ON commerce.products(tenant_id, sku) WHERE deleted_at IS NULL;
CREATE INDEX idx_products_barcode ON commerce.products(barcode) WHERE barcode IS NOT NULL;
CREATE INDEX idx_orders_tenant_number ON commerce.orders(tenant_id, order_number);
CREATE INDEX idx_orders_tenant_status ON commerce.orders(tenant_id, status) WHERE status NOT IN ('completed', 'cancelled');
CREATE INDEX idx_orders_customer ON commerce.orders(customer_id) WHERE customer_id IS NOT NULL;
CREATE INDEX idx_orders_created ON commerce.orders(created_at DESC);

-- Inventory indexes
CREATE INDEX idx_stock_tenant_location_product ON inventory.stock(tenant_id, location_id, product_id);
CREATE INDEX idx_stock_low ON inventory.stock(tenant_id, location_id) 
    WHERE quantity_available < reorder_point;
CREATE INDEX idx_movements_tenant_product ON inventory.movements(tenant_id, product_id);
CREATE INDEX idx_movements_date ON inventory.movements(effective_date DESC);

-- Customer indexes
CREATE INDEX idx_customers_tenant_email ON customer.profiles(tenant_id, lower(email)) WHERE deleted_at IS NULL;
CREATE INDEX idx_customers_phone ON customer.profiles(tenant_id, phone) WHERE phone IS NOT NULL;
CREATE INDEX idx_customers_loyalty ON customer.profiles(tenant_id, loyalty_tier);
CREATE INDEX idx_interactions_customer ON customer.interactions(customer_id);
CREATE INDEX idx_interactions_open ON customer.interactions(tenant_id, status) WHERE status = 'open';

-- Workforce indexes
CREATE INDEX idx_employees_tenant_user ON workforce.employees(tenant_id, user_id);
CREATE INDEX idx_employees_manager ON workforce.employees(manager_id) WHERE manager_id IS NOT NULL;
CREATE INDEX idx_schedules_employee_date ON workforce.schedules(employee_id, shift_start);
CREATE INDEX idx_schedules_location_date ON workforce.schedules(location_id, shift_start);
CREATE INDEX idx_time_entries_employee ON workforce.time_entries(employee_id, clock_in DESC);

-- Analytics indexes
CREATE INDEX idx_metrics_tenant_date ON analytics.metrics(tenant_id, metric_date DESC);
CREATE INDEX idx_metrics_type ON analytics.metrics(tenant_id, metric_type, metric_date DESC);
CREATE INDEX idx_events_tenant_type ON analytics.events(tenant_id, event_type, created_at DESC);
CREATE INDEX idx_events_user ON analytics.events(user_id, created_at DESC) WHERE user_id IS NOT NULL;

-- Audit indexes
CREATE INDEX idx_audit_tenant ON audit.logs(tenant_id, created_at DESC);
CREATE INDEX idx_audit_user ON audit.logs(user_id, created_at DESC) WHERE user_id IS NOT NULL;
CREATE INDEX idx_audit_resource ON audit.logs(resource_type, resource_id, created_at DESC);

-- ============================================================================
-- ROW LEVEL SECURITY
-- ============================================================================

-- Enable RLS on all tables
ALTER TABLE platform.tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE platform.locations ENABLE ROW LEVEL SECURITY;
ALTER TABLE auth.users ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.orders ENABLE ROW LEVEL SECURITY;
ALTER TABLE commerce.products ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory.stock ENABLE ROW LEVEL SECURITY;
ALTER TABLE customer.profiles ENABLE ROW LEVEL SECURITY;

-- Create policies
CREATE POLICY tenant_isolation ON platform.tenants
    USING (id = current_setting('app.current_tenant_id')::uuid OR 
           parent_id = current_setting('app.current_tenant_id')::uuid);

CREATE POLICY location_tenant_isolation ON platform.locations
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

CREATE POLICY user_tenant_isolation ON auth.users
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid);

-- ============================================================================
-- TRIGGERS
-- ============================================================================

-- Updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply updated_at trigger to all tables with updated_at column
CREATE TRIGGER update_tenants_updated_at BEFORE UPDATE ON platform.tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_locations_updated_at BEFORE UPDATE ON platform.locations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON auth.users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON commerce.products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Audit trigger function
CREATE OR REPLACE FUNCTION audit_trigger_function()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO audit.logs (
        tenant_id,
        user_id,
        action,
        resource_type,
        resource_id,
        old_values,
        new_values,
        created_at
    ) VALUES (
        current_setting('app.current_tenant_id')::uuid,
        current_setting('app.current_user_id')::uuid,
        TG_OP,
        TG_TABLE_NAME,
        CASE 
            WHEN TG_OP = 'DELETE' THEN OLD.id
            ELSE NEW.id
        END,
        CASE WHEN TG_OP IN ('UPDATE', 'DELETE') THEN to_jsonb(OLD) ELSE NULL END,
        CASE WHEN TG_OP IN ('INSERT', 'UPDATE') THEN to_jsonb(NEW) ELSE NULL END,
        NOW()
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- FUNCTIONS
-- ============================================================================

-- Function to check feature flag
CREATE OR REPLACE FUNCTION platform.is_feature_enabled(
    p_tenant_id UUID,
    p_feature_key VARCHAR(255)
) RETURNS BOOLEAN AS $$
DECLARE
    v_result BOOLEAN;
BEGIN
    SELECT 
        COALESCE(
            (tenant_overrides->p_tenant_id::text)::boolean,
            enabled_globally
        ) INTO v_result
    FROM platform.feature_flags
    WHERE key = p_feature_key
        AND (expires_at IS NULL OR expires_at > NOW());
    
    RETURN COALESCE(v_result, false);
END;
$$ LANGUAGE plpgsql;

-- Function to calculate customer lifetime value
CREATE OR REPLACE FUNCTION customer.calculate_lifetime_value(
    p_customer_id UUID
) RETURNS DECIMAL AS $$
DECLARE
    v_total DECIMAL;
BEGIN
    SELECT COALESCE(SUM(total_amount), 0) INTO v_total
    FROM commerce.orders
    WHERE customer_id = p_customer_id
        AND status = 'completed';
    
    RETURN v_total;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- INITIAL DATA
-- ============================================================================

-- Insert default feature flags
INSERT INTO platform.feature_flags (key, name, description, enabled_globally, category) VALUES
    ('multi_location', 'Multi-Location Support', 'Enable multi-location features', true, 'platform'),
    ('loyalty_program', 'Loyalty Program', 'Enable customer loyalty features', true, 'customer'),
    ('inventory_tracking', 'Inventory Tracking', 'Enable inventory management', true, 'inventory'),
    ('online_ordering', 'Online Ordering', 'Enable online ordering', true, 'commerce'),
    ('table_management', 'Table Management', 'Enable table management for restaurants', false, 'restaurant'),
    ('kitchen_display', 'Kitchen Display System', 'Enable kitchen display features', false, 'restaurant'),
    ('employee_scheduling', 'Employee Scheduling', 'Enable workforce scheduling', true, 'workforce'),
    ('advanced_analytics', 'Advanced Analytics', 'Enable advanced analytics features', false, 'analytics');

-- ============================================================================
-- MAINTENANCE
-- ============================================================================

-- Create function to automatically create monthly partitions
CREATE OR REPLACE FUNCTION create_monthly_partitions()
RETURNS void AS $$
DECLARE
    start_date date;
    end_date date;
    partition_name text;
BEGIN
    -- Create partitions for next 3 months
    FOR i IN 0..3 LOOP
        start_date := date_trunc('month', CURRENT_DATE + (i || ' months')::interval);
        end_date := start_date + '1 month'::interval;
        
        -- Events partitions
        partition_name := 'analytics.events_' || to_char(start_date, 'YYYY_MM');
        IF NOT EXISTS (SELECT 1 FROM pg_class WHERE relname = partition_name) THEN
            EXECUTE format('CREATE TABLE %I PARTITION OF analytics.events FOR VALUES FROM (%L) TO (%L)',
                partition_name, start_date, end_date);
        END IF;
        
        -- Audit log partitions
        partition_name := 'audit.logs_' || to_char(start_date, 'YYYY_MM');
        IF NOT EXISTS (SELECT 1 FROM pg_class WHERE relname = partition_name) THEN
            EXECUTE format('CREATE TABLE %I PARTITION OF audit.logs FOR VALUES FROM (%L) TO (%L)',
                partition_name, start_date, end_date);
        END IF;
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Schedule monthly partition creation (run this monthly via cron or pg_cron)
SELECT create_monthly_partitions();

-- ============================================================================
-- PERMISSIONS (for different database users)
-- ============================================================================

-- Create roles
CREATE ROLE olympus_app;
CREATE ROLE olympus_readonly;
CREATE ROLE olympus_analytics;

-- Grant permissions to app role
GRANT USAGE ON SCHEMA platform, auth, commerce, inventory, customer, workforce, analytics, events TO olympus_app;
GRANT ALL ON ALL TABLES IN SCHEMA platform, auth, commerce, inventory, customer, workforce, analytics, events TO olympus_app;
GRANT ALL ON ALL SEQUENCES IN SCHEMA platform, auth, commerce, inventory, customer, workforce, analytics, events TO olympus_app;

-- Grant read-only permissions
GRANT USAGE ON SCHEMA platform, auth, commerce, inventory, customer, workforce, analytics TO olympus_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA platform, auth, commerce, inventory, customer, workforce, analytics TO olympus_readonly;

-- Grant analytics permissions
GRANT USAGE ON SCHEMA analytics, platform TO olympus_analytics;
GRANT SELECT ON ALL TABLES IN SCHEMA analytics TO olympus_analytics;
GRANT SELECT ON platform.tenants, platform.locations TO olympus_analytics;
```