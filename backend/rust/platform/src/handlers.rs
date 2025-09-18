use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use serde::Deserialize;
use olympus_shared::types::{ApiResponse, PageRequest};
use crate::models::*;
use crate::services::PlatformService;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

// Tenant handlers
pub async fn list_tenants(
    Extension(service): Extension<Arc<PlatformService>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let offset = ((page - 1) * per_page) as i64;

    match service.list_tenants(per_page as i64, offset).await {
        Ok(tenants) => (StatusCode::OK, Json(ApiResponse::success(tenants))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn create_tenant(
    Extension(service): Extension<Arc<PlatformService>>,
    Json(request): Json<CreateTenantRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.create_tenant(request).await {
        Ok(tenant) => (StatusCode::CREATED, Json(ApiResponse::success(tenant))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_tenant(
    Extension(service): Extension<Arc<PlatformService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.get_tenant(id).await {
        Ok(tenant) => (StatusCode::OK, Json(ApiResponse::success(tenant))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "NOT_FOUND".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn update_tenant(
    Extension(service): Extension<Arc<PlatformService>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTenantRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.update_tenant(id, request).await {
        Ok(tenant) => (StatusCode::OK, Json(ApiResponse::success(tenant))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn delete_tenant(
    Extension(service): Extension<Arc<PlatformService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.delete_tenant(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(serde_json::json!({
                "message": "Tenant deleted successfully"
            }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

// User handlers (placeholders - would integrate with auth service)
pub async fn list_users() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::success(Vec::<serde_json::Value>::new())),
    )
}

pub async fn create_user() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "User creation through auth service".to_string(),
        )),
    )
}

pub async fn get_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get user {} through auth service", id),
        )),
    )
}

pub async fn update_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update user {} through auth service", id),
        )),
    )
}

pub async fn delete_user(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Delete user {} through auth service", id),
        )),
    )
}

// Location handlers
pub async fn list_locations(
    Extension(service): Extension<Arc<PlatformService>>,
    Query(params): Query<TenantIdParam>,
) -> impl IntoResponse {
    match service.list_locations(params.tenant_id).await {
        Ok(locations) => (StatusCode::OK, Json(ApiResponse::success(locations))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

#[derive(Deserialize)]
pub struct TenantIdParam {
    pub tenant_id: Uuid,
}

pub async fn create_location(
    Extension(service): Extension<Arc<PlatformService>>,
    Query(params): Query<TenantIdParam>,
    Json(request): Json<CreateLocationRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.create_location(params.tenant_id, request).await {
        Ok(location) => (StatusCode::CREATED, Json(ApiResponse::success(location))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_location(
    Extension(service): Extension<Arc<PlatformService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.get_location(id).await {
        Ok(location) => (StatusCode::OK, Json(ApiResponse::success(location))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "NOT_FOUND".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn update_location(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update location {} not yet implemented", id),
        )),
    )
}

pub async fn delete_location(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Delete location {} not yet implemented", id),
        )),
    )
}

// Role handlers
pub async fn list_roles(
    Extension(service): Extension<Arc<PlatformService>>,
    Query(params): Query<TenantIdParam>,
) -> impl IntoResponse {
    match service.list_roles(params.tenant_id).await {
        Ok(roles) => (StatusCode::OK, Json(ApiResponse::success(roles))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn create_role(
    Extension(service): Extension<Arc<PlatformService>>,
    Query(params): Query<TenantIdParam>,
    Json(request): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.create_role(params.tenant_id, request).await {
        Ok(role) => (StatusCode::CREATED, Json(ApiResponse::success(role))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_role(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get role {} not yet implemented", id),
        )),
    )
}

pub async fn update_role(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update role {} not yet implemented", id),
        )),
    )
}

pub async fn delete_role(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Delete role {} not yet implemented", id),
        )),
    )
}

// Permission handlers
pub async fn list_permissions() -> impl IntoResponse {
    let permissions = vec![
        Permission {
            id: "users.read".to_string(),
            resource: "users".to_string(),
            action: "read".to_string(),
            description: "View users".to_string(),
            category: "User Management".to_string(),
        },
        Permission {
            id: "users.write".to_string(),
            resource: "users".to_string(),
            action: "write".to_string(),
            description: "Create and update users".to_string(),
            category: "User Management".to_string(),
        },
        Permission {
            id: "orders.read".to_string(),
            resource: "orders".to_string(),
            action: "read".to_string(),
            description: "View orders".to_string(),
            category: "Commerce".to_string(),
        },
        Permission {
            id: "orders.write".to_string(),
            resource: "orders".to_string(),
            action: "write".to_string(),
            description: "Create and update orders".to_string(),
            category: "Commerce".to_string(),
        },
    ];

    (StatusCode::OK, Json(ApiResponse::success(permissions)))
}

pub async fn assign_role(
    Extension(service): Extension<Arc<PlatformService>>,
    Path(user_id): Path<Uuid>,
    Json(request): Json<AssignRoleRequest>,
) -> impl IntoResponse {
    match service.assign_role_to_user(user_id, request.role_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(serde_json::json!({
                "message": "Role assigned successfully"
            }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn remove_role(
    Extension(service): Extension<Arc<PlatformService>>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match service.remove_role_from_user(user_id, role_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(serde_json::json!({
                "message": "Role removed successfully"
            }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

// Settings handlers
pub async fn get_settings() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Settings not yet implemented".to_string(),
        )),
    )
}

pub async fn update_settings() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Settings update not yet implemented".to_string(),
        )),
    )
}