// ============================================================================
// OLYMPUS CLOUD - PRODUCT HANDLERS
// ============================================================================
// Module: commerce/src/handlers/products.rs
// Description: HTTP handlers for product catalog management
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
    Product, ProductCategory, ProductSearchRequest, ProductSearchResponse,
    CreateProductRequest, UpdateProductRequest, ProductSortBy, SortOrder,
    ProductStatus, ProductType,
};
use crate::services::CatalogService;

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

pub fn create_product_router(catalog_service: Arc<CatalogService>) -> Router {
    Router::new()
        // Product CRUD operations
        .route("/products", post(create_product))
        .route("/products", get(list_products))
        .route("/products/search", post(search_products))
        .route("/products/:product_id", get(get_product))
        .route("/products/:product_id", put(update_product))
        .route("/products/:product_id", delete(delete_product))

        // Product categories
        .route("/categories", post(create_category))
        .route("/categories", get(list_categories))
        .route("/categories/:category_id", get(get_category))

        // Product variants (placeholder for future implementation)
        .route("/products/:product_id/variants", get(list_product_variants))
        .route("/products/:product_id/variants", post(create_product_variant))

        .with_state(catalog_service)
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub success: bool,
    pub data: Product,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductListResponse {
    pub success: bool,
    pub data: Vec<Product>,
    pub total_count: i64,
    pub has_more: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductSearchResponseWrapper {
    pub success: bool,
    pub data: ProductSearchResponse,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    pub success: bool,
    pub data: ProductCategory,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryListResponse {
    pub success: bool,
    pub data: Vec<ProductCategory>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ProductListQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub status: Option<String>,
    pub product_type: Option<String>,
    pub category_id: Option<Uuid>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
}

// ============================================================================
// PRODUCT HANDLERS
// ============================================================================

pub async fn create_product(
    State(catalog_service): State<Arc<CatalogService>>,
    Json(request): Json<CreateProductRequest>,
) -> Result<Json<ProductResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| OlympusError::Validation(format!("Invalid request: {}", e)))?;

    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let created_by = Uuid::new_v4(); // Mock user ID

    let product = catalog_service
        .create_product(tenant_id, request, created_by)
        .await?;

    Ok(Json(ProductResponse {
        success: true,
        data: product,
        message: "Product created successfully".to_string(),
    }))
}

pub async fn get_product(
    State(catalog_service): State<Arc<CatalogService>>,
    Path(product_id): Path<Uuid>,
) -> Result<Json<ProductResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let product = catalog_service
        .get_product(tenant_id, product_id)
        .await?
        .ok_or_else(|| OlympusError::NotFound("Product not found".to_string()))?;

    Ok(Json(ProductResponse {
        success: true,
        data: product,
        message: "Product retrieved successfully".to_string(),
    }))
}

pub async fn list_products(
    State(catalog_service): State<Arc<CatalogService>>,
    Query(query): Query<ProductListQuery>,
) -> Result<Json<ProductSearchResponseWrapper>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    // Parse query parameters
    let status = query.status.as_deref().and_then(|s| match s {
        "draft" => Some(ProductStatus::Draft),
        "active" => Some(ProductStatus::Active),
        "inactive" => Some(ProductStatus::Inactive),
        "discontinued" => Some(ProductStatus::Discontinued),
        "out_of_stock" => Some(ProductStatus::OutOfStock),
        _ => None,
    });

    let product_type = query.product_type.as_deref().and_then(|t| match t {
        "simple" => Some(ProductType::Simple),
        "variable" => Some(ProductType::Variable),
        "bundle" => Some(ProductType::Bundle),
        "digital" => Some(ProductType::Digital),
        "service" => Some(ProductType::Service),
        "subscription" => Some(ProductType::Subscription),
        _ => None,
    });

    let sort_by = query.sort_by.as_deref().and_then(|s| match s {
        "name" => Some(ProductSortBy::Name),
        "price" => Some(ProductSortBy::Price),
        "created_at" => Some(ProductSortBy::CreatedAt),
        "updated_at" => Some(ProductSortBy::UpdatedAt),
        "popularity" => Some(ProductSortBy::Popularity),
        "stock" => Some(ProductSortBy::Stock),
        _ => None,
    });

    let sort_order = query.sort_order.as_deref().and_then(|o| match o {
        "asc" => Some(SortOrder::Asc),
        "desc" => Some(SortOrder::Desc),
        _ => None,
    });

    let search_request = ProductSearchRequest {
        query: None,
        category_id: query.category_id,
        status,
        product_type,
        tags: None,
        price_min: None,
        price_max: None,
        in_stock_only: None,
        sort_by,
        sort_order,
        limit: query.limit,
        offset: query.offset,
    };

    let response = catalog_service
        .search_products(tenant_id, search_request)
        .await?;

    Ok(Json(ProductSearchResponseWrapper {
        success: true,
        data: response,
        message: "Products retrieved successfully".to_string(),
    }))
}

pub async fn search_products(
    State(catalog_service): State<Arc<CatalogService>>,
    Json(request): Json<ProductSearchRequest>,
) -> Result<Json<ProductSearchResponseWrapper>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let response = catalog_service
        .search_products(tenant_id, request)
        .await?;

    Ok(Json(ProductSearchResponseWrapper {
        success: true,
        data: response,
        message: "Product search completed successfully".to_string(),
    }))
}

pub async fn update_product(
    State(catalog_service): State<Arc<CatalogService>>,
    Path(product_id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> Result<Json<ProductResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| OlympusError::Validation(format!("Invalid request: {}", e)))?;

    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let updated_by = Uuid::new_v4(); // Mock user ID

    let product = catalog_service
        .update_product(tenant_id, product_id, request, updated_by)
        .await?
        .ok_or_else(|| OlympusError::NotFound("Product not found".to_string()))?;

    Ok(Json(ProductResponse {
        success: true,
        data: product,
        message: "Product updated successfully".to_string(),
    }))
}

pub async fn delete_product(
    State(catalog_service): State<Arc<CatalogService>>,
    Path(product_id): Path<Uuid>,
) -> Result<StatusCode> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let deleted_by = Uuid::new_v4(); // Mock user ID

    let deleted = catalog_service
        .delete_product(tenant_id, product_id, deleted_by)
        .await?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(OlympusError::NotFound("Product not found".to_string()))
    }
}

// ============================================================================
// CATEGORY HANDLERS
// ============================================================================

pub async fn create_category(
    State(catalog_service): State<Arc<CatalogService>>,
    Json(request): Json<CreateCategoryRequest>,
) -> Result<Json<CategoryResponse>> {
    // Validate request
    request.validate()
        .map_err(|e| OlympusError::Validation(format!("Invalid request: {}", e)))?;

    let tenant_id = Uuid::new_v4(); // Mock tenant ID
    let created_by = Uuid::new_v4(); // Mock user ID

    let category = catalog_service
        .create_category(
            tenant_id,
            request.name,
            request.description,
            request.parent_id,
            request.slug,
            created_by,
        )
        .await?;

    Ok(Json(CategoryResponse {
        success: true,
        data: category,
        message: "Category created successfully".to_string(),
    }))
}

pub async fn list_categories(
    State(catalog_service): State<Arc<CatalogService>>,
) -> Result<Json<CategoryListResponse>> {
    let tenant_id = Uuid::new_v4(); // Mock tenant ID

    let categories = catalog_service
        .get_category_tree(tenant_id)
        .await?;

    Ok(Json(CategoryListResponse {
        success: true,
        data: categories,
        message: "Categories retrieved successfully".to_string(),
    }))
}

pub async fn get_category(
    State(_catalog_service): State<Arc<CatalogService>>,
    Path(_category_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // Placeholder implementation
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Category details endpoint - not yet implemented"
    })))
}

// ============================================================================
// PRODUCT VARIANT HANDLERS (PLACEHOLDER)
// ============================================================================

pub async fn list_product_variants(
    State(_catalog_service): State<Arc<CatalogService>>,
    Path(_product_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>> {
    // Placeholder implementation
    Ok(Json(serde_json::json!({
        "success": true,
        "data": [],
        "message": "Product variants endpoint - not yet implemented"
    })))
}

pub async fn create_product_variant(
    State(_catalog_service): State<Arc<CatalogService>>,
    Path(_product_id): Path<Uuid>,
    Json(_request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>> {
    // Placeholder implementation
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Create product variant endpoint - not yet implemented"
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