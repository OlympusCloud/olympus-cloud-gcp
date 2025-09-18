use std::sync::Arc;
use tokio;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();

    println!("Olympus Cloud Rust Services Starting...");

    // For now, just test that modules compile
    let _auth_router = olympus_auth::create_router(Arc::new(
        // This would be initialized with real database connection
        // For now, just testing compilation
        create_mock_auth_service().await?
    ));

    let _platform_router = olympus_platform::create_router();
    let _commerce_router = olympus_commerce::create_router();

    println!("All services initialized successfully!");
    
    Ok(())
}

async fn create_mock_auth_service() -> Result<olympus_auth::services::AuthService, Box<dyn std::error::Error>> {
    // This is a placeholder - in real implementation we'd connect to database
    let db = Arc::new(olympus_shared::database::Database::new("postgresql://localhost/test").await?);
    let jwt_secret = b"test-secret-key-for-development-only";
    
    Ok(olympus_auth::services::AuthService::new(db, jwt_secret, None))
}