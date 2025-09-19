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
// CONFIGURATION MANAGEMENT MODELS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "feature_flag_type", rename_all = "lowercase")]
pub enum FeatureFlagType {
    Boolean,
    String,
    Number,
    Percentage,
    JsonObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "feature_flag_status", rename_all = "lowercase")]
pub enum FeatureFlagStatus {
    Active,
    Inactive,
    Scheduled,
    Expired,
    Testing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "rollout_strategy", rename_all = "snake_case")]
pub enum RolloutStrategy {
    AllUsers,
    PercentageUsers,
    SpecificUsers,
    UserGroups,
    GradualRollout,
    ABTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>, // None for global flags
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub flag_type: FeatureFlagType,
    pub status: FeatureFlagStatus,
    pub default_value: serde_json::Value,
    pub rollout_strategy: RolloutStrategy,
    pub rollout_percentage: Option<f64>,
    pub target_users: Vec<Uuid>,
    pub target_groups: Vec<String>,
    pub conditions: serde_json::Value,
    pub variants: serde_json::Value, // For A/B testing
    pub tags: Vec<String>,
    pub is_global: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateFeatureFlagRequest {
    #[validate(length(min = 1, max = 100))]
    pub key: String,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub flag_type: FeatureFlagType,
    pub default_value: serde_json::Value,
    pub rollout_strategy: RolloutStrategy,
    #[validate(range(min = 0.0, max = 100.0))]
    pub rollout_percentage: Option<f64>,
    pub target_users: Option<Vec<Uuid>>,
    pub target_groups: Option<Vec<String>>,
    pub conditions: Option<serde_json::Value>,
    pub variants: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub is_global: bool,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateFeatureFlagRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub status: Option<FeatureFlagStatus>,
    pub default_value: Option<serde_json::Value>,
    pub rollout_strategy: Option<RolloutStrategy>,
    #[validate(range(min = 0.0, max = 100.0))]
    pub rollout_percentage: Option<f64>,
    pub target_users: Option<Vec<Uuid>>,
    pub target_groups: Option<Vec<String>>,
    pub conditions: Option<serde_json::Value>,
    pub variants: Option<serde_json::Value>,
    pub tags: Option<Vec<String>>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagEvaluation {
    pub flag_key: String,
    pub user_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub is_enabled: bool,
    pub value: serde_json::Value,
    pub variant: Option<String>,
    pub reason: String,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagUsage {
    pub flag_key: String,
    pub tenant_id: Option<Uuid>,
    pub total_evaluations: i64,
    pub enabled_evaluations: i64,
    pub unique_users: i64,
    pub last_evaluated: Option<DateTime<Utc>>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "config_scope", rename_all = "lowercase")]
pub enum ConfigScope {
    Global,
    Tenant,
    User,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "config_type", rename_all = "lowercase")]
pub enum ConfigType {
    String,
    Number,
    Boolean,
    Json,
    Encrypted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub id: Uuid,
    pub scope: ConfigScope,
    pub scope_id: Option<Uuid>, // tenant_id, user_id, location_id depending on scope
    pub key: String,
    pub display_name: String,
    pub description: Option<String>,
    pub config_type: ConfigType,
    pub value: serde_json::Value,
    pub default_value: serde_json::Value,
    pub is_sensitive: bool,
    pub is_readonly: bool,
    pub validation_rules: serde_json::Value,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateConfigurationRequest {
    pub scope: ConfigScope,
    pub scope_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub key: String,
    #[validate(length(min = 1, max = 200))]
    pub display_name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub config_type: ConfigType,
    pub value: serde_json::Value,
    pub default_value: Option<serde_json::Value>,
    pub is_sensitive: bool,
    pub is_readonly: bool,
    pub validation_rules: Option<serde_json::Value>,
    #[validate(length(min = 1, max = 50))]
    pub category: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateConfigurationRequest {
    #[validate(length(min = 1, max = 200))]
    pub display_name: Option<String>,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub value: Option<serde_json::Value>,
    pub default_value: Option<serde_json::Value>,
    pub is_sensitive: Option<bool>,
    pub is_readonly: Option<bool>,
    pub validation_rules: Option<serde_json::Value>,
    #[validate(length(min = 1, max = 50))]
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationAudit {
    pub id: Uuid,
    pub configuration_id: Uuid,
    pub action: String, // created, updated, deleted
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSearchRequest {
    pub scope: Option<ConfigScope>,
    pub scope_id: Option<Uuid>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub key_pattern: Option<String>,
    pub include_sensitive: bool,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSearchResponse {
    pub configurations: Vec<Configuration>,
    pub total_count: i64,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagEvaluationRequest {
    pub flag_key: String,
    pub user_id: Uuid,
    pub user_attributes: Option<serde_json::Value>,
    pub location_id: Option<Uuid>,
    pub custom_attributes: Option<serde_json::Value>,
}