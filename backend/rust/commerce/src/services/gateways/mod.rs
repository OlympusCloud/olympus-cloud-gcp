// ============================================================================
// OLYMPUS CLOUD - PAYMENT GATEWAYS
// ============================================================================
// Module: commerce/src/services/gateways/mod.rs
// Description: Payment gateway integrations and abstractions
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use olympus_shared::error::ApiResult;

pub mod stripe;
pub mod square;

pub use stripe::StripeGateway;
pub use square::SquareGateway;

// ============================================================================
// GATEWAY TRAIT
// ============================================================================

#[async_trait]
pub trait PaymentGateway: Send + Sync {
    /// Create a payment authorization
    async fn authorize(
        &self,
        amount: Decimal,
        currency: &str,
        payment_method: &str,
        metadata: Option<serde_json::Value>,
    ) -> ApiResult<PaymentGatewayResponse>;

    /// Capture an authorized payment
    async fn capture(
        &self,
        payment_id: &str,
        amount: Option<Decimal>,
    ) -> ApiResult<PaymentGatewayResponse>;

    /// Charge a payment (authorize + capture)
    async fn charge(
        &self,
        amount: Decimal,
        currency: &str,
        payment_method: &str,
        metadata: Option<serde_json::Value>,
    ) -> ApiResult<PaymentGatewayResponse>;

    /// Refund a payment
    async fn refund(
        &self,
        payment_id: &str,
        amount: Decimal,
        reason: Option<&str>,
    ) -> ApiResult<PaymentGatewayResponse>;

    /// Cancel/void a payment
    async fn cancel(&self, payment_id: &str) -> ApiResult<PaymentGatewayResponse>;

    /// Get payment details
    async fn get_payment(&self, payment_id: &str) -> ApiResult<PaymentGatewayResponse>;

    /// Create a customer
    async fn create_customer(
        &self,
        email: &str,
        name: Option<&str>,
        metadata: Option<serde_json::Value>,
    ) -> ApiResult<CustomerGatewayResponse>;

    /// Create a payment method
    async fn create_payment_method(
        &self,
        customer_id: &str,
        payment_details: PaymentMethodDetails,
    ) -> ApiResult<PaymentMethodGatewayResponse>;

    /// List payment methods for a customer
    async fn list_payment_methods(
        &self,
        customer_id: &str,
    ) -> ApiResult<Vec<PaymentMethodGatewayResponse>>;

    /// Delete a payment method
    async fn delete_payment_method(&self, payment_method_id: &str) -> ApiResult<()>;
}

// ============================================================================
// RESPONSE TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewayResponse {
    pub id: String,
    pub status: PaymentGatewayStatus,
    pub amount: Decimal,
    pub currency: String,
    pub customer_id: Option<String>,
    pub payment_method_id: Option<String>,
    pub error_message: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentGatewayStatus {
    Pending,
    Authorized,
    Captured,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerGatewayResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodGatewayResponse {
    pub id: String,
    pub customer_id: String,
    pub method_type: String,
    pub last_four: Option<String>,
    pub brand: Option<String>,
    pub exp_month: Option<i32>,
    pub exp_year: Option<i32>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethodDetails {
    pub method_type: PaymentMethodType,
    pub card: Option<CardDetails>,
    pub bank_account: Option<BankAccountDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodType {
    Card,
    BankAccount,
    Wallet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub number: String,
    pub exp_month: i32,
    pub exp_year: i32,
    pub cvc: String,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountDetails {
    pub account_number: String,
    pub routing_number: String,
    pub account_type: String,
    pub account_holder_name: String,
}