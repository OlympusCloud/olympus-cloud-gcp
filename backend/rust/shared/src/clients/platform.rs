//! Platform service client

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::{HttpClient, HttpClientConfig, HttpClientError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub industry: String,
    pub tier: String,
    pub settings: serde_json::Value,
    pub features: serde_json::Value,
    pub branding: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub slug: String,
    pub name: String,
    pub industry: String,
    pub tier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub branding: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub timezone: String,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: String,
    pub timezone: String,
    pub is_primary: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub permissions: Vec<String>,
    pub is_system: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub resource: String,
    pub action: String,
    pub description: String,
}

pub struct PlatformClient {
    http_client: HttpClient,
}

impl PlatformClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpClientError> {
        let http_client = HttpClient::new(config)?;
        Ok(Self { http_client })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.http_client = self.http_client.with_token(token);
        self
    }

    // Tenant operations
    pub async fn get_tenant(&self, tenant_id: Uuid) -> Result<Tenant, HttpClientError> {
        self.http_client.get(&format!("/api/v1/platform/tenants/{}", tenant_id)).await
    }

    pub async fn create_tenant(&self, request: CreateTenantRequest) -> Result<Tenant, HttpClientError> {
        self.http_client.post("/api/v1/platform/tenants", &request).await
    }

    pub async fn update_tenant(&self, tenant_id: Uuid, request: UpdateTenantRequest) -> Result<Tenant, HttpClientError> {
        self.http_client.put(&format!("/api/v1/platform/tenants/{}", tenant_id), &request).await
    }

    pub async fn list_tenants(&self) -> Result<Vec<Tenant>, HttpClientError> {
        self.http_client.get("/api/v1/platform/tenants").await
    }

    // Location operations
    pub async fn get_location(&self, location_id: Uuid) -> Result<Location, HttpClientError> {
        self.http_client.get(&format!("/api/v1/platform/locations/{}", location_id)).await
    }

    pub async fn create_location(&self, tenant_id: Uuid, request: CreateLocationRequest) -> Result<Location, HttpClientError> {
        self.http_client.post(&format!("/api/v1/platform/tenants/{}/locations", tenant_id), &request).await
    }

    pub async fn list_locations(&self, tenant_id: Uuid) -> Result<Vec<Location>, HttpClientError> {
        self.http_client.get(&format!("/api/v1/platform/tenants/{}/locations", tenant_id)).await
    }

    // Role operations
    pub async fn get_role(&self, role_id: Uuid) -> Result<Role, HttpClientError> {
        self.http_client.get(&format!("/api/v1/platform/roles/{}", role_id)).await
    }

    pub async fn create_role(&self, tenant_id: Uuid, request: CreateRoleRequest) -> Result<Role, HttpClientError> {
        self.http_client.post(&format!("/api/v1/platform/tenants/{}/roles", tenant_id), &request).await
    }

    pub async fn list_roles(&self, tenant_id: Uuid) -> Result<Vec<Role>, HttpClientError> {
        self.http_client.get(&format!("/api/v1/platform/tenants/{}/roles", tenant_id)).await
    }

    pub async fn list_permissions(&self) -> Result<Vec<Permission>, HttpClientError> {
        self.http_client.get("/api/v1/platform/permissions").await
    }

    pub async fn health_check(&self) -> Result<bool, HttpClientError> {
        self.http_client.health_check().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_serialization() {
        let tenant = Tenant {
            id: Uuid::new_v4(),
            slug: "test-tenant".to_string(),
            name: "Test Tenant".to_string(),
            industry: "restaurant".to_string(),
            tier: "premium".to_string(),
            settings: serde_json::json!({}),
            features: serde_json::json!({}),
            branding: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&tenant).unwrap();
        assert!(json.contains("test-tenant"));
    }
}