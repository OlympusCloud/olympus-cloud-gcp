// ============================================================================
// OLYMPUS CLOUD - PLATFORM SERVICE
// ============================================================================
// Module: platform/src/lib.rs
// Description: Platform service for configuration management, feature flags, and system settings
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

pub mod handlers;
pub mod models;
pub mod services;

use std::sync::Arc;
use axum::{
    routing::get,
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use crate::handlers::create_configuration_router;
use crate::services::{FeatureFlagsService, ConfigurationService};

/// Platform service configuration
#[derive(Clone)]
pub struct PlatformConfig {
    pub db: Arc<DbPool>,
    pub event_publisher: Arc<EventPublisher>,
}

/// Create platform router with all endpoints and middleware
pub fn create_router(config: PlatformConfig) -> Router {
    // Create services
    let feature_flags_service = Arc::new(FeatureFlagsService::new(
        config.db.clone(),
        config.event_publisher.clone(),
    ));

    let configuration_service = Arc::new(ConfigurationService::new(
        config.db.clone(),
        config.event_publisher.clone(),
    ));

    Router::new()
        // Health check
        .route("/health", get(health_check))

        // Configuration management routes (feature flags & system config)
        .nest("/api/v1/platform", create_configuration_router(
            feature_flags_service.clone(),
            configuration_service.clone(),
        ))

        // Middleware stack
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
    "Platform service healthy"
}

// Re-export important types
pub use handlers::*;
pub use models::*;
pub use services::*;