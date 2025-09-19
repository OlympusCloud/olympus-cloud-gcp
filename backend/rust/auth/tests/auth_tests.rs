//! Comprehensive unit tests for the auth service

use olympus_auth::{
    handlers::*,
    models::{LoginRequest, RegisterRequest, RefreshRequest},
    services::{AuthService, TokenService},
    utils::password,
};
use olympus_shared::{
    database::DatabaseConnection,
    models::User,
    Result,
};
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    Router,
};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tower::ServiceExt;
use uuid::Uuid;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_password_hashing() {
        let password = "Test123!@#";
        let hash = password::hash(password).unwrap();

        assert!(password::verify(password, &hash).unwrap());
        assert!(!password::verify("WrongPassword", &hash).unwrap());
        assert_ne!(hash, password);
        assert!(hash.starts_with("$argon2"));
    }

    #[tokio::test]
    async fn test_password_complexity_validation() {
        // Valid passwords
        assert!(password::validate_complexity("Test123!@#").is_ok());
        assert!(password::validate_complexity("SecureP@ss1").is_ok());

        // Invalid passwords
        assert!(password::validate_complexity("short").is_err());
        assert!(password::validate_complexity("NoNumbers!@#").is_err());
        assert!(password::validate_complexity("nouppercase123!").is_err());
        assert!(password::validate_complexity("NOLOWERCASE123!").is_err());
        assert!(password::validate_complexity("NoSpecialChars123").is_err());
    }

    #[tokio::test]
    async fn test_jwt_token_generation() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let roles = vec!["admin".to_string()];

        let token_service = TokenService::new("test_secret_key_for_testing_only");

        let access_token = token_service.generate_access_token(
            user_id,
            tenant_id,
            &roles,
        ).unwrap();

        let refresh_token = token_service.generate_refresh_token(
            user_id,
            tenant_id,
        ).unwrap();

        assert!(!access_token.is_empty());
        assert!(!refresh_token.is_empty());
        assert_ne!(access_token, refresh_token);
    }

    #[tokio::test]
    async fn test_jwt_token_validation() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let roles = vec!["user".to_string()];

        let token_service = TokenService::new("test_secret_key_for_testing_only");

        let token = token_service.generate_access_token(
            user_id,
            tenant_id,
            &roles,
        ).unwrap();

        let claims = token_service.validate_access_token(&token).unwrap();

        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.tenant_id, tenant_id);
        assert_eq!(claims.roles, roles);
    }

    #[tokio::test]
    async fn test_jwt_token_expiration() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let roles = vec!["user".to_string()];

        let token_service = TokenService::new("test_secret_key_for_testing_only");

        // Create an expired token (with custom exp claim in the past)
        let expired_token = token_service.generate_expired_token_for_testing(
            user_id,
            tenant_id,
            &roles,
        ).unwrap();

        let validation_result = token_service.validate_access_token(&expired_token);
        assert!(validation_result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_jwt_signature() {
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let roles = vec!["user".to_string()];

        let token_service1 = TokenService::new("secret_key_1");
        let token_service2 = TokenService::new("secret_key_2");

        let token = token_service1.generate_access_token(
            user_id,
            tenant_id,
            &roles,
        ).unwrap();

        // Try to validate with different secret
        let validation_result = token_service2.validate_access_token(&token);
        assert!(validation_result.is_err());
    }

    #[tokio::test]
    async fn test_email_validation() {
        use olympus_auth::utils::validation::validate_email;

        // Valid emails
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("test.user@company.co.uk").is_ok());
        assert!(validate_email("user+tag@example.com").is_ok());

        // Invalid emails
        assert!(validate_email("notanemail").is_err());
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user @example.com").is_err());
        assert!(validate_email("").is_err());
    }

    #[tokio::test]
    async fn test_session_management() {
        use olympus_auth::models::Session;

        let session = Session {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            refresh_token: "test_refresh_token".to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        assert!(!session.is_expired());

        let expired_session = Session {
            expires_at: chrono::Utc::now() - chrono::Duration::hours(1),
            ..session
        };

        assert!(expired_session.is_expired());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::{clients::Cli, images::postgres::Postgres};

    async fn setup_test_db() -> PgPool {
        let docker = Cli::default();
        let postgres_image = Postgres::default();
        let node = docker.run(postgres_image);

        let connection_string = format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            node.get_host_port_ipv4(5432)
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .unwrap();

        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_user_registration_flow() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let request = RegisterRequest {
            email: "newuser@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            tenant_id,
        };

        let result = auth_service.register(request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.email, "newuser@example.com");
        assert_eq!(response.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_duplicate_email_registration() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let request = RegisterRequest {
            email: "duplicate@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            tenant_id,
        };

        // First registration should succeed
        let result1 = auth_service.register(request.clone()).await;
        assert!(result1.is_ok());

        // Second registration with same email should fail
        let result2 = auth_service.register(request).await;
        assert!(result2.is_err());
    }

    #[tokio::test]
    async fn test_login_flow() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();

        // First register a user
        let register_request = RegisterRequest {
            email: "logintest@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            tenant_id,
        };

        auth_service.register(register_request).await.unwrap();

        // Now try to login
        let login_request = LoginRequest {
            email: "logintest@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            tenant_id: Some(tenant_id),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.access_token.is_empty());
        assert!(!response.refresh_token.is_empty());
        assert_eq!(response.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_login_with_wrong_password() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();

        // First register a user
        let register_request = RegisterRequest {
            email: "wrongpass@example.com".to_string(),
            password: "CorrectPass123!".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            tenant_id,
        };

        auth_service.register(register_request).await.unwrap();

        // Try to login with wrong password
        let login_request = LoginRequest {
            email: "wrongpass@example.com".to_string(),
            password: "WrongPass123!".to_string(),
            tenant_id: Some(tenant_id),
        };

        let result = auth_service.login(login_request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_refresh_token_flow() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();

        // Register and login
        let register_request = RegisterRequest {
            email: "refresh@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "Refresh".to_string(),
            last_name: "Test".to_string(),
            tenant_id,
        };

        auth_service.register(register_request).await.unwrap();

        let login_request = LoginRequest {
            email: "refresh@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            tenant_id: Some(tenant_id),
        };

        let login_response = auth_service.login(login_request).await.unwrap();

        // Use refresh token to get new access token
        let refresh_request = RefreshRequest {
            refresh_token: login_response.refresh_token,
        };

        let result = auth_service.refresh_token(refresh_request).await;
        assert!(result.is_ok());

        let refresh_response = result.unwrap();
        assert!(!refresh_response.access_token.is_empty());
        assert_ne!(refresh_response.access_token, login_response.access_token);
    }

    #[tokio::test]
    async fn test_logout_flow() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();

        // Register and login
        let register_request = RegisterRequest {
            email: "logout@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            first_name: "Logout".to_string(),
            last_name: "Test".to_string(),
            tenant_id,
        };

        let user = auth_service.register(register_request).await.unwrap();

        let login_request = LoginRequest {
            email: "logout@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            tenant_id: Some(tenant_id),
        };

        let login_response = auth_service.login(login_request).await.unwrap();

        // Logout
        let result = auth_service.logout(user.user_id).await;
        assert!(result.is_ok());

        // Try to use refresh token after logout (should fail)
        let refresh_request = RefreshRequest {
            refresh_token: login_response.refresh_token,
        };

        let refresh_result = auth_service.refresh_token(refresh_request).await;
        assert!(refresh_result.is_err());
    }

    #[tokio::test]
    async fn test_role_based_access() {
        let pool = setup_test_db().await;
        let auth_service = AuthService::new(pool.clone());

        let tenant_id = Uuid::new_v4();

        // Register admin user
        let admin_request = RegisterRequest {
            email: "admin@example.com".to_string(),
            password: "AdminPass123!".to_string(),
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            tenant_id,
        };

        let admin_user = auth_service.register(admin_request).await.unwrap();

        // Set admin role
        auth_service.set_user_roles(
            admin_user.user_id,
            vec!["admin".to_string(), "user".to_string()],
        ).await.unwrap();

        // Login as admin
        let login_request = LoginRequest {
            email: "admin@example.com".to_string(),
            password: "AdminPass123!".to_string(),
            tenant_id: Some(tenant_id),
        };

        let login_response = auth_service.login(login_request).await.unwrap();

        // Validate token contains roles
        let token_service = TokenService::new("test_secret");
        let claims = token_service.validate_access_token(&login_response.access_token).unwrap();

        assert!(claims.roles.contains(&"admin".to_string()));
        assert!(claims.roles.contains(&"user".to_string()));
    }
}

#[cfg(test)]
mod api_tests {
    use super::*;
    use axum::routing::post;
    use tower::util::ServiceExt;

    async fn create_test_app() -> Router {
        let pool = setup_test_db().await;

        Router::new()
            .route("/auth/register", post(register_handler))
            .route("/auth/login", post(login_handler))
            .route("/auth/refresh", post(refresh_handler))
            .route("/auth/logout", post(logout_handler))
            .with_state(pool)
    }

    #[tokio::test]
    async fn test_register_endpoint() {
        let app = create_test_app().await;

        let request_body = json!({
            "email": "api_test@example.com",
            "password": "ApiTest123!",
            "first_name": "API",
            "last_name": "Test",
            "tenant_id": Uuid::new_v4()
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&request_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_login_endpoint() {
        let app = create_test_app().await;
        let tenant_id = Uuid::new_v4();

        // First register
        let register_body = json!({
            "email": "login_api@example.com",
            "password": "LoginApi123!",
            "first_name": "Login",
            "last_name": "API",
            "tenant_id": tenant_id
        });

        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&register_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Then login
        let login_body = json!({
            "email": "login_api@example.com",
            "password": "LoginApi123!",
            "tenant_id": tenant_id
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/login")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&login_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_invalid_request_format() {
        let app = create_test_app().await;

        let invalid_body = json!({
            "wrong_field": "value"
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/auth/register")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&invalid_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}