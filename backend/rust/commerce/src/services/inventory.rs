// ============================================================================
// OLYMPUS CLOUD - INVENTORY SERVICE
// ============================================================================
// Module: commerce/src/services/inventory.rs
// Description: Inventory tracking and management business logic
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{
    InventoryAdjustment, InventoryAdjustmentType, InventoryItem, Product,
    ProductStatus, ProductVariant
};
use olympus_shared::{
    error::{ApiError, ApiResult},
    events::{DomainEvent, EventPublisher, InventoryAdjustedEvent, LowStockAlertEvent,
             StockReservedEvent, StockReleasedEvent, StockReservationItem, commerce_events},
};

// ============================================================================
// INVENTORY SERVICE MODELS
// ============================================================================

#[derive(Debug, Clone)]
pub struct StockLevel {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub available: i32,
    pub reserved: i32,
    pub on_hand: i32,
    pub low_stock_threshold: Option<i32>,
    pub reorder_point: Option<i32>,
    pub reorder_quantity: Option<i32>,
    pub cost_per_unit: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct StockMovement {
    pub id: Uuid,
    pub inventory_item_id: Uuid,
    pub adjustment_type: InventoryAdjustmentType,
    pub quantity_change: i32,
    pub reason: Option<String>,
    pub reference_id: Option<Uuid>,
    pub cost_impact: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone)]
pub struct LowStockAlert {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub sku: String,
    pub product_name: String,
    pub current_stock: i32,
    pub low_stock_threshold: i32,
    pub reorder_point: Option<i32>,
    pub reorder_quantity: Option<i32>,
    pub last_sale_date: Option<DateTime<Utc>>,
    pub days_out_of_stock: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct InventoryValuation {
    pub total_items: i64,
    pub total_quantity: i64,
    pub total_value_cost: Decimal,
    pub total_value_retail: Decimal,
    pub by_location: HashMap<Uuid, LocationValuation>,
    pub by_category: HashMap<Uuid, CategoryValuation>,
    pub low_stock_items: i64,
    pub out_of_stock_items: i64,
}

#[derive(Debug, Clone)]
pub struct LocationValuation {
    pub location_id: Uuid,
    pub quantity: i64,
    pub value_cost: Decimal,
    pub value_retail: Decimal,
    pub item_count: i64,
}

#[derive(Debug, Clone)]
pub struct CategoryValuation {
    pub category_id: Uuid,
    pub quantity: i64,
    pub value_cost: Decimal,
    pub value_retail: Decimal,
    pub item_count: i64,
}

#[derive(Debug, Clone)]
pub struct StockTransfer {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub from_location_id: Uuid,
    pub to_location_id: Uuid,
    pub status: TransferStatus,
    pub items: Vec<TransferItem>,
    pub notes: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub received_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct TransferItem {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity_requested: i32,
    pub quantity_shipped: Option<i32>,
    pub quantity_received: Option<i32>,
    pub cost_per_unit: Option<Decimal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    Draft,
    Pending,
    Shipped,
    Received,
    Cancelled,
}

// Request/Response Models
#[derive(Debug, Clone)]
pub struct StockAdjustmentRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub adjustment_type: InventoryAdjustmentType,
    pub quantity_change: i32,
    pub reason: String,
    pub reference_id: Option<Uuid>,
    pub cost_per_unit: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct StockReservationRequest {
    pub items: Vec<ReservationItem>,
    pub reference_id: Uuid, // Usually order_id
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ReservationItem {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub quantity: i32,
}

#[derive(Debug, Clone)]
pub struct StockAllocationResult {
    pub success: bool,
    pub allocated_items: Vec<AllocatedItem>,
    pub insufficient_stock: Vec<InsufficientStockItem>,
}

#[derive(Debug, Clone)]
pub struct AllocatedItem {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub quantity_allocated: i32,
}

#[derive(Debug, Clone)]
pub struct InsufficientStockItem {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub requested_quantity: i32,
    pub available_quantity: i32,
}

// ============================================================================
// INVENTORY SERVICE IMPLEMENTATION
// ============================================================================

#[derive(Clone)]
pub struct InventoryService {
    pool: PgPool,
    event_publisher: Arc<EventPublisher>,
}

impl InventoryService {
    pub fn new(pool: PgPool, event_publisher: Arc<EventPublisher>) -> Self {
        Self {
            pool,
            event_publisher,
        }
    }

    // ========================================================================
    // STOCK LEVEL MANAGEMENT
    // ========================================================================

    /// Get current stock level for a product/variant at a location
    pub async fn get_stock_level(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        variant_id: Option<Uuid>,
        location_id: Option<Uuid>,
    ) -> ApiResult<Option<StockLevel>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT product_id, variant_id, location_id, quantity_available,
             quantity_reserved, quantity_on_hand, low_stock_threshold,
             cost_per_unit FROM commerce.inventory_items WHERE tenant_id = "
        );
        query.push_bind(tenant_id);
        query.push(" AND product_id = ");
        query.push_bind(product_id);

        if let Some(vid) = variant_id {
            query.push(" AND variant_id = ");
            query.push_bind(vid);
        } else {
            query.push(" AND variant_id IS NULL");
        }

        if let Some(lid) = location_id {
            query.push(" AND location_id = ");
            query.push_bind(lid);
        } else {
            query.push(" AND location_id IS NULL");
        }

        let row = query.build().fetch_optional(&self.pool).await?;

        if let Some(row) = row {
            Ok(Some(StockLevel {
                product_id: row.get("product_id"),
                variant_id: row.get("variant_id"),
                location_id: row.get("location_id"),
                available: row.get("quantity_available"),
                reserved: row.get("quantity_reserved"),
                on_hand: row.get("quantity_on_hand"),
                low_stock_threshold: row.get("low_stock_threshold"),
                reorder_point: None, // TODO: Add to schema
                reorder_quantity: None, // TODO: Add to schema
                cost_per_unit: row.get("cost_per_unit"),
            }))
        } else {
            Ok(None)
        }
    }

    /// Get stock levels for multiple products
    pub async fn get_stock_levels_bulk(
        &self,
        tenant_id: Uuid,
        product_ids: &[Uuid],
        location_id: Option<Uuid>,
    ) -> ApiResult<Vec<StockLevel>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT product_id, variant_id, location_id, quantity_available,
             quantity_reserved, quantity_on_hand, low_stock_threshold,
             cost_per_unit FROM commerce.inventory_items WHERE tenant_id = "
        );
        query.push_bind(tenant_id);
        query.push(" AND product_id = ANY(");
        query.push_bind(product_ids);
        query.push(")");

        if let Some(lid) = location_id {
            query.push(" AND location_id = ");
            query.push_bind(lid);
        }

        let rows = query.build().fetch_all(&self.pool).await?;
        let mut stock_levels = Vec::new();

        for row in rows {
            stock_levels.push(StockLevel {
                product_id: row.get("product_id"),
                variant_id: row.get("variant_id"),
                location_id: row.get("location_id"),
                available: row.get("quantity_available"),
                reserved: row.get("quantity_reserved"),
                on_hand: row.get("quantity_on_hand"),
                low_stock_threshold: row.get("low_stock_threshold"),
                reorder_point: None,
                reorder_quantity: None,
                cost_per_unit: row.get("cost_per_unit"),
            });
        }

        Ok(stock_levels)
    }

    /// Update stock level with adjustment
    pub async fn adjust_stock(
        &self,
        tenant_id: Uuid,
        request: StockAdjustmentRequest,
        adjusted_by: Uuid,
    ) -> ApiResult<StockLevel> {
        let mut tx = self.pool.begin().await?;

        // Get or create inventory item
        let inventory_item = self.get_or_create_inventory_item(
            &mut tx,
            tenant_id,
            request.product_id,
            request.variant_id,
            request.location_id,
        ).await?;

        // Calculate new quantities
        let new_quantity_on_hand = match request.adjustment_type {
            InventoryAdjustmentType::Increase | InventoryAdjustmentType::Return => {
                inventory_item.quantity_on_hand + request.quantity_change
            }
            InventoryAdjustmentType::Decrease | InventoryAdjustmentType::Sale |
            InventoryAdjustmentType::Damage | InventoryAdjustmentType::Loss => {
                inventory_item.quantity_on_hand - request.quantity_change.abs()
            }
            InventoryAdjustmentType::Recount => request.quantity_change,
            InventoryAdjustmentType::Transfer => {
                // Transfers are handled separately
                inventory_item.quantity_on_hand - request.quantity_change.abs()
            }
        };

        // Ensure we don't go below zero for available quantity
        let new_quantity_available = (new_quantity_on_hand - inventory_item.quantity_reserved).max(0);

        // Update inventory item
        sqlx::query!(
            "UPDATE commerce.inventory_items
             SET quantity_on_hand = $1, quantity_available = $2, updated_at = NOW()
             WHERE id = $3",
            new_quantity_on_hand,
            new_quantity_available,
            inventory_item.id
        )
        .execute(&mut *tx)
        .await?;

        // Record the adjustment
        let adjustment_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO commerce.inventory_adjustments
             (id, tenant_id, inventory_item_id, adjustment_type, quantity_change, reason, reference_id, adjusted_by, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())",
            adjustment_id,
            tenant_id,
            inventory_item.id,
            request.adjustment_type as InventoryAdjustmentType,
            request.quantity_change,
            request.reason,
            request.reference_id,
            adjusted_by
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Publish inventory event
        let event_data = InventoryAdjustedEvent {
            product_id: request.product_id,
            variant_id: request.variant_id,
            location_id: request.location_id,
            tenant_id,
            old_quantity: inventory_item.quantity_on_hand,
            new_quantity: new_quantity_on_hand,
            adjustment_reason: match request.adjustment_type {
                InventoryAdjustmentType::Sale => crate::models::InventoryAdjustmentReason::Sale,
                InventoryAdjustmentType::Return => crate::models::InventoryAdjustmentReason::Return,
                InventoryAdjustmentType::Damage => crate::models::InventoryAdjustmentReason::Damage,
                InventoryAdjustmentType::Loss => crate::models::InventoryAdjustmentReason::Theft,
                InventoryAdjustmentType::Transfer => crate::models::InventoryAdjustmentReason::Transfer,
                InventoryAdjustmentType::Recount => crate::models::InventoryAdjustmentReason::Recount,
                _ => crate::models::InventoryAdjustmentReason::Other,
            },
            adjusted_by,
            reference_id: request.reference_id,
        };

        let domain_event = DomainEvent::builder(
            commerce_events::INVENTORY_ADJUSTED.to_string(),
            request.product_id,
            "Product".to_string(),
            tenant_id,
        )
        .user_id(adjusted_by)
        .data(event_data)?
        .build();

        self.event_publisher.publish(&domain_event).await?;

        // Check for low stock and publish alert if needed
        if let Some(threshold) = inventory_item.low_stock_threshold {
            if new_quantity_available <= threshold && inventory_item.quantity_available > threshold {
                // Get product name for alert
                let product_name = sqlx::query!("SELECT name FROM commerce.products WHERE id = $1", request.product_id)
                    .fetch_one(&self.pool)
                    .await?
                    .name;

                let alert_event_data = LowStockAlertEvent {
                    product_id: request.product_id,
                    variant_id: request.variant_id,
                    location_id: request.location_id,
                    tenant_id,
                    current_stock: new_quantity_available,
                    threshold,
                    sku: inventory_item.sku.clone(),
                    product_name,
                };

                let domain_event = DomainEvent::builder(
                    commerce_events::LOW_STOCK_ALERT.to_string(),
                    request.product_id,
                    "Product".to_string(),
                    tenant_id,
                )
                .user_id(adjusted_by)
                .data(alert_event_data)?
                .build();

                self.event_publisher.publish(&domain_event).await?;
            }
        }

        Ok(StockLevel {
            product_id: request.product_id,
            variant_id: request.variant_id,
            location_id: request.location_id,
            available: new_quantity_available,
            reserved: inventory_item.quantity_reserved,
            on_hand: new_quantity_on_hand,
            low_stock_threshold: inventory_item.low_stock_threshold,
            reorder_point: None,
            reorder_quantity: None,
            cost_per_unit: request.cost_per_unit.or(inventory_item.cost_per_unit),
        })
    }

    // ========================================================================
    // STOCK RESERVATION MANAGEMENT
    // ========================================================================

    /// Reserve stock for an order or other purpose
    pub async fn reserve_stock(
        &self,
        tenant_id: Uuid,
        request: StockReservationRequest,
        reserved_by: Uuid,
    ) -> ApiResult<StockAllocationResult> {
        let mut tx = self.pool.begin().await?;
        let mut allocated_items = Vec::new();
        let mut insufficient_stock = Vec::new();

        for item in &request.items {
            // Check available stock
            let stock_level = self.get_stock_level(
                tenant_id,
                item.product_id,
                item.variant_id,
                item.location_id,
            ).await?;

            if let Some(stock) = stock_level {
                if stock.available >= item.quantity {
                    // Reserve the stock
                    sqlx::query!(
                        "UPDATE commerce.inventory_items
                         SET quantity_reserved = quantity_reserved + $1,
                             quantity_available = quantity_available - $1,
                             updated_at = NOW()
                         WHERE tenant_id = $2 AND product_id = $3
                         AND ($4::UUID IS NULL OR variant_id = $4)
                         AND ($5::UUID IS NULL OR location_id = $5)",
                        item.quantity,
                        tenant_id,
                        item.product_id,
                        item.variant_id,
                        item.location_id
                    )
                    .execute(&mut *tx)
                    .await?;

                    allocated_items.push(AllocatedItem {
                        product_id: item.product_id,
                        variant_id: item.variant_id,
                        location_id: item.location_id,
                        quantity_allocated: item.quantity,
                    });

                    // Record reservation adjustment
                    let adjustment_id = Uuid::new_v4();
                    let inventory_item_id = self.get_inventory_item_id(
                        &mut tx,
                        tenant_id,
                        item.product_id,
                        item.variant_id,
                        item.location_id,
                    ).await?;

                    sqlx::query!(
                        "INSERT INTO commerce.inventory_adjustments
                         (id, tenant_id, inventory_item_id, adjustment_type, quantity_change, reason, reference_id, adjusted_by, created_at)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())",
                        adjustment_id,
                        tenant_id,
                        inventory_item_id,
                        InventoryAdjustmentType::Sale as InventoryAdjustmentType,
                        -item.quantity,
                        Some("Stock reservation".to_string()),
                        Some(request.reference_id),
                        reserved_by
                    )
                    .execute(&mut *tx)
                    .await?;
                } else {
                    insufficient_stock.push(InsufficientStockItem {
                        product_id: item.product_id,
                        variant_id: item.variant_id,
                        location_id: item.location_id,
                        requested_quantity: item.quantity,
                        available_quantity: stock.available,
                    });
                }
            } else {
                insufficient_stock.push(InsufficientStockItem {
                    product_id: item.product_id,
                    variant_id: item.variant_id,
                    location_id: item.location_id,
                    requested_quantity: item.quantity,
                    available_quantity: 0,
                });
            }
        }

        let success = insufficient_stock.is_empty();

        if success {
            tx.commit().await?;

            // Publish stock reserved event
            let reserved_items: Vec<StockReservationItem> = allocated_items.iter().map(|item| {
                StockReservationItem {
                    product_id: item.product_id,
                    variant_id: item.variant_id,
                    location_id: item.location_id,
                    quantity_allocated: item.quantity_allocated,
                }
            }).collect();

            let event_data = StockReservedEvent {
                tenant_id,
                reference_id: request.reference_id,
                items: reserved_items,
                reserved_by,
            };

            let domain_event = DomainEvent::builder(
                commerce_events::STOCK_RESERVED.to_string(),
                request.reference_id,
                "Order".to_string(),
                tenant_id,
            )
            .user_id(reserved_by)
            .data(event_data)?
            .build();

            self.event_publisher.publish(&domain_event).await?;
        } else {
            tx.rollback().await?;
        }

        Ok(StockAllocationResult {
            success,
            allocated_items,
            insufficient_stock,
        })
    }

    /// Release reserved stock
    pub async fn release_reservation(
        &self,
        tenant_id: Uuid,
        reference_id: Uuid,
        released_by: Uuid,
    ) -> ApiResult<()> {
        let mut tx = self.pool.begin().await?;

        // Find all reservations for this reference
        let reservations = sqlx::query!(
            "SELECT ia.inventory_item_id, ia.quantity_change
             FROM commerce.inventory_adjustments ia
             JOIN commerce.inventory_items ii ON ia.inventory_item_id = ii.id
             WHERE ia.tenant_id = $1 AND ia.reference_id = $2
             AND ia.adjustment_type = 'sale' AND ia.quantity_change < 0",
            tenant_id,
            reference_id
        )
        .fetch_all(&mut *tx)
        .await?;

        for reservation in reservations {
            let quantity_to_release = reservation.quantity_change.abs();

            // Release the reserved stock
            sqlx::query!(
                "UPDATE commerce.inventory_items
                 SET quantity_reserved = quantity_reserved - $1,
                     quantity_available = quantity_available + $1,
                     updated_at = NOW()
                 WHERE id = $2",
                quantity_to_release,
                reservation.inventory_item_id
            )
            .execute(&mut *tx)
            .await?;

            // Record release adjustment
            let adjustment_id = Uuid::new_v4();
            sqlx::query!(
                "INSERT INTO commerce.inventory_adjustments
                 (id, tenant_id, inventory_item_id, adjustment_type, quantity_change, reason, reference_id, adjusted_by, created_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())",
                adjustment_id,
                tenant_id,
                reservation.inventory_item_id,
                InventoryAdjustmentType::Return as InventoryAdjustmentType,
                quantity_to_release,
                Some("Stock reservation released".to_string()),
                Some(reference_id),
                released_by
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        // Publish stock released event
        let event_data = StockReleasedEvent {
            tenant_id,
            reference_id,
            released_by,
        };

        let domain_event = DomainEvent::builder(
            commerce_events::STOCK_RELEASED.to_string(),
            reference_id,
            "Order".to_string(),
            tenant_id,
        )
        .user_id(released_by)
        .data(event_data)?
        .build();

        self.event_publisher.publish(&domain_event).await?;

        Ok(())
    }

    // ========================================================================
    // LOW STOCK ALERTS
    // ========================================================================

    /// Get all items with low stock alerts
    pub async fn get_low_stock_alerts(
        &self,
        tenant_id: Uuid,
        location_id: Option<Uuid>,
    ) -> ApiResult<Vec<LowStockAlert>> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT ii.product_id, ii.variant_id, ii.location_id,
             p.sku, p.name as product_name, ii.quantity_available,
             ii.low_stock_threshold, ii.last_counted_at
             FROM commerce.inventory_items ii
             JOIN commerce.products p ON ii.product_id = p.id
             WHERE ii.tenant_id = "
        );
        query.push_bind(tenant_id);
        query.push(" AND ii.low_stock_threshold IS NOT NULL");
        query.push(" AND ii.quantity_available <= ii.low_stock_threshold");

        if let Some(lid) = location_id {
            query.push(" AND ii.location_id = ");
            query.push_bind(lid);
        }

        query.push(" ORDER BY ii.quantity_available ASC, p.name ASC");

        let rows = query.build().fetch_all(&self.pool).await?;
        let mut alerts = Vec::new();

        for row in rows {
            alerts.push(LowStockAlert {
                product_id: row.get("product_id"),
                variant_id: row.get("variant_id"),
                location_id: row.get("location_id"),
                sku: row.get("sku"),
                product_name: row.get("product_name"),
                current_stock: row.get("quantity_available"),
                low_stock_threshold: row.get("low_stock_threshold"),
                reorder_point: None, // TODO: Add to schema
                reorder_quantity: None, // TODO: Add to schema
                last_sale_date: None, // TODO: Calculate from order history
                days_out_of_stock: None, // TODO: Calculate
            });
        }

        Ok(alerts)
    }

    /// Update low stock threshold for a product
    pub async fn update_low_stock_threshold(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        variant_id: Option<Uuid>,
        location_id: Option<Uuid>,
        threshold: Option<i32>,
        updated_by: Uuid,
    ) -> ApiResult<()> {
        sqlx::query!(
            "UPDATE commerce.inventory_items
             SET low_stock_threshold = $1, updated_at = NOW()
             WHERE tenant_id = $2 AND product_id = $3
             AND ($4::UUID IS NULL OR variant_id = $4)
             AND ($5::UUID IS NULL OR location_id = $5)",
            threshold,
            tenant_id,
            product_id,
            variant_id,
            location_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ========================================================================
    // INVENTORY VALUATION
    // ========================================================================

    /// Calculate inventory valuation
    pub async fn calculate_inventory_valuation(
        &self,
        tenant_id: Uuid,
        location_id: Option<Uuid>,
    ) -> ApiResult<InventoryValuation> {
        let mut query = sqlx::QueryBuilder::new(
            "SELECT ii.location_id, p.category_id, ii.quantity_on_hand,
             ii.cost_per_unit, p.base_price,
             COUNT(*) as item_count,
             SUM(ii.quantity_on_hand) as total_quantity,
             SUM(ii.quantity_on_hand * COALESCE(ii.cost_per_unit, 0)) as total_cost_value,
             SUM(ii.quantity_on_hand * p.base_price) as total_retail_value
             FROM commerce.inventory_items ii
             JOIN commerce.products p ON ii.product_id = p.id
             WHERE ii.tenant_id = "
        );
        query.push_bind(tenant_id);

        if let Some(lid) = location_id {
            query.push(" AND ii.location_id = ");
            query.push_bind(lid);
        }

        query.push(" GROUP BY ii.location_id, p.category_id");

        let rows = query.build().fetch_all(&self.pool).await?;

        let mut by_location = HashMap::new();
        let mut by_category = HashMap::new();
        let mut total_quantity = 0i64;
        let mut total_value_cost = Decimal::ZERO;
        let mut total_value_retail = Decimal::ZERO;
        let mut total_items = 0i64;

        for row in rows {
            let location_id: Option<Uuid> = row.get("location_id");
            let category_id: Option<Uuid> = row.get("category_id");
            let quantity: i64 = row.get("total_quantity");
            let cost_value: Decimal = row.get("total_cost_value");
            let retail_value: Decimal = row.get("total_retail_value");
            let item_count: i64 = row.get("item_count");

            total_quantity += quantity;
            total_value_cost += cost_value;
            total_value_retail += retail_value;
            total_items += item_count;

            // Group by location
            if let Some(loc_id) = location_id {
                let location_val = by_location.entry(loc_id).or_insert(LocationValuation {
                    location_id: loc_id,
                    quantity: 0,
                    value_cost: Decimal::ZERO,
                    value_retail: Decimal::ZERO,
                    item_count: 0,
                });
                location_val.quantity += quantity;
                location_val.value_cost += cost_value;
                location_val.value_retail += retail_value;
                location_val.item_count += item_count;
            }

            // Group by category
            if let Some(cat_id) = category_id {
                let category_val = by_category.entry(cat_id).or_insert(CategoryValuation {
                    category_id: cat_id,
                    quantity: 0,
                    value_cost: Decimal::ZERO,
                    value_retail: Decimal::ZERO,
                    item_count: 0,
                });
                category_val.quantity += quantity;
                category_val.value_cost += cost_value;
                category_val.value_retail += retail_value;
                category_val.item_count += item_count;
            }
        }

        // Count low stock and out of stock items
        let low_stock_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM commerce.inventory_items
             WHERE tenant_id = $1 AND low_stock_threshold IS NOT NULL
             AND quantity_available <= low_stock_threshold",
            tenant_id
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        let out_of_stock_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM commerce.inventory_items
             WHERE tenant_id = $1 AND quantity_available = 0",
            tenant_id
        )
        .fetch_one(&self.pool)
        .await?
        .count
        .unwrap_or(0);

        Ok(InventoryValuation {
            total_items,
            total_quantity,
            total_value_cost,
            total_value_retail,
            by_location,
            by_category,
            low_stock_items: low_stock_count,
            out_of_stock_items: out_of_stock_count,
        })
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    async fn get_or_create_inventory_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        product_id: Uuid,
        variant_id: Option<Uuid>,
        location_id: Option<Uuid>,
    ) -> ApiResult<InventoryItem> {
        // Try to get existing item
        let existing = sqlx::query_as!(
            InventoryItem,
            "SELECT * FROM commerce.inventory_items
             WHERE tenant_id = $1 AND product_id = $2
             AND ($3::UUID IS NULL OR variant_id = $3)
             AND ($4::UUID IS NULL OR location_id = $4)",
            tenant_id,
            product_id,
            variant_id,
            location_id
        )
        .fetch_optional(&mut **tx)
        .await?;

        if let Some(item) = existing {
            Ok(item)
        } else {
            // Create new inventory item
            let item_id = Uuid::new_v4();

            // Get SKU from product or variant
            let sku = if let Some(v_id) = variant_id {
                sqlx::query!("SELECT sku FROM commerce.product_variants WHERE id = $1", v_id)
                    .fetch_optional(&mut **tx)
                    .await?
                    .and_then(|row| row.sku)
                    .unwrap_or_else(|| format!("VAR-{}", v_id))
            } else {
                sqlx::query!("SELECT sku FROM commerce.products WHERE id = $1", product_id)
                    .fetch_one(&mut **tx)
                    .await?
                    .sku
            };

            sqlx::query!(
                "INSERT INTO commerce.inventory_items
                 (id, tenant_id, product_id, variant_id, location_id, sku,
                  quantity_available, quantity_reserved, quantity_on_hand, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, 0, 0, 0, NOW(), NOW())",
                item_id,
                tenant_id,
                product_id,
                variant_id,
                location_id,
                sku
            )
            .execute(&mut **tx)
            .await?;

            Ok(InventoryItem {
                id: item_id,
                tenant_id,
                product_id,
                variant_id,
                location_id,
                sku,
                quantity_available: 0,
                quantity_reserved: 0,
                quantity_on_hand: 0,
                low_stock_threshold: None,
                cost_per_unit: None,
                last_counted_at: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        }
    }

    async fn get_inventory_item_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        tenant_id: Uuid,
        product_id: Uuid,
        variant_id: Option<Uuid>,
        location_id: Option<Uuid>,
    ) -> ApiResult<Uuid> {
        let item = self.get_or_create_inventory_item(tx, tenant_id, product_id, variant_id, location_id).await?;
        Ok(item.id)
    }
}