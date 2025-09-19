// ============================================================================
// OLYMPUS CLOUD - DATABASE MIGRATION RUNNER
// ============================================================================
// Module: shared/src/database/migrations.rs
// Description: Database migration management and runner
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use sqlx::{migrate::Migrator, PgPool};
use std::path::Path;
use tracing::{info, error};

use super::DbResult;

/// Migration runner for database schema updates
pub struct MigrationRunner {
    pool: PgPool,
    migrations_path: String,
}

impl MigrationRunner {
    /// Create a new migration runner
    pub fn new(pool: PgPool, migrations_path: Option<String>) -> Self {
        let migrations_path = migrations_path
            .unwrap_or_else(|| "./migrations".to_string());

        Self {
            pool,
            migrations_path,
        }
    }

    /// Run all pending migrations
    pub async fn run(&self) -> DbResult<()> {
        info!("Running database migrations from: {}", self.migrations_path);

        let migrator = Migrator::new(Path::new(&self.migrations_path))
            .await
            .map_err(|e| {
                error!("Failed to create migrator: {}", e);
                sqlx::Error::Migrate(Box::new(e))
            })?;

        migrator.run(&self.pool).await.map_err(|e| {
            error!("Failed to run migrations: {}", e);
            e
        })?;

        info!("Database migrations completed successfully");
        Ok(())
    }

    /// Get migration status
    pub async fn status(&self) -> DbResult<Vec<MigrationInfo>> {
        let migrator = Migrator::new(Path::new(&self.migrations_path))
            .await
            .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?;

        let mut migration_info = Vec::new();

        // Get applied migrations from database
        let applied_migrations = sqlx::query_as::<_, (i64, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT version, description, installed_on FROM _sqlx_migrations ORDER BY version"
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default();

        // Compare with available migrations
        for migration in migrator.migrations.iter() {
            let is_applied = applied_migrations
                .iter()
                .any(|applied| applied.0 == migration.version);

            let installed_on = if is_applied {
                applied_migrations
                    .iter()
                    .find(|applied| applied.0 == migration.version)
                    .map(|applied| applied.2)
            } else {
                None
            };

            migration_info.push(MigrationInfo {
                version: migration.version,
                description: migration.description.to_string(),
                is_applied,
                installed_on,
            });
        }

        Ok(migration_info)
    }

    /// Revert last migration (if supported)
    pub async fn revert(&self) -> DbResult<()> {
        info!("Reverting last migration");

        // Note: SQLx doesn't support automatic rollbacks
        // This would need to be implemented manually if needed
        error!("Migration rollback not implemented - would require manual SQL scripts");

        Err(sqlx::Error::Configuration(
            "Migration rollback not supported".into()
        ))
    }

    /// Validate migrations without applying
    pub async fn validate(&self) -> DbResult<bool> {
        info!("Validating migrations");

        let migrator = Migrator::new(Path::new(&self.migrations_path))
            .await
            .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?;

        // Basic validation - check if migrations can be parsed
        for migration in migrator.migrations.iter() {
            if migration.migration_type.is_down_migration() {
                continue; // Skip down migrations as they're not supported
            }

            // Simple SQL validation - check if it's not empty
            if migration.sql.trim().is_empty() {
                error!("Migration {} has empty SQL", migration.version);
                return Ok(false);
            }
        }

        info!("All migrations validated successfully");
        Ok(true)
    }
}

/// Information about a migration
#[derive(Debug, Clone)]
pub struct MigrationInfo {
    pub version: i64,
    pub description: String,
    pub is_applied: bool,
    pub installed_on: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for MigrationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.is_applied { "APPLIED" } else { "PENDING" };
        let installed = if let Some(date) = self.installed_on {
            format!(" ({})", date.format("%Y-%m-%d %H:%M:%S"))
        } else {
            String::new()
        };

        write!(
            f,
            "{:03} | {} | {}{}",
            self.version, status, self.description, installed
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_info_display() {
        let info = MigrationInfo {
            version: 1,
            description: "Initial schema".to_string(),
            is_applied: true,
            installed_on: Some(chrono::Utc::now()),
        };

        let display = format!("{}", info);
        assert!(display.contains("001"));
        assert!(display.contains("APPLIED"));
        assert!(display.contains("Initial schema"));
    }

    #[test]
    fn test_migration_info_pending() {
        let info = MigrationInfo {
            version: 2,
            description: "Add indexes".to_string(),
            is_applied: false,
            installed_on: None,
        };

        let display = format!("{}", info);
        assert!(display.contains("002"));
        assert!(display.contains("PENDING"));
        assert!(display.contains("Add indexes"));
    }
}