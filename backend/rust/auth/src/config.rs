use serde::{Deserialize, Serialize};
use config::{Config, ConfigError, Environment, File};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_duration: i64,  // seconds
    pub refresh_token_duration: i64, // seconds
    pub issuer: String,
    pub audience: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SecurityConfig {
    pub bcrypt_cost: u32,
    pub rate_limit_requests: u32,
    pub rate_limit_window: u64, // seconds
    pub max_login_attempts: i32,
    pub lockout_duration: i64, // minutes
    pub password_min_length: usize,
    pub password_require_uppercase: bool,
    pub password_require_lowercase: bool,
    pub password_require_digit: bool,
    pub password_require_special: bool,
}

impl AuthConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start with default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8000)?
            .set_default("server.workers", 4)?
            .set_default("database.max_connections", 10)?
            .set_default("database.min_connections", 2)?
            .set_default("database.connect_timeout", 30)?
            .set_default("database.idle_timeout", 600)?
            .set_default("redis.pool_size", 10)?
            .set_default("jwt.access_token_duration", 3600)?
            .set_default("jwt.refresh_token_duration", 2592000)?
            .set_default("jwt.issuer", "olympus-auth")?
            .set_default("jwt.audience", "olympus-api")?
            .set_default("security.bcrypt_cost", 12)?
            .set_default("security.rate_limit_requests", 100)?
            .set_default("security.rate_limit_window", 60)?
            .set_default("security.max_login_attempts", 5)?
            .set_default("security.lockout_duration", 30)?
            .set_default("security.password_min_length", 8)?
            .set_default("security.password_require_uppercase", true)?
            .set_default("security.password_require_lowercase", true)?
            .set_default("security.password_require_digit", true)?
            .set_default("security.password_require_special", true)?
            // Add in settings from config file
            .add_source(File::with_name("config/auth").required(false))
            // Add in settings from environment variables (with AUTH_ prefix)
            .add_source(Environment::with_prefix("AUTH").separator("_"))
            .build()?;

        config.try_deserialize()
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8000,
                workers: 4,
            },
            database: DatabaseConfig {
                url: "postgresql://olympus:devpassword@localhost:5432/olympus".to_string(),
                max_connections: 10,
                min_connections: 2,
                connect_timeout: 30,
                idle_timeout: 600,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
            },
            jwt: JwtConfig {
                secret: "development-secret-key-change-in-production".to_string(),
                access_token_duration: 3600,
                refresh_token_duration: 2592000,
                issuer: "olympus-auth".to_string(),
                audience: "olympus-api".to_string(),
            },
            security: SecurityConfig {
                bcrypt_cost: 12,
                rate_limit_requests: 100,
                rate_limit_window: 60,
                max_login_attempts: 5,
                lockout_duration: 30,
                password_min_length: 8,
                password_require_uppercase: true,
                password_require_lowercase: true,
                password_require_digit: true,
                password_require_special: true,
            },
        }
    }
}