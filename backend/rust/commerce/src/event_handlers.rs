// ============================================================================
// OLYMPUS CLOUD - COMMERCE SERVICE EVENT HANDLERS
// ============================================================================
// Module: commerce/src/event_handlers.rs
// Description: Event handlers for commerce service cross-service communication
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 1.0 - Phase 5 Event-Driven Architecture
// ============================================================================

use async_trait::async_trait;
use shared::events::{
    EventHandler, EventContainer, HandlerPriority, HandlerHealth,
    OrderCreatedEvent, OrderStatusChangedEvent, PaymentProcessedEvent,
    InventoryAdjustedEvent, ProductCreatedEvent, ProductUpdatedEvent,
    UserRegisteredEvent, LocationCreatedEvent, TenantCreatedEvent,
    OrderLifecycleEvent, InventoryMovementEvent,
    commerce_events, auth_events, platform_events,
};
use shared::{Error, Result};
use tracing::{info, warn, error, debug, instrument};
use uuid::Uuid;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use rust_decimal::Decimal;

// ============================================================================
// ORDER LIFECYCLE HANDLER
// ============================================================================

/// Handles order lifecycle events and coordinates order processing
pub struct OrderLifecycleHandler {
    name: String,
    processed_count: AtomicU64,
    failed_count: AtomicU64,
    order_metrics: Arc<RwLock<OrderMetrics>>,
}

#[derive(Debug, Clone)]
struct OrderMetrics {
    total_orders: u64,
    pending_orders: u64,
    completed_orders: u64,
    cancelled_orders: u64,
    average_order_value: Decimal,
    total_revenue: Decimal,
}

impl OrderLifecycleHandler {
    pub fn new() -> Self {
        Self {
            name: "OrderLifecycleHandler".to_string(),
            processed_count: AtomicU64::new(0),
            failed_count: AtomicU64::new(0),
            order_metrics: Arc::new(RwLock::new(OrderMetrics {
                total_orders: 0,
                pending_orders: 0,
                completed_orders: 0,
                cancelled_orders: 0,
                average_order_value: Decimal::ZERO,
                total_revenue: Decimal::ZERO,
            })),
        }
    }

    /// Handle new order creation events
    #[instrument(skip(self, event))]
    async fn handle_order_created(&self, event: &OrderCreatedEvent) -> Result<()> {
        info!(
            "Processing new order {} for customer {:?} at location {}",
            event.order_number, event.customer_id, event.location_id
        );

        // Order processing tasks:
        // 1. Validate order data
        // 2. Check inventory availability
        // 3. Calculate taxes and fees
        // 4. Set initial order status
        // 5. Create order processing workflow

        let mut metrics = self.order_metrics.write().await;
        metrics.total_orders += 1;
        metrics.pending_orders += 1;
        metrics.total_revenue += event.total_amount;

        // Update average order value
        if metrics.total_orders > 0 {
            metrics.average_order_value = metrics.total_revenue / Decimal::from(metrics.total_orders);
        }

        info!(
            "Order {} created successfully with total amount {} {}",
            event.order_number, event.total_amount, event.currency
        );

        Ok(())
    }

    /// Handle order status changes
    #[instrument(skip(self, event))]
    async fn handle_order_status_changed(&self, event: &OrderStatusChangedEvent) -> Result<()> {
        info!(
            "Order {} status changed from {} to {}",
            event.order_id, event.old_status, event.new_status
        );

        // Status change processing:
        // 1. Update order tracking
        // 2. Trigger notifications
        // 3. Update inventory if needed
        // 4. Handle payment processing
        // 5. Update metrics

        let mut metrics = self.order_metrics.write().await;

        match event.new_status.as_str() {
            "Completed" => {
                if event.old_status != "Completed" {
                    metrics.completed_orders += 1;
                    if event.old_status == "Pending" || event.old_status == "Confirmed" {
                        metrics.pending_orders = metrics.pending_orders.saturating_sub(1);
                    }
                }
            }
            "Cancelled" => {
                if event.old_status != "Cancelled" {
                    metrics.cancelled_orders += 1;
                    if event.old_status == "Pending" || event.old_status == "Confirmed" {
                        metrics.pending_orders = metrics.pending_orders.saturating_sub(1);
                    }
                }
            }
            _ => {
                // Handle other status changes
                debug!("Order {} transitioned to status: {}", event.order_id, event.new_status);
            }
        }

        if let Some(fulfillment_time) = event.estimated_fulfillment {
            debug!("Estimated fulfillment time: {}", fulfillment_time);
        }

        Ok(())
    }

    /// Handle payment processing events
    #[instrument(skip(self, event))]
    async fn handle_payment_processed(&self, event: &PaymentProcessedEvent) -> Result<()> {
        info!(
            "Payment {} processed for order {} with status {:?}",
            event.payment_id, event.order_id, event.status
        );

        // Payment processing tasks:
        // 1. Update order payment status
        // 2. Handle payment success/failure
        // 3. Update financial records
        // 4. Trigger order fulfillment if payment successful
        // 5. Handle fraud detection

        match event.status {
            shared::events::PaymentStatus::Completed => {
                info!(
                    "Payment completed successfully: {} {} via {}",
                    event.amount, event.currency, event.payment_method
                );
                // TODO: Trigger order fulfillment
            }
            shared::events::PaymentStatus::Failed => {
                warn!(
                    "Payment failed for order {}: {} via {}",
                    event.order_id, event.payment_method, event.gateway
                );
                // TODO: Handle payment failure, potentially hold order
            }
            _ => {
                debug!("Payment status update: {:?}", event.status);
            }
        }

        Ok(())
    }
}

#[async_trait]
impl EventHandler for OrderLifecycleHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    commerce_events::ORDER_CREATED => {
                        if let Ok(order_event) = serde_json::from_value::<OrderCreatedEvent>(domain_event.data.clone()) {
                            self.handle_order_created(&order_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderCreatedEvent".to_string()))
                        }
                    }
                    commerce_events::ORDER_STATUS_CHANGED => {
                        if let Ok(status_event) = serde_json::from_value::<OrderStatusChangedEvent>(domain_event.data.clone()) {
                            self.handle_order_status_changed(&status_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderStatusChangedEvent".to_string()))
                        }
                    }
                    commerce_events::PAYMENT_PROCESSED => {
                        if let Ok(payment_event) = serde_json::from_value::<PaymentProcessedEvent>(domain_event.data.clone()) {
                            self.handle_payment_processed(&payment_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize PaymentProcessedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Order lifecycle handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    commerce_events::ORDER_CREATED => {
                        if let Ok(order_event) = serde_json::from_value::<OrderCreatedEvent>(versioned_event.data.clone()) {
                            self.handle_order_created(&order_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderCreatedEvent from versioned event".to_string()))
                        }
                    }
                    commerce_events::ORDER_STATUS_CHANGED => {
                        if let Ok(status_event) = serde_json::from_value::<OrderStatusChangedEvent>(versioned_event.data.clone()) {
                            self.handle_order_status_changed(&status_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderStatusChangedEvent from versioned event".to_string()))
                        }
                    }
                    commerce_events::PAYMENT_PROCESSED => {
                        if let Ok(payment_event) = serde_json::from_value::<PaymentProcessedEvent>(versioned_event.data.clone()) {
                            self.handle_payment_processed(&payment_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize PaymentProcessedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Order lifecycle handler ignoring versioned event type: {}", versioned_event.event_type);
                        Ok(())
                    }
                }
            }
        };

        match &result {
            Ok(_) => {
                self.processed_count.fetch_add(1, Ordering::Relaxed);
            }
            Err(_) => {
                self.failed_count.fetch_add(1, Ordering::Relaxed);
            }
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            commerce_events::ORDER_CREATED.to_string(),
            commerce_events::ORDER_STATUS_CHANGED.to_string(),
            commerce_events::PAYMENT_PROCESSED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::High // Order processing is high priority
    }

    fn supports_concurrent_processing(&self) -> bool {
        true // Can process multiple orders concurrently
    }

    fn max_concurrent_events(&self) -> usize {
        10 // Process up to 10 order events concurrently
    }

    async fn health_check(&self) -> HandlerHealth {
        let processed = self.processed_count.load(Ordering::Relaxed);
        let failed = self.failed_count.load(Ordering::Relaxed);

        if processed == 0 {
            return HandlerHealth::Healthy;
        }

        let failure_rate = (failed as f64) / (processed as f64);

        if failure_rate > 0.2 {
            HandlerHealth::Unhealthy(format!("High order processing failure rate: {:.2}%", failure_rate * 100.0))
        } else if failure_rate > 0.05 {
            HandlerHealth::Degraded(format!("Elevated order processing failure rate: {:.2}%", failure_rate * 100.0))
        } else {
            HandlerHealth::Healthy
        }
    }
}

// ============================================================================
// INVENTORY MANAGEMENT HANDLER
// ============================================================================

/// Handles inventory-related events and maintains stock levels
pub struct InventoryManagementHandler {
    name: String,
    processed_count: AtomicU64,
    low_stock_threshold: i32,
    inventory_cache: Arc<RwLock<HashMap<Uuid, InventoryInfo>>>,
}

#[derive(Debug, Clone)]
struct InventoryInfo {
    product_id: Uuid,
    location_id: Uuid,
    current_quantity: i32,
    reserved_quantity: i32,
    last_updated: chrono::DateTime<chrono::Utc>,
}

impl InventoryManagementHandler {
    pub fn new() -> Self {
        Self {
            name: "InventoryManagementHandler".to_string(),
            processed_count: AtomicU64::new(0),
            low_stock_threshold: 10,
            inventory_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle inventory adjustments
    #[instrument(skip(self, event))]
    async fn handle_inventory_adjusted(&self, event: &InventoryAdjustedEvent) -> Result<()> {
        info!(
            "Processing inventory adjustment for product {} at location {}: {} -> {}",
            event.product_id, event.location_id, event.old_quantity, event.new_quantity
        );

        // Inventory management tasks:
        // 1. Update inventory records
        // 2. Check for low stock alerts
        // 3. Update inventory cache
        // 4. Trigger reorder if needed
        // 5. Validate adjustment reasons

        let inventory_key = format!("{}-{}", event.product_id, event.location_id);
        let mut cache = self.inventory_cache.write().await;

        cache.insert(event.product_id, InventoryInfo {
            product_id: event.product_id,
            location_id: event.location_id,
            current_quantity: event.new_quantity,
            reserved_quantity: 0, // Would be calculated from active orders
            last_updated: chrono::Utc::now(),
        });

        // Check for low stock
        if event.new_quantity <= self.low_stock_threshold {
            warn!(
                "Low stock alert: Product {} at location {} has {} units remaining",
                event.product_id, event.location_id, event.new_quantity
            );
            // TODO: Trigger low stock notification
        }

        // Log adjustment reason
        match event.adjustment_reason {
            shared::events::InventoryAdjustmentReason::Sale => {
                debug!("Inventory reduced due to sale");
            }
            shared::events::InventoryAdjustmentReason::Return => {
                info!("Inventory increased due to return");
            }
            shared::events::InventoryAdjustmentReason::Damage => {
                warn!("Inventory reduced due to damage");
            }
            shared::events::InventoryAdjustmentReason::Theft => {
                error!("Inventory reduced due to theft - security alert required");
            }
            _ => {
                debug!("Inventory adjustment: {:?}", event.adjustment_reason);
            }
        }

        Ok(())
    }

    /// Handle product creation to initialize inventory
    #[instrument(skip(self, event))]
    async fn handle_product_created(&self, event: &ProductCreatedEvent) -> Result<()> {
        info!("Initializing inventory tracking for new product: {}", event.product_id);

        // Product creation tasks:
        // 1. Initialize inventory records
        // 2. Set initial stock levels
        // 3. Configure reorder points
        // 4. Set up inventory alerts

        debug!("Product {} created with SKU {:?}", event.product_id, event.sku);
        Ok(())
    }

    /// Handle location creation to set up inventory tracking
    #[instrument(skip(self, event))]
    async fn handle_location_created(&self, event: &LocationCreatedEvent) -> Result<()> {
        info!("Setting up inventory tracking for new location: {}", event.location_id);

        // Location setup tasks:
        // 1. Initialize location inventory
        // 2. Set up location-specific stock rules
        // 3. Configure transfer workflows
        // 4. Set up location-specific alerts

        debug!("Location {} created: {}", event.location_id, event.name);
        Ok(())
    }
}

#[async_trait]
impl EventHandler for InventoryManagementHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    commerce_events::INVENTORY_ADJUSTED => {
                        if let Ok(inventory_event) = serde_json::from_value::<InventoryAdjustedEvent>(domain_event.data.clone()) {
                            self.handle_inventory_adjusted(&inventory_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize InventoryAdjustedEvent".to_string()))
                        }
                    }
                    commerce_events::PRODUCT_CREATED => {
                        if let Ok(product_event) = serde_json::from_value::<ProductCreatedEvent>(domain_event.data.clone()) {
                            self.handle_product_created(&product_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize ProductCreatedEvent".to_string()))
                        }
                    }
                    platform_events::LOCATION_CREATED => {
                        if let Ok(location_event) = serde_json::from_value::<LocationCreatedEvent>(domain_event.data.clone()) {
                            self.handle_location_created(&location_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize LocationCreatedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Inventory handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    commerce_events::INVENTORY_ADJUSTED => {
                        if let Ok(inventory_event) = serde_json::from_value::<InventoryAdjustedEvent>(versioned_event.data.clone()) {
                            self.handle_inventory_adjusted(&inventory_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize InventoryAdjustedEvent from versioned event".to_string()))
                        }
                    }
                    commerce_events::PRODUCT_CREATED => {
                        if let Ok(product_event) = serde_json::from_value::<ProductCreatedEvent>(versioned_event.data.clone()) {
                            self.handle_product_created(&product_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize ProductCreatedEvent from versioned event".to_string()))
                        }
                    }
                    platform_events::LOCATION_CREATED => {
                        if let Ok(location_event) = serde_json::from_value::<LocationCreatedEvent>(versioned_event.data.clone()) {
                            self.handle_location_created(&location_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize LocationCreatedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Inventory handler ignoring versioned event type: {}", versioned_event.event_type);
                        Ok(())
                    }
                }
            }
        };

        if result.is_ok() {
            self.processed_count.fetch_add(1, Ordering::Relaxed);
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            commerce_events::INVENTORY_ADJUSTED.to_string(),
            commerce_events::PRODUCT_CREATED.to_string(),
            platform_events::LOCATION_CREATED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::High // Inventory management is high priority
    }

    fn supports_concurrent_processing(&self) -> bool {
        false // Inventory updates should be sequential to avoid race conditions
    }

    fn max_concurrent_events(&self) -> usize {
        1 // Process inventory events sequentially
    }

    async fn health_check(&self) -> HandlerHealth {
        let processed = self.processed_count.load(Ordering::Relaxed);

        if processed > 0 {
            HandlerHealth::Healthy
        } else {
            HandlerHealth::Degraded("No inventory events processed yet".to_string())
        }
    }
}

// ============================================================================
// COMMERCE ANALYTICS HANDLER
// ============================================================================

/// Handles commerce events for analytics and reporting
pub struct CommerceAnalyticsHandler {
    name: String,
    processed_count: AtomicU64,
    analytics_cache: Arc<RwLock<CommerceAnalytics>>,
}

#[derive(Debug, Clone)]
struct CommerceAnalytics {
    total_sales: Decimal,
    transaction_count: u64,
    average_transaction_value: Decimal,
    top_products: HashMap<Uuid, ProductSalesData>,
    daily_sales: HashMap<String, Decimal>, // Date -> Sales amount
}

#[derive(Debug, Clone)]
struct ProductSalesData {
    product_id: Uuid,
    quantity_sold: i32,
    revenue: Decimal,
    last_sale: chrono::DateTime<chrono::Utc>,
}

impl CommerceAnalyticsHandler {
    pub fn new() -> Self {
        Self {
            name: "CommerceAnalyticsHandler".to_string(),
            processed_count: AtomicU64::new(0),
            analytics_cache: Arc::new(RwLock::new(CommerceAnalytics {
                total_sales: Decimal::ZERO,
                transaction_count: 0,
                average_transaction_value: Decimal::ZERO,
                top_products: HashMap::new(),
                daily_sales: HashMap::new(),
            })),
        }
    }

    /// Process order events for analytics
    #[instrument(skip(self, event))]
    async fn process_order_analytics(&self, event: &OrderCreatedEvent) -> Result<()> {
        let mut analytics = self.analytics_cache.write().await;

        analytics.total_sales += event.total_amount;
        analytics.transaction_count += 1;
        analytics.average_transaction_value = analytics.total_sales / Decimal::from(analytics.transaction_count);

        // Track daily sales
        let date_key = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let daily_total = analytics.daily_sales.entry(date_key).or_insert(Decimal::ZERO);
        *daily_total += event.total_amount;

        info!(
            "Analytics updated: Total sales: {}, Avg transaction: {}, Daily sales: {}",
            analytics.total_sales,
            analytics.average_transaction_value,
            daily_total
        );

        Ok(())
    }

    /// Process payment events for analytics
    #[instrument(skip(self, event))]
    async fn process_payment_analytics(&self, event: &PaymentProcessedEvent) -> Result<()> {
        match event.status {
            shared::events::PaymentStatus::Completed => {
                debug!("Payment analytics: Successful payment of {} {}", event.amount, event.currency);
                // TODO: Update payment method analytics
            }
            shared::events::PaymentStatus::Failed => {
                warn!("Payment analytics: Failed payment of {} {}", event.amount, event.currency);
                // TODO: Track payment failure analytics
            }
            _ => {
                debug!("Payment analytics: Payment status update {:?}", event.status);
            }
        }

        Ok(())
    }
}

#[async_trait]
impl EventHandler for CommerceAnalyticsHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    commerce_events::ORDER_CREATED => {
                        if let Ok(order_event) = serde_json::from_value::<OrderCreatedEvent>(domain_event.data.clone()) {
                            self.process_order_analytics(&order_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderCreatedEvent".to_string()))
                        }
                    }
                    commerce_events::PAYMENT_PROCESSED => {
                        if let Ok(payment_event) = serde_json::from_value::<PaymentProcessedEvent>(domain_event.data.clone()) {
                            self.process_payment_analytics(&payment_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize PaymentProcessedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Analytics handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    commerce_events::ORDER_CREATED => {
                        if let Ok(order_event) = serde_json::from_value::<OrderCreatedEvent>(versioned_event.data.clone()) {
                            self.process_order_analytics(&order_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderCreatedEvent from versioned event".to_string()))
                        }
                    }
                    commerce_events::PAYMENT_PROCESSED => {
                        if let Ok(payment_event) = serde_json::from_value::<PaymentProcessedEvent>(versioned_event.data.clone()) {
                            self.process_payment_analytics(&payment_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize PaymentProcessedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Analytics handler ignoring versioned event type: {}", versioned_event.event_type);
                        Ok(())
                    }
                }
            }
        };

        if result.is_ok() {
            self.processed_count.fetch_add(1, Ordering::Relaxed);
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            commerce_events::ORDER_CREATED.to_string(),
            commerce_events::ORDER_STATUS_CHANGED.to_string(),
            commerce_events::PAYMENT_PROCESSED.to_string(),
            commerce_events::INVENTORY_ADJUSTED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Low // Analytics is important but not urgent
    }

    fn supports_concurrent_processing(&self) -> bool {
        true // Analytics can be processed concurrently
    }

    fn max_concurrent_events(&self) -> usize {
        5 // Process up to 5 analytics events concurrently
    }

    async fn health_check(&self) -> HandlerHealth {
        HandlerHealth::Healthy // Analytics handler is always healthy
    }
}

// ============================================================================
// HANDLER FACTORY
// ============================================================================

/// Factory for creating commerce service event handlers
pub struct CommerceEventHandlerFactory;

impl CommerceEventHandlerFactory {
    /// Create all commerce service event handlers
    pub fn create_handlers() -> Vec<Arc<dyn EventHandler>> {
        vec![
            Arc::new(OrderLifecycleHandler::new()),
            Arc::new(InventoryManagementHandler::new()),
            Arc::new(CommerceAnalyticsHandler::new()),
        ]
    }

    /// Create a specific handler by name
    pub fn create_handler(name: &str) -> Option<Arc<dyn EventHandler>> {
        match name {
            "OrderLifecycleHandler" => Some(Arc::new(OrderLifecycleHandler::new())),
            "InventoryManagementHandler" => Some(Arc::new(InventoryManagementHandler::new())),
            "CommerceAnalyticsHandler" => Some(Arc::new(CommerceAnalyticsHandler::new())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::events::{DomainEvent, EventMetadata, OrderSource};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_order_created_event() -> OrderCreatedEvent {
        OrderCreatedEvent {
            order_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            customer_id: Some(Uuid::new_v4()),
            location_id: Uuid::new_v4(),
            order_number: "ORD-001".to_string(),
            total_amount: Decimal::from(100),
            currency: "USD".to_string(),
            item_count: 2,
            order_source: OrderSource::Web,
        }
    }

    fn create_test_domain_event(event_type: &str, data: serde_json::Value) -> EventContainer {
        let domain_event = DomainEvent {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            aggregate_id: Uuid::new_v4(),
            aggregate_type: "Order".to_string(),
            tenant_id: Uuid::new_v4(),
            data,
            metadata: EventMetadata {
                user_id: Some(Uuid::new_v4()),
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

    #[tokio::test]
    async fn test_order_lifecycle_handler() {
        let handler = OrderLifecycleHandler::new();
        let order_event = create_test_order_created_event();
        let event_data = serde_json::to_value(&order_event).unwrap();
        let event = create_test_domain_event(commerce_events::ORDER_CREATED, event_data);

        let result = handler.handle(&event).await;
        assert!(result.is_ok());
        assert_eq!(handler.processed_count.load(Ordering::Relaxed), 1);

        let health = handler.health_check().await;
        assert_eq!(health, HandlerHealth::Healthy);
    }

    #[tokio::test]
    async fn test_inventory_management_handler() {
        let handler = InventoryManagementHandler::new();
        assert_eq!(handler.name(), "InventoryManagementHandler");
        assert_eq!(handler.priority(), HandlerPriority::High);
        assert!(!handler.supports_concurrent_processing()); // Sequential processing
        assert_eq!(handler.max_concurrent_events(), 1);
    }

    #[tokio::test]
    async fn test_commerce_analytics_handler() {
        let handler = CommerceAnalyticsHandler::new();
        let order_event = create_test_order_created_event();
        let event_data = serde_json::to_value(&order_event).unwrap();
        let event = create_test_domain_event(commerce_events::ORDER_CREATED, event_data);

        let result = handler.handle(&event).await;
        assert!(result.is_ok());
        assert_eq!(handler.processed_count.load(Ordering::Relaxed), 1);

        // Analytics handler supports concurrent processing
        assert!(handler.supports_concurrent_processing());
        assert_eq!(handler.priority(), HandlerPriority::Low);
    }

    #[test]
    fn test_handler_factory() {
        let handlers = CommerceEventHandlerFactory::create_handlers();
        assert_eq!(handlers.len(), 3);

        let specific_handler = CommerceEventHandlerFactory::create_handler("OrderLifecycleHandler");
        assert!(specific_handler.is_some());

        let unknown_handler = CommerceEventHandlerFactory::create_handler("UnknownHandler");
        assert!(unknown_handler.is_none());
    }
}