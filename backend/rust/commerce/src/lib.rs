// ============================================================================
// OLYMPUS CLOUD - COMMERCE SERVICE
// ============================================================================
// Module: commerce/src/lib.rs
// Description: Commerce service for product catalog, orders, and payments
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

pub mod models;
pub mod services;
pub mod handlers;
pub mod simple_models;
pub mod simple_service;
pub mod simple_handlers;

#[cfg(test)]
pub mod tests;

use std::sync::Arc;
use axum::{
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use sqlx::PgPool;

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use crate::handlers::{create_product_router, create_order_router};
use crate::services::{CatalogService, OrderService};
use simple_service::SimpleCommerceService;
use simple_handlers::*;

/// Commerce service configuration
#[derive(Clone)]
pub struct CommerceConfig {
    pub db: Arc<DbPool>,
    pub event_publisher: Arc<EventPublisher>,
}

/// Create commerce router with all endpoints and middleware
pub fn create_router(config: CommerceConfig) -> Router {
    // Create services
    let catalog_service = Arc::new(CatalogService::new(
        config.db.clone(),
        config.event_publisher.clone(),
    ));

    let order_service = Arc::new(OrderService::new(
        config.db.clone(),
        config.event_publisher.clone(),
    ));

    Router::new()
        // Health check
        .route("/health", get(health_check))

        // Product catalog routes
        .nest("/api/v1/commerce", create_product_router(catalog_service.clone()))

        // Order management routes
        .nest("/api/v1/commerce", create_order_router(order_service.clone()))

        // Middleware stack
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

/// Create simple router for backward compatibility with demo
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

/// Create router for testing without dependencies
pub fn create_test_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        )
}

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "Commerce service healthy"
}

// Re-export important types
pub use handlers::*;
pub use models::*;
pub use services::*;