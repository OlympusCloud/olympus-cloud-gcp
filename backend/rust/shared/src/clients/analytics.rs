//! Analytics service client for Python integration

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use super::{HttpClient, HttpClientConfig, HttpClientError};
use crate::integration::python_analytics::{
    AnalyticsEvent, AnalyticsRequest, AnalyticsResponse,
    RecommendationRequest, RecommendationResponse,
};

/// Client for interacting with the Python Analytics Service
pub struct AnalyticsClient {
    http_client: HttpClient,
    service_url: String,
}

impl AnalyticsClient {
    pub fn new(config: HttpClientConfig) -> Result<Self, HttpClientError> {
        let service_url = config.base_url.clone();
        let http_client = HttpClient::new(config)?;

        Ok(Self {
            http_client,
            service_url,
        })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.http_client = self.http_client.with_token(token);
        self
    }

    /// Send an analytics event to the Python service
    pub async fn track_event(&self, event: AnalyticsEvent) -> Result<(), HttpClientError> {
        self.http_client
            .post::<AnalyticsEvent, serde_json::Value>("/api/analytics/events", &event)
            .await?;
        Ok(())
    }

    /// Batch send multiple analytics events
    pub async fn track_events_batch(&self, events: Vec<AnalyticsEvent>) -> Result<(), HttpClientError> {
        #[derive(Serialize)]
        struct BatchRequest {
            events: Vec<AnalyticsEvent>,
        }

        let request = BatchRequest { events };

        self.http_client
            .post::<BatchRequest, serde_json::Value>("/api/analytics/events/batch", &request)
            .await?;
        Ok(())
    }

    /// Get analytics data for a specific metric
    pub async fn get_analytics(&self, request: AnalyticsRequest) -> Result<AnalyticsResponse, HttpClientError> {
        self.http_client
            .post("/api/analytics/metrics", &request)
            .await
    }

    /// Get recommendations from the ML service
    pub async fn get_recommendations(&self, request: RecommendationRequest) -> Result<RecommendationResponse, HttpClientError> {
        self.http_client
            .post("/api/analytics/recommendations", &request)
            .await
    }

    /// Get dashboard data for a tenant
    pub async fn get_dashboard(&self, tenant_id: Uuid) -> Result<DashboardData, HttpClientError> {
        self.http_client
            .get(&format!("/api/analytics/dashboard/{}", tenant_id))
            .await
    }

    /// Get customer segments
    pub async fn get_customer_segments(&self, tenant_id: Uuid) -> Result<CustomerSegments, HttpClientError> {
        self.http_client
            .get(&format!("/api/analytics/customers/{}/segments", tenant_id))
            .await
    }

    /// Get inventory forecast
    pub async fn get_inventory_forecast(&self, tenant_id: Uuid, product_id: Uuid) -> Result<InventoryForecast, HttpClientError> {
        self.http_client
            .get(&format!("/api/analytics/inventory/{}/forecast/{}", tenant_id, product_id))
            .await
    }

    /// Health check for analytics service
    pub async fn health_check(&self) -> Result<bool, HttpClientError> {
        self.http_client.health_check().await
    }
}

// Response types specific to analytics service

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub tenant_id: Uuid,
    pub period: String,
    pub revenue: DashboardMetric,
    pub orders: DashboardMetric,
    pub customers: DashboardMetric,
    pub top_products: Vec<ProductMetric>,
    pub insights: Vec<DashboardInsight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetric {
    pub current: f64,
    pub previous: f64,
    pub change_percentage: f64,
    pub trend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductMetric {
    pub product_id: Uuid,
    pub name: String,
    pub quantity_sold: i32,
    pub revenue: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardInsight {
    pub title: String,
    pub description: String,
    pub impact: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegments {
    pub segments: Vec<CustomerSegment>,
    pub total_customers: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegment {
    pub segment_id: String,
    pub name: String,
    pub description: String,
    pub customer_count: u64,
    pub percentage: f64,
    pub avg_order_value: f64,
    pub characteristics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryForecast {
    pub product_id: Uuid,
    pub current_stock: i32,
    pub predicted_demand: Vec<DemandPrediction>,
    pub recommended_reorder_date: DateTime<Utc>,
    pub recommended_reorder_quantity: i32,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPrediction {
    pub date: DateTime<Utc>,
    pub predicted_quantity: i32,
    pub lower_bound: i32,
    pub upper_bound: i32,
}

/// Helper functions for creating analytics events
impl AnalyticsEvent {
    pub fn order_created(tenant_id: Uuid, order_id: Uuid, total: f64, user_id: Option<Uuid>) -> Self {
        let mut properties = HashMap::new();
        properties.insert("order_id".to_string(), serde_json::json!(order_id));

        let mut metrics = HashMap::new();
        metrics.insert("order_total".to_string(), total);

        Self {
            event_id: Uuid::new_v4(),
            event_type: "order_created".to_string(),
            tenant_id,
            user_id,
            location_id: None,
            properties,
            metrics,
            timestamp: Utc::now(),
        }
    }

    pub fn product_viewed(tenant_id: Uuid, product_id: Uuid, user_id: Option<Uuid>) -> Self {
        let mut properties = HashMap::new();
        properties.insert("product_id".to_string(), serde_json::json!(product_id));

        Self {
            event_id: Uuid::new_v4(),
            event_type: "product_viewed".to_string(),
            tenant_id,
            user_id,
            location_id: None,
            properties,
            metrics: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn customer_registered(tenant_id: Uuid, customer_id: Uuid) -> Self {
        let mut properties = HashMap::new();
        properties.insert("customer_id".to_string(), serde_json::json!(customer_id));

        Self {
            event_id: Uuid::new_v4(),
            event_type: "customer_registered".to_string(),
            tenant_id,
            user_id: Some(customer_id),
            location_id: None,
            properties,
            metrics: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analytics_event_creation() {
        let tenant_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();
        let event = AnalyticsEvent::order_created(tenant_id, order_id, 100.0, None);

        assert_eq!(event.event_type, "order_created");
        assert_eq!(event.tenant_id, tenant_id);
        assert_eq!(event.metrics["order_total"], 100.0);
    }
}