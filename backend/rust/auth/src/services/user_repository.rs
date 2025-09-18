use std::sync::Arc;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::{Database, set_tenant_context};
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

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, tenant_id, email, password_hash, first_name, last_name,
                   display_name, phone, avatar_url,
                   roles as "roles!: Vec<String>",
                   permissions as "permissions!: Vec<String>",
                   is_active, email_verified, phone_verified, two_factor_enabled,
                   last_login, failed_login_attempts, locked_until,
                   password_changed_at, created_at, updated_at, deleted_at
            FROM users
            WHERE email = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            email,
            tenant_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AuthError::UserNotFound)?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, user_id: Uuid) -> Result<User> {
        let pool = self.db.pool();

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, tenant_id, email, password_hash, first_name, last_name,
                   display_name, phone, avatar_url,
                   roles as "roles!: Vec<String>",
                   permissions as "permissions!: Vec<String>",
                   is_active, email_verified, phone_verified, two_factor_enabled,
                   last_login, failed_login_attempts, locked_until,
                   password_changed_at, created_at, updated_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AuthError::UserNotFound)?;

        Ok(user)
    }

    pub async fn user_exists(&self, email: &str, tenant_id: Uuid) -> Result<bool> {
        let pool = self.db.pool();

        let exists = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users
                WHERE email = $1 AND tenant_id = $2 AND deleted_at IS NULL
            ) as "exists!"
            "#,
            email,
            tenant_id
        )
        .fetch_one(pool)
        .await?
        .exists;

        Ok(exists)
    }

    pub async fn create_user(&self, user: &User) -> Result<User> {
        let pool = self.db.pool();

        let created_user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (
                id, tenant_id, email, password_hash, first_name, last_name,
                display_name, phone, avatar_url, roles, permissions,
                is_active, email_verified, phone_verified, two_factor_enabled,
                password_changed_at, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            )
            RETURNING id, tenant_id, email, password_hash, first_name, last_name,
                      display_name, phone, avatar_url,
                      roles as "roles!: Vec<String>",
                      permissions as "permissions!: Vec<String>",
                      is_active, email_verified, phone_verified, two_factor_enabled,
                      last_login, failed_login_attempts, locked_until,
                      password_changed_at, created_at, updated_at, deleted_at
            "#,
            user.id,
            user.tenant_id,
            user.email,
            user.password_hash,
            user.first_name,
            user.last_name,
            user.display_name,
            user.phone,
            user.avatar_url,
            &user.roles,
            &user.permissions,
            user.is_active,
            user.email_verified,
            user.phone_verified,
            user.two_factor_enabled,
            user.password_changed_at,
            user.created_at,
            user.updated_at
        )
        .fetch_one(pool)
        .await?;

        Ok(created_user)
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        let pool = self.db.pool();

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET email = $2, first_name = $3, last_name = $4, display_name = $5,
                phone = $6, avatar_url = $7, roles = $8, permissions = $9,
                is_active = $10, email_verified = $11, phone_verified = $12,
                two_factor_enabled = $13, last_login = $14, failed_login_attempts = $15,
                locked_until = $16, password_changed_at = $17, updated_at = $18
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, tenant_id, email, password_hash, first_name, last_name,
                      display_name, phone, avatar_url,
                      roles as "roles!: Vec<String>",
                      permissions as "permissions!: Vec<String>",
                      is_active, email_verified, phone_verified, two_factor_enabled,
                      last_login, failed_login_attempts, locked_until,
                      password_changed_at, created_at, updated_at, deleted_at
            "#,
            user.id,
            user.email,
            user.first_name,
            user.last_name,
            user.display_name,
            user.phone,
            user.avatar_url,
            &user.roles,
            &user.permissions,
            user.is_active,
            user.email_verified,
            user.phone_verified,
            user.two_factor_enabled,
            user.last_login,
            user.failed_login_attempts,
            user.locked_until,
            user.password_changed_at,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_user)
    }

    pub async fn find_tenant_by_slug(&self, slug: &str) -> Result<Tenant> {
        let pool = self.db.pool();

        let tenant = sqlx::query_as!(
            Tenant,
            r#"
            SELECT id, slug, name, industry, subscription_tier,
                   is_active, settings, created_at, updated_at
            FROM tenants
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AuthError::TenantNotFound)?;

        Ok(tenant)
    }

    pub async fn find_tenant_by_id(&self, tenant_id: Uuid) -> Result<Tenant> {
        let pool = self.db.pool();

        let tenant = sqlx::query_as!(
            Tenant,
            r#"
            SELECT id, slug, name, industry, subscription_tier,
                   is_active, settings, created_at, updated_at
            FROM tenants
            WHERE id = $1
            "#,
            tenant_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AuthError::TenantNotFound)?;

        Ok(tenant)
    }

    pub async fn store_refresh_token(&self, token: &RefreshToken) -> Result<()> {
        let pool = self.db.pool();

        sqlx::query!(
            r#"
            INSERT INTO refresh_tokens (
                id, token_hash, user_id, tenant_id, device_id, device_name,
                ip_address, user_agent, expires_at, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            "#,
            token.id,
            token.token_hash,
            token.user_id,
            token.tenant_id,
            token.device_id,
            token.device_name,
            token.ip_address,
            token.user_agent,
            token.expires_at,
            token.created_at
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find_refresh_token(&self, token_hash: &str) -> Result<RefreshToken> {
        let pool = self.db.pool();

        let token = sqlx::query_as!(
            RefreshToken,
            r#"
            SELECT id, token_hash, user_id, tenant_id, device_id, device_name,
                   ip_address, user_agent, expires_at, revoked_at, created_at
            FROM refresh_tokens
            WHERE token_hash = $1
            "#,
            token_hash
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AuthError::InvalidToken)?;

        Ok(token)
    }

    pub async fn revoke_refresh_token(&self, token_id: Uuid) -> Result<()> {
        let pool = self.db.pool();

        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = $2
            WHERE id = $1
            "#,
            token_id,
            Utc::now()
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn revoke_all_user_tokens(&self, user_id: Uuid) -> Result<()> {
        let pool = self.db.pool();

        sqlx::query!(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = $2
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
            user_id,
            Utc::now()
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn cleanup_expired_tokens(&self) -> Result<u64> {
        let pool = self.db.pool();

        let result = sqlx::query!(
            r#"
            DELETE FROM refresh_tokens
            WHERE expires_at < $1 OR revoked_at IS NOT NULL
            "#,
            Utc::now()
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}