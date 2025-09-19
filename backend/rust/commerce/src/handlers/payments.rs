// ============================================================================
// OLYMPUS CLOUD - PAYMENT HANDLERS
// ============================================================================
// Module: commerce/src/handlers/payments.rs
// Description: HTTP handlers for payment processing operations
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    models::{
        CreatePaymentMethodRequest, CreatePaymentRequest, PaymentTransaction, StoredPaymentMethod,
        PaymentResponse, PaymentSummary, ProcessPaymentRequest, Refund, RefundRequest,
    },
    services::payment_service::PaymentService,
};
use olympus_shared::{
    auth::Claims,
    error::{ApiError, ApiResult},
};

#[derive(Debug, Deserialize)]
pub struct PaymentQuery {
    pub status: Option<String>,
    pub gateway: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

// ============================================================================
// PAYMENT METHODS HANDLERS
// ============================================================================

pub async fn create_payment_method(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Json(request): Json<CreatePaymentMethodRequest>,
) -> ApiResult<Json<StoredPaymentMethod>> {
    request.validate().map_err(|e| ApiError::ValidationError {
        field: "request".to_string(),
        message: e.to_string(),
    })?;

    let payment_method = service
        .create_payment_method(claims.tenant_id, request)
        .await?;

    Ok(Json(payment_method))
}

pub async fn get_payment_method(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<StoredPaymentMethod>> {
    let payment_method = service.get_payment_method(claims.tenant_id, id).await?;
    Ok(Json(payment_method))
}

pub async fn list_payment_methods(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Query(query): Query<PaymentQuery>,
) -> ApiResult<Json<Vec<StoredPaymentMethod>>> {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let methods = service
        .list_payment_methods(claims.tenant_id, limit, offset)
        .await?;

    Ok(Json(methods))
}

pub async fn update_payment_method(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
    Json(request): Json<CreatePaymentMethodRequest>,
) -> ApiResult<Json<StoredPaymentMethod>> {
    request.validate().map_err(|e| ApiError::ValidationError {
        field: "request".to_string(),
        message: e.to_string(),
    })?;

    let payment_method = service
        .update_payment_method(claims.tenant_id, id, request)
        .await?;

    Ok(Json(payment_method))
}

pub async fn delete_payment_method(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    service.delete_payment_method(claims.tenant_id, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// PAYMENT PROCESSING HANDLERS
// ============================================================================

pub async fn create_payment(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Json(request): Json<CreatePaymentRequest>,
) -> ApiResult<Json<PaymentResponse>> {
    request.validate().map_err(|e| ApiError::ValidationError {
        field: "request".to_string(),
        message: e.to_string(),
    })?;

    let response = service.create_payment(claims.tenant_id, request).await?;
    Ok(Json(response))
}

pub async fn process_payment(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
    Json(request): Json<ProcessPaymentRequest>,
) -> ApiResult<Json<PaymentResponse>> {
    let response = service
        .process_payment(claims.tenant_id, id, request)
        .await?;
    Ok(Json(response))
}

pub async fn get_payment(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PaymentTransaction>> {
    let payment = service.get_payment(claims.tenant_id, id).await?;
    Ok(Json(payment))
}

pub async fn list_payments(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Query(query): Query<PaymentQuery>,
) -> ApiResult<Json<Vec<PaymentTransaction>>> {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let payments = service
        .list_payments(
            claims.tenant_id,
            query.status,
            query.gateway,
            query.from_date,
            query.to_date,
            limit,
            offset,
        )
        .await?;

    Ok(Json(payments))
}

pub async fn capture_payment(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PaymentResponse>> {
    let response = service.capture_payment(claims.tenant_id, id).await?;
    Ok(Json(response))
}

pub async fn void_payment(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PaymentResponse>> {
    let response = service.void_payment(claims.tenant_id, id).await?;
    Ok(Json(response))
}

// ============================================================================
// REFUND HANDLERS
// ============================================================================

pub async fn create_refund(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Json(request): Json<RefundRequest>,
) -> ApiResult<Json<Refund>> {
    request.validate().map_err(|e| ApiError::ValidationError {
        field: "request".to_string(),
        message: e.to_string(),
    })?;

    let refund = service.create_refund(claims.tenant_id, request).await?;
    Ok(Json(refund))
}

pub async fn get_refund(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Refund>> {
    let refund = service.get_refund(claims.tenant_id, id).await?;
    Ok(Json(refund))
}

pub async fn list_refunds(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Query(query): Query<PaymentQuery>,
) -> ApiResult<Json<Vec<Refund>>> {
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let refunds = service
        .list_refunds(claims.tenant_id, limit, offset)
        .await?;

    Ok(Json(refunds))
}

pub async fn process_refund(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Refund>> {
    let refund = service.process_refund(claims.tenant_id, id).await?;
    Ok(Json(refund))
}

// ============================================================================
// ANALYTICS HANDLERS
// ============================================================================

pub async fn get_payment_summary(
    Extension(claims): Extension<Claims>,
    State(service): State<PaymentService>,
    Query(query): Query<PaymentQuery>,
) -> ApiResult<Json<PaymentSummary>> {
    let summary = service
        .get_payment_summary(
            claims.tenant_id,
            query.from_date,
            query.to_date,
        )
        .await?;

    Ok(Json(summary))
}

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn payment_routes() -> Router<PaymentService> {
    Router::new()
        // Payment Methods
        .route("/payment-methods", post(create_payment_method))
        .route("/payment-methods", get(list_payment_methods))
        .route("/payment-methods/:id", get(get_payment_method))
        .route("/payment-methods/:id", put(update_payment_method))
        .route("/payment-methods/:id", delete(delete_payment_method))
        // Payments
        .route("/payments", post(create_payment))
        .route("/payments", get(list_payments))
        .route("/payments/:id", get(get_payment))
        .route("/payments/:id/process", post(process_payment))
        .route("/payments/:id/capture", post(capture_payment))
        .route("/payments/:id/void", post(void_payment))
        // Refunds
        .route("/refunds", post(create_refund))
        .route("/refunds", get(list_refunds))
        .route("/refunds/:id", get(get_refund))
        .route("/refunds/:id/process", post(process_refund))
        // Analytics
        .route("/payments/summary", get(get_payment_summary))
}