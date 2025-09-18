pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::services::AuthService;

pub fn create_router(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        // Public routes
        .route("/auth/login", post(handlers::login))
        .route("/auth/register", post(handlers::register))
        .route("/auth/refresh", post(handlers::refresh_token))
        .route("/auth/forgot-password", post(handlers::forgot_password))
        .route("/auth/reset-password", post(handlers::reset_password))
        .route("/auth/verify-email", post(handlers::verify_email))
        // Protected routes
        .route("/auth/me", get(handlers::get_current_user))
        .route("/auth/logout", post(handlers::logout))
        .route("/auth/change-password", post(handlers::change_password))
        // Health check
        .route("/health", get(handlers::health_check))
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(axum::Extension(auth_service)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_compilation() {
        assert!(true);
    }
}