// ============================================================================
// OLYMPUS CLOUD - SHARED CONFIGURATION MANAGEMENT
// ============================================================================
// Module: shared/src/config.rs
// Description: Environment-based configuration management for all services
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use serde::{Deserialize, Serialize};
use std::time::Duration;
use config::{Config, ConfigError, Environment, File};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub security: SecurityConfig,
    pub email: EmailConfig,
    pub features: FeatureFlags,
    pub logging: LoggingConfig,
    pub monitoring: MonitoringConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
    pub request_timeout: u64,        // seconds
    pub keep_alive: u64,            // seconds
    pub max_request_size: usize,    // bytes
    pub enable_cors: bool,
    pub cors_origins: Vec<String>,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,       // seconds
    pub idle_timeout: Option<u64>,  // seconds
    pub max_lifetime: Option<u64>,  // seconds
    pub test_before_acquire: bool,
    pub enable_logging: bool,
}

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout: u64,    // seconds
    pub response_timeout: u64,      // seconds
    pub retry_attempts: u32,
    pub retry_delay: u64,          // milliseconds
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_duration: i64,  // seconds
    pub refresh_token_duration: i64, // seconds
    pub issuer: String,
    pub audience: String,
    pub algorithm: String,           // HS256, RS256, etc.
    pub leeway: i64,                // seconds for clock skew
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub password_hash_cost: u32,     // Argon2 cost parameter
    pub rate_limit_requests: u32,
    pub rate_limit_window: u64,      // seconds
    pub max_login_attempts: i32,
    pub lockout_duration: i64,       // minutes
    pub session_timeout: i64,        // minutes
    pub password_policy: PasswordPolicy,
    pub mfa_enabled: bool,
    pub require_email_verification: bool,
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub max_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_digit: bool,
    pub require_special: bool,
    pub prevent_common_passwords: bool,
    pub password_history_count: u32,
}

/// Email configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub provider: String,           // sendgrid, ses, smtp
    pub from_email: String,
    pub from_name: String,
    pub api_key: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub template_path: String,
    pub verification_url_template: String,
    pub reset_url_template: String,
}

/// Feature flags configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub registration_enabled: bool,
    pub email_verification_required: bool,
    pub password_reset_enabled: bool,
    pub mfa_enrollment_required: bool,
    pub social_login_enabled: bool,
    pub api_rate_limiting_enabled: bool,
    pub audit_logging_enabled: bool,
    pub metrics_collection_enabled: bool,
    pub health_checks_enabled: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,              // trace, debug, info, warn, error
    pub format: String,             // json, pretty, compact
    pub output: String,             // stdout, file
    pub file_path: Option<String>,
    pub rotation: LogRotation,
    pub enable_request_logging: bool,
    pub enable_db_query_logging: bool,
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotation {
    pub enabled: bool,
    pub max_size: String,           // "10MB", "1GB"
    pub max_files: u32,
    pub compress: bool,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub metrics_enabled: bool,
    pub metrics_port: u16,
    pub health_check_port: u16,
    pub tracing_enabled: bool,
    pub tracing_endpoint: Option<String>,
    pub service_name: String,
    pub environment: String,
}

impl AppConfig {
    /// Load configuration from files and environment variables
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Set default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8000)?
            .set_default("server.request_timeout", 30)?
            .set_default("server.keep_alive", 75)?
            .set_default("server.max_request_size", 1048576)? // 1MB
            .set_default("server.enable_cors", true)?
            .set_default("server.cors_origins", Vec::<String>::new())?

            // Database defaults
            .set_default("database.max_connections", 20)?
            .set_default("database.min_connections", 1)?
            .set_default("database.acquire_timeout", 30)?
            .set_default("database.idle_timeout", 600)?
            .set_default("database.max_lifetime", 1800)?
            .set_default("database.test_before_acquire", true)?
            .set_default("database.enable_logging", false)?

            // Redis defaults
            .set_default("redis.pool_size", 10)?
            .set_default("redis.connection_timeout", 5)?
            .set_default("redis.response_timeout", 5)?
            .set_default("redis.retry_attempts", 3)?
            .set_default("redis.retry_delay", 100)?

            // JWT defaults
            .set_default("jwt.access_token_duration", 900)?  // 15 minutes
            .set_default("jwt.refresh_token_duration", 2592000)? // 30 days
            .set_default("jwt.issuer", "olympus-cloud")?
            .set_default("jwt.audience", "olympus-api")?
            .set_default("jwt.algorithm", "HS256")?
            .set_default("jwt.leeway", 30)?

            // Security defaults
            .set_default("security.password_hash_cost", 4)?  // Argon2 cost
            .set_default("security.rate_limit_requests", 100)?
            .set_default("security.rate_limit_window", 60)?
            .set_default("security.max_login_attempts", 5)?
            .set_default("security.lockout_duration", 15)?
            .set_default("security.session_timeout", 1440)? // 24 hours
            .set_default("security.mfa_enabled", false)?
            .set_default("security.require_email_verification", true)?

            // Password policy defaults
            .set_default("security.password_policy.min_length", 8)?
            .set_default("security.password_policy.max_length", 128)?
            .set_default("security.password_policy.require_uppercase", true)?
            .set_default("security.password_policy.require_lowercase", true)?
            .set_default("security.password_policy.require_digit", true)?
            .set_default("security.password_policy.require_special", false)?
            .set_default("security.password_policy.prevent_common_passwords", true)?
            .set_default("security.password_policy.password_history_count", 5)?

            // Email defaults
            .set_default("email.provider", "smtp")?
            .set_default("email.from_email", "noreply@olympuscloud.io")?
            .set_default("email.from_name", "Olympus Cloud")?
            .set_default("email.template_path", "./templates/email")?
            .set_default("email.verification_url_template", "https://app.olympuscloud.io/verify?token={token}")?
            .set_default("email.reset_url_template", "https://app.olympuscloud.io/reset?token={token}")?

            // Feature flags defaults
            .set_default("features.registration_enabled", true)?
            .set_default("features.email_verification_required", true)?
            .set_default("features.password_reset_enabled", true)?
            .set_default("features.mfa_enrollment_required", false)?
            .set_default("features.social_login_enabled", false)?
            .set_default("features.api_rate_limiting_enabled", true)?
            .set_default("features.audit_logging_enabled", true)?
            .set_default("features.metrics_collection_enabled", true)?
            .set_default("features.health_checks_enabled", true)?

            // Logging defaults
            .set_default("logging.level", "info")?
            .set_default("logging.format", "json")?
            .set_default("logging.output", "stdout")?
            .set_default("logging.enable_request_logging", true)?
            .set_default("logging.enable_db_query_logging", false)?
            .set_default("logging.rotation.enabled", true)?
            .set_default("logging.rotation.max_size", "100MB")?
            .set_default("logging.rotation.max_files", 10)?
            .set_default("logging.rotation.compress", true)?

            // Monitoring defaults
            .set_default("monitoring.metrics_enabled", true)?
            .set_default("monitoring.metrics_port", 9090)?
            .set_default("monitoring.health_check_port", 8080)?
            .set_default("monitoring.tracing_enabled", false)?
            .set_default("monitoring.service_name", "olympus-service")?
            .set_default("monitoring.environment", "development")?

            // Load from config files (optional)
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))

            // Load from environment variables
            .add_source(Environment::with_prefix("OLYMPUS").separator("__"))
            .build()?;

        config.try_deserialize()
    }

    /// Load configuration for a specific service
    pub fn load_for_service(service_name: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Load shared defaults first
            .add_source(File::with_name("config/default").required(false))

            // Load service-specific config
            .add_source(File::with_name(&format!("config/{}", service_name)).required(false))

            // Load local overrides
            .add_source(File::with_name("config/local").required(false))

            // Environment variables with service prefix
            .add_source(Environment::with_prefix(&format!("OLYMPUS_{}", service_name.to_uppercase())).separator("__"))

            // General environment variables
            .add_source(Environment::with_prefix("OLYMPUS").separator("__"))
            .build()?;

        let mut app_config: Self = config.try_deserialize()?;

        // Override service name in monitoring config
        app_config.monitoring.service_name = format!("olympus-{}", service_name);

        Ok(app_config)
    }

    /// Validate configuration values
    pub fn validate(&self) -> Result<(), String> {
        // Validate server config
        if self.server.port == 0 {
            return Err("Server port cannot be 0".to_string());
        }

        // Validate database config
        if self.database.url.is_empty() {
            return Err("Database URL cannot be empty".to_string());
        }

        if self.database.max_connections <= self.database.min_connections {
            return Err("Max connections must be greater than min connections".to_string());
        }

        // Validate JWT config
        if self.jwt.secret.is_empty() {
            return Err("JWT secret cannot be empty".to_string());
        }

        if self.jwt.secret.len() < 32 && !self.is_development() {
            return Err("JWT secret must be at least 32 characters in production".to_string());
        }

        if self.jwt.access_token_duration <= 0 || self.jwt.refresh_token_duration <= 0 {
            return Err("Token durations must be positive".to_string());
        }

        // Validate password policy
        let policy = &self.security.password_policy;
        if policy.min_length == 0 || policy.min_length > policy.max_length {
            return Err("Invalid password length requirements".to_string());
        }

        // Validate email config if required
        if self.features.email_verification_required || self.features.password_reset_enabled {
            if self.email.from_email.is_empty() {
                return Err("Email from address is required when email features are enabled".to_string());
            }
        }

        Ok(())
    }

    /// Check if running in development environment
    pub fn is_development(&self) -> bool {
        self.monitoring.environment == "development"
    }

    /// Check if running in production environment
    pub fn is_production(&self) -> bool {
        self.monitoring.environment == "production"
    }

    /// Get database connection pool configuration
    pub fn database_pool_config(&self) -> crate::database::DatabaseConfig {
        crate::database::DatabaseConfig {
            database_url: self.database.url.clone(),
            max_connections: self.database.max_connections,
            min_connections: self.database.min_connections,
            acquire_timeout: Duration::from_secs(self.database.acquire_timeout),
            idle_timeout: self.database.idle_timeout.map(Duration::from_secs),
            max_lifetime: self.database.max_lifetime.map(Duration::from_secs),
            test_before_acquire: self.database.test_before_acquire,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8000,
                workers: None,
                request_timeout: 30,
                keep_alive: 75,
                max_request_size: 1048576,
                enable_cors: true,
                cors_origins: vec![],
            },
            database: DatabaseConfig {
                url: "postgresql://olympus:devpassword@localhost:5432/olympus".to_string(),
                max_connections: 20,
                min_connections: 1,
                acquire_timeout: 30,
                idle_timeout: Some(600),
                max_lifetime: Some(1800),
                test_before_acquire: true,
                enable_logging: false,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
                connection_timeout: 5,
                response_timeout: 5,
                retry_attempts: 3,
                retry_delay: 100,
            },
            jwt: JwtConfig {
                secret: "development-secret-key-change-in-production".to_string(),
                access_token_duration: 900,
                refresh_token_duration: 2592000,
                issuer: "olympus-cloud".to_string(),
                audience: "olympus-api".to_string(),
                algorithm: "HS256".to_string(),
                leeway: 30,
            },
            security: SecurityConfig {
                password_hash_cost: 4,
                rate_limit_requests: 100,
                rate_limit_window: 60,
                max_login_attempts: 5,
                lockout_duration: 15,
                session_timeout: 1440,
                password_policy: PasswordPolicy {
                    min_length: 8,
                    max_length: 128,
                    require_uppercase: true,
                    require_lowercase: true,
                    require_digit: true,
                    require_special: false,
                    prevent_common_passwords: true,
                    password_history_count: 5,
                },
                mfa_enabled: false,
                require_email_verification: true,
            },
            email: EmailConfig {
                provider: "smtp".to_string(),
                from_email: "noreply@olympuscloud.io".to_string(),
                from_name: "Olympus Cloud".to_string(),
                api_key: None,
                smtp_host: None,
                smtp_port: None,
                smtp_username: None,
                smtp_password: None,
                template_path: "./templates/email".to_string(),
                verification_url_template: "https://app.olympuscloud.io/verify?token={token}".to_string(),
                reset_url_template: "https://app.olympuscloud.io/reset?token={token}".to_string(),
            },
            features: FeatureFlags {
                registration_enabled: true,
                email_verification_required: true,
                password_reset_enabled: true,
                mfa_enrollment_required: false,
                social_login_enabled: false,
                api_rate_limiting_enabled: true,
                audit_logging_enabled: true,
                metrics_collection_enabled: true,
                health_checks_enabled: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
                output: "stdout".to_string(),
                file_path: None,
                rotation: LogRotation {
                    enabled: true,
                    max_size: "100MB".to_string(),
                    max_files: 10,
                    compress: true,
                },
                enable_request_logging: true,
                enable_db_query_logging: false,
            },
            monitoring: MonitoringConfig {
                metrics_enabled: true,
                metrics_port: 9090,
                health_check_port: 8080,
                tracing_enabled: false,
                tracing_endpoint: None,
                service_name: "olympus-service".to_string(),
                environment: "development".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_port() {
        let mut config = AppConfig::default();
        config.server.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_jwt_secret() {
        let mut config = AppConfig::default();
        config.jwt.secret = "".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_environment_detection() {
        let config = AppConfig::default();
        assert!(config.is_development());
        assert!(!config.is_production());
    }

    #[test]
    fn test_database_pool_config_conversion() {
        let config = AppConfig::default();
        let db_config = config.database_pool_config();
        assert_eq!(db_config.max_connections, config.database.max_connections);
        assert_eq!(db_config.min_connections, config.database.min_connections);
    }
}