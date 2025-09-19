// ============================================================================
// OLYMPUS CLOUD - INVENTORY HANDLERS
// ============================================================================
// Module: commerce/src/handlers/inventory.rs
// Description: HTTP handlers for inventory management APIs
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::services::inventory::InventoryService;

// ============================================================================
// RESPONSE MODELS
// ============================================================================

#[derive(Debug, Serialize)]
pub struct StockLevelResponse {
    pub product_id: Uuid,
    pub variant_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub available: i32,
    pub reserved: i32,
    pub on_hand: i32,
    pub low_stock_threshold: Option<i32>,
    pub cost_per_unit: Option<rust_decimal::Decimal>,
    pub is_low_stock: bool,
    pub is_out_of_stock: bool,
}

// ============================================================================
// HTTP HANDLERS
// ============================================================================

/// Get stock level for a specific product
pub async fn get_stock_level(
    State(inventory_service): State<InventoryService>,
    Path((tenant_id, product_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<StockLevelResponse>, StatusCode> {
    let stock_level = inventory_service
        .get_stock_level(tenant_id, product_id, None, None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(level) = stock_level {
        let response = StockLevelResponse {
            product_id: level.product_id,
            variant_id: level.variant_id,
            location_id: level.location_id,
            available: level.available,
            reserved: level.reserved,
            on_hand: level.on_hand,
            low_stock_threshold: level.low_stock_threshold,
            cost_per_unit: level.cost_per_unit,
            is_low_stock: level.low_stock_threshold.map_or(false, |t| level.available <= t),
            is_out_of_stock: level.available == 0,
        };

        Ok(Json(response))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// ============================================================================
// ROUTER SETUP
// ============================================================================

pub fn inventory_routes() -> Router<InventoryService> {
    Router::new()
        .route("/tenants/:tenant_id/stock/:product_id", get(get_stock_level))
}