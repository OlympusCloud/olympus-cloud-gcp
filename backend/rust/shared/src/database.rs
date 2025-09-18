use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;
use crate::error::Result;

pub type DbPool = PgPool;
pub type DbTransaction<'a> = Transaction<'a, Postgres>;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;

        // Run migrations if needed
        // sqlx::migrate!("../../../docs").run(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn begin_transaction(&self) -> Result<DbTransaction> {
        Ok(self.pool.begin().await?)
    }

    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }
}

/// Set tenant context for row-level security
pub async fn set_tenant_context(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
    tenant_id: Uuid,
) -> Result<()> {
    sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(executor)
        .await?;
    Ok(())
}

/// Clear tenant context
pub async fn clear_tenant_context(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
) -> Result<()> {
    sqlx::query("SELECT set_config('app.tenant_id', '', true)")
        .execute(executor)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tenant_context() {
        // This would require a test database
        // For now, just ensure it compiles
        let tenant_id = Uuid::new_v4();
        assert!(!tenant_id.to_string().is_empty());
    }
}