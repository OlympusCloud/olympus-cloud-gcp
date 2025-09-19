// ============================================================================
// OLYMPUS CLOUD - DATABASE HEALTH CHECKER
// ============================================================================
// Module: shared/src/database/health.rs
// Description: Database health monitoring and diagnostics
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use sqlx::{PgPool, Row};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

use super::DbResult;

/// Database health checker
pub struct HealthChecker {
    pool: PgPool,
    timeout: Duration,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            timeout: Duration::from_secs(5),
        }
    }

    /// Create with custom timeout
    pub fn with_timeout(pool: PgPool, timeout: Duration) -> Self {
        Self { pool, timeout }
    }

    /// Perform a comprehensive health check
    pub async fn check_health(&self) -> HealthCheckResult {
        let start_time = Instant::now();
        let mut result = HealthCheckResult::new();

        // Basic connectivity check
        match self.check_connectivity().await {
            Ok(duration) => {
                result.connectivity = HealthStatus::Healthy;
                result.response_time_ms = duration.as_millis() as u64;
            }
            Err(e) => {
                result.connectivity = HealthStatus::Unhealthy;
                result.errors.push(format!("Connectivity: {}", e));
            }
        }

        // Pool status check
        result.pool_status = self.check_pool_status();

        // Database version check
        match self.check_database_version().await {
            Ok(version) => {
                result.database_version = Some(version);
            }
            Err(e) => {
                result.errors.push(format!("Version check: {}", e));
            }
        }

        // Migration status check
        match self.check_migration_status().await {
            Ok(status) => {
                result.migration_status = status;
            }
            Err(e) => {
                result.errors.push(format!("Migration check: {}", e));
            }
        }

        // Table count check
        match self.check_table_count().await {
            Ok(count) => {
                result.table_count = Some(count);
            }
            Err(e) => {
                result.errors.push(format!("Table count: {}", e));
            }
        }

        result.total_duration_ms = start_time.elapsed().as_millis() as u64;
        result.overall_status = if result.errors.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        };

        result
    }

    /// Check basic database connectivity
    async fn check_connectivity(&self) -> DbResult<Duration> {
        let start = Instant::now();

        sqlx::query("SELECT 1 as health_check")
            .fetch_one(&self.pool)
            .await?;

        Ok(start.elapsed())
    }

    /// Check connection pool status
    fn check_pool_status(&self) -> PoolStatus {
        let stats = self.pool.size();

        PoolStatus {
            total_connections: stats,
            idle_connections: 0, // SQLx doesn't expose idle count easily
            active_connections: stats,
            is_closed: self.pool.is_closed(),
        }
    }

    /// Check database version
    async fn check_database_version(&self) -> DbResult<String> {
        let row = sqlx::query("SELECT version() as version")
            .fetch_one(&self.pool)
            .await?;

        let version: String = row.try_get("version")?;
        Ok(version)
    }

    /// Check migration status
    async fn check_migration_status(&self) -> DbResult<MigrationStatus> {
        // Check if migrations table exists
        let table_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS (
                SELECT FROM information_schema.tables
                WHERE table_schema = 'public'
                AND table_name = '_sqlx_migrations'
            )"
        )
        .fetch_one(&self.pool)
        .await?;

        if !table_exists {
            return Ok(MigrationStatus {
                migrations_applied: 0,
                last_migration: None,
                pending_migrations: true,
            });
        }

        // Get migration count and last migration
        let last_migration = sqlx::query_as::<_, (i64, String, chrono::DateTime<chrono::Utc>)>(
            "SELECT version, description, installed_on
             FROM _sqlx_migrations
             ORDER BY version DESC
             LIMIT 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        let migrations_count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM _sqlx_migrations"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(MigrationStatus {
            migrations_applied: migrations_count as u32,
            last_migration: last_migration.map(|m| LastMigration {
                version: m.0,
                description: m.1,
                installed_on: m.2,
            }),
            pending_migrations: false, // Would need to check against file system
        })
    }

    /// Check table count
    async fn check_table_count(&self) -> DbResult<u32> {
        let count = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)
             FROM information_schema.tables
             WHERE table_schema = 'public'
             AND table_type = 'BASE TABLE'"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count as u32)
    }

    /// Perform a simple liveness check
    pub async fn is_alive(&self) -> bool {
        tokio::time::timeout(self.timeout, self.check_connectivity())
            .await
            .is_ok()
    }

    /// Perform a readiness check
    pub async fn is_ready(&self) -> bool {
        let health = self.check_health().await;
        matches!(health.overall_status, HealthStatus::Healthy)
    }
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub overall_status: HealthStatus,
    pub connectivity: HealthStatus,
    pub response_time_ms: u64,
    pub total_duration_ms: u64,
    pub pool_status: PoolStatus,
    pub database_version: Option<String>,
    pub migration_status: MigrationStatus,
    pub table_count: Option<u32>,
    pub errors: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HealthCheckResult {
    pub fn new() -> Self {
        Self {
            overall_status: HealthStatus::Unknown,
            connectivity: HealthStatus::Unknown,
            response_time_ms: 0,
            total_duration_ms: 0,
            pool_status: PoolStatus::default(),
            database_version: None,
            migration_status: MigrationStatus::default(),
            table_count: None,
            errors: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

/// Connection pool status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStatus {
    pub total_connections: u32,
    pub idle_connections: u32,
    pub active_connections: u32,
    pub is_closed: bool,
}

impl Default for PoolStatus {
    fn default() -> Self {
        Self {
            total_connections: 0,
            idle_connections: 0,
            active_connections: 0,
            is_closed: true,
        }
    }
}

/// Migration status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStatus {
    pub migrations_applied: u32,
    pub last_migration: Option<LastMigration>,
    pub pending_migrations: bool,
}

impl Default for MigrationStatus {
    fn default() -> Self {
        Self {
            migrations_applied: 0,
            last_migration: None,
            pending_migrations: false,
        }
    }
}

/// Last migration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastMigration {
    pub version: i64,
    pub description: String,
    pub installed_on: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_result_creation() {
        let result = HealthCheckResult::new();
        assert_eq!(result.overall_status, HealthStatus::Unknown);
        assert_eq!(result.connectivity, HealthStatus::Unknown);
        assert_eq!(result.response_time_ms, 0);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_pool_status_default() {
        let status = PoolStatus::default();
        assert_eq!(status.total_connections, 0);
        assert_eq!(status.idle_connections, 0);
        assert_eq!(status.active_connections, 0);
        assert!(status.is_closed);
    }

    #[test]
    fn test_migration_status_default() {
        let status = MigrationStatus::default();
        assert_eq!(status.migrations_applied, 0);
        assert!(status.last_migration.is_none());
        assert!(!status.pending_migrations);
    }
}