//! HTTP client for inter-service communication

use async_trait::async_trait;
use reqwest::{Client, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::ClientConfig;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Circuit breaker is open")]
    CircuitBreakerOpen,

    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone)]
pub struct HttpClientConfig {
    pub base_url: String,
    pub timeout_ms: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub circuit_breaker_enabled: bool,
    pub failure_threshold: u32,
    pub recovery_timeout_secs: u64,
}

impl From<ClientConfig> for HttpClientConfig {
    fn from(config: ClientConfig) -> Self {
        Self {
            base_url: config.base_url,
            timeout_ms: config.timeout_ms,
            max_retries: config.max_retries,
            retry_delay_ms: 1000,
            circuit_breaker_enabled: true,
            failure_threshold: config.failure_threshold,
            recovery_timeout_secs: config.recovery_timeout_secs,
        }
    }
}

#[derive(Debug)]
struct CircuitBreaker {
    failures: u32,
    threshold: u32,
    is_open: bool,
    last_failure_time: Option<std::time::Instant>,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    fn new(threshold: u32, recovery_timeout_secs: u64) -> Self {
        Self {
            failures: 0,
            threshold,
            is_open: false,
            last_failure_time: None,
            recovery_timeout: Duration::from_secs(recovery_timeout_secs),
        }
    }

    fn record_success(&mut self) {
        self.failures = 0;
        self.is_open = false;
        self.last_failure_time = None;
    }

    fn record_failure(&mut self) {
        self.failures += 1;
        self.last_failure_time = Some(std::time::Instant::now());

        if self.failures >= self.threshold {
            self.is_open = true;
            warn!("Circuit breaker opened after {} failures", self.failures);
        }
    }

    fn can_attempt(&mut self) -> bool {
        if !self.is_open {
            return true;
        }

        if let Some(last_failure) = self.last_failure_time {
            if last_failure.elapsed() > self.recovery_timeout {
                info!("Circuit breaker attempting recovery");
                self.is_open = false;
                self.failures = 0;
                return true;
            }
        }

        false
    }
}

pub struct HttpClient {
    client: Client,
    config: HttpClientConfig,
    circuit_breaker: Arc<RwLock<CircuitBreaker>>,
    default_headers: HashMap<String, String>,
}

impl HttpClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpClientError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .map_err(HttpClientError::RequestError)?;

        let circuit_breaker = Arc::new(RwLock::new(CircuitBreaker::new(
            config.failure_threshold,
            config.recovery_timeout_secs,
        )));

        Ok(Self {
            client,
            config,
            circuit_breaker,
            default_headers: HashMap::new(),
        })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.default_headers.insert(
            "Authorization".to_string(),
            format!("Bearer {}", token),
        );
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.default_headers.insert(key, value);
        self
    }

    async fn apply_headers(&self, mut request: RequestBuilder) -> RequestBuilder {
        for (key, value) in &self.default_headers {
            request = request.header(key, value);
        }
        request
    }

    async fn execute_with_retry<T>(&self, request_fn: impl Fn() -> RequestBuilder) -> Result<Response, HttpClientError> {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.config.max_retries {
            if self.config.circuit_breaker_enabled {
                let mut breaker = self.circuit_breaker.write().await;
                if !breaker.can_attempt() {
                    return Err(HttpClientError::CircuitBreakerOpen);
                }
            }

            attempts += 1;
            debug!("Attempting request (attempt {}/{})", attempts, self.config.max_retries);

            let request = self.apply_headers(request_fn()).await;

            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        if self.config.circuit_breaker_enabled {
                            let mut breaker = self.circuit_breaker.write().await;
                            breaker.record_success();
                        }
                        return Ok(response);
                    } else if response.status().is_server_error() && attempts < self.config.max_retries {
                        warn!("Server error (status: {}), retrying...", response.status());
                        last_error = Some(HttpClientError::RequestError(
                            reqwest::Error::from(response.error_for_status().unwrap_err())
                        ));
                    } else {
                        return Ok(response);
                    }
                }
                Err(e) => {
                    error!("Request failed: {}", e);
                    last_error = Some(HttpClientError::RequestError(e));

                    if self.config.circuit_breaker_enabled {
                        let mut breaker = self.circuit_breaker.write().await;
                        breaker.record_failure();
                    }
                }
            }

            if attempts < self.config.max_retries {
                let delay = Duration::from_millis(
                    self.config.retry_delay_ms * (2_u64.pow(attempts - 1))
                );
                tokio::time::sleep(delay).await;
            }
        }

        Err(last_error.unwrap_or(HttpClientError::MaxRetriesExceeded))
    }

    pub async fn get<T>(&self, path: &str) -> Result<T, HttpClientError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let response = self.execute_with_retry(|| self.client.get(&url)).await?;

        let json = response.json::<T>().await?;
        Ok(json)
    }

    pub async fn post<B, T>(&self, path: &str, body: &B) -> Result<T, HttpClientError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let response = self.execute_with_retry(|| {
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(body)
        }).await?;

        let json = response.json::<T>().await?;
        Ok(json)
    }

    pub async fn put<B, T>(&self, path: &str, body: &B) -> Result<T, HttpClientError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.config.base_url, path);
        let response = self.execute_with_retry(|| {
            self.client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(body)
        }).await?;

        let json = response.json::<T>().await?;
        Ok(json)
    }

    pub async fn delete(&self, path: &str) -> Result<(), HttpClientError> {
        let url = format!("{}{}", self.config.base_url, path);
        self.execute_with_retry(|| self.client.delete(&url)).await?;
        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool, HttpClientError> {
        let url = format!("{}/health", self.config.base_url);
        let response = self.client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new(3, 60);

        assert!(breaker.can_attempt());

        breaker.record_failure();
        breaker.record_failure();
        assert!(breaker.can_attempt());

        breaker.record_failure();
        assert!(!breaker.can_attempt());

        breaker.record_success();
        assert!(breaker.can_attempt());
    }
}