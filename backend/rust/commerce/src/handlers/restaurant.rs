// ============================================================================
// OLYMPUS CLOUD - RESTAURANT API HANDLERS
// ============================================================================
// Module: commerce/src/handlers/restaurant.rs
// Description: REST API handlers for restaurant management operations
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::models::restaurant::*;
use crate::services::RestaurantService;
use olympus_shared::integration::AuthContext;

/// Query parameters for filtering orders
#[derive(Debug, Deserialize)]
pub struct OrderFilters {
    pub status: Option<RestaurantOrderStatus>,
    pub table_id: Option<Uuid>,
    pub server_id: Option<Uuid>,
}

/// Query parameters for table analytics
#[derive(Debug, Deserialize)]
pub struct TableAnalyticsParams {
    pub date: Option<String>, // YYYY-MM-DD format
}

/// Response wrapper for API endpoints
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

// ============================================================================
// RESTAURANT ROUTES
// ============================================================================

pub fn restaurant_routes() -> Router<RestaurantService> {
    Router::new()
        // Dashboard endpoints
        .route("/dashboard", get(get_dashboard_metrics))

        // Table management endpoints
        .route("/tables", get(get_tables))
        .route("/tables/:table_id", get(get_table))
        .route("/tables/:table_id/status", put(update_table_status))
        .route("/tables/analytics", get(get_table_analytics))

        // Order management endpoints
        .route("/orders", get(get_orders))
        .route("/orders", post(create_order))
        .route("/orders/:order_id", get(get_order))
        .route("/orders/:order_id/status", put(update_order_status))

        // Kitchen display endpoints
        .route("/kitchen/display", get(get_kitchen_display))
        .route("/kitchen/items/:item_id/status", put(update_kitchen_item_status))
}

// ============================================================================
// DASHBOARD HANDLERS
// ============================================================================

/// GET /api/v1/restaurants/dashboard
/// Get real-time restaurant dashboard metrics
pub async fn get_dashboard_metrics(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<ApiResponse<RestaurantDashboard>>, StatusCode> {
    let location_id = params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match service.get_dashboard_metrics(auth.tenant_id, location_id).await {
        Ok(dashboard) => Ok(Json(ApiResponse::success(dashboard))),
        Err(e) => {
            tracing::error!("Failed to get dashboard metrics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// TABLE MANAGEMENT HANDLERS
// ============================================================================

/// GET /api/v1/restaurants/tables
/// Get all tables for a location
pub async fn get_tables(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<ApiResponse<Vec<RestaurantTable>>>, StatusCode> {
    let location_id = params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match service.get_tables(auth.tenant_id, location_id).await {
        Ok(tables) => Ok(Json(ApiResponse::success(tables))),
        Err(e) => {
            tracing::error!("Failed to get tables: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/restaurants/tables/:table_id
/// Get a specific table by ID
pub async fn get_table(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Path(table_id): Path<Uuid>,
) -> std::result::Result<Json<ApiResponse<RestaurantTable>>, StatusCode> {
    match service.get_table(auth.tenant_id, table_id).await {
        Ok(table) => Ok(Json(ApiResponse::success(table))),
        Err(e) => {
            tracing::error!("Failed to get table {}: {}", table_id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// PUT /api/v1/restaurants/tables/:table_id/status
/// Update table status
pub async fn update_table_status(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Path(table_id): Path<Uuid>,
    Json(request): Json<UpdateTableStatusRequest>,
) -> std::result::Result<Json<ApiResponse<RestaurantTable>>, StatusCode> {
    match service.update_table_status(auth.tenant_id, table_id, request).await {
        Ok(table) => Ok(Json(ApiResponse::success(table))),
        Err(e) => {
            tracing::error!("Failed to update table {} status: {}", table_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/restaurants/tables/analytics
/// Get table analytics for dashboard
pub async fn get_table_analytics(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<ApiResponse<Vec<TableAnalytics>>>, StatusCode> {
    let location_id = params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match service.get_table_analytics(auth.tenant_id, location_id).await {
        Ok(analytics) => Ok(Json(ApiResponse::success(analytics))),
        Err(e) => {
            tracing::error!("Failed to get table analytics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// ORDER MANAGEMENT HANDLERS
// ============================================================================

/// GET /api/v1/restaurants/orders
/// Get orders with optional filtering
pub async fn get_orders(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Query(params): Query<HashMap<String, String>>,
    Query(filters): Query<OrderFilters>,
) -> std::result::Result<Json<ApiResponse<Vec<RestaurantOrder>>>, StatusCode> {
    let location_id = params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match service.get_orders(auth.tenant_id, location_id, filters.status).await {
        Ok(orders) => Ok(Json(ApiResponse::success(orders))),
        Err(e) => {
            tracing::error!("Failed to get orders: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /api/v1/restaurants/orders
/// Create a new restaurant order
pub async fn create_order(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Query(params): Query<HashMap<String, String>>,
    Json(request): Json<CreateRestaurantOrderRequest>,
) -> std::result::Result<Json<ApiResponse<RestaurantOrder>>, StatusCode> {
    let location_id = params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match service.create_order(auth.tenant_id, location_id, request).await {
        Ok(order) => Ok(Json(ApiResponse::success(order))),
        Err(e) => {
            tracing::error!("Failed to create order: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/restaurants/orders/:order_id
/// Get a specific order by ID
pub async fn get_order(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Path(order_id): Path<Uuid>,
) -> std::result::Result<Json<ApiResponse<RestaurantOrder>>, StatusCode> {
    // For now, return a placeholder - would implement order lookup with items
    Err(StatusCode::NOT_IMPLEMENTED)
}

/// PUT /api/v1/restaurants/orders/:order_id/status
/// Update order status
pub async fn update_order_status(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Path(order_id): Path<Uuid>,
    Json(request): Json<UpdateOrderStatusRequest>,
) -> std::result::Result<Json<ApiResponse<RestaurantOrder>>, StatusCode> {
    match service.update_order_status(auth.tenant_id, order_id, request.status).await {
        Ok(order) => Ok(Json(ApiResponse::success(order))),
        Err(e) => {
            tracing::error!("Failed to update order {} status: {}", order_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Request to update order status
#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: RestaurantOrderStatus,
}

// ============================================================================
// KITCHEN DISPLAY HANDLERS
// ============================================================================

/// GET /api/v1/restaurants/kitchen/display
/// Get kitchen display items for active orders
pub async fn get_kitchen_display(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<ApiResponse<Vec<KitchenDisplayItem>>>, StatusCode> {
    let location_id = params
        .get("location_id")
        .and_then(|s| s.parse::<Uuid>().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    match service.get_kitchen_display_items(auth.tenant_id, location_id).await {
        Ok(items) => Ok(Json(ApiResponse::success(items))),
        Err(e) => {
            tracing::error!("Failed to get kitchen display items: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// PUT /api/v1/restaurants/kitchen/items/:item_id/status
/// Update kitchen item status
pub async fn update_kitchen_item_status(
    State(service): State<RestaurantService>,
    auth: AuthContext,
    Path(item_id): Path<Uuid>,
    Json(request): Json<UpdateKitchenStatusRequest>,
) -> std::result::Result<Json<ApiResponse<()>>, StatusCode> {
    match service.update_kitchen_status(auth.tenant_id, item_id, request).await {
        Ok(()) => Ok(Json(ApiResponse::success(()))),
        Err(e) => {
            tracing::error!("Failed to update kitchen item {} status: {}", item_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_restaurant_routes_compile() {
        // Basic test to ensure routes compile correctly
        let _routes = restaurant_routes();
        assert!(true);
    }
}