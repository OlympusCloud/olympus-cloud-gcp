use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use redis::aio::ConnectionManager;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub tenant_id: Uuid,
    pub data: serde_json::Value,
    pub metadata: EventMetadata,
    pub version: i32,
    pub occurred_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub user_id: Option<Uuid>,
    pub correlation_id: Uuid,
    pub causation_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

// Authentication Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub login_method: String, // password, oauth, device
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLogoutEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequestedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub reset_token_hash: String,
    pub expires_at: DateTime<Utc>,
}

// Commerce Events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub total_amount: i64, // in cents
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProcessedEvent {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub status: String,
}

// Event Publisher
pub struct EventPublisher {
    redis: ConnectionManager,
}

impl EventPublisher {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(Self { redis })
    }

    pub async fn publish(&mut self, event: &DomainEvent) -> Result<()> {
        let channel = format!("events.{}", event.event_type);
        let payload = serde_json::to_string(event)?;

        redis::cmd("PUBLISH")
            .arg(&channel)
            .arg(&payload)
            .query_async(&mut self.redis)
            .await?;

        // Also publish to tenant-specific channel
        let tenant_channel = format!("events.tenant.{}", event.tenant_id);
        redis::cmd("PUBLISH")
            .arg(&tenant_channel)
            .arg(&payload)
            .query_async(&mut self.redis)
            .await?;

        Ok(())
    }

    pub async fn publish_batch(&mut self, events: &[DomainEvent]) -> Result<()> {
        for event in events {
            self.publish(event).await?;
        }
        Ok(())
    }
}

// Event Builder
impl DomainEvent {
    pub fn builder(event_type: String, aggregate_id: Uuid, tenant_id: Uuid) -> DomainEventBuilder {
        DomainEventBuilder {
            event_type,
            aggregate_id,
            tenant_id,
            data: serde_json::Value::Null,
            metadata: EventMetadata {
                user_id: None,
                correlation_id: Uuid::new_v4(),
                causation_id: None,
                ip_address: None,
                user_agent: None,
            },
            version: 1,
        }
    }
}

pub struct DomainEventBuilder {
    event_type: String,
    aggregate_id: Uuid,
    tenant_id: Uuid,
    data: serde_json::Value,
    metadata: EventMetadata,
    version: i32,
}

impl DomainEventBuilder {
    pub fn data(mut self, data: impl Serialize) -> Result<Self> {
        self.data = serde_json::to_value(data)?;
        Ok(self)
    }

    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.metadata.user_id = Some(user_id);
        self
    }

    pub fn correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.metadata.correlation_id = correlation_id;
        self
    }

    pub fn causation_id(mut self, causation_id: Uuid) -> Self {
        self.metadata.causation_id = Some(causation_id);
        self
    }

    pub fn ip_address(mut self, ip_address: String) -> Self {
        self.metadata.ip_address = Some(ip_address);
        self
    }

    pub fn user_agent(mut self, user_agent: String) -> Self {
        self.metadata.user_agent = Some(user_agent);
        self
    }

    pub fn version(mut self, version: i32) -> Self {
        self.version = version;
        self
    }

    pub fn build(self) -> DomainEvent {
        DomainEvent {
            id: Uuid::new_v4(),
            event_type: self.event_type,
            aggregate_id: self.aggregate_id,
            tenant_id: self.tenant_id,
            data: self.data,
            metadata: self.metadata,
            version: self.version,
            occurred_at: Utc::now(),
        }
    }
}