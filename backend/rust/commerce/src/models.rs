use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use validator::Validate;
use olympus_shared::types::{Money, Currency, Address, PhoneNumber};

// Product Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub brand: Option<String>,
    pub unit_of_measure: String,
    pub price: Decimal,
    pub cost: Option<Decimal>,
    pub tax_rate: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub dimensions: Option<ProductDimensions>,
    pub images: Vec<String>,
    pub tags: Vec<String>,
    pub is_active: bool,
    pub is_digital: bool,
    pub requires_shipping: bool,
    pub track_inventory: bool,
    pub allow_backorder: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: Decimal,
    pub width: Decimal,
    pub height: Decimal,
    pub unit: String, // cm, inch, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: String,
    pub name: String,
    pub options: serde_json::Value, // {"size": "L", "color": "Blue"}
    pub price: Option<Decimal>,
    pub cost: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub barcode: Option<String>,
    pub images: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Category Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Inventory Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Inventory {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity_on_hand: Decimal,
    pub quantity_reserved: Decimal,
    pub quantity_available: Decimal,
    pub reorder_point: Option<Decimal>,
    pub reorder_quantity: Option<Decimal>,
    pub last_counted_at: Option<DateTime<Utc>>,
    pub last_received_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

// Order Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub status: OrderStatus,
    pub order_type: OrderType,
    pub channel: OrderChannel,
    pub currency: Currency,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub shipping_amount: Decimal,
    pub discount_amount: Decimal,
    pub total_amount: Decimal,
    pub paid_amount: Decimal,
    pub refunded_amount: Decimal,
    pub billing_address: Option<Address>,
    pub shipping_address: Option<Address>,
    pub customer_notes: Option<String>,
    pub internal_notes: Option<String>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub ordered_at: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub fulfilled_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Draft,
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Completed,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Standard,
    Pickup,
    Delivery,
    DineIn,
    Takeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderChannel {
    Web,
    Mobile,
    POS,
    Phone,
    Marketplace,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub product_name: String,
    pub product_sku: String,
    pub quantity: Decimal,
    pub unit_price: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,
}

// Cart Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cart {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub status: CartStatus,
    pub currency: Currency,
    pub items_count: i32,
    pub subtotal: Decimal,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CartStatus {
    Active,
    Abandoned,
    Converted,
    Expired,
}

// Payment Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub order_id: Uuid,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub amount: Decimal,
    pub currency: Currency,
    pub gateway: String,
    pub gateway_transaction_id: Option<String>,
    pub gateway_response: Option<serde_json::Value>,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub failure_reason: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Card,
    Cash,
    BankTransfer,
    Wallet,
    GiftCard,
    StoreCredit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Authorized,
    Captured,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

// Customer Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Customer {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<PhoneNumber>,
    pub birth_date: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub addresses: Vec<CustomerAddress>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub accepts_marketing: bool,
    pub vip_status: Option<String>,
    pub loyalty_points: i32,
    pub total_spent: Decimal,
    pub total_orders: i32,
    pub last_order_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAddress {
    pub label: String,
    pub address: Address,
    pub is_default_billing: bool,
    pub is_default_shipping: bool,
}

// Discount Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Discount {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub value: Decimal,
    pub minimum_amount: Option<Decimal>,
    pub minimum_quantity: Option<i32>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub customer_limit: Option<i32>,
    pub applies_to: DiscountAppliesTo,
    pub product_ids: Vec<Uuid>,
    pub category_ids: Vec<Uuid>,
    pub customer_ids: Vec<Uuid>,
    pub is_active: bool,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdjustmentType {
    Add,
    Remove,
    Set,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountType {
    Percentage,
    FixedAmount,
    BuyXGetY,
    FreeShipping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscountAppliesTo {
    All,
    Products,
    Categories,
    Customers,
}

// Request/Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 100))]
    pub sku: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub price: Decimal,
    pub cost: Option<Decimal>,
    pub track_inventory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateOrderRequest {
    pub customer_id: Option<Uuid>,
    pub order_type: OrderType,
    pub items: Vec<OrderItemRequest>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub quantity: Decimal,
    pub unit_price: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPaymentRequest {
    pub order_id: Uuid,
    pub payment_method: PaymentMethod,
    pub amount: Decimal,
    pub card_token: Option<String>,
}