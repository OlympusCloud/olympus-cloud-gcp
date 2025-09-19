use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Password hash error")]
    PasswordHash,

    #[error("Email verification required")]
    EmailVerificationRequired,

    #[error("Account locked")]
    AccountLocked,

    #[error("Account suspended")]
    AccountSuspended,

    #[error("MFA required")]
    MfaRequired,

    #[error("Invalid MFA code")]
    InvalidMfaCode,

    #[error("Session expired")]
    SessionExpired,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Insufficient permissions")]
    InsufficientPermissions,

    #[error("Tenant not found")]
    TenantNotFound,

    #[error("Migration error: {0}")]
    Migration(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Service unavailable")]
    ServiceUnavailable,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Get HTTP status code for the error
    pub fn status_code(&self) -> u16 {
        match self {
            Error::NotFound(_) | Error::TenantNotFound => 404,
            Error::AlreadyExists(_) => 409,
            Error::Unauthorized
            | Error::AuthenticationFailed(_)
            | Error::EmailVerificationRequired
            | Error::MfaRequired
            | Error::InvalidMfaCode
            | Error::SessionExpired => 401,
            Error::Forbidden
            | Error::InsufficientPermissions
            | Error::AccountLocked
            | Error::AccountSuspended => 403,
            Error::Validation(_)
            | Error::InvalidInput(_) => 400,
            Error::RateLimitExceeded => 429,
            Error::ServiceUnavailable
            | Error::ConnectionTimeout => 503,
            _ => 500,
        }
    }

    /// Get error code for client identification
    pub fn error_code(&self) -> &'static str {
        match self {
            Error::Database(_) => "DATABASE_ERROR",
            Error::Redis(_) => "REDIS_ERROR",
            Error::Serialization(_) => "SERIALIZATION_ERROR",
            Error::Validation(_) => "VALIDATION_ERROR",
            Error::NotFound(_) => "NOT_FOUND",
            Error::AlreadyExists(_) => "ALREADY_EXISTS",
            Error::Unauthorized => "UNAUTHORIZED",
            Error::Forbidden => "FORBIDDEN",
            Error::Internal(_) => "INTERNAL_ERROR",
            Error::InvalidInput(_) => "INVALID_INPUT",
            Error::Jwt(_) => "JWT_ERROR",
            Error::AuthenticationFailed(_) => "AUTHENTICATION_FAILED",
            Error::PasswordHash => "PASSWORD_HASH_ERROR",
            Error::EmailVerificationRequired => "EMAIL_VERIFICATION_REQUIRED",
            Error::AccountLocked => "ACCOUNT_LOCKED",
            Error::AccountSuspended => "ACCOUNT_SUSPENDED",
            Error::MfaRequired => "MFA_REQUIRED",
            Error::InvalidMfaCode => "INVALID_MFA_CODE",
            Error::SessionExpired => "SESSION_EXPIRED",
            Error::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Error::InsufficientPermissions => "INSUFFICIENT_PERMISSIONS",
            Error::TenantNotFound => "TENANT_NOT_FOUND",
            Error::Migration(_) => "MIGRATION_ERROR",
            Error::Configuration(_) => "CONFIGURATION_ERROR",
            Error::ConnectionTimeout => "CONNECTION_TIMEOUT",
            Error::ServiceUnavailable => "SERVICE_UNAVAILABLE",
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Error::ConnectionTimeout
            | Error::ServiceUnavailable
            | Error::Database(_)
            | Error::Redis(_)
        )
    }

    /// Check if error should be logged as warning vs error
    pub fn is_client_error(&self) -> bool {
        matches!(
            self,
            Error::Validation(_)
            | Error::InvalidInput(_)
            | Error::NotFound(_)
            | Error::AlreadyExists(_)
            | Error::Unauthorized
            | Error::Forbidden
            | Error::AuthenticationFailed(_)
            | Error::EmailVerificationRequired
            | Error::AccountLocked
            | Error::AccountSuspended
            | Error::MfaRequired
            | Error::InvalidMfaCode
            | Error::SessionExpired
            | Error::RateLimitExceeded
            | Error::InsufficientPermissions
            | Error::TenantNotFound
        )
    }
}

/// Error response for API endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: Option<String>,
}

impl ErrorResponse {
    /// Create a new error response
    pub fn new(error: &Error) -> Self {
        Self {
            error: error.error_code().to_string(),
            code: error.status_code().to_string(),
            message: error.to_string(),
            details: None,
            timestamp: chrono::Utc::now(),
            request_id: None,
        }
    }

    /// Add details to the error response
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Add request ID to the error response
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }
}

impl From<Error> for ErrorResponse {
    fn from(error: Error) -> Self {
        ErrorResponse::new(&error)
    }
}

/// Result extension trait for error handling
pub trait ErrorExt<T> {
    /// Map error to internal error with context
    fn internal_error<S: Into<String>>(self, context: S) -> Result<T>;

    /// Map error to validation error with context
    fn validation_error<S: Into<String>>(self, context: S) -> Result<T>;

    /// Map error to not found error with context
    fn not_found_error<S: Into<String>>(self, context: S) -> Result<T>;
}

impl<T, E: std::fmt::Display> ErrorExt<T> for std::result::Result<T, E> {
    fn internal_error<S: Into<String>>(self, context: S) -> Result<T> {
        self.map_err(|e| Error::Internal(format!("{}: {}", context.into(), e)))
    }

    fn validation_error<S: Into<String>>(self, context: S) -> Result<T> {
        self.map_err(|e| Error::Validation(format!("{}: {}", context.into(), e)))
    }

    fn not_found_error<S: Into<String>>(self, context: S) -> Result<T> {
        self.map_err(|e| Error::NotFound(format!("{}: {}", context.into(), e)))
    }
}