// ============================================================================
// OLYMPUS CLOUD - DATABASE CONNECTION MANAGEMENT
// ============================================================================
// Module: shared/src/database/connection.rs
// Description: Database connection pool and configuration
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use sqlx::{postgres::PgPoolOptions, PgPool, Postgres, Row};
use std::time::Duration;
use tracing::{info, warn};

use super::{DatabaseContext, DbResult};

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
    pub test_before_acquire: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://localhost/olympus_dev".to_string()),
            max_connections: 20,
            min_connections: 1,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Some(Duration::from_secs(600)), // 10 minutes
            max_lifetime: Some(Duration::from_secs(1800)), // 30 minutes
            test_before_acquire: true,
        }
    }
}

/// Database connection pool wrapper
#[derive(Debug, Clone)]
pub struct ConnectionPool {
    pool: PgPool,
}

impl ConnectionPool {
    /// Create a new connection pool
    pub async fn new(config: DatabaseConfig) -> DbResult<Self> {
        info!("Creating database connection pool with config: {:?}", config);

        let mut pool_options = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .test_before_acquire(config.test_before_acquire);

        if let Some(idle_timeout) = config.idle_timeout {
            pool_options = pool_options.idle_timeout(idle_timeout);
        }

        if let Some(max_lifetime) = config.max_lifetime {
            pool_options = pool_options.max_lifetime(max_lifetime);
        }

        let pool = pool_options.connect(&config.database_url).await?;

        info!("Database connection pool created successfully");
        Ok(Self { pool })
    }

    /// Get the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get pool statistics
    pub fn stats(&self) -> u32 {
        self.pool.size()
    }

    /// Check if the pool is closed
    pub fn is_closed(&self) -> bool {
        self.pool.is_closed()
    }

    /// Close the connection pool
    pub fn close(&self) {
        info!("Closing database connection pool");
        self.pool.close();
    }

    /// Execute a health check query
    pub async fn health_check(&self) -> DbResult<bool> {
        let result = sqlx::query("SELECT 1 as health_check")
            .fetch_one(&self.pool)
            .await?;

        let health_value: i32 = result.try_get("health_check")?;
        Ok(health_value == 1)
    }

    /// Set row-level security context for a tenant
    pub async fn set_tenant_context(&self, context: &DatabaseContext) -> DbResult<()> {
        let mut tx = self.pool.begin().await?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(context.tenant_id.to_string())
            .execute(&mut *tx)
            .await?;

        // Set user context if available
        if let Some(user_id) = context.user_id {
            sqlx::query("SELECT set_config('app.current_user_id', $1, true)")
                .bind(user_id.to_string())
                .execute(&mut *tx)
                .await?;
        }

        // Set role context if available
        if let Some(role) = &context.role {
            sqlx::query("SELECT set_config('app.current_role', $1, true)")
                .bind(role)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    /// Clear RLS context
    pub async fn clear_context(&self) -> DbResult<()> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SELECT set_config('app.current_tenant_id', '', false)")
            .execute(&mut *tx)
            .await?;

        sqlx::query("SELECT set_config('app.current_user_id', '', false)")
            .execute(&mut *tx)
            .await?;

        sqlx::query("SELECT set_config('app.current_role', '', false)")
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}

/// Database connection wrapper with context management
#[derive(Debug)]
pub struct DatabaseConnection {
    pool: ConnectionPool,
    context: Option<DatabaseContext>,
}

impl DatabaseConnection {
    /// Create a new database connection
    pub async fn new(config: DatabaseConfig) -> DbResult<Self> {
        let pool = ConnectionPool::new(config).await?;
        Ok(Self {
            pool,
            context: None,
        })
    }

    /// Create connection from existing pool
    pub fn from_pool(pool: ConnectionPool) -> Self {
        Self {
            pool,
            context: None,
        }
    }

    /// Set database context for tenant-scoped operations
    pub async fn with_context(mut self, context: DatabaseContext) -> DbResult<Self> {
        self.pool.set_tenant_context(&context).await?;
        self.context = Some(context);
        Ok(self)
    }

    /// Get the connection pool
    pub fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    /// Get the current context
    pub fn context(&self) -> Option<&DatabaseContext> {
        self.context.as_ref()
    }

    /// Execute a query with automatic context management
    pub async fn execute_with_context<F, R>(&self, operation: F) -> DbResult<R>
    where
        F: FnOnce(&PgPool) -> std::pin::Pin<Box<dyn std::future::Future<Output = DbResult<R>> + Send + '_>>,
    {
        // Set context if available
        if let Some(context) = &self.context {
            self.pool.set_tenant_context(context).await?;
        }

        // Execute operation
        let result = operation(self.pool.pool()).await;

        // Clear context after operation
        if self.context.is_some() {
            if let Err(e) = self.pool.clear_context().await {
                warn!("Failed to clear database context: {}", e);
            }
        }

        result
    }

    /// Begin a transaction with context
    pub async fn begin_transaction(&self) -> DbResult<sqlx::Transaction<'_, Postgres>> {
        if let Some(context) = &self.context {
            self.pool.set_tenant_context(context).await?;
        }
        self.pool.pool().begin().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert_eq!(config.max_connections, 20);
        assert_eq!(config.min_connections, 1);
        assert!(config.test_before_acquire);
    }

    #[test]
    fn test_database_context_creation() {
        use uuid::Uuid;

        let tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let context = DatabaseContext::new(tenant_id)
            .with_user(user_id)
            .with_role("admin".to_string());

        assert_eq!(context.tenant_id, tenant_id);
        assert_eq!(context.user_id, Some(user_id));
        assert_eq!(context.role, Some("admin".to_string()));
    }
}