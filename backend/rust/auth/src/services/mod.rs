pub mod jwt;
pub mod password;
pub mod user_repository;

use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc};
use olympus_shared::database::Database;
use olympus_shared::events::EventPublisher;
use crate::error::{AuthError, Result};
use crate::models::*;
use jwt::JwtService;
use password::PasswordService;
use user_repository::UserRepository;

pub struct AuthService {
    db: Arc<Database>,
    jwt: JwtService,
    password: PasswordService,
    user_repo: UserRepository,
    event_publisher: Option<Arc<tokio::sync::Mutex<EventPublisher>>>,
}

impl AuthService {
    pub fn new(
        db: Arc<Database>,
        jwt_secret: &[u8],
        event_publisher: Option<Arc<tokio::sync::Mutex<EventPublisher>>>,
    ) -> Self {
        Self {
            db: db.clone(),
            jwt: JwtService::new(jwt_secret),
            password: PasswordService::new(),
            user_repo: UserRepository::new(db),
            event_publisher,
        }
    }

    pub async fn login(&self, request: LoginRequest, ip_address: String, user_agent: String) -> Result<TokenResponse> {
        // Find tenant
        let tenant = self.user_repo.find_tenant_by_slug(&request.tenant_slug).await?;
        if !tenant.is_active {
            return Err(AuthError::TenantInactive);
        }

        // Find user
        let mut user = self.user_repo.find_user_by_email(&request.email, tenant.id).await?;

        // Check if account is locked
        if user.is_locked() {
            return Err(AuthError::AccountLocked);
        }

        // Verify password
        if !self.password.verify_password(&request.password, &user.password_hash)? {
            // Increment failed attempts
            user.failed_login_attempts += 1;
            if user.failed_login_attempts >= 5 {
                user.locked_until = Some(Utc::now() + Duration::minutes(30));
            }
            self.user_repo.update_user(&user).await?;
            return Err(AuthError::InvalidCredentials);
        }

        // Check if account is active
        if !user.is_active {
            return Err(AuthError::AccountInactive);
        }

        // Reset failed attempts and update last login
        user.failed_login_attempts = 0;
        user.locked_until = None;
        user.last_login = Some(Utc::now());
        self.user_repo.update_user(&user).await?;

        // Generate tokens
        let session_id = Uuid::new_v4();
        let access_token = self.jwt.generate_access_token(&user, tenant.id, session_id)?;
        let refresh_token_str = self.jwt.generate_refresh_token()?;

        // Store refresh token
        let refresh_token = RefreshToken {
            id: Uuid::new_v4(),
            token_hash: self.password.hash_token(&refresh_token_str)?,
            user_id: user.id,
            tenant_id: tenant.id,
            device_id: request.device_id,
            device_name: request.device_name,
            ip_address,
            user_agent,
            expires_at: Utc::now() + Duration::days(30),
            revoked_at: None,
            created_at: Utc::now(),
        };
        self.user_repo.store_refresh_token(&refresh_token).await?;

        // Publish login event
        if let Some(publisher) = &self.event_publisher {
            let event = olympus_shared::events::DomainEvent::builder(
                "user.logged_in".to_string(),
                user.id,
                tenant.id,
            )
            .user_id(user.id)
            .ip_address(refresh_token.ip_address.clone())
            .user_agent(refresh_token.user_agent.clone())
            .data(olympus_shared::events::UserLoginEvent {
                user_id: user.id,
                tenant_id: tenant.id,
                ip_address: refresh_token.ip_address.clone(),
                user_agent: refresh_token.user_agent.clone(),
                login_method: "password".to_string(),
            })?
            .build();

            let mut publisher = publisher.lock().await;
            publisher.publish(&event).await.ok();
        }

        Ok(TokenResponse {
            access_token,
            refresh_token: refresh_token_str,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user.to_response(&tenant),
        })
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<UserResponse> {
        // Find tenant
        let tenant = self.user_repo.find_tenant_by_slug(&request.tenant_slug).await?;
        if !tenant.is_active {
            return Err(AuthError::TenantInactive);
        }

        // Check if user already exists
        if self.user_repo.user_exists(&request.email, tenant.id).await? {
            return Err(AuthError::EmailAlreadyExists);
        }

        // Hash password
        let password_hash = self.password.hash_password(&request.password)?;

        // Create user
        let user = User {
            id: Uuid::new_v4(),
            tenant_id: tenant.id,
            email: request.email.clone(),
            password_hash,
            first_name: request.first_name,
            last_name: request.last_name,
            display_name: None,
            phone: request.phone,
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
        };

        let created_user = self.user_repo.create_user(&user).await?;

        // Publish user created event
        if let Some(publisher) = &self.event_publisher {
            let event = olympus_shared::events::DomainEvent::builder(
                "user.created".to_string(),
                created_user.id,
                tenant.id,
            )
            .user_id(created_user.id)
            .data(olympus_shared::events::UserCreatedEvent {
                user_id: created_user.id,
                tenant_id: tenant.id,
                email: created_user.email.clone(),
                roles: created_user.roles.clone(),
            })?
            .build();

            let mut publisher = publisher.lock().await;
            publisher.publish(&event).await.ok();
        }

        Ok(created_user.to_response(&tenant))
    }

    pub async fn refresh_token(&self, refresh_token_str: &str, ip_address: String, user_agent: String) -> Result<TokenResponse> {
        // Hash the token to compare with stored hash
        let token_hash = self.password.hash_token(refresh_token_str)?;

        // Find the refresh token
        let refresh_token = self.user_repo.find_refresh_token(&token_hash).await?;

        // Check if token is revoked
        if refresh_token.revoked_at.is_some() {
            return Err(AuthError::TokenRevoked);
        }

        // Check if token is expired
        if refresh_token.expires_at < Utc::now() {
            return Err(AuthError::TokenExpired);
        }

        // Get user and tenant
        let user = self.user_repo.find_user_by_id(refresh_token.user_id).await?;
        let tenant = self.user_repo.find_tenant_by_id(refresh_token.tenant_id).await?;

        // Check if user and tenant are active
        if !user.is_active || !tenant.is_active {
            return Err(AuthError::AccountInactive);
        }

        // Revoke old token
        self.user_repo.revoke_refresh_token(refresh_token.id).await?;

        // Generate new tokens
        let session_id = Uuid::new_v4();
        let access_token = self.jwt.generate_access_token(&user, tenant.id, session_id)?;
        let new_refresh_token_str = self.jwt.generate_refresh_token()?;

        // Store new refresh token
        let new_refresh_token = RefreshToken {
            id: Uuid::new_v4(),
            token_hash: self.password.hash_token(&new_refresh_token_str)?,
            user_id: user.id,
            tenant_id: tenant.id,
            device_id: refresh_token.device_id,
            device_name: refresh_token.device_name,
            ip_address,
            user_agent,
            expires_at: Utc::now() + Duration::days(30),
            revoked_at: None,
            created_at: Utc::now(),
        };
        self.user_repo.store_refresh_token(&new_refresh_token).await?;

        Ok(TokenResponse {
            access_token,
            refresh_token: new_refresh_token_str,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user.to_response(&tenant),
        })
    }

    pub async fn logout(&self, user_id: Uuid) -> Result<()> {
        // Revoke all refresh tokens for the user
        self.user_repo.revoke_all_user_tokens(user_id).await?;
        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims> {
        self.jwt.verify_access_token(token)
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<(User, Tenant)> {
        let user = self.user_repo.find_user_by_id(user_id).await?;
        let tenant = self.user_repo.find_tenant_by_id(user.tenant_id).await?;
        Ok((user, tenant))
    }
}