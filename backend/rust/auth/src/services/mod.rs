pub mod jwt;
pub mod password;
pub mod mock_repository;

pub use mock_repository::UserRepository;

use std::sync::Arc;
use uuid::Uuid;
use chrono::{Duration, Utc, DateTime};
use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use crate::error::{AuthError, Result};
use crate::models::*;
use jwt::{JwtService, DeviceInfo};
use password::PasswordService;
use serde::{Serialize, Deserialize};

pub struct AuthService {
    db: Arc<DbPool>,
    jwt: JwtService,
    password: PasswordService,
    user_repo: UserRepository,
    event_publisher: Option<Arc<tokio::sync::Mutex<EventPublisher>>>,
}

impl AuthService {
    pub fn new(
        db: Arc<DbPool>,
        jwt_secret: &str,
        event_publisher: Option<Arc<tokio::sync::Mutex<EventPublisher>>>,
    ) -> olympus_shared::Result<Self> {
        let jwt = JwtService::new(
            jwt_secret,
            "olympus-cloud".to_string(),
            "olympus-users".to_string(),
            3600,    // 1 hour access token
            2592000, // 30 days refresh token
        )?;

        Ok(Self {
            db: db.clone(),
            jwt,
            password: PasswordService::new(),
            user_repo: UserRepository::new(db),
            event_publisher,
        })
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

        let session_id = Uuid::new_v4().to_string();
        let device_info = DeviceInfo {
            device_id: request.device_id.clone(),
            user_agent: Some(user_agent.clone()),
            ip_address: Some(ip_address.clone()),
        };

        let token_pair = self.jwt.generate_token_pair(
            user.id,
            tenant.id,
            user.email.clone(),
            user.roles.clone(),
            user.permissions.clone(),
            session_id,
            device_info,
        )?;

        let refresh_token = RefreshToken {
            id: Uuid::new_v4(),
            token_hash: self.password.hash_token(&token_pair.refresh_token)?,
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
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_pair.expires_in,
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

        let session_id = Uuid::new_v4().to_string();
        let device_info = DeviceInfo {
            device_id: refresh_token.device_id.clone(),
            user_agent: Some(refresh_token.user_agent.clone()),
            ip_address: Some(refresh_token.ip_address.clone()),
        };

        let token_pair = self.jwt.generate_token_pair(
            user.id,
            tenant.id,
            user.email.clone(),
            user.roles.clone(),
            user.permissions.clone(),
            session_id,
            device_info,
        )?;

        Ok(TokenResponse {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: token_pair.expires_in,
            user: user.to_response(&tenant),
        })
    }

    pub async fn logout(&self, user_id: Uuid) -> Result<()> {
        self.user_repo.revoke_all_user_tokens(user_id).await?;
        Ok(())
    }

    pub async fn verify_token(&self, token: &str) -> Result<Claims> {
        let validation = self.jwt.validate_access_token(token)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        // Convert from JWT TokenValidation to our Claims format
        Ok(Claims {
            sub: validation.claims.sub.parse()
                .map_err(|_| AuthError::InvalidToken("Invalid user ID".to_string()))?,
            tenant_id: validation.claims.tenant_id.parse()
                .map_err(|_| AuthError::InvalidToken("Invalid tenant ID".to_string()))?,
            email: validation.claims.email,
            roles: validation.claims.roles,
            permissions: validation.claims.permissions,
            session_id: validation.claims.session_id.parse()
                .map_err(|_| AuthError::InvalidToken("Invalid session ID".to_string()))?,
            iat: validation.claims.iat,
            exp: validation.claims.exp,
        })
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<(User, Tenant)> {
        let user = self.user_repo.find_user_by_id(user_id).await?;
        let tenant = self.user_repo.find_tenant_by_id(user.tenant_id).await?;
        Ok((user, tenant))
    }

    pub async fn forgot_password(&self, request: ForgotPasswordRequest) -> Result<()> {
        let tenant = self.user_repo.find_tenant_by_slug(&request.tenant_slug).await?;
        if !tenant.is_active {
            return Err(AuthError::TenantInactive);
        }

        // Check if user exists - but don't reveal whether they exist or not for security
        if let Ok(user) = self.user_repo.find_user_by_email(&request.email, tenant.id).await {
            if user.is_active {
                // Generate password reset token
                let reset_token = self.jwt.generate_password_reset_token(user.id, &user.email)?;

                // In a real implementation, you would:
                // 1. Store the token in the database with expiration
                // 2. Send an email with the reset link
                // For now, we'll just log it (DO NOT do this in production!)
                println!("Password reset token for {}: {}", request.email, reset_token);

                // TODO: Implement email service and token storage
                // self.email_service.send_password_reset_email(&user.email, &reset_token).await?;
            }
        }

        // Always return success to avoid email enumeration attacks
        Ok(())
    }

    pub async fn reset_password(&self, request: ResetPasswordRequest) -> Result<()> {
        // Verify the reset token
        let user_id = self.jwt.verify_special_token(&request.token, "password_reset")
            .map_err(|_| AuthError::InvalidToken("Invalid or expired reset token".to_string()))?;

        let mut user = self.user_repo.find_user_by_id(user_id).await?;

        if !user.is_active {
            return Err(AuthError::AccountInactive);
        }

        // Hash the new password
        let new_password_hash = self.password.hash_password(&request.new_password)?;

        // Update the user's password
        user.update_password(new_password_hash);
        self.user_repo.update_user(&user).await?;

        // Revoke all existing refresh tokens for security
        self.user_repo.revoke_all_user_tokens(user_id).await?;

        Ok(())
    }

    pub async fn change_password(&self, user_id: Uuid, request: ChangePasswordRequest) -> Result<()> {
        let mut user = self.user_repo.find_user_by_id(user_id).await?;

        if !user.is_active {
            return Err(AuthError::AccountInactive);
        }

        // Verify current password
        if !self.password.verify_password(&request.current_password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Hash the new password
        let new_password_hash = self.password.hash_password(&request.new_password)?;

        // Update the user's password
        user.update_password(new_password_hash);
        self.user_repo.update_user(&user).await?;

        // Revoke all existing refresh tokens except current session for security
        // In a real implementation, you might want to keep the current session active
        self.user_repo.revoke_all_user_tokens(user_id).await?;

        Ok(())
    }

    pub async fn verify_email(&self, token: &str) -> Result<()> {
        // Verify the email verification token
        let user_id = self.jwt.verify_special_token(token, "email_verification")
            .map_err(|_| AuthError::InvalidToken("Invalid or expired verification token".to_string()))?;

        let mut user = self.user_repo.find_user_by_id(user_id).await?;

        // Mark email as verified
        user.verify_email();
        self.user_repo.update_user(&user).await?;

        Ok(())
    }

    pub async fn get_active_sessions(&self, user_id: Uuid) -> Result<Vec<SessionInfo>> {
        // Get all active refresh tokens for the user (which represent sessions)
        let refresh_tokens = self.user_repo.find_active_refresh_tokens(user_id).await?;

        let sessions = refresh_tokens
            .into_iter()
            .map(|token| SessionInfo {
                id: token.id,
                device_id: token.device_id,
                device_name: token.device_name,
                ip_address: token.ip_address,
                user_agent: token.user_agent,
                created_at: token.created_at,
                last_used_at: token.created_at, // TODO: Track actual last usage
                is_current: false, // TODO: Determine current session
            })
            .collect();

        Ok(sessions)
    }

    pub async fn revoke_session(&self, user_id: Uuid, session_id: Uuid) -> Result<()> {
        // Find the refresh token and verify it belongs to the user
        let refresh_token = self.user_repo.find_refresh_token_by_id(session_id).await?;

        if refresh_token.user_id != user_id {
            return Err(AuthError::InvalidToken("Session does not belong to user".to_string()));
        }

        // Revoke the specific session
        self.user_repo.revoke_refresh_token(session_id).await?;
        Ok(())
    }

    pub async fn revoke_all_other_sessions(&self, user_id: Uuid, current_session_id: Option<Uuid>) -> Result<()> {
        // Revoke all sessions except the current one
        if let Some(current_id) = current_session_id {
            self.user_repo.revoke_all_user_tokens_except(user_id, current_id).await?;
        } else {
            self.user_repo.revoke_all_user_tokens(user_id).await?;
        }
        Ok(())
    }
}

// Session information for display to users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: Uuid,
    pub device_id: Option<String>,
    pub device_name: Option<String>,
    pub ip_address: String,
    pub user_agent: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub is_current: bool,
}