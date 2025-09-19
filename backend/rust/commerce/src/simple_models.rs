use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
// Using f64 for demo simplicity instead of Decimal

// Simplified models for demo

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "commerce.order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Draft,
    Pending,
    Confirmed,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub status: OrderStatus,
    pub total: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub user_id: Option<Uuid>,
    pub status: Option<OrderStatus>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderRequest {
    pub status: Option<OrderStatus>,
    pub total: Option<f64>,
}

// Response DTOs
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub product: Product,
}

#[derive(Debug, Serialize)]
pub struct ProductsResponse {
    pub products: Vec<Product>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order: Order,
}

#[derive(Debug, Serialize)]
pub struct OrdersResponse {
    pub orders: Vec<Order>,
    pub total: i64,
}