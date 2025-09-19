// ============================================================================
// OLYMPUS CLOUD - FEATURE FLAGS SERVICE
// ============================================================================
// Module: platform/src/services/feature_flags.rs
// Description: Feature flag management service for A/B testing and gradual rollouts
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::{Row, query, query_as};
use rand::Rng;

use olympus_shared::{
    database::DbPool,
    events::{EventPublisher, DomainEvent},
    error::{Result, OlympusError},
};

use crate::models::{
    FeatureFlag, FeatureFlagType, FeatureFlagStatus, RolloutStrategy,
    FeatureFlagEvaluation, FeatureFlagUsage, FeatureFlagEvaluationRequest,
    CreateFeatureFlagRequest, UpdateFeatureFlagRequest,
};

#[derive(Clone)]
pub struct FeatureFlagsService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl FeatureFlagsService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ============================================================================
    // FEATURE FLAG CRUD OPERATIONS
    // ============================================================================

    pub async fn create_feature_flag(
        &self,
        tenant_id: Option<Uuid>,
        request: CreateFeatureFlagRequest,
        created_by: Uuid,
    ) -> Result<FeatureFlag> {
        let flag_id = Uuid::new_v4();
        let now = Utc::now();

        // Validate feature flag key is unique
        self.validate_flag_key_unique(tenant_id, &request.key, None).await?;

        // Validate rollout percentage for percentage-based strategies
        if matches!(request.rollout_strategy, RolloutStrategy::PercentageUsers | RolloutStrategy::GradualRollout) {
            if request.rollout_percentage.is_none() {
                return Err(OlympusError::Validation(
                    "Rollout percentage is required for percentage-based strategies".to_string()
                ));
            }
        }

        // Serialize JSON fields
        let conditions_json = request.conditions.unwrap_or_else(|| serde_json::json!({}));
        let variants_json = request.variants.unwrap_or_else(|| serde_json::json!({}));
        let target_users = request.target_users.unwrap_or_default();
        let target_groups = request.target_groups.unwrap_or_default();
        let tags = request.tags.unwrap_or_default();

        // Insert feature flag
        let flag_row = query_as!(
            FeatureFlagRow,
            r#"
            INSERT INTO feature_flags (
                id, tenant_id, key, name, description, flag_type, status,
                default_value, rollout_strategy, rollout_percentage,
                target_users, target_groups, conditions, variants, tags,
                is_global, created_at, updated_at, created_by, updated_by,
                starts_at, ends_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
            RETURNING
                id, tenant_id, key, name, description,
                flag_type as "flag_type: FeatureFlagType",
                status as "status: FeatureFlagStatus",
                default_value, rollout_strategy as "rollout_strategy: RolloutStrategy",
                rollout_percentage, target_users, target_groups, conditions,
                variants, tags, is_global, created_at, updated_at,
                created_by, updated_by, starts_at, ends_at
            "#,
            flag_id,
            tenant_id,
            request.key,
            request.name,
            request.description,
            request.flag_type as FeatureFlagType,
            FeatureFlagStatus::Active as FeatureFlagStatus,
            request.default_value,
            request.rollout_strategy as RolloutStrategy,
            request.rollout_percentage,
            &target_users,
            &target_groups,
            conditions_json,
            variants_json,
            &tags,
            request.is_global,
            now,
            now,
            created_by,
            created_by,
            request.starts_at,
            request.ends_at
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to create feature flag: {}", e)))?;

        let flag = self.flag_row_to_model(flag_row)?;

        // Publish domain event
        let event = DomainEvent::builder()
            .data(serde_json::json!({
                "flag_id": flag_id,
                "tenant_id": tenant_id,
                "key": request.key,
                "name": request.name,
                "flag_type": request.flag_type,
                "rollout_strategy": request.rollout_strategy,
                "is_global": request.is_global,
                "created_by": created_by
            }))
            .source_service("platform")
            .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            tracing::warn!("Failed to publish FeatureFlagCreated event: {}", e);
        }

        Ok(flag)
    }

    pub async fn get_feature_flag(
        &self,
        tenant_id: Option<Uuid>,
        flag_id: Uuid,
    ) -> Result<Option<FeatureFlag>> {
        let flag_row = query_as!(
            FeatureFlagRow,
            r#"
            SELECT
                id, tenant_id, key, name, description,
                flag_type as "flag_type: FeatureFlagType",
                status as "status: FeatureFlagStatus",
                default_value, rollout_strategy as "rollout_strategy: RolloutStrategy",
                rollout_percentage, target_users, target_groups, conditions,
                variants, tags, is_global, created_at, updated_at,
                created_by, updated_by, starts_at, ends_at
            FROM feature_flags
            WHERE id = $1 AND (tenant_id = $2 OR tenant_id IS NULL) AND deleted_at IS NULL
            "#,
            flag_id,
            tenant_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get feature flag: {}", e)))?;

        match flag_row {
            Some(row) => Ok(Some(self.flag_row_to_model(row)?)),
            None => Ok(None),
        }
    }

    pub async fn get_feature_flag_by_key(
        &self,
        tenant_id: Option<Uuid>,
        key: &str,
    ) -> Result<Option<FeatureFlag>> {
        let flag_row = query_as!(
            FeatureFlagRow,
            r#"
            SELECT
                id, tenant_id, key, name, description,
                flag_type as "flag_type: FeatureFlagType",
                status as "status: FeatureFlagStatus",
                default_value, rollout_strategy as "rollout_strategy: RolloutStrategy",
                rollout_percentage, target_users, target_groups, conditions,
                variants, tags, is_global, created_at, updated_at,
                created_by, updated_by, starts_at, ends_at
            FROM feature_flags
            WHERE key = $1 AND (tenant_id = $2 OR tenant_id IS NULL) AND deleted_at IS NULL
            ORDER BY tenant_id NULLS LAST
            LIMIT 1
            "#,
            key,
            tenant_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get feature flag by key: {}", e)))?;

        match flag_row {
            Some(row) => Ok(Some(self.flag_row_to_model(row)?)),
            None => Ok(None),
        }
    }

    pub async fn update_feature_flag(
        &self,
        tenant_id: Option<Uuid>,
        flag_id: Uuid,
        request: UpdateFeatureFlagRequest,
        updated_by: Uuid,
    ) -> Result<Option<FeatureFlag>> {
        let now = Utc::now();

        // Simplified update - in production, would build dynamic query
        if let Some(status) = request.status {
            let flag_row = query_as!(
                FeatureFlagRow,
                r#"
                UPDATE feature_flags
                SET status = $3, updated_at = $4, updated_by = $5
                WHERE id = $1 AND (tenant_id = $2 OR tenant_id IS NULL) AND deleted_at IS NULL
                RETURNING
                    id, tenant_id, key, name, description,
                    flag_type as "flag_type: FeatureFlagType",
                    status as "status: FeatureFlagStatus",
                    default_value, rollout_strategy as "rollout_strategy: RolloutStrategy",
                    rollout_percentage, target_users, target_groups, conditions,
                    variants, tags, is_global, created_at, updated_at,
                    created_by, updated_by, starts_at, ends_at
                "#,
                flag_id,
                tenant_id,
                status as FeatureFlagStatus,
                now,
                updated_by
            )
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(|e| OlympusError::Database(format!("Failed to update feature flag: {}", e)))?;

            match flag_row {
                Some(row) => {
                    let flag = self.flag_row_to_model(row)?;

                    // Publish domain event
                    let event = DomainEvent::builder()
                        .data(serde_json::json!({
                            "flag_id": flag_id,
                            "tenant_id": tenant_id,
                            "updated_fields": ["status"],
                            "new_status": status,
                            "updated_by": updated_by
                        }))
                        .source_service("platform")
                        .build();

                    if let Err(e) = self.event_publisher.publish(&event).await {
                        tracing::warn!("Failed to publish FeatureFlagUpdated event: {}", e);
                    }

                    Ok(Some(flag))
                }
                None => Ok(None),
            }
        } else {
            self.get_feature_flag(tenant_id, flag_id).await
        }
    }

    pub async fn delete_feature_flag(
        &self,
        tenant_id: Option<Uuid>,
        flag_id: Uuid,
        deleted_by: Uuid,
    ) -> Result<bool> {
        let now = Utc::now();

        let rows_affected = query!(
            r#"
            UPDATE feature_flags
            SET deleted_at = $3, updated_by = $4
            WHERE id = $1 AND (tenant_id = $2 OR tenant_id IS NULL) AND deleted_at IS NULL
            "#,
            flag_id,
            tenant_id,
            now,
            deleted_by
        )
        .execute(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to delete feature flag: {}", e)))?
        .rows_affected();

        if rows_affected > 0 {
            // Publish domain event
            let event = DomainEvent::builder()
                .data(serde_json::json!({
                    "flag_id": flag_id,
                    "tenant_id": tenant_id,
                    "deleted_by": deleted_by
                }))
                .source_service("platform")
                .build();

            if let Err(e) = self.event_publisher.publish(&event).await {
                tracing::warn!("Failed to publish FeatureFlagDeleted event: {}", e);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ============================================================================
    // FEATURE FLAG EVALUATION
    // ============================================================================

    pub async fn evaluate_feature_flag(
        &self,
        tenant_id: Option<Uuid>,
        request: FeatureFlagEvaluationRequest,
    ) -> Result<FeatureFlagEvaluation> {
        let now = Utc::now();

        // Get the feature flag
        let flag = self.get_feature_flag_by_key(tenant_id, &request.flag_key).await?;

        let (is_enabled, value, variant, reason) = match flag {
            Some(flag) => {
                // Check if flag is active and within time bounds
                if flag.status != FeatureFlagStatus::Active {
                    (false, flag.default_value, None, "Flag is not active".to_string())
                } else if let Some(starts_at) = flag.starts_at {
                    if now < starts_at {
                        (false, flag.default_value, None, "Flag has not started yet".to_string())
                    } else {
                        self.evaluate_flag_logic(&flag, &request).await?
                    }
                } else if let Some(ends_at) = flag.ends_at {
                    if now > ends_at {
                        (false, flag.default_value, None, "Flag has expired".to_string())
                    } else {
                        self.evaluate_flag_logic(&flag, &request).await?
                    }
                } else {
                    self.evaluate_flag_logic(&flag, &request).await?
                }
            }
            None => (false, serde_json::Value::Bool(false), None, "Flag not found".to_string()),
        };

        let evaluation = FeatureFlagEvaluation {
            flag_key: request.flag_key.clone(),
            user_id: request.user_id,
            tenant_id,
            is_enabled,
            value,
            variant,
            reason,
            evaluated_at: now,
        };

        // Record the evaluation for analytics
        self.record_flag_evaluation(&evaluation).await?;

        Ok(evaluation)
    }

    async fn evaluate_flag_logic(
        &self,
        flag: &FeatureFlag,
        request: &FeatureFlagEvaluationRequest,
    ) -> Result<(bool, serde_json::Value, Option<String>, String)> {
        match flag.rollout_strategy {
            RolloutStrategy::AllUsers => {
                Ok((true, flag.default_value.clone(), None, "All users enabled".to_string()))
            }

            RolloutStrategy::PercentageUsers => {
                let percentage = flag.rollout_percentage.unwrap_or(0.0);
                let user_hash = self.hash_user_for_percentage(request.user_id, &flag.key);
                let is_enabled = user_hash < percentage;
                Ok((
                    is_enabled,
                    flag.default_value.clone(),
                    None,
                    format!("Percentage rollout: {}%", percentage),
                ))
            }

            RolloutStrategy::SpecificUsers => {
                let is_enabled = flag.target_users.contains(&request.user_id);
                Ok((
                    is_enabled,
                    flag.default_value.clone(),
                    None,
                    if is_enabled { "User in target list" } else { "User not in target list" }.to_string(),
                ))
            }

            RolloutStrategy::UserGroups => {
                // Simplified - would check user's groups against target_groups
                Ok((false, flag.default_value.clone(), None, "User groups not implemented".to_string()))
            }

            RolloutStrategy::GradualRollout => {
                let percentage = flag.rollout_percentage.unwrap_or(0.0);
                let user_hash = self.hash_user_for_percentage(request.user_id, &flag.key);
                let is_enabled = user_hash < percentage;
                Ok((
                    is_enabled,
                    flag.default_value.clone(),
                    None,
                    format!("Gradual rollout: {}%", percentage),
                ))
            }

            RolloutStrategy::ABTest => {
                // A/B test logic - assign users to variants
                let variant = self.assign_ab_test_variant(request.user_id, &flag.key, &flag.variants);
                let is_enabled = variant.is_some();
                Ok((
                    is_enabled,
                    flag.default_value.clone(),
                    variant,
                    "A/B test variant assignment".to_string(),
                ))
            }
        }
    }

    fn hash_user_for_percentage(&self, user_id: Uuid, flag_key: &str) -> f64 {
        // Simple hash-based percentage calculation for consistent user assignment
        let combined = format!("{}{}", user_id, flag_key);
        let hash = combined.chars().map(|c| c as u32).sum::<u32>();
        (hash % 100) as f64
    }

    fn assign_ab_test_variant(
        &self,
        user_id: Uuid,
        flag_key: &str,
        variants: &serde_json::Value,
    ) -> Option<String> {
        // Simplified A/B test variant assignment
        if let Ok(variant_map) = serde_json::from_value::<HashMap<String, f64>>(variants.clone()) {
            let user_hash = self.hash_user_for_percentage(user_id, flag_key);
            let mut cumulative = 0.0;

            for (variant_name, weight) in variant_map {
                cumulative += weight;
                if user_hash < cumulative {
                    return Some(variant_name);
                }
            }
        }
        None
    }

    async fn record_flag_evaluation(&self, evaluation: &FeatureFlagEvaluation) -> Result<()> {
        // Record evaluation for analytics
        query!(
            r#"
            INSERT INTO feature_flag_evaluations (
                flag_key, user_id, tenant_id, is_enabled, value, variant,
                reason, evaluated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            evaluation.flag_key,
            evaluation.user_id,
            evaluation.tenant_id,
            evaluation.is_enabled,
            evaluation.value,
            evaluation.variant,
            evaluation.reason,
            evaluation.evaluated_at
        )
        .execute(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to record flag evaluation: {}", e)))?;

        Ok(())
    }

    // ============================================================================
    // ANALYTICS AND REPORTING
    // ============================================================================

    pub async fn get_flag_usage_analytics(
        &self,
        tenant_id: Option<Uuid>,
        flag_key: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Option<FeatureFlagUsage>> {
        let usage = query!(
            r#"
            SELECT
                flag_key,
                tenant_id,
                COUNT(*) as total_evaluations,
                COUNT(*) FILTER (WHERE is_enabled = true) as enabled_evaluations,
                COUNT(DISTINCT user_id) as unique_users,
                MAX(evaluated_at) as last_evaluated
            FROM feature_flag_evaluations
            WHERE flag_key = $1
                AND (tenant_id = $2 OR tenant_id IS NULL)
                AND evaluated_at BETWEEN $3 AND $4
            GROUP BY flag_key, tenant_id
            "#,
            flag_key,
            tenant_id,
            period_start,
            period_end
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to get flag usage analytics: {}", e)))?;

        if let Some(usage) = usage {
            Ok(Some(FeatureFlagUsage {
                flag_key: usage.flag_key,
                tenant_id: usage.tenant_id,
                total_evaluations: usage.total_evaluations,
                enabled_evaluations: usage.enabled_evaluations,
                unique_users: usage.unique_users,
                last_evaluated: usage.last_evaluated,
                period_start,
                period_end,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn list_feature_flags(
        &self,
        tenant_id: Option<Uuid>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<FeatureFlag>> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        let flag_rows = query_as!(
            FeatureFlagRow,
            r#"
            SELECT
                id, tenant_id, key, name, description,
                flag_type as "flag_type: FeatureFlagType",
                status as "status: FeatureFlagStatus",
                default_value, rollout_strategy as "rollout_strategy: RolloutStrategy",
                rollout_percentage, target_users, target_groups, conditions,
                variants, tags, is_global, created_at, updated_at,
                created_by, updated_by, starts_at, ends_at
            FROM feature_flags
            WHERE (tenant_id = $1 OR tenant_id IS NULL) AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(|e| OlympusError::Database(format!("Failed to list feature flags: {}", e)))?;

        flag_rows
            .into_iter()
            .map(|row| self.flag_row_to_model(row))
            .collect()
    }

    // ============================================================================
    // HELPER METHODS
    // ============================================================================

    async fn validate_flag_key_unique(
        &self,
        tenant_id: Option<Uuid>,
        key: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<()> {
        let mut query_str = "SELECT id FROM feature_flags WHERE key = $1 AND (tenant_id = $2 OR tenant_id IS NULL) AND deleted_at IS NULL".to_string();

        if exclude_id.is_some() {
            query_str.push_str(" AND id != $3");
        }

        let exists = if let Some(exclude_id) = exclude_id {
            query!(&query_str, key, tenant_id, exclude_id)
                .fetch_optional(self.db.as_ref())
                .await
        } else {
            query!(&query_str, key, tenant_id)
                .fetch_optional(self.db.as_ref())
                .await
        }
        .map_err(|e| OlympusError::Database(format!("Failed to check flag key uniqueness: {}", e)))?;

        if exists.is_some() {
            return Err(OlympusError::Validation("Feature flag key already exists".to_string()));
        }

        Ok(())
    }

    fn flag_row_to_model(&self, row: FeatureFlagRow) -> Result<FeatureFlag> {
        Ok(FeatureFlag {
            id: row.id,
            tenant_id: row.tenant_id,
            key: row.key,
            name: row.name,
            description: row.description,
            flag_type: row.flag_type,
            status: row.status,
            default_value: row.default_value,
            rollout_strategy: row.rollout_strategy,
            rollout_percentage: row.rollout_percentage,
            target_users: row.target_users,
            target_groups: row.target_groups,
            conditions: row.conditions,
            variants: row.variants,
            tags: row.tags,
            is_global: row.is_global,
            created_at: row.created_at,
            updated_at: row.updated_at,
            created_by: row.created_by,
            updated_by: row.updated_by,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
        })
    }
}

// ============================================================================
// DATABASE ROW TYPES
// ============================================================================

#[derive(Debug)]
struct FeatureFlagRow {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
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
    pub variants: serde_json::Value,
    pub tags: Vec<String>,
    pub is_global: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_by: Uuid,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
}