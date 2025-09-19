// ============================================================================
// OLYMPUS CLOUD - PAYMENT MODELS
// ============================================================================
// Module: shared/src/models/payment.rs
// Description: Payment processing models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{TenantScoped, PaymentStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use rust_decimal::Decimal;

/// Payment method enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_method", rename_all = "UPPERCASE")]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    Paypal,
    BankTransfer,
    Cash,
    Check,
    Other,
}

/// Payment type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "payment_type", rename_all = "UPPERCASE")]
pub enum PaymentType {
    Sale,
    Refund,
    PartialRefund,
    Authorization,
    Capture,
    Void,
}

/// Payment entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub order_id: Uuid,
    pub transaction_id: String,
    pub payment_method: PaymentMethod,
    pub payment_type: PaymentType,
    pub status: PaymentStatus,
    pub amount: Decimal,
    pub currency: serde_json::Value,
    pub gateway: Option<String>,
    pub gateway_response: serde_json::Value,
    pub reference_number: Option<String>,
    pub authorization_code: Option<String>,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub metadata: serde_json::Value,
    pub processed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub refunded_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Payment {
    /// Create a new payment
    pub fn new(
        tenant_id: Uuid,
        order_id: Uuid,
        transaction_id: String,
        payment_method: PaymentMethod,
        amount: Decimal,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            order_id,
            transaction_id,
            payment_method,
            payment_type: PaymentType::Sale,
            status: PaymentStatus::Pending,
            amount,
            currency: serde_json::json!({"code": "USD", "symbol": "$"}),
            gateway: None,
            gateway_response: serde_json::json!({}),
            reference_number: None,
            authorization_code: None,
            card_last_four: None,
            card_brand: None,
            metadata: serde_json::json!({}),
            processed_at: None,
            cancelled_at: None,
            refunded_amount: Decimal::ZERO,
            created_at: now,
            updated_at: now,
        }
    }
}

impl TenantScoped for Payment {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}