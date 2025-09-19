//! Integration utilities for connecting with Go API Gateway and Python Analytics Service

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Standard request/response format for inter-service communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest<T> {
    pub request_id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse<T> {
    pub request_id: Uuid,
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ServiceError>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Go API Gateway integration types
pub mod go_gateway {
    use super::*;

    /// Authentication context passed from Go API Gateway
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuthContext {
        pub user_id: Uuid,
        pub tenant_id: Uuid,
        pub roles: Vec<String>,
        pub permissions: Vec<String>,
        pub session_id: String,
        pub expires_at: DateTime<Utc>,
    }

    /// Request wrapper from Go API Gateway
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GatewayRequest<T> {
        pub auth: AuthContext,
        pub trace_id: String,
        pub client_ip: String,
        pub user_agent: String,
        pub data: T,
    }

    /// Response to Go API Gateway
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GatewayResponse<T> {
        pub status: u16,
        pub data: Option<T>,
        pub error: Option<GatewayError>,
        pub metadata: ResponseMetadata,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct GatewayError {
        pub error_type: String,
        pub message: String,
        pub field_errors: Option<Vec<FieldError>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct FieldError {
        pub field: String,
        pub message: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseMetadata {
        pub request_id: Uuid,
        pub processing_time_ms: u64,
        pub cache_hit: bool,
        pub version: String,
    }
}

/// Python Analytics Service integration types
pub mod python_analytics {
    use super::*;
    use rust_decimal::Decimal;
    use std::collections::HashMap;

    /// Analytics event to send to Python service
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnalyticsEvent {
        pub event_id: Uuid,
        pub event_type: String,
        pub tenant_id: Uuid,
        pub user_id: Option<Uuid>,
        pub location_id: Option<Uuid>,
        pub properties: HashMap<String, serde_json::Value>,
        pub metrics: HashMap<String, f64>,
        pub timestamp: DateTime<Utc>,
    }

    /// Request for analytics data
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnalyticsRequest {
        pub tenant_id: Uuid,
        pub metric_type: MetricType,
        pub date_range: DateRange,
        pub filters: HashMap<String, String>,
        pub group_by: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum MetricType {
        Revenue,
        Orders,
        Customers,
        Inventory,
        Performance,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DateRange {
        pub start: DateTime<Utc>,
        pub end: DateTime<Utc>,
    }

    /// Analytics response from Python service
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AnalyticsResponse {
        pub data: Vec<MetricData>,
        pub summary: MetricSummary,
        pub insights: Vec<Insight>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MetricData {
        pub timestamp: DateTime<Utc>,
        pub value: f64,
        pub dimensions: HashMap<String, String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct MetricSummary {
        pub total: f64,
        pub average: f64,
        pub min: f64,
        pub max: f64,
        pub count: u64,
        pub trend: TrendDirection,
        pub change_percentage: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum TrendDirection {
        Up,
        Down,
        Stable,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Insight {
        pub insight_type: String,
        pub title: String,
        pub description: String,
        pub impact: ImpactLevel,
        pub recommendations: Vec<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum ImpactLevel {
        Low,
        Medium,
        High,
        Critical,
    }

    /// Recommendation request
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RecommendationRequest {
        pub tenant_id: Uuid,
        pub context_type: String,
        pub entity_id: Option<Uuid>,
        pub user_preferences: HashMap<String, serde_json::Value>,
    }

    /// Recommendation response
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RecommendationResponse {
        pub recommendations: Vec<Recommendation>,
        pub model_version: String,
        pub confidence_score: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Recommendation {
        pub item_id: Uuid,
        pub item_type: String,
        pub score: f64,
        pub reason: String,
        pub metadata: serde_json::Value,
    }
}

/// Common error codes for inter-service communication
pub mod error_codes {
    pub const INVALID_REQUEST: &str = "INVALID_REQUEST";
    pub const UNAUTHORIZED: &str = "UNAUTHORIZED";
    pub const FORBIDDEN: &str = "FORBIDDEN";
    pub const NOT_FOUND: &str = "NOT_FOUND";
    pub const CONFLICT: &str = "CONFLICT";
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    pub const SERVICE_UNAVAILABLE: &str = "SERVICE_UNAVAILABLE";
    pub const TIMEOUT: &str = "TIMEOUT";
    pub const RATE_LIMITED: &str = "RATE_LIMITED";
}

/// Helper functions for creating standard responses
impl<T> ServiceResponse<T> {
    pub fn success(request_id: Uuid, data: T) -> Self {
        Self {
            request_id,
            success: true,
            data: Some(data),
            error: None,
            timestamp: Utc::now(),
        }
    }

    pub fn error(request_id: Uuid, code: &str, message: String) -> Self {
        Self {
            request_id,
            success: false,
            data: None,
            error: Some(ServiceError {
                code: code.to_string(),
                message,
                details: None,
            }),
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::go_gateway::*;
    use super::python_analytics::*;

    #[test]
    fn test_service_response_creation() {
        let request_id = Uuid::new_v4();
        let response = ServiceResponse::success(request_id, "test_data");

        assert!(response.success);
        assert_eq!(response.data, Some("test_data"));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_analytics_event_serialization() {
        let event = AnalyticsEvent {
            event_id: Uuid::new_v4(),
            event_type: "order_created".to_string(),
            tenant_id: Uuid::new_v4(),
            user_id: Some(Uuid::new_v4()),
            location_id: None,
            properties: HashMap::new(),
            metrics: HashMap::new(),
            timestamp: Utc::now(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("order_created"));
    }
}