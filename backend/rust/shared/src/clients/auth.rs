//! Auth service client

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{HttpClient, HttpClientConfig, HttpClientError};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub tenant_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub email: String,
    pub tenant_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub roles: Vec<String>,
}

pub struct AuthClient {
    http_client: HttpClient,
}

impl AuthClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.http_client = self.http_client.with_token(token);
        self
    }

    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, HttpClientError> {
        self.http_client.post("/api/v1/auth/login", &request).await
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse, HttpClientError> {
        self.http_client.post("/api/v1/auth/register", &request).await
    }

    pub async fn refresh_token(&self, request: RefreshTokenRequest) -> Result<RefreshTokenResponse, HttpClientError> {
        self.http_client.post("/api/v1/auth/refresh", &request).await
    }

    pub async fn validate_token(&self, request: ValidateTokenRequest) -> Result<ValidateTokenResponse, HttpClientError> {
        self.http_client.post("/api/v1/auth/validate", &request).await
    }

    pub async fn logout(&self) -> Result<(), HttpClientError> {
        self.http_client.post::<(), ()>("/api/v1/auth/logout", &()).await?;
        Ok(())
    }

    pub async fn health_check(&self) -> Result<bool, HttpClientError> {
        self.http_client.health_check().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_request_serialization() {
        let request = LoginRequest {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            tenant_id: Some(Uuid::new_v4()),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("test@example.com"));
    }
}