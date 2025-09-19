// ============================================================================
// OLYMPUS CLOUD - ROW-LEVEL SECURITY (RLS) IMPLEMENTATION
// ============================================================================
// Module: shared/src/database/rls.rs
// Description: Row-level security policies and utilities
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use sqlx::PgPool;
use tracing::{info, warn, error};

use super::{DatabaseContext, DbResult};

/// Row-level security policy manager
pub struct RowLevelSecurity {
    pool: PgPool,
}

impl RowLevelSecurity {
    /// Create a new RLS manager
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Enable RLS for a table
    pub async fn enable_rls(&self, table_name: &str) -> DbResult<()> {
        info!("Enabling RLS for table: {}", table_name);

        let query = format!("ALTER TABLE {} ENABLE ROW LEVEL SECURITY", table_name);
        sqlx::query(&query).execute(&self.pool).await?;

        info!("RLS enabled for table: {}", table_name);
        Ok(())
    }

    /// Disable RLS for a table
    pub async fn disable_rls(&self, table_name: &str) -> DbResult<()> {
        warn!("Disabling RLS for table: {}", table_name);

        let query = format!("ALTER TABLE {} DISABLE ROW LEVEL SECURITY", table_name);
        sqlx::query(&query).execute(&self.pool).await?;

        warn!("RLS disabled for table: {}", table_name);
        Ok(())
    }

    /// Create a tenant isolation policy
    pub async fn create_tenant_policy(&self, table_name: &str, policy_name: &str) -> DbResult<()> {
        info!("Creating tenant policy '{}' for table: {}", policy_name, table_name);

        let query = format!(
            "CREATE POLICY {} ON {}
             FOR ALL
             TO authenticated
             USING (tenant_id::text = current_setting('app.current_tenant_id', true))",
            policy_name, table_name
        );

        sqlx::query(&query).execute(&self.pool).await?;

        info!("Tenant policy '{}' created for table: {}", policy_name, table_name);
        Ok(())
    }

    /// Create a user-specific policy
    pub async fn create_user_policy(&self, table_name: &str, policy_name: &str) -> DbResult<()> {
        info!("Creating user policy '{}' for table: {}", policy_name, table_name);

        let query = format!(
            "CREATE POLICY {} ON {}
             FOR ALL
             TO authenticated
             USING (
                 tenant_id::text = current_setting('app.current_tenant_id', true)
                 AND (
                     current_setting('app.current_user_id', true) = ''
                     OR user_id::text = current_setting('app.current_user_id', true)
                 )
             )",
            policy_name, table_name
        );

        sqlx::query(&query).execute(&self.pool).await?;

        info!("User policy '{}' created for table: {}", policy_name, table_name);
        Ok(())
    }

    /// Create an admin bypass policy
    pub async fn create_admin_policy(&self, table_name: &str, policy_name: &str) -> DbResult<()> {
        info!("Creating admin policy '{}' for table: {}", policy_name, table_name);

        let query = format!(
            "CREATE POLICY {} ON {}
             FOR ALL
             TO authenticated
             USING (
                 current_setting('app.current_role', true) = 'admin'
                 OR current_setting('app.current_role', true) = 'super_admin'
             )",
            policy_name, table_name
        );

        sqlx::query(&query).execute(&self.pool).await?;

        info!("Admin policy '{}' created for table: {}", policy_name, table_name);
        Ok(())
    }

    /// Drop a policy
    pub async fn drop_policy(&self, table_name: &str, policy_name: &str) -> DbResult<()> {
        info!("Dropping policy '{}' from table: {}", policy_name, table_name);

        let query = format!("DROP POLICY IF EXISTS {} ON {}", policy_name, table_name);
        sqlx::query(&query).execute(&self.pool).await?;

        info!("Policy '{}' dropped from table: {}", policy_name, table_name);
        Ok(())
    }

    /// List all policies for a table
    pub async fn list_policies(&self, table_name: &str) -> DbResult<Vec<PolicyInfo>> {
        let policies = sqlx::query_as::<_, (String, String, bool, Option<String>)>(
            "SELECT
                pol.polname::text as name,
                pol.polcmd::text as command,
                pol.polpermissive as is_permissive,
                pol.polroles::text as roles
             FROM pg_policy pol
             JOIN pg_class cls ON pol.polrelid = cls.oid
             WHERE cls.relname = $1
             ORDER BY pol.polname"
        )
        .bind(table_name)
        .fetch_all(&self.pool)
        .await?;

        let policy_info = policies
            .into_iter()
            .map(|(name, command, is_permissive, roles)| PolicyInfo {
                name,
                command,
                is_permissive,
                roles,
                using_expression: None,
                with_check_expression: None,
            })
            .collect();

        Ok(policy_info)
    }

    /// Setup standard RLS policies for all tenant-scoped tables
    pub async fn setup_standard_policies(&self) -> DbResult<()> {
        info!("Setting up standard RLS policies for all tables");

        let tenant_tables = vec![
            "users", "user_sessions", "user_mfa", "api_keys",
            "roles", "user_roles", "user_permissions",
            "products", "product_variants", "categories",
            "orders", "order_items", "payments",
            "customers", "locations", "inventory",
            "domain_events"
        ];

        for table in tenant_tables {
            // Enable RLS
            if let Err(e) = self.enable_rls(table).await {
                error!("Failed to enable RLS for {}: {}", table, e);
                continue;
            }

            // Create tenant isolation policy
            let tenant_policy = format!("{}_tenant_isolation", table);
            if let Err(e) = self.create_tenant_policy(table, &tenant_policy).await {
                error!("Failed to create tenant policy for {}: {}", table, e);
            }

            // Create admin bypass policy
            let admin_policy = format!("{}_admin_access", table);
            if let Err(e) = self.create_admin_policy(table, &admin_policy).await {
                error!("Failed to create admin policy for {}: {}", table, e);
            }
        }

        // Special policies for user-specific tables
        let user_tables = vec!["user_sessions", "user_mfa"];
        for table in user_tables {
            let user_policy = format!("{}_user_access", table);
            if let Err(e) = self.create_user_policy(table, &user_policy).await {
                error!("Failed to create user policy for {}: {}", table, e);
            }
        }

        info!("Standard RLS policies setup completed");
        Ok(())
    }

    /// Set database context for RLS
    pub async fn set_context(&self, context: &DatabaseContext) -> DbResult<()> {
        info!("Setting RLS context for tenant: {}", context.tenant_id);

        // Set tenant context
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(context.tenant_id.to_string())
            .execute(&self.pool)
            .await?;

        // Set user context if available
        if let Some(user_id) = context.user_id {
            sqlx::query("SELECT set_config('app.current_user_id', $1, true)")
                .bind(user_id.to_string())
                .execute(&self.pool)
                .await?;
        }

        // Set role context if available
        if let Some(role) = &context.role {
            sqlx::query("SELECT set_config('app.current_role', $1, true)")
                .bind(role)
                .execute(&self.pool)
                .await?;
        }

        info!("RLS context set successfully");
        Ok(())
    }

    /// Clear RLS context
    pub async fn clear_context(&self) -> DbResult<()> {
        info!("Clearing RLS context");

        sqlx::query("SELECT set_config('app.current_tenant_id', '', false)")
            .execute(&self.pool)
            .await?;

        sqlx::query("SELECT set_config('app.current_user_id', '', false)")
            .execute(&self.pool)
            .await?;

        sqlx::query("SELECT set_config('app.current_role', '', false)")
            .execute(&self.pool)
            .await?;

        info!("RLS context cleared");
        Ok(())
    }

    /// Test RLS policies by attempting to access data
    pub async fn test_policies(&self, context: &DatabaseContext) -> DbResult<PolicyTestResult> {
        info!("Testing RLS policies for tenant: {}", context.tenant_id);

        let mut result = PolicyTestResult::new();

        // Set context
        self.set_context(context).await?;

        // Test basic table access
        for table in &["users", "products", "orders"] {
            let query = format!("SELECT COUNT(*) FROM {}", table);
            match sqlx::query_scalar::<_, i64>(&query)
                .fetch_one(&self.pool)
                .await
            {
                Ok(count) => {
                    result.table_tests.push(TableTest {
                        table: table.to_string(),
                        success: true,
                        row_count: Some(count as u32),
                        error: None,
                    });
                }
                Err(e) => {
                    result.table_tests.push(TableTest {
                        table: table.to_string(),
                        success: false,
                        row_count: None,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        // Clear context
        self.clear_context().await?;

        result.success = result.table_tests.iter().all(|t| t.success);
        Ok(result)
    }
}

/// Policy information
#[derive(Debug, Clone)]
pub struct PolicyInfo {
    pub name: String,
    pub command: String,
    pub is_permissive: bool,
    pub roles: Option<String>,
    pub using_expression: Option<String>,
    pub with_check_expression: Option<String>,
}

/// Policy test result
#[derive(Debug, Clone)]
pub struct PolicyTestResult {
    pub success: bool,
    pub table_tests: Vec<TableTest>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl PolicyTestResult {
    pub fn new() -> Self {
        Self {
            success: false,
            table_tests: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Table access test result
#[derive(Debug, Clone)]
pub struct TableTest {
    pub table: String,
    pub success: bool,
    pub row_count: Option<u32>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_test_result_creation() {
        let result = PolicyTestResult::new();
        assert!(!result.success);
        assert!(result.table_tests.is_empty());
    }

    #[test]
    fn test_table_test_creation() {
        let test = TableTest {
            table: "users".to_string(),
            success: true,
            row_count: Some(10),
            error: None,
        };

        assert_eq!(test.table, "users");
        assert!(test.success);
        assert_eq!(test.row_count, Some(10));
        assert!(test.error.is_none());
    }
}