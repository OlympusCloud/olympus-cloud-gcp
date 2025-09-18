use axum::{Router};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use olympus_shared::database::Database;
use olympus_shared::events::EventPublisher;

mod config;
use config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "olympus=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded");

    // Initialize database
    let database = Arc::new(Database::new(&config.database_url).await?);
    info!("Database connected");

    // Initialize Redis event publisher
    let event_publisher = match EventPublisher::new(&config.redis_url).await {
        Ok(publisher) => {
            info!("Redis event publisher connected");
            Some(Arc::new(tokio::sync::Mutex::new(publisher)))
        }
        Err(e) => {
            tracing::warn!("Failed to connect to Redis: {}", e);
            None
        }
    };

    // Initialize services
    let auth_service = Arc::new(olympus_auth::services::AuthService::new(
        database.clone(),
        config.jwt_secret.as_bytes(),
        event_publisher.clone(),
    ));

    // Create routers
    let auth_router = olympus_auth::create_router(auth_service);

    // Combine all routers
    let app = Router::new()
        .nest("/auth", auth_router)
        // Platform and commerce routers would be added here when services are implemented
        // .nest("/platform", platform_router)
        // .nest("/commerce", commerce_router)
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(CorsLayer::permissive()),
        )
        // Health check endpoint
        .route("/health", axum::routing::get(health_check));

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("🚀 Olympus Rust services starting on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "olympus-rust",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
    }))
}