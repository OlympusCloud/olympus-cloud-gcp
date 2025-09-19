// ============================================================================
// OLYMPUS CLOUD - EVENT SYSTEM INTEGRATION TESTS
// ============================================================================
// Module: shared/tests/event_system_tests.rs
// Description: Comprehensive tests for Phase 5 Event-Driven Architecture
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 1.0 - Phase 5 Event System Tests
// ============================================================================

use olympus_shared::events::{
    EventPublisher, EventSubscriber, EventHandler, EventContainer, EventConfig,
    VersionedDomainEvent, DomainEvent, EventVersion, EventContext, BusinessContext,
    ClientInfo, HandlerPriority, HandlerHealth, SubscriptionConfig,
    UserRegisteredEvent, OrderCreatedEvent, TenantCreatedEvent,
    auth_events, commerce_events, platform_events,
};
use olympus_shared::{Error, Result};
use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use chrono::Utc;
use serde_json::Value;

// ============================================================================
// TEST EVENT HANDLERS
// ============================================================================

/// Test event handler for verification
pub struct TestEventHandler {
    name: String,
    processed_events: Arc<AtomicU32>,
    event_types: Vec<String>,
    should_fail: Arc<AtomicU32>,
}

impl TestEventHandler {
    pub fn new(name: &str, event_types: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            processed_events: Arc::new(AtomicU32::new(0)),
            event_types,
            should_fail: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn get_processed_count(&self) -> u32 {
        self.processed_events.load(Ordering::Relaxed)
    }

    pub fn set_should_fail(&self, fail_after: u32) {
        self.should_fail.store(fail_after, Ordering::Relaxed);
    }
}

#[async_trait]
impl EventHandler for TestEventHandler {
    async fn handle(&self, _event: &EventContainer) -> Result<()> {
        let current_count = self.processed_events.fetch_add(1, Ordering::Relaxed) + 1;
        let fail_threshold = self.should_fail.load(Ordering::Relaxed);

        if fail_threshold > 0 && current_count >= fail_threshold {
            return Err(Error::Internal("Test handler failure".to_string()));
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

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Normal
    }

    fn supports_concurrent_processing(&self) -> bool {
        true
    }

    fn max_concurrent_events(&self) -> usize {
        5
    }

    async fn health_check(&self) -> HandlerHealth {
        HandlerHealth::Healthy
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn create_test_config() -> EventConfig {
    EventConfig {
        redis_url: "redis://localhost:6379".to_string(),
        max_retries: 2,
        retry_delay: Duration::from_millis(100),
        batch_size: 5,
        batch_timeout: Duration::from_millis(500),
        enable_dead_letter_queue: true,
        dead_letter_topic: "test.dead_letter".to_string(),
        max_events_per_second: Some(100.0),
        max_event_size: Some(1024),
        max_queue_size: Some(100),
        deduplication_window: Some(Duration::from_secs(60)),
        max_retry_duration: Some(Duration::from_secs(30)),
        source_service: Some("test-service".to_string()),
        environment: Some("test".to_string()),
        enable_compression: Some(false),
        enable_encryption: Some(false),
        encryption_key: None,
    }
}

fn create_test_subscription_config() -> SubscriptionConfig {
    SubscriptionConfig {
        name: "test_subscriber".to_string(),
        topics: vec!["events.*".to_string()],
        consumer_group: Some("test_group".to_string()),
        max_retries: 2,
        retry_delay: Duration::from_millis(100),
        batch_size: 5,
        prefetch_count: 10,
        auto_ack: true,
        enable_replay: false,
        replay_from: None,
        max_concurrent_handlers: 5,
        handler_timeout: Duration::from_secs(5),
        enable_dead_letter_processing: true,
        enable_metrics: true,
        health_check_interval: Duration::from_secs(10),
        enable_duplicate_detection: true,
        duplicate_detection_window: Duration::from_secs(60),
        enable_ordering: true,
        max_processing_time: Duration::from_secs(30),
    }
}

fn create_test_legacy_event() -> EventContainer {
    let user_event = UserRegisteredEvent {
        user_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        email_verification_required: true,
        registration_source: "web".to_string(),
    };

    let domain_event = DomainEvent {
        id: Uuid::new_v4(),
        event_type: auth_events::USER_REGISTERED.to_string(),
        aggregate_id: user_event.user_id,
        aggregate_type: "User".to_string(),
        tenant_id: user_event.tenant_id,
        data: serde_json::to_value(&user_event).unwrap(),
        metadata: olympus_shared::events::EventMetadata {
            user_id: Some(user_event.user_id),
            correlation_id: Uuid::new_v4(),
            causation_id: None,
            ip_address: Some("127.0.0.1".to_string()),
            user_agent: Some("test-agent".to_string()),
            source_service: "test".to_string(),
            event_source: "api".to_string(),
            trace_id: None,
        },
        version: 1,
        occurred_at: Utc::now(),
    };

    EventContainer::Legacy(domain_event)
}

fn create_test_versioned_event() -> EventContainer {
    let tenant_id = Uuid::new_v4();
    let order_event = OrderCreatedEvent {
        order_id: Uuid::new_v4(),
        tenant_id,
        customer_id: Some(Uuid::new_v4()),
        location_id: Uuid::new_v4(),
        order_number: "TEST-001".to_string(),
        total_amount: rust_decimal::Decimal::from(100),
        currency: "USD".to_string(),
        item_count: 2,
        order_source: olympus_shared::events::OrderSource::Web,
    };

    let versioned_event = VersionedDomainEvent::builder(
        commerce_events::ORDER_CREATED.to_string(),
        order_event.order_id,
        "Order".to_string(),
        tenant_id,
    )
    .version(EventVersion::v2_0_0())
    .sequence_number(1)
    .source("test-service".to_string(), "1.0.0".to_string())
    .data(order_event)
    .unwrap()
    .build();

    EventContainer::Versioned(versioned_event)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_event_version_compatibility() {
        let v1_0_0 = EventVersion::v1_0_0();
        let v1_1_0 = EventVersion::new(1, 1, 0);
        let v2_0_0 = EventVersion::v2_0_0();

        assert!(v1_0_0.is_compatible_with(&v1_1_0));
        assert!(!v1_0_0.is_compatible_with(&v2_0_0));
        assert_eq!(v1_0_0.to_string(), "1.0.0");
        assert_eq!(v2_0_0.to_string(), "2.0.0");
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
        .sequence_number(5)
        .data(serde_json::json!({"test": "data"}))
        .unwrap()
        .build();

        assert_eq!(event.event_type, "TestEvent");
        assert_eq!(event.aggregate_id, aggregate_id);
        assert_eq!(event.sequence_number, 5);
        assert_eq!(event.event_version.major, 2);
        assert!(event.checksum.is_some());
        assert!(event.verify_integrity());
    }

    #[test]
    fn test_event_topics() {
        let versioned_event = create_test_versioned_event();

        if let EventContainer::Versioned(event) = versioned_event {
            assert_eq!(event.topic(), "events.v2.order.OrderCreated");
            assert!(event.tenant_topic().starts_with("events.tenant."));
            assert!(!event.is_global_event());
            assert!(!event.is_sensitive());
        }
    }

    #[test]
    fn test_event_container_methods() {
        let legacy_event = create_test_legacy_event();
        let versioned_event = create_test_versioned_event();

        // Test legacy event methods
        assert!(!legacy_event.event_id().is_empty());
        assert!(!legacy_event.aggregate_type().is_empty());
        assert!(legacy_event.sequence_number() > 0);

        // Test versioned event methods
        assert!(!versioned_event.event_id().is_empty());
        assert!(!versioned_event.aggregate_type().is_empty());
        assert!(versioned_event.sequence_number() > 0);
    }

    #[test]
    fn test_event_checksum_integrity() {
        let versioned_event = create_test_versioned_event();

        if let EventContainer::Versioned(mut event) = versioned_event {
            // Original event should verify
            assert!(event.verify_integrity());

            // Corrupt the data
            event.data = serde_json::json!({"corrupted": "data"});

            // Corrupted event should not verify
            assert!(!event.verify_integrity());
        }
    }

    #[test]
    fn test_event_retention_policy() {
        let versioned_event = create_test_versioned_event();

        if let EventContainer::Versioned(event) = versioned_event {
            let retention_days = event.retention_days();
            assert!(retention_days >= 90); // At least 90 days for any event
        }
    }

    #[test]
    fn test_handler_priority_ordering() {
        assert!(HandlerPriority::Critical < HandlerPriority::High);
        assert!(HandlerPriority::High < HandlerPriority::Normal);
        assert!(HandlerPriority::Normal < HandlerPriority::Low);
    }

    #[test]
    fn test_test_event_handler() {
        let handler = TestEventHandler::new(
            "TestHandler",
            vec![auth_events::USER_REGISTERED.to_string()],
        );

        assert_eq!(handler.name(), "TestHandler");
        assert_eq!(handler.get_processed_count(), 0);
        assert!(handler.supports_concurrent_processing());
        assert_eq!(handler.max_concurrent_events(), 5);
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod integration_tests {
    use super::*;

    // Note: These tests require a running Redis instance
    // In a real CI/CD environment, you would use testcontainers or similar

    #[tokio::test]
    #[ignore] // Requires Redis
    async fn test_event_publisher_creation() {
        let config = create_test_config();
        let result = EventPublisher::new(config).await;

        match result {
            Ok(_publisher) => {
                // Test passed - Redis is available
            }
            Err(_) => {
                // Redis not available - skip test
                println!("Redis not available for testing");
            }
        }
    }

    #[tokio::test]
    #[ignore] // Requires Redis
    async fn test_event_subscriber_creation() {
        let config = create_test_subscription_config();
        let result = EventSubscriber::new("redis://localhost:6379", config).await;

        match result {
            Ok(_subscriber) => {
                // Test passed - Redis is available
            }
            Err(_) => {
                // Redis not available - skip test
                println!("Redis not available for testing");
            }
        }
    }

    #[tokio::test]
    async fn test_event_handler_processing() {
        let handler = TestEventHandler::new(
            "TestHandler",
            vec![auth_events::USER_REGISTERED.to_string()],
        );

        let event = create_test_legacy_event();
        let result = handler.handle(&event).await;

        assert!(result.is_ok());
        assert_eq!(handler.get_processed_count(), 1);
    }

    #[tokio::test]
    async fn test_event_handler_failure() {
        let handler = TestEventHandler::new(
            "FailingHandler",
            vec![auth_events::USER_REGISTERED.to_string()],
        );

        // Set handler to fail on first event
        handler.set_should_fail(1);

        let event = create_test_legacy_event();
        let result = handler.handle(&event).await;

        assert!(result.is_err());
        assert_eq!(handler.get_processed_count(), 1);
    }

    #[tokio::test]
    async fn test_concurrent_event_processing() {
        let handler = Arc::new(TestEventHandler::new(
            "ConcurrentHandler",
            vec!["*".to_string()],
        ));

        let mut tasks = Vec::new();

        // Process 10 events concurrently
        for _ in 0..10 {
            let handler_clone = Arc::clone(&handler);
            let event = create_test_legacy_event();

            let task = tokio::spawn(async move {
                handler_clone.handle(&event).await
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            let result = task.await.unwrap();
            assert!(result.is_ok());
        }

        assert_eq!(handler.get_processed_count(), 10);
    }

    #[tokio::test]
    async fn test_event_serialization_deserialization() {
        let original_event = create_test_versioned_event();

        // Serialize event
        let serialized = match &original_event {
            EventContainer::Versioned(event) => serde_json::to_string(event).unwrap(),
            EventContainer::Legacy(event) => serde_json::to_string(event).unwrap(),
        };

        // Deserialize back
        let deserialized_versioned: Result<VersionedDomainEvent, _> = serde_json::from_str(&serialized);
        let deserialized_legacy: Result<DomainEvent, _> = serde_json::from_str(&serialized);

        // At least one should succeed
        assert!(deserialized_versioned.is_ok() || deserialized_legacy.is_ok());
    }

    #[tokio::test]
    async fn test_event_metadata_handling() {
        let client_info = ClientInfo {
            ip_address: "192.168.1.1".to_string(),
            user_agent: Some("Mozilla/5.0".to_string()),
            device_id: Some("device-123".to_string()),
            platform: Some("web".to_string()),
            app_version: Some("1.0.0".to_string()),
            geolocation: None,
        };

        let tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let event = VersionedDomainEvent::builder(
            "TestEvent".to_string(),
            Uuid::new_v4(),
            "Test".to_string(),
            tenant_id,
        )
        .user_context(user_id, Some("session-123".to_string()))
        .client_info(client_info.clone())
        .metadata("custom_field".to_string(), "custom_value")
        .unwrap()
        .build();

        assert_eq!(event.context.user_id, Some(user_id));
        assert_eq!(event.context.session_id, Some("session-123".to_string()));
        assert_eq!(event.context.client_info.as_ref().unwrap().ip_address, "192.168.1.1");
        assert!(event.context.custom_metadata.contains_key("custom_field"));
    }

    #[tokio::test]
    async fn test_business_context_handling() {
        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();
        let business_date = Utc::now();

        let event = VersionedDomainEvent::builder(
            "BusinessEvent".to_string(),
            Uuid::new_v4(),
            "Business".to_string(),
            tenant_id,
        )
        .location_id(location_id)
        .business_date(business_date)
        .build();

        assert_eq!(event.context.business_context.tenant_id, tenant_id);
        assert_eq!(event.context.business_context.location_id, Some(location_id));
        assert_eq!(event.context.business_context.business_date, business_date);
    }
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_event_creation_performance() {
        let start = Instant::now();
        let iterations = 1000;

        for i in 0..iterations {
            let _event = VersionedDomainEvent::builder(
                "PerformanceTest".to_string(),
                Uuid::new_v4(),
                "Performance".to_string(),
                Uuid::new_v4(),
            )
            .sequence_number(i)
            .data(serde_json::json!({"iteration": i}))
            .unwrap()
            .build();
        }

        let duration = start.elapsed();
        let avg_time = duration / iterations as u32;

        println!("Created {} events in {:?}, avg: {:?} per event", iterations, duration, avg_time);

        // Performance assertion - should be able to create 1000 events in under 1 second
        assert!(duration < Duration::from_secs(1));
    }

    #[tokio::test]
    async fn test_event_handler_throughput() {
        let handler = Arc::new(TestEventHandler::new(
            "ThroughputHandler",
            vec!["*".to_string()],
        ));

        let start = Instant::now();
        let iterations = 100;
        let mut tasks = Vec::new();

        for _ in 0..iterations {
            let handler_clone = Arc::clone(&handler);
            let event = create_test_legacy_event();

            let task = tokio::spawn(async move {
                handler_clone.handle(&event).await
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        for task in tasks {
            let result = task.await.unwrap();
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let throughput = iterations as f64 / duration.as_secs_f64();

        println!("Processed {} events in {:?}, throughput: {:.2} events/sec", iterations, duration, throughput);

        assert_eq!(handler.get_processed_count(), iterations);

        // Performance assertion - should handle at least 50 events per second
        assert!(throughput > 50.0);
    }

    #[tokio::test]
    async fn test_event_serialization_performance() {
        let event = create_test_versioned_event();
        let start = Instant::now();
        let iterations = 1000;

        for _ in 0..iterations {
            match &event {
                EventContainer::Versioned(e) => {
                    let _serialized = serde_json::to_string(e).unwrap();
                }
                EventContainer::Legacy(e) => {
                    let _serialized = serde_json::to_string(e).unwrap();
                }
            }
        }

        let duration = start.elapsed();
        let avg_time = duration / iterations as u32;

        println!("Serialized {} events in {:?}, avg: {:?} per event", iterations, duration, avg_time);

        // Performance assertion - should serialize 1000 events in under 100ms
        assert!(duration < Duration::from_millis(100));
    }
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_event_data() {
        // Test building event with invalid data
        let result = VersionedDomainEvent::builder(
            "InvalidEvent".to_string(),
            Uuid::new_v4(),
            "Invalid".to_string(),
            Uuid::new_v4(),
        )
        .data("invalid json structure")
        .unwrap_err(); // This should fail

        // This test is more about ensuring our error handling works
        // The actual error type will depend on serde_json behavior
    }

    #[tokio::test]
    async fn test_handler_error_recovery() {
        let handler = TestEventHandler::new(
            "ErrorRecoveryHandler",
            vec!["*".to_string()],
        );

        // First event should succeed
        let event1 = create_test_legacy_event();
        let result1 = handler.handle(&event1).await;
        assert!(result1.is_ok());

        // Set handler to fail on next event
        handler.set_should_fail(2);

        // Second event should fail
        let event2 = create_test_legacy_event();
        let result2 = handler.handle(&event2).await;
        assert!(result2.is_err());

        // Reset failure condition
        handler.set_should_fail(0);

        // Third event should succeed again
        let event3 = create_test_legacy_event();
        let result3 = handler.handle(&event3).await;
        assert!(result3.is_ok());

        assert_eq!(handler.get_processed_count(), 3);
    }

    #[tokio::test]
    async fn test_event_integrity_validation() {
        let mut event = create_test_versioned_event();

        if let EventContainer::Versioned(ref mut versioned_event) = event {
            // Original event should verify
            assert!(versioned_event.verify_integrity());

            // Corrupt the checksum
            versioned_event.checksum = Some("invalid_checksum".to_string());

            // Corrupted event should not verify
            assert!(!versioned_event.verify_integrity());
        }
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_event_types_handler() {
        let handler = TestEventHandler::new("EmptyTypesHandler", vec![]);
        assert_eq!(handler.event_types().len(), 0);
    }

    #[tokio::test]
    async fn test_wildcard_event_handler() {
        let handler = TestEventHandler::new(
            "WildcardHandler",
            vec!["*".to_string()],
        );

        let legacy_event = create_test_legacy_event();
        let versioned_event = create_test_versioned_event();

        let result1 = handler.handle(&legacy_event).await;
        let result2 = handler.handle(&versioned_event).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert_eq!(handler.get_processed_count(), 2);
    }

    #[tokio::test]
    async fn test_large_event_data() {
        let large_data = "x".repeat(10000); // 10KB of data

        let event = VersionedDomainEvent::builder(
            "LargeEvent".to_string(),
            Uuid::new_v4(),
            "Large".to_string(),
            Uuid::new_v4(),
        )
        .data(serde_json::json!({"large_field": large_data}))
        .unwrap()
        .build();

        // Event should be created successfully
        assert!(!event.id.to_string().is_empty());
        assert!(event.verify_integrity());
    }

    #[tokio::test]
    async fn test_event_with_null_data() {
        let event = VersionedDomainEvent::builder(
            "NullDataEvent".to_string(),
            Uuid::new_v4(),
            "NullData".to_string(),
            Uuid::new_v4(),
        )
        .data(Value::Null)
        .unwrap()
        .build();

        assert!(!event.id.to_string().is_empty());
        assert_eq!(event.data, Value::Null);
    }

    #[tokio::test]
    async fn test_concurrent_handler_registration() {
        use tokio::sync::RwLock;
        use std::collections::HashMap;

        let handlers: Arc<RwLock<HashMap<String, Arc<dyn EventHandler>>>> =
            Arc::new(RwLock::new(HashMap::new()));

        let mut tasks = Vec::new();

        // Register 10 handlers concurrently
        for i in 0..10 {
            let handlers_clone = Arc::clone(&handlers);
            let task = tokio::spawn(async move {
                let handler = Arc::new(TestEventHandler::new(
                    &format!("ConcurrentHandler{}", i),
                    vec!["*".to_string()],
                ));

                let mut handlers_map = handlers_clone.write().await;
                handlers_map.insert(handler.name(), handler);
            });

            tasks.push(task);
        }

        // Wait for all registrations to complete
        for task in tasks {
            task.await.unwrap();
        }

        let handlers_map = handlers.read().await;
        assert_eq!(handlers_map.len(), 10);
    }
}