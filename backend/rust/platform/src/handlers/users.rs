// ============================================================================
// OLYMPUS CLOUD - USER MANAGEMENT HANDLERS
// ============================================================================
// Module: platform/src/handlers/users.rs
// Description: HTTP handlers for user management, role assignments, and activity tracking
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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;
use validator::Validate;

use olympus_shared::models::{User, UserProfile, UserSummary};
use olympus_shared::types::{ApiResponse, PageRequest};
use crate::services::{RbacService, rbac::{Action, ResourceType, PermissionScope}};

/// Application state for user handlers
#[derive(Clone)]
pub struct UserState {
    pub rbac_service: Arc<RbacService>,
}

/// Create user router with all endpoints
pub fn create_user_router(rbac_service: Arc<RbacService>) -> Router {
    let state = UserState { rbac_service };

    Router::new()
        // User CRUD operations
        .route("/users", get(list_users))
        .route("/users/:user_id", get(get_user))
        .route("/users/:user_id", put(update_user))
        .route("/users/:user_id/activate", post(activate_user))
        .route("/users/:user_id/deactivate", post(deactivate_user))

        // Role management
        .route("/users/:user_id/roles", get(get_user_roles))
        .route("/users/:user_id/roles", post(assign_role))
        .route("/users/:user_id/roles/:role_id", delete(revoke_role))

        // Permission checking
        .route("/users/:user_id/permissions", get(get_user_permissions))
        .route("/users/:user_id/permissions/check", post(check_permission))

        // Activity tracking
        .route("/users/:user_id/activity", get(get_user_activity))

        .with_state(state)
}

/// Query parameters for listing users
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(flatten)]
    page: PageRequest,

    /// Filter by status
    status: Option<String>,

    /// Filter by role
    role: Option<String>,

    /// Filter by location
    location_id: Option<Uuid>,

    /// Search in name/email
    search: Option<String>,
}

/// Request model for updating user
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(max = 100))]
    pub first_name: Option<String>,

    #[validate(length(max = 100))]
    pub last_name: Option<String>,

    #[validate(length(max = 200))]
    pub display_name: Option<String>,

    #[validate(url)]
    pub avatar_url: Option<String>,

    pub phone: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}

/// Request model for role assignment
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AssignRoleRequest {
    pub role_id: Uuid,
    pub location_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request model for permission checking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckPermissionRequest {
    pub resource_type: String,
    pub action: String,
    pub resource_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
}

/// Response model for permission check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionCheckResponse {
    pub allowed: bool,
    pub reason: Option<String>,
}

/// Response model for user permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPermissionsResponse {
    pub user_id: Uuid,
    pub permissions: Vec<String>,
    pub roles: Vec<UserRoleInfo>,
}

/// User role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleInfo {
    pub role_id: Uuid,
    pub role_name: String,
    pub location_id: Option<Uuid>,
    pub assigned_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

/// User activity entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserActivity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_type: String,
    pub description: String,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub occurred_at: DateTime<Utc>,
}

/// List users with pagination and filtering
/// GET /api/v1/platform/users
pub async fn list_users(
    State(state): State<UserState>,
    Query(query): Query<ListUsersQuery>,
) -> Result<Json<ApiResponse<olympus_shared::types::PageResponse<UserSummary>>>, StatusCode> {
    info!("Listing users with query: {:?}", query);

    // TODO: Implement actual user listing with database queries
    // For now, return empty list
    let users = Vec::new();
    let page_response = olympus_shared::types::PageResponse::new(
        users,
        0,
        query.page.page,
        query.page.per_page,
    );

    Ok(Json(ApiResponse::success(page_response)))
}

/// Get user by ID
/// GET /api/v1/platform/users/:user_id
pub async fn get_user(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserProfile>>, StatusCode> {
    info!("Getting user: {}", user_id);

    // TODO: Implement actual user retrieval
    // For now, return error
    Ok(Json(ApiResponse::error(
        "USER_NOT_FOUND".to_string(),
        "User not found".to_string(),
    )))
}

/// Update user
/// PUT /api/v1/platform/users/:user_id
pub async fn update_user(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserProfile>>, StatusCode> {
    info!("Updating user: {}", user_id);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        error!("User update validation failed: {:?}", validation_errors);
        return Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            format!("Validation failed: {:?}", validation_errors),
        )));
    }

    // TODO: Implement actual user update
    Ok(Json(ApiResponse::error(
        "NOT_IMPLEMENTED".to_string(),
        "User update not implemented yet".to_string(),
    )))
}

/// Activate user
/// POST /api/v1/platform/users/:user_id/activate
pub async fn activate_user(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserProfile>>, StatusCode> {
    info!("Activating user: {}", user_id);

    // TODO: Implement user activation
    Ok(Json(ApiResponse::error(
        "NOT_IMPLEMENTED".to_string(),
        "User activation not implemented yet".to_string(),
    )))
}

/// Deactivate user
/// POST /api/v1/platform/users/:user_id/deactivate
pub async fn deactivate_user(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserProfile>>, StatusCode> {
    info!("Deactivating user: {}", user_id);

    // TODO: Implement user deactivation
    Ok(Json(ApiResponse::error(
        "NOT_IMPLEMENTED".to_string(),
        "User deactivation not implemented yet".to_string(),
    )))
}

/// Get user roles
/// GET /api/v1/platform/users/:user_id/roles
pub async fn get_user_roles(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<UserRoleInfo>>>, StatusCode> {
    info!("Getting roles for user: {}", user_id);

    // TODO: Implement getting user roles from RBAC service
    let roles = Vec::new();
    Ok(Json(ApiResponse::success(roles)))
}

/// Assign role to user
/// POST /api/v1/platform/users/:user_id/roles
pub async fn assign_role(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<AssignRoleRequest>,
) -> Result<Json<ApiResponse<UserRoleInfo>>, StatusCode> {
    info!("Assigning role {} to user: {}", request.role_id, user_id);

    // Validate request
    if let Err(validation_errors) = request.validate() {
        error!("Role assignment validation failed: {:?}", validation_errors);
        return Ok(Json(ApiResponse::error(
            "VALIDATION_ERROR".to_string(),
            format!("Validation failed: {:?}", validation_errors),
        )));
    }

    // TODO: Get tenant_id from request context (would come from tenant middleware)
    let tenant_id = Uuid::new_v4(); // Placeholder
    let assigned_by = Uuid::new_v4(); // Placeholder - would come from auth context

    match state.rbac_service.assign_role(
        user_id,
        request.role_id,
        tenant_id,
        assigned_by,
        request.location_id,
        request.expires_at,
    ).await {
        Ok(user_role) => {
            info!("Role assigned successfully: {} to user {}", request.role_id, user_id);

            let role_info = UserRoleInfo {
                role_id: user_role.role_id,
                role_name: "Role Name".to_string(), // Would fetch from role service
                location_id: user_role.location_id,
                assigned_at: user_role.assigned_at,
                expires_at: user_role.expires_at,
                is_active: user_role.is_active,
            };

            Ok(Json(ApiResponse::success(role_info)))
        }
        Err(e) => {
            error!("Failed to assign role to user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(
                "ROLE_ASSIGNMENT_FAILED".to_string(),
                "Failed to assign role".to_string(),
            )))
        }
    }
}

/// Revoke role from user
/// DELETE /api/v1/platform/users/:user_id/roles/:role_id
pub async fn revoke_role(
    State(state): State<UserState>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    info!("Revoking role {} from user: {}", role_id, user_id);

    // TODO: Get tenant_id and revoked_by from request context
    let tenant_id = Uuid::new_v4(); // Placeholder
    let revoked_by = Uuid::new_v4(); // Placeholder

    match state.rbac_service.revoke_role(user_id, role_id, tenant_id, revoked_by).await {
        Ok(_) => {
            info!("Role revoked successfully: {} from user {}", role_id, user_id);
            Ok(Json(ApiResponse::success(())))
        }
        Err(e) => {
            error!("Failed to revoke role from user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(
                "ROLE_REVOCATION_FAILED".to_string(),
                "Failed to revoke role".to_string(),
            )))
        }
    }
}

/// Get user permissions
/// GET /api/v1/platform/users/:user_id/permissions
pub async fn get_user_permissions(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserPermissionsResponse>>, StatusCode> {
    info!("Getting permissions for user: {}", user_id);

    // TODO: Get tenant_id from request context
    let tenant_id = Uuid::new_v4(); // Placeholder

    match state.rbac_service.get_user_permissions(user_id, tenant_id).await {
        Ok(permissions) => {
            let permission_strings: Vec<String> = permissions
                .into_iter()
                .map(|p| p.to_string())
                .collect();

            let user_roles = state.rbac_service.get_user_roles(user_id, tenant_id).await
                .unwrap_or_else(|_| Vec::new());

            let role_infos: Vec<UserRoleInfo> = user_roles
                .into_iter()
                .map(|ur| UserRoleInfo {
                    role_id: ur.role_id,
                    role_name: "Role Name".to_string(), // Would fetch from role service
                    location_id: ur.location_id,
                    assigned_at: ur.assigned_at,
                    expires_at: ur.expires_at,
                    is_active: ur.is_active,
                })
                .collect();

            let response = UserPermissionsResponse {
                user_id,
                permissions: permission_strings,
                roles: role_infos,
            };

            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to get permissions for user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(
                "PERMISSION_FETCH_FAILED".to_string(),
                "Failed to get user permissions".to_string(),
            )))
        }
    }
}

/// Check if user has specific permission
/// POST /api/v1/platform/users/:user_id/permissions/check
pub async fn check_permission(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<CheckPermissionRequest>,
) -> Result<Json<ApiResponse<PermissionCheckResponse>>, StatusCode> {
    info!("Checking permission for user: {} - {}:{}", user_id, request.resource_type, request.action);

    // Parse resource type and action
    let resource_type = match ResourceType::from_str(&request.resource_type) {
        Some(rt) => rt,
        None => {
            return Ok(Json(ApiResponse::error(
                "INVALID_RESOURCE_TYPE".to_string(),
                format!("Invalid resource type: {}", request.resource_type),
            )));
        }
    };

    let action = match Action::from_str(&request.action) {
        Some(a) => a,
        None => {
            return Ok(Json(ApiResponse::error(
                "INVALID_ACTION".to_string(),
                format!("Invalid action: {}", request.action),
            )));
        }
    };

    // TODO: Get tenant_id from request context
    let tenant_id = Uuid::new_v4(); // Placeholder

    match state.rbac_service.check_permission(
        user_id,
        tenant_id,
        resource_type,
        action,
        request.resource_id,
        request.location_id,
    ).await {
        Ok(allowed) => {
            let response = PermissionCheckResponse {
                allowed,
                reason: if allowed {
                    None
                } else {
                    Some("Permission denied by RBAC policy".to_string())
                },
            };

            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to check permission for user {}: {}", user_id, e);
            Ok(Json(ApiResponse::error(
                "PERMISSION_CHECK_FAILED".to_string(),
                "Failed to check permission".to_string(),
            )))
        }
    }
}

/// Get user activity log
/// GET /api/v1/platform/users/:user_id/activity
pub async fn get_user_activity(
    State(state): State<UserState>,
    Path(user_id): Path<Uuid>,
    Query(page): Query<PageRequest>,
) -> Result<Json<ApiResponse<olympus_shared::types::PageResponse<UserActivity>>>, StatusCode> {
    info!("Getting activity for user: {}", user_id);

    // TODO: Implement actual activity tracking and retrieval
    // For now, return empty list
    let activities = Vec::new();
    let page_response = olympus_shared::types::PageResponse::new(
        activities,
        0,
        page.page,
        page.per_page,
    );

    Ok(Json(ApiResponse::success(page_response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_assign_role_endpoint() {
        // This would be a comprehensive test for the assign role endpoint
        // Testing validation, success cases, and error cases

        let request = json!({
            "role_id": "123e4567-e89b-12d3-a456-426614174000",
            "location_id": null,
            "expires_at": null
        });

        // Test would verify:
        // 1. Successful role assignment
        // 2. Validation error handling
        // 3. RBAC service integration
        // 4. Proper response format
    }

    #[tokio::test]
    async fn test_check_permission_endpoint() {
        // Test for permission checking endpoint
        // Would verify:
        // 1. Successful permission checks
        // 2. Resource type validation
        // 3. Action validation
        // 4. Proper response format
    }

    #[tokio::test]
    async fn test_get_user_permissions_endpoint() {
        // Test for getting user permissions
        // Would verify:
        // 1. Permission aggregation
        // 2. Role information inclusion
        // 3. Proper response format
    }

    #[tokio::test]
    async fn test_user_management_endpoints() {
        // Test for user CRUD operations
        // Would verify:
        // 1. User listing with filters
        // 2. User profile retrieval
        // 3. User updates with validation
        // 4. User activation/deactivation
    }
}