// ============================================================================
// OLYMPUS CLOUD - ORDER MODELS
// ============================================================================
// Module: shared/src/models/order.rs
// Description: Order management models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{AuditFields, TenantScoped};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use rust_decimal::Decimal;

/// Order status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "order_status", rename_all = "UPPERCASE")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
}

/// Payment status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_status", rename_all = "UPPERCASE")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartialRefund,
}

/// Fulfillment status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "fulfillment_status", rename_all = "UPPERCASE")]
pub enum FulfillmentStatus {
    Unfulfilled,
    Partial,
    Fulfilled,
    Cancelled,
}

/// Order entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub location_id: Uuid,
    pub status: OrderStatus,
    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub discount_amount: Decimal,
    pub shipping_amount: Decimal,
    pub total_amount: Decimal,
    pub currency: serde_json::Value,
    pub payment_status: PaymentStatus,
    pub fulfillment_status: FulfillmentStatus,
    pub shipping_address: Option<serde_json::Value>,
    pub billing_address: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub fulfilled_at: Option<DateTime<Utc>>,
    #[sqlx(flatten)]
    pub audit_fields: AuditFields,
}

impl Order {
    /// Create a new order
    pub fn new(tenant_id: Uuid, order_number: String, location_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            order_number,
            customer_id: None,
            location_id,
            status: OrderStatus::Pending,
            subtotal: Decimal::ZERO,
            tax_amount: Decimal::ZERO,
            discount_amount: Decimal::ZERO,
            shipping_amount: Decimal::ZERO,
            total_amount: Decimal::ZERO,
            currency: serde_json::json!({"code": "USD", "symbol": "$"}),
            payment_status: PaymentStatus::Pending,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            shipping_address: None,
            billing_address: None,
            notes: None,
            tags: vec![],
            metadata: serde_json::json!({}),
            cancelled_at: None,
            fulfilled_at: None,
            audit_fields: AuditFields {
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        }
    }

    /// Calculate total amount
    pub fn calculate_total(&mut self) {
        self.total_amount = self.subtotal + self.tax_amount + self.shipping_amount - self.discount_amount;
        self.audit_fields.updated_at = Utc::now();
    }

    /// Check if order can be cancelled
    pub fn can_cancel(&self) -> bool {
        matches!(self.status, OrderStatus::Pending | OrderStatus::Confirmed)
    }

    /// Cancel the order
    pub fn cancel(&mut self) {
        if self.can_cancel() {
            self.status = OrderStatus::Cancelled;
            self.cancelled_at = Some(Utc::now());
            self.audit_fields.updated_at = Utc::now();
        }
    }
}

impl TenantScoped for Order {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

/// Order item entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub sku: Option<String>,
    pub name: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub discount_amount: Decimal,
    pub tax_amount: Decimal,
    pub total_amount: Decimal,
    pub fulfillment_status: FulfillmentStatus,
    pub fulfilled_quantity: i32,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl OrderItem {
    /// Create a new order item
    pub fn new(
        order_id: Uuid,
        product_id: Uuid,
        name: String,
        quantity: i32,
        unit_price: Decimal,
    ) -> Self {
        let total = unit_price * Decimal::from(quantity);
        Self {
            id: Uuid::new_v4(),
            order_id,
            product_id,
            variant_id: None,
            sku: None,
            name,
            quantity,
            unit_price,
            discount_amount: Decimal::ZERO,
            tax_amount: Decimal::ZERO,
            total_amount: total,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            fulfilled_quantity: 0,
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
        }
    }

    /// Calculate total amount
    pub fn calculate_total(&mut self) {
        let line_total = self.unit_price * Decimal::from(self.quantity);
        self.total_amount = line_total + self.tax_amount - self.discount_amount;
    }
}