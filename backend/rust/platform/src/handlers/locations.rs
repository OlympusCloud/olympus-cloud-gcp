// ============================================================================
// OLYMPUS CLOUD - LOCATION HANDLERS
// ============================================================================
// Module: platform/src/handlers/locations.rs
// Description: HTTP handlers for location management APIs
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use olympus_shared::error::{Result, OlympusError};
use crate::models::{
    Location, LocationHierarchy, LocationAssignment, LocationStatistics,
    LocationSearchRequest, LocationSearchResponse, CreateLocationRequest,
    UpdateLocationRequest, CreateLocationAssignmentRequest,
};
use crate::services::LocationService;

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

pub fn create_location_router(location_service: Arc<LocationService>) -> Router {
    Router::new()
        // Location CRUD operations
        .route("/locations", post(create_location))
        .route("/locations", get(list_locations))
        .route("/locations/:location_id", get(get_location))
        .route("/locations/:location_id", put(update_location))
        .route("/locations/:location_id", delete(delete_location))

        // Location hierarchy operations
        .route("/locations/hierarchy", get(get_location_hierarchy))
        .route("/locations/:location_id/hierarchy", get(get_location_subtree))

        // Location assignment operations
        .route("/locations/:location_id/assignments", post(assign_user_to_location))
        .route("/locations/:location_id/assignments", get(get_location_assignments))
        .route("/locations/assignments/:assignment_id", delete(remove_location_assignment))

        // Location analytics and search
        .route("/locations/search", post(search_locations))
        .route("/locations/:location_id/statistics", get(get_location_statistics))
        .route("/locations/:location_id/utilization", get(get_location_utilization))

        .with_state(location_service)
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationResponse {
    pub success: bool,
    pub data: Location,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationListResponse {
    pub success: bool,
    pub data: Vec<Location>,
    pub total_count: i64,
    pub has_more: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationHierarchyResponse {
    pub success: bool,
    pub data: Vec<LocationHierarchy>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationAssignmentResponse {
    pub success: bool,
    pub data: LocationAssignment,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationStatisticsResponse {
    pub success: bool,
    pub data: LocationStatistics,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct LocationListQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub status: Option<String>,
    pub location_type: Option<String>,
    pub parent_location_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct HierarchyQuery {
    pub root_location_id: Option<Uuid>,
    pub max_depth: Option<i32>,
}

// ============================================================================
// LOCATION CRUD HANDLERS
// ============================================================================

pub async fn create_location(
    State(location_service): State<Arc<LocationService>>,
    Json(request): Json<CreateLocationRequest>,
) -> Result<Json<LocationResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| OlympusError::Validation(format!("Invalid request: {}", e)))?;

    // Extract tenant and user context (in real implementation, these would come from middleware)
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let created_by = Uuid::new_v4(); // Mock user ID

    let location = location_service
        .create_location(tenant_id, request, created_by)
        .await?;

    Ok(Json(LocationResponse {
        success: true,
        data: location,
        message: "Location created successfully".to_string(),
    }))
}

pub async fn get_location(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
) -> Result<Json<LocationResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let location = location_service
        .get_location(tenant_id, location_id)
        .await?
        .ok_or_else(|| OlympusError::NotFound("Location not found".to_string()))?;

    Ok(Json(LocationResponse {
        success: true,
        data: location,
        message: "Location retrieved successfully".to_string(),
    }))
}

pub async fn list_locations(
    State(location_service): State<Arc<LocationService>>,
    Query(query): Query<LocationListQuery>,
) -> Result<Json<LocationListResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let search_request = LocationSearchRequest {
        query: None,
        location_type: None, // Would parse from query.location_type
        status: None,        // Would parse from query.status
        parent_location_id: query.parent_location_id,
        manager_user_id: None,
        city: None,
        state_province: None,
        country: None,
        radius_km: None,
        center_lat: None,
        center_lng: None,
        limit: query.limit,
        offset: query.offset,
    };

    let response = location_service
        .search_locations(tenant_id, search_request)
        .await?;

    Ok(Json(LocationListResponse {
        success: true,
        data: response.locations,
        total_count: response.total_count,
        has_more: response.has_more,
        message: "Locations retrieved successfully".to_string(),
    }))
}

pub async fn update_location(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
    Json(request): Json<UpdateLocationRequest>,
) -> Result<Json<LocationResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| OlympusError::Validation(format!("Invalid request: {}", e)))?;

    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let updated_by = Uuid::new_v4(); // Mock user ID

    let location = location_service
        .update_location(tenant_id, location_id, request, updated_by)
        .await?
        .ok_or_else(|| OlympusError::NotFound("Location not found".to_string()))?;

    Ok(Json(LocationResponse {
        success: true,
        data: location,
        message: "Location updated successfully".to_string(),
    }))
}

pub async fn delete_location(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
) -> Result<StatusCode> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let deleted_by = Uuid::new_v4(); // Mock user ID

    let deleted = location_service
        .delete_location(tenant_id, location_id, deleted_by)
        .await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(OlympusError::NotFound("Location not found".to_string()))
    }
}

// ============================================================================
// LOCATION HIERARCHY HANDLERS
// ============================================================================

pub async fn get_location_hierarchy(
    State(location_service): State<Arc<LocationService>>,
    Query(query): Query<HierarchyQuery>,
) -> Result<Json<LocationHierarchyResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let hierarchy = location_service
        .get_location_hierarchy(tenant_id, query.root_location_id)
        .await?;

    Ok(Json(LocationHierarchyResponse {
        success: true,
        data: hierarchy,
        message: "Location hierarchy retrieved successfully".to_string(),
    }))
}

pub async fn get_location_subtree(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
    Query(query): Query<HierarchyQuery>,
) -> Result<Json<LocationHierarchyResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let hierarchy = location_service
        .get_location_hierarchy(tenant_id, Some(location_id))
        .await?;

    Ok(Json(LocationHierarchyResponse {
        success: true,
        data: hierarchy,
        message: "Location subtree retrieved successfully".to_string(),
    }))
}

// ============================================================================
// LOCATION ASSIGNMENT HANDLERS
// ============================================================================

pub async fn assign_user_to_location(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
    Json(request): Json<CreateLocationAssignmentRequest>,
) -> Result<Json<LocationAssignmentResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| OlympusError::Validation(format!("Invalid request: {}", e)))?;

    // Ensure location_id matches path parameter
    if request.location_id != location_id {
        return Err(OlympusError::Validation(
            "Location ID in path does not match request body".to_string(),
        ));
    }

    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let assigned_by = Uuid::new_v4(); // Mock user ID

    let assignment = location_service
        .assign_user_to_location(tenant_id, request, assigned_by)
        .await?;

    Ok(Json(LocationAssignmentResponse {
        success: true,
        data: assignment,
        message: "User assigned to location successfully".to_string(),
    }))
}

pub async fn get_location_assignments(
    State(_location_service): State<Arc<LocationService>>,
    Path(_location_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // Simplified implementation - would query location_assignments table
    Ok(Json(serde_json::json!({
        "success": true,
        "data": [],
        "message": "Location assignments retrieved successfully"
    })))
}

pub async fn remove_location_assignment(
    State(_location_service): State<Arc<LocationService>>,
    Path(_assignment_id): Path<Uuid>,
) -> Result<StatusCode> {
    // Simplified implementation - would delete from location_assignments table
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// SEARCH AND ANALYTICS HANDLERS
// ============================================================================

pub async fn search_locations(
    State(location_service): State<Arc<LocationService>>,
    Json(request): Json<LocationSearchRequest>,
) -> Result<Json<serde_json::Value>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let response = location_service
        .search_locations(tenant_id, request)
        .await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": response,
        "message": "Location search completed successfully"
    })))
}

pub async fn get_location_statistics(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
) -> Result<Json<LocationStatisticsResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let statistics = location_service
        .get_location_statistics(tenant_id, location_id)
        .await?
        .ok_or_else(|| OlympusError::NotFound("Location not found".to_string()))?;

    Ok(Json(LocationStatisticsResponse {
        success: true,
        data: statistics,
        message: "Location statistics retrieved successfully".to_string(),
    }))
}

pub async fn get_location_utilization(
    State(location_service): State<Arc<LocationService>>,
    Path(location_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    // Get location statistics which includes capacity utilization
    let statistics = location_service
        .get_location_statistics(tenant_id, location_id)
        .await?
        .ok_or_else(|| OlympusError::NotFound("Location not found".to_string()))?;

    let utilization_data = serde_json::json!({
        "location_id": location_id,
        "capacity_utilization": statistics.capacity_utilization,
        "active_users": statistics.active_users,
        "total_users": statistics.total_users,
        "utilization_status": match statistics.capacity_utilization {
            Some(util) if util >= 90.0 => "high",
            Some(util) if util >= 70.0 => "medium",
            Some(util) if util >= 50.0 => "normal",
            Some(_) => "low",
            None => "unknown"
        }
    });

    Ok(Json(serde_json::json!({
        "success": true,
        "data": utilization_data,
        "message": "Location utilization retrieved successfully"
    })))
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

impl axum::response::IntoResponse for OlympusError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            OlympusError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            OlympusError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            OlympusError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        let body = Json(serde_json::json!({
            "success": false,
            "error": error_message
        }));

        (status, body).into_response()
    }
}