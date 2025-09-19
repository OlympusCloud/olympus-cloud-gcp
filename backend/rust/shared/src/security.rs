use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{Engine as _, engine::general_purpose};
use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

use crate::error::{Result, Error};

/// Customer data encryption service
/// Handles field-level encryption for sensitive customer data (PII)
#[derive(Clone)]
pub struct CustomerDataEncryption {
    cipher: Aes256Gcm,
    key_id: String,
}

/// Encrypted data wrapper that includes metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub data: String,           // Base64 encoded encrypted data
    pub nonce: String,          // Base64 encoded nonce
    pub key_id: String,         // Key identifier for rotation
    pub algorithm: String,      // Encryption algorithm used
    pub timestamp: i64,         // When encrypted (for key rotation)
}

/// Data classification levels for customer information
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataClassification {
    Public,      // Name, company - no encryption needed
    Internal,    // Internal notes - basic encryption
    Confidential, // Contact info - standard encryption
    Restricted,  // Payment data, SSN - highest encryption + audit
}

impl CustomerDataEncryption {
    /// Create new encryption service with a master key
    pub fn new(master_key: &[u8], key_id: String) -> Result<Self> {
        if master_key.len() != 32 {
            return Err(Error::InvalidConfiguration(
                "Master key must be exactly 32 bytes".to_string()
            ));
        }

        let key = Key::<Aes256Gcm>::from_slice(master_key);
        let cipher = Aes256Gcm::new(key);

        Ok(Self {
            cipher,
            key_id,
        })
    }

    /// Encrypt sensitive customer data
    pub fn encrypt(&self, plaintext: &str, classification: DataClassification) -> Result<EncryptedData> {
        // Skip encryption for public data
        if classification == DataClassification::Public {
            return Ok(EncryptedData {
                data: general_purpose::STANDARD.encode(plaintext.as_bytes()),
                nonce: String::new(),
                key_id: "none".to_string(),
                algorithm: "none".to_string(),
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        // Generate random nonce for each encryption
        let mut nonce_bytes = [0u8; 12];
        thread_rng().fill(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the data
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| Error::EncryptionError(e.to_string()))?;

        Ok(EncryptedData {
            data: general_purpose::STANDARD.encode(&ciphertext),
            nonce: general_purpose::STANDARD.encode(nonce),
            key_id: self.key_id.clone(),
            algorithm: "AES-256-GCM".to_string(),
            timestamp: chrono::Utc::now().timestamp(),
        })
    }

    /// Decrypt customer data
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<String> {
        // Handle unencrypted public data
        if encrypted.algorithm == "none" {
            let plaintext = general_purpose::STANDARD
                .decode(&encrypted.data)
                .map_err(|e| Error::DecryptionError(e.to_string()))?;
            return Ok(String::from_utf8(plaintext)
                .map_err(|e| Error::DecryptionError(e.to_string()))?);
        }

        // Verify key ID matches
        if encrypted.key_id != self.key_id {
            return Err(Error::DecryptionError(
                format!("Key ID mismatch: expected {}, got {}", self.key_id, encrypted.key_id)
            ));
        }

        // Decode base64 data
        let ciphertext = general_purpose::STANDARD
            .decode(&encrypted.data)
            .map_err(|e| Error::DecryptionError(e.to_string()))?;

        let nonce_bytes = general_purpose::STANDARD
            .decode(&encrypted.nonce)
            .map_err(|e| Error::DecryptionError(e.to_string()))?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        // Decrypt the data
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| Error::DecryptionError(e.to_string()))?;

        String::from_utf8(plaintext)
            .map_err(|e| Error::DecryptionError(e.to_string()))
    }

    /// Encrypt customer email (confidential data)
    pub fn encrypt_email(&self, email: &str) -> Result<EncryptedData> {
        self.encrypt(email, DataClassification::Confidential)
    }

    /// Encrypt customer phone (confidential data)
    pub fn encrypt_phone(&self, phone: &str) -> Result<EncryptedData> {
        self.encrypt(phone, DataClassification::Confidential)
    }

    /// Encrypt customer address (confidential data)
    pub fn encrypt_address(&self, address: &serde_json::Value) -> Result<EncryptedData> {
        let address_str = serde_json::to_string(address)
            .map_err(|e| Error::SerializationError(e.to_string()))?;
        self.encrypt(&address_str, DataClassification::Confidential)
    }

    /// Create searchable hash for encrypted email (for duplicate detection)
    pub fn hash_email_for_search(&self, email: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(email.to_lowercase().as_bytes());
        hasher.update(self.key_id.as_bytes()); // Salt with key ID
        format!("{:x}", hasher.finalize())
    }
}

/// Data anonymization service for GDPR compliance
pub struct DataAnonymizer;

impl DataAnonymizer {
    /// Anonymize customer email (replace with hash)
    pub fn anonymize_email(email: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(email.as_bytes());
        format!("anonymized-{:.8}", format!("{:x}", hasher.finalize()))
    }

    /// Anonymize customer phone (keep country code, mask rest)
    pub fn anonymize_phone(phone: &str) -> String {
        if phone.len() <= 4 {
            return "***".to_string();
        }

        let visible_length = phone.len().min(3);
        let prefix = &phone[..visible_length];
        let masked = "*".repeat(phone.len() - visible_length);
        format!("{}{}", prefix, masked)
    }

    /// Anonymize customer name (keep first letter of first name)
    pub fn anonymize_name(first_name: &str, last_name: &str) -> (String, String) {
        let anon_first = if first_name.is_empty() {
            "Anonymous".to_string()
        } else {
            format!("{}***", first_name.chars().next().unwrap_or('A'))
        };

        let anon_last = if last_name.is_empty() {
            "User".to_string()
        } else {
            format!("{}***", last_name.chars().next().unwrap_or('U'))
        };

        (anon_first, anon_last)
    }

    /// Anonymize address (keep city/state, mask street)
    pub fn anonymize_address(address: &serde_json::Value) -> serde_json::Value {
        let mut anon_address = address.clone();

        // Mask street address
        if let Some(street) = anon_address.get_mut("street") {
            *street = serde_json::Value::String("*** [REDACTED] ***".to_string());
        }

        // Keep city and state for analytics but mask house number
        if let Some(street2) = anon_address.get_mut("street2") {
            *street2 = serde_json::Value::String("*** [REDACTED] ***".to_string());
        }

        // Remove postal code
        if anon_address.get("postal_code").is_some() {
            anon_address["postal_code"] = serde_json::Value::String("***".to_string());
        }

        anon_address
    }
}

/// Audit trail entry for customer data access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerDataAuditEntry {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub customer_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub action: CustomerDataAction,
    pub field_name: Option<String>,
    pub old_value_hash: Option<String>,  // Hash of old value for verification
    pub new_value_hash: Option<String>,  // Hash of new value for verification
    pub classification: DataClassification,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
    pub justification: Option<String>,   // Business justification for access
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Customer data actions that require auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomerDataAction {
    View,
    Create,
    Update,
    Delete,
    Export,
    Anonymize,
    Decrypt,     // Explicit decryption events
    BulkExport,  // Bulk operations need special attention
    ApiAccess,   // External API access to customer data
}

impl fmt::Display for CustomerDataAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomerDataAction::View => write!(f, "VIEW"),
            CustomerDataAction::Create => write!(f, "CREATE"),
            CustomerDataAction::Update => write!(f, "UPDATE"),
            CustomerDataAction::Delete => write!(f, "DELETE"),
            CustomerDataAction::Export => write!(f, "EXPORT"),
            CustomerDataAction::Anonymize => write!(f, "ANONYMIZE"),
            CustomerDataAction::Decrypt => write!(f, "DECRYPT"),
            CustomerDataAction::BulkExport => write!(f, "BULK_EXPORT"),
            CustomerDataAction::ApiAccess => write!(f, "API_ACCESS"),
        }
    }
}

/// Service for managing customer data audit trails
pub struct CustomerDataAuditor {
    // In a real implementation, this would have database connections
}

impl CustomerDataAuditor {
    pub fn new() -> Self {
        Self {}
    }

    /// Record customer data access for audit trail
    pub async fn log_access(
        &self,
        tenant_id: uuid::Uuid,
        customer_id: uuid::Uuid,
        user_id: uuid::Uuid,
        action: CustomerDataAction,
        field_name: Option<String>,
        classification: DataClassification,
        ip_address: Option<std::net::IpAddr>,
        user_agent: Option<String>,
        justification: Option<String>,
    ) -> Result<CustomerDataAuditEntry> {
        let entry = CustomerDataAuditEntry {
            id: uuid::Uuid::new_v4(),
            tenant_id,
            customer_id,
            user_id,
            action,
            field_name,
            old_value_hash: None,
            new_value_hash: None,
            classification,
            ip_address,
            user_agent,
            justification,
            timestamp: chrono::Utc::now(),
        };

        // TODO: In real implementation, persist to audit.customer_data_access table
        tracing::info!(
            tenant_id = %entry.tenant_id,
            customer_id = %entry.customer_id,
            user_id = %entry.user_id,
            action = %entry.action,
            field = ?entry.field_name,
            classification = ?entry.classification,
            "Customer data access logged"
        );

        Ok(entry)
    }

    /// Log data modification with before/after hashes
    pub async fn log_modification(
        &self,
        tenant_id: uuid::Uuid,
        customer_id: uuid::Uuid,
        user_id: uuid::Uuid,
        action: CustomerDataAction,
        field_name: String,
        old_value: Option<&str>,
        new_value: Option<&str>,
        classification: DataClassification,
        ip_address: Option<std::net::IpAddr>,
        justification: Option<String>,
    ) -> Result<CustomerDataAuditEntry> {
        let entry = CustomerDataAuditEntry {
            id: uuid::Uuid::new_v4(),
            tenant_id,
            customer_id,
            user_id,
            action,
            field_name: Some(field_name),
            old_value_hash: old_value.map(|v| self.hash_value(v)),
            new_value_hash: new_value.map(|v| self.hash_value(v)),
            classification,
            ip_address,
            user_agent: None,
            justification,
            timestamp: chrono::Utc::now(),
        };

        // TODO: In real implementation, persist to audit.customer_data_modifications table
        tracing::warn!(
            tenant_id = %entry.tenant_id,
            customer_id = %entry.customer_id,
            user_id = %entry.user_id,
            action = %entry.action,
            field = ?entry.field_name,
            classification = ?entry.classification,
            has_old_value = entry.old_value_hash.is_some(),
            has_new_value = entry.new_value_hash.is_some(),
            "Customer data modification logged"
        );

        Ok(entry)
    }

    /// Create hash of value for audit trail (without storing actual value)
    fn hash_value(&self, value: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value.as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string() // First 16 chars for space efficiency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer_data_encryption() {
        let master_key = [0u8; 32]; // In real usage, use a proper random key
        let encryption = CustomerDataEncryption::new(&master_key, "test-key-1".to_string()).unwrap();

        let email = "customer@example.com";
        let encrypted = encryption.encrypt_email(email).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();

        assert_eq!(email, decrypted);
        assert_eq!(encrypted.algorithm, "AES-256-GCM");
        assert_eq!(encrypted.key_id, "test-key-1");
    }

    #[test]
    fn test_data_anonymization() {
        let email = "customer@example.com";
        let anon_email = DataAnonymizer::anonymize_email(email);
        assert!(anon_email.starts_with("anonymized-"));
        assert_ne!(email, anon_email);

        let phone = "+1234567890";
        let anon_phone = DataAnonymizer::anonymize_phone(phone);
        assert!(anon_phone.starts_with("+12"));
        assert!(anon_phone.contains("*"));

        let (first, last) = DataAnonymizer::anonymize_name("John", "Doe");
        assert_eq!(first, "J***");
        assert_eq!(last, "D***");
    }

    #[test]
    fn test_public_data_no_encryption() {
        let master_key = [0u8; 32];
        let encryption = CustomerDataEncryption::new(&master_key, "test-key-1".to_string()).unwrap();

        let public_data = "Company Name";
        let encrypted = encryption.encrypt(public_data, DataClassification::Public).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();

        assert_eq!(public_data, decrypted);
        assert_eq!(encrypted.algorithm, "none");
        assert_eq!(encrypted.key_id, "none");
    }

    #[test]
    fn test_email_search_hash() {
        let master_key = [0u8; 32];
        let encryption = CustomerDataEncryption::new(&master_key, "test-key-1".to_string()).unwrap();

        let email1 = "customer@example.com";
        let email2 = "CUSTOMER@EXAMPLE.COM"; // Different case
        let email3 = "different@example.com";

        let hash1 = encryption.hash_email_for_search(email1);
        let hash2 = encryption.hash_email_for_search(email2);
        let hash3 = encryption.hash_email_for_search(email3);

        // Same email (case insensitive) should produce same hash
        assert_eq!(hash1, hash2);
        // Different email should produce different hash
        assert_ne!(hash1, hash3);
    }
}