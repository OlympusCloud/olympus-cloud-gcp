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

// ============================================================================
// LOCATION MANAGEMENT MODELS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "location_type", rename_all = "lowercase")]
pub enum LocationType {
    Headquarters,
    Branch,
    Store,
    Warehouse,
    Office,
    Restaurant,
    Salon,
    EventVenue,
    Kiosk,
    Mobile,
    Virtual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "location_status", rename_all = "lowercase")]
pub enum LocationStatus {
    Active,
    Inactive,
    UnderMaintenance,
    Planned,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street_line_1: String,
    pub street_line_2: Option<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub primary_phone: Option<String>,
    pub secondary_phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessHours {
    pub monday: Option<String>,
    pub tuesday: Option<String>,
    pub wednesday: Option<String>,
    pub thursday: Option<String>,
    pub friday: Option<String>,
    pub saturday: Option<String>,
    pub sunday: Option<String>,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_location_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub location_type: LocationType,
    pub status: LocationStatus,
    pub address: Address,
    pub contact_info: ContactInfo,
    pub business_hours: Option<BusinessHours>,
    pub manager_user_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub area_square_feet: Option<f64>,
    pub settings: serde_json::Value,
    pub metadata: serde_json::Value,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLocationRequest {
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub location_type: LocationType,
    pub parent_location_id: Option<Uuid>,
    pub address: Address,
    pub contact_info: Option<ContactInfo>,
    pub business_hours: Option<BusinessHours>,
    pub manager_user_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub area_square_feet: Option<f64>,
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub is_public: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateLocationRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub location_type: Option<LocationType>,
    pub status: Option<LocationStatus>,
    pub parent_location_id: Option<Uuid>,
    pub address: Option<Address>,
    pub contact_info: Option<ContactInfo>,
    pub business_hours: Option<BusinessHours>,
    pub manager_user_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub area_square_feet: Option<f64>,
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationHierarchy {
    pub location: Location,
    pub children: Vec<LocationHierarchy>,
    pub depth: i32,
    pub path: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub location_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub is_primary: bool,
    pub access_level: String,
    pub assigned_at: DateTime<Utc>,
    pub assigned_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLocationAssignmentRequest {
    pub user_id: Uuid,
    pub location_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub role: String,
    pub is_primary: bool,
    #[validate(length(min = 1, max = 50))]
    pub access_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStatistics {
    pub location_id: Uuid,
    pub total_users: i64,
    pub active_users: i64,
    pub total_orders: i64,
    pub total_revenue: rust_decimal::Decimal,
    pub capacity_utilization: Option<f64>,
    pub last_activity: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSearchRequest {
    pub query: Option<String>,
    pub location_type: Option<LocationType>,
    pub status: Option<LocationStatus>,
    pub parent_location_id: Option<Uuid>,
    pub manager_user_id: Option<Uuid>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub country: Option<String>,
    pub radius_km: Option<f64>,
    pub center_lat: Option<f64>,
    pub center_lng: Option<f64>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSearchResponse {
    pub locations: Vec<Location>,
    pub total_count: i64,
    pub has_more: bool,
}