use olympus_auth::{create_router, models::*, services::{AuthService, JwtService, PasswordService}};
use olympus_shared::{database::Database, events::EventPublisher, types::ApiResponse};
use axum::http::StatusCode;
use axum_test::TestServer;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_register_user() {
    // Setup test services
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let jwt_service = Arc::new(JwtService::new("test_secret".to_string()));
    let password_service = Arc::new(PasswordService::new());
    let event_publisher = Arc::new(EventPublisher::new("redis://localhost:6379").await.unwrap());
    let auth_service = Arc::new(AuthService::new(db, jwt_service, password_service, event_publisher));

    // Create test server
    let app = create_router(auth_service);
    let server = TestServer::new(app).unwrap();

    // Test registration
    let register_request = RegisterRequest {
        tenant_id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        password: "SecurePassword123!".to_string(),
        first_name: Some("Test".to_string()),
        last_name: Some("User".to_string()),
    };

    let response = server
        .post("/auth/register")
        .json(&register_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: ApiResponse<AuthResponse> = response.json();
    assert!(body.data.is_some());
    assert!(body.data.unwrap().access_token.len() > 0);
}

#[tokio::test]
async fn test_login_user() {
    // Setup test services
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let jwt_service = Arc::new(JwtService::new("test_secret".to_string()));
    let password_service = Arc::new(PasswordService::new());
    let event_publisher = Arc::new(EventPublisher::new("redis://localhost:6379").await.unwrap());
    let auth_service = Arc::new(AuthService::new(db.clone(), jwt_service.clone(), password_service.clone(), event_publisher.clone()));

    // Create test server
    let app = create_router(auth_service.clone());
    let server = TestServer::new(app).unwrap();

    let tenant_id = Uuid::new_v4();
    let email = "login_test@example.com".to_string();
    let password = "TestPassword123!".to_string();

    // First register the user
    let register_request = RegisterRequest {
        tenant_id,
        email: email.clone(),
        password: password.clone(),
        first_name: Some("Login".to_string()),
        last_name: Some("Test".to_string()),
    };

    server
        .post("/auth/register")
        .json(&register_request)
        .await;

    // Now test login
    let login_request = LoginRequest {
        tenant_id,
        email: email.clone(),
        password: password.clone(),
    };

    let response = server
        .post("/auth/login")
        .json(&login_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ApiResponse<AuthResponse> = response.json();
    assert!(body.data.is_some());

    let auth_response = body.data.unwrap();
    assert!(auth_response.access_token.len() > 0);
    assert!(auth_response.refresh_token.len() > 0);
    assert_eq!(auth_response.user.email, email);
}

#[tokio::test]
async fn test_refresh_token() {
    // Setup test services
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let jwt_service = Arc::new(JwtService::new("test_secret".to_string()));
    let password_service = Arc::new(PasswordService::new());
    let event_publisher = Arc::new(EventPublisher::new("redis://localhost:6379").await.unwrap());
    let auth_service = Arc::new(AuthService::new(db.clone(), jwt_service.clone(), password_service.clone(), event_publisher.clone()));

    // Create test server
    let app = create_router(auth_service.clone());
    let server = TestServer::new(app).unwrap();

    let tenant_id = Uuid::new_v4();

    // Register and login to get tokens
    let register_request = RegisterRequest {
        tenant_id,
        email: "refresh_test@example.com".to_string(),
        password: "RefreshPassword123!".to_string(),
        first_name: Some("Refresh".to_string()),
        last_name: Some("Test".to_string()),
    };

    let register_response = server
        .post("/auth/register")
        .json(&register_request)
        .await;

    let body: ApiResponse<AuthResponse> = register_response.json();
    let refresh_token = body.data.unwrap().refresh_token;

    // Test refresh
    let refresh_request = RefreshTokenRequest {
        refresh_token: refresh_token.clone(),
    };

    let response = server
        .post("/auth/refresh")
        .json(&refresh_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let refresh_body: ApiResponse<TokenResponse> = response.json();
    assert!(refresh_body.data.is_some());
    assert!(refresh_body.data.unwrap().access_token.len() > 0);
}

#[tokio::test]
async fn test_invalid_credentials() {
    // Setup test services
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let jwt_service = Arc::new(JwtService::new("test_secret".to_string()));
    let password_service = Arc::new(PasswordService::new());
    let event_publisher = Arc::new(EventPublisher::new("redis://localhost:6379").await.unwrap());
    let auth_service = Arc::new(AuthService::new(db, jwt_service, password_service, event_publisher));

    // Create test server
    let app = create_router(auth_service);
    let server = TestServer::new(app).unwrap();

    // Test login with invalid credentials
    let login_request = LoginRequest {
        tenant_id: Uuid::new_v4(),
        email: "nonexistent@example.com".to_string(),
        password: "WrongPassword".to_string(),
    };

    let response = server
        .post("/auth/login")
        .json(&login_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_duplicate_email_registration() {
    // Setup test services
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let jwt_service = Arc::new(JwtService::new("test_secret".to_string()));
    let password_service = Arc::new(PasswordService::new());
    let event_publisher = Arc::new(EventPublisher::new("redis://localhost:6379").await.unwrap());
    let auth_service = Arc::new(AuthService::new(db, jwt_service, password_service, event_publisher));

    // Create test server
    let app = create_router(auth_service);
    let server = TestServer::new(app).unwrap();

    let tenant_id = Uuid::new_v4();
    let email = "duplicate@example.com".to_string();

    // First registration
    let register_request = RegisterRequest {
        tenant_id,
        email: email.clone(),
        password: "Password123!".to_string(),
        first_name: Some("First".to_string()),
        last_name: Some("User".to_string()),
    };

    server
        .post("/auth/register")
        .json(&register_request)
        .await;

    // Try to register again with same email
    let duplicate_request = RegisterRequest {
        tenant_id,
        email: email.clone(),
        password: "DifferentPassword123!".to_string(),
        first_name: Some("Second".to_string()),
        last_name: Some("User".to_string()),
    };

    let response = server
        .post("/auth/register")
        .json(&duplicate_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CONFLICT);
}