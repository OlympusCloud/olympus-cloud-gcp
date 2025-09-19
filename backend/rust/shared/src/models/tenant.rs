// ============================================================================
// OLYMPUS CLOUD - TENANT MODELS
// ============================================================================
// Module: shared/src/models/tenant.rs
// Description: Tenant entity and multi-tenancy models
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use super::{AuditFields, SoftDelete, ValidateEntity};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use validator::{Validate, ValidationError};

/// Industry type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "industry_type", rename_all = "lowercase")]
pub enum IndustryType {
    Restaurant,
    Retail,
    Salon,
    Hospitality,
    Events,
    Other,
}

impl Default for IndustryType {
    fn default() -> Self {
        Self::Other
    }
}

impl IndustryType {
    /// Get display name for the industry
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Restaurant => "Restaurant & Food Service",
            Self::Retail => "Retail & E-commerce",
            Self::Salon => "Beauty & Wellness",
            Self::Hospitality => "Hospitality & Travel",
            Self::Events => "Events & Entertainment",
            Self::Other => "Other",
        }
    }

    /// Get default features for the industry
    pub fn default_features(&self) -> Vec<String> {
        match self {
            Self::Restaurant => vec![
                "table_management".to_string(),
                "menu_management".to_string(),
                "kitchen_display".to_string(),
                "delivery_tracking".to_string(),
            ],
            Self::Retail => vec![
                "inventory_management".to_string(),
                "e_commerce".to_string(),
                "barcode_scanning".to_string(),
                "customer_loyalty".to_string(),
            ],
            Self::Salon => vec![
                "appointment_booking".to_string(),
                "service_management".to_string(),
                "staff_scheduling".to_string(),
                "customer_profiles".to_string(),
            ],
            Self::Hospitality => vec![
                "room_management".to_string(),
                "reservation_system".to_string(),
                "guest_services".to_string(),
                "housekeeping".to_string(),
            ],
            Self::Events => vec![
                "event_planning".to_string(),
                "ticket_sales".to_string(),
                "venue_management".to_string(),
                "attendee_tracking".to_string(),
            ],
            Self::Other => vec![
                "basic_pos".to_string(),
                "customer_management".to_string(),
                "reporting".to_string(),
            ],
        }
    }
}

/// Subscription tier enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "subscription_tier", rename_all = "lowercase")]
pub enum SubscriptionTier {
    Free,
    Starter,
    Professional,
    Enterprise,
    Custom,
}

impl Default for SubscriptionTier {
    fn default() -> Self {
        Self::Free
    }
}

impl SubscriptionTier {
    /// Get user limit for the tier
    pub fn user_limit(&self) -> Option<i32> {
        match self {
            Self::Free => Some(2),
            Self::Starter => Some(10),
            Self::Professional => Some(50),
            Self::Enterprise => Some(200),
            Self::Custom => None, // Unlimited
        }
    }

    /// Get location limit for the tier
    pub fn location_limit(&self) -> Option<i32> {
        match self {
            Self::Free => Some(1),
            Self::Starter => Some(3),
            Self::Professional => Some(10),
            Self::Enterprise => Some(50),
            Self::Custom => None, // Unlimited
        }
    }

    /// Get storage limit in GB
    pub fn storage_limit_gb(&self) -> Option<i32> {
        match self {
            Self::Free => Some(1),
            Self::Starter => Some(10),
            Self::Professional => Some(100),
            Self::Enterprise => Some(1000),
            Self::Custom => None, // Unlimited
        }
    }

    /// Get monthly price in cents (USD)
    pub fn monthly_price_cents(&self) -> i32 {
        match self {
            Self::Free => 0,
            Self::Starter => 2900,    // $29.00
            Self::Professional => 9900, // $99.00
            Self::Enterprise => 29900, // $299.00
            Self::Custom => 0,        // Contact sales
        }
    }

    /// Get available features for the tier
    pub fn available_features(&self) -> Vec<String> {
        match self {
            Self::Free => vec![
                "basic_pos".to_string(),
                "basic_reporting".to_string(),
                "customer_management".to_string(),
            ],
            Self::Starter => vec![
                "basic_pos".to_string(),
                "inventory_management".to_string(),
                "customer_management".to_string(),
                "basic_reporting".to_string(),
                "email_support".to_string(),
            ],
            Self::Professional => vec![
                "advanced_pos".to_string(),
                "inventory_management".to_string(),
                "customer_management".to_string(),
                "advanced_reporting".to_string(),
                "analytics_dashboard".to_string(),
                "api_access".to_string(),
                "priority_support".to_string(),
                "multi_location".to_string(),
            ],
            Self::Enterprise => vec![
                "advanced_pos".to_string(),
                "inventory_management".to_string(),
                "customer_management".to_string(),
                "advanced_reporting".to_string(),
                "analytics_dashboard".to_string(),
                "api_access".to_string(),
                "priority_support".to_string(),
                "multi_location".to_string(),
                "white_label".to_string(),
                "custom_integrations".to_string(),
                "dedicated_support".to_string(),
                "sla_guarantee".to_string(),
            ],
            Self::Custom => vec![
                "all_features".to_string(),
                "custom_development".to_string(),
                "dedicated_infrastructure".to_string(),
                "24_7_support".to_string(),
            ],
        }
    }
}

/// Subscription status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "subscription_status", rename_all = "UPPERCASE")]
pub enum SubscriptionStatus {
    Trial,
    Active,
    PastDue,
    Cancelled,
    Suspended,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Trial
    }
}

/// Main tenant entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub industry: serde_json::Value,
    pub subscription_tier: SubscriptionTier,
    pub subscription_status: SubscriptionStatus,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub billing_email: Option<String>,
    pub support_email: Option<String>,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub settings: serde_json::Value,
    pub features: Vec<String>,
    pub is_active: bool,
    pub user_limit: Option<i32>,
    pub location_limit: Option<i32>,
    pub storage_limit_gb: Option<i32>,
    #[sqlx(flatten)]
    pub audit_fields: AuditFields,
}

impl Tenant {
    /// Create a new tenant with default values
    pub fn new(slug: String, name: String, industry: IndustryType) -> Self {
        let now = Utc::now();
        let tier = SubscriptionTier::Free;

        Self {
            id: Uuid::new_v4(),
            slug,
            name: name.clone(),
            display_name: name,
            description: None,
            industry: serde_json::to_value(industry).unwrap_or_default(),
            subscription_tier: tier,
            subscription_status: SubscriptionStatus::Trial,
            trial_ends_at: Some(now + chrono::Duration::days(14)), // 14-day trial
            billing_email: None,
            support_email: None,
            website: None,
            logo_url: None,
            settings: serde_json::json!({}),
            features: industry.default_features(),
            is_active: true,
            user_limit: tier.user_limit(),
            location_limit: tier.location_limit(),
            storage_limit_gb: tier.storage_limit_gb(),
            audit_fields: AuditFields {
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        }
    }

    /// Check if the tenant is on trial
    pub fn is_on_trial(&self) -> bool {
        self.subscription_status == SubscriptionStatus::Trial
            && self.trial_ends_at.map(|end| end > Utc::now()).unwrap_or(false)
    }

    /// Check if the trial has expired
    pub fn is_trial_expired(&self) -> bool {
        self.subscription_status == SubscriptionStatus::Trial
            && self.trial_ends_at.map(|end| end <= Utc::now()).unwrap_or(true)
    }

    /// Check if the tenant has active subscription
    pub fn has_active_subscription(&self) -> bool {
        self.subscription_status == SubscriptionStatus::Active || self.is_on_trial()
    }

    /// Check if a feature is enabled
    pub fn has_feature(&self, feature: &str) -> bool {
        self.features.contains(&feature.to_string())
            || self.subscription_tier.available_features().contains(&feature.to_string())
    }

    /// Enable a feature
    pub fn enable_feature(&mut self, feature: String) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
            self.audit_fields.updated_at = Utc::now();
        }
    }

    /// Disable a feature
    pub fn disable_feature(&mut self, feature: &str) {
        if let Some(pos) = self.features.iter().position(|f| f == feature) {
            self.features.remove(pos);
            self.audit_fields.updated_at = Utc::now();
        }
    }

    /// Update subscription tier
    pub fn update_subscription(&mut self, tier: SubscriptionTier, status: SubscriptionStatus) {
        self.subscription_tier = tier;
        self.subscription_status = status;
        self.user_limit = tier.user_limit();
        self.location_limit = tier.location_limit();
        self.storage_limit_gb = tier.storage_limit_gb();

        // Add tier features
        for feature in tier.available_features() {
            self.enable_feature(feature);
        }

        self.audit_fields.updated_at = Utc::now();
    }

    /// Check if tenant can add more users
    pub fn can_add_user(&self, current_user_count: i32) -> bool {
        self.user_limit.map(|limit| current_user_count < limit).unwrap_or(true)
    }

    /// Check if tenant can add more locations
    pub fn can_add_location(&self, current_location_count: i32) -> bool {
        self.location_limit.map(|limit| current_location_count < limit).unwrap_or(true)
    }

    /// Get the tenant's industry type
    pub fn get_industry_type(&self) -> IndustryType {
        serde_json::from_value(self.industry.clone()).unwrap_or_default()
    }
}

impl SoftDelete for Tenant {
    fn is_deleted(&self) -> bool {
        self.audit_fields.deleted_at.is_some()
    }

    fn delete(&mut self) {
        self.audit_fields.deleted_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
        self.is_active = false;
    }

    fn restore(&mut self) {
        self.audit_fields.deleted_at = None;
        self.audit_fields.updated_at = Utc::now();
        self.is_active = true;
    }
}

impl ValidateEntity for Tenant {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // Slug validation
        if !self.slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return Err(ValidationError::new("invalid_slug_format"));
        }

        if self.slug.len() < 3 || self.slug.len() > 100 {
            return Err(ValidationError::new("invalid_slug_length"));
        }

        // Name validation
        if self.name.trim().is_empty() {
            return Err(ValidationError::new("empty_name"));
        }

        // Email validation (if provided)
        if let Some(email) = &self.billing_email {
            if !email.contains('@') {
                return Err(ValidationError::new("invalid_billing_email"));
            }
        }

        if let Some(email) = &self.support_email {
            if !email.contains('@') {
                return Err(ValidationError::new("invalid_support_email"));
            }
        }

        // URL validation (if provided)
        if let Some(website) = &self.website {
            if !website.starts_with("http://") && !website.starts_with("https://") {
                return Err(ValidationError::new("invalid_website_url"));
            }
        }

        Ok(())
    }
}

/// Request model for creating a new tenant
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 3, max = 100))]
    pub slug: String,

    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(length(max = 255))]
    pub display_name: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub industry: IndustryType,

    #[validate(email)]
    pub billing_email: Option<String>,

    #[validate(email)]
    pub support_email: Option<String>,

    #[validate(url)]
    pub website: Option<String>,

    #[validate(url)]
    pub logo_url: Option<String>,

    pub settings: Option<serde_json::Value>,
}

/// Request model for updating a tenant
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTenantRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,

    #[validate(length(max = 255))]
    pub display_name: Option<String>,

    #[validate(length(max = 1000))]
    pub description: Option<String>,

    pub industry: Option<IndustryType>,

    #[validate(email)]
    pub billing_email: Option<String>,

    #[validate(email)]
    pub support_email: Option<String>,

    #[validate(url)]
    pub website: Option<String>,

    #[validate(url)]
    pub logo_url: Option<String>,

    pub subscription_tier: Option<SubscriptionTier>,
    pub subscription_status: Option<SubscriptionStatus>,
    pub settings: Option<serde_json::Value>,
}

/// Tenant summary for lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSummary {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub display_name: String,
    pub industry: IndustryType,
    pub subscription_tier: SubscriptionTier,
    pub subscription_status: SubscriptionStatus,
    pub is_active: bool,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Tenant> for TenantSummary {
    fn from(tenant: Tenant) -> Self {
        let industry = tenant.get_industry_type();
        Self {
            id: tenant.id,
            slug: tenant.slug,
            name: tenant.name,
            display_name: tenant.display_name,
            industry,
            subscription_tier: tenant.subscription_tier,
            subscription_status: tenant.subscription_status,
            is_active: tenant.is_active,
            trial_ends_at: tenant.trial_ends_at,
            created_at: tenant.audit_fields.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_creation() {
        let slug = "test-company".to_string();
        let name = "Test Company".to_string();
        let industry = IndustryType::Restaurant;

        let tenant = Tenant::new(slug.clone(), name.clone(), industry);

        assert_eq!(tenant.slug, slug);
        assert_eq!(tenant.name, name);
        assert_eq!(tenant.get_industry_type(), industry);
        assert_eq!(tenant.subscription_tier, SubscriptionTier::Free);
        assert_eq!(tenant.subscription_status, SubscriptionStatus::Trial);
        assert!(tenant.is_on_trial());
        assert!(!tenant.is_trial_expired());
    }

    #[test]
    fn test_subscription_limits() {
        let tier = SubscriptionTier::Professional;
        assert_eq!(tier.user_limit(), Some(50));
        assert_eq!(tier.location_limit(), Some(10));
        assert_eq!(tier.storage_limit_gb(), Some(100));

        let tier = SubscriptionTier::Custom;
        assert_eq!(tier.user_limit(), None); // Unlimited
    }

    #[test]
    fn test_feature_management() {
        let mut tenant = Tenant::new(
            "test".to_string(),
            "Test".to_string(),
            IndustryType::Restaurant,
        );

        // Should have default restaurant features
        assert!(tenant.has_feature("table_management"));

        // Add a custom feature
        tenant.enable_feature("custom_feature".to_string());
        assert!(tenant.has_feature("custom_feature"));

        // Remove a feature
        tenant.disable_feature("custom_feature");
        assert!(!tenant.has_feature("custom_feature"));
    }

    #[test]
    fn test_subscription_upgrade() {
        let mut tenant = Tenant::new(
            "test".to_string(),
            "Test".to_string(),
            IndustryType::Restaurant,
        );

        assert_eq!(tenant.subscription_tier, SubscriptionTier::Free);

        tenant.update_subscription(SubscriptionTier::Professional, SubscriptionStatus::Active);

        assert_eq!(tenant.subscription_tier, SubscriptionTier::Professional);
        assert_eq!(tenant.subscription_status, SubscriptionStatus::Active);
        assert!(tenant.has_feature("advanced_reporting"));
    }

    #[test]
    fn test_user_and_location_limits() {
        let tenant = Tenant::new(
            "test".to_string(),
            "Test".to_string(),
            IndustryType::Restaurant,
        );

        // Free tier allows 2 users and 1 location
        assert!(tenant.can_add_user(1)); // 1 < 2
        assert!(!tenant.can_add_user(2)); // 2 >= 2

        assert!(!tenant.can_add_location(1)); // 1 >= 1
    }

    #[test]
    fn test_tenant_validation() {
        let tenant = Tenant::new(
            "Invalid Slug!".to_string(), // Invalid characters
            "Test".to_string(),
            IndustryType::Restaurant,
        );

        assert!(tenant.validate().is_err());

        let tenant = Tenant::new(
            "valid-slug".to_string(),
            "Test Company".to_string(),
            IndustryType::Restaurant,
        );

        assert!(tenant.validate().is_ok());
    }
}