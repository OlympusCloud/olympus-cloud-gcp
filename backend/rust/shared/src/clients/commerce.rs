//! Commerce service client

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::{HttpClient, HttpClientConfig, HttpClientError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Decimal,
    pub tax_rate: Decimal,
    pub is_active: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Decimal,
    pub tax_rate: Decimal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub order_number: String,
    pub status: String,
    pub subtotal: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
    pub payment_status: String,
    pub items: Vec<OrderItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
    pub modifiers: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub location_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub items: Vec<CreateOrderItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub modifiers: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: Decimal,
    pub method: String,
    pub status: String,
    pub transaction_id: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessPaymentRequest {
    pub order_id: Uuid,
    pub amount: Decimal,
    pub method: String,
    pub payment_details: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub quantity_on_hand: i32,
    pub quantity_reserved: i32,
    pub reorder_point: i32,
    pub reorder_quantity: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryRequest {
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub adjustment: i32,
    pub reason: String,
}

pub struct CommerceClient {
    http_client: HttpClient,
}

impl CommerceClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.http_client = self.http_client.with_token(token);
        self
    }

    // Product operations
    pub async fn get_product(&self, product_id: Uuid) -> Result<Product, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/products/{}", product_id)).await
    }

    pub async fn create_product(&self, request: CreateProductRequest) -> Result<Product, HttpClientError> {
        self.http_client.post("/api/v1/commerce/products", &request).await
    }

    pub async fn list_products(&self, tenant_id: Uuid) -> Result<Vec<Product>, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/tenants/{}/products", tenant_id)).await
    }

    // Order operations
    pub async fn get_order(&self, order_id: Uuid) -> Result<Order, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/orders/{}", order_id)).await
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order, HttpClientError> {
        self.http_client.post("/api/v1/commerce/orders", &request).await
    }

    pub async fn update_order_status(&self, order_id: Uuid, request: UpdateOrderStatusRequest) -> Result<Order, HttpClientError> {
        self.http_client.put(&format!("/api/v1/commerce/orders/{}/status", order_id), &request).await
    }

    pub async fn list_orders(&self, tenant_id: Uuid) -> Result<Vec<Order>, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/tenants/{}/orders", tenant_id)).await
    }

    // Payment operations
    pub async fn process_payment(&self, request: ProcessPaymentRequest) -> Result<Payment, HttpClientError> {
        self.http_client.post("/api/v1/commerce/payments", &request).await
    }

    pub async fn get_payment(&self, payment_id: Uuid) -> Result<Payment, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/payments/{}", payment_id)).await
    }

    pub async fn list_order_payments(&self, order_id: Uuid) -> Result<Vec<Payment>, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/orders/{}/payments", order_id)).await
    }

    // Inventory operations
    pub async fn get_inventory(&self, product_id: Uuid, location_id: Uuid) -> Result<InventoryItem, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/inventory/{}/{}", product_id, location_id)).await
    }

    pub async fn update_inventory(&self, request: UpdateInventoryRequest) -> Result<InventoryItem, HttpClientError> {
        self.http_client.post("/api/v1/commerce/inventory/adjust", &request).await
    }

    pub async fn list_inventory(&self, location_id: Uuid) -> Result<Vec<InventoryItem>, HttpClientError> {
        self.http_client.get(&format!("/api/v1/commerce/locations/{}/inventory", location_id)).await
    }

    pub async fn health_check(&self) -> Result<bool, HttpClientError> {
        self.http_client.health_check().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_serialization() {
        let order = Order {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            location_id: Uuid::new_v4(),
            customer_id: None,
            order_number: "ORD-001".to_string(),
            status: "pending".to_string(),
            subtotal: Decimal::from(100),
            tax: Decimal::from(10),
            total: Decimal::from(110),
            payment_status: "pending".to_string(),
            items: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("ORD-001"));
    }
}