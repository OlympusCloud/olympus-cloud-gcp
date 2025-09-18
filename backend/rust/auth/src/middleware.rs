use axum::{
    extract::Extension,
    http::{Request, StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::services::AuthService;

pub async fn auth_middleware(
    Extension(auth_service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let authorization = headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        });

    let token = match authorization {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    match auth_service.verify_token(token).await {
        Ok(_claims) => {
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}