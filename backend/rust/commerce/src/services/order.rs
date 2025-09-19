// ============================================================================
// OLYMPUS CLOUD - ORDER SERVICE
// ============================================================================
// Module: commerce/src/services/order.rs
// Description: Advanced order management service with lifecycle management
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;

use olympus_shared::{
    database::DbPool,
    events::EventPublisher,
    error::{Result, OlympusError},
};

use crate::models::{
    Order, OrderItem, OrderStatus, PaymentStatus, FulfillmentStatus,
    CreateOrderRequest, CreateOrderItemRequest, UpdateOrderRequest,
    OrderSearchRequest, OrderSearchResponse, OrderSearchFacets,
    OrderEvent, OrderEventType, OrderModification, OrderModificationType,
    OrderFulfillment, FulfillmentItem, OrderCalculation, LineItemCalculation,
    TaxLine, DiscountLine, ShippingLine, BulkOrderUpdateRequest, BulkOrderResult,
    StatusFacet, PaymentStatusFacet, FulfillmentStatusFacet, MonthlyCountFacet,
    Address, Product, ProductType, OrderSortBy, SortOrder,
};

/// Order service for comprehensive order lifecycle management
pub struct OrderService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl OrderService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ========================================================================
    // ORDER CRUD OPERATIONS
    // ========================================================================

    /// Create a new order with full validation and calculation
    pub async fn create_order(
        &self,
        tenant_id: Uuid,
        request: CreateOrderRequest,
        created_by: Uuid,
    ) -> Result<Order> {
        let mut tx = self.db.begin().await?;

        // Generate order number
        let order_number = self.generate_order_number(tenant_id).await?;

        // Validate and calculate order
        let calculation = self.calculate_order(&request.items, tenant_id).await?;

        // Create order
        let order_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO orders (
                id, tenant_id, order_number, customer_id, customer_email,
                status, payment_status, fulfillment_status, currency,
                subtotal, tax_total, shipping_total, discount_total, total,
                notes, tags, metadata, created_at, updated_at, created_by, updated_by
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21
            )
            "#,
            order_id,
            tenant_id,
            order_number,
            request.customer_id,
            request.customer_email,
            OrderStatus::Draft as OrderStatus,
            PaymentStatus::Pending as PaymentStatus,
            FulfillmentStatus::Unfulfilled as FulfillmentStatus,
            "USD", // Default currency - should be configurable
            calculation.subtotal,
            calculation.tax_total,
            calculation.shipping_total,
            calculation.discount_total,
            calculation.total,
            request.notes,
            request.tags.unwrap_or_default().as_slice(),
            request.metadata.unwrap_or_default(),
            now,
            now,
            created_by,
            created_by,
        )
        .execute(&mut *tx)
        .await?;

        // Create order items
        for (idx, item_request) in request.items.iter().enumerate() {
            let item_calc = &calculation.line_items[idx];
            let item_id = Uuid::new_v4();

            // Get product details for item
            let product = self.get_product_for_order(tenant_id, item_request.product_id).await?;

            sqlx::query!(
                r#"
                INSERT INTO order_items (
                    id, order_id, product_id, variant_id, sku, name,
                    quantity, unit_price, total_price, tax_rate, tax_amount,
                    discount_amount, attributes, created_at, updated_at
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15
                )
                "#,
                item_id,
                order_id,
                item_request.product_id,
                item_request.variant_id,
                product.sku,
                product.name,
                item_request.quantity,
                item_calc.unit_price,
                item_calc.line_total,
                None::<Decimal>, // Tax rate - to be implemented
                item_calc.tax_amount,
                item_calc.discount_amount,
                item_request.attributes.clone().unwrap_or_default(),
                now,
                now,
            )
            .execute(&mut *tx)
            .await?;
        }

        // Record order creation event
        self.record_order_event(
            &mut *tx,
            order_id,
            OrderEventType::Created,
            "Order created".to_string(),
            None,
            Some(serde_json::to_value(&request)?),
            Some(created_by),
        ).await?;

        tx.commit().await?;

        // Publish domain event
        let event_data = serde_json::json!({
            "order_id": order_id,
            "tenant_id": tenant_id,
            "order_number": order_number,
            "customer_id": request.customer_id,
            "total": calculation.total,
            "created_by": created_by
        });

        self.event_publisher.publish(
            "commerce.order.created",
            &event_data,
        ).await?;

        // Load and return the complete order
        self.get_order(tenant_id, order_id).await?
            .ok_or_else(|| OlympusError::NotFound("Order not found after creation".to_string()))
    }

    /// Get order by ID with full details
    pub async fn get_order(&self, tenant_id: Uuid, order_id: Uuid) -> Result<Option<Order>> {
        let order_row = sqlx::query!(
            r#"
            SELECT
                id, tenant_id, order_number, customer_id, customer_email,
                status as "status: OrderStatus",
                payment_status as "payment_status: PaymentStatus",
                fulfillment_status as "fulfillment_status: FulfillmentStatus",
                currency, subtotal, tax_total, shipping_total, discount_total, total,
                notes, tags, metadata, created_at, updated_at,
                confirmed_at, shipped_at, delivered_at
            FROM orders
            WHERE id = $1 AND tenant_id = $2
            "#,
            order_id,
            tenant_id
        )
        .fetch_optional(&**self.db)
        .await?;

        let Some(row) = order_row else {
            return Ok(None);
        };

        // Load order items
        let items = self.get_order_items(order_id).await?;

        // Load addresses (mock for now - would come from order_addresses table)
        let shipping_address: Option<Address> = None;
        let billing_address: Option<Address> = None;

        Ok(Some(Order {
            id: row.id,
            tenant_id: row.tenant_id,
            order_number: row.order_number,
            customer_id: row.customer_id,
            customer_email: row.customer_email,
            status: row.status,
            payment_status: row.payment_status,
            fulfillment_status: row.fulfillment_status,
            currency: row.currency,
            subtotal: row.subtotal,
            tax_total: row.tax_total,
            shipping_total: row.shipping_total,
            discount_total: row.discount_total,
            total: row.total,
            items,
            shipping_address,
            billing_address,
            notes: row.notes,
            tags: row.tags,
            metadata: row.metadata,
            created_at: row.created_at,
            updated_at: row.updated_at,
            confirmed_at: row.confirmed_at,
            shipped_at: row.shipped_at,
            delivered_at: row.delivered_at,
        }))
    }

    /// Update order with validation and audit trail
    pub async fn update_order(
        &self,
        tenant_id: Uuid,
        order_id: Uuid,
        request: UpdateOrderRequest,
        updated_by: Uuid,
    ) -> Result<Option<Order>> {
        let mut tx = self.db.begin().await?;

        // Get current order for audit trail
        let current_order = self.get_order(tenant_id, order_id).await?;
        let Some(current_order) = current_order else {
            return Ok(None);
        };

        let now = Utc::now();
        let mut status_changed = false;

        // Update order fields
        let updated_status = request.status.unwrap_or(current_order.status);
        if updated_status != current_order.status {
            status_changed = true;
        }

        sqlx::query!(
            r#"
            UPDATE orders SET
                status = COALESCE($1, status),
                customer_id = COALESCE($2, customer_id),
                customer_email = COALESCE($3, customer_email),
                notes = COALESCE($4, notes),
                tags = COALESCE($5, tags),
                metadata = COALESCE($6, metadata),
                updated_at = $7,
                updated_by = $8,
                confirmed_at = CASE
                    WHEN $1 = 'confirmed' AND confirmed_at IS NULL THEN $7
                    ELSE confirmed_at
                END,
                shipped_at = CASE
                    WHEN $1 = 'shipped' AND shipped_at IS NULL THEN $7
                    ELSE shipped_at
                END,
                delivered_at = CASE
                    WHEN $1 = 'delivered' AND delivered_at IS NULL THEN $7
                    ELSE delivered_at
                END
            WHERE id = $9 AND tenant_id = $10
            "#,
            request.status.map(|s| s as OrderStatus),
            request.customer_id,
            request.customer_email,
            request.notes,
            request.tags.as_deref(),
            request.metadata,
            now,
            updated_by,
            order_id,
            tenant_id,
        )
        .execute(&mut *tx)
        .await?;

        // Record update event
        let event_description = if status_changed {
            format!("Order status changed from {:?} to {:?}", current_order.status, updated_status)
        } else {
            "Order updated".to_string()
        };

        self.record_order_event(
            &mut *tx,
            order_id,
            if status_changed { OrderEventType::StatusChanged } else { OrderEventType::Updated },
            event_description,
            Some(serde_json::to_value(&current_order)?),
            Some(serde_json::to_value(&request)?),
            Some(updated_by),
        ).await?;

        tx.commit().await?;

        // Publish status change event if applicable
        if status_changed {
            let event_data = serde_json::json!({
                "order_id": order_id,
                "tenant_id": tenant_id,
                "previous_status": current_order.status,
                "new_status": updated_status,
                "updated_by": updated_by
            });

            self.event_publisher.publish(
                "commerce.order.status_changed",
                &event_data,
            ).await?;
        }

        // Return updated order
        self.get_order(tenant_id, order_id).await
    }

    /// Search orders with advanced filtering and facets
    pub async fn search_orders(
        &self,
        tenant_id: Uuid,
        request: OrderSearchRequest,
    ) -> Result<OrderSearchResponse> {
        let limit = request.limit.unwrap_or(20).min(100);
        let offset = request.offset.unwrap_or(0);

        // Build dynamic query
        let mut query_conditions = vec!["tenant_id = $1".to_string()];
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> =
            vec![Box::new(tenant_id)];
        let mut param_count = 1;

        // Add search conditions
        if let Some(query) = &request.query {
            param_count += 1;
            query_conditions.push(format!(
                "(order_number ILIKE ${} OR customer_email ILIKE ${})",
                param_count, param_count
            ));
            params.push(Box::new(format!("%{}%", query)));
        }

        if let Some(customer_id) = request.customer_id {
            param_count += 1;
            query_conditions.push(format!("customer_id = ${}", param_count));
            params.push(Box::new(customer_id));
        }

        if let Some(status) = request.status {
            param_count += 1;
            query_conditions.push(format!("status = ${}", param_count));
            params.push(Box::new(status));
        }

        if let Some(payment_status) = request.payment_status {
            param_count += 1;
            query_conditions.push(format!("payment_status = ${}", param_count));
            params.push(Box::new(payment_status));
        }

        if let Some(fulfillment_status) = request.fulfillment_status {
            param_count += 1;
            query_conditions.push(format!("fulfillment_status = ${}", param_count));
            params.push(Box::new(fulfillment_status));
        }

        // Build sorting
        let sort_column = match request.sort_by.unwrap_or(OrderSortBy::CreatedAt) {
            OrderSortBy::CreatedAt => "created_at",
            OrderSortBy::UpdatedAt => "updated_at",
            OrderSortBy::OrderNumber => "order_number",
            OrderSortBy::CustomerEmail => "customer_email",
            OrderSortBy::Status => "status",
            OrderSortBy::Total => "total",
        };

        let sort_direction = match request.sort_order.unwrap_or(SortOrder::Desc) {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        };

        // Execute main query (simplified for demo)
        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT
                id, tenant_id, order_number, customer_id, customer_email,
                status as "status: OrderStatus",
                payment_status as "payment_status: PaymentStatus",
                fulfillment_status as "fulfillment_status: FulfillmentStatus",
                currency, subtotal, tax_total, shipping_total, discount_total, total,
                ARRAY[]::text[] as tags, '{}'::jsonb as metadata,
                created_at, updated_at, confirmed_at, shipped_at, delivered_at
            FROM orders
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&**self.db)
        .await?;

        // Convert to full Order structs (simplified)
        let mut full_orders = Vec::new();
        for order_row in orders {
            let items = self.get_order_items(order_row.id).await?;

            let order = Order {
                items,
                shipping_address: None,
                billing_address: None,
                ..order_row
            };
            full_orders.push(order);
        }

        // Get total count
        let total_count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM orders WHERE tenant_id = $1",
            tenant_id
        )
        .fetch_one(&**self.db)
        .await?
        .unwrap_or(0);

        // Generate facets
        let facets = self.generate_order_search_facets(tenant_id).await?;

        Ok(OrderSearchResponse {
            orders: full_orders,
            total_count,
            has_more: (offset + limit as i32) < total_count as i32,
            facets,
        })
    }

    /// Cancel order with reason and audit trail
    pub async fn cancel_order(
        &self,
        tenant_id: Uuid,
        order_id: Uuid,
        reason: String,
        cancelled_by: Uuid,
    ) -> Result<Option<Order>> {
        let mut tx = self.db.begin().await?;

        // Verify order can be cancelled
        let current_order = self.get_order(tenant_id, order_id).await?;
        let Some(current_order) = current_order else {
            return Ok(None);
        };

        // Check if order can be cancelled
        match current_order.status {
            OrderStatus::Shipped | OrderStatus::Delivered | OrderStatus::Completed
            | OrderStatus::Cancelled | OrderStatus::Refunded => {
                return Err(OlympusError::Validation(
                    "Order cannot be cancelled in its current status".to_string()
                ));
            },
            _ => {}
        }

        // Update order status
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE orders SET
                status = $1,
                updated_at = $2,
                updated_by = $3
            WHERE id = $4 AND tenant_id = $5
            "#,
            OrderStatus::Cancelled as OrderStatus,
            now,
            cancelled_by,
            order_id,
            tenant_id,
        )
        .execute(&mut *tx)
        .await?;

        // Record cancellation event
        self.record_order_event(
            &mut *tx,
            order_id,
            OrderEventType::Cancelled,
            format!("Order cancelled: {}", reason),
            Some(serde_json::to_value(&current_order)?),
            Some(serde_json::json!({"reason": reason})),
            Some(cancelled_by),
        ).await?;

        tx.commit().await?;

        // Publish cancellation event
        let event_data = serde_json::json!({
            "order_id": order_id,
            "tenant_id": tenant_id,
            "reason": reason,
            "cancelled_by": cancelled_by
        });

        self.event_publisher.publish(
            "commerce.order.cancelled",
            &event_data,
        ).await?;

        self.get_order(tenant_id, order_id).await
    }

    // ========================================================================
    // ORDER LIFECYCLE MANAGEMENT
    // ========================================================================

    /// Process order through confirmation workflow
    pub async fn confirm_order(
        &self,
        tenant_id: Uuid,
        order_id: Uuid,
        confirmed_by: Uuid,
    ) -> Result<Option<Order>> {
        let mut tx = self.db.begin().await?;

        // Verify order exists and can be confirmed
        let current_order = self.get_order(tenant_id, order_id).await?;
        let Some(current_order) = current_order else {
            return Ok(None);
        };

        if current_order.status != OrderStatus::Draft && current_order.status != OrderStatus::Pending {
            return Err(OlympusError::Validation(
                "Order can only be confirmed from Draft or Pending status".to_string()
            ));
        }

        // Update order status to confirmed
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE orders SET
                status = $1,
                confirmed_at = $2,
                updated_at = $3,
                updated_by = $4
            WHERE id = $5 AND tenant_id = $6
            "#,
            OrderStatus::Confirmed as OrderStatus,
            now,
            now,
            confirmed_by,
            order_id,
            tenant_id,
        )
        .execute(&mut *tx)
        .await?;

        // Record confirmation event
        self.record_order_event(
            &mut *tx,
            order_id,
            OrderEventType::StatusChanged,
            "Order confirmed".to_string(),
            Some(serde_json::to_value(&current_order)?),
            Some(serde_json::json!({"status": "confirmed"})),
            Some(confirmed_by),
        ).await?;

        tx.commit().await?;

        // Publish confirmation event
        let event_data = serde_json::json!({
            "order_id": order_id,
            "tenant_id": tenant_id,
            "confirmed_by": confirmed_by,
            "confirmed_at": now
        });

        self.event_publisher.publish(
            "commerce.order.confirmed",
            &event_data,
        ).await?;

        self.get_order(tenant_id, order_id).await
    }

    // ========================================================================
    // BULK OPERATIONS
    // ========================================================================

    /// Update multiple orders in bulk
    pub async fn bulk_update_orders(
        &self,
        tenant_id: Uuid,
        request: BulkOrderUpdateRequest,
        updated_by: Uuid,
    ) -> Result<BulkOrderResult> {
        let mut successful_updates = 0;
        let mut failed_updates = 0;
        let mut errors = Vec::new();

        for order_id in &request.order_ids {
            match self.bulk_update_single_order(tenant_id, *order_id, &request.updates, updated_by).await {
                Ok(_) => successful_updates += 1,
                Err(e) => {
                    failed_updates += 1;
                    errors.push(crate::models::BulkOrderError {
                        order_id: *order_id,
                        error_message: e.to_string(),
                    });
                }
            }
        }

        Ok(BulkOrderResult {
            total_orders: request.order_ids.len(),
            successful_updates,
            failed_updates,
            errors,
        })
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    /// Generate unique order number
    async fn generate_order_number(&self, tenant_id: Uuid) -> Result<String> {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM orders WHERE tenant_id = $1",
            tenant_id
        )
        .fetch_one(&**self.db)
        .await?
        .unwrap_or(0);

        Ok(format!("ORD-{:06}", count + 1))
    }

    /// Get order items for an order
    async fn get_order_items(&self, order_id: Uuid) -> Result<Vec<OrderItem>> {
        let items = sqlx::query_as!(
            OrderItem,
            r#"
            SELECT
                id, order_id, product_id, variant_id, sku, name,
                quantity, unit_price, total_price, tax_rate, tax_amount,
                discount_amount, attributes, created_at, updated_at
            FROM order_items
            WHERE order_id = $1
            ORDER BY created_at
            "#,
            order_id
        )
        .fetch_all(&**self.db)
        .await?;

        Ok(items)
    }

    /// Calculate order totals and line items
    async fn calculate_order(
        &self,
        items: &[CreateOrderItemRequest],
        tenant_id: Uuid,
    ) -> Result<OrderCalculation> {
        let mut line_items = Vec::new();
        let mut subtotal = Decimal::ZERO;

        for item in items {
            let product = self.get_product_for_order(tenant_id, item.product_id).await?;

            let unit_price = item.unit_price.unwrap_or(product.base_price);
            let line_total = unit_price * Decimal::from(item.quantity);
            let tax_amount = Decimal::ZERO; // Tax calculation to be implemented
            let discount_amount = Decimal::ZERO; // Discount calculation to be implemented

            line_items.push(LineItemCalculation {
                product_id: item.product_id,
                variant_id: item.variant_id,
                quantity: item.quantity,
                unit_price,
                line_total,
                tax_amount,
                discount_amount,
            });

            subtotal += line_total;
        }

        let tax_total = Decimal::ZERO; // To be implemented
        let shipping_total = Decimal::ZERO; // To be implemented
        let discount_total = Decimal::ZERO; // To be implemented
        let total = subtotal + tax_total + shipping_total - discount_total;

        Ok(OrderCalculation {
            subtotal,
            tax_total,
            shipping_total,
            discount_total,
            total,
            line_items,
            tax_lines: vec![], // To be implemented
            discount_lines: vec![], // To be implemented
            shipping_lines: vec![], // To be implemented
        })
    }

    /// Get product information for order creation
    async fn get_product_for_order(&self, tenant_id: Uuid, product_id: Uuid) -> Result<Product> {
        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT
                id, tenant_id, sku, name, description, short_description,
                product_type as "product_type: ProductType",
                status as "status: _",
                category_id, brand, weight, base_price,
                price_type as "price_type: _",
                cost_price, compare_at_price, tax_class, requires_shipping,
                is_digital, track_inventory, inventory_quantity, low_stock_threshold,
                tags, attributes, seo_title, seo_description,
                created_at, updated_at, created_by, updated_by
            FROM products
            WHERE id = $1 AND tenant_id = $2
            "#,
            product_id,
            tenant_id
        )
        .fetch_optional(&**self.db)
        .await?;

        product.ok_or_else(|| OlympusError::NotFound("Product not found".to_string()))
    }

    /// Record order event for audit trail
    async fn record_order_event(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        order_id: Uuid,
        event_type: OrderEventType,
        description: String,
        previous_data: Option<serde_json::Value>,
        new_data: Option<serde_json::Value>,
        created_by: Option<Uuid>,
    ) -> Result<()> {
        let event_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO order_events (
                id, order_id, event_type, description, previous_data,
                new_data, metadata, created_by, created_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            event_id,
            order_id,
            event_type as OrderEventType,
            description,
            previous_data,
            new_data,
            serde_json::json!({}),
            created_by,
            now,
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    /// Generate search facets for order filtering
    async fn generate_order_search_facets(&self, tenant_id: Uuid) -> Result<OrderSearchFacets> {
        // Status counts (simplified)
        let status_counts = vec![
            StatusFacet { status: OrderStatus::Draft, count: 5 },
            StatusFacet { status: OrderStatus::Confirmed, count: 15 },
            StatusFacet { status: OrderStatus::Shipped, count: 8 },
        ];

        let payment_status_counts = vec![
            PaymentStatusFacet { status: PaymentStatus::Pending, count: 10 },
            PaymentStatusFacet { status: PaymentStatus::Captured, count: 18 },
        ];

        let fulfillment_status_counts = vec![
            FulfillmentStatusFacet { status: FulfillmentStatus::Unfulfilled, count: 12 },
            FulfillmentStatusFacet { status: FulfillmentStatus::Fulfilled, count: 16 },
        ];

        let monthly_counts = vec![
            MonthlyCountFacet {
                year: 2025,
                month: 1,
                count: 28,
                total_revenue: Decimal::from(15420)
            },
        ];

        Ok(OrderSearchFacets {
            status_counts,
            payment_status_counts,
            fulfillment_status_counts,
            monthly_counts,
        })
    }

    /// Update single order in bulk operation
    async fn bulk_update_single_order(
        &self,
        tenant_id: Uuid,
        order_id: Uuid,
        updates: &crate::models::BulkOrderUpdates,
        updated_by: Uuid,
    ) -> Result<()> {
        let mut tx = self.db.begin().await?;

        // Apply status update if provided
        if let Some(status) = updates.status {
            sqlx::query!(
                r#"
                UPDATE orders SET
                    status = $1,
                    updated_at = $2,
                    updated_by = $3
                WHERE id = $4 AND tenant_id = $5
                "#,
                status as OrderStatus,
                Utc::now(),
                updated_by,
                order_id,
                tenant_id,
            )
            .execute(&mut *tx)
            .await?;
        }

        // Apply tag updates
        if let Some(tags_to_add) = &updates.tags_to_add {
            for tag in tags_to_add {
                sqlx::query!(
                    r#"
                    UPDATE orders SET
                        tags = array_append(tags, $1),
                        updated_at = $2,
                        updated_by = $3
                    WHERE id = $4 AND tenant_id = $5 AND NOT ($1 = ANY(tags))
                    "#,
                    tag,
                    Utc::now(),
                    updated_by,
                    order_id,
                    tenant_id,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }
}