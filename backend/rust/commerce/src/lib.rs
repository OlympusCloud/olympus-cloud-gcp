pub mod simple_models;
pub mod simple_service;
pub mod simple_handlers;

use std::sync::Arc;
use axum::{
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use sqlx::PgPool;

use simple_service::SimpleCommerceService;
use simple_handlers::*;

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

pub fn create_simple_router(db: Arc<PgPool>) -> Router {
    let service = Arc::new(SimpleCommerceService::new(db));

    Router::new()
        // Health check
        .route("/health", get(health_check))
        // Product routes
        .route("/tenants/:tenant_id/products", post(create_product))
        .route("/tenants/:tenant_id/products", get(list_products))
        .route("/tenants/:tenant_id/products/:product_id", get(get_product))
        .route("/tenants/:tenant_id/products/:product_id", put(update_product))
        // Order routes
        .route("/tenants/:tenant_id/orders", post(create_order))
        .route("/tenants/:tenant_id/orders", get(list_orders))
        .route("/tenants/:tenant_id/orders/:order_id", get(get_order))
        .route("/tenants/:tenant_id/orders/:order_id", put(update_order))
        // Add service state
        .with_state(service)
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