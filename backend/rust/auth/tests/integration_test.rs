#[cfg(all(test, not(feature = "mock-queries")))]
mod database_tests {
    use std::sync::Arc;
    use olympus_auth::services::AuthService;
    use olympus_auth::models::{LoginRequest, RegisterRequest};
    use olympus_shared::database::Database;

    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_auth_flow_with_database() {
        let db = Arc::new(Database::new("postgresql://localhost/test").await.unwrap_or_else(|_| {
            panic!("Database connection failed")
        }));

        let jwt_secret = b"test-secret-key-for-integration-testing-only";
        let auth_service = AuthService::new(db, jwt_secret, None);

        let register_req = RegisterRequest {
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            phone: None,
            tenant_slug: "test-tenant".to_string(),
        };

        let user_response = auth_service.register(register_req).await.unwrap();
        assert_eq!(user_response.email, "test@example.com");

        let login_req = LoginRequest {
            email: "test@example.com".to_string(),
            password: "SecurePassword123!".to_string(),
            tenant_slug: "test-tenant".to_string(),
            device_id: None,
            device_name: None,
        };

        let token_response = auth_service.login(login_req, "127.0.0.1".to_string(), "test-agent".to_string()).await.unwrap();
        assert!(!token_response.access_token.is_empty());

        let claims = auth_service.verify_token(&token_response.access_token).await.unwrap();
        assert_eq!(claims.email, "test@example.com");
    }
}

#[test]
fn test_jwt_service() {
    use olympus_auth::services::jwt::JwtService;
    use olympus_auth::models::User;
    use uuid::Uuid;
    use chrono::Utc;

    let jwt_service = JwtService::new(b"test-secret-key-must-be-long-enough-for-hs256");
    
    let user = User {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
        password_hash: String::new(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        display_name: None,
        phone: None,
        avatar_url: None,
        roles: vec!["user".to_string()],
        permissions: vec![],
        is_active: true,
        email_verified: false,
        phone_verified: false,
        two_factor_enabled: false,
        last_login: None,
        failed_login_attempts: 0,
        locked_until: None,
        password_changed_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    let session_id = Uuid::new_v4();
    let token = jwt_service.generate_access_token(&user, user.tenant_id, session_id).unwrap();
    assert!(!token.is_empty());

    let claims = jwt_service.verify_access_token(&token).unwrap();
    assert_eq!(claims.sub, user.id);
    assert_eq!(claims.email, user.email);
}

#[test]
fn test_password_service() {
    use olympus_auth::services::password::PasswordService;

    let password_service = PasswordService::new();
    let password = "SecurePassword123!";

    let hash = password_service.hash_password(password).unwrap();
    assert!(!hash.is_empty());
    assert_ne!(hash, password);

    assert!(password_service.verify_password(password, &hash).unwrap());
    assert!(!password_service.verify_password("WrongPassword", &hash).unwrap());
}