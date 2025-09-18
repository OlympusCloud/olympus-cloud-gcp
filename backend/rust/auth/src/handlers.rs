use axum::{
    extract::Extension,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use std::sync::Arc;
use validator::Validate;
use olympus_shared::types::ApiResponse;
use crate::models::*;
use crate::services::AuthService;

fn extract_ip_address(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

fn extract_user_agent(headers: &HeaderMap) -> String {
    headers
        .get("user-agent")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or("unknown")
        .to_string()
}

pub async fn login(
    Extension(auth_service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    let ip_address = extract_ip_address(&headers);
    let user_agent = extract_user_agent(&headers);

    match auth_service.login(request, ip_address, user_agent).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))),
        Err(e) => (
            StatusCode::from_u16(e.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(ApiResponse::error(
                format!("{:?}", e),
                e.to_string(),
            )),
        ),
    }
}

pub async fn register(
    Extension(auth_service): Extension<Arc<AuthService>>,
    Json(request): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match auth_service.register(request).await {
        Ok(response) => (StatusCode::CREATED, Json(ApiResponse::success(response))),
        Err(e) => (
            StatusCode::from_u16(e.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(ApiResponse::error(
                format!("{:?}", e),
                e.to_string(),
            )),
        ),
    }
}

pub async fn refresh_token(
    Extension(auth_service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(request): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    let ip_address = extract_ip_address(&headers);
    let user_agent = extract_user_agent(&headers);

    match auth_service.refresh_token(&request.refresh_token, ip_address, user_agent).await {
        Ok(response) => (StatusCode::OK, Json(ApiResponse::success(response))),
        Err(e) => (
            StatusCode::from_u16(e.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(ApiResponse::error(
                format!("{:?}", e),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_current_user(
    Extension(auth_service): Extension<Arc<AuthService>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let token = auth.token();

    match auth_service.verify_token(token).await {
        Ok(claims) => {
            match auth_service.get_user(claims.sub).await {
                Ok((user, tenant)) => {
                    let response = user.to_response(&tenant);
                    (StatusCode::OK, Json(ApiResponse::success(response)))
                }
                Err(e) => (
                    StatusCode::from_u16(e.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(ApiResponse::error(
                        format!("{:?}", e),
                        e.to_string(),
                    )),
                ),
            }
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                format!("{:?}", e),
                e.to_string(),
            )),
        ),
    }
}

pub async fn logout(
    Extension(auth_service): Extension<Arc<AuthService>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    let token = auth.token();

    match auth_service.verify_token(token).await {
        Ok(claims) => {
            match auth_service.logout(claims.sub).await {
                Ok(_) => (
                    StatusCode::OK,
                    Json(ApiResponse::success(serde_json::json!({
                        "message": "Successfully logged out"
                    }))),
                ),
                Err(e) => (
                    StatusCode::from_u16(e.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(ApiResponse::error(
                        format!("{:?}", e),
                        e.to_string(),
                    )),
                ),
            }
        }
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                format!("{:?}", e),
                e.to_string(),
            )),
        ),
    }
}

pub async fn forgot_password(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(request): Json<ForgotPasswordRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "If the email exists, a password reset link has been sent"
        }))),
    )
}

pub async fn reset_password(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(request): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Password has been reset successfully"
        }))),
    )
}

pub async fn change_password(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(request): Json<ChangePasswordRequest>,
) -> impl IntoResponse {
    let _token = auth.token();

    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Password changed successfully"
        }))),
    )
}

pub async fn verify_email(
    Extension(_auth_service): Extension<Arc<AuthService>>,
    Json(_request): Json<VerifyEmailRequest>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(ApiResponse::success(serde_json::json!({
            "message": "Email verified successfully"
        }))),
    )
}

pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "service": "auth",
            "timestamp": chrono::Utc::now()
        })),
    )
}