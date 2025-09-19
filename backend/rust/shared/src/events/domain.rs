// ============================================================================
// OLYMPUS CLOUD - COMPREHENSIVE DOMAIN EVENTS (Phase 5)
// ============================================================================
// Module: shared/src/events/domain.rs
// Description: Domain events with versioning, context, and cross-service communication
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 2.0 - Enhanced for Phase 5 Event-Driven Architecture
// ============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use std::collections::HashMap;

// Re-export existing event types for compatibility
pub use super::types::*;

// ============================================================================
// EVENT VERSION MANAGEMENT
// ============================================================================

/// Event version information for backward compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl EventVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn v1_0_0() -> Self {
        Self::new(1, 0, 0)
    }

    pub fn v2_0_0() -> Self {
        Self::new(2, 0, 0)
    }

    pub fn is_compatible_with(&self, other: &EventVersion) -> bool {
        self.major == other.major
    }
}

impl std::fmt::Display for EventVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ============================================================================
// ENHANCED EVENT CONTEXT
// ============================================================================

/// Enhanced event context for cross-service communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    pub user_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub correlation_id: Uuid,
    pub causation_id: Option<Uuid>, // ID of the event that caused this one
    pub request_id: Option<String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub source_service: String,
    pub source_version: String,
    pub environment: String, // "development", "staging", "production"
    pub client_info: Option<ClientInfo>,
    pub business_context: BusinessContext,
    pub custom_metadata: HashMap<String, serde_json::Value>,
}

/// Client information for event context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub device_id: Option<String>,
    pub platform: Option<String>, // "web", "ios", "android", "api"
    pub app_version: Option<String>,
    pub geolocation: Option<GeolocationData>,
}

/// Business context for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessContext {
    pub tenant_id: Uuid,
    pub location_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub business_date: DateTime<Utc>, // Business date for reporting
    pub fiscal_period: Option<String>,
    pub campaign_id: Option<Uuid>,
}

// ============================================================================
// VERSIONED DOMAIN EVENT
// ============================================================================

/// Enhanced domain event with versioning and context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedDomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub event_version: EventVersion,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_version: i64, // Optimistic concurrency control
    pub sequence_number: i64,   // Event ordering within aggregate
    pub data: serde_json::Value,
    pub context: EventContext,
    pub occurred_at: DateTime<Utc>,
    pub recorded_at: DateTime<Utc>,
    pub checksum: Option<String>, // Event integrity verification
}

impl VersionedDomainEvent {
    /// Create builder for versioned domain event
    pub fn builder(
        event_type: String,
        aggregate_id: Uuid,
        aggregate_type: String,
        tenant_id: Uuid,
    ) -> VersionedDomainEventBuilder {
        VersionedDomainEventBuilder::new(event_type, aggregate_id, aggregate_type, tenant_id)
    }

    /// Get event stream name with versioning
    pub fn stream_name(&self) -> String {
        format!("{}-{}-v{}",
            self.aggregate_type.to_lowercase(),
            self.aggregate_id,
            self.event_version.major
        )
    }

    /// Get versioned topic for pub/sub
    pub fn topic(&self) -> String {
        format!("events.v{}.{}.{}",
            self.event_version.major,
            self.aggregate_type.to_lowercase(),
            self.event_type
        )
    }

    /// Get tenant-specific versioned topic
    pub fn tenant_topic(&self) -> String {
        format!("events.tenant.{}.v{}.{}",
            self.context.business_context.tenant_id,
            self.event_version.major,
            self.event_type
        )
    }

    /// Get location-specific topic if location is available
    pub fn location_topic(&self) -> Option<String> {
        self.context.business_context.location_id.map(|location_id| {
            format!("events.location.{}.v{}.{}",
                location_id,
                self.event_version.major,
                self.event_type
            )
        })
    }

    /// Check if event should be replicated globally
    pub fn is_global_event(&self) -> bool {
        matches!(
            self.event_type.as_str(),
            "TenantCreated" | "TenantDeleted" | "SystemMaintenanceScheduled" |
            "SecurityIncidentDetected" | "ComplianceViolationDetected"
        )
    }

    /// Check if event contains sensitive data
    pub fn is_sensitive(&self) -> bool {
        matches!(
            self.event_type.as_str(),
            "UserRegistered" | "PasswordChanged" | "PaymentProcessed" |
            "PiiDataAccessed" | "SecurityEvent"
        )
    }

    /// Get retention period for this event type
    pub fn retention_days(&self) -> u32 {
        match self.event_type.as_str() {
            // Financial events - 7 years
            "PaymentProcessed" | "OrderCreated" | "RefundProcessed" => 7 * 365,
            // Security events - 2 years
            "UserLoggedIn" | "UserLoggedOut" | "SecurityEvent" => 2 * 365,
            // System events - 90 days
            "SystemEvent" | "HealthCheckEvent" => 90,
            // Default - 1 year
            _ => 365,
        }
    }

    /// Calculate event checksum for integrity verification
    pub fn calculate_checksum(&self) -> String {
        use sha2::{Sha256, Digest};

        let data = format!("{}:{}:{}:{}:{}:{}",
            self.event_type,
            self.aggregate_id,
            self.aggregate_version,
            self.sequence_number,
            self.occurred_at.timestamp(),
            serde_json::to_string(&self.data).unwrap_or_default()
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Verify event integrity
    pub fn verify_integrity(&self) -> bool {
        if let Some(ref stored_checksum) = self.checksum {
            let calculated_checksum = self.calculate_checksum();
            stored_checksum == &calculated_checksum
        } else {
            // No checksum to verify
            true
        }
    }
}

// ============================================================================
// VERSIONED EVENT BUILDER
// ============================================================================

/// Builder for creating versioned domain events
pub struct VersionedDomainEventBuilder {
    event_type: String,
    event_version: EventVersion,
    aggregate_id: Uuid,
    aggregate_type: String,
    aggregate_version: i64,
    sequence_number: i64,
    data: serde_json::Value,
    context: EventContext,
}

impl VersionedDomainEventBuilder {
    pub fn new(
        event_type: String,
        aggregate_id: Uuid,
        aggregate_type: String,
        tenant_id: Uuid,
    ) -> Self {
        Self {
            event_type,
            event_version: EventVersion::v2_0_0(),
            aggregate_id,
            aggregate_type,
            aggregate_version: 1,
            sequence_number: 1,
            data: serde_json::Value::Null,
            context: EventContext {
                user_id: None,
                session_id: None,
                correlation_id: Uuid::new_v4(),
                causation_id: None,
                request_id: None,
                trace_id: None,
                span_id: None,
                source_service: "unknown".to_string(),
                source_version: "1.0.0".to_string(),
                environment: "development".to_string(),
                client_info: None,
                business_context: BusinessContext {
                    tenant_id,
                    location_id: None,
                    department_id: None,
                    business_date: Utc::now(),
                    fiscal_period: None,
                    campaign_id: None,
                },
                custom_metadata: HashMap::new(),
            },
        }
    }

    /// Set event version
    pub fn version(mut self, version: EventVersion) -> Self {
        self.event_version = version;
        self
    }

    /// Set aggregate version for optimistic locking
    pub fn aggregate_version(mut self, version: i64) -> Self {
        self.aggregate_version = version;
        self
    }

    /// Set sequence number for event ordering
    pub fn sequence_number(mut self, number: i64) -> Self {
        self.sequence_number = number;
        self
    }

    /// Set event data
    pub fn data<T: Serialize>(mut self, data: T) -> crate::Result<Self> {
        self.data = serde_json::to_value(data)?;
        Ok(self)
    }

    /// Set user context
    pub fn user_context(mut self, user_id: Uuid, session_id: Option<String>) -> Self {
        self.context.user_id = Some(user_id);
        self.context.session_id = session_id;
        self
    }

    /// Set correlation ID for request tracing
    pub fn correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.context.correlation_id = correlation_id;
        self
    }

    /// Set causation ID (previous event that caused this one)
    pub fn causation_id(mut self, causation_id: Uuid) -> Self {
        self.context.causation_id = Some(causation_id);
        self
    }

    /// Set tracing information
    pub fn trace_info(mut self, trace_id: String, span_id: Option<String>) -> Self {
        self.context.trace_id = Some(trace_id);
        self.context.span_id = span_id;
        self
    }

    /// Set source service information
    pub fn source(mut self, service: String, version: String) -> Self {
        self.context.source_service = service;
        self.context.source_version = version;
        self
    }

    /// Set environment
    pub fn environment(mut self, environment: String) -> Self {
        self.context.environment = environment;
        self
    }

    /// Set client information
    pub fn client_info(mut self, client_info: ClientInfo) -> Self {
        self.context.client_info = Some(client_info);
        self
    }

    /// Set location context
    pub fn location_id(mut self, location_id: Uuid) -> Self {
        self.context.business_context.location_id = Some(location_id);
        self
    }

    /// Set business date
    pub fn business_date(mut self, date: DateTime<Utc>) -> Self {
        self.context.business_context.business_date = date;
        self
    }

    /// Add custom metadata
    pub fn metadata<T: Serialize>(mut self, key: String, value: T) -> crate::Result<Self> {
        self.context.custom_metadata.insert(key, serde_json::to_value(value)?);
        Ok(self)
    }

    /// Build the versioned domain event
    pub fn build(self) -> VersionedDomainEvent {
        let now = Utc::now();
        let mut event = VersionedDomainEvent {
            id: Uuid::new_v4(),
            event_type: self.event_type,
            event_version: self.event_version,
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            aggregate_version: self.aggregate_version,
            sequence_number: self.sequence_number,
            data: self.data,
            context: self.context,
            occurred_at: now,
            recorded_at: now,
            checksum: None,
        };

        // Calculate and set checksum
        event.checksum = Some(event.calculate_checksum());
        event
    }
}

// ============================================================================
// SPECIALIZED DOMAIN EVENTS FOR PHASE 5
// ============================================================================

/// Enhanced authentication events with additional context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAuthenticationAttemptEvent {
    pub user_id: Option<Uuid>, // None for failed attempts with invalid users
    pub tenant_id: Uuid,
    pub email: String,
    pub attempt_result: AuthenticationResult,
    pub failure_reason: Option<AuthenticationFailureReason>,
    pub client_info: ClientInfo,
    pub security_context: SecurityContext,
    pub mfa_required: bool,
    pub mfa_completed: bool,
}

/// Enhanced order lifecycle events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLifecycleEvent {
    pub order_id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub location_id: Uuid,
    pub lifecycle_stage: OrderLifecycleStage,
    pub previous_stage: Option<OrderLifecycleStage>,
    pub transition_reason: String,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub staff_member_id: Option<Uuid>,
    pub automated_transition: bool,
    pub business_metrics: OrderBusinessMetrics,
}

/// Inventory movement events with detailed tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryMovementEvent {
    pub movement_id: Uuid,
    pub product_id: Uuid,
    pub tenant_id: Uuid,
    pub from_location_id: Option<Uuid>,
    pub to_location_id: Option<Uuid>,
    pub movement_type: InventoryMovementType,
    pub quantity_moved: i32,
    pub unit_cost: Option<Decimal>,
    pub total_value: Option<Decimal>,
    pub reference_document_id: Option<Uuid>,
    pub reference_document_type: Option<String>,
    pub batch_number: Option<String>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub serial_numbers: Vec<String>,
    pub reason_code: String,
    pub performed_by: Option<Uuid>,
}

// ============================================================================
// SUPPORTING ENUMS AND STRUCTS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationResult {
    Success,
    Failed,
    Blocked,
    RequiresMfa,
    AccountLocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationFailureReason {
    InvalidCredentials,
    AccountNotFound,
    AccountDisabled,
    AccountLocked,
    TooManyAttempts,
    InvalidMfaCode,
    ExpiredSession,
    InvalidToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    pub risk_score: f64,
    pub threat_indicators: Vec<String>,
    pub previous_login_location: Option<String>,
    pub device_fingerprint: Option<String>,
    pub is_suspicious: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderLifecycleStage {
    Created,
    Confirmed,
    Preparing,
    Ready,
    InDelivery,
    Delivered,
    Completed,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBusinessMetrics {
    pub order_value: Decimal,
    pub profit_margin: Option<Decimal>,
    pub preparation_time_minutes: Option<i32>,
    pub customer_wait_time_minutes: Option<i32>,
    pub discount_amount: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub tip_amount: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryMovementType {
    Sale,
    Return,
    Transfer,
    Adjustment,
    Damage,
    Theft,
    Expiry,
    Promotion,
    Recount,
    Receiving,
    Shipping,
}

// ============================================================================
// EVENT FACTORY HELPERS
// ============================================================================

/// Factory for creating standardized events
pub struct EventFactory {
    source_service: String,
    source_version: String,
    environment: String,
}

impl EventFactory {
    pub fn new(source_service: String, source_version: String, environment: String) -> Self {
        Self {
            source_service,
            source_version,
            environment,
        }
    }

    /// Create a user authentication attempt event
    pub fn user_authentication_attempt(
        &self,
        tenant_id: Uuid,
        email: String,
        result: AuthenticationResult,
        client_info: ClientInfo,
    ) -> crate::Result<VersionedDomainEvent> {
        let event_data = UserAuthenticationAttemptEvent {
            user_id: None, // Set by caller if known
            tenant_id,
            email,
            attempt_result: result,
            failure_reason: None,
            client_info,
            security_context: SecurityContext {
                risk_score: 0.0,
                threat_indicators: vec![],
                previous_login_location: None,
                device_fingerprint: None,
                is_suspicious: false,
            },
            mfa_required: false,
            mfa_completed: false,
        };

        VersionedDomainEvent::builder(
            "UserAuthenticationAttempt".to_string(),
            Uuid::new_v4(), // Generate a unique ID for this attempt
            "Authentication".to_string(),
            tenant_id,
        )
        .version(EventVersion::v2_0_0())
        .source(self.source_service.clone(), self.source_version.clone())
        .environment(self.environment.clone())
        .data(event_data)?
        .build()
        .let_result(Ok)
    }

    /// Create an order lifecycle event
    pub fn order_lifecycle(
        &self,
        order_id: Uuid,
        tenant_id: Uuid,
        location_id: Uuid,
        stage: OrderLifecycleStage,
        previous_stage: Option<OrderLifecycleStage>,
    ) -> crate::Result<VersionedDomainEvent> {
        let event_data = OrderLifecycleEvent {
            order_id,
            tenant_id,
            customer_id: None,
            location_id,
            lifecycle_stage: stage,
            previous_stage,
            transition_reason: "Automatic transition".to_string(),
            estimated_completion: None,
            staff_member_id: None,
            automated_transition: true,
            business_metrics: OrderBusinessMetrics {
                order_value: Decimal::ZERO,
                profit_margin: None,
                preparation_time_minutes: None,
                customer_wait_time_minutes: None,
                discount_amount: None,
                tax_amount: None,
                tip_amount: None,
            },
        };

        VersionedDomainEvent::builder(
            "OrderLifecycle".to_string(),
            order_id,
            "Order".to_string(),
            tenant_id,
        )
        .version(EventVersion::v2_0_0())
        .source(self.source_service.clone(), self.source_version.clone())
        .environment(self.environment.clone())
        .location_id(location_id)
        .data(event_data)?
        .build()
        .let_result(Ok)
    }
}

// Extension trait to allow .let_result() for cleaner builder patterns
trait LetResult<T> {
    fn let_result<F, R>(self, f: F) -> R
    where
        F: FnOnce(T) -> R;
}

impl<T> LetResult<T> for T {
    fn let_result<F, R>(self, f: F) -> R
    where
        F: FnOnce(T) -> R,
    {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_version_compatibility() {
        let v1_0_0 = EventVersion::v1_0_0();
        let v1_1_0 = EventVersion::new(1, 1, 0);
        let v2_0_0 = EventVersion::v2_0_0();

        assert!(v1_0_0.is_compatible_with(&v1_1_0));
        assert!(!v1_0_0.is_compatible_with(&v2_0_0));
    }

    #[test]
    fn test_versioned_event_creation() {
        let tenant_id = Uuid::new_v4();
        let aggregate_id = Uuid::new_v4();

        let event = VersionedDomainEvent::builder(
            "TestEvent".to_string(),
            aggregate_id,
            "Test".to_string(),
            tenant_id,
        )
        .version(EventVersion::v2_0_0())
        .data(serde_json::json!({"test": "data"}))
        .unwrap()
        .build();

        assert_eq!(event.event_type, "TestEvent");
        assert_eq!(event.aggregate_id, aggregate_id);
        assert_eq!(event.context.business_context.tenant_id, tenant_id);
        assert!(event.checksum.is_some());
        assert!(event.verify_integrity());
    }

    #[test]
    fn test_event_topics() {
        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        let event = VersionedDomainEvent::builder(
            "OrderCreated".to_string(),
            Uuid::new_v4(),
            "Order".to_string(),
            tenant_id,
        )
        .location_id(location_id)
        .build();

        assert_eq!(event.topic(), "events.v2.order.OrderCreated");
        assert_eq!(event.tenant_topic(), format!("events.tenant.{}.v2.OrderCreated", tenant_id));
        assert_eq!(event.location_topic(), Some(format!("events.location.{}.v2.OrderCreated", location_id)));
    }

    #[test]
    fn test_event_checksum_verification() {
        let event = VersionedDomainEvent::builder(
            "TestEvent".to_string(),
            Uuid::new_v4(),
            "Test".to_string(),
            Uuid::new_v4(),
        )
        .data(serde_json::json!({"important": "data"}))
        .unwrap()
        .build();

        // Event should verify its own integrity
        assert!(event.verify_integrity());

        // Create a corrupted event
        let mut corrupted_event = event.clone();
        corrupted_event.data = serde_json::json!({"corrupted": "data"});

        // Corrupted event should not verify
        assert!(!corrupted_event.verify_integrity());
    }

    #[test]
    fn test_event_factory() {
        let factory = EventFactory::new(
            "test-service".to_string(),
            "1.0.0".to_string(),
            "test".to_string(),
        );

        let client_info = ClientInfo {
            ip_address: "127.0.0.1".to_string(),
            user_agent: Some("Test Agent".to_string()),
            device_id: None,
            platform: Some("test".to_string()),
            app_version: None,
            geolocation: None,
        };

        let event = factory.user_authentication_attempt(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            AuthenticationResult::Success,
            client_info,
        ).unwrap();

        assert_eq!(event.event_type, "UserAuthenticationAttempt");
        assert_eq!(event.context.source_service, "test-service");
        assert_eq!(event.context.environment, "test");
    }
}