use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use olympus_shared::types::{Address, PhoneNumber, BusinessHours, FileAttachment, Money, Currency};

// Tenant Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub industry: Industry,
    pub subscription_tier: SubscriptionTier,
    pub subscription_status: SubscriptionStatus,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub billing_email: String,
    pub support_email: Option<String>,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub settings: TenantSettings,
    pub features: Vec<String>,
    pub is_active: bool,
    pub user_limit: Option<i32>,
    pub location_limit: Option<i32>,
    pub storage_limit_gb: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Industry {
    Restaurant,
    Retail,
    Salon,
    Hospitality,
    Events,
    Healthcare,
    Education,
    Professional,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionTier {
    Free,
    Starter,
    Professional,
    Enterprise,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Trial,
    Active,
    PastDue,
    Cancelled,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSettings {
    pub timezone: String,
    pub currency: Currency,
    pub language: String,
    pub date_format: String,
    pub time_format: String,
    pub week_starts_on: i8, // 0 = Sunday, 1 = Monday
    pub fiscal_year_start: i8, // Month (1-12)
    pub tax_rate: f64,
    pub multi_location: bool,
    pub multi_currency: bool,
}

// Location Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Location {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub address: Address,
    pub phone: PhoneNumber,
    pub email: Option<String>,
    pub manager_id: Option<Uuid>,
    pub timezone: String,
    pub business_hours: Vec<BusinessHours>,
    pub is_primary: bool,
    pub is_active: bool,
    pub features: Vec<String>,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// Role & Permission Models
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: String,
    pub resource: String,
    pub action: String,
    pub description: String,
    pub category: String,
}

// User extension for platform features
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserProfile {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub employee_id: Option<String>,
    pub department: Option<String>,
    pub job_title: Option<String>,
    pub reports_to: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub hire_date: Option<DateTime<Utc>>,
    pub birth_date: Option<DateTime<Utc>>,
    pub address: Option<Address>,
    pub emergency_contact: Option<EmergencyContact>,
    pub preferences: UserPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyContact {
    pub name: String,
    pub relationship: String,
    pub phone: PhoneNumber,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub timezone: String,
    pub notifications: NotificationPreferences,
    pub dashboard_layout: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub email: bool,
    pub sms: bool,
    pub push: bool,
    pub in_app: bool,
    pub digest_frequency: DigestFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DigestFrequency {
    Realtime,
    Hourly,
    Daily,
    Weekly,
    Never,
}

// Request/Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 3, max = 50))]
    pub slug: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub industry: Industry,
    #[validate(email)]
    pub billing_email: String,
    pub subscription_tier: SubscriptionTier,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub billing_email: Option<String>,
    pub support_email: Option<String>,
    pub website: Option<String>,
    pub settings: Option<TenantSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateLocationRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 20))]
    pub code: String,
    pub description: Option<String>,
    #[validate]
    pub address: Address,
    #[validate]
    pub phone: PhoneNumber,
    pub email: Option<String>,
    pub timezone: String,
    pub business_hours: Vec<BusinessHours>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignRoleRequest {
    pub role_id: Uuid,
}

// Statistics & Analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantStatistics {
    pub total_users: i64,
    pub active_users: i64,
    pub total_locations: i64,
    pub storage_used_gb: f64,
    pub api_calls_this_month: i64,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationStatistics {
    pub total_employees: i64,
    pub total_customers: i64,
    pub revenue_mtd: Money,
    pub revenue_ytd: Money,
    pub transactions_today: i64,
    pub inventory_value: Money,
}