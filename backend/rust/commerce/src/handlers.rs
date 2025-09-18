use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use serde::Deserialize;
use olympus_shared::types::{ApiResponse, PageRequest};
use crate::models::*;
use crate::services::CommerceService;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub tenant_id: Uuid,
}

#[derive(Deserialize)]
pub struct LocationParams {
    pub location_id: Option<Uuid>,
}

// Product handlers
pub async fn list_products(
    Extension(service): Extension<Arc<CommerceService>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let offset = ((page - 1) * per_page) as i64;

    match service.list_products(params.tenant_id, per_page as i64, offset).await {
        Ok(products) => (StatusCode::OK, Json(ApiResponse::success(products))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn create_product(
    Extension(service): Extension<Arc<CommerceService>>,
    Json(request): Json<CreateProductRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.create_product(request).await {
        Ok(product) => (StatusCode::CREATED, Json(ApiResponse::success(product))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_product(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.get_product(id).await {
        Ok(product) => (StatusCode::OK, Json(ApiResponse::success(product))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "NOT_FOUND".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn update_product(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateProductRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.update_product(id, request).await {
        Ok(product) => (StatusCode::OK, Json(ApiResponse::success(product))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn delete_product(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.delete_product(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse::success(serde_json::json!({
                "message": "Product deleted successfully"
            }))),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

// Variant handlers (placeholders)
pub async fn list_variants(Path(product_id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("List variants for product {} not yet implemented", product_id),
        )),
    )
}

pub async fn create_variant(Path(product_id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Create variant for product {} not yet implemented", product_id),
        )),
    )
}

// Category handlers (placeholders)
pub async fn list_categories() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Categories not yet implemented".to_string(),
        )),
    )
}

pub async fn create_category() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Create category not yet implemented".to_string(),
        )),
    )
}

pub async fn get_category(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get category {} not yet implemented", id),
        )),
    )
}

pub async fn update_category(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update category {} not yet implemented", id),
        )),
    )
}

pub async fn delete_category(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Delete category {} not yet implemented", id),
        )),
    )
}

// Inventory handlers
pub async fn list_inventory() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "List inventory not yet implemented".to_string(),
        )),
    )
}

pub async fn get_inventory(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(product_id): Path<Uuid>,
    Query(params): Query<LocationParams>,
) -> impl IntoResponse {
    match service.get_inventory(product_id, params.location_id).await {
        Ok(inventory) => (StatusCode::OK, Json(ApiResponse::success(inventory))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "NOT_FOUND".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn update_inventory(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(product_id): Path<Uuid>,
    Json(request): Json<UpdateInventoryRequest>,
) -> impl IntoResponse {
    match service.update_inventory(product_id, request.location_id, request.quantity).await {
        Ok(inventory) => (StatusCode::OK, Json(ApiResponse::success(inventory))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn adjust_inventory(
    Extension(service): Extension<Arc<CommerceService>>,
    Json(request): Json<AdjustInventoryRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.adjust_inventory(request).await {
        Ok(inventory) => (StatusCode::OK, Json(ApiResponse::success(inventory))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn transfer_inventory() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Transfer inventory not yet implemented".to_string(),
        )),
    )
}

// Order handlers
pub async fn list_orders(
    Extension(service): Extension<Arc<CommerceService>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let offset = ((page - 1) * per_page) as i64;

    match service.list_orders(params.tenant_id, per_page as i64, offset).await {
        Ok(orders) => (StatusCode::OK, Json(ApiResponse::success(orders))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn create_order(
    Extension(service): Extension<Arc<CommerceService>>,
    Json(request): Json<CreateOrderRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.create_order(request).await {
        Ok(order) => (StatusCode::CREATED, Json(ApiResponse::success(order))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_order(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.get_order(id).await {
        Ok(order) => (StatusCode::OK, Json(ApiResponse::success(order))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "NOT_FOUND".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn update_order(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update order {} not yet implemented", id),
        )),
    )
}

pub async fn cancel_order(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.cancel_order(id).await {
        Ok(order) => (StatusCode::OK, Json(ApiResponse::success(order))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn fulfill_order(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.fulfill_order(id).await {
        Ok(order) => (StatusCode::OK, Json(ApiResponse::success(order))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn refund_order(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Refund order {} not yet implemented", id),
        )),
    )
}

// Cart handlers (placeholders)
pub async fn create_cart() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Create cart not yet implemented".to_string(),
        )),
    )
}

pub async fn get_cart(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get cart {} not yet implemented", id),
        )),
    )
}

pub async fn add_to_cart(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Add to cart {} not yet implemented", id),
        )),
    )
}

pub async fn update_cart_item(
    Path((cart_id, item_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update cart {} item {} not yet implemented", cart_id, item_id),
        )),
    )
}

pub async fn remove_from_cart(
    Path((cart_id, item_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Remove from cart {} item {} not yet implemented", cart_id, item_id),
        )),
    )
}

pub async fn checkout(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Checkout cart {} not yet implemented", id),
        )),
    )
}

// Payment handlers
pub async fn list_payments() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "List payments not yet implemented".to_string(),
        )),
    )
}

pub async fn get_payment(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get payment {} not yet implemented", id),
        )),
    )
}

pub async fn process_payment(
    Extension(service): Extension<Arc<CommerceService>>,
    Json(request): Json<ProcessPaymentRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.process_payment(request).await {
        Ok(payment) => (StatusCode::OK, Json(ApiResponse::success(payment))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn capture_payment(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Capture payment {} not yet implemented", id),
        )),
    )
}

pub async fn void_payment(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Void payment {} not yet implemented", id),
        )),
    )
}

// Customer handlers
pub async fn list_customers(
    Extension(service): Extension<Arc<CommerceService>>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20);
    let offset = ((page - 1) * per_page) as i64;

    match service.list_customers(params.tenant_id, per_page as i64, offset).await {
        Ok(customers) => (StatusCode::OK, Json(ApiResponse::success(customers))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn create_customer(
    Extension(service): Extension<Arc<CommerceService>>,
    Json(request): Json<CreateCustomerRequest>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse::<()>::error(
                "VALIDATION_ERROR".to_string(),
                e.to_string(),
            )),
        );
    }

    match service.create_customer(request).await {
        Ok(customer) => (StatusCode::CREATED, Json(ApiResponse::success(customer))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<()>::error(
                "INTERNAL_ERROR".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn get_customer(
    Extension(service): Extension<Arc<CommerceService>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match service.get_customer(id).await {
        Ok(customer) => (StatusCode::OK, Json(ApiResponse::success(customer))),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(ApiResponse::<()>::error(
                "NOT_FOUND".to_string(),
                e.to_string(),
            )),
        ),
    }
}

pub async fn update_customer(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update customer {} not yet implemented", id),
        )),
    )
}

pub async fn get_customer_orders(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get customer {} orders not yet implemented", id),
        )),
    )
}

// Discount handlers (placeholders)
pub async fn list_discounts() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "List discounts not yet implemented".to_string(),
        )),
    )
}

pub async fn create_discount() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Create discount not yet implemented".to_string(),
        )),
    )
}

pub async fn get_discount(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Get discount {} not yet implemented", id),
        )),
    )
}

pub async fn update_discount(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Update discount {} not yet implemented", id),
        )),
    )
}

pub async fn delete_discount(Path(id): Path<Uuid>) -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            format!("Delete discount {} not yet implemented", id),
        )),
    )
}

pub async fn validate_discount() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Validate discount not yet implemented".to_string(),
        )),
    )
}

// Report handlers (placeholders)
pub async fn sales_report() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Sales report not yet implemented".to_string(),
        )),
    )
}

pub async fn inventory_report() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Inventory report not yet implemented".to_string(),
        )),
    )
}

pub async fn customer_report() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(ApiResponse::<()>::error(
            "NOT_IMPLEMENTED".to_string(),
            "Customer report not yet implemented".to_string(),
        )),
    )
}

#[derive(Deserialize, Validate)]
pub struct UpdateInventoryRequest {
    pub location_id: Uuid,
    pub quantity: i32,
}