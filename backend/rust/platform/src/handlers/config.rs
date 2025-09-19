// ============================================================================
// OLYMPUS CLOUD - CONFIGURATION HANDLERS
// ============================================================================
// Module: platform/src/handlers/config.rs
// Description: HTTP handlers for configuration management APIs (feature flags & system config)
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use std::collections::HashMap;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};

use olympus_shared::error::{Result, Error};
use crate::models::{
    FeatureFlag, FeatureFlagEvaluation, FeatureFlagUsage, FeatureFlagEvaluationRequest,
    CreateFeatureFlagRequest, UpdateFeatureFlagRequest,
    Configuration, ConfigurationAudit, ConfigScope, ConfigType,
    ConfigurationSearchRequest, ConfigurationSearchResponse,
    CreateConfigurationRequest, UpdateConfigurationRequest,
};
use crate::services::{FeatureFlagsService, ConfigurationService};

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

pub fn create_configuration_router(
    feature_flags_service: Arc<FeatureFlagsService>,
    config_service: Arc<ConfigurationService>,
) -> Router {
    Router::new()
        // Feature Flag endpoints
        .route("/feature-flags", post(create_feature_flag))
        .route("/feature-flags", get(list_feature_flags))
        .route("/feature-flags/:flag_id", get(get_feature_flag))
        .route("/feature-flags/:flag_id", put(update_feature_flag))
        .route("/feature-flags/:flag_id", delete(delete_feature_flag))
        .route("/feature-flags/key/:flag_key", get(get_feature_flag_by_key))
        .route("/feature-flags/evaluate", post(evaluate_feature_flag))
        .route("/feature-flags/:flag_key/usage", get(get_feature_flag_usage))

        // System Configuration endpoints
        .route("/configurations", post(create_configuration))
        .route("/configurations", get(search_configurations))
        .route("/configurations/:config_id", get(get_configuration))
        .route("/configurations/:config_id", put(update_configuration))
        .route("/configurations/:config_id", delete(delete_configuration))
        .route("/configurations/:config_id/audit", get(get_configuration_audit))
        .route("/configurations/scope/:scope", get(get_configurations_by_scope))
        .route("/configurations/key/:scope/:key", get(get_configuration_by_key))

        .with_state((feature_flags_service, config_service))
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlagResponse {
    pub success: bool,
    pub data: FeatureFlag,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlagListResponse {
    pub success: bool,
    pub data: Vec<FeatureFlag>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlagEvaluationResponse {
    pub success: bool,
    pub data: FeatureFlagEvaluation,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureFlagUsageResponse {
    pub success: bool,
    pub data: FeatureFlagUsage,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationResponse {
    pub success: bool,
    pub data: Configuration,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationListResponse {
    pub success: bool,
    pub data: ConfigurationSearchResponse,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigurationAuditResponse {
    pub success: bool,
    pub data: Vec<ConfigurationAudit>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct FeatureFlagListQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UsageQuery {
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigScopeQuery {
    pub scope_id: Option<Uuid>,
    pub include_sensitive: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigKeyQuery {
    pub include_sensitive: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

// ============================================================================
// FEATURE FLAG HANDLERS
// ============================================================================

pub async fn create_feature_flag(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Json(request): Json<CreateFeatureFlagRequest>,
) -> Result<Json<FeatureFlagResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| Error::Validation(format!("Invalid request: {}", e)))?;

    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID
    let created_by = Uuid::new_v4(); // Mock user ID

    let flag = feature_flags_service
        .create_feature_flag(tenant_id, request, created_by)
        .await?;

    Ok(Json(FeatureFlagResponse {
        success: true,
        data: flag,
        message: "Feature flag created successfully".to_string(),
    }))
}

pub async fn get_feature_flag(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(flag_id): Path<Uuid>,
) -> Result<Json<FeatureFlagResponse>> {
    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID

    let flag = feature_flags_service
        .get_feature_flag(tenant_id, flag_id)
        .await?
        .ok_or_else(|| Error::NotFound("Feature flag not found".to_string()))?;

    Ok(Json(FeatureFlagResponse {
        success: true,
        data: flag,
        message: "Feature flag retrieved successfully".to_string(),
    }))
}

pub async fn get_feature_flag_by_key(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(flag_key): Path<String>,
) -> Result<Json<FeatureFlagResponse>> {
    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID

    let flag = feature_flags_service
        .get_feature_flag_by_key(tenant_id, &flag_key)
        .await?
        .ok_or_else(|| Error::NotFound("Feature flag not found".to_string()))?;

    Ok(Json(FeatureFlagResponse {
        success: true,
        data: flag,
        message: "Feature flag retrieved successfully".to_string(),
    }))
}

pub async fn list_feature_flags(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Query(query): Query<FeatureFlagListQuery>,
) -> Result<Json<FeatureFlagListResponse>> {
    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID

    let flags = feature_flags_service
        .list_feature_flags(tenant_id, query.limit, query.offset)
        .await?;

    Ok(Json(FeatureFlagListResponse {
        success: true,
        data: flags,
        message: "Feature flags retrieved successfully".to_string(),
    }))
}

pub async fn update_feature_flag(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(flag_id): Path<Uuid>,
    Json(request): Json<UpdateFeatureFlagRequest>,
) -> Result<Json<FeatureFlagResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| Error::Validation(format!("Invalid request: {}", e)))?;

    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID
    let updated_by = Uuid::new_v4(); // Mock user ID

    let flag = feature_flags_service
        .update_feature_flag(tenant_id, flag_id, request, updated_by)
        .await?
        .ok_or_else(|| Error::NotFound("Feature flag not found".to_string()))?;

    Ok(Json(FeatureFlagResponse {
        success: true,
        data: flag,
        message: "Feature flag updated successfully".to_string(),
    }))
}

pub async fn delete_feature_flag(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(flag_id): Path<Uuid>,
) -> Result<StatusCode> {
    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID
    let deleted_by = Uuid::new_v4(); // Mock user ID

    let deleted = feature_flags_service
        .delete_feature_flag(tenant_id, flag_id, deleted_by)
        .await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(Error::NotFound("Feature flag not found".to_string()))
    }
}

pub async fn evaluate_feature_flag(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Json(request): Json<FeatureFlagEvaluationRequest>,
) -> Result<Json<FeatureFlagEvaluationResponse>> {
    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID

    let evaluation = feature_flags_service
        .evaluate_feature_flag(tenant_id, request)
        .await?;

    Ok(Json(FeatureFlagEvaluationResponse {
        success: true,
        data: evaluation,
        message: "Feature flag evaluated successfully".to_string(),
    }))
}

pub async fn get_feature_flag_usage(
    State((feature_flags_service, _)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(flag_key): Path<String>,
    Query(query): Query<UsageQuery>,
) -> Result<Json<FeatureFlagUsageResponse>> {
    let tenant_id = Some(Uuid::new_v4()); // Mock tenant ID
    let now = Utc::now();

    let period_start = query.period_start.unwrap_or_else(|| now - chrono::Duration::days(30));
    let period_end = query.period_end.unwrap_or(now);

    let usage = feature_flags_service
        .get_flag_usage_analytics(tenant_id, &flag_key, period_start, period_end)
        .await?
        .ok_or_else(|| Error::NotFound("Feature flag usage not found".to_string()))?;

    Ok(Json(FeatureFlagUsageResponse {
        success: true,
        data: usage,
        message: "Feature flag usage retrieved successfully".to_string(),
    }))
}

// ============================================================================
// SYSTEM CONFIGURATION HANDLERS
// ============================================================================

pub async fn create_configuration(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Json(request): Json<CreateConfigurationRequest>,
) -> Result<Json<ConfigurationResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| Error::Validation(format!("Invalid request: {}", e)))?;

    let created_by = Uuid::new_v4(); // Mock user ID

    let config = config_service
        .create_configuration(request, created_by)
        .await?;

    Ok(Json(ConfigurationResponse {
        success: true,
        data: config,
        message: "Configuration created successfully".to_string(),
    }))
}

pub async fn get_configuration(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(config_id): Path<Uuid>,
    Query(query): Query<ConfigKeyQuery>,
) -> Result<Json<ConfigurationResponse>> {
    let include_sensitive = query.include_sensitive.unwrap_or(false);

    let config = config_service
        .get_configuration(config_id, include_sensitive)
        .await?
        .ok_or_else(|| Error::NotFound("Configuration not found".to_string()))?;

    Ok(Json(ConfigurationResponse {
        success: true,
        data: config,
        message: "Configuration retrieved successfully".to_string(),
    }))
}

pub async fn get_configuration_by_key(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path((scope_str, key)): Path<(String, String)>,
    Query(query): Query<ConfigScopeQuery>,
) -> Result<Json<ConfigurationResponse>> {
    let scope = match scope_str.as_str() {
        "global" => ConfigScope::Global,
        "tenant" => ConfigScope::Tenant,
        "user" => ConfigScope::User,
        "location" => ConfigScope::Location,
        _ => return Err(Error::Validation("Invalid scope".to_string())),
    };

    let include_sensitive = query.include_sensitive.unwrap_or(false);

    let config = config_service
        .get_configuration_by_key(scope, query.scope_id, &key, include_sensitive)
        .await?
        .ok_or_else(|| Error::NotFound("Configuration not found".to_string()))?;

    Ok(Json(ConfigurationResponse {
        success: true,
        data: config,
        message: "Configuration retrieved successfully".to_string(),
    }))
}

pub async fn search_configurations(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Json(request): Json<ConfigurationSearchRequest>,
) -> Result<Json<ConfigurationListResponse>> {
    let response = config_service
        .search_configurations(request)
        .await?;

    Ok(Json(ConfigurationListResponse {
        success: true,
        data: response,
        message: "Configurations searched successfully".to_string(),
    }))
}

pub async fn get_configurations_by_scope(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(scope_str): Path<String>,
    Query(query): Query<ConfigScopeQuery>,
) -> Result<Json<serde_json::Value>> {
    let scope = match scope_str.as_str() {
        "global" => ConfigScope::Global,
        "tenant" => ConfigScope::Tenant,
        "user" => ConfigScope::User,
        "location" => ConfigScope::Location,
        _ => return Err(Error::Validation("Invalid scope".to_string())),
    };

    let include_sensitive = query.include_sensitive.unwrap_or(false);

    let configs = config_service
        .get_configurations_by_scope(scope, query.scope_id, include_sensitive)
        .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": configs,
        "message": "Configurations retrieved successfully"
    })))
}

pub async fn update_configuration(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(config_id): Path<Uuid>,
    Json(request): Json<UpdateConfigurationRequest>,
) -> Result<Json<ConfigurationResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| Error::Validation(format!("Invalid request: {}", e)))?;

    let updated_by = Uuid::new_v4(); // Mock user ID

    let config = config_service
        .update_configuration(config_id, request, updated_by, None, None)
        .await?
        .ok_or_else(|| Error::NotFound("Configuration not found".to_string()))?;

    Ok(Json(ConfigurationResponse {
        success: true,
        data: config,
        message: "Configuration updated successfully".to_string(),
    }))
}

pub async fn delete_configuration(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(config_id): Path<Uuid>,
) -> Result<StatusCode> {
    let deleted_by = Uuid::new_v4(); // Mock user ID

    let deleted = config_service
        .delete_configuration(config_id, deleted_by)
        .await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(Error::NotFound("Configuration not found".to_string()))
    }
}

pub async fn get_configuration_audit(
    State((_, config_service)): State<(Arc<FeatureFlagsService>, Arc<ConfigurationService>)>,
    Path(config_id): Path<Uuid>,
    Query(query): Query<AuditQuery>,
) -> Result<Json<ConfigurationAuditResponse>> {
    let audits = config_service
        .get_configuration_audit_history(config_id, query.limit, query.offset)
        .await?;

    Ok(Json(ConfigurationAuditResponse {
        success: true,
        data: audits,
        message: "Configuration audit history retrieved successfully".to_string(),
    }))
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Error::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(serde_json::json!({
            "success": false,
            "error": error_message
        }));

        (status, body).into_response()
    }
}