// ============================================================================
// OLYMPUS CLOUD - PLATFORM MONITORING AND OBSERVABILITY SERVICE
// ============================================================================
// Module: platform/src/services/monitoring.rs
// Description: Platform monitoring, metrics collection, and observability
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use anyhow::Result;
use tracing::{info, warn, error, debug};
use serde_json::Value;

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use olympus_shared::error::Error as OlympusError;

use crate::models::{
    TenantHealthCheck, TenantAnalytics, FeatureFlagAnalytics, FeatureFlagUsage
};

#[derive(Clone)]
pub struct PlatformMonitoringService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub total_tenants: i64,
    pub active_tenants: i64,
    pub total_users: i64,
    pub active_users_24h: i64,
    pub total_requests_24h: i64,
    pub avg_response_time_ms: f64,
    pub error_rate: f64,
    pub db_connections_active: i32,
    pub db_connections_idle: i32,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

#[derive(Debug, Clone)]
pub struct TenantMetrics {
    pub tenant_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub active_users: i32,
    pub requests_count: i64,
    pub error_count: i64,
    pub avg_response_time: f64,
    pub storage_used_gb: f64,
    pub bandwidth_used_gb: f64,
    pub feature_flags_evaluated: i64,
    pub configurations_accessed: i64,
}

#[derive(Debug, Clone)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub metric_name: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub is_active: bool,
    pub notify_channels: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum AlertCondition {
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
    PercentageIncrease,
    PercentageDecrease,
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub metric_name: String,
    pub current_value: f64,
    pub threshold: f64,
    pub severity: AlertSeverity,
    pub message: String,
    pub triggered_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
}

impl PlatformMonitoringService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // ========================================================================
    // SYSTEM METRICS COLLECTION
    // ========================================================================

    pub async fn collect_system_metrics(&self) -> Result<SystemMetrics> {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);

        // Collect tenant metrics
        let tenant_stats = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as total_tenants,
                COUNT(*) FILTER (WHERE is_active = true) as active_tenants
            FROM tenants
            "#
        )
        .fetch_one(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(e.to_string()))?;

        // Collect user metrics
        let user_stats = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as total_users,
                COUNT(*) FILTER (WHERE last_login_at > $1) as active_users_24h
            FROM users
            "#,
            yesterday
        )
        .fetch_one(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(e.to_string()))?;

        // Collect request metrics (would typically come from logs/metrics store)
        let request_stats = self.get_request_metrics_24h().await?;

        // Collect database metrics
        let db_stats = self.get_database_metrics().await?;

        // Collect system resource metrics
        let resource_stats = self.get_system_resource_metrics().await?;

        Ok(SystemMetrics {
            timestamp: now,
            total_tenants: tenant_stats.total_tenants.unwrap_or(0),
            active_tenants: tenant_stats.active_tenants.unwrap_or(0),
            total_users: user_stats.total_users.unwrap_or(0),
            active_users_24h: user_stats.active_users_24h.unwrap_or(0),
            total_requests_24h: request_stats.total_requests,
            avg_response_time_ms: request_stats.avg_response_time,
            error_rate: request_stats.error_rate,
            db_connections_active: db_stats.active_connections,
            db_connections_idle: db_stats.idle_connections,
            memory_usage_mb: resource_stats.memory_usage_mb,
            cpu_usage_percent: resource_stats.cpu_usage_percent,
        })
    }

    pub async fn collect_tenant_metrics(&self, tenant_id: Uuid) -> Result<TenantMetrics> {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);

        // Active users for tenant
        let active_users = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE tenant_id = $1 AND last_login_at > $2",
            tenant_id,
            yesterday
        )
        .fetch_one(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(e.to_string()))?;

        // Get request metrics for tenant (would typically come from application metrics)
        let request_metrics = self.get_tenant_request_metrics(tenant_id).await?;

        // Get storage and bandwidth usage
        let usage_metrics = self.get_tenant_usage_metrics(tenant_id).await?;

        // Get feature flag and configuration access metrics
        let platform_metrics = self.get_tenant_platform_metrics(tenant_id).await?;

        Ok(TenantMetrics {
            tenant_id,
            timestamp: now,
            active_users: active_users.count.unwrap_or(0) as i32,
            requests_count: request_metrics.total_requests,
            error_count: request_metrics.error_count,
            avg_response_time: request_metrics.avg_response_time,
            storage_used_gb: usage_metrics.storage_used_gb,
            bandwidth_used_gb: usage_metrics.bandwidth_used_gb,
            feature_flags_evaluated: platform_metrics.feature_flags_evaluated,
            configurations_accessed: platform_metrics.configurations_accessed,
        })
    }

    // ========================================================================
    // HEALTH MONITORING
    // ========================================================================

    pub async fn run_system_health_checks(&self) -> Result<Vec<TenantHealthCheck>> {
        let mut health_checks = Vec::new();

        // Database health check
        let db_health = self.check_database_health().await?;
        health_checks.push(db_health);

        // Redis health check
        let redis_health = self.check_redis_health().await?;
        health_checks.push(redis_health);

        // External services health check
        let external_health = self.check_external_services_health().await?;
        health_checks.extend(external_health);

        // Resource utilization checks
        let resource_health = self.check_resource_utilization().await?;
        health_checks.extend(resource_health);

        Ok(health_checks)
    }

    async fn check_database_health(&self) -> Result<TenantHealthCheck> {
        let start = std::time::Instant::now();

        let result = sqlx::query!("SELECT 1 as test, pg_database_size(current_database()) as db_size")
            .fetch_one(&*self.db)
            .await;

        let response_time = start.elapsed().as_millis() as i32;

        match result {
            Ok(row) => {
                let db_size_mb = row.db_size.unwrap_or(0) as f64 / 1024.0 / 1024.0;
                let status = if response_time > 1000 { "warning" } else { "healthy" };

                Ok(TenantHealthCheck {
                    tenant_id: Uuid::nil(), // System-wide check
                    check_name: "database_health".to_string(),
                    status: status.to_string(),
                    last_check: Utc::now(),
                    response_time_ms: Some(response_time),
                    error_count: 0,
                    details: serde_json::json!({
                        "response_time_ms": response_time,
                        "database_size_mb": db_size_mb,
                        "connection_status": "ok"
                    }),
                })
            }
            Err(e) => {
                error!("Database health check failed: {}", e);
                Ok(TenantHealthCheck {
                    tenant_id: Uuid::nil(),
                    check_name: "database_health".to_string(),
                    status: "critical".to_string(),
                    last_check: Utc::now(),
                    response_time_ms: Some(response_time),
                    error_count: 1,
                    details: serde_json::json!({
                        "error": e.to_string(),
                        "connection_status": "failed"
                    }),
                })
            }
        }
    }

    async fn check_redis_health(&self) -> Result<TenantHealthCheck> {
        // This would integrate with Redis client
        // For now, returning a mock implementation
        Ok(TenantHealthCheck {
            tenant_id: Uuid::nil(),
            check_name: "redis_health".to_string(),
            status: "healthy".to_string(),
            last_check: Utc::now(),
            response_time_ms: Some(5),
            error_count: 0,
            details: serde_json::json!({
                "connection_status": "ok",
                "memory_usage": "normal"
            }),
        })
    }

    async fn check_external_services_health(&self) -> Result<Vec<TenantHealthCheck>> {
        // This would check external service dependencies
        // Payment processors, email services, etc.
        Ok(vec![])
    }

    async fn check_resource_utilization(&self) -> Result<Vec<TenantHealthCheck>> {
        let mut checks = Vec::new();

        // Memory utilization check
        let memory_usage = self.get_memory_usage().await?;
        let memory_status = if memory_usage > 90.0 {
            "critical"
        } else if memory_usage > 80.0 {
            "warning"
        } else {
            "healthy"
        };

        checks.push(TenantHealthCheck {
            tenant_id: Uuid::nil(),
            check_name: "memory_utilization".to_string(),
            status: memory_status.to_string(),
            last_check: Utc::now(),
            response_time_ms: None,
            error_count: if memory_usage > 90.0 { 1 } else { 0 },
            details: serde_json::json!({
                "memory_usage_percent": memory_usage,
                "warning_threshold": 80.0,
                "critical_threshold": 90.0
            }),
        });

        // CPU utilization check
        let cpu_usage = self.get_cpu_usage().await?;
        let cpu_status = if cpu_usage > 95.0 {
            "critical"
        } else if cpu_usage > 85.0 {
            "warning"
        } else {
            "healthy"
        };

        checks.push(TenantHealthCheck {
            tenant_id: Uuid::nil(),
            check_name: "cpu_utilization".to_string(),
            status: cpu_status.to_string(),
            last_check: Utc::now(),
            response_time_ms: None,
            error_count: if cpu_usage > 95.0 { 1 } else { 0 },
            details: serde_json::json!({
                "cpu_usage_percent": cpu_usage,
                "warning_threshold": 85.0,
                "critical_threshold": 95.0
            }),
        });

        Ok(checks)
    }

    // ========================================================================
    // ALERTING SYSTEM
    // ========================================================================

    pub async fn evaluate_alert_rules(&self) -> Result<Vec<Alert>> {
        let mut triggered_alerts = Vec::new();

        let alert_rules = self.get_active_alert_rules().await?;
        let system_metrics = self.collect_system_metrics().await?;

        for rule in alert_rules {
            if let Some(alert) = self.evaluate_alert_rule(&rule, &system_metrics).await? {
                triggered_alerts.push(alert);
            }
        }

        // Store triggered alerts
        for alert in &triggered_alerts {
            self.store_alert(alert).await?;
        }

        Ok(triggered_alerts)
    }

    async fn evaluate_alert_rule(&self, rule: &AlertRule, metrics: &SystemMetrics) -> Result<Option<Alert>> {
        let current_value = self.get_metric_value(&rule.metric_name, metrics).await?;

        let is_triggered = match rule.condition {
            AlertCondition::GreaterThan => current_value > rule.threshold,
            AlertCondition::LessThan => current_value < rule.threshold,
            AlertCondition::Equals => (current_value - rule.threshold).abs() < 0.001,
            AlertCondition::NotEquals => (current_value - rule.threshold).abs() >= 0.001,
            AlertCondition::PercentageIncrease => {
                // Would need historical data for this
                false
            }
            AlertCondition::PercentageDecrease => {
                // Would need historical data for this
                false
            }
        };

        if is_triggered {
            Ok(Some(Alert {
                id: Uuid::new_v4(),
                rule_id: rule.id,
                tenant_id: None, // System-wide alert
                metric_name: rule.metric_name.clone(),
                current_value,
                threshold: rule.threshold,
                severity: rule.severity.clone(),
                message: format!(
                    "Alert: {} - {} is {} (threshold: {})",
                    rule.name,
                    rule.metric_name,
                    current_value,
                    rule.threshold
                ),
                triggered_at: Utc::now(),
                resolved_at: None,
                acknowledged_at: None,
                acknowledged_by: None,
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_metric_value(&self, metric_name: &str, metrics: &SystemMetrics) -> Result<f64> {
        match metric_name {
            "total_tenants" => Ok(metrics.total_tenants as f64),
            "active_tenants" => Ok(metrics.active_tenants as f64),
            "total_users" => Ok(metrics.total_users as f64),
            "active_users_24h" => Ok(metrics.active_users_24h as f64),
            "avg_response_time_ms" => Ok(metrics.avg_response_time_ms),
            "error_rate" => Ok(metrics.error_rate),
            "memory_usage_percent" => Ok(metrics.memory_usage_mb),
            "cpu_usage_percent" => Ok(metrics.cpu_usage_percent),
            _ => Err(OlympusError::NotFound(format!("Unknown metric: {}", metric_name)).into()),
        }
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    async fn get_request_metrics_24h(&self) -> Result<RequestMetrics> {
        // This would typically come from application logs or metrics store
        // For now, returning mock data
        Ok(RequestMetrics {
            total_requests: 150000,
            avg_response_time: 85.5,
            error_rate: 0.02,
        })
    }

    async fn get_database_metrics(&self) -> Result<DatabaseMetrics> {
        let stats = sqlx::query!(
            r#"
            SELECT
                setting as max_connections
            FROM pg_settings
            WHERE name = 'max_connections'
            "#
        )
        .fetch_one(&*self.db)
        .await
        .map_err(|e| OlympusError::Database(e.to_string()))?;

        Ok(DatabaseMetrics {
            active_connections: 25, // Mock data
            idle_connections: 10,
            max_connections: stats.max_connections.parse().unwrap_or(100),
        })
    }

    async fn get_system_resource_metrics(&self) -> Result<ResourceMetrics> {
        // This would integrate with system monitoring
        Ok(ResourceMetrics {
            memory_usage_mb: 1024.0,
            cpu_usage_percent: 15.5,
        })
    }

    async fn get_tenant_request_metrics(&self, _tenant_id: Uuid) -> Result<TenantRequestMetrics> {
        // Mock implementation
        Ok(TenantRequestMetrics {
            total_requests: 5000,
            error_count: 10,
            avg_response_time: 95.0,
        })
    }

    async fn get_tenant_usage_metrics(&self, _tenant_id: Uuid) -> Result<TenantUsageStats> {
        // Mock implementation
        Ok(TenantUsageStats {
            storage_used_gb: 2.5,
            bandwidth_used_gb: 0.8,
        })
    }

    async fn get_tenant_platform_metrics(&self, _tenant_id: Uuid) -> Result<TenantPlatformStats> {
        // Mock implementation
        Ok(TenantPlatformStats {
            feature_flags_evaluated: 1000,
            configurations_accessed: 250,
        })
    }

    async fn get_active_alert_rules(&self) -> Result<Vec<AlertRule>> {
        // This would fetch from database
        // For now, returning default rules
        Ok(vec![
            AlertRule {
                id: Uuid::new_v4(),
                name: "High Error Rate".to_string(),
                metric_name: "error_rate".to_string(),
                condition: AlertCondition::GreaterThan,
                threshold: 0.05, // 5% error rate
                severity: AlertSeverity::Critical,
                is_active: true,
                notify_channels: vec!["email".to_string(), "slack".to_string()],
            },
            AlertRule {
                id: Uuid::new_v4(),
                name: "High Response Time".to_string(),
                metric_name: "avg_response_time_ms".to_string(),
                condition: AlertCondition::GreaterThan,
                threshold: 200.0, // 200ms average response time
                severity: AlertSeverity::Warning,
                is_active: true,
                notify_channels: vec!["slack".to_string()],
            },
        ])
    }

    async fn store_alert(&self, _alert: &Alert) -> Result<()> {
        // This would store the alert in database
        Ok(())
    }

    async fn get_memory_usage(&self) -> Result<f64> {
        // Mock implementation - would integrate with system monitoring
        Ok(65.5)
    }

    async fn get_cpu_usage(&self) -> Result<f64> {
        // Mock implementation - would integrate with system monitoring
        Ok(25.8)
    }
}

// Helper structs for metrics
#[derive(Debug)]
struct RequestMetrics {
    total_requests: i64,
    avg_response_time: f64,
    error_rate: f64,
}

#[derive(Debug)]
struct DatabaseMetrics {
    active_connections: i32,
    idle_connections: i32,
    max_connections: i32,
}

#[derive(Debug)]
struct ResourceMetrics {
    memory_usage_mb: f64,
    cpu_usage_percent: f64,
}

#[derive(Debug)]
struct TenantRequestMetrics {
    total_requests: i64,
    error_count: i64,
    avg_response_time: f64,
}

#[derive(Debug)]
struct TenantUsageStats {
    storage_used_gb: f64,
    bandwidth_used_gb: f64,
}

#[derive(Debug)]
struct TenantPlatformStats {
    feature_flags_evaluated: i64,
    configurations_accessed: i64,
}