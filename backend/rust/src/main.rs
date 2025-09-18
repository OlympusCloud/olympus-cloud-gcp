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
mod health;
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
    let jwt_service = Arc::new(olympus_auth::services::JwtService::new(config.jwt_secret.clone()));
    let password_service = Arc::new(olympus_auth::services::PasswordService::new());
    let auth_service = Arc::new(olympus_auth::services::AuthService::new(
        database.clone(),
        jwt_service,
        password_service,
        event_publisher.clone().unwrap_or(Arc::new(tokio::sync::Mutex::new(EventPublisher::mock()))),
    ));

    let platform_service = Arc::new(olympus_platform::services::PlatformService::new(
        database.clone(),
    ));

    let commerce_service = Arc::new(olympus_commerce::services::CommerceService::new(
        database.clone(),
        event_publisher.clone().unwrap_or(Arc::new(tokio::sync::Mutex::new(EventPublisher::mock()))),
    ));

    // Create routers
    let auth_router = olympus_auth::create_router(auth_service);
    let platform_router = olympus_platform::create_router(platform_service);
    let commerce_router = olympus_commerce::create_router(commerce_service);

    // Initialize health monitoring
    health::init_health_monitoring();

    // Combine all routers
    let app = Router::new()
        .nest("/api/v1/auth", auth_router)
        .nest("/api/v1/platform", platform_router)
        .nest("/api/v1/commerce", commerce_router)
        // Health monitoring endpoints
        .route("/health", axum::routing::get(health::health_check))
        .route("/ready", axum::routing::get(health::readiness_check))
        .route("/live", axum::routing::get(health::liveness_check))
        .route("/metrics", axum::routing::get(health::metrics_handler))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .on_request(DefaultOnRequest::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(CorsLayer::permissive())
                .layer(axum::Extension(database.clone()))
                .layer(axum::Extension(event_publisher.clone())),
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("🚀 Olympus Rust services starting on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}