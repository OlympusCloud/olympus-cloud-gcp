use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{rand_core::OsRng, SaltString}};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedField {
    pub encrypted_data: String,
    pub nonce: String,
}

pub struct CustomerDataSecurity {
    cipher: Aes256Gcm,
}

impl CustomerDataSecurity {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn encrypt_pii(&self, data: &str) -> Result<EncryptedField, String> {
        let nonce = Nonce::from_slice(b"unique nonce"); // In production, use random nonce
        
        match self.cipher.encrypt(nonce, data.as_bytes()) {
            Ok(ciphertext) => Ok(EncryptedField {
                encrypted_data: base64::encode(ciphertext),
                nonce: base64::encode(nonce),
            }),
            Err(_) => Err("Encryption failed".to_string()),
        }
    }

    pub fn decrypt_pii(&self, encrypted: &EncryptedField) -> Result<String, String> {
        let ciphertext = base64::decode(&encrypted.encrypted_data)
            .map_err(|_| "Invalid encrypted data")?;
        let nonce_bytes = base64::decode(&encrypted.nonce)
            .map_err(|_| "Invalid nonce")?;
        let nonce = Nonce::from_slice(&nonce_bytes);

        match self.cipher.decrypt(nonce, ciphertext.as_ref()) {
            Ok(plaintext) => String::from_utf8(plaintext)
                .map_err(|_| "Invalid UTF-8".to_string()),
            Err(_) => Err("Decryption failed".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccessLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub customer_id: Uuid,
    pub action: DataAction,
    pub field_accessed: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataAction {
    Read,
    Write,
    Delete,
    Export,
}

pub struct DataAuditLogger;

impl DataAuditLogger {
    pub fn log_access(
        user_id: Uuid,
        customer_id: Uuid,
        action: DataAction,
        field: &str,
        ip: Option<String>,
        user_agent: Option<String>,
    ) -> DataAccessLog {
        DataAccessLog {
            id: Uuid::new_v4(),
            user_id,
            customer_id,
            action,
            field_accessed: field.to_string(),
            timestamp: chrono::Utc::now(),
            ip_address: ip,
            user_agent,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccessControl {
    permissions: HashMap<String, Vec<String>>, // role -> permissions
}

impl AccessControl {
    pub fn new() -> Self {
        let mut permissions = HashMap::new();
        
        // Define role-based permissions
        permissions.insert("admin".to_string(), vec![
            "customer.read".to_string(),
            "customer.write".to_string(),
            "customer.delete".to_string(),
            "customer.export".to_string(),
            "customer.pii.read".to_string(),
        ]);
        
        permissions.insert("manager".to_string(), vec![
            "customer.read".to_string(),
            "customer.write".to_string(),
            "customer.pii.read".to_string(),
        ]);
        
        permissions.insert("staff".to_string(), vec![
            "customer.read".to_string(),
        ]);

        Self { permissions }
    }

    pub fn check_permission(&self, role: &str, permission: &str) -> bool {
        self.permissions
            .get(role)
            .map(|perms| perms.contains(&permission.to_string()))
            .unwrap_or(false)
    }

    pub fn can_access_pii(&self, role: &str) -> bool {
        self.check_permission(role, "customer.pii.read")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureCustomerData {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: EncryptedField,
    pub phone: Option<EncryptedField>,
    pub first_name: EncryptedField,
    pub last_name: EncryptedField,
    pub date_of_birth: Option<EncryptedField>,
    pub address: Option<EncryptedField>,
    // Non-PII fields remain unencrypted
    pub customer_number: String,
    pub loyalty_tier: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl SecureCustomerData {
    pub fn mask_for_display(&self, role: &str, access_control: &AccessControl) -> serde_json::Value {
        let can_see_pii = access_control.can_access_pii(role);
        
        serde_json::json!({
            "id": self.id,
            "tenant_id": self.tenant_id,
            "email": if can_see_pii { "[ENCRYPTED]" } else { "***@***.***" },
            "phone": if can_see_pii { "[ENCRYPTED]" } else { "***-***-****" },
            "first_name": if can_see_pii { "[ENCRYPTED]" } else { "***" },
            "last_name": if can_see_pii { "[ENCRYPTED]" } else { "***" },
            "customer_number": self.customer_number,
            "loyalty_tier": self.loyalty_tier,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
        })
    }
}