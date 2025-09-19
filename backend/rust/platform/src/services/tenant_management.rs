// ============================================================================
// OLYMPUS CLOUD - ADVANCED TENANT MANAGEMENT SERVICE
// ============================================================================
// Module: platform/src/services/tenant_management.rs
// Description: Advanced tenant management, isolation, and resource management
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;
use tracing::{info, warn, error};

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use olympus_shared::error::Error as OlympusError;

use crate::models::{
    Tenant, TenantStatus, SubscriptionTier, TenantLimits, TenantUsageMetrics,
    TenantResource, TenantQuota, TenantIsolation, TenantHealthCheck, TenantAnalytics,
    CreateTenantRequest, UpdateTenantRequest,
};

#[derive(Clone)]
pub struct TenantManagementService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl TenantManagementService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ========================================================================
    // TENANT LIFECYCLE MANAGEMENT
    // ========================================================================

    #[allow(dead_code)]
    pub async fn create_tenant(&self, request: CreateTenantRequest, created_by: Uuid) -> Result<Tenant> {
        let mut tx = self.db.begin().await
            .map_err(|e| OlympusOlympusError::Database(e.to_string()))?;

        // Check if slug is unique
        let existing = sqlx::query!(
            "SELECT id FROM tenants WHERE slug = $1",
            request.slug
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to check tenant slug: {}", e)))?;

        if existing.is_some() {
            return Err(OlympusError::Validation("Tenant slug already exists".to_string()));
        }

        let tenant_id = Uuid::new_v4();

        // Create default limits based on subscription tier
        let default_limits = self.get_default_limits_for_tier(&request.subscription_tier);

        // Create default usage metrics
        let default_usage = TenantUsageMetrics {
            current_users: 0,
            current_locations: 0,
            current_products: 0,
            orders_this_month: 0,
            storage_used_gb: 0.0,
            api_calls_this_hour: 0,
            last_updated: Utc::now(),
        };

        // Insert tenant
        let tenant_row = sqlx::query!(
            r#"
            INSERT INTO tenants (
                id, slug, name, display_name, industry, status, subscription_tier, billing_cycle,
                is_active, settings, domain, logo_url, primary_color, secondary_color,
                timezone, locale, currency, billing_email, technical_contact_email,
                data_region, compliance_requirements, trial_ends_at, subscription_starts_at,
                feature_flags, custom_features, onboarding_completed, onboarding_step,
                created_by, updated_by
            ) VALUES (
                $1, $2, $3, $4, $5, $6::tenant_status, $7::subscription_tier, $8::billing_cycle,
                $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29
            )
            RETURNING created_at, updated_at
            "#,
            tenant_id,
            request.slug,
            request.name,
            request.display_name,
            request.industry,
            TenantStatus::PendingSetup as TenantStatus,
            request.subscription_tier as SubscriptionTier,
            request.billing_cycle.unwrap_or(crate::models::BillingCycle::Monthly) as crate::models::BillingCycle,
            true,
            request.settings.unwrap_or_else(|| serde_json::json!({})),
            request.domain,
            request.logo_url,
            request.primary_color,
            request.secondary_color,
            request.timezone,
            request.locale,
            request.currency,
            request.billing_email,
            request.technical_contact_email,
            request.data_region,
            request.compliance_requirements.unwrap_or_default().as_slice(),
            request.subscription_tier.trial_end_date(),
            Some(Utc::now()),
            serde_json::json!({}),
            request.custom_features.unwrap_or_default().as_slice(),
            false,
            Some("welcome".to_string()),
            created_by,
            created_by
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create tenant: {}", e)))?;

        // Create tenant isolation configuration
        let isolation_config = TenantIsolation {
            id: Uuid::new_v4(),
            tenant_id,
            database_schema: format!("tenant_{}", tenant_id.simple()),
            file_storage_prefix: format!("tenant-{}/", tenant_id),
            cache_namespace: format!("tenant:{}", tenant_id),
            encryption_key_id: format!("tenant-key-{}", tenant_id),
            network_isolation_config: serde_json::json!({
                "vpc_id": null,
                "subnet_ids": [],
                "security_groups": []
            }),
            backup_config: serde_json::json!({
                "enabled": true,
                "schedule": "0 2 * * *",
                "retention_days": 30
            }),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        sqlx::query!(
            r#"
            INSERT INTO tenant_isolation (
                id, tenant_id, database_schema, file_storage_prefix, cache_namespace,
                encryption_key_id, network_isolation_config, backup_config,
                created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            isolation_config.id,
            isolation_config.tenant_id,
            isolation_config.database_schema,
            isolation_config.file_storage_prefix,
            isolation_config.cache_namespace,
            isolation_config.encryption_key_id,
            isolation_config.network_isolation_config,
            isolation_config.backup_config,
            isolation_config.created_at,
            isolation_config.updated_at
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create tenant isolation: {}", e)))?;

        // Create default quotas
        self.create_default_quotas(&mut tx, tenant_id).await?;

        tx.commit().await
            .map_err(|e| OlympusError::Database(format!("Failed to commit transaction: {}", e)))?;

        // Publish tenant created event
        let event_data = serde_json::json!({
            "tenant_id": tenant_id,
            "slug": request.slug,
            "name": request.name,
            "subscription_tier": request.subscription_tier,
            "created_by": created_by
        });

        if let Err(e) = self.event_publisher.publish("tenant.created", event_data, Some(tenant_id)).await {
            warn!("Failed to publish tenant created event: {}", e);
        }

        // Return created tenant
        self.get_tenant_by_id(tenant_id).await
    }

    pub async fn update_tenant(&self, tenant_id: Uuid, request: UpdateTenantRequest, updated_by: Uuid) -> Result<Tenant> {
        let mut tx = self.db.begin().await
            .map_err(|e| OlympusOlympusError::Database(e.to_string()))?;

        // Build dynamic update query
        let mut query_parts = Vec::new();
        let mut param_count = 1;

        if request.name.is_some() {
            query_parts.push(format!("name = ${}", param_count));
            param_count += 1;
        }
        if request.display_name.is_some() {
            query_parts.push(format!("display_name = ${}", param_count));
            param_count += 1;
        }
        // Add more fields as needed...

        query_parts.push(format!("updated_by = ${}", param_count));
        param_count += 1;
        query_parts.push(format!("updated_at = ${}", param_count));

        let update_query = format!(
            "UPDATE tenants SET {} WHERE id = ${}",
            query_parts.join(", "),
            param_count + 1
        );

        // Execute update (simplified for example)
        sqlx::query(&update_query)
            .bind(&request.name)
            .bind(&request.display_name)
            .bind(updated_by)
            .bind(Utc::now())
            .bind(tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| OlympusError::Database(format!("Failed to update tenant: {}", e)))?;

        tx.commit().await
            .map_err(|e| OlympusError::Database(format!("Failed to commit transaction: {}", e)))?;

        // Publish tenant updated event
        let event_data = serde_json::json!({
            "tenant_id": tenant_id,
            "updated_by": updated_by,
            "changes": request
        });

        if let Err(e) = self.event_publisher.publish("tenant.updated", event_data, Some(tenant_id)).await {
            warn!("Failed to publish tenant updated event: {}", e);
        }

        self.get_tenant_by_id(tenant_id).await
    }

    pub async fn suspend_tenant(&self, tenant_id: Uuid, reason: String, suspended_by: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE tenants SET status = $1::tenant_status, is_active = false, updated_by = $2, updated_at = $3 WHERE id = $4",
            TenantStatus::Suspended as TenantStatus,
            suspended_by,
            Utc::now(),
            tenant_id
        )
        .execute(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to suspend tenant: {}", e)))?;

        // Publish suspension event
        let event_data = serde_json::json!({
            "tenant_id": tenant_id,
            "reason": reason,
            "suspended_by": suspended_by
        });

        if let Err(e) = self.event_publisher.publish("tenant.suspended", event_data, Some(tenant_id)).await {
            warn!("Failed to publish tenant suspended event: {}", e);
        }

        info!("Tenant {} suspended by {}: {}", tenant_id, suspended_by, reason);
        Ok(())
    }

    // ========================================================================
    // RESOURCE AND QUOTA MANAGEMENT
    // ========================================================================

    pub async fn check_quota(&self, tenant_id: Uuid, quota_type: &str) -> Result<bool> {
        let quota = sqlx::query!(
            "SELECT limit_value, current_usage, is_hard_limit FROM tenant_quotas WHERE tenant_id = $1 AND quota_type = $2",
            tenant_id,
            quota_type
        )
        .fetch_optional(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to check quota: {}", e)))?;

        match quota {
            Some(q) => Ok(q.current_usage < q.limit_value || !q.is_hard_limit),
            None => Ok(true), // No quota defined means unlimited
        }
    }

    pub async fn increment_usage(&self, tenant_id: Uuid, quota_type: &str, amount: i64) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE tenant_quotas
            SET current_usage = current_usage + $1, updated_at = $2
            WHERE tenant_id = $3 AND quota_type = $4
            "#,
            amount,
            Utc::now(),
            tenant_id,
            quota_type
        )
        .execute(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to increment usage: {}", e)))?;

        Ok(())
    }

    pub async fn reset_quota(&self, tenant_id: Uuid, quota_type: &str) -> Result<()> {
        let now = Utc::now();

        sqlx::query!(
            r#"
            UPDATE tenant_quotas
            SET current_usage = 0, last_reset = $1, updated_at = $1
            WHERE tenant_id = $2 AND quota_type = $3
            "#,
            now,
            tenant_id,
            quota_type
        )
        .execute(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to reset quota: {}", e)))?;

        Ok(())
    }

    // ========================================================================
    // HEALTH MONITORING AND ANALYTICS
    // ========================================================================

    pub async fn run_health_checks(&self, tenant_id: Uuid) -> Result<Vec<TenantHealthCheck>> {
        let mut health_checks = Vec::new();

        // Database connectivity check
        let db_start = std::time::Instant::now();
        let db_result = sqlx::query!("SELECT 1 as test")
            .fetch_one(&*self.db)
            .await;

        let db_check = TenantHealthCheck {
            tenant_id,
            check_name: "database_connectivity".to_string(),
            status: if db_result.is_ok() { "healthy" } else { "critical" }.to_string(),
            last_check: Utc::now(),
            response_time_ms: Some(db_start.elapsed().as_millis() as i32),
            error_count: if db_result.is_err() { 1 } else { 0 },
            details: if let Err(e) = db_result {
                serde_json::json!({"error": e.to_string()})
            } else {
                serde_json::json!({"status": "ok"})
            },
        };
        health_checks.push(db_check);

        // API response time check
        let api_start = std::time::Instant::now();
        let tenant_result = self.get_tenant_by_id(tenant_id).await;

        let api_check = TenantHealthCheck {
            tenant_id,
            check_name: "api_response_time".to_string(),
            status: if tenant_result.is_ok() { "healthy" } else { "warning" }.to_string(),
            last_check: Utc::now(),
            response_time_ms: Some(api_start.elapsed().as_millis() as i32),
            error_count: if tenant_result.is_err() { 1 } else { 0 },
            details: serde_json::json!({
                "response_time_ms": api_start.elapsed().as_millis(),
                "threshold_ms": 100
            }),
        };
        health_checks.push(api_check);

        // Quota utilization check
        let quota_usage = self.get_quota_utilization(tenant_id).await?;
        let high_usage_quotas: Vec<_> = quota_usage.iter()
            .filter(|(_, usage)| *usage > 0.8)
            .collect();

        let quota_check = TenantHealthCheck {
            tenant_id,
            check_name: "quota_utilization".to_string(),
            status: if high_usage_quotas.is_empty() { "healthy" } else { "warning" }.to_string(),
            last_check: Utc::now(),
            response_time_ms: None,
            error_count: high_usage_quotas.len() as i32,
            details: serde_json::json!({
                "high_usage_quotas": high_usage_quotas,
                "all_quotas": quota_usage
            }),
        };
        health_checks.push(quota_check);

        Ok(health_checks)
    }

    pub async fn calculate_health_score(&self, tenant_id: Uuid) -> Result<f64> {
        let health_checks = self.run_health_checks(tenant_id).await?;

        let mut score = 100.0;

        for check in health_checks {
            match check.status.as_str() {
                "critical" => score -= 30.0,
                "warning" => score -= 10.0,
                _ => {} // healthy checks don't reduce score
            }

            // Penalty for slow response times
            if let Some(response_time) = check.response_time_ms {
                if response_time > 1000 {
                    score -= 20.0;
                } else if response_time > 500 {
                    score -= 10.0;
                } else if response_time > 100 {
                    score -= 5.0;
                }
            }
        }

        Ok(score.max(0.0))
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    async fn get_tenant_by_id(&self, tenant_id: Uuid) -> Result<Tenant> {
        // This would be a full implementation that fetches the tenant
        // For now, returning a minimal implementation
        Err(OlympusError::NotFound("Tenant not found".to_string()).into())
    }

    fn get_default_limits_for_tier(&self, tier: &SubscriptionTier) -> TenantLimits {
        match tier {
            SubscriptionTier::Trial => TenantLimits {
                max_users: Some(5),
                max_locations: Some(1),
                max_products: Some(100),
                max_orders_per_month: Some(50),
                max_storage_gb: Some(1),
                max_api_calls_per_hour: Some(1000),
                features_enabled: vec!["basic_pos".to_string()],
                integrations_allowed: vec![],
            },
            SubscriptionTier::Basic => TenantLimits {
                max_users: Some(10),
                max_locations: Some(2),
                max_products: Some(500),
                max_orders_per_month: Some(500),
                max_storage_gb: Some(5),
                max_api_calls_per_hour: Some(5000),
                features_enabled: vec!["basic_pos".to_string(), "inventory".to_string()],
                integrations_allowed: vec!["stripe".to_string()],
            },
            SubscriptionTier::Professional => TenantLimits {
                max_users: Some(50),
                max_locations: Some(10),
                max_products: Some(5000),
                max_orders_per_month: Some(5000),
                max_storage_gb: Some(50),
                max_api_calls_per_hour: Some(25000),
                features_enabled: vec![
                    "advanced_pos".to_string(),
                    "inventory".to_string(),
                    "analytics".to_string(),
                    "multi_location".to_string()
                ],
                integrations_allowed: vec![
                    "stripe".to_string(),
                    "paypal".to_string(),
                    "quickbooks".to_string()
                ],
            },
            SubscriptionTier::Enterprise => TenantLimits {
                max_users: None, // Unlimited
                max_locations: None,
                max_products: None,
                max_orders_per_month: None,
                max_storage_gb: None,
                max_api_calls_per_hour: Some(100000),
                features_enabled: vec![
                    "enterprise_pos".to_string(),
                    "inventory".to_string(),
                    "analytics".to_string(),
                    "multi_location".to_string(),
                    "white_label".to_string(),
                    "api_access".to_string()
                ],
                integrations_allowed: vec![
                    "stripe".to_string(),
                    "paypal".to_string(),
                    "quickbooks".to_string(),
                    "sap".to_string(),
                    "oracle".to_string()
                ],
            },
            SubscriptionTier::Custom => TenantLimits {
                max_users: None,
                max_locations: None,
                max_products: None,
                max_orders_per_month: None,
                max_storage_gb: None,
                max_api_calls_per_hour: None,
                features_enabled: vec![], // Configured per tenant
                integrations_allowed: vec![], // Configured per tenant
            },
        }
    }

    async fn create_default_quotas(&self, tx: &mut sqlx::PgConnection, tenant_id: Uuid) -> Result<()> {
        let quotas = vec![
            ("api_calls_per_hour", 1000, true),
            ("storage_gb", 5, true),
            ("users", 10, true),
            ("locations", 2, true),
            ("products", 500, false),
            ("orders_per_month", 500, false),
        ];

        for (quota_type, limit, is_hard) in quotas {
            sqlx::query!(
                r#"
                INSERT INTO tenant_quotas (
                    id, tenant_id, quota_type, limit_value, current_usage,
                    reset_interval, last_reset, next_reset, is_hard_limit,
                    created_at, updated_at
                ) VALUES (
                    $1, $2, $3, $4, 0, 'monthly', $5, $6, $7, $5, $5
                )
                "#,
                Uuid::new_v4(),
                tenant_id,
                quota_type,
                limit as i64,
                Utc::now(),
                Utc::now() + chrono::Duration::days(30),
                is_hard
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| OlympusError::Database(e.to_string()))?;
        }

        Ok(())
    }

    async fn get_quota_utilization(&self, tenant_id: Uuid) -> Result<Vec<(String, f64)>> {
        let quotas = sqlx::query!(
            "SELECT quota_type, limit_value, current_usage FROM tenant_quotas WHERE tenant_id = $1",
            tenant_id
        )
        .fetch_all(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(e.to_string()))?;

        Ok(quotas.into_iter()
            .map(|q| {
                let utilization = if q.limit_value > 0 {
                    q.current_usage as f64 / q.limit_value as f64
                } else {
                    0.0
                };
                (q.quota_type, utilization)
            })
            .collect())
    }
}

// Trait extensions for subscription tiers
trait SubscriptionTierExt {
    fn trial_end_date(&self) -> Option<DateTime<Utc>>;
    fn max_trial_days(&self) -> i64;
}

impl SubscriptionTierExt for SubscriptionTier {
    fn trial_end_date(&self) -> Option<DateTime<Utc>> {
        match self {
            SubscriptionTier::Trial => Some(Utc::now() + chrono::Duration::days(14)),
            _ => None,
        }
    }

    fn max_trial_days(&self) -> i64 {
        match self {
            SubscriptionTier::Trial => 14,
            SubscriptionTier::Basic => 7,
            SubscriptionTier::Professional => 30,
            SubscriptionTier::Enterprise => 30,
            SubscriptionTier::Custom => 0,
        }
    }
}