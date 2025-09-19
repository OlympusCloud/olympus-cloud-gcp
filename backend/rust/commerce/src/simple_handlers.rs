use std::sync::Arc;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::simple_models::*;
use crate::simple_service::SimpleCommerceService;
use olympus_shared::error::Error;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ListOrdersQuery {
    pub user_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// Product handlers
pub async fn create_product(
    State(service): State<Arc<SimpleCommerceService>>,
    Path(tenant_id): Path<Uuid>,
    Json(request): Json<CreateProductRequest>,
) -> std::result::Result<Json<ProductResponse>, (StatusCode, String)> {
    match service.create_product(tenant_id, request).await {
        Ok(product) => Ok(Json(ProductResponse { product })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_product(
    State(service): State<Arc<SimpleCommerceService>>,
    Path((tenant_id, product_id)): Path<(Uuid, Uuid)>,
) -> std::result::Result<Json<ProductResponse>, (StatusCode, String)> {
    match service.get_product(tenant_id, product_id).await {
        Ok(product) => Ok(Json(ProductResponse { product })),
        Err(Error::NotFound(_)) => Err((StatusCode::NOT_FOUND, "Product not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_products(
    State(service): State<Arc<SimpleCommerceService>>,
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<PaginationQuery>,
) -> std::result::Result<Json<ProductsResponse>, (StatusCode, String)> {
    match service.list_products(tenant_id, params.limit, params.offset).await {
        Ok((products, total)) => Ok(Json(ProductsResponse { products, total })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_product(
    State(service): State<Arc<SimpleCommerceService>>,
    Path((tenant_id, product_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateProductRequest>,
) -> std::result::Result<Json<ProductResponse>, (StatusCode, String)> {
    match service.update_product(tenant_id, product_id, request).await {
        Ok(product) => Ok(Json(ProductResponse { product })),
        Err(Error::NotFound(_)) => Err((StatusCode::NOT_FOUND, "Product not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

// Order handlers
pub async fn create_order(
    State(service): State<Arc<SimpleCommerceService>>,
    Path(tenant_id): Path<Uuid>,
    Json(request): Json<CreateOrderRequest>,
) -> std::result::Result<Json<OrderResponse>, (StatusCode, String)> {
    match service.create_order(tenant_id, request).await {
        Ok(order) => Ok(Json(OrderResponse { order })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_order(
    State(service): State<Arc<SimpleCommerceService>>,
    Path((tenant_id, order_id)): Path<(Uuid, Uuid)>,
) -> std::result::Result<Json<OrderResponse>, (StatusCode, String)> {
    match service.get_order(tenant_id, order_id).await {
        Ok(order) => Ok(Json(OrderResponse { order })),
        Err(Error::NotFound(_)) => Err((StatusCode::NOT_FOUND, "Order not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_orders(
    State(service): State<Arc<SimpleCommerceService>>,
    Path(tenant_id): Path<Uuid>,
    Query(params): Query<ListOrdersQuery>,
) -> std::result::Result<Json<OrdersResponse>, (StatusCode, String)> {
    match service.list_orders(tenant_id, params.user_id, params.limit, params.offset).await {
        Ok((orders, total)) => Ok(Json(OrdersResponse { orders, total })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_order(
    State(service): State<Arc<SimpleCommerceService>>,
    Path((tenant_id, order_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateOrderRequest>,
) -> std::result::Result<Json<OrderResponse>, (StatusCode, String)> {
    match service.update_order(tenant_id, order_id, request).await {
        Ok(order) => Ok(Json(OrderResponse { order })),
        Err(Error::NotFound(_)) => Err((StatusCode::NOT_FOUND, "Order not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}