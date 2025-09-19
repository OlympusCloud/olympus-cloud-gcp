// ============================================================================
// OLYMPUS CLOUD - INVENTORY TRANSACTION MODELS
// ============================================================================
// Module: commerce/src/models/inventory_transactions.rs
// Description: Comprehensive inventory transaction and stock management models
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use validator::Validate;

/// Types of stock movements
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "stock_movement_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StockMovementType {
    Inbound,      // Receiving inventory
    Outbound,     // Shipping/selling inventory
    Transfer,     // Moving between locations
    Adjustment,   // Manual adjustments
    Reservation,  // Reserving stock for orders
    Release,      // Releasing reserved stock
    Allocation,   // Allocating stock to specific orders
    Deallocation, // Removing allocations
}

/// Transaction status for inventory operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "inventory_transaction_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryTransactionStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
    Failed,
    RolledBack,
}

/// Enhanced adjustment types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "adjustment_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AdjustmentType {
    Add,
    Remove,
    Set,
    Sale,
    Purchase,
    Return,
    Damage,
    Theft,
    Expired,
    TransferIn,
    TransferOut,
    Production,
    Consumption,
    CycleCount,
    Revaluation,
}

/// Inventory transaction for ACID compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryTransaction {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub transaction_number: String,
    pub transaction_type: StockMovementType,
    pub status: InventoryTransactionStatus,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub source_location_id: Option<Uuid>,
    pub destination_location_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub notes: Option<String>,

    // Transaction control
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub rollback_reason: Option<String>,

    // Audit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Individual items within an inventory transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryTransactionItem {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub inventory_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,

    // Quantities
    pub quantity: i32,
    pub quantity_processed: i32,
    pub unit_cost: Option<Decimal>,
    pub total_cost: Option<Decimal>,

    // Before/after for audit
    pub quantity_before: i32,
    pub reserved_before: i32,
    pub quantity_after: Option<i32>,
    pub reserved_after: Option<i32>,

    // Status
    pub status: InventoryTransactionStatus,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,

    pub created_at: DateTime<Utc>,
}

/// Stock reservations for order management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockReservation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inventory_id: Uuid,
    pub reference_type: String,
    pub reference_id: Uuid,
    pub quantity: i32,
    pub reserved_until: DateTime<Utc>,
    pub status: ReservationStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Reservation status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReservationStatus {
    Active,
    Expired,
    Released,
    Allocated,
}

/// Stock movement audit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inventory_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub movement_type: StockMovementType,

    // Movement details
    pub quantity_change: i32,
    pub quantity_before: i32,
    pub quantity_after: i32,
    pub reserved_change: i32,
    pub reserved_before: i32,
    pub reserved_after: i32,

    // Cost tracking
    pub unit_cost: Option<Decimal>,
    pub total_value_change: Option<Decimal>,
    pub running_value: Option<Decimal>,

    // Reference information
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reason: Option<String>,

    // Audit information
    pub performed_by: Option<Uuid>,
    pub performed_at: DateTime<Utc>,
    pub location_id: Option<Uuid>,

    // Additional metadata
    pub batch_number: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub lot_number: Option<String>,
    pub metadata: serde_json::Value,

    pub created_at: DateTime<Utc>,
}

/// Inventory valuation for cost tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryValuation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inventory_id: Uuid,
    pub valuation_method: ValuationMethod,

    // Cost information
    pub unit_cost: Decimal,
    pub total_quantity: i32,
    pub total_value: Decimal,
    pub average_cost: Option<Decimal>,

    // Validity period
    pub valued_at: DateTime<Utc>,
    pub valid_from: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,

    // Audit
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Valuation methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ValuationMethod {
    Fifo,     // First In, First Out
    Lifo,     // Last In, First Out
    Average,  // Weighted Average
    Specific, // Specific Identification
}

/// Inventory lots for batch/lot tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryLot {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inventory_id: Uuid,
    pub lot_number: String,
    pub batch_number: Option<String>,

    // Quantity tracking
    pub quantity_received: i32,
    pub quantity_available: i32,
    pub quantity_allocated: i32,

    // Cost and valuation
    pub unit_cost: Decimal,
    pub total_cost: Decimal,

    // Dates
    pub received_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub manufacture_date: Option<NaiveDate>,

    // Status
    pub status: LotStatus,

    // Supplier information
    pub supplier_id: Option<Uuid>,
    pub purchase_order_reference: Option<String>,

    // Quality control
    pub quality_status: QualityStatus,
    pub quality_notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lot status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LotStatus {
    Active,
    Expired,
    Quarantine,
    Disposed,
}

/// Quality status for lots
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QualityStatus {
    Pending,
    Approved,
    Rejected,
    OnHold,
}

/// Enhanced inventory model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedInventory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Uuid,
    pub quantity_on_hand: i32,
    pub quantity_reserved: i32,
    pub quantity_available: i32,
    pub reorder_point: Option<i32>,
    pub reorder_quantity: Option<i32>,
    pub last_counted_at: Option<DateTime<Utc>>,

    // Enhanced fields
    pub average_cost: Decimal,
    pub total_value: Decimal,
    pub last_movement_at: Option<DateTime<Utc>>,
    pub minimum_stock_level: i32,
    pub maximum_stock_level: Option<i32>,
    pub optimal_stock_level: Option<i32>,
    pub safety_stock: i32,
    pub lead_time_days: i32,
    pub abc_classification: String,
    pub velocity_score: Decimal,
    pub last_sale_at: Option<DateTime<Utc>>,
    pub is_serialized: bool,
    pub is_lot_tracked: bool,
    pub is_expired_tracking: bool,

    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to start an inventory transaction
#[derive(Debug, Validate, Deserialize)]
pub struct StartTransactionRequest {
    pub transaction_type: StockMovementType,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub source_location_id: Option<Uuid>,
    pub destination_location_id: Option<Uuid>,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
}

/// Request to add item to transaction
#[derive(Debug, Validate, Deserialize)]
pub struct AddTransactionItemRequest {
    pub inventory_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
    pub unit_cost: Option<Decimal>,
}

/// Request to reserve stock
#[derive(Debug, Validate, Deserialize)]
pub struct ReserveStockRequest {
    pub inventory_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub reference_type: String,
    pub reference_id: Uuid,
    #[validate(range(min = 1))]
    pub quantity: i32,
    pub reserved_until: Option<DateTime<Utc>>,
}

/// Request to update inventory levels
#[derive(Debug, Validate, Deserialize)]
pub struct UpdateInventoryRequest {
    pub adjustment_type: AdjustmentType,
    #[validate(range(min = 1))]
    pub quantity: i32,
    #[validate(length(max = 500))]
    pub reason: Option<String>,
    pub unit_cost: Option<Decimal>,
}

/// Request to create inventory lot
#[derive(Debug, Validate, Deserialize)]
pub struct CreateLotRequest {
    pub inventory_id: Uuid,
    #[validate(length(min = 1, max = 100))]
    pub lot_number: String,
    pub batch_number: Option<String>,
    #[validate(range(min = 1))]
    pub quantity_received: i32,
    pub unit_cost: Decimal,
    pub received_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub manufacture_date: Option<NaiveDate>,
    pub supplier_id: Option<Uuid>,
    pub purchase_order_reference: Option<String>,
}

/// Inventory analytics and metrics
#[derive(Debug, Serialize)]
pub struct InventoryMetrics {
    pub total_products: i64,
    pub total_value: Decimal,
    pub low_stock_items: i64,
    pub out_of_stock_items: i64,
    pub reserved_value: Decimal,
    pub average_inventory_age: f64,
    pub turnover_ratio: f64,
    pub abc_distribution: ABCDistribution,
    pub movement_velocity: MovementVelocity,
}

/// ABC classification distribution
#[derive(Debug, Serialize)]
pub struct ABCDistribution {
    pub a_class_count: i64,
    pub a_class_value: Decimal,
    pub b_class_count: i64,
    pub b_class_value: Decimal,
    pub c_class_count: i64,
    pub c_class_value: Decimal,
}

/// Movement velocity metrics
#[derive(Debug, Serialize)]
pub struct MovementVelocity {
    pub fast_moving_items: i64,
    pub medium_moving_items: i64,
    pub slow_moving_items: i64,
    pub dead_stock_items: i64,
}

/// Inventory forecasting data
#[derive(Debug, Serialize)]
pub struct InventoryForecast {
    pub inventory_id: Uuid,
    pub product_name: String,
    pub current_stock: i32,
    pub predicted_demand: i32,
    pub suggested_reorder_quantity: i32,
    pub suggested_reorder_date: DateTime<Utc>,
    pub stock_out_risk: StockOutRisk,
    pub seasonality_factor: f64,
    pub trend_factor: f64,
}

/// Stock out risk levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StockOutRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_movement_type_serialization() {
        let movement = StockMovementType::Inbound;
        let serialized = serde_json::to_string(&movement).unwrap();
        assert_eq!(serialized, "\"Inbound\"");
    }

    #[test]
    fn test_transaction_status_default() {
        let status = InventoryTransactionStatus::Pending;
        assert_eq!(status, InventoryTransactionStatus::Pending);
    }

    #[test]
    fn test_valuation_method_equality() {
        assert_eq!(ValuationMethod::Fifo, ValuationMethod::Fifo);
        assert_ne!(ValuationMethod::Fifo, ValuationMethod::Lifo);
    }
}