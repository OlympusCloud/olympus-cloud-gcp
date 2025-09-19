#[cfg(test)]
mod integration_tests {
    use olympus_auth::services::jwt::JwtService;
    use olympus_auth::services::password::PasswordService;
    use olympus_auth::models::User;
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_password_service_comprehensive() {
        let service = PasswordService::new();

        // Test valid password
        let valid_password = "ValidPass123!@#";
        let hash = service.hash_password(valid_password).unwrap();
        assert!(service.verify_password(valid_password, &hash).unwrap());
        assert!(!service.verify_password("WrongPass123!@#", &hash).unwrap());

        // Test password strength requirements
        assert!(service.hash_password("short").is_err()); // Too short
        assert!(service.hash_password("onlylowercase").is_err()); // Only lowercase, no digits/special
        assert!(service.hash_password("lowercasedigits123").is_err()); // Only 2 criteria

        // Test random password generation
        let random_pass = service.generate_random_password(20);
        assert_eq!(random_pass.len(), 20);
        assert!(service.hash_password(&random_pass).is_ok());
    }

    #[test]
    fn test_jwt_service_comprehensive() {
        let jwt_service = JwtService::new(b"test-secret-key-must-be-at-least-32-bytes-long");

        let user = create_test_user();
        let tenant_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();

        // Test access token generation and verification
        let access_token = jwt_service.generate_access_token(&user, tenant_id, session_id).unwrap();
        assert!(!access_token.is_empty());

        let claims = jwt_service.verify_access_token(&access_token).unwrap();
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.email, user.email);
        assert_eq!(claims.tenant_id, tenant_id);
        assert_eq!(claims.session_id, session_id);

        // Test refresh token generation (no parameters needed)
        let refresh_token = jwt_service.generate_refresh_token().unwrap();
        assert!(!refresh_token.is_empty());

        // Test special tokens
        let email_token = jwt_service.generate_email_verification_token(user.id, &user.email).unwrap();
        assert!(!email_token.is_empty());

        let password_reset_token = jwt_service.generate_password_reset_token(user.id, &user.email).unwrap();
        assert!(!password_reset_token.is_empty());

        // Test invalid token
        let invalid_token = "invalid.jwt.token";
        assert!(jwt_service.verify_access_token(invalid_token).is_err());
    }

    #[test]
    fn test_auth_models_validation() {
        use olympus_auth::models::{LoginRequest, RegisterRequest, RefreshTokenRequest};

        // Test LoginRequest
        let login = LoginRequest {
            email: "test@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            tenant_slug: "test-tenant".to_string(),
            device_id: Some("device-123".to_string()),
            device_name: Some("Test Device".to_string()),
        };
        assert_eq!(login.email, "test@example.com");

        // Test RegisterRequest
        let register = RegisterRequest {
            email: "newuser@example.com".to_string(),
            password: "SecurePass456!".to_string(),
            first_name: "New".to_string(),
            last_name: "User".to_string(),
            phone: Some("+1234567890".to_string()),
            tenant_slug: "new-tenant".to_string(),
        };
        assert_eq!(register.email, "newuser@example.com");

        // Test RefreshTokenRequest
        let refresh = RefreshTokenRequest {
            refresh_token: "some.refresh.token".to_string(),
        };
        assert!(!refresh.refresh_token.is_empty());
    }

    #[test]
    fn test_user_model_serialization() {
        use serde_json;

        let user = create_test_user();

        // Test serialization
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("test@example.com"));

        // Test deserialization
        let deserialized: User = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.id, user.id);
        assert_eq!(deserialized.email, user.email);
    }

    #[test]
    fn test_claims_expiration() {
        let jwt_service = JwtService::new(b"test-secret-key-must-be-at-least-32-bytes-long");
        let user = create_test_user();

        // Generate token with custom expiration
        let token = jwt_service.generate_access_token(
            &user,
            Uuid::new_v4(),
            Uuid::new_v4()
        ).unwrap();

        // Verify token is valid now
        let claims = jwt_service.verify_access_token(&token).unwrap();

        // Check expiration is set (1 hour from now)
        let now = chrono::Utc::now().timestamp();
        assert!(claims.exp > now);
        assert!(claims.exp <= now + 3600); // 1 hour = 3600 seconds
    }

    #[cfg(feature = "mock-queries")]
    #[test]
    fn test_mock_auth_service() {
        use std::sync::Arc;
        use olympus_shared::database::Database;

        // This test runs with mock queries enabled
        // It doesn't need a real database connection
        let runtime = tokio::runtime::Runtime::new().unwrap();

        runtime.block_on(async {
            // Mock database setup
            let db = Arc::new(Database::mock());
            let jwt_secret = b"test-secret-key-for-mock-testing";

            // Create auth service with mock database
            let auth_service = olympus_auth::services::AuthService::new(db, jwt_secret, None);

            // Verify service is created
            assert!(auth_service.jwt_service.is_some());
        });
    }

    // Helper function to create test user
    fn create_test_user() -> User {
        User {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: String::new(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            display_name: Some("Test User".to_string()),
            phone: Some("+1234567890".to_string()),
            avatar_url: None,
            roles: vec!["user".to_string(), "admin".to_string()],
            permissions: vec!["read".to_string(), "write".to_string()],
            is_active: true,
            email_verified: true,
            phone_verified: false,
            two_factor_enabled: false,
            last_login: Some(Utc::now()),
            failed_login_attempts: 0,
            locked_until: None,
            password_changed_at: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}