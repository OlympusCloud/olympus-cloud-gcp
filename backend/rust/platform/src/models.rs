// ============================================================================
// OLYMPUS CLOUD - PLATFORM MODELS
// ============================================================================
// Module: platform/src/models.rs
// Description: Platform service models for tenant management, RBAC, and configuration
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use chrono::{DateTime, Utc};
use olympus_shared::models::{AuditFields, SoftDelete, TenantScoped, ValidateEntity};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use std::collections::HashMap;
use uuid::Uuid;
use validator::{Validate, ValidationError};

// ============================================================================
// TENANT MANAGEMENT MODELS
// ============================================================================

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
    /// Get maximum user limit for tier
    pub fn max_users(&self) -> Option<i32> {
        match self {
            Self::Free => Some(5),
            Self::Starter => Some(25),
            Self::Professional => Some(100),
            Self::Enterprise => Some(500),
            Self::Custom => None, // Unlimited
        }
    }

    /// Get maximum location limit for tier
    pub fn max_locations(&self) -> Option<i32> {
        match self {
            Self::Free => Some(1),
            Self::Starter => Some(3),
            Self::Professional => Some(10),
            Self::Enterprise => Some(50),
            Self::Custom => None, // Unlimited
        }
    }

    /// Check if tier supports feature
    pub fn supports_feature(&self, feature: &str) -> bool {
        match (self, feature) {
            (Self::Free, "basic_analytics") => true,
            (Self::Free, "advanced_analytics") => false,
            (Self::Free, "api_access") => false,
            (Self::Free, "white_labeling") => false,
            (Self::Free, "priority_support") => false,

            (Self::Starter, "basic_analytics") => true,
            (Self::Starter, "advanced_analytics") => false,
            (Self::Starter, "api_access") => true,
            (Self::Starter, "white_labeling") => false,
            (Self::Starter, "priority_support") => false,

            (Self::Professional, "basic_analytics") => true,
            (Self::Professional, "advanced_analytics") => true,
            (Self::Professional, "api_access") => true,
            (Self::Professional, "white_labeling") => false,
            (Self::Professional, "priority_support") => true,

            (Self::Enterprise, _) => true, // All features
            (Self::Custom, _) => true,     // All features

            (_, _) => false,
        }
    }
}

/// Tenant status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "tenant_status", rename_all = "UPPERCASE")]
pub enum TenantStatus {
    Active,
    Suspended,
    Cancelled,
    Trial,
    Pending,
}

impl Default for TenantStatus {
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
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub industry: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub status: TenantStatus,
    pub subscription_tier: SubscriptionTier,
    pub subscription_started_at: Option<DateTime<Utc>>,
    pub subscription_expires_at: Option<DateTime<Utc>>,
    pub trial_started_at: Option<DateTime<Utc>>,
    pub trial_expires_at: Option<DateTime<Utc>>,
    pub billing_email: Option<String>,
    pub billing_address: serde_json::Value,
    pub feature_flags: serde_json::Value,
    pub settings: serde_json::Value,
    pub metadata: serde_json::Value,
    #[sqlx(flatten)]
    pub audit_fields: AuditFields,
}

impl Tenant {
    /// Create a new tenant with default values
    pub fn new(slug: String, name: String, industry: String) -> Self {
        let now = Utc::now();
        let trial_expires_at = now + chrono::Duration::days(30); // 30-day trial

        Self {
            id: Uuid::new_v4(),
            slug,
            name: name.clone(),
            display_name: Some(name),
            description: None,
            industry,
            website: None,
            logo_url: None,
            status: TenantStatus::Trial,
            subscription_tier: SubscriptionTier::Free,
            subscription_started_at: None,
            subscription_expires_at: None,
            trial_started_at: Some(now),
            trial_expires_at: Some(trial_expires_at),
            billing_email: None,
            billing_address: serde_json::json!({}),
            feature_flags: serde_json::json!({}),
            settings: serde_json::json!({
                "timezone": "UTC",
                "currency": "USD",
                "language": "en",
                "date_format": "YYYY-MM-DD",
                "time_format": "24h"
            }),
            metadata: serde_json::json!({}),
            audit_fields: AuditFields {
                created_at: now,
                updated_at: now,
                deleted_at: None,
            },
        }
    }

    /// Get display name or fallback to name
    pub fn display_name(&self) -> String {
        self.display_name.clone().unwrap_or_else(|| self.name.clone())
    }

    /// Check if tenant is active
    pub fn is_active(&self) -> bool {
        self.status == TenantStatus::Active
    }

    /// Check if tenant is in trial
    pub fn is_trial(&self) -> bool {
        self.status == TenantStatus::Trial
    }

    /// Check if trial has expired
    pub fn is_trial_expired(&self) -> bool {
        self.trial_expires_at
            .map(|expires| expires <= Utc::now())
            .unwrap_or(false)
    }

    /// Check if subscription has expired
    pub fn is_subscription_expired(&self) -> bool {
        self.subscription_expires_at
            .map(|expires| expires <= Utc::now())
            .unwrap_or(false)
    }

    /// Check if tenant can access feature
    pub fn can_access_feature(&self, feature: &str) -> bool {
        if !self.is_active() && !self.is_trial() {
            return false;
        }

        // Check feature flags first
        if let Some(flag_value) = self.feature_flags.get(feature) {
            if let Some(enabled) = flag_value.as_bool() {
                return enabled;
            }
        }

        // Fall back to subscription tier permissions
        self.subscription_tier.supports_feature(feature)
    }

    /// Update subscription
    pub fn update_subscription(&mut self, tier: SubscriptionTier, expires_at: Option<DateTime<Utc>>) {
        self.subscription_tier = tier;
        self.subscription_started_at = Some(Utc::now());
        self.subscription_expires_at = expires_at;
        self.status = TenantStatus::Active;
        self.audit_fields.updated_at = Utc::now();
    }

    /// Activate tenant (end trial, start subscription)
    pub fn activate(&mut self) {
        self.status = TenantStatus::Active;
        self.audit_fields.updated_at = Utc::now();
    }

    /// Suspend tenant
    pub fn suspend(&mut self) {
        self.status = TenantStatus::Suspended;
        self.audit_fields.updated_at = Utc::now();
    }

    /// Cancel tenant subscription
    pub fn cancel(&mut self) {
        self.status = TenantStatus::Cancelled;
        self.audit_fields.updated_at = Utc::now();
    }
}

impl TenantScoped for Tenant {
    fn tenant_id(&self) -> Uuid {
        self.id
    }
}

impl SoftDelete for Tenant {
    fn is_deleted(&self) -> bool {
        self.audit_fields.deleted_at.is_some()
    }

    fn delete(&mut self) {
        self.audit_fields.deleted_at = Some(Utc::now());
        self.audit_fields.updated_at = Utc::now();
        self.status = TenantStatus::Cancelled;
    }

    fn restore(&mut self) {
        self.audit_fields.deleted_at = None;
        self.audit_fields.updated_at = Utc::now();
        self.status = TenantStatus::Active;
    }
}

impl ValidateEntity for Tenant {
    type Error = ValidationError;

    fn validate(&self) -> Result<(), Self::Error> {
        // Slug validation
        if self.slug.len() < 2 || self.slug.len() > 50 {
            return Err(ValidationError::new("invalid_slug_length"));
        }

        if !self.slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            return Err(ValidationError::new("invalid_slug_characters"));
        }

        // Name validation
        if self.name.is_empty() || self.name.len() > 100 {
            return Err(ValidationError::new("invalid_name_length"));
        }

        // Website validation (if provided)
        if let Some(website) = &self.website {
            if !website.starts_with("http://") && !website.starts_with("https://") {
                return Err(ValidationError::new("invalid_website_format"));
            }
        }

        // Billing email validation (if provided)
        if let Some(email) = &self.billing_email {
            if !email.contains('@') {
                return Err(ValidationError::new("invalid_billing_email"));
            }
        }

        Ok(())
    }
}

/// Request model for creating a new tenant
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateTenantRequest {
    #[validate(length(min = 2, max = 50))]
    pub slug: String,

    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(max = 100))]
    pub display_name: Option<String>,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    #[validate(length(min = 1, max = 50))]
    pub industry: String,

    #[validate(url)]
    pub website: Option<String>,

    #[validate(url)]
    pub logo_url: Option<String>,

    pub subscription_tier: Option<SubscriptionTier>,

    #[validate(email)]
    pub billing_email: Option<String>,

    pub billing_address: Option<serde_json::Value>,
    pub feature_flags: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Request model for updating a tenant
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateTenantRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    #[validate(length(max = 100))]
    pub display_name: Option<String>,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    #[validate(length(min = 1, max = 50))]
    pub industry: Option<String>,

    #[validate(url)]
    pub website: Option<String>,

    #[validate(url)]
    pub logo_url: Option<String>,

    pub status: Option<TenantStatus>,
    pub subscription_tier: Option<SubscriptionTier>,

    #[validate(email)]
    pub billing_email: Option<String>,

    pub billing_address: Option<serde_json::Value>,
    pub feature_flags: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Subscription update request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateSubscriptionRequest {
    pub tier: SubscriptionTier,
    pub expires_at: Option<DateTime<Utc>>,

    #[validate(email)]
    pub billing_email: Option<String>,

    pub billing_address: Option<serde_json::Value>,
}

/// Feature flag update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFeatureFlagsRequest {
    pub flags: HashMap<String, bool>,
}

/// Tenant summary for lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSummary {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub display_name: String,
    pub industry: String,
    pub status: TenantStatus,
    pub subscription_tier: SubscriptionTier,
    pub subscription_expires_at: Option<DateTime<Utc>>,
    pub trial_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<Tenant> for TenantSummary {
    fn from(tenant: Tenant) -> Self {
        let display_name = tenant.display_name();
        Self {
            id: tenant.id,
            slug: tenant.slug,
            name: tenant.name,
            display_name,
            industry: tenant.industry,
            status: tenant.status,
            subscription_tier: tenant.subscription_tier,
            subscription_expires_at: tenant.subscription_expires_at,
            trial_expires_at: tenant.trial_expires_at,
            created_at: tenant.audit_fields.created_at,
        }
    }
}

/// Tenant detail response (without sensitive fields)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantDetail {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub industry: String,
    pub website: Option<String>,
    pub logo_url: Option<String>,
    pub status: TenantStatus,
    pub subscription_tier: SubscriptionTier,
    pub subscription_started_at: Option<DateTime<Utc>>,
    pub subscription_expires_at: Option<DateTime<Utc>>,
    pub trial_started_at: Option<DateTime<Utc>>,
    pub trial_expires_at: Option<DateTime<Utc>>,
    pub feature_flags: serde_json::Value,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Tenant> for TenantDetail {
    fn from(tenant: Tenant) -> Self {
        let display_name = tenant.display_name();
        Self {
            id: tenant.id,
            slug: tenant.slug,
            name: tenant.name,
            display_name,
            description: tenant.description,
            industry: tenant.industry,
            website: tenant.website,
            logo_url: tenant.logo_url,
            status: tenant.status,
            subscription_tier: tenant.subscription_tier,
            subscription_started_at: tenant.subscription_started_at,
            subscription_expires_at: tenant.subscription_expires_at,
            trial_started_at: tenant.trial_started_at,
            trial_expires_at: tenant.trial_expires_at,
            feature_flags: tenant.feature_flags,
            settings: tenant.settings,
            created_at: tenant.audit_fields.created_at,
            updated_at: tenant.audit_fields.updated_at,
        }
    }
}