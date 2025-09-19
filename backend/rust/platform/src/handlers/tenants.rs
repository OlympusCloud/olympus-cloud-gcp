// ============================================================================
// OLYMPUS CLOUD - TENANT HANDLERS
// ============================================================================
// Module: platform/src/handlers/tenants.rs
// Description: HTTP handlers for tenant administration and management
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post, put},
    Router,
};
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

use olympus_shared::types::{ApiResponse, PageRequest};
use crate::models::{
    CreateTenantRequest,
    TenantDetail,
    TenantSummary,
    UpdateFeatureFlagsRequest,
    UpdateSubscriptionRequest,
    UpdateTenantRequest
};
use crate::services::TenantService;

/// Application state for tenant handlers
#[derive(Clone)]
pub struct TenantState {
    pub tenant_service: Arc<TenantService>,
}

/// Create tenant router with all endpoints
pub fn create_tenant_router(tenant_service: Arc<TenantService>) -> Router {
    let state = TenantState { tenant_service };

    Router::new()
        // Tenant CRUD operations
        .route("/tenants", post(create_tenant))
        .route("/tenants", get(list_tenants))
        .route("/tenants/:tenant_id", get(get_tenant))
        .route("/tenants/:tenant_id", put(update_tenant))
        .route("/tenants/:tenant_id", delete(delete_tenant))

        // Tenant by slug
        .route("/tenants/by-slug/:slug", get(get_tenant_by_slug))

        // Tenant status management
        .route("/tenants/:tenant_id/activate", post(activate_tenant))
        .route("/tenants/:tenant_id/suspend", post(suspend_tenant))

        // Subscription management
        .route("/tenants/:tenant_id/subscription", put(update_subscription))

        // Feature flag management
        .route("/tenants/:tenant_id/feature-flags", put(update_feature_flags))

        // Tenant statistics
        .route("/tenants/:tenant_id/stats", get(get_tenant_stats))

        .with_state(state)
}

/// Query parameters for listing tenants
#[derive(Debug, Deserialize)]
pub struct ListTenantsQuery {
    #[serde(flatten)]
    page: PageRequest,

    /// Filter by status
    status: Option<String>,

    /// Filter by subscription tier
    tier: Option<String>,

    /// Search in name/slug
    search: Option<String>,
}

/// Create a new tenant
/// POST /api/v1/platform/tenants
pub async fn create_tenant(
    State(state): State<TenantState>,
    Json(request): Json<CreateTenantRequest>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Creating tenant: {}", request.slug);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        error!("Tenant creation validation failed: {:?}", validation_errors);
        return Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            format!("Validation failed: {:?}", validation_errors),
        )));
    }

    match state.tenant_service.create_tenant(request).await {
        Ok(tenant) => {
            info!("Tenant created successfully: {}", tenant.id);
            Ok(Json(ApiResponse::success(tenant)))
        }
        Err(e) => {
            error!("Failed to create tenant: {}", e);
            match e {
                olympus_shared::Error::AlreadyExists(msg) => Ok(Json(ApiResponse::error(
                    "SLUG_ALREADY_EXISTS".to_string(),
                    msg,
                ))),
                olympus_shared::Error::Validation(msg) => Ok(Json(ApiResponse::error(
                    "VALIDATION_ERROR".to_string(),
                    msg,
                ))),
                _ => Ok(Json(ApiResponse::error(
                    "INTERNAL_ERROR".to_string(),
                    "Failed to create tenant".to_string(),
                ))),
            }
        }
    }
}

/// Get tenant by ID
/// GET /api/v1/platform/tenants/:tenant_id
pub async fn get_tenant(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Getting tenant: {}", tenant_id);

    match state.tenant_service.get_tenant(tenant_id).await {
        Ok(tenant) => Ok(Json(ApiResponse::success(tenant))),
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to get tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to get tenant".to_string(),
            )))
        }
    }
}

/// Get tenant by slug
/// GET /api/v1/platform/tenants/by-slug/:slug
pub async fn get_tenant_by_slug(
    State(state): State<TenantState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Getting tenant by slug: {}", slug);

    match state.tenant_service.get_tenant_by_slug(&slug).await {
        Ok(tenant) => Ok(Json(ApiResponse::success(tenant))),
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to get tenant by slug {}: {}", slug, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to get tenant".to_string(),
            )))
        }
    }
}

/// List tenants with pagination and filtering
/// GET /api/v1/platform/tenants
pub async fn list_tenants(
    State(state): State<TenantState>,
    Query(query): Query<ListTenantsQuery>,
) -> Result<Json<ApiResponse<olympus_shared::types::PageResponse<TenantSummary>>>, StatusCode> {
    info!("Listing tenants with query: {:?}", query);

    // TODO: Implement filtering by status, tier, and search
    // For now, just use pagination
    match state.tenant_service.list_tenants(query.page).await {
        Ok(page_response) => Ok(Json(ApiResponse::success(page_response))),
        Err(e) => {
            error!("Failed to list tenants: {}", e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to list tenants".to_string(),
            )))
        }
    }
}

/// Update tenant
/// PUT /api/v1/platform/tenants/:tenant_id
pub async fn update_tenant(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
    Json(request): Json<UpdateTenantRequest>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Updating tenant: {}", tenant_id);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        error!("Tenant update validation failed: {:?}", validation_errors);
        return Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            format!("Validation failed: {:?}", validation_errors),
        )));
    }

    match state.tenant_service.update_tenant(tenant_id, request).await {
        Ok(tenant) => {
            info!("Tenant updated successfully: {}", tenant_id);
            Ok(Json(ApiResponse::success(tenant)))
        }
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(olympus_shared::Error::Validation(msg)) => Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            msg,
        ))),
        Err(e) => {
            error!("Failed to update tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to update tenant".to_string(),
            )))
        }
    }
}

/// Activate tenant (end trial period)
/// POST /api/v1/platform/tenants/:tenant_id/activate
pub async fn activate_tenant(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Activating tenant: {}", tenant_id);

    match state.tenant_service.activate_tenant(tenant_id).await {
        Ok(tenant) => {
            info!("Tenant activated successfully: {}", tenant_id);
            Ok(Json(ApiResponse::success(tenant)))
        }
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to activate tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to activate tenant".to_string(),
            )))
        }
    }
}

/// Suspend tenant
/// POST /api/v1/platform/tenants/:tenant_id/suspend
pub async fn suspend_tenant(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Suspending tenant: {}", tenant_id);

    match state.tenant_service.suspend_tenant(tenant_id).await {
        Ok(tenant) => {
            info!("Tenant suspended successfully: {}", tenant_id);
            Ok(Json(ApiResponse::success(tenant)))
        }
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to suspend tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to suspend tenant".to_string(),
            )))
        }
    }
}

/// Update tenant subscription
/// PUT /api/v1/platform/tenants/:tenant_id/subscription
pub async fn update_subscription(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
    Json(request): Json<UpdateSubscriptionRequest>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Updating subscription for tenant: {}", tenant_id);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        error!("Subscription update validation failed: {:?}", validation_errors);
        return Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            format!("Validation failed: {:?}", validation_errors),
        )));
    }

    match state.tenant_service.update_subscription(tenant_id, request).await {
        Ok(tenant) => {
            info!("Subscription updated successfully for tenant: {}", tenant_id);
            Ok(Json(ApiResponse::success(tenant)))
        }
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(olympus_shared::Error::Validation(msg)) => Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            msg,
        ))),
        Err(e) => {
            error!("Failed to update subscription for tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to update subscription".to_string(),
            )))
        }
    }
}

/// Update tenant feature flags
/// PUT /api/v1/platform/tenants/:tenant_id/feature-flags
pub async fn update_feature_flags(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
    Json(request): Json<UpdateFeatureFlagsRequest>,
) -> Result<Json<ApiResponse<TenantDetail>>, StatusCode> {
    info!("Updating feature flags for tenant: {}", tenant_id);

    match state.tenant_service.update_feature_flags(tenant_id, request).await {
        Ok(tenant) => {
            info!("Feature flags updated successfully for tenant: {}", tenant_id);
            Ok(Json(ApiResponse::success(tenant)))
        }
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to update feature flags for tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to update feature flags".to_string(),
            )))
        }
    }
}

/// Get tenant usage statistics
/// GET /api/v1/platform/tenants/:tenant_id/stats
pub async fn get_tenant_stats(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    info!("Getting stats for tenant: {}", tenant_id);

    match state.tenant_service.get_tenant_stats(tenant_id).await {
        Ok(stats) => Ok(Json(ApiResponse::success(stats))),
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to get stats for tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to get tenant stats".to_string(),
            )))
        }
    }
}

/// Soft delete tenant
/// DELETE /api/v1/platform/tenants/:tenant_id
pub async fn delete_tenant(
    State(state): State<TenantState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    info!("Deleting tenant: {}", tenant_id);

    match state.tenant_service.delete_tenant(tenant_id).await {
        Ok(_) => {
            info!("Tenant deleted successfully: {}", tenant_id);
            Ok(Json(ApiResponse::success(())))
        }
        Err(olympus_shared::Error::NotFound(_)) => Ok(Json(ApiResponse::error(
            "TENANT_NOT_FOUND".to_string(),
            "Tenant not found".to_string(),
        ))),
        Err(e) => {
            error!("Failed to delete tenant {}: {}", tenant_id, e);
            Ok(Json(ApiResponse::error(
                "INTERNAL_ERROR".to_string(),
                "Failed to delete tenant".to_string(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Method;
    use axum_test::TestServer;
    use serde_json::json;
    use std::sync::Arc;
    use uuid::Uuid;

    // Mock tenant service for testing
    struct MockTenantService;

    #[async_trait::async_trait]
    impl TenantService for MockTenantService {
        // Implementation would go here for testing
        // For now, this is just a placeholder structure
    }

    #[tokio::test]
    async fn test_create_tenant_endpoint() {
        // This would be a comprehensive test for the create tenant endpoint
        // Testing validation, success cases, and error cases

        let request = json!({
            "slug": "test-tenant",
            "name": "Test Tenant",
            "industry": "Technology"
        });

        // Test would verify:
        // 1. Successful tenant creation
        // 2. Validation error handling
        // 3. Duplicate slug handling
        // 4. Proper response format
    }

    #[tokio::test]
    async fn test_get_tenant_endpoint() {
        // Test for getting tenant by ID
        // Would verify:
        // 1. Successful retrieval
        // 2. Not found handling
        // 3. Proper response format
    }

    #[tokio::test]
    async fn test_list_tenants_endpoint() {
        // Test for listing tenants with pagination
        // Would verify:
        // 1. Pagination parameters
        // 2. Filtering parameters
        // 3. Proper response format
    }

    #[tokio::test]
    async fn test_update_tenant_endpoint() {
        // Test for updating tenant
        // Would verify:
        // 1. Successful updates
        // 2. Validation handling
        // 3. Not found handling
    }

    #[tokio::test]
    async fn test_subscription_management() {
        // Test subscription update endpoint
        // Would verify:
        // 1. Subscription tier changes
        // 2. Feature flag updates
        // 3. Billing information updates
    }

    #[tokio::test]
    async fn test_tenant_status_management() {
        // Test activate/suspend endpoints
        // Would verify:
        // 1. Status transitions
        // 2. Event publishing
        // 3. Proper error handling
    }
}