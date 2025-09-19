// ============================================================================
// OLYMPUS CLOUD - ENHANCED EVENT SUBSCRIBER (Phase 5)
// ============================================================================
// Module: shared/src/events/subscriber.rs
// Description: Advanced Redis-based event subscription with replay and processing
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 2.0 - Enhanced for Phase 5 Event-Driven Architecture
// ============================================================================

use super::{DomainEvent, VersionedDomainEvent, EventContainer};
use crate::{Error, Result};
use async_trait::async_trait;
use futures::StreamExt;
use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{atomic::AtomicU64, Arc};
use tokio::sync::{mpsc, RwLock, Semaphore};
use tokio::time::{sleep, timeout, Duration, Instant};
use tracing::{debug, error, info, warn, instrument};
use uuid::Uuid;
use std::sync::atomic::Ordering;

/// Enhanced event handler trait for processing both legacy and versioned events
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle a received event container
    async fn handle(&self, event: &EventContainer) -> Result<()>;

    /// Get the event types this handler is interested in
    fn event_types(&self) -> Vec<String>;

    /// Get handler name for logging
    fn name(&self) -> String;

    /// Get handler priority for processing order
    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Normal
    }

    /// Check if handler can process events concurrently
    fn supports_concurrent_processing(&self) -> bool {
        false
    }

    /// Get maximum number of concurrent events this handler can process
    fn max_concurrent_events(&self) -> usize {
        1
    }

    /// Handle errors during event processing with enhanced context
    async fn handle_error(&self, event: &EventContainer, error: &Error, attempt: u32) {
        let event_id = match event {
            EventContainer::Legacy(e) => e.id.to_string(),
            EventContainer::Versioned(e) => e.id.to_string(),
        };

        error!(
            "Handler {} failed to process event {} (attempt {}): {}",
            self.name(),
            event_id,
            attempt,
            error
        );
    }

    /// Called before processing starts (for initialization)
    async fn before_processing(&self) -> Result<()> {
        Ok(())
    }

    /// Called after processing completes (for cleanup)
    async fn after_processing(&self) -> Result<()> {
        Ok(())
    }

    /// Get handler health status
    async fn health_check(&self) -> HandlerHealth {
        HandlerHealth::Healthy
    }
}

/// Handler priority for processing order
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandlerPriority {
    Critical = 0,
    High = 1,
    Normal = 2,
    Low = 3,
}

/// Handler health status
#[derive(Debug, Clone, PartialEq)]
pub enum HandlerHealth {
    Healthy,
    Degraded(String),
    Unhealthy(String),
}

/// Enhanced subscription configuration for Phase 5
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

    // Phase 5 enhancements
    pub enable_replay: bool,
    pub replay_from: Option<chrono::DateTime<chrono::Utc>>,
    pub max_concurrent_handlers: usize,
    pub handler_timeout: Duration,
    pub enable_dead_letter_processing: bool,
    pub enable_metrics: bool,
    pub health_check_interval: Duration,
    pub enable_duplicate_detection: bool,
    pub duplicate_detection_window: Duration,
    pub enable_ordering: bool,
    pub max_processing_time: Duration,
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

            // Phase 5 defaults
            enable_replay: false,
            replay_from: None,
            max_concurrent_handlers: 10,
            handler_timeout: Duration::from_secs(30),
            enable_dead_letter_processing: true,
            enable_metrics: true,
            health_check_interval: Duration::from_secs(60),
            enable_duplicate_detection: true,
            duplicate_detection_window: Duration::from_secs(300),
            enable_ordering: true,
            max_processing_time: Duration::from_secs(300),
        }
    }
}

/// Enhanced event subscriber with replay, ordering, and advanced processing
pub struct EventSubscriber {
    redis: ConnectionManager,
    handlers: Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
    config: SubscriptionConfig,
    shutdown_tx: Option<mpsc::Sender<()>>,
    processed_events: Arc<RwLock<HashSet<String>>>,
    handler_semaphore: Arc<Semaphore>,
    metrics: Arc<RwLock<SubscriberMetrics>>,
    last_processed_timestamp: Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
    processing_queue: Arc<RwLock<Vec<QueuedEventForProcessing>>>,
}

/// Metrics for subscriber performance monitoring
#[derive(Debug, Clone)]
pub struct SubscriberMetrics {
    pub events_received: u64,
    pub events_processed: u64,
    pub events_failed: u64,
    pub events_duplicated: u64,
    pub events_replayed: u64,
    pub average_processing_time_ms: f64,
    pub handlers_active: usize,
    pub queue_depth: usize,
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

/// Queued event for processing with metadata
#[derive(Debug, Clone)]
struct QueuedEventForProcessing {
    event: EventContainer,
    received_at: Instant,
    attempts: u32,
    handler_name: String,
    priority: HandlerPriority,
}

impl EventSubscriber {
    /// Create a new enhanced event subscriber
    pub async fn new(redis_url: &str, config: SubscriptionConfig) -> Result<Self> {
        let client = redis::Client::open(redis_url)
            .map_err(|e| Error::Configuration(format!("Invalid Redis URL: {}", e)))?;

        let redis = ConnectionManager::new(client)
            .await
            .map_err(|e| Error::Redis(e))?;

        info!("Enhanced event subscriber connected to Redis");

        Ok(Self {
            redis,
            handlers: Arc::new(RwLock::new(HashMap::new())),
            config: config.clone(),
            shutdown_tx: None,
            processed_events: Arc::new(RwLock::new(HashSet::new())),
            handler_semaphore: Arc::new(Semaphore::new(config.max_concurrent_handlers)),
            metrics: Arc::new(RwLock::new(SubscriberMetrics {
                events_received: 0,
                events_processed: 0,
                events_failed: 0,
                events_duplicated: 0,
                events_replayed: 0,
                average_processing_time_ms: 0.0,
                handlers_active: 0,
                queue_depth: 0,
                last_health_check: chrono::Utc::now(),
            })),
            last_processed_timestamp: Arc::new(RwLock::new(None)),
            processing_queue: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Register an enhanced event handler
    pub async fn register_handler(&self, handler: Arc<dyn EventHandler>) -> Result<()> {
        let handler_name = handler.name();
        let event_types = handler.event_types();
        let priority = handler.priority();
        let concurrent_support = handler.supports_concurrent_processing();
        let max_concurrent = handler.max_concurrent_events();

        // Initialize handler
        handler.before_processing().await?;

        // Check handler health
        let health = handler.health_check().await;
        if matches!(health, HandlerHealth::Unhealthy(_)) {
            return Err(Error::Internal(format!("Handler {} is unhealthy: {:?}", handler_name, health)));
        }

        let mut handlers = self.handlers.write().await;
        handlers.insert(handler_name.clone(), handler);

        info!(
            "Registered enhanced event handler '{}' for event types: {:?}, priority: {:?}, concurrent: {}, max_concurrent: {}",
            handler_name, event_types, priority, concurrent_support, max_concurrent
        );

        Ok(())
    }

    /// Start enhanced subscribing with replay and advanced processing
    pub async fn start(&mut self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        // Start replay if configured
        if self.config.enable_replay {
            self.start_replay_process().await?;
        }

        let redis = self.redis.clone();
        let handlers = Arc::clone(&self.handlers);
        let config = self.config.clone();
        let processed_events = Arc::clone(&self.processed_events);
        let handler_semaphore = Arc::clone(&self.handler_semaphore);
        let metrics = Arc::clone(&self.metrics);
        let last_processed_timestamp = Arc::clone(&self.last_processed_timestamp);
        let processing_queue = Arc::clone(&self.processing_queue);

        // Start health check task
        let health_check_handle = self.start_health_check_task();

        // Start metrics reporting task
        let metrics_handle = self.start_metrics_task();

        // Start processing queue task
        let queue_processor_handle = self.start_queue_processor();

        // Start subscription task
        tokio::spawn(async move {
            let mut subscriber = Self::create_enhanced_subscriber(redis.clone(), &config).await;

            if let Err(e) = subscriber {
                error!("Failed to create enhanced subscriber: {}", e);
                return;
            }

            let mut subscriber = subscriber.unwrap();

            loop {
                tokio::select! {
                    result = Self::receive_enhanced_events(
                        &mut subscriber,
                        &handlers,
                        &config,
                        &processed_events,
                        &handler_semaphore,
                        &metrics,
                        &last_processed_timestamp,
                        &processing_queue,
                    ) => {
                        if let Err(e) = result {
                            error!("Error receiving enhanced events: {}", e);
                            sleep(Duration::from_secs(1)).await;
                        }
                    }

                    _ = shutdown_rx.recv() => {
                        info!("Shutting down enhanced event subscriber");

                        // Cleanup tasks
                        health_check_handle.abort();
                        metrics_handle.abort();
                        queue_processor_handle.abort();

                        // Wait for handlers to complete
                        Self::graceful_handler_shutdown(&handlers).await;

                        break;
                    }
                }
            }
        });

        info!("Enhanced event subscriber started with config: {:?}", self.config.name);
        Ok(())
    }

    /// Create enhanced Redis subscriber with consumer groups
    async fn create_enhanced_subscriber(
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
            info!("Enhanced subscriber subscribed to topic: {}", topic);
        }

        // Subscribe to pattern-based topics if needed
        if config.topics.is_empty() {
            pubsub.psubscribe("events.*").await.map_err(Error::Redis)?;
            info!("Enhanced subscriber subscribed to pattern: events.*");
        }

        // Subscribe to versioned event patterns
        pubsub.psubscribe("events.v*.*.* ").await.map_err(Error::Redis)?;
        info!("Enhanced subscriber subscribed to versioned event pattern");

        // Subscribe to dead letter queue if enabled
        if config.enable_dead_letter_processing {
            pubsub.subscribe("events.dead_letter").await.map_err(Error::Redis)?;
            info!("Enhanced subscriber subscribed to dead letter queue");
        }

        Ok(pubsub)
    }

    /// Receive and process enhanced events with versioning support
    #[instrument(skip_all)]
    async fn receive_enhanced_events(
        subscriber: &mut redis::aio::PubSub,
        handlers: &Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
        config: &SubscriptionConfig,
        processed_events: &Arc<RwLock<HashSet<String>>>,
        handler_semaphore: &Arc<Semaphore>,
        metrics: &Arc<RwLock<SubscriberMetrics>>,
        last_processed_timestamp: &Arc<RwLock<Option<chrono::DateTime<chrono::Utc>>>>,
        processing_queue: &Arc<RwLock<Vec<QueuedEventForProcessing>>>,
    ) -> Result<()> {
        let msg = subscriber.on_message().next().await.ok_or_else(|| Error::Internal("No message received".to_string()))?;
        let channel: String = msg.get_channel_name().to_string();
        let payload: String = msg.get_payload().map_err(Error::Redis)?;

        debug!("Enhanced subscriber received message on channel '{}'", channel);

        // Update metrics
        {
            let mut metrics_guard = metrics.write().await;
            metrics_guard.events_received += 1;
        }

        // Parse the event (try versioned first, then legacy)
        let event_container = match Self::parse_event_container(&payload) {
            Ok(container) => container,
            Err(e) => {
                warn!("Failed to parse event from channel '{}': {}", channel, e);
                return Ok(());
            }
        };

        // Check for duplicates if enabled
        if config.enable_duplicate_detection {
            let event_id = match &event_container {
                EventContainer::Legacy(e) => e.id.to_string(),
                EventContainer::Versioned(e) => e.id.to_string(),
            };

            {
                let processed = processed_events.read().await;
                if processed.contains(&event_id) {
                    let mut metrics_guard = metrics.write().await;
                    metrics_guard.events_duplicated += 1;
                    debug!("Duplicate event detected: {}", event_id);
                    return Ok(());
                }
            }

            // Add to processed events
            {
                let mut processed = processed_events.write().await;
                processed.insert(event_id);
            }
        }

        // Update last processed timestamp
        {
            let mut timestamp = last_processed_timestamp.write().await;
            *timestamp = Some(chrono::Utc::now());
        }

        // Process the event with enhanced handling
        Self::process_enhanced_event(
            &event_container,
            handlers,
            config,
            handler_semaphore,
            metrics,
            processing_queue,
        ).await
    }

    /// Process an enhanced event with registered handlers
    #[instrument(skip_all, fields(event_type = %Self::get_event_type(&event_container)))]
    async fn process_enhanced_event(
        event_container: &EventContainer,
        handlers: &Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>,
        config: &SubscriptionConfig,
        handler_semaphore: &Arc<Semaphore>,
        metrics: &Arc<RwLock<SubscriberMetrics>>,
        processing_queue: &Arc<RwLock<Vec<QueuedEventForProcessing>>>,
    ) -> Result<()> {
        let event_type = Self::get_event_type(event_container);
        let handlers_map = handlers.read().await;
        let mut processing_tasks = Vec::new();

        // Find handlers interested in this event type
        let mut applicable_handlers = Vec::new();
        for (handler_name, handler) in handlers_map.iter() {
            if handler.event_types().contains(&event_type) ||
               handler.event_types().contains(&"*".to_string()) {
                applicable_handlers.push((handler_name.clone(), Arc::clone(handler)));
            }
        }

        // Sort handlers by priority
        applicable_handlers.sort_by_key(|(_, handler)| handler.priority());

        drop(handlers_map); // Release the read lock

        // Process handlers based on their concurrency support
        for (handler_name, handler) in applicable_handlers {
            if handler.supports_concurrent_processing() {
                // Process concurrently with semaphore control
                let handler_clone = Arc::clone(&handler);
                let event_clone = event_container.clone();
                let config_clone = config.clone();
                let semaphore_clone = Arc::clone(handler_semaphore);
                let metrics_clone = Arc::clone(metrics);

                let task = tokio::spawn(async move {
                    let _permit = semaphore_clone.acquire().await.unwrap();
                    let start_time = Instant::now();

                    let result = Self::handle_enhanced_event_with_retry(
                        &*handler_clone,
                        &event_clone,
                        &config_clone,
                    ).await;

                    let processing_time = start_time.elapsed().as_millis() as f64;
                    Self::update_processing_metrics(&metrics_clone, processing_time, result.is_ok()).await;

                    result
                });

                processing_tasks.push(task);
            } else {
                // Process sequentially
                if config.enable_ordering {
                    let mut queue = processing_queue.write().await;
                    queue.push(QueuedEventForProcessing {
                        event: event_container.clone(),
                        received_at: Instant::now(),
                        attempts: 0,
                        handler_name: handler_name.clone(),
                        priority: handler.priority(),
                    });
                } else {
                    // Process immediately but sequentially
                    let start_time = Instant::now();
                    let result = Self::handle_enhanced_event_with_retry(
                        &*handler,
                        event_container,
                        config,
                    ).await;

                    let processing_time = start_time.elapsed().as_millis() as f64;
                    Self::update_processing_metrics(metrics, processing_time, result.is_ok()).await;

                    if let Err(e) = result {
                        error!("Sequential handler '{}' failed: {}", handler_name, e);
                    }
                }
            }
        }

        // Wait for all concurrent handlers to complete
        for task in processing_tasks {
            if let Err(e) = task.await {
                error!("Concurrent handler task failed: {}", e);
            }
        }

        let event_id = match event_container {
            EventContainer::Legacy(e) => e.id.to_string(),
            EventContainer::Versioned(e) => e.id.to_string(),
        };

        debug!("Enhanced event {} processed by all applicable handlers", event_id);
        Ok(())
    }

    /// Handle an enhanced event with retry logic and timeout
    #[instrument(skip_all, fields(handler_name = %handler.name()))]
    async fn handle_enhanced_event_with_retry(
        handler: &dyn EventHandler,
        event: &EventContainer,
        config: &SubscriptionConfig,
    ) -> Result<()> {
        let mut retry_count = 0;
        let event_id = match event {
            EventContainer::Legacy(e) => e.id.to_string(),
            EventContainer::Versioned(e) => e.id.to_string(),
        };

        loop {
            // Apply timeout to handler processing
            let handle_result = timeout(config.handler_timeout, handler.handle(event)).await;

            match handle_result {
                Ok(Ok(())) => {
                    debug!(
                        "Enhanced handler '{}' successfully processed event {}",
                        handler.name(),
                        event_id
                    );
                    return Ok(());
                }
                Ok(Err(e)) if retry_count < config.max_retries => {
                    retry_count += 1;
                    warn!(
                        "Enhanced handler '{}' failed to process event {} (attempt {}): {}",
                        handler.name(),
                        event_id,
                        retry_count,
                        e
                    );

                    // Exponential backoff
                    let delay = config.retry_delay * (2_u32.pow(retry_count.saturating_sub(1)));
                    sleep(delay).await;
                }
                Err(_timeout_error) => {
                    error!(
                        "Enhanced handler '{}' timed out processing event {} after {:?}",
                        handler.name(),
                        event_id,
                        config.handler_timeout
                    );

                    let timeout_error = Error::Internal(format!("Handler timeout after {:?}", config.handler_timeout));
                    handler.handle_error(event, &timeout_error, retry_count + 1).await;
                    return Err(timeout_error);
                }
                Ok(Err(e)) => {
                    error!(
                        "Enhanced handler '{}' failed to process event {} after {} retries: {}",
                        handler.name(),
                        event_id,
                        config.max_retries,
                        e
                    );

                    handler.handle_error(event, &e, retry_count + 1).await;
                    return Err(e);
                }
            }
        }
    }

    /// Parse event container from JSON payload
    fn parse_event_container(payload: &str) -> Result<EventContainer> {
        // Try to parse as versioned event first
        if let Ok(versioned_event) = serde_json::from_str::<VersionedDomainEvent>(payload) {
            return Ok(EventContainer::Versioned(versioned_event));
        }

        // Fall back to legacy event
        if let Ok(legacy_event) = serde_json::from_str::<DomainEvent>(payload) {
            return Ok(EventContainer::Legacy(legacy_event));
        }

        Err(Error::Internal("Failed to parse event container".to_string()))
    }

    /// Get event type from container
    fn get_event_type(event: &EventContainer) -> String {
        match event {
            EventContainer::Legacy(e) => e.event_type.clone(),
            EventContainer::Versioned(e) => e.event_type.clone(),
        }
    }

    /// Update processing metrics
    async fn update_processing_metrics(
        metrics: &Arc<RwLock<SubscriberMetrics>>,
        processing_time_ms: f64,
        success: bool,
    ) {
        let mut metrics_guard = metrics.write().await;
        if success {
            metrics_guard.events_processed += 1;
        } else {
            metrics_guard.events_failed += 1;
        }
        metrics_guard.average_processing_time_ms =
            (metrics_guard.average_processing_time_ms * 0.9) + (processing_time_ms * 0.1);
    }

    /// Start event replay process
    async fn start_replay_process(&self) -> Result<()> {
        if let Some(replay_from) = self.config.replay_from {
            info!("Starting event replay from: {}", replay_from);
            // TODO: Implement event replay from Redis streams
            // This would read from Redis streams starting from the specified timestamp
        }
        Ok(())
    }

    /// Start health check task
    fn start_health_check_task(&self) -> tokio::task::JoinHandle<()> {
        let handlers = Arc::clone(&self.handlers);
        let config = self.config.clone();
        let metrics = Arc::clone(&self.metrics);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(config.health_check_interval);
            loop {
                interval.tick().await;

                let handlers_map = handlers.read().await;
                let mut healthy_count = 0;
                let mut unhealthy_handlers = Vec::new();

                for (name, handler) in handlers_map.iter() {
                    match handler.health_check().await {
                        HandlerHealth::Healthy => healthy_count += 1,
                        HandlerHealth::Degraded(reason) => {
                            warn!("Handler '{}' is degraded: {}", name, reason);
                            healthy_count += 1; // Still count as operational
                        }
                        HandlerHealth::Unhealthy(reason) => {
                            error!("Handler '{}' is unhealthy: {}", name, reason);
                            unhealthy_handlers.push(name.clone());
                        }
                    }
                }

                let mut metrics_guard = metrics.write().await;
                metrics_guard.handlers_active = healthy_count;
                metrics_guard.last_health_check = chrono::Utc::now();

                if !unhealthy_handlers.is_empty() {
                    error!("Unhealthy handlers detected: {:?}", unhealthy_handlers);
                }
            }
        })
    }

    /// Start metrics reporting task
    fn start_metrics_task(&self) -> tokio::task::JoinHandle<()> {
        let metrics = Arc::clone(&self.metrics);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;

                let metrics_guard = metrics.read().await;
                info!(
                    "Subscriber metrics: received={}, processed={}, failed={}, duplicated={}, replayed={}, avg_processing_time={}ms, handlers_active={}, queue_depth={}",
                    metrics_guard.events_received,
                    metrics_guard.events_processed,
                    metrics_guard.events_failed,
                    metrics_guard.events_duplicated,
                    metrics_guard.events_replayed,
                    metrics_guard.average_processing_time_ms,
                    metrics_guard.handlers_active,
                    metrics_guard.queue_depth
                );
            }
        })
    }

    /// Start queue processor for ordered events
    fn start_queue_processor(&self) -> tokio::task::JoinHandle<()> {
        let processing_queue = Arc::clone(&self.processing_queue);
        let handlers = Arc::clone(&self.handlers);
        let config = self.config.clone();
        let metrics = Arc::clone(&self.metrics);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;

                let mut queue = processing_queue.write().await;
                if queue.is_empty() {
                    continue;
                }

                // Sort by priority
                queue.sort_by_key(|item| item.priority.clone());

                if let Some(mut queued_event) = queue.pop() {
                    drop(queue); // Release the lock

                    let handlers_map = handlers.read().await;
                    if let Some(handler) = handlers_map.get(&queued_event.handler_name) {
                        let handler_clone = Arc::clone(handler);
                        drop(handlers_map);

                        queued_event.attempts += 1;
                        let start_time = Instant::now();

                        let result = Self::handle_enhanced_event_with_retry(
                            &*handler_clone,
                            &queued_event.event,
                            &config,
                        ).await;

                        let processing_time = start_time.elapsed().as_millis() as f64;
                        Self::update_processing_metrics(&metrics, processing_time, result.is_ok()).await;

                        if let Err(e) = result {
                            error!("Queued handler '{}' failed: {}", queued_event.handler_name, e);
                        }
                    }
                }

                // Update queue depth metric
                let queue_size = {
                    let queue = processing_queue.read().await;
                    queue.len()
                };

                let mut metrics_guard = metrics.write().await;
                metrics_guard.queue_depth = queue_size;
            }
        })
    }

    /// Graceful handler shutdown
    async fn graceful_handler_shutdown(handlers: &Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>>) {
        let handlers_map = handlers.read().await;

        for (name, handler) in handlers_map.iter() {
            if let Err(e) = handler.after_processing().await {
                error!("Handler '{}' cleanup failed: {}", name, e);
            }
        }
    }

    /// Subscribe to tenant-specific events dynamically
    pub async fn subscribe_to_tenant(&mut self, tenant_id: uuid::Uuid) -> Result<()> {
        let topic = format!("events.tenant.{}", tenant_id);
        warn!("Dynamic tenant subscription not fully implemented - restart subscriber with new config");
        info!("Would subscribe to topic: {}", topic);
        Ok(())
    }

    /// Get enhanced subscription statistics
    pub async fn get_stats(&self) -> SubscriptionStats {
        let handlers = self.handlers.read().await;
        let metrics = self.metrics.read().await;
        let queue_size = {
            let queue = self.processing_queue.read().await;
            queue.len()
        };

        SubscriptionStats {
            handler_count: handlers.len(),
            subscribed_topics: self.config.topics.clone(),
            is_running: self.shutdown_tx.is_some(),
            metrics: metrics.clone(),
            queue_depth: queue_size,
        }
    }

    /// Get subscriber metrics
    pub async fn get_metrics(&self) -> SubscriberMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }

    /// Enhanced graceful shutdown
    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }

        // Wait for processing queue to drain
        let mut attempts = 0;
        const MAX_SHUTDOWN_ATTEMPTS: u32 = 20;

        while attempts < MAX_SHUTDOWN_ATTEMPTS {
            let queue_size = {
                let queue = self.processing_queue.read().await;
                queue.len()
            };

            if queue_size == 0 {
                break;
            }

            info!("Waiting for {} queued events to process (attempt {})", queue_size, attempts + 1);
            tokio::time::sleep(Duration::from_millis(250)).await;
            attempts += 1;
        }

        // Final metrics report
        let final_metrics = self.get_metrics().await;
        info!("Final subscriber metrics: {:?}", final_metrics);

        info!("Enhanced event subscriber shutdown complete");
        Ok(())
    }
}

/// Enhanced subscription statistics
#[derive(Debug)]
pub struct SubscriptionStats {
    pub handler_count: usize,
    pub subscribed_topics: Vec<String>,
    pub is_running: bool,
    pub metrics: SubscriberMetrics,
    pub queue_depth: usize,
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
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        match event {
            EventContainer::Legacy(event) => {
                info!(
                    "Handler '{}' processing legacy event: {} (type: {}, tenant: {})",
                    self.name, event.id, event.event_type, event.tenant_id
                );
            }
            EventContainer::Versioned(event) => {
                info!(
                    "Handler '{}' processing versioned event: {} (type: {}, tenant: {})",
                    self.name, event.id, event.event_type, event.context.business_context.tenant_id
                );
            }
        }

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

        let event_container = EventContainer::Legacy(event);
        assert!(handler.handle(&event_container).await.is_ok());
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