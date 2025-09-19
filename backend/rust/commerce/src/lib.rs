pub mod handlers;
pub mod models;
pub mod services;

use axum::{
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

async fn health_check() -> &'static str {
    "Commerce service healthy"
}