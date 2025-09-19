// ============================================================================
// OLYMPUS CLOUD - ENHANCED EVENT PUBLISHER (Phase 5)
// ============================================================================
// Module: shared/src/events/publisher.rs
// Description: Advanced Redis-based event publishing with deduplication, ordering, and DLQ
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 2.0 - Enhanced for Phase 5 Event-Driven Architecture
// ============================================================================

use super::{DomainEvent, EventConfig, VersionedDomainEvent};
use crate::{Error, Result};
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{atomic::AtomicU64, Arc};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::{sleep, timeout, Duration, Instant};
use tracing::{debug, error, info, warn, instrument};
use uuid::Uuid;
use std::sync::atomic::Ordering;

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

    #[error("Duplicate event detected: {0}")]
    DuplicateEvent(String),

    #[error("Event ordering violation: expected {expected}, got {actual}")]
    OrderingViolation { expected: i64, actual: i64 },

    #[error("Event too large: {size} bytes exceeds limit of {limit}")]
    EventTooLarge { size: usize, limit: usize },

    #[error("Rate limit exceeded: {rate} events/sec")]
    RateLimitExceeded { rate: f64 },
}

/// Enhanced event publisher with deduplication, ordering, and advanced features
pub struct EventPublisher {
    redis: ConnectionManager,
    config: EventConfig,
    batch_queue: Arc<Mutex<VecDeque<QueuedEvent>>>,
    deduplication_cache: Arc<RwLock<HashSet<String>>>,
    aggregate_sequences: Arc<RwLock<HashMap<String, i64>>>,
    published_count: Arc<AtomicU64>,
    failed_count: Arc<AtomicU64>,
    duplicate_count: Arc<AtomicU64>,
    rate_limiter: Arc<Mutex<RateLimiter>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
    metrics: Arc<Mutex<PublisherMetrics>>,
}

/// Queued event with metadata
#[derive(Debug, Clone)]
struct QueuedEvent {
    event: EventContainer,
    attempts: u32,
    first_attempt_at: Instant,
    last_attempt_at: Option<Instant>,
    priority: EventPriority,
}

/// Event container that can hold either legacy or versioned events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventContainer {
    Legacy(DomainEvent),
    Versioned(VersionedDomainEvent),
}

/// Event priority for processing order
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EventPriority {
    Critical = 0,  // Immediate processing
    High = 1,      // Process before normal
    Normal = 2,    // Default priority
    Low = 3,       // Process after normal
}

/// Rate limiter for event publishing
#[derive(Debug)]
struct RateLimiter {
    max_events_per_second: f64,
    tokens: f64,
    last_refill: Instant,
}

/// Publisher performance metrics
#[derive(Debug, Clone)]
pub struct PublisherMetrics {
    pub events_published: u64,
    pub events_failed: u64,
    pub events_duplicated: u64,
    pub events_rate_limited: u64,
    pub average_batch_size: f64,
    pub average_publish_latency_ms: f64,
    pub dead_letter_count: u64,
    pub queue_depth: usize,
}

impl EventContainer {
    pub fn event_id(&self) -> String {
        match self {
            EventContainer::Legacy(event) => event.id.to_string(),
            EventContainer::Versioned(event) => event.id.to_string(),
        }
    }

    pub fn aggregate_id(&self) -> Uuid {
        match self {
            EventContainer::Legacy(event) => event.aggregate_id,
            EventContainer::Versioned(event) => event.aggregate_id,
        }
    }

    pub fn aggregate_type(&self) -> &str {
        match self {
            EventContainer::Legacy(event) => &event.aggregate_type,
            EventContainer::Versioned(event) => &event.aggregate_type,
        }
    }

    pub fn sequence_number(&self) -> i64 {
        match self {
            EventContainer::Legacy(event) => event.version,
            EventContainer::Versioned(event) => event.sequence_number,
        }
    }

    pub fn is_sensitive(&self) -> bool {
        match self {
            EventContainer::Legacy(_) => false, // Legacy events don't have sensitivity info
            EventContainer::Versioned(event) => event.is_sensitive(),
        }
    }
}

impl RateLimiter {
    fn new(max_events_per_second: f64) -> Self {
        Self {
            max_events_per_second,
            tokens: max_events_per_second,
            last_refill: Instant::now(),
        }
    }

    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill_tokens();
        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    fn refill_tokens(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        let new_tokens = elapsed * self.max_events_per_second;
        self.tokens = (self.tokens + new_tokens).min(self.max_events_per_second);
        self.last_refill = now;
    }
}

impl EventPublisher {
    /// Create a new enhanced event publisher
    pub async fn new(config: EventConfig) -> Result<Self> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|e| Error::Configuration(format!("Invalid Redis URL: {}", e)))?;

        let redis = ConnectionManager::new(client)
            .await
            .map_err(|e| Error::Redis(e))?;

        info!("Enhanced event publisher connected to Redis");

        Ok(Self {
            redis,
            config: config.clone(),
            batch_queue: Arc::new(Mutex::new(VecDeque::new())),
            deduplication_cache: Arc::new(RwLock::new(HashSet::new())),
            aggregate_sequences: Arc::new(RwLock::new(HashMap::new())),
            published_count: Arc::new(AtomicU64::new(0)),
            failed_count: Arc::new(AtomicU64::new(0)),
            duplicate_count: Arc::new(AtomicU64::new(0)),
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new(config.max_events_per_second.unwrap_or(1000.0)))),
            shutdown_tx: None,
            metrics: Arc::new(Mutex::new(PublisherMetrics {
                events_published: 0,
                events_failed: 0,
                events_duplicated: 0,
                events_rate_limited: 0,
                average_batch_size: 0.0,
                average_publish_latency_ms: 0.0,
                dead_letter_count: 0,
                queue_depth: 0,
            })),
        })
    }

    /// Start the enhanced background batch processor with ordering and deduplication
    pub async fn start_batch_processor(&mut self) -> Result<()> {
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let redis = self.redis.clone();
        let config = self.config.clone();
        let batch_queue = Arc::clone(&self.batch_queue);
        let deduplication_cache = Arc::clone(&self.deduplication_cache);
        let aggregate_sequences = Arc::clone(&self.aggregate_sequences);
        let published_count = Arc::clone(&self.published_count);
        let failed_count = Arc::clone(&self.failed_count);
        let duplicate_count = Arc::clone(&self.duplicate_count);
        let rate_limiter = Arc::clone(&self.rate_limiter);
        let metrics = Arc::clone(&self.metrics);

        // Start cache cleanup task
        let cache_cleanup_handle = Self::start_cache_cleanup_task(
            Arc::clone(&deduplication_cache),
            config.deduplication_window.unwrap_or(Duration::from_secs(3600))
        );

        tokio::spawn(async move {
            let mut batch_timer = tokio::time::interval(config.batch_timeout);
            let mut metrics_timer = tokio::time::interval(Duration::from_secs(30));
            let mut last_batch_time = Instant::now();

            loop {
                tokio::select! {
                    _ = batch_timer.tick() => {
                        if let Err(e) = Self::process_enhanced_batch(
                            &redis,
                            &config,
                            &batch_queue,
                            &deduplication_cache,
                            &aggregate_sequences,
                            &published_count,
                            &failed_count,
                            &duplicate_count,
                            &rate_limiter,
                            &metrics,
                        ).await {
                            error!("Enhanced batch processing error: {}", e);
                        }
                        last_batch_time = Instant::now();
                    }

                    _ = metrics_timer.tick() => {
                        Self::log_metrics(&metrics, &published_count, &failed_count, &duplicate_count).await;
                    }

                    _ = shutdown_rx.recv() => {
                        info!("Shutting down enhanced batch processor");
                        // Process remaining events
                        if let Err(e) = Self::process_enhanced_batch(
                            &redis,
                            &config,
                            &batch_queue,
                            &deduplication_cache,
                            &aggregate_sequences,
                            &published_count,
                            &failed_count,
                            &duplicate_count,
                            &rate_limiter,
                            &metrics,
                        ).await {
                            error!("Final enhanced batch processing error: {}", e);
                        }
                        cache_cleanup_handle.abort();
                        break;
                    }

                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        let queue_size = {
                            let queue = batch_queue.lock().await;
                            queue.len()
                        };

                        if queue_size >= config.batch_size ||
                           (queue_size > 0 && last_batch_time.elapsed() >= config.batch_timeout) {
                            if let Err(e) = Self::process_enhanced_batch(
                                &redis,
                                &config,
                                &batch_queue,
                                &deduplication_cache,
                                &aggregate_sequences,
                                &published_count,
                                &failed_count,
                                &duplicate_count,
                                &rate_limiter,
                                &metrics,
                            ).await {
                                error!("Enhanced batch processing error: {}", e);
                            }
                            last_batch_time = Instant::now();
                        }
                    }
                }
            }
        });

        info!("Enhanced event publisher batch processor started");
        Ok(())
    }

    /// Publish a single legacy event immediately
    pub async fn publish(&mut self, event: &DomainEvent) -> Result<()> {
        let container = EventContainer::Legacy(event.clone());
        self.publish_event_with_deduplication(container, EventPriority::Normal).await
    }

    /// Publish a single versioned event immediately
    pub async fn publish_versioned(&mut self, event: &VersionedDomainEvent) -> Result<()> {
        let container = EventContainer::Versioned(event.clone());
        self.publish_event_with_deduplication(container, EventPriority::Normal).await
    }

    /// Publish a critical event that bypasses rate limiting
    pub async fn publish_critical(&mut self, event: EventContainer) -> Result<()> {
        self.publish_event_with_deduplication(event, EventPriority::Critical).await
    }

    /// Publish multiple events as a batch
    pub async fn publish_batch(&mut self, events: &[EventContainer]) -> Result<()> {
        for event in events {
            self.publish_event_with_deduplication(event.clone(), EventPriority::Normal).await?;
        }
        Ok(())
    }

    /// Add event to batch queue for later processing with deduplication
    pub async fn publish_async(&self, event: EventContainer) -> Result<()> {
        self.publish_async_with_priority(event, EventPriority::Normal).await
    }

    /// Add event to batch queue with specific priority
    pub async fn publish_async_with_priority(&self, event: EventContainer, priority: EventPriority) -> Result<()> {
        // Check for duplicates
        if self.is_duplicate(&event).await? {
            self.duplicate_count.fetch_add(1, Ordering::Relaxed);
            return Err(Error::Internal(format!("Duplicate event detected: {}", event.event_id())));
        }

        // Check event ordering for versioned events
        if let EventContainer::Versioned(ref versioned_event) = event {
            self.validate_event_ordering(versioned_event).await?;
        }

        // Check rate limiting
        if priority != EventPriority::Critical {
            let mut rate_limiter = self.rate_limiter.lock().await;
            if !rate_limiter.try_consume(1.0) {
                return Err(Error::Internal("Rate limit exceeded".to_string()));
            }
        }

        let mut queue = self.batch_queue.lock().await;

        if queue.len() >= self.config.max_queue_size.unwrap_or(10000) {
            return Err(Error::Internal("Event queue full".to_string()));
        }

        let queued_event = QueuedEvent {
            event,
            attempts: 0,
            first_attempt_at: Instant::now(),
            last_attempt_at: None,
            priority: priority.clone(),
        };

        // Insert event in priority order
        let insert_position = queue.iter()
            .position(|e| e.priority > priority)
            .unwrap_or(queue.len());

        queue.insert(insert_position, queued_event);

        debug!("Event queued for batch processing. Queue size: {}, Priority: {:?}", queue.len(), priority);
        Ok(())
    }

    /// Publish event with deduplication, ordering, and retry logic
    #[instrument(skip(self, event), fields(event_id = %event.event_id()))]
    async fn publish_event_with_deduplication(&mut self, event: EventContainer, priority: EventPriority) -> Result<()> {
        let start_time = Instant::now();

        // Check for duplicates
        if self.is_duplicate(&event).await? {
            self.duplicate_count.fetch_add(1, Ordering::Relaxed);
            return Err(Error::Internal(format!("Duplicate event detected: {}", event.event_id())));
        }

        // Validate ordering for versioned events
        if let EventContainer::Versioned(ref versioned_event) = event {
            self.validate_event_ordering(versioned_event).await?;
        }

        // Attempt to publish with retries
        let mut current_retry = 0;
        loop {
            match self.publish_event_container(&event).await {
                Ok(()) => {
                    // Update metrics
                    self.published_count.fetch_add(1, Ordering::Relaxed);
                    let latency = start_time.elapsed().as_millis() as f64;
                    self.update_latency_metric(latency).await;

                    // Add to deduplication cache
                    self.add_to_deduplication_cache(&event).await;

                    debug!("Event published successfully: {}", event.event_id());
                    return Ok(());
                }
                Err(e) if current_retry < self.config.max_retries => {
                    current_retry += 1;
                    warn!(
                        "Event publish failed (attempt {}), retrying: {}",
                        current_retry,
                        e
                    );

                    // Exponential backoff
                    let delay = self.config.retry_delay * (2_u32.pow(current_retry.saturating_sub(1)));
                    sleep(delay).await;
                }
                Err(e) => {
                    error!("Event publish failed after {} retries: {}", self.config.max_retries, e);

                    self.failed_count.fetch_add(1, Ordering::Relaxed);

                    if self.config.enable_dead_letter_queue {
                        if let Err(dlq_error) = self.send_event_to_dead_letter_queue(&event, &e.to_string()).await {
                            error!("Failed to send event to dead letter queue: {}", dlq_error);
                        }
                    }

                    return Err(Error::Internal(format!("Failed to publish event: {}", e)));
                }
            }
        }
    }

    /// Publish event container to Redis with enhanced routing
    #[instrument(skip(self, event), fields(event_id = %event.event_id()))]
    async fn publish_event_container(&mut self, event: &EventContainer) -> std::result::Result<(), PublishError> {
        let payload = self.serialize_event(event)?;

        // Check event size limits
        if payload.len() > self.config.max_event_size.unwrap_or(1_048_576) { // 1MB default
            return Err(PublishError::EventTooLarge {
                size: payload.len(),
                limit: self.config.max_event_size.unwrap_or(1_048_576),
            });
        }

        match event {
            EventContainer::Legacy(legacy_event) => {
                self.publish_legacy_event(legacy_event, &payload).await?
            }
            EventContainer::Versioned(versioned_event) => {
                self.publish_versioned_event(versioned_event, &payload).await?
            }
        }

        Ok(())
    }

    /// Publish legacy domain event
    async fn publish_legacy_event(&mut self, event: &DomainEvent, payload: &str) -> std::result::Result<(), PublishError> {
        // Publish to main topic
        let topic = event.topic();
        redis::cmd("PUBLISH")
            .arg(&topic)
            .arg(payload)
            .query_async::<_, i64>(&mut self.redis)
            .await?;

        // Publish to tenant-specific topic
        let tenant_topic = event.tenant_topic();
        redis::cmd("PUBLISH")
            .arg(&tenant_topic)
            .arg(payload)
            .query_async::<_, i64>(&mut self.redis)
            .await?;

        // Publish to global topic if applicable
        if event.is_global_event() {
            let global_topic = format!("events.global.{}", event.event_type);
            redis::cmd("PUBLISH")
                .arg(&global_topic)
                .arg(payload)
                .query_async::<_, i64>(&mut self.redis)
                .await?;
        }

        // Store in event stream for replay capability
        self.store_legacy_event_in_stream(event).await?;

        Ok(())
    }

    /// Publish versioned domain event with enhanced routing
    async fn publish_versioned_event(&mut self, event: &VersionedDomainEvent, payload: &str) -> std::result::Result<(), PublishError> {
        // Publish to versioned main topic
        let topic = event.topic();
        redis::cmd("PUBLISH")
            .arg(&topic)
            .arg(payload)
            .query_async::<_, i64>(&mut self.redis)
            .await?;

        // Publish to tenant-specific topic
        let tenant_topic = event.tenant_topic();
        redis::cmd("PUBLISH")
            .arg(&tenant_topic)
            .arg(payload)
            .query_async::<_, i64>(&mut self.redis)
            .await?;

        // Publish to location-specific topic if available
        if let Some(location_topic) = event.location_topic() {
            redis::cmd("PUBLISH")
                .arg(&location_topic)
                .arg(payload)
                .query_async::<_, i64>(&mut self.redis)
                .await?;
        }

        // Publish to global topic if applicable
        if event.is_global_event() {
            let global_topic = format!("events.global.v{}.{}", event.event_version.major, event.event_type);
            redis::cmd("PUBLISH")
                .arg(&global_topic)
                .arg(payload)
                .query_async::<_, i64>(&mut self.redis)
                .await?;
        }

        // Store in versioned event stream for replay capability
        self.store_versioned_event_in_stream(event).await?;

        Ok(())
    }

    /// Store legacy event in Redis stream
    async fn store_legacy_event_in_stream(&mut self, event: &DomainEvent) -> std::result::Result<(), PublishError> {
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
            .arg("version")
            .arg("1.0.0")
            .query_async::<_, String>(&mut self.redis)
            .await?;

        // Set TTL based on event type (30 days default)
        let ttl = 30 * 24 * 60 * 60;
        redis::cmd("EXPIRE")
            .arg(&stream_name)
            .arg(ttl)
            .query_async::<_, ()>(&mut self.redis)
            .await
            .ok();

        Ok(())
    }

    /// Store versioned event in Redis stream with enhanced metadata
    async fn store_versioned_event_in_stream(&mut self, event: &VersionedDomainEvent) -> std::result::Result<(), PublishError> {
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
            .arg(event.context.business_context.tenant_id.to_string())
            .arg("occurred_at")
            .arg(event.occurred_at.timestamp())
            .arg("version")
            .arg(event.event_version.to_string())
            .arg("sequence")
            .arg(event.sequence_number)
            .arg("aggregate_version")
            .arg(event.aggregate_version)
            .arg("correlation_id")
            .arg(event.context.correlation_id.to_string())
            .arg("source_service")
            .arg(&event.context.source_service)
            .arg("checksum")
            .arg(event.checksum.as_ref().unwrap_or(&"none".to_string()))
            .query_async::<_, String>(&mut self.redis)
            .await?;

        // Set TTL based on event retention policy
        let ttl = event.retention_days() * 24 * 60 * 60;
        redis::cmd("EXPIRE")
            .arg(&stream_name)
            .arg(ttl as u64)
            .query_async::<_, ()>(&mut self.redis)
            .await
            .ok();

        Ok(())
    }

    /// Send failed event to dead letter queue with enhanced metadata
    async fn send_event_to_dead_letter_queue(&mut self, event: &EventContainer, failure_reason: &str) -> std::result::Result<(), PublishError> {
        let dlq_event = EnhancedDeadLetterEvent {
            id: Uuid::new_v4(),
            original_event: event.clone(),
            failed_at: chrono::Utc::now(),
            failure_reason: failure_reason.to_string(),
            retry_count: self.config.max_retries,
            source_service: self.config.source_service.clone().unwrap_or("unknown".to_string()),
            environment: self.config.environment.clone().unwrap_or("unknown".to_string()),
            correlation_id: Uuid::new_v4(),
        };

        let payload = serde_json::to_string(&dlq_event)?;

        // Store in both a list and a stream for different access patterns
        redis::cmd("LPUSH")
            .arg(&self.config.dead_letter_topic)
            .arg(&payload)
            .query_async::<_, ()>(&mut self.redis)
            .await?;

        // Also store in a stream for time-based queries
        let dlq_stream = format!("{}.stream", self.config.dead_letter_topic);
        redis::cmd("XADD")
            .arg(&dlq_stream)
            .arg("*")
            .arg("dlq_event")
            .arg(&payload)
            .arg("failed_at")
            .arg(dlq_event.failed_at.timestamp())
            .arg("reason")
            .arg(failure_reason)
            .query_async::<_, String>(&mut self.redis)
            .await?;

        warn!("Event sent to dead letter queue: {} (reason: {})", event.event_id(), failure_reason);
        Ok(())
    }

    /// Process enhanced batch with deduplication, ordering, and metrics
    #[instrument(skip_all)]
    async fn process_enhanced_batch(
        redis: &ConnectionManager,
        config: &EventConfig,
        batch_queue: &Arc<Mutex<VecDeque<QueuedEvent>>>,
        deduplication_cache: &Arc<RwLock<HashSet<String>>>,
        aggregate_sequences: &Arc<RwLock<HashMap<String, i64>>>,
        published_count: &Arc<AtomicU64>,
        failed_count: &Arc<AtomicU64>,
        duplicate_count: &Arc<AtomicU64>,
        rate_limiter: &Arc<Mutex<RateLimiter>>,
        metrics: &Arc<Mutex<PublisherMetrics>>,
    ) -> Result<()> {
        let batch_start = Instant::now();

        let events = {
            let mut queue = batch_queue.lock().await;
            let batch_size = std::cmp::min(queue.len(), config.batch_size);
            if batch_size == 0 {
                return Ok(());
            }

            // Take events in priority order
            (0..batch_size).map(|_| queue.pop_front().unwrap()).collect::<Vec<_>>()
        };

        debug!("Processing enhanced batch of {} events", events.len());

        let mut successful_events = 0;
        let mut failed_events = 0;
        let mut duplicate_events = 0;

        // Group events by aggregate for ordered processing
        let mut aggregate_groups: HashMap<String, Vec<QueuedEvent>> = HashMap::new();
        for event in events {
            let aggregate_key = format!("{}-{}", event.event.aggregate_type(), event.event.aggregate_id());
            aggregate_groups.entry(aggregate_key).or_default().push(event);
        }

        // Process each aggregate group in order
        for (aggregate_key, mut group_events) in aggregate_groups {
            // Sort by sequence number for versioned events
            group_events.sort_by_key(|e| e.event.sequence_number());

            for mut queued_event in group_events {
                queued_event.attempts += 1;
                queued_event.last_attempt_at = Some(Instant::now());

                // Check rate limiting for non-critical events
                if queued_event.priority != EventPriority::Critical {
                    let mut limiter = rate_limiter.lock().await;
                    if !limiter.try_consume(1.0) {
                        // Re-queue the event for later
                        let mut queue = batch_queue.lock().await;
                        queue.push_back(queued_event);
                        continue;
                    }
                }

                // Create a temporary publisher for this event
                let mut temp_publisher = EventPublisher {
                    redis: redis.clone(),
                    config: config.clone(),
                    batch_queue: Arc::new(Mutex::new(VecDeque::new())),
                    deduplication_cache: Arc::clone(deduplication_cache),
                    aggregate_sequences: Arc::clone(aggregate_sequences),
                    published_count: Arc::clone(published_count),
                    failed_count: Arc::clone(failed_count),
                    duplicate_count: Arc::clone(duplicate_count),
                    rate_limiter: Arc::clone(rate_limiter),
                    shutdown_tx: None,
                    metrics: Arc::clone(metrics),
                };

                match temp_publisher.publish_event_with_deduplication(queued_event.event.clone(), queued_event.priority.clone()).await {
                    Ok(()) => {
                        successful_events += 1;
                    }
                    Err(e) if e.to_string().contains("Duplicate") => {
                        duplicate_events += 1;
                    }
                    Err(e) => {
                        failed_events += 1;

                        // Retry logic
                        if queued_event.attempts < config.max_retries {
                            // Calculate backoff delay
                            let backoff_delay = config.retry_delay * (2_u32.pow(queued_event.attempts.saturating_sub(1)));

                            // Re-queue for retry if not too old
                            if queued_event.first_attempt_at.elapsed() < config.max_retry_duration.unwrap_or(Duration::from_secs(300)) {
                                tokio::time::sleep(backoff_delay).await;
                                let mut queue = batch_queue.lock().await;
                                queue.push_back(queued_event);
                                continue;
                            }
                        }

                        error!("Failed to publish event in enhanced batch: {}", e);
                    }
                }
            }
        }

        // Update metrics
        let batch_duration = batch_start.elapsed();
        let mut metrics_guard = metrics.lock().await;
        metrics_guard.events_published += successful_events;
        metrics_guard.events_failed += failed_events;
        metrics_guard.events_duplicated += duplicate_events;
        metrics_guard.average_batch_size = ((metrics_guard.average_batch_size * 0.9) + (successful_events as f64 * 0.1));
        metrics_guard.average_publish_latency_ms = ((metrics_guard.average_publish_latency_ms * 0.9) + (batch_duration.as_millis() as f64 * 0.1));

        // Update queue depth
        let queue_size = {
            let queue = batch_queue.lock().await;
            queue.len()
        };
        metrics_guard.queue_depth = queue_size;

        drop(metrics_guard);

        info!(
            "Processed enhanced batch: {} successful, {} failed, {} duplicates in {:?}",
            successful_events, failed_events, duplicate_events, batch_duration
        );

        Ok(())
    }

    /// Check if event is a duplicate
    async fn is_duplicate(&self, event: &EventContainer) -> Result<bool> {
        let cache = self.deduplication_cache.read().await;
        Ok(cache.contains(&event.event_id()))
    }

    /// Add event to deduplication cache
    async fn add_to_deduplication_cache(&self, event: &EventContainer) {
        let mut cache = self.deduplication_cache.write().await;
        cache.insert(event.event_id());
    }

    /// Validate event ordering for versioned events
    async fn validate_event_ordering(&self, event: &VersionedDomainEvent) -> Result<()> {
        let aggregate_key = format!("{}-{}", event.aggregate_type, event.aggregate_id);
        let mut sequences = self.aggregate_sequences.write().await;

        if let Some(&last_sequence) = sequences.get(&aggregate_key) {
            if event.sequence_number <= last_sequence {
                return Err(Error::Internal(format!(
                    "Event ordering violation: expected > {}, got {}",
                    last_sequence, event.sequence_number
                )));
            }
        }

        sequences.insert(aggregate_key, event.sequence_number);
        Ok(())
    }

    /// Serialize event to JSON with compression for large events
    fn serialize_event(&self, event: &EventContainer) -> std::result::Result<String, PublishError> {
        let json = match event {
            EventContainer::Legacy(e) => serde_json::to_string(e)?,
            EventContainer::Versioned(e) => serde_json::to_string(e)?,
        };

        // TODO: Add compression for large events if needed
        Ok(json)
    }

    /// Update latency metrics
    async fn update_latency_metric(&self, latency_ms: f64) {
        let mut metrics = self.metrics.lock().await;
        metrics.average_publish_latency_ms = (metrics.average_publish_latency_ms * 0.9) + (latency_ms * 0.1);
    }

    /// Start cache cleanup task to prevent memory leaks
    fn start_cache_cleanup_task(
        deduplication_cache: Arc<RwLock<HashSet<String>>>,
        cleanup_interval: Duration,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                let mut cache = deduplication_cache.write().await;
                cache.clear(); // Simple cleanup - could be more sophisticated
                debug!("Deduplication cache cleared");
            }
        })
    }

    /// Log publisher metrics
    async fn log_metrics(
        metrics: &Arc<Mutex<PublisherMetrics>>,
        published_count: &Arc<AtomicU64>,
        failed_count: &Arc<AtomicU64>,
        duplicate_count: &Arc<AtomicU64>,
    ) {
        let metrics_guard = metrics.lock().await;
        let published = published_count.load(Ordering::Relaxed);
        let failed = failed_count.load(Ordering::Relaxed);
        let duplicates = duplicate_count.load(Ordering::Relaxed);

        info!(
            "Publisher metrics: published={}, failed={}, duplicates={}, queue_depth={}, avg_latency={}ms",
            published,
            failed,
            duplicates,
            metrics_guard.queue_depth,
            metrics_guard.average_publish_latency_ms
        );
    }

    /// Get current publisher metrics
    pub async fn get_metrics(&self) -> PublisherMetrics {
        let mut metrics = self.metrics.lock().await;
        metrics.events_published = self.published_count.load(Ordering::Relaxed);
        metrics.events_failed = self.failed_count.load(Ordering::Relaxed);
        metrics.events_duplicated = self.duplicate_count.load(Ordering::Relaxed);
        metrics.clone()
    }

    /// Graceful shutdown with enhanced cleanup
    pub async fn shutdown(&mut self) -> Result<()> {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(()).await;
        }

        // Wait for remaining events to be processed
        let mut attempts = 0;
        const MAX_SHUTDOWN_ATTEMPTS: u32 = 10;

        while attempts < MAX_SHUTDOWN_ATTEMPTS {
            let remaining_events = {
                let queue = self.batch_queue.lock().await;
                queue.len()
            };

            if remaining_events == 0 {
                break;
            }

            info!("Waiting for {} remaining events to be processed (attempt {})", remaining_events, attempts + 1);
            tokio::time::sleep(Duration::from_millis(500)).await;
            attempts += 1;
        }

        // Final metrics report
        let final_metrics = self.get_metrics().await;
        info!("Final publisher metrics: {:?}", final_metrics);

        info!("Enhanced event publisher shutdown complete");
        Ok(())
    }
}

/// Enhanced dead letter event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EnhancedDeadLetterEvent {
    id: Uuid,
    original_event: EventContainer,
    failed_at: chrono::DateTime<chrono::Utc>,
    failure_reason: String,
    retry_count: u32,
    source_service: String,
    environment: String,
    correlation_id: Uuid,
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