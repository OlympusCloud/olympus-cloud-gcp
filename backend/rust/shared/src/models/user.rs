// ============================================================================
// OLYMPUS CLOUD - USER MODELS
// ============================================================================
// Module: shared/src/models/user.rs
// Description: User entity and related models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{AuditFields, SoftDelete, TenantScoped, ValidateEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use validator::{Validate, ValidationError};

/// User status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_status", rename_all = "UPPERCASE")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Deleted,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// User role enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    SuperAdmin,
    TenantAdmin,
    LocationAdmin,
    Manager,
    Employee,
    Customer,
    Guest,
}

impl UserRole {
    /// Check if this role has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::SuperAdmin | Self::TenantAdmin | Self::LocationAdmin)
    }

    /// Check if this role can manage users
    pub fn can_manage_users(&self) -> bool {
        matches!(self, Self::SuperAdmin | Self::TenantAdmin)
    }

    /// Check if this role can access admin features
    pub fn can_access_admin(&self) -> bool {
        matches!(
            self,
            Self::SuperAdmin | Self::TenantAdmin | Self::LocationAdmin | Self::Manager
        )
    }
}

/// Main user entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub password_hash: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub status: UserStatus,
    pub email_verified: bool,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub preferences: serde_json::Value,
    pub metadata: serde_json::Value,
    #[sqlx(flatten)]
    pub audit_fields: AuditFields,
}

impl User {
    /// Create a new user with default values
    pub fn new(tenant_id: Uuid, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            email,
            username: None,
            password_hash,
            first_name: None,
            last_name: None,
            display_name: None,
            avatar_url: None,
            phone: None,
            status: UserStatus::Active,
            email_verified: false,
            email_verified_at: None,
            last_login_at: None,
            failed_login_attempts: 0,
            locked_until: None,
            password_changed_at: Some(now),
            preferences: serde_json::json!({}),
            metadata: serde_json::json!({}),
            audit_fields: AuditFields {
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        }
    }

    /// Get the user's full name
    pub fn full_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.email.clone(),
        }
    }

    /// Get the user's display name or fallback to full name
    pub fn display_name(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| self.full_name())
    }

    /// Check if the user is currently locked
    pub fn is_locked(&self) -> bool {
        self.locked_until
            .map(|locked_until| locked_until > Utc::now())
            .unwrap_or(false)
    }

    /// Check if the user can login
    pub fn can_login(&self) -> bool {
        self.status == UserStatus::Active && !self.is_locked()
    }

    /// Record a failed login attempt
    pub fn record_failed_login(&mut self) {
        self.failed_login_attempts += 1;

        // Lock account after 5 failed attempts for 15 minutes
        if self.failed_login_attempts >= 5 {
            self.locked_until = Some(Utc::now() + chrono::Duration::minutes(15));
        }

        self.audit_fields.updated_at = Utc::now();
    }

    /// Record a successful login
    pub fn record_successful_login(&mut self) {
        self.failed_login_attempts = 0;
        self.locked_until = None;
        self.last_login_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
    }

    /// Update password hash
    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.password_changed_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
    }

    /// Verify email
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.email_verified_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
    }
}

impl TenantScoped for User {
    fn tenant_id(&self) -> Uuid {
        self.tenant_id
    }
}

impl SoftDelete for User {
    fn is_deleted(&self) -> bool {
        self.audit_fields.deleted_at.is_some()
    }

    fn delete(&mut self) {
        self.audit_fields.deleted_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
        self.status = UserStatus::Deleted;
    }

    fn restore(&mut self) {
        self.audit_fields.deleted_at = None;
        self.audit_fields.updated_at = Utc::now();
        self.status = UserStatus::Active;
    }
}

impl ValidateEntity for User {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // Email validation
        if !self.email.contains('@') {
            return Err(ValidationError::new("invalid_email"));
        }

        // Username validation (if provided)
        if let Some(username) = &self.username {
            if username.len() < 3 || username.len() > 50 {
                return Err(ValidationError::new("invalid_username_length"));
            }
        }

        // Phone validation (if provided)
        if let Some(phone) = &self.phone {
            if phone.is_empty() {
                return Err(ValidationError::new("invalid_phone"));
            }
        }

        Ok(())
    }
}

/// Request model for creating a new user
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(max = 100))]
    pub first_name: Option<String>,

    #[validate(length(max = 100))]
    pub last_name: Option<String>,

    #[validate(length(max = 200))]
    pub display_name: Option<String>,

    #[validate(url)]
    pub avatar_url: Option<String>,

    #[validate(phone)]
    pub phone: Option<String>,

    pub preferences: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Request model for updating a user
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 3, max = 50))]
    pub username: Option<String>,

    #[validate(length(max = 100))]
    pub first_name: Option<String>,

    #[validate(length(max = 100))]
    pub last_name: Option<String>,

    #[validate(length(max = 200))]
    pub display_name: Option<String>,

    #[validate(url)]
    pub avatar_url: Option<String>,

    #[validate(phone)]
    pub phone: Option<String>,

    pub status: Option<UserStatus>,
    pub preferences: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// User profile response (without sensitive fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub email: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub email_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub preferences: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            display_name: user.display_name,
            avatar_url: user.avatar_url,
            phone: user.phone,
            email_verified: user.email_verified,
            last_login_at: user.last_login_at,
            preferences: user.preferences,
            created_at: user.audit_fields.created_at,
        }
    }
}

/// User summary for lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub status: UserStatus,
    pub email_verified: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserSummary {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name(),
            status: user.status,
            email_verified: user.email_verified,
            last_login_at: user.last_login_at,
            created_at: user.audit_fields.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let tenant_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();

        let user = User::new(tenant_id, email.clone(), password_hash.clone());

        assert_eq!(user.tenant_id, tenant_id);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
        assert_eq!(user.status, UserStatus::Active);
        assert!(!user.email_verified);
        assert_eq!(user.failed_login_attempts, 0);
    }

    #[test]
    fn test_user_full_name() {
        let mut user = User::new(Uuid::new_v4(), "test@example.com".to_string(), "hash".to_string());

        // No name provided
        assert_eq!(user.full_name(), "test@example.com");

        // First name only
        user.first_name = Some("John".to_string());
        assert_eq!(user.full_name(), "John");

        // Both names
        user.last_name = Some("Doe".to_string());
        assert_eq!(user.full_name(), "John Doe");
    }

    #[test]
    fn test_user_lock_mechanism() {
        let mut user = User::new(Uuid::new_v4(), "test@example.com".to_string(), "hash".to_string());

        assert!(user.can_login());
        assert!(!user.is_locked());

        // Record failed attempts
        for _ in 0..4 {
            user.record_failed_login();
            assert!(!user.is_locked()); // Not locked yet
        }

        // 5th attempt should lock the account
        user.record_failed_login();
        assert!(user.is_locked());
        assert!(!user.can_login());

        // Successful login should unlock
        user.record_successful_login();
        assert!(!user.is_locked());
        assert!(user.can_login());
        assert_eq!(user.failed_login_attempts, 0);
    }

    #[test]
    fn test_user_role_permissions() {
        assert!(UserRole::SuperAdmin.is_admin());
        assert!(UserRole::TenantAdmin.is_admin());
        assert!(UserRole::LocationAdmin.is_admin());
        assert!(!UserRole::Manager.is_admin());

        assert!(UserRole::SuperAdmin.can_manage_users());
        assert!(UserRole::TenantAdmin.can_manage_users());
        assert!(!UserRole::LocationAdmin.can_manage_users());

        assert!(UserRole::Manager.can_access_admin());
        assert!(!UserRole::Employee.can_access_admin());
    }

    #[test]
    fn test_user_validation() {
        let user = User::new(
            Uuid::new_v4(),
            "invalid-email".to_string(), // Invalid email
            "hash".to_string(),
        );

        assert!(user.validate().is_err());

        let user = User::new(
            Uuid::new_v4(),
            "valid@example.com".to_string(),
            "hash".to_string(),
        );

        assert!(user.validate().is_ok());
    }
}