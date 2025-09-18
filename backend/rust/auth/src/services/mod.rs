pub mod jwt;
pub mod password;
pub mod mock_repository;

pub use mock_repository::UserRepository;

use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc};
use olympus_shared::database::Database;
use olympus_shared::events::EventPublisher;
use crate::error::{AuthError, Result};
use crate::models::*;
use jwt::JwtService;
use password::PasswordService;

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
        let tenant = self.user_repo.find_tenant_by_slug(&request.tenant_slug).await?;
        if !tenant.is_active {
            return Err(AuthError::TenantInactive);
        }

        let mut user = self.user_repo.find_user_by_email(&request.email, tenant.id).await?;

        if user.is_locked() {
            return Err(AuthError::AccountLocked);
        }

        if !self.password.verify_password(&request.password, &user.password_hash)? {
            user.failed_login_attempts += 1;
            if user.failed_login_attempts >= 5 {
                user.locked_until = Some(Utc::now() + Duration::minutes(30));
            }
            self.user_repo.update_user(&user).await?;
            return Err(AuthError::InvalidCredentials);
        }

        if !user.is_active {
            return Err(AuthError::AccountInactive);
        }

        user.failed_login_attempts = 0;
        user.locked_until = None;
        user.last_login = Some(Utc::now());
        self.user_repo.update_user(&user).await?;

        let session_id = Uuid::new_v4();
        let access_token = self.jwt.generate_access_token(&user, tenant.id, session_id)?;
        let refresh_token_str = self.jwt.generate_refresh_token()?;

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

        Ok(TokenResponse {
            access_token,
            refresh_token: refresh_token_str,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user.to_response(&tenant),
        })
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<UserResponse> {
        let tenant = self.user_repo.find_tenant_by_slug(&request.tenant_slug).await?;
        if !tenant.is_active {
            return Err(AuthError::TenantInactive);
        }

        if self.user_repo.user_exists(&request.email, tenant.id).await? {
            return Err(AuthError::EmailAlreadyExists);
        }

        let password_hash = self.password.hash_password(&request.password)?;

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
        Ok(created_user.to_response(&tenant))
    }

    pub async fn refresh_token(&self, refresh_token_str: &str, _ip_address: String, _user_agent: String) -> Result<TokenResponse> {
        let token_hash = self.password.hash_token(refresh_token_str)?;
        let refresh_token = self.user_repo.find_refresh_token(&token_hash).await?;

        if refresh_token.revoked_at.is_some() {
            return Err(AuthError::TokenRevoked);
        }

        if refresh_token.expires_at < Utc::now() {
            return Err(AuthError::TokenExpired);
        }

        let user = self.user_repo.find_user_by_id(refresh_token.user_id).await?;
        let tenant = self.user_repo.find_tenant_by_id(refresh_token.tenant_id).await?;

        if !user.is_active || !tenant.is_active {
            return Err(AuthError::AccountInactive);
        }

        self.user_repo.revoke_refresh_token(refresh_token.id).await?;

        let session_id = Uuid::new_v4();
        let access_token = self.jwt.generate_access_token(&user, tenant.id, session_id)?;
        let new_refresh_token_str = self.jwt.generate_refresh_token()?;

        Ok(TokenResponse {
            access_token,
            refresh_token: new_refresh_token_str,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user.to_response(&tenant),
        })
    }

    pub async fn logout(&self, user_id: Uuid) -> Result<()> {
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