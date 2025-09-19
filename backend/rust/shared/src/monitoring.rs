//! Health checks and monitoring utilities

use axum::{extract::State, response::Json, routing::get, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub response_time_ms: Option<u64>,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: HealthStatus,
    pub version: String,
    pub service_name: String,
    pub uptime_seconds: u64,
    pub components: Vec<ComponentHealth>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub service_name: String,
    pub checks_passed: Vec<String>,
    pub checks_failed: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LivenessResponse {
    pub alive: bool,
    pub service_name: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub request_count: u64,
    pub error_count: u64,
    pub avg_response_time_ms: f64,
    pub p95_response_time_ms: f64,
    pub p99_response_time_ms: f64,
    pub active_connections: u32,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

pub struct HealthChecker {
    service_name: String,
    version: String,
    start_time: DateTime<Utc>,
    components: Arc<RwLock<HashMap<String, ComponentHealth>>>,
    db_pool: Option<PgPool>,
    redis_client: Option<redis::Client>,
}

impl HealthChecker {
    pub fn new(service_name: String, version: String) -> Self {
        Self {
            service_name,
            version,
            start_time: Utc::now(),
            components: Arc::new(RwLock::new(HashMap::new())),
            db_pool: None,
            redis_client: None,
        }
    }

    pub fn with_database(mut self, pool: PgPool) -> Self {
        self.db_pool = Some(pool);
        self
    }

    pub fn with_redis(mut self, client: redis::Client) -> Self {
        self.redis_client = Some(client);
        self
    }

    async fn check_database(&self) -> ComponentHealth {
        let mut health = ComponentHealth {
            name: "database".to_string(),
            status: HealthStatus::Unhealthy,
            message: None,
            response_time_ms: None,
            last_check: Utc::now(),
        };

        if let Some(pool) = &self.db_pool {
            let start = std::time::Instant::now();

            match sqlx::query("SELECT 1").fetch_one(pool).await {
                Ok(_) => {
                    health.status = HealthStatus::Healthy;
                    health.response_time_ms = Some(start.elapsed().as_millis() as u64);
                }
                Err(e) => {
                    error!("Database health check failed: {}", e);
                    health.message = Some(format!("Connection failed: {}", e));
                }
            }
        } else {
            health.message = Some("Database not configured".to_string());
        }

        health
    }

    async fn check_redis(&self) -> ComponentHealth {
        let mut health = ComponentHealth {
            name: "redis".to_string(),
            status: HealthStatus::Unhealthy,
            message: None,
            response_time_ms: None,
            last_check: Utc::now(),
        };

        if let Some(client) = &self.redis_client {
            let start = std::time::Instant::now();

            match client.get_tokio_connection().await {
                Ok(mut conn) => {
                    let pong: Result<String, _> = redis::cmd("PING")
                        .query_async(&mut conn)
                        .await;

                    match pong {
                        Ok(_) => {
                            health.status = HealthStatus::Healthy;
                            health.response_time_ms = Some(start.elapsed().as_millis() as u64);
                        }
                        Err(e) => {
                            error!("Redis ping failed: {}", e);
                            health.message = Some(format!("Ping failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    error!("Redis connection failed: {}", e);
                    health.message = Some(format!("Connection failed: {}", e));
                }
            }
        } else {
            health.message = Some("Redis not configured".to_string());
        }

        health
    }

    pub async fn check_health(&self) -> HealthResponse {
        let mut components = Vec::new();

        // Check database
        if self.db_pool.is_some() {
            components.push(self.check_database().await);
        }

        // Check Redis
        if self.redis_client.is_some() {
            components.push(self.check_redis().await);
        }

        // Add custom component checks
        let custom_components = self.components.read().await;
        components.extend(custom_components.values().cloned());

        // Determine overall status
        let overall_status = if components.iter().all(|c| matches!(c.status, HealthStatus::Healthy)) {
            HealthStatus::Healthy
        } else if components.iter().any(|c| matches!(c.status, HealthStatus::Unhealthy)) {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };

        let uptime = Utc::now()
            .signed_duration_since(self.start_time)
            .num_seconds() as u64;

        HealthResponse {
            status: overall_status,
            version: self.version.clone(),
            service_name: self.service_name.clone(),
            uptime_seconds: uptime,
            components,
            timestamp: Utc::now(),
        }
    }

    pub async fn check_readiness(&self) -> ReadinessResponse {
        let health = self.check_health().await;
        let mut checks_passed = Vec::new();
        let mut checks_failed = Vec::new();

        for component in &health.components {
            match component.status {
                HealthStatus::Healthy => checks_passed.push(component.name.clone()),
                _ => checks_failed.push(component.name.clone()),
            }
        }

        ReadinessResponse {
            ready: checks_failed.is_empty(),
            service_name: self.service_name.clone(),
            checks_passed,
            checks_failed,
            timestamp: Utc::now(),
        }
    }

    pub async fn check_liveness(&self) -> LivenessResponse {
        LivenessResponse {
            alive: true,
            service_name: self.service_name.clone(),
            timestamp: Utc::now(),
        }
    }

    pub async fn update_component_health(&self, name: String, health: ComponentHealth) {
        let mut components = self.components.write().await;
        components.insert(name, health);
    }
}

// Axum handlers
pub async fn health_handler(State(checker): State<Arc<HealthChecker>>) -> Json<HealthResponse> {
    Json(checker.check_health().await)
}

pub async fn readiness_handler(State(checker): State<Arc<HealthChecker>>) -> Json<ReadinessResponse> {
    Json(checker.check_readiness().await)
}

pub async fn liveness_handler(State(checker): State<Arc<HealthChecker>>) -> Json<LivenessResponse> {
    Json(checker.check_liveness().await)
}

// Create monitoring routes
pub fn monitoring_routes(health_checker: Arc<HealthChecker>) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/ready", get(readiness_handler))
        .route("/live", get(liveness_handler))
        .with_state(health_checker)
}

// Prometheus metrics
use prometheus::{Encoder, TextEncoder, Counter, Gauge, Histogram, HistogramOpts};
use prometheus::{register_counter, register_gauge, register_histogram};

lazy_static::lazy_static! {
    static ref HTTP_REQUESTS_TOTAL: Counter = register_counter!(
        "http_requests_total",
        "Total number of HTTP requests"
    ).unwrap();

    static ref HTTP_REQUEST_DURATION: Histogram = register_histogram!(
        HistogramOpts::new(
            "http_request_duration_seconds",
            "HTTP request duration in seconds"
        )
        .buckets(vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0])
    ).unwrap();

    static ref HTTP_REQUESTS_IN_FLIGHT: Gauge = register_gauge!(
        "http_requests_in_flight",
        "Number of HTTP requests currently being processed"
    ).unwrap();

    static ref DATABASE_CONNECTIONS_ACTIVE: Gauge = register_gauge!(
        "database_connections_active",
        "Number of active database connections"
    ).unwrap();

    static ref CACHE_HIT_RATE: Gauge = register_gauge!(
        "cache_hit_rate",
        "Cache hit rate percentage"
    ).unwrap();
}

pub fn collect_metrics() -> String {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

pub async fn metrics_handler() -> String {
    collect_metrics()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_checker_creation() {
        let checker = HealthChecker::new(
            "test-service".to_string(),
            "1.0.0".to_string(),
        );

        let health = checker.check_health().await;
        assert_eq!(health.service_name, "test-service");
        assert_eq!(health.version, "1.0.0");
        assert!(health.uptime_seconds >= 0);
    }

    #[tokio::test]
    async fn test_liveness_check() {
        let checker = HealthChecker::new(
            "test-service".to_string(),
            "1.0.0".to_string(),
        );

        let liveness = checker.check_liveness().await;
        assert!(liveness.alive);
        assert_eq!(liveness.service_name, "test-service");
    }
}