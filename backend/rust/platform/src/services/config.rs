// ============================================================================
// OLYMPUS CLOUD - CONFIGURATION SERVICE
// ============================================================================
// Module: platform/src/services/config.rs
// Description: System configuration management service for tenant and global settings
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::{Row, query, query_as};

use olympus_shared::{
    database::DbPool,
    events::{EventPublisher, DomainEvent},
    error::{Result, Error},
};

use crate::models::{
    Configuration, ConfigScope, ConfigType, ConfigurationAudit,
    ConfigurationSearchRequest, ConfigurationSearchResponse,
    CreateConfigurationRequest, UpdateConfigurationRequest,
};

#[derive(Clone)]
pub struct ConfigurationService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl ConfigurationService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ============================================================================
    // CONFIGURATION CRUD OPERATIONS
    // ============================================================================

    pub async fn create_configuration(
        &self,
        request: CreateConfigurationRequest,
        created_by: Uuid,
    ) -> Result<Configuration> {
        let config_id = Uuid::new_v4();
        let now = Utc::now();

        // Validate configuration key is unique within scope
        self.validate_config_key_unique(
            request.scope,
            request.scope_id,
            &request.key,
            None,
        ).await?;

        // Validate configuration value according to type and rules
        self.validate_config_value(&request.config_type, &request.value, &request.validation_rules)?;

        // Serialize JSON fields
        let default_value = request.default_value.unwrap_or_else(|| serde_json::json!(null));
        let validation_rules = request.validation_rules.unwrap_or_else(|| serde_json::json!({}));
        let tags = request.tags.unwrap_or_default();

        // Insert configuration
        let config_row = query_as!(
            ConfigurationRow,
            r#"
            INSERT INTO platform.configurations (
                id, scope, scope_id, key, display_name, description, config_type,
                value, default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            RETURNING
                id, scope as "scope: ConfigScope", scope_id, key, display_name,
                description, config_type as "config_type: ConfigType", value,
                default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            "#,
            config_id,
            request.scope as ConfigScope,
            request.scope_id,
            request.key,
            request.display_name,
            request.description,
            request.config_type as ConfigType,
            request.value,
            default_value,
            request.is_sensitive,
            request.is_readonly,
            validation_rules,
            request.category,
            &tags,
            now,
            now,
            created_by,
            created_by
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(|e| Error::Database(format!("Failed to create configuration: {}", e)))?;

        let config = self.config_row_to_model(config_row)?;

        // Create audit record
        self.create_audit_record(
            config_id,
            "created".to_string(),
            None,
            Some(request.value.clone()),
            created_by,
            None,
            None,
            None,
        ).await?;

        // Publish domain event
        let event = DomainEvent::builder(
            "ConfigurationCreated".to_string(),
            request.scope_id.unwrap_or_else(|| Uuid::new_v4()), // Use scope_id as tenant context
            "platform".to_string(),
            created_by,
        )
        .data(serde_json::json!({
            "config_id": config_id,
            "scope": request.scope,
            "scope_id": request.scope_id,
            "key": request.key,
            "category": request.category,
            "is_sensitive": request.is_sensitive,
            "created_by": created_by
        }))?
        .build();

        if let Err(e) = self.event_publisher.publish(&event).await {
            tracing::warn!("Failed to publish ConfigurationCreated event: {}", e);
        }

        Ok(config)
    }

    pub async fn get_configuration(
        &self,
        config_id: Uuid,
        include_sensitive: bool,
    ) -> Result<Option<Configuration>> {
        let config_row = query_as!(
            ConfigurationRow,
            r#"
            SELECT
                id, scope as "scope: ConfigScope", scope_id, key, display_name,
                description, config_type as "config_type: ConfigType", value,
                default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            FROM platform.configurations
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            config_id
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(Error::Database)?;

        match config_row {
            Some(row) => {
                let mut config = self.config_row_to_model(row)?;

                // Mask sensitive values if not explicitly requested
                if config.is_sensitive && !include_sensitive {
                    config.value = serde_json::json!("***MASKED***");
                }

                Ok(Some(config))
            }
            None => Ok(None),
        }
    }

    pub async fn get_configuration_by_key(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        key: &str,
        include_sensitive: bool,
    ) -> Result<Option<Configuration>> {
        let config_row = query_as!(
            ConfigurationRow,
            r#"
            SELECT
                id, scope as "scope: ConfigScope", scope_id, key, display_name,
                description, config_type as "config_type: ConfigType", value,
                default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            FROM platform.configurations
            WHERE scope = $1 AND scope_id = $2 AND key = $3 AND deleted_at IS NULL
            "#,
            scope as ConfigScope,
            scope_id,
            key
        )
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(Error::Database)?;

        match config_row {
            Some(row) => {
                let mut config = self.config_row_to_model(row)?;

                // Mask sensitive values if not explicitly requested
                if config.is_sensitive && !include_sensitive {
                    config.value = serde_json::json!("***MASKED***");
                }

                Ok(Some(config))
            }
            None => Ok(None),
        }
    }

    pub async fn update_configuration(
        &self,
        config_id: Uuid,
        request: UpdateConfigurationRequest,
        updated_by: Uuid,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<Option<Configuration>> {
        let now = Utc::now();

        // Get current configuration for audit trail
        let current_config = self.get_configuration(config_id, true).await?;
        let current_config = match current_config {
            Some(config) => config,
            None => return Ok(None),
        };

        // Check if configuration is readonly
        if current_config.is_readonly {
            return Err(Error::Validation("Configuration is readonly".to_string()));
        }

        // Validate new value if provided
        if let Some(ref new_value) = request.value {
            self.validate_config_value(
                &current_config.config_type,
                new_value,
                &Some(current_config.validation_rules.clone()),
            )?;
        }

        // Simplified update - in production, would build dynamic query
        if let Some(new_value) = request.value {
            let config_row = query_as!(
                ConfigurationRow,
                r#"
                UPDATE platform.configurations
                SET value = $2, updated_at = $3, updated_by = $4
                WHERE id = $1 AND deleted_at IS NULL
                RETURNING
                    id, scope as "scope: ConfigScope", scope_id, key, display_name,
                    description, config_type as "config_type: ConfigType", value,
                    default_value, is_sensitive, is_readonly, validation_rules,
                    category, tags, created_at, updated_at, created_by, updated_by
                "#,
                config_id,
                new_value,
                now,
                updated_by
            )
            .fetch_optional(self.db.as_ref())
            .await
            .map_err(Error::Database)?;

            match config_row {
                Some(row) => {
                    let config = self.config_row_to_model(row)?;

                    // Create audit record
                    self.create_audit_record(
                        config_id,
                        "updated".to_string(),
                        Some(current_config.value),
                        Some(new_value.clone()),
                        updated_by,
                        None,
                        ip_address,
                        user_agent,
                    ).await?;

                    // Publish domain event
                    let event = DomainEvent::builder(
                        "ConfigurationUpdated".to_string(),
                        config.scope_id.unwrap_or_else(|| Uuid::new_v4()), // Use a default UUID if no scope_id
                        "platform".to_string(),
                        updated_by,
                    )
                    .data(serde_json::json!({
                        "config_id": config_id,
                        "scope": config.scope,
                        "scope_id": config.scope_id,
                        "key": config.key,
                        "updated_fields": ["value"],
                        "is_sensitive": config.is_sensitive,
                        "updated_by": updated_by
                    }))?
                    .build();

                    if let Err(e) = self.event_publisher.publish(&event).await {
                        tracing::warn!("Failed to publish ConfigurationUpdated event: {}", e);
                    }

                    Ok(Some(config))
                }
                None => Ok(None),
            }
        } else {
            Ok(Some(current_config))
        }
    }

    pub async fn delete_configuration(
        &self,
        config_id: Uuid,
        deleted_by: Uuid,
    ) -> Result<bool> {
        let now = Utc::now();

        // Get current configuration for audit trail
        let current_config = self.get_configuration(config_id, true).await?;
        let current_config = match current_config {
            Some(config) => config,
            None => return Ok(false),
        };

        // Check if configuration is readonly
        if current_config.is_readonly {
            return Err(Error::Validation("Configuration is readonly".to_string()));
        }

        let rows_affected = query!(
            r#"
            UPDATE platform.configurations
            SET deleted_at = $2, updated_by = $3
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            config_id,
            now,
            deleted_by
        )
        .execute(self.db.as_ref())
        .await
        .map_err(Error::Database)?
        .rows_affected();

        if rows_affected > 0 {
            // Create audit record
            self.create_audit_record(
                config_id,
                "deleted".to_string(),
                Some(current_config.value),
                None,
                deleted_by,
                None,
                None,
                None,
            ).await?;

            // Publish domain event
            let event = DomainEvent::builder(
                "ConfigurationDeleted".to_string(),
                current_config.scope_id.unwrap_or_else(|| Uuid::new_v4()), // Use scope_id as tenant context
                "platform".to_string(),
                deleted_by,
            )
            .data(serde_json::json!({
                "config_id": config_id,
                "scope": current_config.scope,
                "scope_id": current_config.scope_id,
                "key": current_config.key,
                "deleted_by": deleted_by
            }))?
            .build();

            if let Err(e) = self.event_publisher.publish(&event).await {
                tracing::warn!("Failed to publish ConfigurationDeleted event: {}", e);
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ============================================================================
    // CONFIGURATION SEARCH AND BULK OPERATIONS
    // ============================================================================

    pub async fn search_configurations(
        &self,
        request: ConfigurationSearchRequest,
    ) -> Result<ConfigurationSearchResponse> {
        let limit = request.limit.unwrap_or(50).min(100);
        let offset = request.offset.unwrap_or(0);

        // Build search query based on filters
        let mut where_conditions = vec!["deleted_at IS NULL".to_string()];

        if let Some(scope) = request.scope {
            where_conditions.push(format!("scope = '{:?}'", scope));
        }

        if let Some(scope_id) = request.scope_id {
            where_conditions.push(format!("scope_id = '{}'", scope_id));
        }

        if let Some(category) = &request.category {
            where_conditions.push(format!("category = '{}'", category));
        }

        if !request.include_sensitive {
            where_conditions.push("is_sensitive = false".to_string());
        }

        // Simplified query - a full implementation would handle all filters
        let config_rows = query_as!(
            ConfigurationRow,
            r#"
            SELECT
                id, scope as "scope: ConfigScope", scope_id, key, display_name,
                description, config_type as "config_type: ConfigType", value,
                default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            FROM platform.configurations
            WHERE deleted_at IS NULL
            ORDER BY category, key
            LIMIT $1 OFFSET $2
            "#,
            limit as i64,
            offset as i64
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(Error::Database)?;

        let total_count = query!(
            "SELECT COUNT(*) as count FROM platform.configurations WHERE deleted_at IS NULL"
        )
        .fetch_one(self.db.as_ref())
        .await
        .map_err(Error::Database)?
        .count
        .unwrap_or(0);

        let mut configurations = Vec::new();
        for row in config_rows {
            let mut config = self.config_row_to_model(row)?;

            // Mask sensitive values if not explicitly requested
            if config.is_sensitive && !request.include_sensitive {
                config.value = serde_json::json!("***MASKED***");
            }

            configurations.push(config);
        }

        Ok(ConfigurationSearchResponse {
            configurations,
            total_count,
            has_more: (offset as i64 + limit as i64) < total_count,
        })
    }

    pub async fn get_configurations_by_scope(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        include_sensitive: bool,
    ) -> Result<HashMap<String, serde_json::Value>> {
        let config_rows = query_as!(
            ConfigurationRow,
            r#"
            SELECT
                id, scope as "scope: ConfigScope", scope_id, key, display_name,
                description, config_type as "config_type: ConfigType", value,
                default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            FROM platform.configurations
            WHERE scope = $1 AND scope_id = $2 AND deleted_at IS NULL
            ORDER BY key
            "#,
            scope as ConfigScope,
            scope_id
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(Error::Database)?;

        let mut result = HashMap::new();
        for row in config_rows {
            let config = self.config_row_to_model(row)?;

            let value = if config.is_sensitive && !include_sensitive {
                serde_json::json!("***MASKED***")
            } else {
                config.value
            };

            result.insert(config.key, value);
        }

        Ok(result)
    }

    // ============================================================================
    // AUDIT AND COMPLIANCE
    // ============================================================================

    pub async fn get_configuration_audit_history(
        &self,
        config_id: Uuid,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<ConfigurationAudit>> {
        let limit = limit.unwrap_or(50).min(100);
        let offset = offset.unwrap_or(0);

        let audit_rows = query_as!(
            ConfigurationAuditRow,
            r#"
            SELECT
                id, configuration_id, action, old_value, new_value,
                changed_by, changed_at, reason, ip_address, user_agent
            FROM configuration_audits
            WHERE configuration_id = $1
            ORDER BY changed_at DESC
            LIMIT $2 OFFSET $3
            "#,
            config_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(self.db.as_ref())
        .await
        .map_err(Error::Database)?;

        let audits = audit_rows
            .into_iter()
            .map(|row| ConfigurationAudit {
                id: row.id,
                configuration_id: row.configuration_id,
                action: row.action,
                old_value: row.old_value,
                new_value: row.new_value,
                changed_by: row.changed_by,
                changed_at: row.changed_at,
                reason: row.reason,
                ip_address: row.ip_address,
                user_agent: row.user_agent,
            })
            .collect();

        Ok(audits)
    }

    async fn create_audit_record(
        &self,
        configuration_id: Uuid,
        action: String,
        old_value: Option<serde_json::Value>,
        new_value: Option<serde_json::Value>,
        changed_by: Uuid,
        reason: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<()> {
        let audit_id = Uuid::new_v4();
        let now = Utc::now();

        query!(
            r#"
            INSERT INTO configuration_audits (
                id, configuration_id, action, old_value, new_value,
                changed_by, changed_at, reason, ip_address, user_agent
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            audit_id,
            configuration_id,
            action,
            old_value,
            new_value,
            changed_by,
            now,
            reason,
            ip_address,
            user_agent
        )
        .execute(self.db.as_ref())
        .await
        .map_err(Error::Database)?;

        Ok(())
    }

    // ============================================================================
    // HELPER METHODS
    // ============================================================================

    async fn validate_config_key_unique(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        key: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<()> {
        let mut query_str = "SELECT id FROM platform.configurations WHERE scope = $1 AND scope_id = $2 AND key = $3 AND deleted_at IS NULL".to_string();

        if exclude_id.is_some() {
            query_str.push_str(" AND id != $4");
        }

        let exists = if let Some(exclude_id) = exclude_id {
            query!(&query_str, scope as ConfigScope, scope_id, key, exclude_id)
                .fetch_optional(self.db.as_ref())
                .await
        } else {
            query!(&query_str, scope as ConfigScope, scope_id, key)
                .fetch_optional(self.db.as_ref())
                .await
        }
        .map_err(Error::Database)?;

        if exists.is_some() {
            return Err(Error::Validation("Configuration key already exists in this scope".to_string()));
        }

        Ok(())
    }

    fn validate_config_value(
        &self,
        config_type: &ConfigType,
        value: &serde_json::Value,
        validation_rules: &Option<serde_json::Value>,
    ) -> Result<()> {
        // Type validation
        match config_type {
            ConfigType::String => {
                if !value.is_string() {
                    return Err(Error::Validation("Value must be a string".to_string()));
                }
            }
            ConfigType::Number => {
                if !value.is_number() {
                    return Err(Error::Validation("Value must be a number".to_string()));
                }
            }
            ConfigType::Boolean => {
                if !value.is_boolean() {
                    return Err(Error::Validation("Value must be a boolean".to_string()));
                }
            }
            ConfigType::Json => {
                if !value.is_object() && !value.is_array() {
                    return Err(Error::Validation("Value must be a JSON object or array".to_string()));
                }
            }
            ConfigType::Encrypted => {
                if !value.is_string() {
                    return Err(Error::Validation("Encrypted value must be a string".to_string()));
                }
            }
        }

        // Additional validation rules (simplified implementation)
        if let Some(rules) = validation_rules {
            if let Some(min_length) = rules.get("min_length") {
                if let (Some(str_val), Some(min)) = (value.as_str(), min_length.as_u64()) {
                    if str_val.len() < min as usize {
                        return Err(Error::Validation(format!("Value must be at least {} characters", min)));
                    }
                }
            }
        }

        Ok(())
    }

    fn config_row_to_model(&self, row: ConfigurationRow) -> Result<Configuration> {
        Ok(Configuration {
            id: row.id,
            scope: row.scope,
            scope_id: row.scope_id,
            key: row.key,
            display_name: row.display_name,
            description: row.description,
            config_type: row.config_type,
            value: row.value,
            default_value: row.default_value,
            is_sensitive: row.is_sensitive,
            is_readonly: row.is_readonly,
            validation_rules: row.validation_rules,
            category: row.category,
            tags: row.tags,
            created_at: row.created_at,
            updated_at: row.updated_at,
            created_by: row.created_by,
            updated_by: row.updated_by,
        })
    }
}

// ============================================================================
// DATABASE ROW TYPES
// ============================================================================

#[derive(Debug)]
struct ConfigurationRow {
    pub id: Uuid,
    pub scope: ConfigScope,
    pub scope_id: Option<Uuid>,
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

#[derive(Debug)]
struct ConfigurationAuditRow {
    pub id: Uuid,
    pub configuration_id: Uuid,
    pub action: String,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}