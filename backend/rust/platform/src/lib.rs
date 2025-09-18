pub mod models;
pub mod services;
pub mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::services::PlatformService;

pub fn create_router(platform_service: Arc<PlatformService>) -> Router {
    Router::new()
        // Tenant management
        .route("/tenants", get(handlers::list_tenants))
        .route("/tenants", post(handlers::create_tenant))
        .route("/tenants/:id", get(handlers::get_tenant))
        .route("/tenants/:id", put(handlers::update_tenant))
        .route("/tenants/:id", delete(handlers::delete_tenant))
        // User management
        .route("/users", get(handlers::list_users))
        .route("/users", post(handlers::create_user))
        .route("/users/:id", get(handlers::get_user))
        .route("/users/:id", put(handlers::update_user))
        .route("/users/:id", delete(handlers::delete_user))
        // Role management
        .route("/roles", get(handlers::list_roles))
        .route("/roles", post(handlers::create_role))
        .route("/roles/:id", get(handlers::get_role))
        .route("/roles/:id", put(handlers::update_role))
        .route("/roles/:id", delete(handlers::delete_role))
        // Permission management
        .route("/permissions", get(handlers::list_permissions))
        .route("/users/:user_id/roles", post(handlers::assign_role))
        .route("/users/:user_id/roles/:role_id", delete(handlers::remove_role))
        // Location management
        .route("/locations", get(handlers::list_locations))
        .route("/locations", post(handlers::create_location))
        .route("/locations/:id", get(handlers::get_location))
        .route("/locations/:id", put(handlers::update_location))
        .route("/locations/:id", delete(handlers::delete_location))
        // Settings
        .route("/settings", get(handlers::get_settings))
        .route("/settings", put(handlers::update_settings))
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(axum::Extension(platform_service)),
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