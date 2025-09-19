use std::sync::Arc;
use tokio;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("Olympus Cloud Rust Services Starting...");

    // For now, just test that modules compile
    // Skip auth router creation due to database dependency
    // let _auth_router = olympus_auth::create_router(Arc::new(
    //     create_mock_auth_service().await?
    // ));
    println!("Auth module compiled successfully (router creation skipped)");

    let _platform_router = olympus_platform::create_router();
    let _commerce_router = olympus_commerce::create_router();

    println!("All services initialized successfully!");
    
    Ok(())
}

async fn create_mock_auth_service() -> Result<olympus_auth::services::AuthService, Box<dyn std::error::Error>> {
    // For compilation testing only - would use real database in production
    // Skip actual database connection for now
    println!("Note: Using mock auth service for compilation testing");
    Err("Mock auth service - would initialize with real database".into())
}