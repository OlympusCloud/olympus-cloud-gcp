use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::Database;
use olympus_shared::error::Result;
use crate::models::*;

pub struct PlatformService {
    db: Arc<Database>,
}

impl PlatformService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    // Tenant management
    pub async fn create_tenant(&self, request: CreateTenantRequest) -> Result<Tenant> {
        let pool = self.db.pool();

        let tenant = Tenant {
            id: Uuid::new_v4(),
            slug: request.slug,
            name: request.name.clone(),
            display_name: request.name.clone(),
            description: None,
            industry: request.industry,
            subscription_tier: request.subscription_tier,
            subscription_status: SubscriptionStatus::Trial,
            trial_ends_at: Some(Utc::now() + chrono::Duration::days(14)),
            billing_email: request.billing_email,
            support_email: None,
            website: None,
            logo_url: None,
            settings: TenantSettings {
                timezone: "UTC".to_string(),
                currency: olympus_shared::types::Currency::USD,
                language: "en".to_string(),
                date_format: "MM/DD/YYYY".to_string(),
                time_format: "12h".to_string(),
                week_starts_on: 0,
                fiscal_year_start: 1,
                tax_rate: 0.0,
                multi_location: false,
                multi_currency: false,
            },
            features: vec![],
            is_active: true,
            user_limit: Some(10),
            location_limit: Some(1),
            storage_limit_gb: Some(5),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        // Insert tenant into database
        let result = sqlx::query_as!(
            Tenant,
            r#"
            INSERT INTO tenants (
                id, slug, name, display_name, industry, subscription_tier,
                subscription_status, trial_ends_at, billing_email, settings,
                features, is_active, user_limit, location_limit, storage_limit_gb,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )
            RETURNING *
            "#,
            tenant.id,
            tenant.slug,
            tenant.name,
            tenant.display_name,
            serde_json::to_value(&tenant.industry)?,
            serde_json::to_value(&tenant.subscription_tier)?,
            serde_json::to_value(&tenant.subscription_status)?,
            tenant.trial_ends_at,
            tenant.billing_email,
            serde_json::to_value(&tenant.settings)?,
            &tenant.features,
            tenant.is_active,
            tenant.user_limit,
            tenant.location_limit,
            tenant.storage_limit_gb,
            tenant.created_at,
            tenant.updated_at
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_tenant(&self, tenant_id: Uuid) -> Result<Tenant> {
        let pool = self.db.pool();

        let tenant = sqlx::query_as!(
            Tenant,
            r#"
            SELECT * FROM tenants
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            tenant_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| olympus_shared::error::Error::NotFound("Tenant not found".to_string()))?;

        Ok(tenant)
    }

    pub async fn list_tenants(&self, limit: i64, offset: i64) -> Result<Vec<Tenant>> {
        let pool = self.db.pool();

        let tenants = sqlx::query_as!(
            Tenant,
            r#"
            SELECT * FROM tenants
            WHERE deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(tenants)
    }

    pub async fn update_tenant(&self, tenant_id: Uuid, request: UpdateTenantRequest) -> Result<Tenant> {
        let pool = self.db.pool();

        // Build dynamic update query based on provided fields
        let mut query = String::from("UPDATE tenants SET updated_at = $1");
        let mut param_count = 1;

        if request.name.is_some() {
            param_count += 1;
            query.push_str(&format!(", name = ${}", param_count));
        }

        if request.display_name.is_some() {
            param_count += 1;
            query.push_str(&format!(", display_name = ${}", param_count));
        }

        if request.description.is_some() {
            param_count += 1;
            query.push_str(&format!(", description = ${}", param_count));
        }

        query.push_str(&format!(" WHERE id = ${} AND deleted_at IS NULL RETURNING *", param_count + 1));

        // Execute update
        let tenant = sqlx::query_as::<_, Tenant>(&query)
            .bind(Utc::now())
            .bind(tenant_id)
            .fetch_one(pool)
            .await?;

        Ok(tenant)
    }

    pub async fn delete_tenant(&self, tenant_id: Uuid) -> Result<()> {
        let pool = self.db.pool();

        sqlx::query!(
            r#"
            UPDATE tenants
            SET deleted_at = $1, updated_at = $1
            WHERE id = $2
            "#,
            Utc::now(),
            tenant_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Location management
    pub async fn create_location(&self, tenant_id: Uuid, request: CreateLocationRequest) -> Result<Location> {
        let pool = self.db.pool();

        let location = Location {
            id: Uuid::new_v4(),
            tenant_id,
            name: request.name,
            code: request.code,
            description: request.description,
            address: request.address,
            phone: request.phone,
            email: request.email,
            manager_id: None,
            timezone: request.timezone,
            business_hours: request.business_hours,
            is_primary: false,
            is_active: true,
            features: vec![],
            settings: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let result = sqlx::query_as!(
            Location,
            r#"
            INSERT INTO locations (
                id, tenant_id, name, code, description, address, phone,
                email, timezone, business_hours, is_primary, is_active,
                features, settings, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )
            RETURNING *
            "#,
            location.id,
            location.tenant_id,
            location.name,
            location.code,
            location.description,
            serde_json::to_value(&location.address)?,
            serde_json::to_value(&location.phone)?,
            location.email,
            location.timezone,
            serde_json::to_value(&location.business_hours)?,
            location.is_primary,
            location.is_active,
            &location.features,
            location.settings,
            location.created_at,
            location.updated_at
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_location(&self, location_id: Uuid) -> Result<Location> {
        let pool = self.db.pool();

        let location = sqlx::query_as!(
            Location,
            r#"
            SELECT * FROM locations
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            location_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| olympus_shared::error::Error::NotFound("Location not found".to_string()))?;

        Ok(location)
    }

    pub async fn list_locations(&self, tenant_id: Uuid) -> Result<Vec<Location>> {
        let pool = self.db.pool();

        let locations = sqlx::query_as!(
            Location,
            r#"
            SELECT * FROM locations
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY is_primary DESC, name ASC
            "#,
            tenant_id
        )
        .fetch_all(pool)
        .await?;

        Ok(locations)
    }

    // Role management
    pub async fn create_role(&self, tenant_id: Uuid, request: CreateRoleRequest) -> Result<Role> {
        let pool = self.db.pool();

        let role = Role {
            id: Uuid::new_v4(),
            tenant_id,
            name: request.name,
            display_name: request.display_name,
            description: request.description,
            permissions: request.permissions,
            is_system: false,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = sqlx::query_as!(
            Role,
            r#"
            INSERT INTO roles (
                id, tenant_id, name, display_name, description,
                permissions, is_system, is_active, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
            )
            RETURNING *
            "#,
            role.id,
            role.tenant_id,
            role.name,
            role.display_name,
            role.description,
            &role.permissions,
            role.is_system,
            role.is_active,
            role.created_at,
            role.updated_at
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn list_roles(&self, tenant_id: Uuid) -> Result<Vec<Role>> {
        let pool = self.db.pool();

        let roles = sqlx::query_as!(
            Role,
            r#"
            SELECT * FROM roles
            WHERE tenant_id = $1 AND is_active = true
            ORDER BY name ASC
            "#,
            tenant_id
        )
        .fetch_all(pool)
        .await?;

        Ok(roles)
    }

    pub async fn assign_role_to_user(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        let pool = self.db.pool();

        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role_id, assigned_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
            user_id,
            role_id,
            Utc::now()
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn remove_role_from_user(&self, user_id: Uuid, role_id: Uuid) -> Result<()> {
        let pool = self.db.pool();

        sqlx::query!(
            r#"
            DELETE FROM user_roles
            WHERE user_id = $1 AND role_id = $2
            "#,
            user_id,
            role_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // Statistics
    pub async fn get_tenant_statistics(&self, tenant_id: Uuid) -> Result<TenantStatistics> {
        let pool = self.db.pool();

        // This would aggregate data from multiple tables
        let stats = TenantStatistics {
            total_users: 0,
            active_users: 0,
            total_locations: 0,
            storage_used_gb: 0.0,
            api_calls_this_month: 0,
            last_activity: Utc::now(),
        };

        Ok(stats)
    }
}