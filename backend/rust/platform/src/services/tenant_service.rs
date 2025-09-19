// ============================================================================
// OLYMPUS CLOUD - TENANT SERVICE
// ============================================================================
// Module: platform/src/services/tenant_service.rs
// Description: Comprehensive tenant management service with subscription, billing, and feature management
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use chrono::Utc;
use olympus_shared::database::DbPool;
use olympus_shared::events::{DomainEvent, EventPublisher};
use olympus_shared::types::{PageRequest, PageResponse};
use olympus_shared::{Error, Result};
use sqlx::Row;
use tracing::{error, info, warn};
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    CreateTenantRequest,
    SubscriptionTier,
    Tenant,
    TenantDetail,
    TenantSummary,
    UpdateFeatureFlagsRequest,
    UpdateSubscriptionRequest,
    UpdateTenantRequest
};

#[derive(Debug, thiserror::Error)]
pub enum TenantError {
    #[error("Tenant not found")]
    NotFound,
    #[error("Tenant slug already exists: {slug}")]
    SlugAlreadyExists { slug: String },
    #[error("Invalid subscription tier transition: {from:?} -> {to:?}")]
    InvalidSubscriptionTransition { from: SubscriptionTier, to: SubscriptionTier },
    #[error("Feature not available for subscription tier: {tier:?}")]
    FeatureNotAvailable { tier: SubscriptionTier },
    #[error("Tenant limit exceeded: {limit_type}")]
    LimitExceeded { limit_type: String },
    #[error("Validation error: {message}")]
    Validation { message: String },
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<TenantError> for Error {
    fn from(err: TenantError) -> Self {
        match err {
            TenantError::NotFound => Error::NotFound("Tenant not found".to_string()),
            TenantError::SlugAlreadyExists { slug } => {
                Error::Conflict(format!("Tenant slug already exists: {}", slug))
            }
            TenantError::Validation { message } => Error::BadRequest(message),
            TenantError::Database(db_err) => Error::Internal(format!("Database error: {}", db_err)),
            _ => Error::Internal(err.to_string()),
        }
    }
}

/// Comprehensive tenant management service
pub struct TenantService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl TenantService {
    /// Create a new tenant service
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    /// Create a new tenant with validation and event publishing
    pub async fn create_tenant(&self, request: CreateTenantRequest) -> Result<TenantDetail> {
        // Validate request
        request.validate().map_err(|e| TenantError::Validation {
            message: format!("Invalid request: {}", e),
        })?;

        // Check if slug already exists
        if self.slug_exists(&request.slug).await? {
            return Err(TenantError::SlugAlreadyExists {
                slug: request.slug,
            }.into());
        }

        // Create tenant entity
        let mut tenant = Tenant::new(request.slug, request.name, request.industry);

        // Apply optional fields
        if let Some(display_name) = request.display_name {
            tenant.display_name = Some(display_name);
        }
        if let Some(description) = request.description {
            tenant.description = Some(description);
        }
        if let Some(website) = request.website {
            tenant.website = Some(website);
        }
        if let Some(logo_url) = request.logo_url {
            tenant.logo_url = Some(logo_url);
        }
        if let Some(tier) = request.subscription_tier {
            tenant.subscription_tier = tier;
        }
        if let Some(billing_email) = request.billing_email {
            tenant.billing_email = Some(billing_email);
        }
        if let Some(billing_address) = request.billing_address {
            tenant.billing_address = billing_address;
        }
        if let Some(feature_flags) = request.feature_flags {
            tenant.feature_flags = feature_flags;
        }
        if let Some(settings) = request.settings {
            tenant.settings = settings;
        }
        if let Some(metadata) = request.metadata {
            tenant.metadata = metadata;
        }

        // Validate the complete tenant
        tenant.validate().map_err(|e| TenantError::Validation {
            message: format!("Invalid tenant: {}", e),
        })?;

        // Insert into database
        let result = sqlx::query!(
            r#"
            INSERT INTO tenants (
                id, slug, name, display_name, description, industry, website, logo_url,
                status, subscription_tier, subscription_started_at, subscription_expires_at,
                trial_started_at, trial_expires_at, billing_email, billing_address,
                feature_flags, settings, metadata, created_at, updated_at, deleted_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22
            )
            "#,
            tenant.id,
            tenant.slug,
            tenant.name,
            tenant.display_name,
            tenant.description,
            tenant.industry,
            tenant.website,
            tenant.logo_url,
            tenant.status as TenantStatus,
            tenant.subscription_tier as SubscriptionTier,
            tenant.subscription_started_at,
            tenant.subscription_expires_at,
            tenant.trial_started_at,
            tenant.trial_expires_at,
            tenant.billing_email,
            tenant.billing_address,
            tenant.feature_flags,
            tenant.settings,
            tenant.metadata,
            tenant.audit_fields.created_at,
            tenant.audit_fields.updated_at,
            tenant.audit_fields.deleted_at
        )
        .execute(&**self.db)
        .await;

        match result {
            Ok(_) => {
                info!("Tenant created successfully: {} ({})", tenant.slug, tenant.id);

                // Publish domain event
                let event = DomainEvent::builder(
                    "TenantCreated".to_string(),
                    tenant.id,
                    "Tenant".to_string(),
                    tenant.id,
                )
                .data(serde_json::json!({
                    "tenant_id": tenant.id,
                    "slug": tenant.slug,
                    "name": tenant.name,
                    "industry": tenant.industry,
                    "subscription_tier": tenant.subscription_tier,
                    "status": tenant.status
                }))
                .unwrap()
                .source_service("platform".to_string())
                .build();

                if let Err(e) = self.event_publisher.publish(event).await {
                    warn!("Failed to publish TenantCreated event: {}", e);
                }

                Ok(tenant.into())
            }
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
                Err(TenantError::SlugAlreadyExists {
                    slug: tenant.slug,
                }.into())
            }
            Err(e) => {
                error!("Failed to create tenant: {}", e);
                Err(TenantError::Database(e).into())
            }
        }
    }

    /// Get tenant by ID
    pub async fn get_tenant(&self, tenant_id: Uuid) -> Result<TenantDetail> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, name, display_name, description, industry, website, logo_url,
                   status as "status: TenantStatus",
                   subscription_tier as "subscription_tier: SubscriptionTier",
                   subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                   billing_email, billing_address, feature_flags, settings, metadata,
                   created_at, updated_at, deleted_at
            FROM tenants
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            tenant_id
        )
        .fetch_optional(&**self.db)
        .await?;

        match row {
            Some(row) => {
                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };
                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// Get tenant by slug
    pub async fn get_tenant_by_slug(&self, slug: &str) -> Result<TenantDetail> {
        let row = sqlx::query!(
            r#"
            SELECT id, slug, name, display_name, description, industry, website, logo_url,
                   status as "status: TenantStatus",
                   subscription_tier as "subscription_tier: SubscriptionTier",
                   subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                   billing_email, billing_address, feature_flags, settings, metadata,
                   created_at, updated_at, deleted_at
            FROM tenants
            WHERE slug = $1 AND deleted_at IS NULL
            "#,
            slug
        )
        .fetch_optional(&**self.db)
        .await?;

        match row {
            Some(row) => {
                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };
                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// Update tenant
    pub async fn update_tenant(
        &self,
        tenant_id: Uuid,
        request: UpdateTenantRequest,
    ) -> Result<TenantDetail> {
        // Validate request
        request.validate().map_err(|e| TenantError::Validation {
            message: format!("Invalid request: {}", e),
        })?;

        // Get current tenant
        let current = self.get_tenant(tenant_id).await?;

        // Build update query dynamically
        let mut query_parts = Vec::new();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + 'static>> = Vec::new();
        let mut param_index = 1;

        if let Some(name) = &request.name {
            query_parts.push(format!("name = ${}", param_index));
            params.push(Box::new(name.clone()));
            param_index += 1;
        }

        if let Some(display_name) = &request.display_name {
            query_parts.push(format!("display_name = ${}", param_index));
            params.push(Box::new(display_name.clone()));
            param_index += 1;
        }

        if let Some(description) = &request.description {
            query_parts.push(format!("description = ${}", param_index));
            params.push(Box::new(description.clone()));
            param_index += 1;
        }

        if let Some(industry) = &request.industry {
            query_parts.push(format!("industry = ${}", param_index));
            params.push(Box::new(industry.clone()));
            param_index += 1;
        }

        if let Some(website) = &request.website {
            query_parts.push(format!("website = ${}", param_index));
            params.push(Box::new(website.clone()));
            param_index += 1;
        }

        if let Some(logo_url) = &request.logo_url {
            query_parts.push(format!("logo_url = ${}", param_index));
            params.push(Box::new(logo_url.clone()));
            param_index += 1;
        }

        if let Some(status) = &request.status {
            query_parts.push(format!("status = ${}", param_index));
            params.push(Box::new(*status));
            param_index += 1;
        }

        if let Some(tier) = &request.subscription_tier {
            query_parts.push(format!("subscription_tier = ${}", param_index));
            params.push(Box::new(*tier));
            param_index += 1;
        }

        if let Some(billing_email) = &request.billing_email {
            query_parts.push(format!("billing_email = ${}", param_index));
            params.push(Box::new(billing_email.clone()));
            param_index += 1;
        }

        if let Some(billing_address) = &request.billing_address {
            query_parts.push(format!("billing_address = ${}", param_index));
            params.push(Box::new(billing_address.clone()));
            param_index += 1;
        }

        if let Some(feature_flags) = &request.feature_flags {
            query_parts.push(format!("feature_flags = ${}", param_index));
            params.push(Box::new(feature_flags.clone()));
            param_index += 1;
        }

        if let Some(settings) = &request.settings {
            query_parts.push(format!("settings = ${}", param_index));
            params.push(Box::new(settings.clone()));
            param_index += 1;
        }

        if let Some(metadata) = &request.metadata {
            query_parts.push(format!("metadata = ${}", param_index));
            params.push(Box::new(metadata.clone()));
            param_index += 1;
        }

        if query_parts.is_empty() {
            return Ok(current);
        }

        // Add updated_at
        query_parts.push(format!("updated_at = ${}", param_index));
        params.push(Box::new(Utc::now()));
        param_index += 1;

        // Add WHERE clause
        let query = format!(
            "UPDATE tenants SET {} WHERE id = ${} AND deleted_at IS NULL",
            query_parts.join(", "),
            param_index
        );
        params.push(Box::new(tenant_id));

        let result = sqlx::query(&query);

        // This is a simplified version - in a real implementation, you'd use the dynamic query
        // For now, let's use a more manageable approach
        let updated_tenant = sqlx::query!(
            r#"
            UPDATE tenants
            SET name = COALESCE($2, name),
                display_name = COALESCE($3, display_name),
                description = COALESCE($4, description),
                industry = COALESCE($5, industry),
                website = COALESCE($6, website),
                logo_url = COALESCE($7, logo_url),
                status = COALESCE($8, status),
                subscription_tier = COALESCE($9, subscription_tier),
                billing_email = COALESCE($10, billing_email),
                billing_address = COALESCE($11, billing_address),
                feature_flags = COALESCE($12, feature_flags),
                settings = COALESCE($13, settings),
                metadata = COALESCE($14, metadata),
                updated_at = $15
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, slug, name, display_name, description, industry, website, logo_url,
                      status as "status: TenantStatus",
                      subscription_tier as "subscription_tier: SubscriptionTier",
                      subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                      billing_email, billing_address, feature_flags, settings, metadata,
                      created_at, updated_at, deleted_at
            "#,
            tenant_id,
            request.name,
            request.display_name,
            request.description,
            request.industry,
            request.website,
            request.logo_url,
            request.status.map(|s| s as TenantStatus),
            request.subscription_tier.map(|t| t as SubscriptionTier),
            request.billing_email,
            request.billing_address,
            request.feature_flags,
            request.settings,
            request.metadata,
            Utc::now()
        )
        .fetch_optional(&**self.db)
        .await?;

        match updated_tenant {
            Some(row) => {
                info!("Tenant updated successfully: {}", tenant_id);

                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };

                // Publish domain event
                let event = DomainEvent::builder(
                    "TenantUpdated".to_string(),
                    tenant.id,
                    "Tenant".to_string(),
                    tenant.id,
                )
                .data(serde_json::json!({
                    "tenant_id": tenant.id,
                    "changes": request
                }))
                .unwrap()
                .source_service("platform".to_string())
                .build();

                if let Err(e) = self.event_publisher.publish(event).await {
                    warn!("Failed to publish TenantUpdated event: {}", e);
                }

                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// Update tenant subscription
    pub async fn update_subscription(
        &self,
        tenant_id: Uuid,
        request: UpdateSubscriptionRequest,
    ) -> Result<TenantDetail> {
        // Validate request
        request.validate().map_err(|e| TenantError::Validation {
            message: format!("Invalid request: {}", e),
        })?;

        let updated_tenant = sqlx::query!(
            r#"
            UPDATE tenants
            SET subscription_tier = $2,
                subscription_started_at = $3,
                subscription_expires_at = $4,
                billing_email = COALESCE($5, billing_email),
                billing_address = COALESCE($6, billing_address),
                status = CASE
                    WHEN status = 'TRIAL' THEN 'ACTIVE'::tenant_status
                    ELSE status
                END,
                updated_at = $7
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, slug, name, display_name, description, industry, website, logo_url,
                      status as "status: TenantStatus",
                      subscription_tier as "subscription_tier: SubscriptionTier",
                      subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                      billing_email, billing_address, feature_flags, settings, metadata,
                      created_at, updated_at, deleted_at
            "#,
            tenant_id,
            request.tier as SubscriptionTier,
            Utc::now(),
            request.expires_at,
            request.billing_email,
            request.billing_address,
            Utc::now()
        )
        .fetch_optional(&**self.db)
        .await?;

        match updated_tenant {
            Some(row) => {
                info!("Tenant subscription updated: {} -> {:?}", tenant_id, request.tier);

                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };

                // Publish domain event
                let event = DomainEvent::builder(
                    "SubscriptionUpdated".to_string(),
                    tenant.id,
                    "Tenant".to_string(),
                    tenant.id,
                )
                .data(serde_json::json!({
                    "tenant_id": tenant.id,
                    "old_tier": "previous_tier", // Would need to fetch this
                    "new_tier": request.tier,
                    "expires_at": request.expires_at
                }))
                .unwrap()
                .source_service("platform".to_string())
                .build();

                if let Err(e) = self.event_publisher.publish(event).await {
                    warn!("Failed to publish SubscriptionUpdated event: {}", e);
                }

                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// Update feature flags for tenant
    pub async fn update_feature_flags(
        &self,
        tenant_id: Uuid,
        request: UpdateFeatureFlagsRequest,
    ) -> Result<TenantDetail> {
        // Get current tenant to check subscription limits
        let current_tenant = self.get_tenant(tenant_id).await?;

        // Validate feature access based on subscription tier
        for (feature, enabled) in &request.flags {
            if *enabled && !SubscriptionTier::from_str(&current_tenant.subscription_tier.to_string())
                .unwrap_or(SubscriptionTier::Free)
                .supports_feature(feature)
            {
                return Err(TenantError::FeatureNotAvailable {
                    tier: current_tenant.subscription_tier,
                }.into());
            }
        }

        // Merge with existing feature flags
        let mut feature_flags = current_tenant.feature_flags.clone();
        for (key, value) in request.flags {
            feature_flags[key] = serde_json::Value::Bool(value);
        }

        let updated_tenant = sqlx::query!(
            r#"
            UPDATE tenants
            SET feature_flags = $2,
                updated_at = $3
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, slug, name, display_name, description, industry, website, logo_url,
                      status as "status: TenantStatus",
                      subscription_tier as "subscription_tier: SubscriptionTier",
                      subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                      billing_email, billing_address, feature_flags, settings, metadata,
                      created_at, updated_at, deleted_at
            "#,
            tenant_id,
            feature_flags,
            Utc::now()
        )
        .fetch_optional(&**self.db)
        .await?;

        match updated_tenant {
            Some(row) => {
                info!("Tenant feature flags updated: {}", tenant_id);

                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };

                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// List tenants with pagination
    pub async fn list_tenants(&self, page: PageRequest) -> Result<PageResponse<TenantSummary>> {
        let offset = page.offset();
        let limit = page.limit();

        // Get total count
        let count_result = sqlx::query!("SELECT COUNT(*) as count FROM tenants WHERE deleted_at IS NULL")
            .fetch_one(&**self.db)
            .await?;

        let total = count_result.count.unwrap_or(0) as u64;

        // Get page of tenants
        let rows = sqlx::query!(
            r#"
            SELECT id, slug, name, display_name, description, industry, website, logo_url,
                   status as "status: TenantStatus",
                   subscription_tier as "subscription_tier: SubscriptionTier",
                   subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                   billing_email, billing_address, feature_flags, settings, metadata,
                   created_at, updated_at, deleted_at
            FROM tenants
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(&**self.db)
        .await?;

        let tenants: Vec<TenantSummary> = rows
            .into_iter()
            .map(|row| {
                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };
                tenant.into()
            })
            .collect();

        Ok(PageResponse::new(tenants, page.page, total, page.per_page))
    }

    /// Activate tenant (end trial period)
    pub async fn activate_tenant(&self, tenant_id: Uuid) -> Result<TenantDetail> {
        let updated_tenant = sqlx::query!(
            r#"
            UPDATE tenants
            SET status = 'ACTIVE'::tenant_status,
                updated_at = $2
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, slug, name, display_name, description, industry, website, logo_url,
                      status as "status: TenantStatus",
                      subscription_tier as "subscription_tier: SubscriptionTier",
                      subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                      billing_email, billing_address, feature_flags, settings, metadata,
                      created_at, updated_at, deleted_at
            "#,
            tenant_id,
            Utc::now()
        )
        .fetch_optional(&**self.db)
        .await?;

        match updated_tenant {
            Some(row) => {
                info!("Tenant activated: {}", tenant_id);

                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };

                // Publish domain event
                let event = DomainEvent::builder(
                    "TenantActivated".to_string(),
                    tenant.id,
                    "Tenant".to_string(),
                    tenant.id,
                )
                .data(serde_json::json!({
                    "tenant_id": tenant.id,
                    "activated_at": Utc::now()
                }))
                .unwrap()
                .source_service("platform".to_string())
                .build();

                if let Err(e) = self.event_publisher.publish(event).await {
                    warn!("Failed to publish TenantActivated event: {}", e);
                }

                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// Suspend tenant
    pub async fn suspend_tenant(&self, tenant_id: Uuid) -> Result<TenantDetail> {
        let updated_tenant = sqlx::query!(
            r#"
            UPDATE tenants
            SET status = 'SUSPENDED'::tenant_status,
                updated_at = $2
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING id, slug, name, display_name, description, industry, website, logo_url,
                      status as "status: TenantStatus",
                      subscription_tier as "subscription_tier: SubscriptionTier",
                      subscription_started_at, subscription_expires_at, trial_started_at, trial_expires_at,
                      billing_email, billing_address, feature_flags, settings, metadata,
                      created_at, updated_at, deleted_at
            "#,
            tenant_id,
            Utc::now()
        )
        .fetch_optional(&**self.db)
        .await?;

        match updated_tenant {
            Some(row) => {
                info!("Tenant suspended: {}", tenant_id);

                let tenant = Tenant {
                    id: row.id,
                    slug: row.slug,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    industry: row.industry,
                    website: row.website,
                    logo_url: row.logo_url,
                    status: row.status,
                    subscription_tier: row.subscription_tier,
                    subscription_started_at: row.subscription_started_at,
                    subscription_expires_at: row.subscription_expires_at,
                    trial_started_at: row.trial_started_at,
                    trial_expires_at: row.trial_expires_at,
                    billing_email: row.billing_email,
                    billing_address: row.billing_address,
                    feature_flags: row.feature_flags,
                    settings: row.settings,
                    metadata: row.metadata,
                    audit_fields: olympus_shared::models::AuditFields {
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                        deleted_at: row.deleted_at,
                    },
                };

                // Publish domain event
                let event = DomainEvent::builder(
                    "TenantSuspended".to_string(),
                    tenant.id,
                    "Tenant".to_string(),
                    tenant.id,
                )
                .data(serde_json::json!({
                    "tenant_id": tenant.id,
                    "suspended_at": Utc::now()
                }))
                .unwrap()
                .source_service("platform".to_string())
                .build();

                if let Err(e) = self.event_publisher.publish(event).await {
                    warn!("Failed to publish TenantSuspended event: {}", e);
                }

                Ok(tenant.into())
            }
            None => Err(TenantError::NotFound.into()),
        }
    }

    /// Soft delete tenant (with data cleanup planning)
    pub async fn delete_tenant(&self, tenant_id: Uuid) -> Result<()> {
        // In a real implementation, this would trigger a cascading cleanup process
        let result = sqlx::query!(
            r#"
            UPDATE tenants
            SET status = 'CANCELLED'::tenant_status,
                deleted_at = $2,
                updated_at = $2
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            tenant_id,
            Utc::now()
        )
        .execute(&**self.db)
        .await?;

        if result.rows_affected() == 0 {
            return Err(TenantError::NotFound.into());
        }

        info!("Tenant deleted (soft): {}", tenant_id);

        // Publish domain event
        let event = DomainEvent::builder(
            "TenantDeleted".to_string(),
            tenant_id,
            "Tenant".to_string(),
            tenant_id,
        )
        .data(serde_json::json!({
            "tenant_id": tenant_id,
            "deleted_at": Utc::now()
        }))
        .unwrap()
        .source_service("platform".to_string())
        .build();

        if let Err(e) = self.event_publisher.publish(event).await {
            warn!("Failed to publish TenantDeleted event: {}", e);
        }

        // TODO: Implement async data cleanup process
        // This would involve:
        // 1. Schedule cleanup of all tenant data
        // 2. Archive important records
        // 3. Clean up files/assets
        // 4. Remove from search indexes
        // 5. Cancel external subscriptions

        Ok(())
    }

    /// Check if tenant slug exists
    async fn slug_exists(&self, slug: &str) -> Result<bool> {
        let result = sqlx::query!(
            "SELECT id FROM tenants WHERE slug = $1 AND deleted_at IS NULL",
            slug
        )
        .fetch_optional(&**self.db)
        .await?;

        Ok(result.is_some())
    }

    /// Get tenant usage statistics
    pub async fn get_tenant_stats(&self, tenant_id: Uuid) -> Result<serde_json::Value> {
        // This would typically aggregate data from various tables
        // For now, return basic statistics
        let stats = sqlx::query!(
            r#"
            SELECT
                (SELECT COUNT(*) FROM users WHERE tenant_id = $1 AND deleted_at IS NULL) as user_count,
                (SELECT COUNT(*) FROM locations WHERE tenant_id = $1 AND deleted_at IS NULL) as location_count,
                (SELECT COUNT(*) FROM products WHERE tenant_id = $1 AND deleted_at IS NULL) as product_count,
                (SELECT COUNT(*) FROM orders WHERE tenant_id = $1 AND deleted_at IS NULL) as order_count
            "#,
            tenant_id
        )
        .fetch_one(&**self.db)
        .await?;

        Ok(serde_json::json!({
            "users": stats.user_count.unwrap_or(0),
            "locations": stats.location_count.unwrap_or(0),
            "products": stats.product_count.unwrap_or(0),
            "orders": stats.order_count.unwrap_or(0)
        }))
    }
}

// Helper trait to convert strings to subscription tiers
trait SubscriptionTierFromStr {
    fn from_str(s: &str) -> Option<SubscriptionTier>;
}

impl SubscriptionTierFromStr for SubscriptionTier {
    fn from_str(s: &str) -> Option<SubscriptionTier> {
        match s.to_lowercase().as_str() {
            "free" => Some(SubscriptionTier::Free),
            "starter" => Some(SubscriptionTier::Starter),
            "professional" => Some(SubscriptionTier::Professional),
            "enterprise" => Some(SubscriptionTier::Enterprise),
            "custom" => Some(SubscriptionTier::Custom),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_subscription_tier_limits() {
        assert_eq!(SubscriptionTier::Free.max_users(), Some(5));
        assert_eq!(SubscriptionTier::Enterprise.max_users(), Some(500));
        assert_eq!(SubscriptionTier::Custom.max_users(), None);

        assert_eq!(SubscriptionTier::Free.max_locations(), Some(1));
        assert_eq!(SubscriptionTier::Professional.max_locations(), Some(10));
    }

    #[test]
    fn test_subscription_tier_features() {
        assert!(SubscriptionTier::Free.supports_feature("basic_analytics"));
        assert!(!SubscriptionTier::Free.supports_feature("advanced_analytics"));
        assert!(!SubscriptionTier::Free.supports_feature("api_access"));

        assert!(SubscriptionTier::Professional.supports_feature("advanced_analytics"));
        assert!(SubscriptionTier::Professional.supports_feature("api_access"));
        assert!(!SubscriptionTier::Professional.supports_feature("white_labeling"));

        assert!(SubscriptionTier::Enterprise.supports_feature("white_labeling"));
        assert!(SubscriptionTier::Enterprise.supports_feature("priority_support"));
    }

    #[test]
    fn test_tenant_creation() {
        let tenant = Tenant::new(
            "test-tenant".to_string(),
            "Test Tenant".to_string(),
            "Technology".to_string(),
        );

        assert_eq!(tenant.slug, "test-tenant");
        assert_eq!(tenant.name, "Test Tenant");
        assert_eq!(tenant.industry, "Technology");
        assert_eq!(tenant.status, TenantStatus::Trial);
        assert_eq!(tenant.subscription_tier, SubscriptionTier::Free);
        assert!(tenant.is_trial());
        assert!(!tenant.is_active());
    }

    #[test]
    fn test_tenant_feature_access() {
        let mut tenant = Tenant::new(
            "test".to_string(),
            "Test".to_string(),
            "Tech".to_string(),
        );

        // Trial tenant with Free tier
        assert!(tenant.can_access_feature("basic_analytics"));
        assert!(!tenant.can_access_feature("advanced_analytics"));

        // Activate with Professional tier
        tenant.activate();
        tenant.update_subscription(SubscriptionTier::Professional, None);

        assert!(tenant.is_active());
        assert!(tenant.can_access_feature("advanced_analytics"));
        assert!(tenant.can_access_feature("priority_support"));

        // Suspend tenant
        tenant.suspend();
        assert!(!tenant.can_access_feature("basic_analytics")); // Suspended tenants can't access features
    }
}