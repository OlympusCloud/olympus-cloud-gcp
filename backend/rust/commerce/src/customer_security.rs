use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

use olympus_shared::security::{
    CustomerDataEncryption, EncryptedData, DataClassification,
    CustomerDataAuditor, CustomerDataAction, DataAnonymizer
};
use olympus_shared::error::Result;

/// Secure customer profile with encrypted PII data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCustomerProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_number: Option<String>,
    pub external_id: Option<String>,

    // Public data (not encrypted)
    pub first_name: String,      // Can be public for display
    pub last_name: String,       // Can be public for display
    pub display_name: Option<String>,

    // Encrypted confidential data
    pub email: Option<EncryptedData>,
    pub phone: Option<EncryptedData>,
    pub billing_address: Option<EncryptedData>,
    pub shipping_address: Option<EncryptedData>,

    // Searchable hashes for encrypted fields
    pub email_hash: Option<String>,  // For duplicate detection
    pub phone_hash: Option<String>,  // For duplicate detection

    // Customer data
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub loyalty_tier: Option<String>,
    pub loyalty_points: i32,
    pub total_spent: rust_decimal::Decimal,
    pub visit_count: i32,
    pub avg_order_value: rust_decimal::Decimal,
    pub last_visit: Option<DateTime<Utc>>,
    pub preferred_location_id: Option<Uuid>,

    // Preferences and settings
    pub communication_preferences: serde_json::Value,
    pub marketing_consent: bool,
    pub marketing_consent_date: Option<DateTime<Utc>>,
    pub data_processing_consent: bool,
    pub data_processing_consent_date: Option<DateTime<Utc>>,

    // Metadata
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub metadata: serde_json::Value,

    // Status
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Customer data access control service
/// Implements data protection, encryption, and audit logging for customer PII
pub struct CustomerSecurityService {
    encryption: Arc<CustomerDataEncryption>,
    auditor: Arc<CustomerDataAuditor>,
}

impl CustomerSecurityService {
    pub fn new(
        encryption: Arc<CustomerDataEncryption>,
        auditor: Arc<CustomerDataAuditor>,
    ) -> Self {
        Self {
            encryption,
            auditor,
        }
    }

    /// Create a new secure customer profile with encrypted PII
    pub async fn create_secure_profile(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
        raw_profile: &RawCustomerProfile,
        ip_address: Option<std::net::IpAddr>,
        justification: Option<String>,
    ) -> Result<SecureCustomerProfile> {
        let customer_id = Uuid::new_v4();

        // Encrypt sensitive data
        let encrypted_email = if let Some(email) = &raw_profile.email {
            let encrypted = self.encryption.encrypt_email(email)?;
            Some(encrypted)
        } else {
            None
        };

        let encrypted_phone = if let Some(phone) = &raw_profile.phone {
            let encrypted = self.encryption.encrypt_phone(phone)?;
            Some(encrypted)
        } else {
            None
        };

        let encrypted_billing = if let Some(addr) = &raw_profile.billing_address {
            let encrypted = self.encryption.encrypt_address(addr)?;
            Some(encrypted)
        } else {
            None
        };

        let encrypted_shipping = if let Some(addr) = &raw_profile.shipping_address {
            let encrypted = self.encryption.encrypt_address(addr)?;
            Some(encrypted)
        } else {
            None
        };

        // Create searchable hashes
        let email_hash = raw_profile.email.as_ref()
            .map(|email| self.encryption.hash_email_for_search(email));

        let phone_hash = raw_profile.phone.as_ref()
            .map(|phone| {
                let mut hasher = Sha256::new();
                hasher.update(phone.as_bytes());
                format!("{:x}", hasher.finalize())
            });

        // Create secure profile
        let secure_profile = SecureCustomerProfile {
            id: customer_id,
            tenant_id,
            customer_number: raw_profile.customer_number.clone(),
            external_id: raw_profile.external_id.clone(),
            first_name: raw_profile.first_name.clone(),
            last_name: raw_profile.last_name.clone(),
            display_name: raw_profile.display_name.clone(),
            email: encrypted_email,
            phone: encrypted_phone,
            billing_address: encrypted_billing,
            shipping_address: encrypted_shipping,
            email_hash,
            phone_hash,
            date_of_birth: raw_profile.date_of_birth,
            loyalty_tier: raw_profile.loyalty_tier.clone(),
            loyalty_points: raw_profile.loyalty_points.unwrap_or(0),
            total_spent: raw_profile.total_spent.unwrap_or_default(),
            visit_count: 0,
            avg_order_value: rust_decimal::Decimal::ZERO,
            last_visit: None,
            preferred_location_id: raw_profile.preferred_location_id,
            communication_preferences: raw_profile.communication_preferences.clone().unwrap_or_default(),
            marketing_consent: raw_profile.marketing_consent.unwrap_or(false),
            marketing_consent_date: if raw_profile.marketing_consent.unwrap_or(false) {
                Some(Utc::now())
            } else {
                None
            },
            data_processing_consent: raw_profile.data_processing_consent.unwrap_or(false),
            data_processing_consent_date: if raw_profile.data_processing_consent.unwrap_or(false) {
                Some(Utc::now())
            } else {
                None
            },
            tags: raw_profile.tags.clone().unwrap_or_default(),
            notes: raw_profile.notes.clone(),
            metadata: raw_profile.metadata.clone().unwrap_or_default(),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        // Log customer data creation
        self.auditor.log_access(
            tenant_id,
            customer_id,
            user_id,
            CustomerDataAction::Create,
            None,
            DataClassification::Confidential,
            ip_address,
            None,
            justification.clone(),
        ).await?;

        // Log specific field encryptions
        if raw_profile.email.is_some() {
            self.auditor.log_modification(
                tenant_id,
                customer_id,
                user_id,
                CustomerDataAction::Create,
                "email".to_string(),
                None,
                raw_profile.email.as_deref(),
                DataClassification::Confidential,
                ip_address,
                justification.clone(),
            ).await?;
        }

        if raw_profile.phone.is_some() {
            self.auditor.log_modification(
                tenant_id,
                customer_id,
                user_id,
                CustomerDataAction::Create,
                "phone".to_string(),
                None,
                raw_profile.phone.as_deref(),
                DataClassification::Confidential,
                ip_address,
                justification,
            ).await?;
        }

        Ok(secure_profile)
    }

    /// Decrypt customer email for display
    pub async fn decrypt_customer_email(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        user_id: Uuid,
        encrypted_email: &EncryptedData,
        ip_address: Option<std::net::IpAddr>,
        justification: String,
    ) -> Result<String> {
        // Log access to encrypted data
        self.auditor.log_access(
            tenant_id,
            customer_id,
            user_id,
            CustomerDataAction::Decrypt,
            Some("email".to_string()),
            DataClassification::Confidential,
            ip_address,
            None,
            Some(justification),
        ).await?;

        // Decrypt the email
        self.encryption.decrypt(encrypted_email)
    }

    /// Decrypt customer phone for display
    pub async fn decrypt_customer_phone(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        user_id: Uuid,
        encrypted_phone: &EncryptedData,
        ip_address: Option<std::net::IpAddr>,
        justification: String,
    ) -> Result<String> {
        // Log access to encrypted data
        self.auditor.log_access(
            tenant_id,
            customer_id,
            user_id,
            CustomerDataAction::Decrypt,
            Some("phone".to_string()),
            DataClassification::Confidential,
            ip_address,
            None,
            Some(justification),
        ).await?;

        // Decrypt the phone
        self.encryption.decrypt(encrypted_phone)
    }

    /// Anonymize customer data for GDPR compliance
    pub async fn anonymize_customer_data(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        user_id: Uuid,
        mut profile: SecureCustomerProfile,
        ip_address: Option<std::net::IpAddr>,
        justification: String,
    ) -> Result<SecureCustomerProfile> {
        // Log anonymization action
        self.auditor.log_access(
            tenant_id,
            customer_id,
            user_id,
            CustomerDataAction::Anonymize,
            None,
            DataClassification::Restricted,
            ip_address,
            None,
            Some(justification),
        ).await?;

        // Anonymize names
        let (anon_first, anon_last) = DataAnonymizer::anonymize_name(
            &profile.first_name,
            &profile.last_name,
        );
        profile.first_name = anon_first;
        profile.last_name = anon_last;
        profile.display_name = Some("Anonymized User".to_string());

        // Remove encrypted PII data
        profile.email = None;
        profile.phone = None;
        profile.billing_address = None;
        profile.shipping_address = None;
        profile.email_hash = None;
        profile.phone_hash = None;

        // Remove personal identifiers
        profile.customer_number = None;
        profile.external_id = None;
        profile.date_of_birth = None;

        // Clear notes and metadata that might contain PII
        profile.notes = Some("Data anonymized per GDPR request".to_string());
        profile.metadata = serde_json::json!({
            "anonymized": true,
            "anonymized_at": Utc::now(),
            "anonymized_by": user_id
        });

        // Update timestamps
        profile.updated_at = Utc::now();

        Ok(profile)
    }

    /// Export customer data for GDPR data portability
    pub async fn export_customer_data(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        user_id: Uuid,
        profile: &SecureCustomerProfile,
        ip_address: Option<std::net::IpAddr>,
        justification: String,
    ) -> Result<CustomerDataExport> {
        // Log export action
        self.auditor.log_access(
            tenant_id,
            customer_id,
            user_id,
            CustomerDataAction::Export,
            None,
            DataClassification::Confidential,
            ip_address,
            None,
            Some(justification),
        ).await?;

        // Decrypt all encrypted fields for export
        let email = if let Some(encrypted_email) = &profile.email {
            Some(self.encryption.decrypt(encrypted_email)?)
        } else {
            None
        };

        let phone = if let Some(encrypted_phone) = &profile.phone {
            Some(self.encryption.decrypt(encrypted_phone)?)
        } else {
            None
        };

        let billing_address = if let Some(encrypted_addr) = &profile.billing_address {
            let decrypted = self.encryption.decrypt(encrypted_addr)?;
            Some(serde_json::from_str(&decrypted)?)
        } else {
            None
        };

        let shipping_address = if let Some(encrypted_addr) = &profile.shipping_address {
            let decrypted = self.encryption.decrypt(encrypted_addr)?;
            Some(serde_json::from_str(&decrypted)?)
        } else {
            None
        };

        Ok(CustomerDataExport {
            customer_id: profile.id,
            customer_number: profile.customer_number.clone(),
            first_name: profile.first_name.clone(),
            last_name: profile.last_name.clone(),
            email,
            phone,
            billing_address,
            shipping_address,
            date_of_birth: profile.date_of_birth,
            loyalty_tier: profile.loyalty_tier.clone(),
            loyalty_points: profile.loyalty_points,
            total_spent: profile.total_spent,
            visit_count: profile.visit_count,
            communication_preferences: profile.communication_preferences.clone(),
            marketing_consent: profile.marketing_consent,
            marketing_consent_date: profile.marketing_consent_date,
            data_processing_consent: profile.data_processing_consent,
            data_processing_consent_date: profile.data_processing_consent_date,
            tags: profile.tags.clone(),
            notes: profile.notes.clone(),
            created_at: profile.created_at,
            updated_at: profile.updated_at,
            exported_at: Utc::now(),
        })
    }
}

/// Raw customer profile data before encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCustomerProfile {
    pub customer_number: Option<String>,
    pub external_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub shipping_address: Option<serde_json::Value>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub loyalty_tier: Option<String>,
    pub loyalty_points: Option<i32>,
    pub total_spent: Option<rust_decimal::Decimal>,
    pub preferred_location_id: Option<Uuid>,
    pub communication_preferences: Option<serde_json::Value>,
    pub marketing_consent: Option<bool>,
    pub data_processing_consent: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub notes: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Customer data export for GDPR compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDataExport {
    pub customer_id: Uuid,
    pub customer_number: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub shipping_address: Option<serde_json::Value>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub loyalty_tier: Option<String>,
    pub loyalty_points: i32,
    pub total_spent: rust_decimal::Decimal,
    pub visit_count: i32,
    pub communication_preferences: serde_json::Value,
    pub marketing_consent: bool,
    pub marketing_consent_date: Option<DateTime<Utc>>,
    pub data_processing_consent: bool,
    pub data_processing_consent_date: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub exported_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use olympus_shared::security::{CustomerDataEncryption, CustomerDataAuditor};

    #[tokio::test]
    async fn test_secure_customer_profile_creation() {
        let master_key = [0u8; 32]; // Test key
        let encryption = Arc::new(
            CustomerDataEncryption::new(&master_key, "test-key".to_string()).unwrap()
        );
        let auditor = Arc::new(CustomerDataAuditor::new());

        let service = CustomerSecurityService::new(encryption, auditor);

        let raw_profile = RawCustomerProfile {
            customer_number: Some("CUST001".to_string()),
            external_id: None,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            display_name: Some("John Doe".to_string()),
            email: Some("john.doe@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            billing_address: Some(serde_json::json!({
                "street": "123 Main St",
                "city": "Anytown",
                "state": "CA",
                "postal_code": "12345"
            })),
            shipping_address: None,
            date_of_birth: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 15).unwrap()),
            loyalty_tier: Some("Gold".to_string()),
            loyalty_points: Some(1000),
            total_spent: Some(rust_decimal::Decimal::new(50000, 2)), // $500.00
            preferred_location_id: Some(Uuid::new_v4()),
            communication_preferences: Some(serde_json::json!({"email": true, "sms": false})),
            marketing_consent: Some(true),
            data_processing_consent: Some(true),
            tags: Some(vec!["vip".to_string(), "frequent".to_string()]),
            notes: Some("Preferred customer".to_string()),
            metadata: Some(serde_json::json!({"source": "website"})),
        };

        let secure_profile = service.create_secure_profile(
            Uuid::new_v4(),
            Uuid::new_v4(),
            &raw_profile,
            None,
            Some("Creating test customer".to_string()),
        ).await.unwrap();

        // Verify encryption worked
        assert!(secure_profile.email.is_some());
        assert!(secure_profile.phone.is_some());
        assert!(secure_profile.billing_address.is_some());
        assert!(secure_profile.email_hash.is_some());
        assert!(secure_profile.phone_hash.is_some());

        // Verify public data is preserved
        assert_eq!(secure_profile.first_name, "John");
        assert_eq!(secure_profile.last_name, "Doe");
        assert_eq!(secure_profile.loyalty_points, 1000);
    }

    #[tokio::test]
    async fn test_customer_data_anonymization() {
        let master_key = [0u8; 32];
        let encryption = Arc::new(
            CustomerDataEncryption::new(&master_key, "test-key".to_string()).unwrap()
        );
        let auditor = Arc::new(CustomerDataAuditor::new());

        let service = CustomerSecurityService::new(encryption, auditor);

        let mut profile = SecureCustomerProfile {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            customer_number: Some("CUST001".to_string()),
            external_id: Some("EXT001".to_string()),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            display_name: Some("John Doe".to_string()),
            email: Some(EncryptedData {
                data: "encrypted_email".to_string(),
                nonce: "nonce".to_string(),
                key_id: "test-key".to_string(),
                algorithm: "AES-256-GCM".to_string(),
                timestamp: 0,
            }),
            phone: Some(EncryptedData {
                data: "encrypted_phone".to_string(),
                nonce: "nonce".to_string(),
                key_id: "test-key".to_string(),
                algorithm: "AES-256-GCM".to_string(),
                timestamp: 0,
            }),
            billing_address: None,
            shipping_address: None,
            email_hash: Some("hash123".to_string()),
            phone_hash: Some("hash456".to_string()),
            date_of_birth: Some(chrono::NaiveDate::from_ymd_opt(1990, 1, 15).unwrap()),
            loyalty_tier: Some("Gold".to_string()),
            loyalty_points: 1000,
            total_spent: rust_decimal::Decimal::new(50000, 2),
            visit_count: 10,
            avg_order_value: rust_decimal::Decimal::new(5000, 2),
            last_visit: Some(Utc::now()),
            preferred_location_id: Some(Uuid::new_v4()),
            communication_preferences: serde_json::json!({"email": true}),
            marketing_consent: true,
            marketing_consent_date: Some(Utc::now()),
            data_processing_consent: true,
            data_processing_consent_date: Some(Utc::now()),
            tags: vec!["vip".to_string()],
            notes: Some("Important customer".to_string()),
            metadata: serde_json::json!({"source": "website"}),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let anonymized = service.anonymize_customer_data(
            profile.tenant_id,
            profile.id,
            Uuid::new_v4(),
            profile,
            None,
            "GDPR deletion request".to_string(),
        ).await.unwrap();

        // Verify anonymization
        assert_eq!(anonymized.first_name, "J***");
        assert_eq!(anonymized.last_name, "D***");
        assert_eq!(anonymized.display_name, Some("Anonymized User".to_string()));
        assert!(anonymized.email.is_none());
        assert!(anonymized.phone.is_none());
        assert!(anonymized.email_hash.is_none());
        assert!(anonymized.phone_hash.is_none());
        assert!(anonymized.customer_number.is_none());
        assert!(anonymized.external_id.is_none());
        assert!(anonymized.date_of_birth.is_none());
    }
}