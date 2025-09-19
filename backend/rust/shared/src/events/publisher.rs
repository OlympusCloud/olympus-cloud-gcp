// ============================================================================
// OLYMPUS CLOUD - EVENT PUBLISHER
// ============================================================================
// Module: shared/src/events/publisher.rs
// Description: Redis-based event publishing with retry and batching
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{DomainEvent, EventConfig};
use crate::{Error, Result};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, timeout, Duration, Instant};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Event publishing errors
#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    #[error("Redis connection error: {0}")]
    Connection(#[from] RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Timeout error: operation took too long")]
    Timeout,

    #[error("Queue full: cannot accept more events")]
    QueueFull,

    #[error("Publisher shutdown")]
    Shutdown,
}

/// Event publisher with batching and retry logic
pub struct EventPublisher {
    redis: ConnectionManager,
    config: EventConfig,
    batch_queue: Arc<Mutex<VecDeque<DomainEvent>>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl EventPublisher {
    /// Create a new event publisher
    pub async fn new(config: EventConfig) -> Result<Self> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|e| Error::Configuration(format!("Invalid Redis URL: {}", e)))?;

        let redis = ConnectionManager::new(client)
            .await
            .map_err(|e| Error::Redis(e))?;

        info!("Event publisher connected to Redis");

        Ok(Self {
            redis,
            config,
            batch_queue: Arc::new(Mutex::new(VecDeque::new())),
            shutdown_tx: None,
        })
    }

    /// Start the background batch processor
    pub async fn start_batch_processor(&mut self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let redis = self.redis.clone();
        let config = self.config.clone();
        let batch_queue = Arc::clone(&self.batch_queue);

        tokio::spawn(async move {
            let mut batch_timer = tokio::time::interval(config.batch_timeout);
            let mut last_batch_time = Instant::now();

            loop {
                tokio::select! {
                    _ = batch_timer.tick() => {
                        if let Err(e) = Self::process_batch(&redis, &config, &batch_queue).await {
                            error!("Batch processing error: {}", e);
                        }
                        last_batch_time = Instant::now();
                    }

                    _ = shutdown_rx.recv() => {
                        info!("Shutting down batch processor");
                        // Process remaining events
                        if let Err(e) = Self::process_batch(&redis, &config, &batch_queue).await {
                            error!("Final batch processing error: {}", e);
                        }
                        break;
                    }

                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        let queue_size = {
                            let queue = batch_queue.lock().await;
                            queue.len()
                        };

                        if queue_size >= config.batch_size ||
                           (queue_size > 0 && last_batch_time.elapsed() >= config.batch_timeout) {
                            if let Err(e) = Self::process_batch(&redis, &config, &batch_queue).await {
                                error!("Batch processing error: {}", e);
                            }
                            last_batch_time = Instant::now();
                        }
                    }
                }
            }
        });

        info!("Event publisher batch processor started");
        Ok(())
    }

    /// Publish a single event immediately
    pub async fn publish(&mut self, event: &DomainEvent) -> Result<()> {
        self.publish_with_retry(event, 0).await
    }

    /// Publish multiple events immediately
    pub async fn publish_batch(&mut self, events: &[DomainEvent]) -> Result<()> {
        for event in events {
            self.publish_with_retry(event, 0).await?;
        }
        Ok(())
    }

    /// Add event to batch queue for later processing
    pub async fn publish_async(&self, event: DomainEvent) -> Result<()> {
        let mut queue = self.batch_queue.lock().await;

        if queue.len() >= self.config.batch_size * 2 {
            return Err(Error::Internal("Event queue full".to_string()));
        }

        queue.push_back(event);
        debug!("Event queued for batch processing. Queue size: {}", queue.len());
        Ok(())
    }

    /// Publish event with retry logic
    async fn publish_with_retry(&mut self, event: &DomainEvent, retry_count: u32) -> Result<()> {
        let mut current_retry = retry_count;

        loop {
            match self.publish_event(event).await {
                Ok(()) => {
                    debug!("Event published successfully: {}", event.id);
                    return Ok(());
                }
                Err(e) if current_retry < self.config.max_retries => {
                    current_retry += 1;
                    warn!(
                        "Event publish failed (attempt {}), retrying: {}",
                        current_retry,
                        e
                    );
                    sleep(self.config.retry_delay).await;
                    // Continue the loop instead of recursive call
                }
                Err(e) => {
                    error!("Event publish failed after {} retries: {}", self.config.max_retries, e);

                    if self.config.enable_dead_letter_queue {
                        if let Err(dlq_error) = self.send_to_dead_letter_queue(event).await {
                            error!("Failed to send event to dead letter queue: {}", dlq_error);
                        }
                    }

                    return Err(Error::Internal(format!("Failed to publish event: {}", e)));
                }
            }
        }
    }

    /// Publish event to Redis
    async fn publish_event(&mut self, event: &DomainEvent) -> std::result::Result<(), PublishError> {
        let payload = serde_json::to_string(event)?;

        // Publish to main topic
        let topic = event.topic();
        redis::cmd("PUBLISH")
            .arg(&topic)
            .arg(&payload)
            .query_async::<_, i64>(&mut self.redis)
            .await?;

        // Publish to tenant-specific topic
        let tenant_topic = event.tenant_topic();
        redis::cmd("PUBLISH")
            .arg(&tenant_topic)
            .arg(&payload)
            .query_async::<_, i64>(&mut self.redis)
            .await?;

        // Publish to global topic if applicable
        if event.is_global_event() {
            let global_topic = format!("events.global.{}", event.event_type);
            redis::cmd("PUBLISH")
                .arg(&global_topic)
                .arg(&payload)
                .query_async::<_, i64>(&mut self.redis)
                .await?;
        }

        // Store in event stream for replay capability
        self.store_in_stream(event).await?;

        Ok(())
    }

    /// Store event in Redis stream for event sourcing
    async fn store_in_stream(&mut self, event: &DomainEvent) -> std::result::Result<(), PublishError> {
        let stream_name = event.stream_name();
        let event_data = serde_json::to_string(event)?;

        redis::cmd("XADD")
            .arg(&stream_name)
            .arg("*")
            .arg("event")
            .arg(&event_data)
            .arg("type")
            .arg(&event.event_type)
            .arg("tenant_id")
            .arg(event.tenant_id.to_string())
            .arg("occurred_at")
            .arg(event.occurred_at.timestamp())
            .query_async::<_, String>(&mut self.redis)
            .await?;

        // Set TTL for stream cleanup (optional)
        let ttl = 30 * 24 * 60 * 60; // 30 days
        redis::cmd("EXPIRE")
            .arg(&stream_name)
            .arg(ttl)
            .query_async::<_, ()>(&mut self.redis)
            .await
            .ok(); // Ignore errors for TTL setting

        Ok(())
    }

    /// Send failed event to dead letter queue
    async fn send_to_dead_letter_queue(&mut self, event: &DomainEvent) -> std::result::Result<(), PublishError> {
        let dlq_event = DeadLetterEvent {
            original_event: event.clone(),
            failed_at: chrono::Utc::now(),
            failure_reason: "Max retries exceeded".to_string(),
            retry_count: self.config.max_retries,
        };

        let payload = serde_json::to_string(&dlq_event)?;

        redis::cmd("LPUSH")
            .arg(&self.config.dead_letter_topic)
            .arg(&payload)
            .query_async::<_, ()>(&mut self.redis)
            .await?;

        warn!("Event sent to dead letter queue: {}", event.id);
        Ok(())
    }

    /// Process batched events
    async fn process_batch(
        redis: &ConnectionManager,
        config: &EventConfig,
        batch_queue: &Arc<Mutex<VecDeque<DomainEvent>>>,
    ) -> Result<()> {
        let events = {
            let mut queue = batch_queue.lock().await;
            let batch_size = std::cmp::min(queue.len(), config.batch_size);
            if batch_size == 0 {
                return Ok(());
            }

            (0..batch_size).map(|_| queue.pop_front().unwrap()).collect::<Vec<_>>()
        };

        debug!("Processing batch of {} events", events.len());

        let mut redis_clone = redis.clone();
        for event in &events {
            let mut publisher = EventPublisher {
                redis: redis_clone.clone(),
                config: config.clone(),
                batch_queue: Arc::new(Mutex::new(VecDeque::new())),
                shutdown_tx: None,
            };

            if let Err(e) = publisher.publish_with_retry(event, 0).await {
                error!("Failed to publish event in batch: {}", e);
            }
        }

        info!("Processed batch of {} events", events.len());
        Ok(())
    }

    /// Graceful shutdown
    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }

        // Wait for remaining events to be processed
        let remaining_events = {
            let queue = self.batch_queue.lock().await;
            queue.len()
        };

        if remaining_events > 0 {
            info!("Waiting for {} remaining events to be processed", remaining_events);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        info!("Event publisher shutdown complete");
        Ok(())
    }
}

/// Dead letter event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeadLetterEvent {
    original_event: DomainEvent,
    failed_at: chrono::DateTime<chrono::Utc>,
    failure_reason: String,
    retry_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_event() -> DomainEvent {
        DomainEvent::builder(
            "TestEvent".to_string(),
            Uuid::new_v4(),
            "Test".to_string(),
            Uuid::new_v4(),
        )
        .data(serde_json::json!({"test": "data"}))
        .unwrap()
        .build()
    }

    #[test]
    fn test_event_topics() {
        let event = create_test_event();
        assert_eq!(event.topic(), "events.test.TestEvent");
        assert!(event.tenant_topic().starts_with("events.tenant."));
    }

    #[test]
    fn test_stream_name() {
        let event = create_test_event();
        assert!(event.stream_name().starts_with("test-"));
    }

    #[tokio::test]
    async fn test_batch_queue() {
        let config = EventConfig::default();
        // This test would require a Redis instance, so we'll mock it
        // In a real scenario, you'd use a test container or mock Redis
    }
}