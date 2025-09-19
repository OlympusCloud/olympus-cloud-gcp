//! Client modules for inter-service communication
//!
//! This module provides HTTP and gRPC clients for communicating between services
//! in the Olympus Cloud platform. All clients include:
//! - Automatic JWT token attachment
//! - Request/response logging and tracing
//! - Retry with exponential backoff
//! - Circuit breaker pattern for fault tolerance
//! - Request timeout configuration
//! - Load balancing for multiple endpoints

pub mod http_client;
pub mod grpc_client;
pub mod auth;
pub mod platform;
pub mod commerce;
pub mod analytics;

pub use http_client::{HttpClient, HttpClientConfig, HttpClientError};
pub use grpc_client::{GrpcClient, GrpcClientConfig, GrpcClientError};
pub use auth::AuthClient;
pub use platform::PlatformClient;
pub use commerce::CommerceClient;
pub use analytics::AnalyticsClient;

/// Common client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for HTTP clients
    pub base_url: String,
    /// Request timeout in milliseconds
    pub timeout_ms: u64,
    /// Maximum number of retries
    pub max_retries: u32,
    /// Circuit breaker failure threshold
    pub failure_threshold: u32,
    /// Circuit breaker recovery timeout in seconds
    pub recovery_timeout_secs: u64,
    /// JWT token for authentication
    pub jwt_token: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            timeout_ms: 30000,
            max_retries: 3,
            failure_threshold: 5,
            recovery_timeout_secs: 60,
            jwt_token: None,
        }
    }
}