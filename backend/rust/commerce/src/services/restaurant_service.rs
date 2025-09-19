// ============================================================================
// OLYMPUS CLOUD - RESTAURANT SERVICE
// ============================================================================
// Module: commerce/src/services/restaurant_service.rs
// Description: Restaurant-specific business logic for table and order management
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use crate::models::restaurant::*;
use olympus_shared::{Result, Error};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tracing::{info, warn, error};

#[derive(Clone)]
pub struct RestaurantService {
    db: PgPool,
}

impl RestaurantService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // ============================================================================
    // TABLE MANAGEMENT
    // ============================================================================

    /// Get all tables for a location
    pub async fn get_tables(&self, tenant_id: Uuid, location_id: Uuid) -> Result<Vec<RestaurantTable>> {
        let tables = sqlx::query_as!(
            RestaurantTable,
            r#"
            SELECT
                id, tenant_id, location_id, table_number, name,
                capacity, status as "status: TableStatus", section,
                position_x, position_y, current_order_id, server_id,
                last_cleaned_at, created_at, updated_at
            FROM commerce.restaurant_tables
            WHERE tenant_id = $1 AND location_id = $2
            ORDER BY table_number
            "#,
            tenant_id,
            location_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(tables)
    }

    /// Get a specific table by ID
    pub async fn get_table(&self, tenant_id: Uuid, table_id: Uuid) -> Result<RestaurantTable> {
        let table = sqlx::query_as!(
            RestaurantTable,
            r#"
            SELECT
                id, tenant_id, location_id, table_number, name,
                capacity, status as "status: TableStatus", section,
                position_x, position_y, current_order_id, server_id,
                last_cleaned_at, created_at, updated_at
            FROM commerce.restaurant_tables
            WHERE tenant_id = $1 AND id = $2
            "#,
            tenant_id,
            table_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(table)
    }

    /// Update table status
    pub async fn update_table_status(
        &self,
        tenant_id: Uuid,
        table_id: Uuid,
        request: UpdateTableStatusRequest
    ) -> Result<RestaurantTable> {
        let mut tx = self.db.begin().await?;

        // Update table status
        let updated_table = sqlx::query_as!(
            RestaurantTable,
            r#"
            UPDATE commerce.restaurant_tables
            SET
                status = $3,
                server_id = $4,
                last_cleaned_at = CASE WHEN $3 = 'Cleaning' THEN NOW() ELSE last_cleaned_at END,
                updated_at = NOW()
            WHERE tenant_id = $1 AND id = $2
            RETURNING
                id, tenant_id, location_id, table_number, name,
                capacity, status as "status: TableStatus", section,
                position_x, position_y, current_order_id, server_id,
                last_cleaned_at, created_at, updated_at
            "#,
            tenant_id,
            table_id,
            request.status as TableStatus,
            request.server_id
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        info!(
            "Table {} status updated to {:?} by server {:?}",
            table_id, request.status, request.server_id
        );

        Ok(updated_table)
    }

    /// Get table analytics for dashboard
    pub async fn get_table_analytics(&self, tenant_id: Uuid, location_id: Uuid) -> Result<Vec<TableAnalytics>> {
        let analytics = sqlx::query!(
            r#"
            SELECT
                t.id as table_id,
                t.table_number,
                COALESCE(COUNT(o.id), 0)::int as turns_today,
                COALESCE(AVG(EXTRACT(EPOCH FROM (o.check_closed_at - o.seat_time)) / 60), 0) as average_turn_time,
                COALESCE(SUM(o.total_amount), 0) as revenue_today,
                MAX(o.seat_time) as last_occupied_at,
                t.status as "status: TableStatus"
            FROM commerce.restaurant_tables t
            LEFT JOIN commerce.restaurant_orders o ON t.id = o.table_id
                AND o.check_closed_at IS NOT NULL
                AND DATE(o.check_closed_at) = CURRENT_DATE
            WHERE t.tenant_id = $1 AND t.location_id = $2
            GROUP BY t.id, t.table_number, t.status
            ORDER BY t.table_number
            "#,
            tenant_id,
            location_id
        )
        .fetch_all(&self.db)
        .await?;

        let result = analytics
            .into_iter()
            .map(|row| TableAnalytics {
                table_id: row.table_id,
                table_number: row.table_number,
                turns_today: row.turns_today,
                average_turn_time: row.average_turn_time.unwrap_or(0.0) as f64,
                revenue_today: row.revenue_today.unwrap_or_else(|| Decimal::new(0, 2)),
                last_occupied_at: row.last_occupied_at,
                current_status: row.status,
            })
            .collect();

        Ok(result)
    }

    // ============================================================================
    // ORDER MANAGEMENT
    // ============================================================================

    /// Create a new restaurant order
    pub async fn create_order(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        request: CreateRestaurantOrderRequest,
    ) -> Result<RestaurantOrder> {
        let mut tx = self.db.begin().await?;

        // Generate order number
        let order_number = self.generate_order_number(&mut tx, tenant_id).await?;

        // Calculate totals (simplified - would integrate with product pricing)
        let subtotal = Decimal::new(0, 2); // Would calculate from items
        let tax_rate = Decimal::new(875, 4); // 8.75%
        let tax_amount = subtotal * tax_rate;
        let total_amount = subtotal + tax_amount;

        // Create order
        let order = sqlx::query_as!(
            RestaurantOrder,
            r#"
            INSERT INTO commerce.restaurant_orders (
                tenant_id, location_id, order_number, table_id, server_id,
                customer_name, guest_count, order_type, status,
                subtotal, tax_amount, total_amount, payment_status, notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING
                id, tenant_id, location_id, order_number, table_id, server_id,
                customer_name, guest_count,
                order_type as "order_type: RestaurantOrderType",
                status as "status: RestaurantOrderStatus",
                subtotal, tax_amount, tip_amount, total_amount,
                payment_status as "payment_status: PaymentStatus",
                notes, created_at, updated_at, seat_time, order_time,
                kitchen_time, served_time, check_closed_at
            "#,
            tenant_id,
            location_id,
            order_number,
            request.table_id,
            request.server_id,
            request.customer_name,
            request.guest_count,
            request.order_type as RestaurantOrderType,
            RestaurantOrderStatus::Open as RestaurantOrderStatus,
            subtotal,
            tax_amount,
            total_amount,
            PaymentStatus::Pending as PaymentStatus,
            request.notes
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update table if specified
        if let Some(table_id) = request.table_id {
            sqlx::query!(
                r#"
                UPDATE commerce.restaurant_tables
                SET current_order_id = $1, status = 'Occupied', updated_at = NOW()
                WHERE tenant_id = $2 AND id = $3
                "#,
                order.id,
                tenant_id,
                table_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        info!("Created restaurant order {} for tenant {}", order.id, tenant_id);

        // Return order with empty items for now
        Ok(RestaurantOrder {
            items: vec![], // Would be populated separately
            ..order
        })
    }

    /// Get orders for a location
    pub async fn get_orders(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
        status_filter: Option<RestaurantOrderStatus>,
    ) -> Result<Vec<RestaurantOrder>> {
        let orders = if let Some(status) = status_filter {
            sqlx::query_as!(
                RestaurantOrder,
                r#"
                SELECT
                    id, tenant_id, location_id, order_number, table_id, server_id,
                    customer_name, guest_count,
                    order_type as "order_type: RestaurantOrderType",
                    status as "status: RestaurantOrderStatus",
                    subtotal, tax_amount, tip_amount, total_amount,
                    payment_status as "payment_status: PaymentStatus",
                    notes, created_at, updated_at, seat_time, order_time,
                    kitchen_time, served_time, check_closed_at
                FROM commerce.restaurant_orders
                WHERE tenant_id = $1 AND location_id = $2 AND status = $3
                ORDER BY created_at DESC
                "#,
                tenant_id,
                location_id,
                status as RestaurantOrderStatus
            )
            .fetch_all(&self.db)
            .await?
        } else {
            sqlx::query_as!(
                RestaurantOrder,
                r#"
                SELECT
                    id, tenant_id, location_id, order_number, table_id, server_id,
                    customer_name, guest_count,
                    order_type as "order_type: RestaurantOrderType",
                    status as "status: RestaurantOrderStatus",
                    subtotal, tax_amount, tip_amount, total_amount,
                    payment_status as "payment_status: PaymentStatus",
                    notes, created_at, updated_at, seat_time, order_time,
                    kitchen_time, served_time, check_closed_at
                FROM commerce.restaurant_orders
                WHERE tenant_id = $1 AND location_id = $2
                ORDER BY created_at DESC
                "#,
                tenant_id,
                location_id
            )
            .fetch_all(&self.db)
            .await?
        };

        // For now, return orders with empty items (would be loaded separately for performance)
        let orders_with_items = orders
            .into_iter()
            .map(|order| RestaurantOrder {
                items: vec![],
                ..order
            })
            .collect();

        Ok(orders_with_items)
    }

    /// Update order status
    pub async fn update_order_status(
        &self,
        tenant_id: Uuid,
        order_id: Uuid,
        new_status: RestaurantOrderStatus,
    ) -> Result<RestaurantOrder> {
        let mut tx = self.db.begin().await?;

        let updated_order = sqlx::query_as!(
            RestaurantOrder,
            r#"
            UPDATE commerce.restaurant_orders
            SET
                status = $3,
                kitchen_time = CASE WHEN $3 = 'Fired' THEN NOW() ELSE kitchen_time END,
                served_time = CASE WHEN $3 = 'Served' THEN NOW() ELSE served_time END,
                check_closed_at = CASE WHEN $3 = 'Closed' THEN NOW() ELSE check_closed_at END,
                updated_at = NOW()
            WHERE tenant_id = $1 AND id = $2
            RETURNING
                id, tenant_id, location_id, order_number, table_id, server_id,
                customer_name, guest_count,
                order_type as "order_type: RestaurantOrderType",
                status as "status: RestaurantOrderStatus",
                subtotal, tax_amount, tip_amount, total_amount,
                payment_status as "payment_status: PaymentStatus",
                notes, created_at, updated_at, seat_time, order_time,
                kitchen_time, served_time, check_closed_at
            "#,
            tenant_id,
            order_id,
            new_status as RestaurantOrderStatus
        )
        .fetch_one(&mut *tx)
        .await?;

        // If order is closed, update table status
        if new_status == RestaurantOrderStatus::Closed {
            if let Some(table_id) = updated_order.table_id {
                sqlx::query!(
                    r#"
                    UPDATE commerce.restaurant_tables
                    SET current_order_id = NULL, status = 'Cleaning', updated_at = NOW()
                    WHERE tenant_id = $1 AND id = $2
                    "#,
                    tenant_id,
                    table_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        info!("Order {} status updated to {:?}", order_id, new_status);

        Ok(RestaurantOrder {
            items: vec![], // Would be loaded separately
            ..updated_order
        })
    }

    // ============================================================================
    // KITCHEN DISPLAY SYSTEM
    // ============================================================================

    /// Get kitchen display items for active orders
    pub async fn get_kitchen_display_items(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<Vec<KitchenDisplayItem>> {
        let items = sqlx::query!(
            r#"
            SELECT
                o.id as order_id,
                o.order_number,
                t.table_number,
                oi.id as item_id,
                oi.name as item_name,
                oi.quantity,
                oi.special_instructions,
                oi.kitchen_status,
                o.created_at as ordered_at,
                oi.fired_at,
                COALESCE(oi.ready_at, o.created_at + INTERVAL '15 minutes') as estimated_ready_time
            FROM commerce.restaurant_orders o
            JOIN commerce.restaurant_order_items oi ON o.id = oi.order_id
            LEFT JOIN commerce.restaurant_tables t ON o.table_id = t.id
            WHERE o.tenant_id = $1
                AND o.location_id = $2
                AND o.status IN ('Fired', 'InProgress')
                AND oi.kitchen_status IN ('Pending', 'InPreparation')
            ORDER BY o.created_at ASC, oi.fired_at ASC
            "#,
            tenant_id,
            location_id
        )
        .fetch_all(&self.db)
        .await?;

        let display_items = items
            .into_iter()
            .map(|row| {
                let status = match row.kitchen_status.as_str() {
                    "Pending" => KitchenStatus::Pending,
                    "InPreparation" => KitchenStatus::InPreparation,
                    "Ready" => KitchenStatus::Ready,
                    "Served" => KitchenStatus::Served,
                    "Cancelled" => KitchenStatus::Cancelled,
                    _ => KitchenStatus::Pending,
                };

                KitchenDisplayItem {
                    order_id: row.order_id,
                    order_number: row.order_number,
                    table_number: row.table_number,
                    item_id: row.item_id,
                    item_name: row.item_name,
                    quantity: row.quantity,
                    modifiers: vec![], // Would be loaded from modifiers table
                    special_instructions: row.special_instructions,
                    status,
                    ordered_at: row.ordered_at,
                    fired_at: row.fired_at,
                    estimated_ready_time: row.estimated_ready_time,
                    priority: KitchenPriority::Normal, // Would be calculated based on timing
                }
            })
            .collect();

        Ok(display_items)
    }

    /// Update kitchen item status
    pub async fn update_kitchen_status(
        &self,
        tenant_id: Uuid,
        item_id: Uuid,
        request: UpdateKitchenStatusRequest,
    ) -> Result<()> {
        let status_str = match request.status {
            KitchenStatus::Pending => "Pending",
            KitchenStatus::InPreparation => "InPreparation",
            KitchenStatus::Ready => "Ready",
            KitchenStatus::Served => "Served",
            KitchenStatus::Cancelled => "Cancelled",
        };

        sqlx::query!(
            r#"
            UPDATE commerce.restaurant_order_items
            SET
                kitchen_status = $3,
                ready_at = CASE WHEN $3 = 'Ready' THEN NOW() ELSE ready_at END,
                served_at = CASE WHEN $3 = 'Served' THEN NOW() ELSE served_at END,
                updated_at = NOW()
            WHERE id = $1
                AND order_id IN (
                    SELECT id FROM commerce.restaurant_orders WHERE tenant_id = $2
                )
            "#,
            item_id,
            tenant_id,
            status_str
        )
        .execute(&self.db)
        .await?;

        info!("Kitchen item {} status updated to {:?}", item_id, request.status);

        Ok(())
    }

    // ============================================================================
    // DASHBOARD METRICS
    // ============================================================================

    /// Get real-time restaurant dashboard metrics
    pub async fn get_dashboard_metrics(
        &self,
        tenant_id: Uuid,
        location_id: Uuid,
    ) -> Result<RestaurantDashboard> {
        // Get table counts
        let table_stats = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as total_tables,
                COUNT(*) FILTER (WHERE status = 'Occupied') as occupied_tables,
                COUNT(*) FILTER (WHERE status = 'Available') as available_tables,
                COUNT(*) FILTER (WHERE status = 'Reserved') as reserved_tables
            FROM commerce.restaurant_tables
            WHERE tenant_id = $1 AND location_id = $2
            "#,
            tenant_id,
            location_id
        )
        .fetch_one(&self.db)
        .await?;

        // Get order stats
        let order_stats = sqlx::query!(
            r#"
            SELECT
                COUNT(*) FILTER (WHERE status IN ('Open', 'Fired', 'InProgress', 'Ready', 'Served')) as open_orders,
                COUNT(*) FILTER (WHERE DATE(created_at) = CURRENT_DATE) as today_covers,
                COALESCE(SUM(total_amount) FILTER (WHERE DATE(created_at) = CURRENT_DATE), 0) as today_revenue
            FROM commerce.restaurant_orders
            WHERE tenant_id = $1 AND location_id = $2
            "#,
            tenant_id,
            location_id
        )
        .fetch_one(&self.db)
        .await?;

        // Get kitchen queue
        let kitchen_queue = sqlx::query!(
            r#"
            SELECT COUNT(*) as queue_items
            FROM commerce.restaurant_order_items oi
            JOIN commerce.restaurant_orders o ON oi.order_id = o.id
            WHERE o.tenant_id = $1
                AND o.location_id = $2
                AND oi.kitchen_status IN ('Pending', 'InPreparation')
            "#,
            tenant_id,
            location_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(RestaurantDashboard {
            total_tables: table_stats.total_tables.unwrap_or(0) as i32,
            occupied_tables: table_stats.occupied_tables.unwrap_or(0) as i32,
            available_tables: table_stats.available_tables.unwrap_or(0) as i32,
            reserved_tables: table_stats.reserved_tables.unwrap_or(0) as i32,
            open_orders: order_stats.open_orders.unwrap_or(0) as i32,
            kitchen_queue_items: kitchen_queue.queue_items.unwrap_or(0) as i32,
            today_revenue: order_stats.today_revenue.unwrap_or_else(|| Decimal::new(0, 2)),
            today_covers: order_stats.today_covers.unwrap_or(0) as i32,
            average_table_turn_time: None, // Would calculate from historical data
            current_wait_time: None,       // Would calculate from current queue
            peak_hour_indicator: false,    // Would determine from time and historical patterns
        })
    }

    // ============================================================================
    // HELPER METHODS
    // ============================================================================

    /// Generate unique order number for the day
    async fn generate_order_number(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: Uuid,
    ) -> Result<String> {
        let today = Utc::now().format("%Y%m%d").to_string();

        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM commerce.restaurant_orders
            WHERE tenant_id = $1 AND DATE(created_at) = CURRENT_DATE
            "#,
            tenant_id
        )
        .fetch_one(&mut **tx)
        .await?;

        let order_number = format!("{}-{:04}", today, count.count.unwrap_or(0) + 1);
        Ok(order_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        // Test database setup would go here
        todo!("Implement test database setup")
    }

    #[tokio::test]
    async fn test_create_order() {
        // Test implementation would go here
        todo!("Implement order creation test")
    }

    #[tokio::test]
    async fn test_table_status_update() {
        // Test implementation would go here
        todo!("Implement table status update test")
    }
}