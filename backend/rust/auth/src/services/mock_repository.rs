use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::DbPool;
use crate::error::{AuthError, Result};
use crate::models::{User, Tenant, RefreshToken};

pub struct UserRepository {
    _db: Arc<DbPool>,
}

impl UserRepository {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { _db: db }
    }

    pub async fn find_user_by_email(&self, email: &str, _tenant_id: Uuid) -> Result<User> {
        if email == "test@example.com" {
            Ok(User {
                id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                email: email.to_string(),
                password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$hash".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                display_name: None,
                phone: None,
                avatar_url: None,
                roles: vec!["user".to_string()],
                permissions: vec![],
                is_active: true,
                email_verified: false,
                phone_verified: false,
                two_factor_enabled: false,
                last_login: None,
                failed_login_attempts: 0,
                locked_until: None,
                password_changed_at: Some(Utc::now()),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
            })
        } else {
            Err(AuthError::UserNotFound)
        }
    }

    pub async fn find_user_by_id(&self, _user_id: Uuid) -> Result<User> {
        Ok(User {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$test$hash".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            display_name: None,
            phone: None,
            avatar_url: None,
            roles: vec!["user".to_string()],
            permissions: vec![],
            is_active: true,
            email_verified: false,
            phone_verified: false,
            two_factor_enabled: false,
            last_login: None,
            failed_login_attempts: 0,
            locked_until: None,
            password_changed_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }

    pub async fn user_exists(&self, email: &str, _tenant_id: Uuid) -> Result<bool> {
        Ok(email == "existing@example.com")
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        Ok(user.clone())
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        Ok(user.clone())
    }

    pub async fn find_tenant_by_slug(&self, slug: &str) -> Result<Tenant> {
        if slug == "test-tenant" {
            Ok(Tenant {
                id: Uuid::new_v4(),
                slug: slug.to_string(),
                name: "Test Tenant".to_string(),
                industry: "Technology".to_string(),
                subscription_tier: "basic".to_string(),
                is_active: true,
                settings: serde_json::json!({}),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
        } else {
            Err(AuthError::TenantNotFound)
        }
    }

    pub async fn find_tenant_by_id(&self, _tenant_id: Uuid) -> Result<Tenant> {
        Ok(Tenant {
            id: Uuid::new_v4(),
            slug: "test-tenant".to_string(),
            name: "Test Tenant".to_string(),
            industry: "Technology".to_string(),
            subscription_tier: "basic".to_string(),
            is_active: true,
            settings: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn store_refresh_token(&self, _token: &RefreshToken) -> Result<()> {
        Ok(())
    }

    pub async fn find_refresh_token(&self, _token_hash: &str) -> Result<RefreshToken> {
        Ok(RefreshToken {
            id: Uuid::new_v4(),
            token_hash: "hash".to_string(),
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            device_id: None,
            device_name: None,
            ip_address: "127.0.0.1".to_string(),
            user_agent: "test".to_string(),
            expires_at: Utc::now() + chrono::Duration::days(30),
            revoked_at: None,
            created_at: Utc::now(),
        })
    }

    pub async fn revoke_refresh_token(&self, _token_id: Uuid) -> Result<()> {
        Ok(())
    }

    pub async fn revoke_all_user_tokens(&self, _user_id: Uuid) -> Result<()> {
        Ok(())
    }

    pub async fn find_active_refresh_tokens(&self, _user_id: Uuid) -> Result<Vec<RefreshToken>> {
        // Mock implementation - return a couple of sample sessions
        Ok(vec![
            RefreshToken {
                id: Uuid::new_v4(),
                token_hash: "hash1".to_string(),
                user_id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                device_id: Some("device1".to_string()),
                device_name: Some("iPhone 14".to_string()),
                ip_address: "192.168.1.100".to_string(),
                user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X)".to_string(),
                expires_at: Utc::now() + chrono::Duration::days(30),
                revoked_at: None,
                created_at: Utc::now() - chrono::Duration::hours(2),
            },
            RefreshToken {
                id: Uuid::new_v4(),
                token_hash: "hash2".to_string(),
                user_id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                device_id: Some("device2".to_string()),
                device_name: Some("MacBook Pro".to_string()),
                ip_address: "192.168.1.101".to_string(),
                user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)".to_string(),
                expires_at: Utc::now() + chrono::Duration::days(30),
                revoked_at: None,
                created_at: Utc::now() - chrono::Duration::days(1),
            },
        ])
    }

    pub async fn find_refresh_token_by_id(&self, token_id: Uuid) -> Result<RefreshToken> {
        Ok(RefreshToken {
            id: token_id,
            token_hash: "hash".to_string(),
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            device_id: Some("device1".to_string()),
            device_name: Some("Test Device".to_string()),
            ip_address: "127.0.0.1".to_string(),
            user_agent: "test".to_string(),
            expires_at: Utc::now() + chrono::Duration::days(30),
            revoked_at: None,
            created_at: Utc::now(),
        })
    }

    pub async fn revoke_all_user_tokens_except(&self, _user_id: Uuid, _except_token_id: Uuid) -> Result<()> {
        Ok(())
    }
}