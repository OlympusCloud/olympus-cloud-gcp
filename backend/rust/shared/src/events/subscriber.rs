// ============================================================================
// OLYMPUS CLOUD - EVENT SUBSCRIBER
// ============================================================================
// Module: shared/src/events/subscriber.rs
// Description: Redis-based event subscription and handling
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::DomainEvent;
use crate::{Error, Result};
use async_trait::async_trait;
use futures::StreamExt;
use redis::aio::ConnectionManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

/// Event handler trait for processing events
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle a received event
    async fn handle(&self, event: &DomainEvent) -> Result<()>;

    /// Get the event types this handler is interested in
    fn event_types(&self) -> Vec<String>;

    /// Get handler name for logging
    fn name(&self) -> String;

    /// Handle errors during event processing
    async fn handle_error(&self, event: &DomainEvent, error: &Error) {
        error!(
            "Handler {} failed to process event {}: {}",
            self.name(),
            event.id,
            error
        );
    }
}

/// Subscription configuration
#[derive(Debug, Clone)]
pub struct SubscriptionConfig {
    pub name: String,
    pub topics: Vec<String>,
    pub consumer_group: Option<String>,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub batch_size: usize,
    pub prefetch_count: usize,
    pub auto_ack: bool,
}

impl Default for SubscriptionConfig {
    fn default() -> Self {
        Self {
            name: "default_subscriber".to_string(),
            topics: vec![],
            consumer_group: None,
            max_retries: 3,
            retry_delay: Duration::from_millis(1000),
            batch_size: 10,
            prefetch_count: 100,
            auto_ack: true,
        }
    }
}

/// Event subscriber for consuming events from Redis
pub struct EventSubscriber {
    redis: ConnectionManager,
    handlers: Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
    config: SubscriptionConfig,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl EventSubscriber {
    /// Create a new event subscriber
    pub async fn new(redis_url: &str, config: SubscriptionConfig) -> Result<Self> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| Error::Configuration(format!("Invalid Redis URL: {}", e)))?;

        let redis = ConnectionManager::new(client)
            .await
            .map_err(|e| Error::Redis(e))?;

        info!("Event subscriber connected to Redis");

        Ok(Self {
            redis,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            config,
            shutdown_tx: None,
        })
    }

    /// Register an event handler
    pub async fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        let handler_name = handler.name();
        let event_types = handler.event_types();

        let mut handlers = self.handlers.write().await;
        handlers.insert(handler_name.clone(), handler);

        info!(
            "Registered event handler '{}' for event types: {:?}",
            handler_name, event_types
        );
    }

    /// Start subscribing to events
    pub async fn start(&mut self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let redis = self.redis.clone();
        let handlers = Arc::clone(&self.handlers);
        let config = self.config.clone();

        // Start subscription task
        tokio::spawn(async move {
            let mut subscriber = Self::create_subscriber(redis.clone(), &config).await;

            if let Err(e) = subscriber {
                error!("Failed to create subscriber: {}", e);
                return;
            }

            let mut subscriber = subscriber.unwrap();

            loop {
                tokio::select! {
                    result = Self::receive_events(&mut subscriber, &handlers, &config) => {
                        if let Err(e) = result {
                            error!("Error receiving events: {}", e);
                            sleep(Duration::from_secs(1)).await;
                        }
                    }

                    _ = shutdown_rx.recv() => {
                        info!("Shutting down event subscriber");
                        break;
                    }
                }
            }
        });

        info!("Event subscriber started with config: {:?}", self.config.name);
        Ok(())
    }

    /// Create Redis subscriber
    async fn create_subscriber(
        redis: ConnectionManager,
        config: &SubscriptionConfig,
    ) -> Result<redis::aio::PubSub> {
        let client = redis::Client::open("redis://localhost:6379")
            .map_err(|e| Error::Configuration(format!("Invalid Redis URL: {}", e)))?;

        let mut pubsub = client.get_async_connection().await
            .map_err(Error::Redis)?
            .into_pubsub();

        // Subscribe to configured topics
        for topic in &config.topics {
            pubsub.subscribe(topic).await.map_err(Error::Redis)?;
            info!("Subscribed to topic: {}", topic);
        }

        // Subscribe to pattern-based topics if needed
        if config.topics.is_empty() {
            pubsub.psubscribe("events.*").await.map_err(Error::Redis)?;
            info!("Subscribed to pattern: events.*");
        }

        Ok(pubsub)
    }

    /// Receive and process events
    async fn receive_events(
        subscriber: &mut redis::aio::PubSub,
        handlers: &Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
        config: &SubscriptionConfig,
    ) -> Result<()> {
        let msg = subscriber.on_message().next().await.ok_or_else(|| Error::Internal("No message received".to_string()))?;
        let channel: String = msg.get_channel_name().to_string();
        let payload: String = msg.get_payload().map_err(Error::Redis)?;

        debug!("Received message on channel '{}': {}", channel, payload);

        // Parse the event
        let event: DomainEvent = match serde_json::from_str(&payload) {
            Ok(event) => event,
            Err(e) => {
                warn!("Failed to parse event from channel '{}': {}", channel, e);
                return Ok(());
            }
        };

        // Process the event with applicable handlers
        Self::process_event(&event, handlers, config).await
    }

    /// Process an event with registered handlers
    async fn process_event(
        event: &DomainEvent,
        handlers: &Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
        config: &SubscriptionConfig,
    ) -> Result<()> {
        let handlers_map = handlers.read().await;
        let mut processing_tasks = Vec::new();

        // Find handlers interested in this event type
        for (handler_name, handler) in handlers_map.iter() {
            if handler.event_types().contains(&event.event_type) ||
               handler.event_types().contains(&"*".to_string()) {

                let handler_clone = Arc::clone(handler);
                let event_clone = event.clone();
                let config_clone = config.clone();

                // Process each handler concurrently
                let task = tokio::spawn(async move {
                    Self::handle_event_with_retry(
                        &*handler_clone,
                        &event_clone,
                        &config_clone,
                    ).await
                });

                processing_tasks.push(task);
            }
        }

        // Wait for all handlers to complete
        for task in processing_tasks {
            if let Err(e) = task.await {
                error!("Handler task failed: {}", e);
            }
        }

        debug!("Event {} processed by all applicable handlers", event.id);
        Ok(())
    }

    /// Handle an event with retry logic
    async fn handle_event_with_retry(
        handler: &dyn EventHandler,
        event: &DomainEvent,
        config: &SubscriptionConfig,
    ) -> Result<()> {
        let mut retry_count = 0;

        loop {
            match handler.handle(event).await {
                Ok(()) => {
                    debug!(
                        "Handler '{}' successfully processed event {}",
                        handler.name(),
                        event.id
                    );
                    return Ok(());
                }
                Err(e) if retry_count < config.max_retries => {
                    retry_count += 1;
                    warn!(
                        "Handler '{}' failed to process event {} (attempt {}): {}",
                        handler.name(),
                        event.id,
                        retry_count,
                        e
                    );

                    sleep(config.retry_delay * retry_count).await;
                }
                Err(e) => {
                    error!(
                        "Handler '{}' failed to process event {} after {} retries: {}",
                        handler.name(),
                        event.id,
                        config.max_retries,
                        e
                    );

                    handler.handle_error(event, &e).await;
                    return Err(e);
                }
            }
        }
    }

    /// Subscribe to tenant-specific events
    pub async fn subscribe_to_tenant(&mut self, tenant_id: uuid::Uuid) -> Result<()> {
        let topic = format!("events.tenant.{}", tenant_id);

        // This would require reconnecting the pubsub, which is complex
        // In practice, you might want to restart the subscriber or use multiple connections
        warn!("Dynamic tenant subscription not implemented - restart subscriber with new config");

        Ok(())
    }

    /// Get subscription statistics
    pub async fn get_stats(&self) -> SubscriptionStats {
        let handlers = self.handlers.read().await;

        SubscriptionStats {
            handler_count: handlers.len(),
            subscribed_topics: self.config.topics.clone(),
            is_running: self.shutdown_tx.is_some(),
        }
    }

    /// Graceful shutdown
    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }

        info!("Event subscriber shutdown complete");
        Ok(())
    }
}

/// Subscription statistics
#[derive(Debug)]
pub struct SubscriptionStats {
    pub handler_count: usize,
    pub subscribed_topics: Vec<String>,
    pub is_running: bool,
}

/// Example event handler implementation
pub struct LoggingEventHandler {
    name: String,
    event_types: Vec<String>,
}

impl LoggingEventHandler {
    pub fn new(name: String, event_types: Vec<String>) -> Self {
        Self { name, event_types }
    }
}

#[async_trait]
impl EventHandler for LoggingEventHandler {
    async fn handle(&self, event: &DomainEvent) -> Result<()> {
        info!(
            "Handler '{}' processing event: {} (type: {}, tenant: {})",
            self.name, event.id, event.event_type, event.tenant_id
        );

        // Simulate some processing time
        sleep(Duration::from_millis(10)).await;

        Ok(())
    }

    fn event_types(&self) -> Vec<String> {
        self.event_types.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_logging_handler() {
        let handler = LoggingEventHandler::new(
            "test_handler".to_string(),
            vec!["TestEvent".to_string()],
        );

        let event = DomainEvent::builder(
            "TestEvent".to_string(),
            Uuid::new_v4(),
            "Test".to_string(),
            Uuid::new_v4(),
        )
        .build();

        assert!(handler.handle(&event).await.is_ok());
        assert_eq!(handler.name(), "test_handler");
        assert!(handler.event_types().contains(&"TestEvent".to_string()));
    }

    #[test]
    fn test_subscription_config() {
        let config = SubscriptionConfig::default();
        assert_eq!(config.name, "default_subscriber");
        assert_eq!(config.max_retries, 3);
        assert!(config.auto_ack);
    }
}