use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::Database;
use crate::error::{AuthError, Result};
use crate::models::{User, Tenant, RefreshToken};

pub struct UserRepository {
    db: Arc<Database>,
}

impl UserRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn find_user_by_email(&self, email: &str, tenant_id: Uuid) -> Result<User> {
        let pool = self.db.pool();
        let row = sqlx::query!(
            "SELECT id, tenant_id, email, password_hash, first_name, last_name,
                    display_name, phone, avatar_url, roles, permissions,
                    is_active, email_verified, phone_verified, two_factor_enabled,
                    last_login, failed_login_attempts, locked_until,
                    password_changed_at, created_at, updated_at, deleted_at
             FROM users WHERE email = $1 AND tenant_id = $2 AND deleted_at IS NULL",
            email, tenant_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(AuthError::UserNotFound)?;

        Ok(User {
            id: row.id,
            tenant_id: row.tenant_id,
            email: row.email,
            password_hash: row.password_hash,
            first_name: row.first_name,
            last_name: row.last_name,
            display_name: row.display_name,
            phone: row.phone,
            avatar_url: row.avatar_url,
            roles: row.roles.unwrap_or_default(),
            permissions: row.permissions.unwrap_or_default(),
            is_active: row.is_active,
            email_verified: row.email_verified,
            phone_verified: row.phone_verified,
            two_factor_enabled: row.two_factor_enabled,
            last_login: row.last_login,
            failed_login_attempts: row.failed_login_attempts,
            locked_until: row.locked_until,
            password_changed_at: row.password_changed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        })
    }

    pub async fn find_tenant_by_slug(&self, slug: &str) -> Result<Tenant> {
        let pool = self.db.pool();
        let row = sqlx::query!(
            "SELECT id, slug, name, industry, subscription_tier,
                    is_active, settings, created_at, updated_at
             FROM tenants WHERE slug = $1",
            slug
        )
        .fetch_optional(pool)
        .await?
        .ok_or(AuthError::TenantNotFound)?;

        Ok(Tenant {
            id: row.id,
            slug: row.slug,
            name: row.name,
            industry: row.industry,
            subscription_tier: row.subscription_tier,
            is_active: row.is_active,
            settings: row.settings,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        let pool = self.db.pool();
        sqlx::query!(
            "INSERT INTO users (id, tenant_id, email, password_hash, first_name, last_name,
                               display_name, phone, avatar_url, roles, permissions,
                               is_active, email_verified, phone_verified, two_factor_enabled,
                               password_changed_at, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)",
            user.id, user.tenant_id, user.email, user.password_hash,
            user.first_name, user.last_name, user.display_name, user.phone, user.avatar_url,
            &user.roles, &user.permissions, user.is_active, user.email_verified,
            user.phone_verified, user.two_factor_enabled, user.password_changed_at,
            user.created_at, user.updated_at
        )
        .execute(pool)
        .await?;

        Ok(user.clone())
    }

    pub async fn find_user_by_id(&self, user_id: Uuid) -> Result<User> {
        let pool = self.db.pool();
        let row = sqlx::query!(
            "SELECT id, tenant_id, email, password_hash, first_name, last_name,
                    display_name, phone, avatar_url, roles, permissions,
                    is_active, email_verified, phone_verified, two_factor_enabled,
                    last_login, failed_login_attempts, locked_until,
                    password_changed_at, created_at, updated_at, deleted_at
             FROM users WHERE id = $1 AND deleted_at IS NULL",
            user_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or(AuthError::UserNotFound)?;

        Ok(User {
            id: row.id,
            tenant_id: row.tenant_id,
            email: row.email,
            password_hash: row.password_hash,
            first_name: row.first_name,
            last_name: row.last_name,
            display_name: row.display_name,
            phone: row.phone,
            avatar_url: row.avatar_url,
            roles: row.roles.unwrap_or_default(),
            permissions: row.permissions.unwrap_or_default(),
            is_active: row.is_active,
            email_verified: row.email_verified,
            phone_verified: row.phone_verified,
            two_factor_enabled: row.two_factor_enabled,
            last_login: row.last_login,
            failed_login_attempts: row.failed_login_attempts,
            locked_until: row.locked_until,
            password_changed_at: row.password_changed_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        })
    }

    pub async fn user_exists(&self, email: &str, tenant_id: Uuid) -> Result<bool> {
        let pool = self.db.pool();
        let row = sqlx::query!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND tenant_id = $2 AND deleted_at IS NULL)", email, tenant_id)
            .fetch_one(pool)
            .await?;
        Ok(row.exists.unwrap_or(false))
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        let pool = self.db.pool();
        sqlx::query!(
            "UPDATE users SET email = $2, first_name = $3, last_name = $4, last_login = $5,
                             failed_login_attempts = $6, locked_until = $7, updated_at = $8
             WHERE id = $1",
            user.id, user.email, user.first_name, user.last_name,
            user.last_login, user.failed_login_attempts, user.locked_until, Utc::now()
        )
        .execute(pool)
        .await?;
        Ok(user.clone())
    }

    pub async fn find_tenant_by_id(&self, tenant_id: Uuid) -> Result<Tenant> {
        let pool = self.db.pool();
        let row = sqlx::query!("SELECT id, slug, name, industry, subscription_tier, is_active, settings, created_at, updated_at FROM tenants WHERE id = $1", tenant_id)
            .fetch_optional(pool)
            .await?
            .ok_or(AuthError::TenantNotFound)?;

        Ok(Tenant {
            id: row.id,
            slug: row.slug,
            name: row.name,
            industry: row.industry,
            subscription_tier: row.subscription_tier,
            is_active: row.is_active,
            settings: row.settings,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    pub async fn store_refresh_token(&self, token: &RefreshToken) -> Result<()> {
        let pool = self.db.pool();
        sqlx::query!(
            "INSERT INTO refresh_tokens (id, token_hash, user_id, tenant_id, device_id, device_name, ip_address, user_agent, expires_at, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            token.id, token.token_hash, token.user_id, token.tenant_id,
            token.device_id, token.device_name, token.ip_address, token.user_agent,
            token.expires_at, token.created_at
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn find_refresh_token(&self, token_hash: &str) -> Result<RefreshToken> {
        let pool = self.db.pool();
        let row = sqlx::query!("SELECT id, token_hash, user_id, tenant_id, device_id, device_name, ip_address, user_agent, expires_at, revoked_at, created_at FROM refresh_tokens WHERE token_hash = $1", token_hash)
            .fetch_optional(pool)
            .await?
            .ok_or(AuthError::InvalidToken)?;

        Ok(RefreshToken {
            id: row.id,
            token_hash: row.token_hash,
            user_id: row.user_id,
            tenant_id: row.tenant_id,
            device_id: row.device_id,
            device_name: row.device_name,
            ip_address: row.ip_address,
            user_agent: row.user_agent,
            expires_at: row.expires_at,
            revoked_at: row.revoked_at,
            created_at: row.created_at,
        })
    }

    pub async fn revoke_refresh_token(&self, token_id: Uuid) -> Result<()> {
        let pool = self.db.pool();
        sqlx::query!("UPDATE refresh_tokens SET revoked_at = $2 WHERE id = $1", token_id, Utc::now())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<()> {
        let pool = self.db.pool();
        sqlx::query!("UPDATE refresh_tokens SET revoked_at = $2 WHERE user_id = $1 AND revoked_at IS NULL", user_id, Utc::now())
            .execute(pool)
            .await?;
        Ok(())
    }
}