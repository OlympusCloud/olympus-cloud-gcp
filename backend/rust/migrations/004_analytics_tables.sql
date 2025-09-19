-- ============================================================================
-- OLYMPUS CLOUD - ANALYTICS AND EVENTS SYSTEM
-- ============================================================================
-- Migration: 004_analytics_tables.sql
-- Description: Analytics tables and event sourcing system
-- Author: Claude Code Agent
-- Date: 2025-01-18
-- ============================================================================

-- Create schemas
CREATE SCHEMA IF NOT EXISTS events;
CREATE SCHEMA IF NOT EXISTS analytics;

-- Event and analytics types
CREATE TYPE events.event_status AS ENUM (
    'pending',
    'processed',
    'failed',
    'retrying'
);

CREATE TYPE analytics.metric_type AS ENUM (
    'counter',
    'gauge',
    'histogram',
    'timer'
);

CREATE TYPE analytics.aggregation_period AS ENUM (
    'minute',
    'hour',
    'day',
    'week',
    'month',
    'quarter',
    'year'
);

-- ============================================================================
-- EVENT SOURCING TABLES
-- ============================================================================

-- Domain events for event sourcing
CREATE TABLE events.domain_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID REFERENCES tenants(id) ON DELETE CASCADE,
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    event_version INTEGER NOT NULL DEFAULT 1,
    event_data JSONB NOT NULL,
    metadata JSONB DEFAULT '{}',
    user_id UUID REFERENCES users(id),
    correlation_id UUID,
    causation_id UUID,
    sequence_number BIGSERIAL,
    status events.event_status NOT NULL DEFAULT 'pending',
    processed_at TIMESTAMPTZ,
    retry_count INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Ensure events are ordered properly
    CONSTRAINT events_sequence_order UNIQUE(aggregate_id, sequence_number)
);

-- Event subscriptions for services
CREATE TABLE events.event_subscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subscriber_name VARCHAR(100) NOT NULL,
    event_type VARCHAR(100) NOT NULL,
    last_processed_sequence BIGINT NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Unique subscription per subscriber per event type
    CONSTRAINT unique_subscription UNIQUE(subscriber_name, event_type)
);

-- Snapshots for event sourcing optimization
CREATE TABLE events.aggregate_snapshots (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    aggregate_id UUID NOT NULL,
    aggregate_type VARCHAR(100) NOT NULL,
    version INTEGER NOT NULL,
    snapshot_data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- One snapshot per aggregate version
    CONSTRAINT unique_snapshot UNIQUE(aggregate_id, version)
);

-- ============================================================================
-- ANALYTICS TABLES
-- ============================================================================

-- Business metrics tracking
CREATE TABLE analytics.business_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID REFERENCES locations(id),
    metric_name VARCHAR(100) NOT NULL,
    metric_type analytics.metric_type NOT NULL,
    value DECIMAL(19,4) NOT NULL,
    unit VARCHAR(50),
    dimensions JSONB DEFAULT '{}',
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_key DATE NOT NULL DEFAULT CURRENT_DATE,
    hour_key INTEGER NOT NULL DEFAULT EXTRACT(HOUR FROM CURRENT_TIMESTAMP),

    -- Partitioning friendly
    CONSTRAINT valid_hour CHECK (hour_key >= 0 AND hour_key <= 23)
) PARTITION BY RANGE (recorded_at);

-- Create monthly partitions for business metrics
CREATE TABLE analytics.business_metrics_2025_01 PARTITION OF analytics.business_metrics
    FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

CREATE TABLE analytics.business_metrics_2025_02 PARTITION OF analytics.business_metrics
    FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');

-- User activity tracking
CREATE TABLE analytics.user_activities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    session_id VARCHAR(255),
    activity_type VARCHAR(100) NOT NULL,
    resource_type VARCHAR(100),
    resource_id UUID,
    properties JSONB DEFAULT '{}',
    ip_address INET,
    user_agent TEXT,
    referrer TEXT,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    date_key DATE NOT NULL DEFAULT CURRENT_DATE
) PARTITION BY RANGE (recorded_at);

-- Create monthly partitions for user activities
CREATE TABLE analytics.user_activities_2025_01 PARTITION OF analytics.user_activities
    FOR VALUES FROM ('2025-01-01') TO ('2025-02-01');

CREATE TABLE analytics.user_activities_2025_02 PARTITION OF analytics.user_activities
    FOR VALUES FROM ('2025-02-01') TO ('2025-03-01');

-- Aggregated analytics data
CREATE TABLE analytics.metric_aggregations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    location_id UUID REFERENCES locations(id),
    metric_name VARCHAR(100) NOT NULL,
    aggregation_period analytics.aggregation_period NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    count_value BIGINT,
    sum_value DECIMAL(19,4),
    avg_value DECIMAL(19,4),
    min_value DECIMAL(19,4),
    max_value DECIMAL(19,4),
    percentile_50 DECIMAL(19,4),
    percentile_95 DECIMAL(19,4),
    percentile_99 DECIMAL(19,4),
    dimensions JSONB DEFAULT '{}',
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Unique aggregation per metric per period
    CONSTRAINT unique_aggregation UNIQUE(tenant_id, location_id, metric_name, aggregation_period, period_start)
);

-- Customer journey tracking
CREATE TABLE analytics.customer_journeys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id UUID REFERENCES customers(id),
    session_id VARCHAR(255),
    journey_id UUID NOT NULL,
    step_number INTEGER NOT NULL,
    step_name VARCHAR(100) NOT NULL,
    step_type VARCHAR(50) NOT NULL, -- page_view, action, conversion, etc.
    properties JSONB DEFAULT '{}',
    duration_ms INTEGER,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Ensure journey steps are ordered
    CONSTRAINT unique_journey_step UNIQUE(journey_id, step_number)
);

-- A/B test experiments
CREATE TABLE analytics.experiments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    hypothesis TEXT,
    variants JSONB NOT NULL, -- Array of variant configurations
    traffic_allocation JSONB NOT NULL, -- Percentage allocation per variant
    success_metrics JSONB NOT NULL, -- Array of metrics to track
    start_date TIMESTAMPTZ,
    end_date TIMESTAMPTZ,
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    results JSONB DEFAULT '{}',
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT unique_experiment_name UNIQUE(tenant_id, name)
);

-- A/B test participants
CREATE TABLE analytics.experiment_participants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experiment_id UUID NOT NULL REFERENCES analytics.experiments(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id),
    customer_id UUID REFERENCES customers(id),
    session_id VARCHAR(255),
    variant_name VARCHAR(100) NOT NULL,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    converted_at TIMESTAMPTZ,
    conversion_value DECIMAL(19,4),

    -- One assignment per participant per experiment
    CONSTRAINT unique_participant UNIQUE(experiment_id, COALESCE(user_id, customer_id, session_id))
);

-- Cohort definitions and tracking
CREATE TABLE analytics.cohorts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    definition JSONB NOT NULL, -- Criteria for cohort membership
    cohort_date DATE NOT NULL, -- Date cohort was created
    member_count INTEGER DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    -- Constraints
    CONSTRAINT unique_cohort_name UNIQUE(tenant_id, name)
);

-- Cohort membership
CREATE TABLE analytics.cohort_members (
    cohort_id UUID NOT NULL REFERENCES analytics.cohorts(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    left_at TIMESTAMPTZ,

    PRIMARY KEY (cohort_id, customer_id)
);

-- ============================================================================
-- INDEXES FOR PERFORMANCE
-- ============================================================================

-- Domain events indexes
CREATE INDEX idx_domain_events_tenant ON events.domain_events(tenant_id);
CREATE INDEX idx_domain_events_aggregate ON events.domain_events(aggregate_id, sequence_number);
CREATE INDEX idx_domain_events_type ON events.domain_events(event_type);
CREATE INDEX idx_domain_events_created ON events.domain_events(created_at);
CREATE INDEX idx_domain_events_status ON events.domain_events(status) WHERE status != 'processed';
CREATE INDEX idx_domain_events_sequence ON events.domain_events(sequence_number);

-- Event subscriptions indexes
CREATE INDEX idx_event_subscriptions_subscriber ON events.event_subscriptions(subscriber_name);
CREATE INDEX idx_event_subscriptions_active ON events.event_subscriptions(is_active) WHERE is_active = true;

-- Aggregate snapshots indexes
CREATE INDEX idx_snapshots_aggregate ON events.aggregate_snapshots(aggregate_id, version DESC);

-- Business metrics indexes
CREATE INDEX idx_business_metrics_tenant_date ON analytics.business_metrics(tenant_id, date_key);
CREATE INDEX idx_business_metrics_location_date ON analytics.business_metrics(location_id, date_key);
CREATE INDEX idx_business_metrics_name_date ON analytics.business_metrics(metric_name, date_key);
CREATE INDEX idx_business_metrics_recorded ON analytics.business_metrics(recorded_at);

-- User activities indexes
CREATE INDEX idx_user_activities_tenant_date ON analytics.user_activities(tenant_id, date_key);
CREATE INDEX idx_user_activities_user_date ON analytics.user_activities(user_id, date_key);
CREATE INDEX idx_user_activities_type_date ON analytics.user_activities(activity_type, date_key);
CREATE INDEX idx_user_activities_session ON analytics.user_activities(session_id);

-- Metric aggregations indexes
CREATE INDEX idx_aggregations_tenant_metric ON analytics.metric_aggregations(tenant_id, metric_name);
CREATE INDEX idx_aggregations_period ON analytics.metric_aggregations(aggregation_period, period_start);
CREATE INDEX idx_aggregations_location ON analytics.metric_aggregations(location_id, period_start);

-- Customer journeys indexes
CREATE INDEX idx_journeys_tenant ON analytics.customer_journeys(tenant_id);
CREATE INDEX idx_journeys_customer ON analytics.customer_journeys(customer_id);
CREATE INDEX idx_journeys_session ON analytics.customer_journeys(session_id);
CREATE INDEX idx_journeys_journey_id ON analytics.customer_journeys(journey_id, step_number);

-- Experiments indexes
CREATE INDEX idx_experiments_tenant ON analytics.experiments(tenant_id);
CREATE INDEX idx_experiments_status ON analytics.experiments(status);
CREATE INDEX idx_experiments_dates ON analytics.experiments(start_date, end_date);

-- Experiment participants indexes
CREATE INDEX idx_participants_experiment ON analytics.experiment_participants(experiment_id);
CREATE INDEX idx_participants_user ON analytics.experiment_participants(user_id);
CREATE INDEX idx_participants_customer ON analytics.experiment_participants(customer_id);

-- Cohorts indexes
CREATE INDEX idx_cohorts_tenant ON analytics.cohorts(tenant_id);
CREATE INDEX idx_cohorts_date ON analytics.cohorts(cohort_date);
CREATE INDEX idx_cohorts_active ON analytics.cohorts(is_active) WHERE is_active = true;

-- Cohort members indexes
CREATE INDEX idx_cohort_members_cohort ON analytics.cohort_members(cohort_id);
CREATE INDEX idx_cohort_members_customer ON analytics.cohort_members(customer_id);

-- ============================================================================
-- ROW LEVEL SECURITY POLICIES
-- ============================================================================

-- Enable RLS on all analytics tables
ALTER TABLE events.domain_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE events.event_subscriptions ENABLE ROW LEVEL SECURITY;
ALTER TABLE events.aggregate_snapshots ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.business_metrics ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.user_activities ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.metric_aggregations ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.customer_journeys ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.experiments ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.experiment_participants ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.cohorts ENABLE ROW LEVEL SECURITY;
ALTER TABLE analytics.cohort_members ENABLE ROW LEVEL SECURITY;

-- Tenant-based policies for most tables
CREATE POLICY domain_events_tenant_policy ON events.domain_events
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY business_metrics_tenant_policy ON analytics.business_metrics
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY user_activities_tenant_policy ON analytics.user_activities
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY metric_aggregations_tenant_policy ON analytics.metric_aggregations
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY customer_journeys_tenant_policy ON analytics.customer_journeys
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY experiments_tenant_policy ON analytics.experiments
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

CREATE POLICY cohorts_tenant_policy ON analytics.cohorts
    FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::UUID);

-- Service-level policies for event subscriptions (no tenant restriction)
CREATE POLICY event_subscriptions_service_policy ON events.event_subscriptions
    FOR ALL USING (true); -- Services need global access

CREATE POLICY aggregate_snapshots_service_policy ON events.aggregate_snapshots
    FOR ALL USING (true); -- Services need global access

-- Related table policies
CREATE POLICY experiment_participants_tenant_policy ON analytics.experiment_participants
    FOR ALL USING (
        experiment_id IN (
            SELECT id FROM analytics.experiments
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

CREATE POLICY cohort_members_tenant_policy ON analytics.cohort_members
    FOR ALL USING (
        cohort_id IN (
            SELECT id FROM analytics.cohorts
            WHERE tenant_id = current_setting('app.current_tenant_id', true)::UUID
        )
    );

-- ============================================================================
-- UPDATED_AT TRIGGERS
-- ============================================================================

-- Apply updated_at triggers to tables that need them
CREATE TRIGGER update_event_subscriptions_updated_at
    BEFORE UPDATE ON events.event_subscriptions
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_experiments_updated_at
    BEFORE UPDATE ON analytics.experiments
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

CREATE TRIGGER update_cohorts_updated_at
    BEFORE UPDATE ON analytics.cohorts
    FOR EACH ROW EXECUTE FUNCTION platform.update_updated_at_column();

-- ============================================================================
-- ANALYTICS FUNCTIONS
-- ============================================================================

-- Function to process pending events
CREATE OR REPLACE FUNCTION events.process_pending_events(batch_size INTEGER DEFAULT 100)
RETURNS INTEGER AS $$
DECLARE
    processed_count INTEGER := 0;
    event_record RECORD;
BEGIN
    FOR event_record IN
        SELECT id, event_type, event_data, tenant_id, aggregate_id
        FROM events.domain_events
        WHERE status = 'pending'
        ORDER BY sequence_number
        LIMIT batch_size
        FOR UPDATE SKIP LOCKED
    LOOP
        -- Mark as processed
        UPDATE events.domain_events
        SET
            status = 'processed',
            processed_at = CURRENT_TIMESTAMP
        WHERE id = event_record.id;

        processed_count := processed_count + 1;
    END LOOP;

    RETURN processed_count;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate cohort retention
CREATE OR REPLACE FUNCTION analytics.calculate_cohort_retention(
    cohort_uuid UUID,
    period_days INTEGER DEFAULT 30
)
RETURNS TABLE(
    period_number INTEGER,
    customers_active INTEGER,
    retention_rate DECIMAL(5,4)
) AS $$
BEGIN
    RETURN QUERY
    WITH cohort_base AS (
        SELECT c.cohort_date, COUNT(*) as total_customers
        FROM analytics.cohorts c
        JOIN analytics.cohort_members cm ON c.id = cm.cohort_id
        WHERE c.id = cohort_uuid
        GROUP BY c.cohort_date
    ),
    periods AS (
        SELECT generate_series(0, 12) as period_num
    ),
    period_activity AS (
        SELECT
            p.period_num,
            COUNT(DISTINCT o.customer_id) as active_customers
        FROM periods p
        LEFT JOIN analytics.cohort_members cm ON TRUE
        LEFT JOIN orders o ON o.customer_id = cm.customer_id
            AND o.created_at >= (
                SELECT cohort_date + (p.period_num * period_days || ' days')::INTERVAL
                FROM analytics.cohorts WHERE id = cohort_uuid
            )
            AND o.created_at < (
                SELECT cohort_date + ((p.period_num + 1) * period_days || ' days')::INTERVAL
                FROM analytics.cohorts WHERE id = cohort_uuid
            )
        WHERE cm.cohort_id = cohort_uuid
        GROUP BY p.period_num
    )
    SELECT
        pa.period_num::INTEGER,
        pa.active_customers::INTEGER,
        ROUND(pa.active_customers::DECIMAL / cb.total_customers, 4)
    FROM period_activity pa
    CROSS JOIN cohort_base cb
    ORDER BY pa.period_num;
END;
$$ LANGUAGE plpgsql;

-- ============================================================================
-- GRANTS AND PERMISSIONS
-- ============================================================================

-- Grant permissions on events schema tables
GRANT SELECT, INSERT, UPDATE, DELETE ON events.domain_events TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON events.event_subscriptions TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON events.aggregate_snapshots TO olympus_app;

-- Grant permissions on analytics schema tables
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.business_metrics TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.user_activities TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.metric_aggregations TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.customer_journeys TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.experiments TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.experiment_participants TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.cohorts TO olympus_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON analytics.cohort_members TO olympus_app;

-- Grant execute permissions on functions
GRANT EXECUTE ON FUNCTION events.process_pending_events(INTEGER) TO olympus_app;
GRANT EXECUTE ON FUNCTION analytics.calculate_cohort_retention(UUID, INTEGER) TO olympus_app;