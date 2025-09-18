use olympus_platform::{create_router, models::*, services::PlatformService};
use olympus_shared::{database::Database, types::ApiResponse};
use axum::http::StatusCode;
use axum_test::TestServer;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_create_tenant() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    // Test tenant creation
    let create_request = CreateTenantRequest {
        slug: "test-company".to_string(),
        name: "Test Company".to_string(),
        industry: Some("Technology".to_string()),
        subscription_tier: SubscriptionTier::Starter,
        billing_email: "billing@testcompany.com".to_string(),
    };

    let response = server
        .post("/tenants")
        .json(&create_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: ApiResponse<Tenant> = response.json();
    assert!(body.data.is_some());

    let tenant = body.data.unwrap();
    assert_eq!(tenant.slug, "test-company");
    assert_eq!(tenant.name, "Test Company");
    assert_eq!(tenant.subscription_status, SubscriptionStatus::Trial);
}

#[tokio::test]
async fn test_list_tenants() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    // Create a few tenants first
    for i in 0..3 {
        let create_request = CreateTenantRequest {
            slug: format!("tenant-{}", i),
            name: format!("Tenant {}", i),
            industry: Some("Technology".to_string()),
            subscription_tier: SubscriptionTier::Free,
            billing_email: format!("tenant{}@example.com", i),
        };

        server
            .post("/tenants")
            .json(&create_request)
            .await;
    }

    // Test listing tenants
    let response = server
        .get("/tenants?page=1&per_page=10")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ApiResponse<Vec<Tenant>> = response.json();
    assert!(body.data.is_some());
    assert!(body.data.unwrap().len() >= 3);
}

#[tokio::test]
async fn test_get_tenant() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    // Create a tenant
    let create_request = CreateTenantRequest {
        slug: "get-test".to_string(),
        name: "Get Test Company".to_string(),
        industry: Some("Healthcare".to_string()),
        subscription_tier: SubscriptionTier::Professional,
        billing_email: "billing@gettest.com".to_string(),
    };

    let create_response = server
        .post("/tenants")
        .json(&create_request)
        .await;

    let create_body: ApiResponse<Tenant> = create_response.json();
    let tenant_id = create_body.data.unwrap().id;

    // Test getting the tenant
    let response = server
        .get(&format!("/tenants/{}", tenant_id))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ApiResponse<Tenant> = response.json();
    assert!(body.data.is_some());

    let tenant = body.data.unwrap();
    assert_eq!(tenant.id, tenant_id);
    assert_eq!(tenant.slug, "get-test");
}

#[tokio::test]
async fn test_update_tenant() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    // Create a tenant
    let create_request = CreateTenantRequest {
        slug: "update-test".to_string(),
        name: "Original Name".to_string(),
        industry: Some("Finance".to_string()),
        subscription_tier: SubscriptionTier::Free,
        billing_email: "old@updatetest.com".to_string(),
    };

    let create_response = server
        .post("/tenants")
        .json(&create_request)
        .await;

    let create_body: ApiResponse<Tenant> = create_response.json();
    let tenant_id = create_body.data.unwrap().id;

    // Update the tenant
    let update_request = UpdateTenantRequest {
        name: Some("Updated Name".to_string()),
        display_name: Some("Updated Display Name".to_string()),
        description: Some("Updated description".to_string()),
    };

    let response = server
        .put(&format!("/tenants/{}", tenant_id))
        .json(&update_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ApiResponse<Tenant> = response.json();
    assert!(body.data.is_some());

    let tenant = body.data.unwrap();
    assert_eq!(tenant.name, "Updated Name");
    assert_eq!(tenant.display_name, "Updated Display Name");
}

#[tokio::test]
async fn test_create_location() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    let tenant_id = Uuid::new_v4();

    // Create location
    let create_request = CreateLocationRequest {
        name: "Main Office".to_string(),
        code: "HQ001".to_string(),
        description: Some("Headquarters location".to_string()),
        address: Some(serde_json::json!({
            "street": "123 Main St",
            "city": "San Francisco",
            "state": "CA",
            "zip": "94105",
            "country": "USA"
        })),
        phone: Some(serde_json::json!({
            "main": "+1-415-555-0100",
            "fax": "+1-415-555-0101"
        })),
        email: Some("hq@company.com".to_string()),
        timezone: Some("America/Los_Angeles".to_string()),
        business_hours: Some(serde_json::json!({
            "monday": "09:00-18:00",
            "tuesday": "09:00-18:00",
            "wednesday": "09:00-18:00",
            "thursday": "09:00-18:00",
            "friday": "09:00-17:00"
        })),
    };

    let response = server
        .post(&format!("/locations?tenant_id={}", tenant_id))
        .json(&create_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: ApiResponse<Location> = response.json();
    assert!(body.data.is_some());

    let location = body.data.unwrap();
    assert_eq!(location.name, "Main Office");
    assert_eq!(location.code, "HQ001");
    assert_eq!(location.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_create_role() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    let tenant_id = Uuid::new_v4();

    // Create role
    let create_request = CreateRoleRequest {
        name: "manager".to_string(),
        display_name: "Manager".to_string(),
        description: Some("Manager role with elevated permissions".to_string()),
        permissions: vec![
            "users.read".to_string(),
            "users.write".to_string(),
            "orders.read".to_string(),
            "orders.write".to_string(),
            "reports.read".to_string(),
        ],
    };

    let response = server
        .post(&format!("/roles?tenant_id={}", tenant_id))
        .json(&create_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: ApiResponse<Role> = response.json();
    assert!(body.data.is_some());

    let role = body.data.unwrap();
    assert_eq!(role.name, "manager");
    assert_eq!(role.permissions.len(), 5);
    assert_eq!(role.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_list_roles() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    let tenant_id = Uuid::new_v4();

    // Create a few roles
    let roles = vec!["admin", "user", "viewer"];
    for role_name in roles {
        let create_request = CreateRoleRequest {
            name: role_name.to_string(),
            display_name: role_name.to_string(),
            description: Some(format!("{} role", role_name)),
            permissions: vec![format!("{}.read", role_name)],
        };

        server
            .post(&format!("/roles?tenant_id={}", tenant_id))
            .json(&create_request)
            .await;
    }

    // Test listing roles
    let response = server
        .get(&format!("/roles?tenant_id={}", tenant_id))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ApiResponse<Vec<Role>> = response.json();
    assert!(body.data.is_some());
    assert_eq!(body.data.unwrap().len(), 3);
}

#[tokio::test]
async fn test_list_permissions() {
    // Setup test service
    let db = Arc::new(Database::new("postgresql://test:test@localhost:5432/test_db"));
    let platform_service = Arc::new(PlatformService::new(db));

    // Create test server
    let app = create_router(platform_service);
    let server = TestServer::new(app).unwrap();

    // Test getting permissions list
    let response = server
        .get("/permissions")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ApiResponse<Vec<Permission>> = response.json();
    assert!(body.data.is_some());

    let permissions = body.data.unwrap();
    assert!(permissions.len() > 0);
    assert!(permissions.iter().any(|p| p.id == "users.read"));
}