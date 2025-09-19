//! Comprehensive unit tests for the platform service

use olympus_platform::{
    models::{Tenant, Location, Role, Permission},
    services::{TenantService, LocationService, RoleService},
};
use olympus_shared::{
    database::DatabaseConnection,
    Result,
};
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;
use chrono::Utc;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_tenant_slug_generation() {
        let name = "My Restaurant Chain";
        let slug = olympus_platform::utils::generate_slug(name);
        assert_eq!(slug, "my-restaurant-chain");

        let name_with_special = "John's Café & Bar";
        let slug = olympus_platform::utils::generate_slug(name_with_special);
        assert_eq!(slug, "johns-cafe-bar");
    }

    #[tokio::test]
    async fn test_tenant_tier_validation() {
        use olympus_platform::models::TenantTier;

        assert!(TenantTier::is_valid("starter"));
        assert!(TenantTier::is_valid("growth"));
        assert!(TenantTier::is_valid("premium"));
        assert!(TenantTier::is_valid("enterprise"));
        assert!(!TenantTier::is_valid("invalid"));
    }

    #[tokio::test]
    async fn test_industry_validation() {
        use olympus_platform::models::Industry;

        assert!(Industry::is_valid("restaurant"));
        assert!(Industry::is_valid("retail"));
        assert!(Industry::is_valid("hospitality"));
        assert!(Industry::is_valid("events"));
        assert!(Industry::is_valid("salon"));
        assert!(!Industry::is_valid("invalid"));
    }

    #[tokio::test]
    async fn test_feature_flags() {
        use olympus_platform::models::FeatureFlags;

        let mut features = FeatureFlags::default();

        assert!(!features.has_feature("pos"));
        features.enable_feature("pos");
        assert!(features.has_feature("pos"));

        features.disable_feature("pos");
        assert!(!features.has_feature("pos"));

        // Test tier-based features
        let starter_features = FeatureFlags::for_tier("starter");
        assert!(starter_features.has_feature("basic_reporting"));
        assert!(!starter_features.has_feature("advanced_analytics"));

        let enterprise_features = FeatureFlags::for_tier("enterprise");
        assert!(enterprise_features.has_feature("basic_reporting"));
        assert!(enterprise_features.has_feature("advanced_analytics"));
        assert!(enterprise_features.has_feature("white_label"));
    }

    #[tokio::test]
    async fn test_timezone_validation() {
        use olympus_platform::utils::validate_timezone;

        assert!(validate_timezone("America/New_York").is_ok());
        assert!(validate_timezone("Europe/London").is_ok());
        assert!(validate_timezone("Asia/Tokyo").is_ok());
        assert!(validate_timezone("Invalid/Timezone").is_err());
    }

    #[tokio::test]
    async fn test_permission_matching() {
        use olympus_platform::models::Permission;

        let permission = Permission {
            resource: "orders".to_string(),
            action: "read".to_string(),
        };

        assert!(permission.matches("orders", "read"));
        assert!(!permission.matches("orders", "write"));
        assert!(!permission.matches("products", "read"));

        let wildcard_permission = Permission {
            resource: "orders".to_string(),
            action: "*".to_string(),
        };

        assert!(wildcard_permission.matches("orders", "read"));
        assert!(wildcard_permission.matches("orders", "write"));
        assert!(wildcard_permission.matches("orders", "delete"));
        assert!(!wildcard_permission.matches("products", "read"));
    }

    #[tokio::test]
    async fn test_role_permission_aggregation() {
        use olympus_platform::models::{Role, Permission};

        let role = Role {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "manager".to_string(),
            permissions: vec![
                Permission { resource: "orders".to_string(), action: "*".to_string() },
                Permission { resource: "products".to_string(), action: "read".to_string() },
                Permission { resource: "reports".to_string(), action: "read".to_string() },
            ],
            is_system: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(role.has_permission("orders", "read"));
        assert!(role.has_permission("orders", "write"));
        assert!(role.has_permission("products", "read"));
        assert!(!role.has_permission("products", "write"));
        assert!(role.has_permission("reports", "read"));
        assert!(!role.has_permission("users", "read"));
    }

    #[tokio::test]
    async fn test_location_validation() {
        use olympus_platform::models::Location;

        let location = Location {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Store".to_string(),
            address: "123 Main St".to_string(),
            city: "New York".to_string(),
            state: "NY".to_string(),
            country: "USA".to_string(),
            postal_code: "10001".to_string(),
            timezone: "America/New_York".to_string(),
            is_primary: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(location.validate().is_ok());

        let invalid_location = Location {
            postal_code: "".to_string(),
            timezone: "Invalid/TZ".to_string(),
            ..location
        };

        assert!(invalid_location.validate().is_err());
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
    async fn test_tenant_creation() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());

        let result = tenant_service.create_tenant(
            "Test Restaurant",
            "restaurant",
            "starter",
        ).await;

        assert!(result.is_ok());

        let tenant = result.unwrap();
        assert_eq!(tenant.name, "Test Restaurant");
        assert_eq!(tenant.slug, "test-restaurant");
        assert_eq!(tenant.industry, "restaurant");
        assert_eq!(tenant.tier, "starter");
    }

    #[tokio::test]
    async fn test_tenant_slug_uniqueness() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());

        // Create first tenant
        let result1 = tenant_service.create_tenant(
            "Unique Store",
            "retail",
            "growth",
        ).await;
        assert!(result1.is_ok());

        // Try to create second tenant with same name (should get different slug)
        let result2 = tenant_service.create_tenant(
            "Unique Store",
            "retail",
            "growth",
        ).await;
        assert!(result2.is_ok());

        let tenant2 = result2.unwrap();
        assert_ne!(tenant2.slug, "unique-store");
        assert!(tenant2.slug.starts_with("unique-store-"));
    }

    #[tokio::test]
    async fn test_tenant_update() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());

        let tenant = tenant_service.create_tenant(
            "Update Test",
            "hospitality",
            "starter",
        ).await.unwrap();

        let updated = tenant_service.update_tenant(
            tenant.id,
            Some("Updated Name"),
            None,
            Some("growth"),
        ).await.unwrap();

        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.tier, "growth");
        assert_eq!(updated.industry, "hospitality"); // Unchanged
    }

    #[tokio::test]
    async fn test_location_management() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());
        let location_service = LocationService::new(pool.clone());

        let tenant = tenant_service.create_tenant(
            "Multi-Location Business",
            "retail",
            "premium",
        ).await.unwrap();

        // Create primary location
        let location1 = location_service.create_location(
            tenant.id,
            "Headquarters",
            "100 Main St",
            "New York",
            "NY",
            "USA",
            "10001",
            "America/New_York",
            true,
        ).await.unwrap();

        assert!(location1.is_primary);

        // Create second location
        let location2 = location_service.create_location(
            tenant.id,
            "West Coast Office",
            "200 Market St",
            "San Francisco",
            "CA",
            "USA",
            "94102",
            "America/Los_Angeles",
            false,
        ).await.unwrap();

        assert!(!location2.is_primary);

        // List locations
        let locations = location_service.list_tenant_locations(tenant.id).await.unwrap();
        assert_eq!(locations.len(), 2);
    }

    #[tokio::test]
    async fn test_primary_location_switching() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());
        let location_service = LocationService::new(pool.clone());

        let tenant = tenant_service.create_tenant(
            "Switch Primary Test",
            "restaurant",
            "growth",
        ).await.unwrap();

        let location1 = location_service.create_location(
            tenant.id,
            "Original Primary",
            "100 First Ave",
            "Chicago",
            "IL",
            "USA",
            "60601",
            "America/Chicago",
            true,
        ).await.unwrap();

        let location2 = location_service.create_location(
            tenant.id,
            "New Primary",
            "200 Second St",
            "Chicago",
            "IL",
            "USA",
            "60602",
            "America/Chicago",
            false,
        ).await.unwrap();

        // Make location2 primary
        location_service.set_primary_location(tenant.id, location2.id).await.unwrap();

        // Check that location1 is no longer primary
        let updated_location1 = location_service.get_location(location1.id).await.unwrap();
        assert!(!updated_location1.is_primary);

        let updated_location2 = location_service.get_location(location2.id).await.unwrap();
        assert!(updated_location2.is_primary);
    }

    #[tokio::test]
    async fn test_role_creation_and_assignment() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());
        let role_service = RoleService::new(pool.clone());

        let tenant = tenant_service.create_tenant(
            "Role Test Company",
            "events",
            "premium",
        ).await.unwrap();

        // Create custom role
        let role = role_service.create_role(
            tenant.id,
            "event_coordinator",
            vec![
                ("events", "*"),
                ("venues", "read"),
                ("reports", "read"),
            ],
        ).await.unwrap();

        assert_eq!(role.name, "event_coordinator");
        assert_eq!(role.permissions.len(), 3);

        // List roles for tenant
        let roles = role_service.list_tenant_roles(tenant.id).await.unwrap();
        assert!(roles.len() >= 1); // May include system roles
    }

    #[tokio::test]
    async fn test_system_roles() {
        let pool = setup_test_db().await;
        let role_service = RoleService::new(pool.clone());

        // Ensure system roles exist
        role_service.ensure_system_roles().await.unwrap();

        let system_roles = role_service.get_system_roles().await.unwrap();

        let role_names: Vec<String> = system_roles.iter()
            .map(|r| r.name.clone())
            .collect();

        assert!(role_names.contains(&"admin".to_string()));
        assert!(role_names.contains(&"manager".to_string()));
        assert!(role_names.contains(&"staff".to_string()));
        assert!(role_names.contains(&"viewer".to_string()));
    }

    #[tokio::test]
    async fn test_tenant_deletion_cascading() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());
        let location_service = LocationService::new(pool.clone());
        let role_service = RoleService::new(pool.clone());

        let tenant = tenant_service.create_tenant(
            "Delete Test",
            "salon",
            "starter",
        ).await.unwrap();

        // Create related data
        let location = location_service.create_location(
            tenant.id,
            "Test Location",
            "123 Test St",
            "Test City",
            "TS",
            "USA",
            "12345",
            "America/New_York",
            true,
        ).await.unwrap();

        let role = role_service.create_role(
            tenant.id,
            "test_role",
            vec![("test", "read")],
        ).await.unwrap();

        // Delete tenant
        tenant_service.delete_tenant(tenant.id).await.unwrap();

        // Verify cascading deletion
        assert!(location_service.get_location(location.id).await.is_err());
        assert!(role_service.get_role(role.id).await.is_err());
    }

    #[tokio::test]
    async fn test_tenant_settings_and_branding() {
        let pool = setup_test_db().await;
        let tenant_service = TenantService::new(pool.clone());

        let tenant = tenant_service.create_tenant(
            "Branded Business",
            "restaurant",
            "premium",
        ).await.unwrap();

        let settings = json!({
            "currency": "USD",
            "date_format": "MM/DD/YYYY",
            "language": "en",
            "tax_rate": 0.08
        });

        let branding = json!({
            "primary_color": "#FF5733",
            "logo_url": "https://example.com/logo.png",
            "font_family": "Roboto"
        });

        let updated = tenant_service.update_tenant_settings(
            tenant.id,
            settings,
            branding,
        ).await.unwrap();

        assert_eq!(updated.settings["currency"], "USD");
        assert_eq!(updated.branding["primary_color"], "#FF5733");
    }
}