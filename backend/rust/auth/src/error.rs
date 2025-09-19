use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Tenant not found")]
    TenantNotFound,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Account is inactive")]
    AccountInactive,

    #[error("Tenant is inactive")]
    TenantInactive,

    #[error("Account is locked")]
    AccountLocked,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Token revoked")]
    TokenRevoked,

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Password hash error: {0}")]
    PasswordHashError(String),

    #[error("JWT error: {0}")]
    JwtError(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Shared error: {0}")]
    Shared(#[from] olympus_shared::error::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AuthError>;

impl AuthError {
    pub fn status_code(&self) -> u16 {
        match self {
            AuthError::InvalidCredentials
            | AuthError::WeakPassword(_)
            | AuthError::Validation(_) => 400,
            AuthError::InvalidToken(_) | AuthError::TokenExpired | AuthError::TokenRevoked => 401,
            AuthError::AccountInactive | AuthError::TenantInactive | AuthError::AccountLocked => 403,
            AuthError::UserNotFound | AuthError::TenantNotFound => 404,
            AuthError::EmailAlreadyExists => 409,
            _ => 500,
        }
    }
}