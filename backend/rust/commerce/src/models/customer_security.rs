// ============================================================================
// OLYMPUS CLOUD - CUSTOMER SECURITY MODELS
// ============================================================================
// Module: commerce/src/models/customer_security.rs
// Description: Enhanced customer security and privacy models
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use std::net::IpAddr;

/// Enhanced customer model with security and privacy features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCustomer {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub addresses: serde_json::Value,
    pub default_address_id: Option<Uuid>,
    pub accepts_marketing: bool,
    pub tax_exempt: bool,
    pub notes: Option<String>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
    pub total_spent: rust_decimal::Decimal,
    pub order_count: i32,
    pub last_order_at: Option<DateTime<Utc>>,

    // Security fields
    pub email_verified: bool,
    pub email_verification_token: Option<String>,
    pub email_verification_expires_at: Option<DateTime<Utc>>,
    pub password_reset_token: Option<String>,
    pub password_reset_expires_at: Option<DateTime<Utc>>,
    pub login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub last_login_ip: Option<IpAddr>,

    // Privacy fields
    pub privacy_consent_given: bool,
    pub privacy_consent_date: Option<DateTime<Utc>>,
    pub marketing_consent_given: bool,
    pub marketing_consent_date: Option<DateTime<Utc>>,
    pub gdpr_deletion_requested: bool,
    pub gdpr_deletion_requested_at: Option<DateTime<Utc>>,
    pub data_retention_expires_at: Option<DateTime<Utc>>,
    pub data_classification: DataClassification,

    // Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Data classification levels for customer data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

impl Default for DataClassification {
    fn default() -> Self {
        Self::Public
    }
}

/// Encrypted customer data storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerEncryptedData {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub tenant_id: Uuid,
    pub data_type: String,
    pub encrypted_value: String,
    pub encryption_key_version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub accessed_at: Option<DateTime<Utc>>,
    pub access_count: i32,
}

/// Customer audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAuditLog {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub tenant_id: Uuid,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub performed_by: Option<Uuid>,
    pub performed_by_type: PerformedByType,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub session_id: Option<String>,
    pub compliance_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Who performed the action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum PerformedByType {
    User,
    System,
    Api,
}

impl Default for PerformedByType {
    fn default() -> Self {
        Self::User
    }
}

/// Customer data access log for GDPR compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDataAccessLog {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub tenant_id: Uuid,
    pub accessed_by: Option<Uuid>,
    pub access_type: DataAccessType,
    pub data_fields: Vec<String>,
    pub purpose: Option<String>,
    pub legal_basis: Option<GdprLegalBasis>,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Types of data access
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum DataAccessType {
    Read,
    Export,
    Modify,
    Delete,
}

/// GDPR legal basis for data processing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum GdprLegalBasis {
    Consent,
    Contract,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterest,
}

/// Customer consent tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerConsent {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub tenant_id: Uuid,
    pub consent_type: String,
    pub status: bool,
    pub consent_method: ConsentMethod,
    pub consent_text: Option<String>,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub valid_from: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Methods of obtaining consent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum ConsentMethod {
    Explicit,
    Implicit,
    OptIn,
    OptOut,
}

/// Request to update customer security settings
#[derive(Debug, Validate, Deserialize)]
pub struct UpdateCustomerSecurityRequest {
    #[validate(email)]
    pub email: Option<String>,
    pub privacy_consent_given: Option<bool>,
    pub marketing_consent_given: Option<bool>,
    pub data_retention_years: Option<i32>,
}

/// Request to log data access
#[derive(Debug, Validate, Deserialize)]
pub struct LogDataAccessRequest {
    pub access_type: DataAccessType,
    pub data_fields: Vec<String>,
    #[validate(length(max = 255))]
    pub purpose: Option<String>,
    pub legal_basis: Option<GdprLegalBasis>,
}

/// Request to record consent
#[derive(Debug, Validate, Deserialize)]
pub struct RecordConsentRequest {
    #[validate(length(min = 1, max = 100))]
    pub consent_type: String,
    pub status: bool,
    pub consent_method: ConsentMethod,
    pub consent_text: Option<String>,
    pub valid_until: Option<DateTime<Utc>>,
}

/// Request for GDPR data export
#[derive(Debug, Serialize)]
pub struct GdprExportData {
    pub customer: serde_json::Value,
    pub orders: serde_json::Value,
    pub consents: serde_json::Value,
    pub exported_at: DateTime<Utc>,
    pub export_id: Uuid,
}

/// Customer security analytics
#[derive(Debug, Serialize)]
pub struct CustomerSecurityMetrics {
    pub total_customers: i64,
    pub verified_customers: i64,
    pub locked_customers: i64,
    pub gdpr_deletion_requests: i64,
    pub privacy_consent_rate: f64,
    pub marketing_consent_rate: f64,
    pub data_access_events_today: i64,
    pub security_incidents_this_week: i64,
}

/// Customer privacy settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPrivacySettings {
    pub customer_id: Uuid,
    pub privacy_consent: bool,
    pub marketing_consent: bool,
    pub analytics_consent: bool,
    pub cookies_consent: bool,
    pub data_retention_period: Option<i32>, // months
    pub communication_preferences: CommunicationPreferences,
    pub data_sharing_preferences: DataSharingPreferences,
}

/// Communication preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPreferences {
    pub email_notifications: bool,
    pub sms_notifications: bool,
    pub push_notifications: bool,
    pub postal_mail: bool,
    pub phone_calls: bool,
}

/// Data sharing preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSharingPreferences {
    pub analytics_partners: bool,
    pub marketing_partners: bool,
    pub service_providers: bool,
    pub legal_compliance: bool, // Cannot be opted out
}

impl Default for CommunicationPreferences {
    fn default() -> Self {
        Self {
            email_notifications: true,
            sms_notifications: false,
            push_notifications: true,
            postal_mail: false,
            phone_calls: false,
        }
    }
}

impl Default for DataSharingPreferences {
    fn default() -> Self {
        Self {
            analytics_partners: false,
            marketing_partners: false,
            service_providers: true, // Required for service delivery
            legal_compliance: true,  // Cannot be opted out
        }
    }
}

/// Customer data anonymization result
#[derive(Debug, Serialize)]
pub struct AnonymizationResult {
    pub customer_id: Uuid,
    pub anonymized_at: DateTime<Utc>,
    pub data_removed: Vec<String>,
    pub retention_period_ended: bool,
    pub legal_hold_active: bool,
}

/// Customer security incident
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncident {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub tenant_id: Uuid,
    pub incident_type: SecurityIncidentType,
    pub severity: IncidentSeverity,
    pub description: String,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Types of security incidents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum SecurityIncidentType {
    FailedLogin,
    AccountLocked,
    SuspiciousActivity,
    DataBreach,
    UnauthorizedAccess,
    PasswordReset,
}

/// Incident severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_classification_default() {
        let classification = DataClassification::default();
        assert_eq!(classification, DataClassification::Public);
    }

    #[test]
    fn test_communication_preferences_default() {
        let prefs = CommunicationPreferences::default();
        assert!(prefs.email_notifications);
        assert!(!prefs.sms_notifications);
        assert!(prefs.push_notifications);
    }

    #[test]
    fn test_data_sharing_preferences_default() {
        let prefs = DataSharingPreferences::default();
        assert!(!prefs.analytics_partners);
        assert!(!prefs.marketing_partners);
        assert!(prefs.service_providers);
        assert!(prefs.legal_compliance);
    }
}