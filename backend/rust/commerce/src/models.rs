// ============================================================================
// OLYMPUS CLOUD - COMMERCE MODELS
// ============================================================================
// Module: commerce/src/models.rs
// Description: Comprehensive commerce domain models for products, orders, and payments
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use validator::Validate;

// ============================================================================
// PRODUCT CATALOG MODELS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "product_status", rename_all = "lowercase")]
pub enum ProductStatus {
    Draft,
    Active,
    Inactive,
    Discontinued,
    OutOfStock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "product_type", rename_all = "lowercase")]
pub enum ProductType {
    Simple,
    Variable,
    Bundle,
    Digital,
    Service,
    Subscription,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "price_type", rename_all = "lowercase")]
pub enum PriceType {
    Fixed,
    Variable,
    Tiered,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub product_type: ProductType,
    pub status: ProductStatus,
    pub category_id: Option<Uuid>,
    pub brand: Option<String>,
    pub weight: Option<Decimal>,
    pub dimensions: Option<ProductDimensions>,
    pub base_price: Decimal,
    pub price_type: PriceType,
    pub cost_price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub tax_class: Option<String>,
    pub requires_shipping: bool,
    pub is_digital: bool,
    pub track_inventory: bool,
    pub inventory_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub tags: Vec<String>,
    pub attributes: serde_json::Value,
    pub images: Vec<ProductImage>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDimensions {
    pub length: Decimal,
    pub width: Decimal,
    pub height: Decimal,
    pub unit: String, // cm, in, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    pub id: Uuid,
    pub url: String,
    pub alt_text: Option<String>,
    pub position: i32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub image_url: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: Uuid,
    pub product_id: Uuid,
    pub sku: Option<String>,
    pub name: String,
    pub price: Decimal,
    pub cost_price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub dimensions: Option<ProductDimensions>,
    pub inventory_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub track_inventory: bool,
    pub requires_shipping: bool,
    pub is_active: bool,
    pub position: i32,
    pub attributes: serde_json::Value,
    pub images: Vec<ProductImage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttribute {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub display_name: String,
    pub attribute_type: AttributeType,
    pub is_required: bool,
    pub is_variant_attribute: bool,
    pub sort_order: i32,
    pub options: Vec<AttributeOption>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "attribute_type", rename_all = "lowercase")]
pub enum AttributeType {
    Text,
    Number,
    Boolean,
    Select,
    MultiSelect,
    Color,
    Image,
    Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeOption {
    pub id: Uuid,
    pub value: String,
    pub label: String,
    pub color_code: Option<String>,
    pub image_url: Option<String>,
    pub sort_order: i32,
}

// ============================================================================
// PRICING AND DISCOUNT MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub rule_type: PricingRuleType,
    pub applies_to: PricingAppliesTo,
    pub target_ids: Vec<Uuid>, // product_ids, category_ids, etc.
    pub conditions: serde_json::Value,
    pub discount_type: DiscountType,
    pub discount_value: Decimal,
    pub min_quantity: Option<i32>,
    pub max_quantity: Option<i32>,
    pub min_amount: Option<Decimal>,
    pub max_amount: Option<Decimal>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pricing_rule_type", rename_all = "lowercase")]
pub enum PricingRuleType {
    BulkDiscount,
    TieredPricing,
    PercentageDiscount,
    FixedDiscount,
    BOGO, // Buy One Get One
    CategoryDiscount,
    CustomerGroupDiscount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "pricing_applies_to", rename_all = "lowercase")]
pub enum PricingAppliesTo {
    AllProducts,
    SpecificProducts,
    Categories,
    Collections,
    CustomerGroups,
    NewCustomers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "discount_type", rename_all = "lowercase")]
pub enum DiscountType {
    Percentage,
    FixedAmount,
    FixedPrice,
    FreeShipping,
}

// ============================================================================
// ORDER MANAGEMENT MODELS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
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
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_status", rename_all = "lowercase")]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Captured,
    PartiallyRefunded,
    Refunded,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fulfillment_status", rename_all = "lowercase")]
pub enum FulfillmentStatus {
    Unfulfilled,
    PartiallyFulfilled,
    Fulfilled,
    Shipped,
    Delivered,
    Returned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub customer_email: Option<String>,
    pub status: OrderStatus,
    pub payment_status: PaymentStatus,
    pub fulfillment_status: FulfillmentStatus,
    pub currency: String,
    pub subtotal: Decimal,
    pub tax_total: Decimal,
    pub shipping_total: Decimal,
    pub discount_total: Decimal,
    pub total: Decimal,
    pub items: Vec<OrderItem>,
    pub shipping_address: Option<Address>,
    pub billing_address: Option<Address>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: String,
    pub name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub tax_rate: Option<Decimal>,
    pub tax_amount: Option<Decimal>,
    pub discount_amount: Option<Decimal>,
    pub attributes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
}

// ============================================================================
// PAYMENT MODELS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_method", rename_all = "lowercase")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    PayPal,
    Stripe,
    Square,
    BankTransfer,
    Cash,
    Check,
    StoreCredit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub order_id: Uuid,
    pub payment_method: PaymentMethod,
    pub gateway: String,
    pub gateway_transaction_id: Option<String>,
    pub status: PaymentStatus,
    pub amount: Decimal,
    pub currency: String,
    pub fees: Option<Decimal>,
    pub gateway_response: serde_json::Value,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// INVENTORY MODELS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "inventory_adjustment_type", rename_all = "lowercase")]
pub enum InventoryAdjustmentType {
    Increase,
    Decrease,
    Sale,
    Return,
    Damage,
    Loss,
    Transfer,
    Recount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub sku: String,
    pub quantity_available: i32,
    pub quantity_reserved: i32,
    pub quantity_on_hand: i32,
    pub low_stock_threshold: Option<i32>,
    pub cost_per_unit: Option<Decimal>,
    pub last_counted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inventory_item_id: Uuid,
    pub adjustment_type: InventoryAdjustmentType,
    pub quantity_change: i32,
    pub reason: Option<String>,
    pub reference_id: Option<Uuid>, // order_id, transfer_id, etc.
    pub adjusted_by: Uuid,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 1, max = 50))]
    pub sku: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    #[validate(length(max = 500))]
    pub short_description: Option<String>,
    pub product_type: ProductType,
    pub category_id: Option<Uuid>,
    pub brand: Option<String>,
    pub weight: Option<Decimal>,
    pub dimensions: Option<ProductDimensions>,
    #[validate(range(min = 0))]
    pub base_price: Decimal,
    pub price_type: PriceType,
    pub cost_price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub tax_class: Option<String>,
    pub requires_shipping: bool,
    pub is_digital: bool,
    pub track_inventory: bool,
    pub inventory_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub attributes: Option<serde_json::Value>,
    pub images: Option<Vec<ProductImage>>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProductRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    #[validate(length(max = 2000))]
    pub description: Option<String>,
    #[validate(length(max = 500))]
    pub short_description: Option<String>,
    pub status: Option<ProductStatus>,
    pub category_id: Option<Uuid>,
    pub brand: Option<String>,
    pub weight: Option<Decimal>,
    pub dimensions: Option<ProductDimensions>,
    pub base_price: Option<Decimal>,
    pub price_type: Option<PriceType>,
    pub cost_price: Option<Decimal>,
    pub compare_at_price: Option<Decimal>,
    pub tax_class: Option<String>,
    pub requires_shipping: Option<bool>,
    pub is_digital: Option<bool>,
    pub track_inventory: Option<bool>,
    pub inventory_quantity: Option<i32>,
    pub low_stock_threshold: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub attributes: Option<serde_json::Value>,
    pub images: Option<Vec<ProductImage>>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSearchRequest {
    pub query: Option<String>,
    pub category_id: Option<Uuid>,
    pub status: Option<ProductStatus>,
    pub product_type: Option<ProductType>,
    pub tags: Option<Vec<String>>,
    pub price_min: Option<Decimal>,
    pub price_max: Option<Decimal>,
    pub in_stock_only: Option<bool>,
    pub sort_by: Option<ProductSortBy>,
    pub sort_order: Option<SortOrder>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductSortBy {
    Name,
    Price,
    CreatedAt,
    UpdatedAt,
    Popularity,
    Stock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSearchResponse {
    pub products: Vec<Product>,
    pub total_count: i64,
    pub has_more: bool,
    pub facets: ProductSearchFacets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSearchFacets {
    pub categories: Vec<CategoryFacet>,
    pub price_ranges: Vec<PriceRangeFacet>,
    pub brands: Vec<BrandFacet>,
    pub attributes: Vec<AttributeFacet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryFacet {
    pub category_id: Uuid,
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRangeFacet {
    pub min: Decimal,
    pub max: Decimal,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandFacet {
    pub brand: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeFacet {
    pub attribute_name: String,
    pub values: Vec<AttributeValueFacet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeValueFacet {
    pub value: String,
    pub count: i64,
}