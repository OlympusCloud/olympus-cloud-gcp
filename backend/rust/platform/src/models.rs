use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub industry: String,
    pub subscription_tier: String,
    pub is_active: bool,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 2, max = 50))]
    pub slug: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub industry: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub industry: Option<String>,
    pub settings: Option<serde_json::Value>,
}