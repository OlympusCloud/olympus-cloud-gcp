-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Create enums for better type safety
CREATE TYPE subscription_tier AS ENUM ('FREE', 'STARTER', 'PROFESSIONAL', 'ENTERPRISE', 'CUSTOM');
CREATE TYPE subscription_status AS ENUM ('TRIAL', 'ACTIVE', 'PAST_DUE', 'CANCELLED', 'SUSPENDED');
CREATE TYPE user_status AS ENUM ('ACTIVE', 'INACTIVE', 'SUSPENDED', 'DELETED');
CREATE TYPE order_status AS ENUM ('PENDING', 'CONFIRMED', 'PROCESSING', 'SHIPPED', 'DELIVERED', 'CANCELLED', 'REFUNDED');
CREATE TYPE payment_status AS ENUM ('PENDING', 'PROCESSING', 'COMPLETED', 'FAILED', 'CANCELLED', 'REFUNDED', 'PARTIAL_REFUND');
CREATE TYPE fulfillment_status AS ENUM ('UNFULFILLED', 'PARTIAL', 'FULFILLED', 'CANCELLED');
CREATE TYPE payment_method AS ENUM ('CREDIT_CARD', 'DEBIT_CARD', 'PAYPAL', 'BANK_TRANSFER', 'CASH', 'CHECK', 'OTHER');
CREATE TYPE payment_type AS ENUM ('SALE', 'REFUND', 'PARTIAL_REFUND', 'AUTHORIZATION', 'CAPTURE', 'VOID');
CREATE TYPE adjustment_type AS ENUM ('ADD', 'REMOVE', 'SET');

-- Tenants table (multi-tenancy root)
CREATE TABLE tenants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    slug VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT,
    industry JSONB,
    subscription_tier subscription_tier NOT NULL DEFAULT 'FREE',
    subscription_status subscription_status NOT NULL DEFAULT 'TRIAL',
    trial_ends_at TIMESTAMPTZ,
    billing_email VARCHAR(255),
    support_email VARCHAR(255),
    website VARCHAR(500),
    logo_url VARCHAR(500),
    settings JSONB NOT NULL DEFAULT '{}',
    features TEXT[] DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    user_limit INTEGER,
    location_limit INTEGER,
    storage_limit_gb INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT valid_slug CHECK (slug ~ '^[a-z0-9-]+$')
);

-- Create indexes for tenants
CREATE INDEX idx_tenants_slug ON tenants(slug);
CREATE INDEX idx_tenants_active ON tenants(is_active) WHERE deleted_at IS NULL;

-- Users table (authentication)
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(100),
    password_hash TEXT NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    display_name VARCHAR(200),
    avatar_url VARCHAR(500),
    phone VARCHAR(50),
    status user_status NOT NULL DEFAULT 'ACTIVE',
    email_verified BOOLEAN NOT NULL DEFAULT false,
    email_verified_at TIMESTAMPTZ,
    last_login_at TIMESTAMPTZ,
    failed_login_attempts INTEGER DEFAULT 0,
    locked_until TIMESTAMPTZ,
    password_changed_at TIMESTAMPTZ,
    preferences JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT unique_email_per_tenant UNIQUE(tenant_id, email),
    CONSTRAINT unique_username_per_tenant UNIQUE(tenant_id, username)
);

-- Create indexes for users
CREATE INDEX idx_users_tenant ON users(tenant_id);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_status ON users(status) WHERE deleted_at IS NULL;

-- Sessions table (JWT refresh tokens)
CREATE TABLE sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token TEXT NOT NULL UNIQUE,
    device_info JSONB,
    ip_address INET,
    user_agent TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for sessions
CREATE INDEX idx_sessions_user ON sessions(user_id);
CREATE INDEX idx_sessions_token ON sessions(refresh_token);
CREATE INDEX idx_sessions_expires ON sessions(expires_at);

-- Locations table (multi-location support)
CREATE TABLE locations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50),
    description TEXT,
    address JSONB,
    phone JSONB,
    email VARCHAR(255),
    manager_id UUID REFERENCES users(id),
    timezone VARCHAR(100),
    business_hours JSONB,
    is_primary BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    features TEXT[] DEFAULT '{}',
    settings JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT unique_location_code UNIQUE(tenant_id, code)
);

-- Create indexes for locations
CREATE INDEX idx_locations_tenant ON locations(tenant_id);
CREATE INDEX idx_locations_active ON locations(is_active) WHERE deleted_at IS NULL;

-- Roles table (RBAC)
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    display_name VARCHAR(200),
    description TEXT,
    permissions TEXT[] DEFAULT '{}',
    is_system BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_role_name UNIQUE(tenant_id, name)
);

-- Create indexes for roles
CREATE INDEX idx_roles_tenant ON roles(tenant_id);
CREATE INDEX idx_roles_active ON roles(is_active);

-- User roles junction table
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    assigned_by UUID REFERENCES users(id),

    PRIMARY KEY (user_id, role_id)
);

-- Create indexes for user_roles
CREATE INDEX idx_user_roles_user ON user_roles(user_id);
CREATE INDEX idx_user_roles_role ON user_roles(role_id);

-- Categories table (product organization)
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES categories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    description TEXT,
    image_url VARCHAR(500),
    sort_order INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT unique_category_slug UNIQUE(tenant_id, slug)
);

-- Create indexes for categories
CREATE INDEX idx_categories_tenant ON categories(tenant_id);
CREATE INDEX idx_categories_parent ON categories(parent_id);
CREATE INDEX idx_categories_slug ON categories(slug);

-- Products table
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    sku VARCHAR(100) NOT NULL,
    name VARCHAR(500) NOT NULL,
    description TEXT,
    category_id UUID REFERENCES categories(id),
    brand VARCHAR(255),
    unit_price DECIMAL(19,4) NOT NULL,
    compare_at_price DECIMAL(19,4),
    cost DECIMAL(19,4),
    tax_rate DECIMAL(5,4) DEFAULT 0,
    weight_value DECIMAL(10,3),
    weight_unit VARCHAR(20),
    dimensions JSONB,
    is_digital BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    requires_shipping BOOLEAN DEFAULT true,
    track_inventory BOOLEAN DEFAULT true,
    allow_backorder BOOLEAN DEFAULT false,
    images TEXT[] DEFAULT '{}',
    attributes JSONB DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT unique_product_sku UNIQUE(tenant_id, sku)
);

-- Create indexes for products
CREATE INDEX idx_products_tenant ON products(tenant_id);
CREATE INDEX idx_products_sku ON products(sku);
CREATE INDEX idx_products_category ON products(category_id);
CREATE INDEX idx_products_active ON products(is_active) WHERE deleted_at IS NULL;

-- Product variants table
CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    sku VARCHAR(100) NOT NULL,
    name VARCHAR(500),
    options JSONB NOT NULL DEFAULT '{}',
    price DECIMAL(19,4),
    compare_at_price DECIMAL(19,4),
    cost DECIMAL(19,4),
    weight_value DECIMAL(10,3),
    weight_unit VARCHAR(20),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for product_variants
CREATE INDEX idx_variants_product ON product_variants(product_id);
CREATE INDEX idx_variants_sku ON product_variants(sku);

-- Inventory table
CREATE TABLE inventory (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    variant_id UUID REFERENCES product_variants(id) ON DELETE CASCADE,
    location_id UUID NOT NULL REFERENCES locations(id) ON DELETE CASCADE,
    quantity_on_hand INTEGER NOT NULL DEFAULT 0,
    quantity_reserved INTEGER NOT NULL DEFAULT 0,
    quantity_available INTEGER GENERATED ALWAYS AS (quantity_on_hand - quantity_reserved) STORED,
    reorder_point INTEGER,
    reorder_quantity INTEGER,
    last_counted_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_inventory UNIQUE(product_id, location_id)
);

-- Create indexes for inventory
CREATE INDEX idx_inventory_product ON inventory(product_id);
CREATE INDEX idx_inventory_location ON inventory(location_id);
CREATE INDEX idx_inventory_available ON inventory(quantity_available);
-- Partial unique index for variant inventory
CREATE UNIQUE INDEX idx_unique_variant_inventory ON inventory(variant_id, location_id) WHERE variant_id IS NOT NULL;

-- Inventory adjustments log
CREATE TABLE inventory_adjustments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    inventory_id UUID NOT NULL REFERENCES inventory(id) ON DELETE CASCADE,
    adjustment_type adjustment_type NOT NULL,
    quantity INTEGER NOT NULL,
    reason VARCHAR(500),
    reference_type VARCHAR(50),
    reference_id UUID,
    user_id UUID REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

);

-- Create indexes for inventory_adjustments
CREATE INDEX idx_adjustments_inventory ON inventory_adjustments(inventory_id);
CREATE INDEX idx_adjustments_created ON inventory_adjustments(created_at);

-- Customers table
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    phone VARCHAR(50),
    company VARCHAR(255),
    addresses JSONB DEFAULT '[]',
    default_address_id UUID,
    accepts_marketing BOOLEAN DEFAULT false,
    tax_exempt BOOLEAN DEFAULT false,
    notes TEXT,
    tags TEXT[] DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    total_spent DECIMAL(19,4) DEFAULT 0,
    order_count INTEGER DEFAULT 0,
    last_order_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMPTZ,

    CONSTRAINT unique_customer_email UNIQUE(tenant_id, email)
);

-- Create indexes for customers
CREATE INDEX idx_customers_tenant ON customers(tenant_id);
CREATE INDEX idx_customers_email ON customers(email);

-- Orders table
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_number VARCHAR(50) NOT NULL,
    customer_id UUID REFERENCES customers(id),
    location_id UUID NOT NULL REFERENCES locations(id),
    status order_status NOT NULL DEFAULT 'PENDING',
    subtotal DECIMAL(19,4) NOT NULL,
    tax_amount DECIMAL(19,4) DEFAULT 0,
    discount_amount DECIMAL(19,4) DEFAULT 0,
    shipping_amount DECIMAL(19,4) DEFAULT 0,
    total_amount DECIMAL(19,4) NOT NULL,
    currency JSONB NOT NULL,
    payment_status payment_status NOT NULL DEFAULT 'PENDING',
    fulfillment_status fulfillment_status NOT NULL DEFAULT 'UNFULFILLED',
    shipping_address JSONB,
    billing_address JSONB,
    notes TEXT,
    tags TEXT[] DEFAULT '{}',
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    cancelled_at TIMESTAMPTZ,
    fulfilled_at TIMESTAMPTZ,

    CONSTRAINT unique_order_number UNIQUE(tenant_id, order_number)
);

-- Create indexes for orders
CREATE INDEX idx_orders_tenant ON orders(tenant_id);
CREATE INDEX idx_orders_customer ON orders(customer_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created ON orders(created_at DESC);

-- Order items table
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    variant_id UUID REFERENCES product_variants(id),
    sku VARCHAR(100),
    name VARCHAR(500) NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(19,4) NOT NULL,
    discount_amount DECIMAL(19,4) DEFAULT 0,
    tax_amount DECIMAL(19,4) DEFAULT 0,
    total_amount DECIMAL(19,4) NOT NULL,
    fulfillment_status fulfillment_status NOT NULL DEFAULT 'UNFULFILLED',
    fulfilled_quantity INTEGER DEFAULT 0,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

);

-- Create indexes for order_items
CREATE INDEX idx_order_items_order ON order_items(order_id);
CREATE INDEX idx_order_items_product ON order_items(product_id);

-- Payments table
CREATE TABLE payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    transaction_id VARCHAR(255) NOT NULL,
    payment_method payment_method NOT NULL,
    payment_type payment_type NOT NULL,
    status payment_status NOT NULL,
    amount DECIMAL(19,4) NOT NULL,
    currency JSONB NOT NULL,
    gateway VARCHAR(100),
    gateway_response JSONB,
    reference_number VARCHAR(255),
    authorization_code VARCHAR(100),
    card_last_four VARCHAR(4),
    card_brand VARCHAR(50),
    metadata JSONB DEFAULT '{}',
    processed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    refunded_amount DECIMAL(19,4) DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

);

-- Create indexes for payments
CREATE INDEX idx_payments_tenant ON payments(tenant_id);
CREATE INDEX idx_payments_order ON payments(order_id);
CREATE INDEX idx_payments_transaction ON payments(transaction_id);
CREATE INDEX idx_payments_status ON payments(status);

-- Audit log table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    action VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100) NOT NULL,
    resource_id UUID,
    changes JSONB,
    ip_address INET,
    user_agent TEXT,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

);

-- Create indexes for audit_logs
CREATE INDEX idx_audit_tenant ON audit_logs(tenant_id);
CREATE INDEX idx_audit_user ON audit_logs(user_id);
CREATE INDEX idx_audit_resource ON audit_logs(resource_type, resource_id);
CREATE INDEX idx_audit_created ON audit_logs(created_at DESC);

-- Row Level Security Policies
ALTER TABLE tenants ENABLE ROW LEVEL SECURITY;
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE locations ENABLE ROW LEVEL SECURITY;
ALTER TABLE roles ENABLE ROW LEVEL SECURITY;
ALTER TABLE products ENABLE ROW LEVEL SECURITY;
ALTER TABLE inventory ENABLE ROW LEVEL SECURITY;
ALTER TABLE customers ENABLE ROW LEVEL SECURITY;
ALTER TABLE orders ENABLE ROW LEVEL SECURITY;

-- Create update timestamp trigger
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply update trigger to all tables with updated_at
CREATE TRIGGER update_tenants_updated_at BEFORE UPDATE ON tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_locations_updated_at BEFORE UPDATE ON locations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_roles_updated_at BEFORE UPDATE ON roles
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_categories_updated_at BEFORE UPDATE ON categories
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_products_updated_at BEFORE UPDATE ON products
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_product_variants_updated_at BEFORE UPDATE ON product_variants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_inventory_updated_at BEFORE UPDATE ON inventory
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_customers_updated_at BEFORE UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_payments_updated_at BEFORE UPDATE ON payments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();