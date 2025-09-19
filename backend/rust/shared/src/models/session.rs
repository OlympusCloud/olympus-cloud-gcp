// ============================================================================
// OLYMPUS CLOUD - SESSION MODELS
// ============================================================================
// Module: shared/src/models/session.rs
// Description: Session management and authentication token models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{TenantScoped, ValidateEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use validator::ValidationError;

/// Session status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "session_status", rename_all = "lowercase")]
pub enum SessionStatus {
    Active,
    Expired,
    Revoked,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// Token type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
    EmailVerification,
    PasswordReset,
    Mfa,
}

/// MFA type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "mfa_type", rename_all = "lowercase")]
pub enum MfaType {
    Totp,
    Sms,
    Email,
    BackupCode,
}

/// User session entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub access_token_jti: Option<String>,
    pub device_fingerprint: Option<String>,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub ip_address: Option<std::net::IpAddr>,
    pub location: serde_json::Value,
    pub user_agent: Option<String>,
    pub status: SessionStatus,
    pub expires_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by: Option<Uuid>,
    pub revoke_reason: Option<String>,
}

impl UserSession {
    /// Create a new user session
    pub fn new(
        user_id: Uuid,
        refresh_token_hash: String,
        expires_at: DateTime<Utc>,
        ip_address: Option<std::net::IpAddr>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            refresh_token_hash,
            access_token_jti: None,
            device_fingerprint: None,
            device_name: None,
            device_type: None,
            ip_address,
            location: serde_json::json!({}),
            user_agent,
            status: SessionStatus::Active,
            expires_at,
            last_activity_at: now,
            created_at: now,
            revoked_at: None,
            revoked_by: None,
            revoke_reason: None,
        }
    }

    /// Check if the session is valid
    pub fn is_valid(&self) -> bool {
        self.status == SessionStatus::Active && self.expires_at > Utc::now()
    }

    /// Check if the session is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Check if the session is revoked
    pub fn is_revoked(&self) -> bool {
        self.status == SessionStatus::Revoked
    }

    /// Update last activity
    pub fn update_activity(&mut self) {
        self.last_activity_at = Utc::now();
    }

    /// Revoke the session
    pub fn revoke(&mut self, revoked_by: Option<Uuid>, reason: Option<String>) {
        self.status = SessionStatus::Revoked;
        self.revoked_at = Some(Utc::now());
        self.revoked_by = revoked_by;
        self.revoke_reason = reason;
    }

    /// Set device information
    pub fn set_device_info(
        &mut self,
        fingerprint: Option<String>,
        name: Option<String>,
        device_type: Option<String>,
    ) {
        self.device_fingerprint = fingerprint;
        self.device_name = name;
        self.device_type = device_type;
    }

    /// Set location information
    pub fn set_location(&mut self, location: serde_json::Value) {
        self.location = location;
    }

    /// Get device summary
    pub fn device_summary(&self) -> String {
        match (&self.device_name, &self.device_type) {
            (Some(name), Some(device_type)) => format!("{} ({})", name, device_type),
            (Some(name), None) => name.clone(),
            (None, Some(device_type)) => device_type.clone(),
            (None, None) => "Unknown Device".to_string(),
        }
    }
}

impl ValidateEntity for UserSession {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // Refresh token hash must not be empty
        if self.refresh_token_hash.trim().is_empty() {
            return Err(ValidationError::new("empty_refresh_token"));
        }

        // Expires at must be in the future for active sessions
        if self.status == SessionStatus::Active && self.expires_at <= Utc::now() {
            return Err(ValidationError::new("invalid_expiry_time"));
        }

        Ok(())
    }
}

/// Email verification token
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub email: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl EmailVerificationToken {
    /// Create a new email verification token
    pub fn new(user_id: Uuid, token: String, email: String, expires_in_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            email,
            expires_at: now + chrono::Duration::hours(expires_in_hours),
            used_at: None,
            created_at: now,
        }
    }

    /// Check if the token is valid
    pub fn is_valid(&self) -> bool {
        self.used_at.is_none() && self.expires_at > Utc::now()
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Check if the token is used
    pub fn is_used(&self) -> bool {
        self.used_at.is_some()
    }

    /// Mark the token as used
    pub fn mark_used(&mut self) {
        self.used_at = Some(Utc::now());
    }
}

/// Password reset token
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PasswordResetToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl PasswordResetToken {
    /// Create a new password reset token
    pub fn new(user_id: Uuid, token: String, expires_in_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            token,
            expires_at: now + chrono::Duration::hours(expires_in_hours),
            used_at: None,
            created_at: now,
        }
    }

    /// Check if the token is valid
    pub fn is_valid(&self) -> bool {
        self.used_at.is_none() && self.expires_at > Utc::now()
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    /// Check if the token is used
    pub fn is_used(&self) -> bool {
        self.used_at.is_some()
    }

    /// Mark the token as used
    pub fn mark_used(&mut self) {
        self.used_at = Some(Utc::now());
    }
}

/// Multi-factor authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserMfa {
    pub id: Uuid,
    pub user_id: Uuid,
    pub mfa_type: MfaType,
    pub secret_key: Option<String>,
    pub phone_number: Option<String>,
    pub backup_codes: Vec<String>,
    pub is_enabled: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserMfa {
    /// Create a new MFA configuration
    pub fn new(user_id: Uuid, mfa_type: MfaType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            mfa_type,
            secret_key: None,
            phone_number: None,
            backup_codes: vec![],
            is_enabled: false,
            verified_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Enable MFA
    pub fn enable(&mut self) {
        self.is_enabled = true;
        self.verified_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Disable MFA
    pub fn disable(&mut self) {
        self.is_enabled = false;
        self.updated_at = Utc::now();
    }

    /// Set secret key for TOTP
    pub fn set_secret_key(&mut self, secret: String) {
        self.secret_key = Some(secret);
        self.updated_at = Utc::now();
    }

    /// Set phone number for SMS
    pub fn set_phone_number(&mut self, phone: String) {
        self.phone_number = Some(phone);
        self.updated_at = Utc::now();
    }

    /// Generate backup codes
    pub fn generate_backup_codes(&mut self, count: usize) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        self.backup_codes = (0..count)
            .map(|_| {
                format!("{:04}-{:04}-{:04}",
                    rng.gen_range(1000..9999),
                    rng.gen_range(1000..9999),
                    rng.gen_range(1000..9999)
                )
            })
            .collect();

        self.updated_at = Utc::now();
    }

    /// Use a backup code
    pub fn use_backup_code(&mut self, code: &str) -> bool {
        if let Some(index) = self.backup_codes.iter().position(|c| c == code) {
            self.backup_codes.remove(index);
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }
}

/// API key for service-to-service authentication
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub key_prefix: String,
    pub permissions: Vec<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ApiKey {
    /// Create a new API key
    pub fn new(
        user_id: Uuid,
        name: String,
        key_hash: String,
        key_prefix: String,
        expires_at: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            key_hash,
            key_prefix,
            permissions: vec![],
            scopes: vec![],
            expires_at,
            last_used_at: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if the API key is valid
    pub fn is_valid(&self) -> bool {
        self.is_active && self.expires_at.map(|exp| exp > Utc::now()).unwrap_or(true)
    }

    /// Check if the API key is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|exp| exp <= Utc::now()).unwrap_or(false)
    }

    /// Record usage
    pub fn record_usage(&mut self) {
        self.last_used_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Revoke the API key
    pub fn revoke(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Add permission
    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
            self.updated_at = Utc::now();
        }
    }

    /// Remove permission
    pub fn remove_permission(&mut self, permission: &str) {
        if let Some(index) = self.permissions.iter().position(|p| p == permission) {
            self.permissions.remove(index);
            self.updated_at = Utc::now();
        }
    }

    /// Check if has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}

/// Request to create a new session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub device_fingerprint: Option<String>,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub ip_address: Option<std::net::IpAddr>,
    pub user_agent: Option<String>,
    pub location: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let user_id = Uuid::new_v4();
        let token_hash = "hashed_refresh_token".to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(24);

        let session = UserSession::new(
            user_id,
            token_hash.clone(),
            expires_at,
            None,
            None,
        );

        assert_eq!(session.user_id, user_id);
        assert_eq!(session.refresh_token_hash, token_hash);
        assert_eq!(session.status, SessionStatus::Active);
        assert!(session.is_valid());
        assert!(!session.is_expired());
        assert!(!session.is_revoked());
    }

    #[test]
    fn test_session_revocation() {
        let mut session = UserSession::new(
            Uuid::new_v4(),
            "token".to_string(),
            Utc::now() + chrono::Duration::hours(24),
            None,
            None,
        );

        let revoker_id = Uuid::new_v4();
        session.revoke(Some(revoker_id), Some("Manual revocation".to_string()));

        assert!(!session.is_valid());
        assert!(session.is_revoked());
        assert_eq!(session.revoked_by, Some(revoker_id));
        assert_eq!(session.revoke_reason, Some("Manual revocation".to_string()));
    }

    #[test]
    fn test_email_verification_token() {
        let user_id = Uuid::new_v4();
        let token = "verification_token".to_string();
        let email = "test@example.com".to_string();

        let mut verification_token = EmailVerificationToken::new(
            user_id,
            token.clone(),
            email.clone(),
            24, // 24 hours
        );

        assert!(verification_token.is_valid());
        assert!(!verification_token.is_used());
        assert!(!verification_token.is_expired());

        verification_token.mark_used();
        assert!(!verification_token.is_valid());
        assert!(verification_token.is_used());
    }

    #[test]
    fn test_mfa_configuration() {
        let user_id = Uuid::new_v4();
        let mut mfa = UserMfa::new(user_id, MfaType::Totp);

        assert!(!mfa.is_enabled);
        assert!(mfa.verified_at.is_none());

        mfa.set_secret_key("JBSWY3DPEHPK3PXP".to_string());
        mfa.enable();

        assert!(mfa.is_enabled);
        assert!(mfa.verified_at.is_some());
        assert!(mfa.secret_key.is_some());
    }

    #[test]
    fn test_backup_codes() {
        let mut mfa = UserMfa::new(Uuid::new_v4(), MfaType::Totp);

        mfa.generate_backup_codes(10);
        assert_eq!(mfa.backup_codes.len(), 10);

        let code = mfa.backup_codes[0].clone();
        assert!(mfa.use_backup_code(&code));
        assert_eq!(mfa.backup_codes.len(), 9);

        // Can't use the same code twice
        assert!(!mfa.use_backup_code(&code));
    }

    #[test]
    fn test_api_key() {
        let user_id = Uuid::new_v4();
        let mut api_key = ApiKey::new(
            user_id,
            "Test API Key".to_string(),
            "hashed_key".to_string(),
            "ak_test".to_string(),
            Some(Utc::now() + chrono::Duration::days(30)),
        );

        assert!(api_key.is_valid());
        assert!(!api_key.is_expired());

        api_key.add_permission("read:users".to_string());
        assert!(api_key.has_permission("read:users"));
        assert!(!api_key.has_permission("write:users"));

        api_key.record_usage();
        assert!(api_key.last_used_at.is_some());

        api_key.revoke();
        assert!(!api_key.is_valid());
    }
}