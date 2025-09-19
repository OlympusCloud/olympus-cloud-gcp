// ============================================================================
// OLYMPUS CLOUD - EVENT TYPE DEFINITIONS
// ============================================================================
// Module: shared/src/events/types.rs
// Description: Strongly-typed event definitions for all domains
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// ============================================================================
// AUTHENTICATION EVENTS
// ============================================================================

/// User registration event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email_verification_required: bool,
    pub registration_source: String, // "web", "mobile", "api", "invite"
}

/// User login event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedInEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub session_id: String,
    pub device_id: Option<String>,
    pub login_method: LoginMethod,
    pub ip_address: String,
    pub user_agent: String,
    pub mfa_used: bool,
    pub location: Option<GeolocationData>,
}

/// User logout event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedOutEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub session_id: String,
    pub logout_reason: LogoutReason,
}

/// Password reset requested event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetRequestedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub reset_token_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub request_source: String, // "web", "mobile", "api"
}

/// Password changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub change_reason: PasswordChangeReason,
    pub reset_token_id: Option<Uuid>,
}

/// Email verification event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerifiedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub verification_token_id: Uuid,
    pub verified_at: DateTime<Utc>,
}

/// Account locked event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLockedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub lock_reason: LockReason,
    pub locked_until: Option<DateTime<Utc>>,
    pub failed_attempts: i32,
}

/// MFA enrollment event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaEnrolledEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub mfa_type: MfaType,
    pub backup_codes_generated: u32,
}

// ============================================================================
// PLATFORM EVENTS
// ============================================================================

/// Tenant created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantCreatedEvent {
    pub tenant_id: Uuid,
    pub slug: String,
    pub name: String,
    pub industry: String,
    pub subscription_tier: String,
    pub created_by: Uuid,
    pub trial_ends_at: Option<DateTime<Utc>>,
}

/// Tenant subscription changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSubscriptionChangedEvent {
    pub tenant_id: Uuid,
    pub old_tier: String,
    pub new_tier: String,
    pub changed_by: Uuid,
    pub effective_date: DateTime<Utc>,
    pub billing_cycle: String,
}

/// User role changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleChangedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub old_roles: Vec<String>,
    pub new_roles: Vec<String>,
    pub changed_by: Uuid,
    pub reason: String,
}

/// Location created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationCreatedEvent {
    pub location_id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: Option<String>,
    pub address: AddressData,
    pub created_by: Uuid,
}

/// Feature flag changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagChangedEvent {
    pub tenant_id: Uuid,
    pub feature_name: String,
    pub old_value: bool,
    pub new_value: bool,
    pub changed_by: Uuid,
    pub rollout_percentage: Option<f32>,
}

// ============================================================================
// COMMERCE EVENTS
// ============================================================================

/// Product created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCreatedEvent {
    pub product_id: Uuid,
    pub tenant_id: Uuid,
    pub sku: Option<String>,
    pub name: String,
    pub category_id: Option<Uuid>,
    pub unit_price: Decimal,
    pub created_by: Uuid,
    pub is_active: bool,
}

/// Product updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductUpdatedEvent {
    pub product_id: Uuid,
    pub tenant_id: Uuid,
    pub updated_fields: Vec<String>,
    pub updated_by: Uuid,
    pub old_price: Option<Decimal>,
    pub new_price: Option<Decimal>,
}

/// Order created event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub location_id: Uuid,
    pub order_number: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub item_count: i32,
    pub order_source: OrderSource,
}

/// Order status changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusChangedEvent {
    pub order_id: Uuid,
    pub tenant_id: Uuid,
    pub old_status: String,
    pub new_status: String,
    pub changed_by: Option<Uuid>,
    pub reason: Option<String>,
    pub estimated_fulfillment: Option<DateTime<Utc>>,
}

/// Payment processed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentProcessedEvent {
    pub payment_id: Uuid,
    pub order_id: Uuid,
    pub tenant_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub payment_method: PaymentMethodType,
    pub gateway: String,
    pub transaction_id: String,
    pub status: PaymentStatus,
}

/// Inventory adjusted event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAdjustedEvent {
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub tenant_id: Uuid,
    pub old_quantity: i32,
    pub new_quantity: i32,
    pub adjustment_reason: InventoryAdjustmentReason,
    pub adjusted_by: Option<Uuid>,
    pub reference_id: Option<Uuid>, // Order ID, transfer ID, etc.
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginMethod {
    Password,
    OAuth,
    ApiKey,
    Device,
    MagicLink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogoutReason {
    UserInitiated,
    SessionExpired,
    SecurityEvent,
    AdminAction,
    SystemShutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordChangeReason {
    UserRequested,
    PasswordReset,
    SecurityRequirement,
    AdminAction,
    PolicyViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LockReason {
    FailedLogins,
    SecurityViolation,
    AdminAction,
    SuspiciousActivity,
    ComplianceRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaType {
    TOTP,
    SMS,
    Email,
    Hardware,
    BackupCodes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSource {
    Web,
    Mobile,
    Api,
    POS,
    Phone,
    InPerson,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethodType {
    CreditCard,
    DebitCard,
    Cash,
    Check,
    PayPal,
    ApplePay,
    GooglePay,
    BankTransfer,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartialRefund,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InventoryAdjustmentReason {
    Sale,
    Return,
    Damage,
    Theft,
    Recount,
    Transfer,
    Promotion,
    Waste,
    Other,
}

/// Geolocation data for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationData {
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy: Option<f64>,
}

/// Address data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressData {
    pub street1: String,
    pub street2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub formatted: Option<String>,
}

// ============================================================================
// EVENT TYPE CONSTANTS
// ============================================================================

/// Authentication event types
pub mod auth_events {
    pub const USER_REGISTERED: &str = "UserRegistered";
    pub const USER_LOGGED_IN: &str = "UserLoggedIn";
    pub const USER_LOGGED_OUT: &str = "UserLoggedOut";
    pub const PASSWORD_RESET_REQUESTED: &str = "PasswordResetRequested";
    pub const PASSWORD_CHANGED: &str = "PasswordChanged";
    pub const EMAIL_VERIFIED: &str = "EmailVerified";
    pub const ACCOUNT_LOCKED: &str = "AccountLocked";
    pub const MFA_ENROLLED: &str = "MfaEnrolled";
}

/// Platform event types
pub mod platform_events {
    pub const TENANT_CREATED: &str = "TenantCreated";
    pub const TENANT_SUBSCRIPTION_CHANGED: &str = "TenantSubscriptionChanged";
    pub const USER_ROLE_CHANGED: &str = "UserRoleChanged";
    pub const LOCATION_CREATED: &str = "LocationCreated";
    pub const FEATURE_FLAG_CHANGED: &str = "FeatureFlagChanged";
}

/// Commerce event types
pub mod commerce_events {
    pub const PRODUCT_CREATED: &str = "ProductCreated";
    pub const PRODUCT_UPDATED: &str = "ProductUpdated";
    pub const ORDER_CREATED: &str = "OrderCreated";
    pub const ORDER_STATUS_CHANGED: &str = "OrderStatusChanged";
    pub const PAYMENT_PROCESSED: &str = "PaymentProcessed";
    pub const INVENTORY_ADJUSTED: &str = "InventoryAdjusted";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = UserRegisteredEvent {
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email_verification_required: true,
            registration_source: "web".to_string(),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: UserRegisteredEvent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.email, deserialized.email);
        assert_eq!(event.first_name, deserialized.first_name);
    }

    #[test]
    fn test_payment_status_variants() {
        let statuses = vec![
            PaymentStatus::Pending,
            PaymentStatus::Processing,
            PaymentStatus::Completed,
            PaymentStatus::Failed,
            PaymentStatus::Cancelled,
            PaymentStatus::Refunded,
            PaymentStatus::PartialRefund,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status).unwrap();
            let _deserialized: PaymentStatus = serde_json::from_str(&serialized).unwrap();
        }
    }

    #[test]
    fn test_event_type_constants() {
        assert_eq!(auth_events::USER_REGISTERED, "UserRegistered");
        assert_eq!(platform_events::TENANT_CREATED, "TenantCreated");
        assert_eq!(commerce_events::ORDER_CREATED, "OrderCreated");
    }
}