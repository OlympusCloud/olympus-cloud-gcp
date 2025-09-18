use axum::{
    extract::{Request, State},
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::AuthService;

#[derive(Clone)]
pub struct AuthState {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub async fn auth_middleware(
    State(auth_service): State<Arc<AuthService>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers().clone();

    let authorization = headers
        .typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = authorization.token();

    match auth_service.verify_token(token).await {
        Ok(claims) => {
            // Add auth state to request extensions
            let auth_state = AuthState {
                user_id: claims.sub,
                tenant_id: claims.tenant_id,
                roles: claims.roles,
                permissions: claims.permissions,
            };

            request.extensions_mut().insert(auth_state);

            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn require_role(
    required_role: &str,
    auth_state: &AuthState,
) -> Result<(), StatusCode> {
    if auth_state.roles.contains(&required_role.to_string()) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn require_permission(
    required_permission: &str,
    auth_state: &AuthState,
) -> Result<(), StatusCode> {
    if auth_state.permissions.contains(&required_permission.to_string()) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn require_any_role(
    required_roles: &[&str],
    auth_state: &AuthState,
) -> Result<(), StatusCode> {
    for role in required_roles {
        if auth_state.roles.contains(&role.to_string()) {
            return Ok(());
        }
    }
    Err(StatusCode::FORBIDDEN)
}

pub async fn require_any_permission(
    required_permissions: &[&str],
    auth_state: &AuthState,
) -> Result<(), StatusCode> {
    for permission in required_permissions {
        if auth_state.permissions.contains(&permission.to_string()) {
            return Ok(());
        }
    }
    Err(StatusCode::FORBIDDEN)
}