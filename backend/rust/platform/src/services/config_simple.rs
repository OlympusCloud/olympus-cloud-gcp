// ============================================================================
// OLYMPUS CLOUD - SIMPLE CONFIGURATION SERVICE
// ============================================================================
// Module: platform/src/services/config_simple.rs
// Description: Simplified configuration service without event publishing
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;

use crate::models::{
    Configuration, ConfigScope, ConfigType,
    CreateConfigurationRequest, UpdateConfigurationRequest,
};

#[derive(Clone)]
pub struct SimpleConfigurationService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl SimpleConfigurationService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ============================================================================
    // BASIC CONFIGURATION OPERATIONS
    // ============================================================================

    pub async fn get_configuration_by_key(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        key: &str,
    ) -> Result<Option<Configuration>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id, scope, scope_id, key, display_name, description, config_type,
                value, default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            FROM platform.configurations
            WHERE scope = $1::config_scope AND key = $2
              AND (scope_id = $3 OR ($3 IS NULL AND scope_id IS NULL))
            "#,
            scope as ConfigScope,
            key,
            scope_id
        )
        .fetch_optional(&*self.db)
        .await?;

        match rows {
            Some(row) => Ok(Some(Configuration {
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
            })),
            None => Ok(None),
        }
    }

    pub async fn get_configurations_by_scope(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
    ) -> Result<Vec<Configuration>> {
        let rows = sqlx::query!(
            r#"
            SELECT
                id, scope, scope_id, key, display_name, description, config_type,
                value, default_value, is_sensitive, is_readonly, validation_rules,
                category, tags, created_at, updated_at, created_by, updated_by
            FROM platform.configurations
            WHERE scope = $1::config_scope
              AND (scope_id = $2 OR ($2 IS NULL AND scope_id IS NULL))
            ORDER BY category, key
            "#,
            scope as ConfigScope,
            scope_id
        )
        .fetch_all(&*self.db)
        .await?;

        Ok(rows.into_iter().map(|row| Configuration {
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
        }).collect())
    }

    pub async fn get_configuration_value<T>(&self, scope: ConfigScope, scope_id: Option<Uuid>, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        if let Some(config) = self.get_configuration_by_key(scope, scope_id, key).await? {
            match serde_json::from_value(config.value) {
                Ok(value) => Ok(Some(value)),
                Err(_) => {
                    // Try default value if main value fails to deserialize
                    match serde_json::from_value(config.default_value) {
                        Ok(value) => Ok(Some(value)),
                        Err(_) => Ok(None),
                    }
                }
            }
        } else {
            Ok(None)
        }
    }

    pub async fn get_string_config(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        key: &str,
        default: Option<&str>,
    ) -> Result<String> {
        match self.get_configuration_value::<String>(scope, scope_id, key).await? {
            Some(value) => Ok(value),
            None => Ok(default.unwrap_or("").to_string()),
        }
    }

    pub async fn get_bool_config(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        key: &str,
        default: bool,
    ) -> Result<bool> {
        match self.get_configuration_value::<bool>(scope, scope_id, key).await? {
            Some(value) => Ok(value),
            None => Ok(default),
        }
    }

    pub async fn get_number_config(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        key: &str,
        default: f64,
    ) -> Result<f64> {
        match self.get_configuration_value::<f64>(scope, scope_id, key).await? {
            Some(value) => Ok(value),
            None => Ok(default),
        }
    }

    // ============================================================================
    // TENANT-SPECIFIC HELPERS
    // ============================================================================

    pub async fn get_tenant_config<T>(&self, tenant_id: Uuid, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.get_configuration_value(ConfigScope::Tenant, Some(tenant_id), key).await
    }

    pub async fn get_global_config<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.get_configuration_value(ConfigScope::Global, None, key).await
    }

    // ============================================================================
    // BULK OPERATIONS
    // ============================================================================

    pub async fn get_configurations_by_category(
        &self,
        scope: ConfigScope,
        scope_id: Option<Uuid>,
        category: &str,
    ) -> Result<HashMap<String, serde_json::Value>> {
        let rows = sqlx::query!(
            r#"
            SELECT key, value
            FROM platform.configurations
            WHERE scope = $1::config_scope AND category = $2
              AND (scope_id = $3 OR ($3 IS NULL AND scope_id IS NULL))
            "#,
            scope as ConfigScope,
            category,
            scope_id
        )
        .fetch_all(&*self.db)
        .await?;

        Ok(rows.into_iter()
            .map(|row| (row.key, row.value))
            .collect())
    }

    pub async fn get_all_tenant_configurations(&self, tenant_id: Uuid) -> Result<HashMap<String, serde_json::Value>> {
        let rows = sqlx::query!(
            r#"
            SELECT key, value
            FROM platform.configurations
            WHERE scope = 'tenant'::config_scope AND scope_id = $1
            "#,
            tenant_id
        )
        .fetch_all(&*self.db)
        .await?;

        Ok(rows.into_iter()
            .map(|row| (row.key, row.value))
            .collect())
    }

    // ============================================================================
    // FEATURE FLAG INTEGRATION
    // ============================================================================

    pub async fn is_feature_enabled(&self, tenant_id: Option<Uuid>, feature_key: &str) -> Result<bool> {
        // Check tenant-specific configuration first
        if let Some(tid) = tenant_id {
            if let Some(enabled) = self.get_tenant_config::<bool>(tid, &format!("feature.{}", feature_key)).await? {
                return Ok(enabled);
            }
        }

        // Fall back to global configuration
        if let Some(enabled) = self.get_global_config::<bool>(&format!("feature.{}", feature_key)).await? {
            return Ok(enabled);
        }

        // Default to disabled
        Ok(false)
    }

    pub async fn get_feature_config<T>(&self, tenant_id: Option<Uuid>, feature_key: &str, config_key: &str) -> Result<Option<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let full_key = format!("feature.{}.{}", feature_key, config_key);

        // Check tenant-specific configuration first
        if let Some(tid) = tenant_id {
            if let Some(value) = self.get_tenant_config::<T>(tid, &full_key).await? {
                return Ok(Some(value));
            }
        }

        // Fall back to global configuration
        self.get_global_config::<T>(&full_key).await
    }
}