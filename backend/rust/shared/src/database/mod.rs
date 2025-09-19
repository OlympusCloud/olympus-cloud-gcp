// ============================================================================
// OLYMPUS CLOUD - DATABASE MODULE
// ============================================================================
// Module: shared/src/database/mod.rs
// Description: Database connection management and utilities
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

pub mod connection;
pub mod migrations;
pub mod health;
pub mod rls;

pub use connection::{DatabaseConnection, DatabaseConfig, ConnectionPool};
pub use migrations::MigrationRunner;
pub use health::HealthChecker;
pub use rls::RowLevelSecurity;

use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

/// Type alias for database pool
pub type DbPool = PgPool;

/// Type alias for database transaction
pub type DbTransaction<'a> = Transaction<'a, Postgres>;

/// Database result type
pub type DbResult<T> = Result<T, sqlx::Error>;

/// Database context for tenant-scoped operations
#[derive(Debug, Clone)]
pub struct DatabaseContext {
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub role: Option<String>,
}

impl DatabaseContext {
    /// Create a new database context
    pub fn new(tenant_id: Uuid) -> Self {
        Self {
            tenant_id,
            user_id: None,
            role: None,
        }
    }

    /// Set user context
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Set role context
    pub fn with_role(mut self, role: String) -> Self {
        self.role = Some(role);
        self
    }
}