// ============================================================================
// OLYMPUS CLOUD - RESTAURANT MODELS
// ============================================================================
// Module: commerce/src/models/restaurant.rs
// Description: Restaurant-specific data models for table management and operations
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Table status in a restaurant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableStatus {
    Available,
    Occupied,
    Reserved,
    Cleaning,
    OutOfOrder,
}

/// Table information for restaurant management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantTable {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub table_number: String,
    pub name: Option<String>,
    pub capacity: i32,
    pub status: TableStatus,
    pub section: Option<String>,
    pub position_x: Option<f64>,
    pub position_y: Option<f64>,
    pub current_order_id: Option<Uuid>,
    pub server_id: Option<Uuid>,
    pub last_cleaned_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Order item with restaurant-specific modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantOrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub modifiers: Vec<OrderItemModifier>,
    pub special_instructions: Option<String>,
    pub kitchen_status: KitchenStatus,
    pub fired_at: Option<DateTime<Utc>>,
    pub ready_at: Option<DateTime<Utc>>,
    pub served_at: Option<DateTime<Utc>>,
}

/// Modifier for order items (e.g., "No onions", "Extra cheese")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemModifier {
    pub id: Uuid,
    pub name: String,
    pub price_adjustment: Decimal,
    pub modifier_type: ModifierType,
}

/// Type of modifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModifierType {
    Addition,
    Removal,
    Substitution,
    Size,
    Preparation,
}

/// Kitchen status for order items
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KitchenStatus {
    Pending,
    InPreparation,
    Ready,
    Served,
    Cancelled,
}

/// Restaurant order with table and service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestaurantOrder {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub order_number: String,
    pub table_id: Option<Uuid>,
    pub server_id: Option<Uuid>,
    pub customer_name: Option<String>,
    pub guest_count: Option<i32>,
    pub order_type: RestaurantOrderType,
    pub status: RestaurantOrderStatus,
    pub items: Vec<RestaurantOrderItem>,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub tip_amount: Option<Decimal>,
    pub total_amount: Decimal,
    pub payment_status: PaymentStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub seat_time: Option<DateTime<Utc>>,
    pub order_time: Option<DateTime<Utc>>,
    pub kitchen_time: Option<DateTime<Utc>>,
    pub served_time: Option<DateTime<Utc>>,
    pub check_closed_at: Option<DateTime<Utc>>,
}

/// Type of restaurant order
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestaurantOrderType {
    DineIn,
    Takeout,
    Delivery,
    Pickup,
}

/// Restaurant-specific order status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RestaurantOrderStatus {
    Open,
    Fired,        // Sent to kitchen
    InProgress,   // Being prepared
    Ready,        // Ready to serve
    Served,       // Delivered to table
    Closed,       // Bill paid and table cleared
    Cancelled,
}

/// Payment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    PartiallyPaid,
    Paid,
    Refunded,
    Failed,
}

/// Kitchen display item for kitchen staff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KitchenDisplayItem {
    pub order_id: Uuid,
    pub order_number: String,
    pub table_number: Option<String>,
    pub item_id: Uuid,
    pub item_name: String,
    pub quantity: i32,
    pub modifiers: Vec<String>,
    pub special_instructions: Option<String>,
    pub status: KitchenStatus,
    pub ordered_at: DateTime<Utc>,
    pub fired_at: Option<DateTime<Utc>>,
    pub estimated_ready_time: Option<DateTime<Utc>>,
    pub priority: KitchenPriority,
}

/// Priority level for kitchen items
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KitchenPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Rush = 3,
}

/// Request to update table status
#[derive(Debug, Deserialize)]
pub struct UpdateTableStatusRequest {
    pub status: TableStatus,
    pub server_id: Option<Uuid>,
    pub notes: Option<String>,
}

/// Request to create a restaurant order
#[derive(Debug, Deserialize)]
pub struct CreateRestaurantOrderRequest {
    pub table_id: Option<Uuid>,
    pub server_id: Option<Uuid>,
    pub customer_name: Option<String>,
    pub guest_count: Option<i32>,
    pub order_type: RestaurantOrderType,
    pub items: Vec<CreateOrderItemRequest>,
    pub notes: Option<String>,
}

/// Request to add an item to an order
#[derive(Debug, Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub modifiers: Vec<CreateModifierRequest>,
    pub special_instructions: Option<String>,
}

/// Request to add a modifier
#[derive(Debug, Deserialize)]
pub struct CreateModifierRequest {
    pub name: String,
    pub price_adjustment: Decimal,
    pub modifier_type: ModifierType,
}

/// Request to update kitchen item status
#[derive(Debug, Deserialize)]
pub struct UpdateKitchenStatusRequest {
    pub status: KitchenStatus,
    pub estimated_ready_time: Option<DateTime<Utc>>,
}

/// Real-time order update for WebSocket
#[derive(Debug, Clone, Serialize)]
pub struct OrderUpdate {
    pub order_id: Uuid,
    pub table_id: Option<Uuid>,
    pub status: RestaurantOrderStatus,
    pub kitchen_items: Vec<KitchenDisplayItem>,
    pub updated_at: DateTime<Utc>,
    pub update_type: OrderUpdateType,
}

/// Type of order update
#[derive(Debug, Clone, Serialize)]
pub enum OrderUpdateType {
    StatusChanged,
    ItemAdded,
    ItemRemoved,
    ItemStatusChanged,
    PaymentUpdated,
}

/// Restaurant dashboard metrics
#[derive(Debug, Clone, Serialize)]
pub struct RestaurantDashboard {
    pub total_tables: i32,
    pub occupied_tables: i32,
    pub available_tables: i32,
    pub reserved_tables: i32,
    pub open_orders: i32,
    pub kitchen_queue_items: i32,
    pub today_revenue: Decimal,
    pub today_covers: i32,
    pub average_table_turn_time: Option<f64>, // minutes
    pub current_wait_time: Option<f64>,       // minutes
    pub peak_hour_indicator: bool,
}

/// Table occupancy analytics
#[derive(Debug, Clone, Serialize)]
pub struct TableAnalytics {
    pub table_id: Uuid,
    pub table_number: String,
    pub turns_today: i32,
    pub average_turn_time: f64,
    pub revenue_today: Decimal,
    pub last_occupied_at: Option<DateTime<Utc>>,
    pub current_status: TableStatus,
}