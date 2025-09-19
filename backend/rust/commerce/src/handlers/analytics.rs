// ============================================================================
// OLYMPUS CLOUD - COMMERCE ANALYTICS HANDLERS
// ============================================================================
// Module: commerce/src/handlers/analytics.rs
// Description: HTTP handlers for commerce analytics and reporting endpoints
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use olympus_shared::error::{OlympusError, Result};
use olympus_shared::validation::ValidatedJson;
use crate::services::analytics::{
    AnalyticsService, AnalyticsExportRequest, AnalyticsExportType, CustomerAnalyticsRequest,
    InventoryAnalyticsRequest, OrderAnalyticsRequest, ProductAnalyticsRequest,
    RevenueAnalyticsRequest, SalesAnalyticsRequest,
};

// ============================================================================
// ANALYTICS ROUTER
// ============================================================================

/// Create analytics router with all endpoints
pub fn create_analytics_router(analytics_service: Arc<AnalyticsService>) -> Router {
    Router::new()
        // Sales analytics
        .route(
            "/tenants/:tenant_id/analytics/sales",
            get(get_sales_analytics),
        )
        // Product analytics
        .route(
            "/tenants/:tenant_id/analytics/products",
            get(get_product_analytics),
        )
        // Order analytics
        .route(
            "/tenants/:tenant_id/analytics/orders",
            get(get_order_analytics),
        )
        // Revenue analytics
        .route(
            "/tenants/:tenant_id/analytics/revenue",
            get(get_revenue_analytics),
        )
        // Customer analytics
        .route(
            "/tenants/:tenant_id/analytics/customers",
            get(get_customer_analytics),
        )
        // Inventory analytics
        .route(
            "/tenants/:tenant_id/analytics/inventory",
            get(get_inventory_analytics),
        )
        // Comprehensive dashboard
        .route(
            "/tenants/:tenant_id/analytics/dashboard",
            get(get_analytics_dashboard),
        )
        // Export functionality
        .route(
            "/tenants/:tenant_id/analytics/export/:export_type",
            post(export_analytics_data),
        )
        // Cache refresh
        .route(
            "/tenants/:tenant_id/analytics/refresh",
            post(refresh_analytics_cache),
        )
        .with_state(analytics_service)
}

// ============================================================================
// SALES ANALYTICS HANDLERS
// ============================================================================

/// Get comprehensive sales performance metrics
#[axum::debug_handler]
pub async fn get_sales_analytics(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<SalesAnalyticsQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<SalesAnalyticsResponse>> {
    let request = SalesAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        location_filter: params.location,
        channel_filter: params.channel,
    };

    let metrics = service.get_sales_performance(tenant_id, &request).await?;

    Ok(Json(SalesAnalyticsResponse {
        success: true,
        data: metrics,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// PRODUCT ANALYTICS HANDLERS
// ============================================================================

/// Get product performance metrics
#[axum::debug_handler]
pub async fn get_product_analytics(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<ProductAnalyticsQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<ProductAnalyticsResponse>> {
    let request = ProductAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        category_filter: params.category_id,
        limit: params.limit,
    };

    let metrics = service.get_product_performance(tenant_id, &request).await?;

    Ok(Json(ProductAnalyticsResponse {
        success: true,
        data: metrics,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// ORDER ANALYTICS HANDLERS
// ============================================================================

/// Get order analytics and patterns
#[axum::debug_handler]
pub async fn get_order_analytics(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<OrderAnalyticsQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<OrderAnalyticsResponse>> {
    let request = OrderAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        status_filter: params.status,
    };

    let metrics = service.get_order_analytics(tenant_id, &request).await?;

    Ok(Json(OrderAnalyticsResponse {
        success: true,
        data: metrics,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// REVENUE ANALYTICS HANDLERS
// ============================================================================

/// Get comprehensive revenue analytics
#[axum::debug_handler]
pub async fn get_revenue_analytics(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<RevenueAnalyticsQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<RevenueAnalyticsResponse>> {
    let request = RevenueAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        group_by: params.group_by,
    };

    let metrics = service.get_revenue_analytics(tenant_id, &request).await?;

    Ok(Json(RevenueAnalyticsResponse {
        success: true,
        data: metrics,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// CUSTOMER ANALYTICS HANDLERS
// ============================================================================

/// Get customer analytics and segmentation
#[axum::debug_handler]
pub async fn get_customer_analytics(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<CustomerAnalyticsQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<CustomerAnalyticsResponse>> {
    let request = CustomerAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        segment_filter: params.segment,
    };

    let metrics = service.get_customer_analytics(tenant_id, &request).await?;

    Ok(Json(CustomerAnalyticsResponse {
        success: true,
        data: metrics,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// INVENTORY ANALYTICS HANDLERS
// ============================================================================

/// Get inventory analytics and stock analysis
#[axum::debug_handler]
pub async fn get_inventory_analytics(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<InventoryAnalyticsQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<InventoryAnalyticsResponse>> {
    let request = InventoryAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        location_filter: params.location,
    };

    let metrics = service.get_inventory_analytics(tenant_id, &request).await?;

    Ok(Json(InventoryAnalyticsResponse {
        success: true,
        data: metrics,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// DASHBOARD HANDLER
// ============================================================================

/// Get comprehensive analytics dashboard with all key metrics
#[axum::debug_handler]
pub async fn get_analytics_dashboard(
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<DashboardQueryParams>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<DashboardResponse>> {
    // Create common request parameters
    let sales_request = SalesAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        location_filter: None,
        channel_filter: None,
    };

    let product_request = ProductAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        category_filter: None,
        limit: Some(10), // Top 10 for dashboard
    };

    let order_request = OrderAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        status_filter: None,
    };

    let revenue_request = RevenueAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        group_by: None,
    };

    let customer_request = CustomerAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        segment_filter: None,
    };

    let inventory_request = InventoryAnalyticsRequest {
        start_date: params.start_date,
        end_date: params.end_date,
        location_filter: None,
    };

    // Fetch all metrics concurrently
    let (sales, products, orders, revenue, customers, inventory) = tokio::try_join!(
        service.get_sales_performance(tenant_id, &sales_request),
        service.get_product_performance(tenant_id, &product_request),
        service.get_order_analytics(tenant_id, &order_request),
        service.get_revenue_analytics(tenant_id, &revenue_request),
        service.get_customer_analytics(tenant_id, &customer_request),
        service.get_inventory_analytics(tenant_id, &inventory_request),
    )?;

    let dashboard = DashboardData {
        sales_overview: DashboardSalesOverview {
            total_sales: sales.total_sales,
            total_orders: sales.total_orders,
            average_order_value: sales.average_order_value,
            growth_rate: sales.growth_rate,
        },
        top_products: products.best_sellers.into_iter().take(5).collect(),
        order_summary: DashboardOrderSummary {
            total_orders: orders.total_orders,
            completion_rate: orders.completion_rate,
            average_processing_hours: orders.average_processing_hours,
        },
        revenue_summary: DashboardRevenueSummary {
            gross_revenue: revenue.gross_revenue,
            net_revenue: revenue.net_revenue,
            total_refunds: revenue.total_refunds,
        },
        customer_insights: DashboardCustomerInsights {
            total_customers: customers.total_customers,
            new_customers: customers.new_customers,
            retention_rate: customers.retention_rate,
            average_lifetime_value: customers.average_lifetime_value,
        },
        inventory_status: DashboardInventoryStatus {
            total_products: inventory.total_products,
            low_stock_items: inventory.low_stock_items,
            out_of_stock_items: inventory.out_of_stock_items,
            total_inventory_value: inventory.total_inventory_value,
        },
    };

    Ok(Json(DashboardResponse {
        success: true,
        data: dashboard,
        metadata: ResponseMetadata {
            generated_at: Utc::now(),
            tenant_id,
            period: format!(
                "{} to {}",
                params
                    .start_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "beginning".to_string()),
                params
                    .end_date
                    .map(|d| d.format("%Y-%m-%d").to_string())
                    .unwrap_or_else(|| "now".to_string())
            ),
        },
    }))
}

// ============================================================================
// EXPORT HANDLER
// ============================================================================

/// Export analytics data in various formats
#[axum::debug_handler]
pub async fn export_analytics_data(
    Path((tenant_id, export_type)): Path<(Uuid, String)>,
    State(service): State<Arc<AnalyticsService>>,
    ValidatedJson(request): ValidatedJson<AnalyticsExportRequest>,
) -> Result<String> {
    let export_type = match export_type.as_str() {
        "sales" => AnalyticsExportType::Sales,
        "products" => AnalyticsExportType::Products,
        "orders" => AnalyticsExportType::Orders,
        "revenue" => AnalyticsExportType::Revenue,
        "customers" => AnalyticsExportType::Customers,
        "inventory" => AnalyticsExportType::Inventory,
        _ => {
            return Err(OlympusError::ValidationError(format!(
                "Invalid export type: {}. Valid types: sales, products, orders, revenue, customers, inventory",
                export_type
            )));
        }
    };

    let csv_data = service
        .export_analytics_csv(tenant_id, export_type, request)
        .await?;

    Ok(csv_data)
}

// ============================================================================
// CACHE REFRESH HANDLER
// ============================================================================

/// Refresh analytics cache for real-time dashboards
#[axum::debug_handler]
pub async fn refresh_analytics_cache(
    Path(tenant_id): Path<Uuid>,
    State(service): State<Arc<AnalyticsService>>,
) -> Result<Json<CacheRefreshResponse>> {
    service.cache_analytics_metrics(tenant_id).await?;

    Ok(Json(CacheRefreshResponse {
        success: true,
        message: "Analytics cache refreshed successfully".to_string(),
        refreshed_at: Utc::now(),
    }))
}

// ============================================================================
// QUERY PARAMETER MODELS
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct SalesAnalyticsQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub channel: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ProductAnalyticsQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category_id: Option<Uuid>,
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct OrderAnalyticsQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<crate::models::OrderStatus>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RevenueAnalyticsQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub group_by: Option<crate::services::analytics::RevenueGroupBy>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CustomerAnalyticsQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub segment: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct InventoryAnalyticsQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DashboardQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

// ============================================================================
// RESPONSE MODELS
// ============================================================================

#[derive(Debug, Serialize)]
pub struct SalesAnalyticsResponse {
    pub success: bool,
    pub data: crate::services::analytics::SalesPerformanceMetrics,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct ProductAnalyticsResponse {
    pub success: bool,
    pub data: crate::services::analytics::ProductPerformanceMetrics,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct OrderAnalyticsResponse {
    pub success: bool,
    pub data: crate::services::analytics::OrderAnalyticsMetrics,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct RevenueAnalyticsResponse {
    pub success: bool,
    pub data: crate::services::analytics::RevenueAnalyticsMetrics,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct CustomerAnalyticsResponse {
    pub success: bool,
    pub data: crate::services::analytics::CustomerAnalyticsMetrics,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct InventoryAnalyticsResponse {
    pub success: bool,
    pub data: crate::services::analytics::InventoryAnalyticsMetrics,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponse {
    pub success: bool,
    pub data: DashboardData,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct CacheRefreshResponse {
    pub success: bool,
    pub message: String,
    pub refreshed_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    pub generated_at: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub period: String,
}

// ============================================================================
// DASHBOARD MODELS
// ============================================================================

#[derive(Debug, Serialize)]
pub struct DashboardData {
    pub sales_overview: DashboardSalesOverview,
    pub top_products: Vec<crate::services::analytics::ProductSalesMetric>,
    pub order_summary: DashboardOrderSummary,
    pub revenue_summary: DashboardRevenueSummary,
    pub customer_insights: DashboardCustomerInsights,
    pub inventory_status: DashboardInventoryStatus,
}

#[derive(Debug, Serialize)]
pub struct DashboardSalesOverview {
    pub total_sales: rust_decimal::Decimal,
    pub total_orders: i32,
    pub average_order_value: rust_decimal::Decimal,
    pub growth_rate: rust_decimal::Decimal,
}

#[derive(Debug, Serialize)]
pub struct DashboardOrderSummary {
    pub total_orders: i32,
    pub completion_rate: f64,
    pub average_processing_hours: rust_decimal::Decimal,
}

#[derive(Debug, Serialize)]
pub struct DashboardRevenueSummary {
    pub gross_revenue: rust_decimal::Decimal,
    pub net_revenue: rust_decimal::Decimal,
    pub total_refunds: rust_decimal::Decimal,
}

#[derive(Debug, Serialize)]
pub struct DashboardCustomerInsights {
    pub total_customers: i32,
    pub new_customers: i32,
    pub retention_rate: f64,
    pub average_lifetime_value: rust_decimal::Decimal,
}

#[derive(Debug, Serialize)]
pub struct DashboardInventoryStatus {
    pub total_products: i32,
    pub low_stock_items: i32,
    pub out_of_stock_items: i32,
    pub total_inventory_value: rust_decimal::Decimal,
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

impl From<validator::ValidationErrors> for OlympusError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let error_messages: Vec<String> = errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |error| {
                    format!(
                        "Field '{}': {}",
                        field,
                        error.message.as_ref().unwrap_or(&"Invalid value".into())
                    )
                })
            })
            .collect();

        OlympusError::ValidationError(error_messages.join(", "))
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Validate date range parameters
fn validate_date_range(start_date: Option<DateTime<Utc>>, end_date: Option<DateTime<Utc>>) -> Result<()> {
    if let (Some(start), Some(end)) = (start_date, end_date) {
        if start > end {
            return Err(OlympusError::ValidationError(
                "Start date must be before end date".to_string(),
            ));
        }

        let max_range = chrono::Duration::days(365); // Maximum 1 year range
        if end.signed_duration_since(start) > max_range {
            return Err(OlympusError::ValidationError(
                "Date range cannot exceed 365 days".to_string(),
            ));
        }
    }
    Ok(())
}

/// Apply common query parameter validations
pub async fn validate_analytics_params(
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<()> {
    validate_date_range(start_date, end_date)?;
    Ok(())
}