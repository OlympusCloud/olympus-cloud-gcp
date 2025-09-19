//! gRPC client for inter-service communication

use std::time::Duration;
use thiserror::Error;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Status};
use tracing::{debug, error, info};

use super::ClientConfig;

#[derive(Debug, Error)]
pub enum GrpcClientError {
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),

    #[error("Request error: {0}")]
    RequestError(#[from] Status),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Connection failed")]
    ConnectionFailed,
}

#[derive(Debug, Clone)]
pub struct GrpcClientConfig {
    pub endpoint: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub keep_alive_interval_secs: u64,
    pub keep_alive_timeout_secs: u64,
}

impl From<ClientConfig> for GrpcClientConfig {
    fn from(config: ClientConfig) -> Self {
        Self {
            endpoint: config.base_url.replace("http://", "").replace("https://", ""),
            timeout_ms: config.timeout_ms,
            max_retries: config.max_retries,
            keep_alive_interval_secs: 10,
            keep_alive_timeout_secs: 20,
        }
    }
}

pub struct GrpcClient {
    channel: Channel,
    config: GrpcClientConfig,
    jwt_token: Option<String>,
}

impl GrpcClient {
    pub async fn new(config: GrpcClientConfig) -> Result<Self, GrpcClientError> {
        let endpoint = Endpoint::from_shared(format!("http://{}", config.endpoint))
            .map_err(|e| GrpcClientError::InvalidConfig(e.to_string()))?
            .timeout(Duration::from_millis(config.timeout_ms));

        let channel = endpoint
            .connect()
            .await
            .map_err(GrpcClientError::TransportError)?;

        info!("gRPC client connected to {}", config.endpoint);

        Ok(Self {
            channel,
            config,
            jwt_token: None,
        })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.jwt_token = Some(token);
        self
    }

    pub fn channel(&self) -> Channel {
        self.channel.clone()
    }

    pub fn add_auth_metadata<T>(&self, mut request: Request<T>) -> Request<T> {
        if let Some(token) = &self.jwt_token {
            request.metadata_mut().insert(
                "authorization",
                format!("Bearer {}", token)
                    .parse()
                    .expect("Invalid token format"),
            );
        }
        request
    }

    pub async fn health_check(&self) -> Result<bool, GrpcClientError> {
        debug!("Performing gRPC health check for {}", self.config.endpoint);

        match tonic::transport::Endpoint::from_shared(format!("http://{}", self.config.endpoint))
            .map_err(|e| GrpcClientError::InvalidConfig(e.to_string()))?
            .connect()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                error!("gRPC health check failed: {}", e);
                Ok(false)
            }
        }
    }
}

pub trait GrpcRetry {
    async fn execute_with_retry<F, T, Fut>(&self, f: F) -> Result<T, Status>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, Status>>;
}

impl GrpcRetry for GrpcClient {
    async fn execute_with_retry<F, T, Fut>(&self, f: F) -> Result<T, Status>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, Status>>,
    {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.config.max_retries {
            attempts += 1;
            debug!(
                "Attempting gRPC request (attempt {}/{})",
                attempts, self.config.max_retries
            );

            match f().await {
                Ok(response) => return Ok(response),
                Err(status) => {
                    error!("gRPC request failed: {}", status);
                    let status_code = status.code();
                    last_error = Some(status);

                    if !matches!(
                        status_code,
                        tonic::Code::Unavailable
                            | tonic::Code::ResourceExhausted
                            | tonic::Code::DeadlineExceeded
                    ) {
                        return Err(last_error.unwrap());
                    }
                }
            }

            if attempts < self.config.max_retries {
                let delay = Duration::from_millis(1000 * (2_u64.pow(attempts - 1)));
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error.unwrap_or_else(|| {
            Status::resource_exhausted("Maximum retries exceeded")
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_conversion() {
        let client_config = ClientConfig {
            base_url: "http://localhost:50051".to_string(),
            timeout_ms: 5000,
            max_retries: 3,
            failure_threshold: 5,
            recovery_timeout_secs: 60,
            jwt_token: None,
        };

        let grpc_config: GrpcClientConfig = client_config.into();

        assert_eq!(grpc_config.endpoint, "localhost:50051");
        assert_eq!(grpc_config.timeout_ms, 5000);
        assert_eq!(grpc_config.max_retries, 3);
    }
}