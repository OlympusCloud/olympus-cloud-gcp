// ============================================================================
// OLYMPUS CLOUD - EVENT SYSTEM FOUNDATION
// ============================================================================
// Module: shared/src/events/mod.rs
// Description: Domain event definitions and event-driven architecture
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

pub mod publisher;
pub mod subscriber;
pub mod types;

pub use publisher::{EventPublisher, PublishError};
pub use subscriber::{EventSubscriber, EventHandler, SubscriptionConfig};
pub use types::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Base domain event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub aggregate_type: String,
    pub tenant_id: Uuid,
    pub data: serde_json::Value,
    pub metadata: EventMetadata,
    pub version: i64,
    pub occurred_at: DateTime<Utc>,
}

/// Event metadata for context and tracing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub user_id: Option<Uuid>,
    pub correlation_id: Uuid,
    pub causation_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub source_service: String,
    pub event_source: String,          // "api", "background_job", "system"
    pub trace_id: Option<String>,
}

impl DomainEvent {
    /// Create a new domain event builder
    pub fn builder(
        event_type: String,
        aggregate_id: Uuid,
        aggregate_type: String,
        tenant_id: Uuid,
    ) -> DomainEventBuilder {
        DomainEventBuilder {
            event_type,
            aggregate_id,
            aggregate_type,
            tenant_id,
            data: serde_json::Value::Null,
            metadata: EventMetadata {
                user_id: None,
                correlation_id: Uuid::new_v4(),
                causation_id: None,
                ip_address: None,
                user_agent: None,
                source_service: "unknown".to_string(),
                event_source: "api".to_string(),
                trace_id: None,
            },
            version: 1,
        }
    }

    /// Get event stream name for this event
    pub fn stream_name(&self) -> String {
        format!("{}-{}", self.aggregate_type.to_lowercase(), self.aggregate_id)
    }

    /// Get event topic for pub/sub
    pub fn topic(&self) -> String {
        format!("events.{}.{}", self.aggregate_type.to_lowercase(), self.event_type)
    }

    /// Get tenant-specific topic
    pub fn tenant_topic(&self) -> String {
        format!("events.tenant.{}.{}", self.tenant_id, self.event_type)
    }

    /// Check if event should be published globally
    pub fn is_global_event(&self) -> bool {
        matches!(
            self.event_type.as_str(),
            "TenantCreated" | "TenantDeleted" | "SystemMaintenanceScheduled"
        )
    }
}

/// Builder for creating domain events
pub struct DomainEventBuilder {
    event_type: String,
    aggregate_id: Uuid,
    aggregate_type: String,
    tenant_id: Uuid,
    data: serde_json::Value,
    metadata: EventMetadata,
    version: i64,
}

impl DomainEventBuilder {
    /// Set event data
    pub fn data<T: Serialize>(mut self, data: T) -> crate::Result<Self> {
        self.data = serde_json::to_value(data)?;
        Ok(self)
    }

    /// Set user context
    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.metadata.user_id = Some(user_id);
        self
    }

    /// Set correlation ID for request tracing
    pub fn correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.metadata.correlation_id = correlation_id;
        self
    }

    /// Set causation ID (previous event that caused this one)
    pub fn causation_id(mut self, causation_id: Uuid) -> Self {
        self.metadata.causation_id = Some(causation_id);
        self
    }

    /// Set IP address
    pub fn ip_address(mut self, ip_address: String) -> Self {
        self.metadata.ip_address = Some(ip_address);
        self
    }

    /// Set user agent
    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.metadata.user_agent = Some(user_agent);
        self
    }

    /// Set source service
    pub fn source_service(mut self, service: String) -> Self {
        self.metadata.source_service = service;
        self
    }

    /// Set event source
    pub fn event_source(mut self, source: String) -> Self {
        self.metadata.event_source = source;
        self
    }

    /// Set trace ID for distributed tracing
    pub fn trace_id(mut self, trace_id: String) -> Self {
        self.metadata.trace_id = Some(trace_id);
        self
    }

    /// Set event version
    pub fn version(mut self, version: i64) -> Self {
        self.version = version;
        self
    }

    /// Build the domain event
    pub fn build(self) -> DomainEvent {
        DomainEvent {
            id: Uuid::new_v4(),
            event_type: self.event_type,
            aggregate_id: self.aggregate_id,
            aggregate_type: self.aggregate_type,
            tenant_id: self.tenant_id,
            data: self.data,
            metadata: self.metadata,
            version: self.version,
            occurred_at: Utc::now(),
        }
    }
}

/// Event publishing configuration
#[derive(Debug, Clone)]
pub struct EventConfig {
    pub redis_url: String,
    pub max_retries: u32,
    pub retry_delay: std::time::Duration,
    pub batch_size: usize,
    pub batch_timeout: std::time::Duration,
    pub enable_dead_letter_queue: bool,
    pub dead_letter_topic: String,
}

impl Default for EventConfig {
    fn default() -> Self {
        Self {
            redis_url: "redis://localhost:6379".to_string(),
            max_retries: 3,
            retry_delay: std::time::Duration::from_millis(1000),
            batch_size: 100,
            batch_timeout: std::time::Duration::from_secs(5),
            enable_dead_letter_queue: true,
            dead_letter_topic: "events.dead_letter".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_event_creation() {
        let tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let aggregate_id = Uuid::new_v4();

        let event = DomainEvent::builder(
            "UserCreated".to_string(),
            aggregate_id,
            "User".to_string(),
            tenant_id,
        )
        .user_id(user_id)
        .data(serde_json::json!({"email": "test@example.com"}))
        .unwrap()
        .build();

        assert_eq!(event.event_type, "UserCreated");
        assert_eq!(event.aggregate_id, aggregate_id);
        assert_eq!(event.tenant_id, tenant_id);
        assert_eq!(event.metadata.user_id, Some(user_id));
    }

    #[test]
    fn test_event_topics() {
        let event = DomainEvent::builder(
            "UserCreated".to_string(),
            Uuid::new_v4(),
            "User".to_string(),
            Uuid::new_v4(),
        )
        .build();

        assert_eq!(event.topic(), "events.user.UserCreated");
        assert!(event.tenant_topic().starts_with("events.tenant."));
    }

    #[test]
    fn test_stream_name() {
        let aggregate_id = Uuid::new_v4();
        let event = DomainEvent::builder(
            "UserCreated".to_string(),
            aggregate_id,
            "User".to_string(),
            Uuid::new_v4(),
        )
        .build();

        assert_eq!(event.stream_name(), format!("user-{}", aggregate_id));
    }

    #[test]
    fn test_global_events() {
        let event = DomainEvent::builder(
            "TenantCreated".to_string(),
            Uuid::new_v4(),
            "Tenant".to_string(),
            Uuid::new_v4(),
        )
        .build();

        assert!(event.is_global_event());

        let event = DomainEvent::builder(
            "UserCreated".to_string(),
            Uuid::new_v4(),
            "User".to_string(),
            Uuid::new_v4(),
        )
        .build();

        assert!(!event.is_global_event());
    }
}