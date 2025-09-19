// ============================================================================
// OLYMPUS CLOUD - PAYMENT SERVICE
// ============================================================================
// Module: commerce/src/services/payment_service.rs
// Description: Business logic for payment processing and gateway integration
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use uuid::Uuid;

use crate::models::{
    CreatePaymentMethodRequest, CreatePaymentRequest, PaymentTransaction, PaymentAction, StoredPaymentMethod,
    PaymentResponse, PaymentTransactionStatus, PaymentSummary, PaymentType, ProcessPaymentRequest, Refund,
    RefundRequest, RefundStatus,
};
use olympus_shared::{
    error::{ApiError, ApiResult},
    events::{Event, EventPublisher},
};

use super::gateways::{PaymentGateway, StripeGateway, SquareGateway};

pub struct PaymentService {
    pool: PgPool,
    event_publisher: Arc<EventPublisher>,
    stripe_gateway: Arc<StripeGateway>,
    square_gateway: Arc<SquareGateway>,
}

impl PaymentService {
    pub fn new(pool: PgPool, event_publisher: Arc<EventPublisher>) -> Self {
        Self {
            pool,
            event_publisher,
            stripe_gateway: Arc::new(StripeGateway::new()),
            square_gateway: Arc::new(SquareGateway::new()),
        }
    }

    // ========================================================================
    // PAYMENT METHODS
    // ========================================================================

    pub async fn create_payment_method(
        &self,
        tenant_id: Uuid,
        request: CreatePaymentMethodRequest,
    ) -> ApiResult<StoredPaymentMethod> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let payment_method = sqlx::query_as!(
            StoredPaymentMethod,
            r#"
            INSERT INTO commerce.payment_methods (
                id, tenant_id, customer_id, gateway, gateway_method_id,
                method_type, display_name, last_four, brand,
                exp_month, exp_year, is_default, metadata,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING *
            "#,
            id,
            tenant_id,
            request.customer_id,
            request.gateway as _,
            request.gateway_method_id,
            request.method_type as _,
            request.display_name,
            request.last_four,
            request.brand,
            request.exp_month,
            request.exp_year,
            request.is_default,
            request.metadata,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        self.publish_payment_event(
            "payment_method.created",
            tenant_id,
            serde_json::to_value(&payment_method).unwrap(),
        )
        .await?;

        Ok(payment_method)
    }

    pub async fn get_payment_method(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> ApiResult<StoredPaymentMethod> {
        let payment_method = sqlx::query_as!(
            StoredPaymentMethod,
            r#"
            SELECT * FROM commerce.payment_methods
            WHERE id = $1 AND tenant_id = $2
            "#,
            id,
            tenant_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Payment method not found".to_string()))?;

        Ok(payment_method)
    }

    pub async fn list_payment_methods(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> ApiResult<Vec<StoredPaymentMethod>> {
        let methods = sqlx::query_as!(
            StoredPaymentMethod,
            r#"
            SELECT * FROM commerce.payment_methods
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(methods)
    }

    pub async fn update_payment_method(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        request: CreatePaymentMethodRequest,
    ) -> ApiResult<StoredPaymentMethod> {
        let now = Utc::now();

        let payment_method = sqlx::query_as!(
            StoredPaymentMethod,
            r#"
            UPDATE commerce.payment_methods
            SET display_name = $3,
                last_four = $4,
                brand = $5,
                exp_month = $6,
                exp_year = $7,
                is_default = $8,
                metadata = $9,
                updated_at = $10
            WHERE id = $1 AND tenant_id = $2
            RETURNING *
            "#,
            id,
            tenant_id,
            request.display_name,
            request.last_four,
            request.brand,
            request.exp_month,
            request.exp_year,
            request.is_default,
            request.metadata,
            now
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Payment method not found".to_string()))?;

        self.publish_payment_event(
            "payment_method.updated",
            tenant_id,
            serde_json::to_value(&payment_method).unwrap(),
        )
        .await?;

        Ok(payment_method)
    }

    pub async fn delete_payment_method(&self, tenant_id: Uuid, id: Uuid) -> ApiResult<()> {
        let result = sqlx::query!(
            r#"
            DELETE FROM commerce.payment_methods
            WHERE id = $1 AND tenant_id = $2
            "#,
            id,
            tenant_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound("Payment method not found".to_string()));
        }

        self.publish_payment_event(
            "payment_method.deleted",
            tenant_id,
            serde_json::json!({ "id": id }),
        )
        .await?;

        Ok(())
    }

    // ========================================================================
    // PAYMENT PROCESSING
    // ========================================================================

    pub async fn create_payment(
        &self,
        tenant_id: Uuid,
        request: CreatePaymentRequest,
    ) -> ApiResult<PaymentResponse> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // Start transaction
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Create payment record
        let payment = sqlx::query_as!(
            PaymentTransaction,
            r#"
            INSERT INTO commerce.payments (
                id, tenant_id, order_id, payment_method_id, gateway,
                amount, currency, status, payment_type, metadata,
                created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING *
            "#,
            id,
            tenant_id,
            request.order_id,
            request.payment_method_id,
            request.gateway as _,
            request.amount,
            request.currency,
            PaymentTransactionStatus::Pending as _,
            request.payment_type as _,
            request.metadata,
            now,
            now
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Process payment with gateway
        let gateway_response = self
            .process_with_gateway(&payment, request.payment_type)
            .await;

        let (status, error_message, success) = match gateway_response {
            Ok(response) => {
                let status = match request.payment_type {
                    PaymentType::Authorization => PaymentTransactionStatus::Authorized,
                    PaymentType::Sale => PaymentTransactionStatus::Completed,
                    _ => PaymentTransactionStatus::Processing,
                };
                (status, None, true)
            }
            Err(e) => (PaymentTransactionStatus::Failed, Some(e.to_string()), false),
        };

        // Update payment status
        let payment = sqlx::query_as!(
            PaymentTransaction,
            r#"
            UPDATE commerce.payments
            SET status = $3,
                error_message = $4,
                processed_at = $5,
                updated_at = $5
            WHERE id = $1 AND tenant_id = $2
            RETURNING *
            "#,
            id,
            tenant_id,
            status as _,
            error_message,
            now
        )
        .fetch_one(&mut tx)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Publish event
        self.publish_payment_event(
            "payment.created",
            tenant_id,
            serde_json::to_value(&payment).unwrap(),
        )
        .await?;

        Ok(PaymentResponse {
            payment,
            success,
            message: error_message,
        })
    }

    pub async fn process_payment(
        &self,
        tenant_id: Uuid,
        payment_id: Uuid,
        request: ProcessPaymentRequest,
    ) -> ApiResult<PaymentResponse> {
        // Get existing payment
        let mut payment = self.get_payment(tenant_id, payment_id).await?;

        // Validate action
        let new_status = match request.action {
            PaymentAction::Capture => {
                if payment.status != PaymentTransactionStatus::Authorized {
                    return Err(ApiError::ValidationError {
                        field: "status".to_string(),
                        message: "Payment must be authorized to capture".to_string(),
                    });
                }
                PaymentTransactionStatus::Completed
            }
            PaymentAction::Cancel => {
                if payment.status == PaymentTransactionStatus::Completed {
                    return Err(ApiError::ValidationError {
                        field: "status".to_string(),
                        message: "Cannot cancel completed payment".to_string(),
                    });
                }
                PaymentTransactionStatus::Cancelled
            }
            PaymentAction::Refund | PaymentAction::PartialRefund => {
                if payment.status != PaymentTransactionStatus::Completed {
                    return Err(ApiError::ValidationError {
                        field: "status".to_string(),
                        message: "Payment must be completed to refund".to_string(),
                    });
                }
                if request.action == PaymentAction::Refund {
                    PaymentTransactionStatus::Refunded
                } else {
                    PaymentTransactionStatus::PartiallyRefunded
                }
            }
            _ => payment.status,
        };

        // Process with gateway
        let gateway_response = self
            .execute_gateway_action(&payment, request.action.clone())
            .await;

        let (success, error_message) = match gateway_response {
            Ok(_) => (true, None),
            Err(e) => (false, Some(e.to_string())),
        };

        if success {
            payment.status = new_status;
            payment.gateway_payment_id = request.gateway_payment_id.or(payment.gateway_payment_id);
            payment.gateway_customer_id = request.gateway_customer_id.or(payment.gateway_customer_id);
        } else {
            payment.error_message = error_message.clone();
        }

        let now = Utc::now();

        // Update payment in database
        let payment = sqlx::query_as!(
            PaymentTransaction,
            r#"
            UPDATE commerce.payments
            SET status = $3,
                gateway_payment_id = $4,
                gateway_customer_id = $5,
                error_message = $6,
                processed_at = $7,
                updated_at = $7
            WHERE id = $1 AND tenant_id = $2
            RETURNING *
            "#,
            payment_id,
            tenant_id,
            payment.status as _,
            payment.gateway_payment_id,
            payment.gateway_customer_id,
            payment.error_message,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Publish event
        let event_type = format!("payment.{:?}", request.action).to_lowercase();
        self.publish_payment_event(
            &event_type,
            tenant_id,
            serde_json::to_value(&payment).unwrap(),
        )
        .await?;

        Ok(PaymentResponse {
            payment,
            success,
            message: error_message,
        })
    }

    pub async fn get_payment(&self, tenant_id: Uuid, id: Uuid) -> ApiResult<PaymentTransaction> {
        let payment = sqlx::query_as!(
            PaymentTransaction,
            r#"
            SELECT * FROM commerce.payments
            WHERE id = $1 AND tenant_id = $2
            "#,
            id,
            tenant_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Payment not found".to_string()))?;

        Ok(payment)
    }

    pub async fn list_payments(
        &self,
        tenant_id: Uuid,
        status: Option<String>,
        gateway: Option<String>,
        from_date: Option<String>,
        to_date: Option<String>,
        limit: i64,
        offset: i64,
    ) -> ApiResult<Vec<PaymentTransaction>> {
        // Build dynamic query based on filters
        let mut query = String::from("SELECT * FROM commerce.payments WHERE tenant_id = $1");
        let mut param_count = 1;
        let mut params: Vec<String> = vec![];

        if status.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND status = ${}", param_count));
            params.push(status.unwrap());
        }

        if gateway.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND gateway = ${}", param_count));
            params.push(gateway.unwrap());
        }

        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND created_at >= ${}", param_count));
            params.push(from_date.unwrap());
        }

        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND created_at <= ${}", param_count));
            params.push(to_date.unwrap());
        }

        param_count += 1;
        query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${}", param_count));
        param_count += 1;
        query.push_str(&format!(" OFFSET ${}", param_count));

        // For simplicity, using a basic query without dynamic filters for now
        let payments = sqlx::query_as!(
            PaymentTransaction,
            r#"
            SELECT * FROM commerce.payments
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(payments)
    }

    pub async fn capture_payment(
        &self,
        tenant_id: Uuid,
        payment_id: Uuid,
    ) -> ApiResult<PaymentResponse> {
        self.process_payment(
            tenant_id,
            payment_id,
            ProcessPaymentRequest {
                payment_id,
                gateway_payment_id: None,
                gateway_customer_id: None,
                action: PaymentAction::Capture,
                amount: None,
                reason: None,
            },
        )
        .await
    }

    pub async fn void_payment(
        &self,
        tenant_id: Uuid,
        payment_id: Uuid,
    ) -> ApiResult<PaymentResponse> {
        self.process_payment(
            tenant_id,
            payment_id,
            ProcessPaymentRequest {
                payment_id,
                gateway_payment_id: None,
                gateway_customer_id: None,
                action: PaymentAction::Cancel,
                amount: None,
                reason: None,
            },
        )
        .await
    }

    // ========================================================================
    // REFUNDS
    // ========================================================================

    pub async fn create_refund(
        &self,
        tenant_id: Uuid,
        request: RefundRequest,
    ) -> ApiResult<Refund> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // Get the original payment
        let payment = self.get_payment(tenant_id, request.payment_id).await?;

        if payment.status != PaymentTransactionStatus::Completed {
            return Err(ApiError::ValidationError {
                field: "payment_status".to_string(),
                message: "Payment must be completed to refund".to_string(),
            });
        }

        // Create refund record
        let refund = sqlx::query_as!(
            Refund,
            r#"
            INSERT INTO commerce.refunds (
                id, tenant_id, payment_id, amount, currency,
                status, reason, metadata, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#,
            id,
            tenant_id,
            request.payment_id,
            request.amount,
            payment.currency,
            RefundStatus::Pending as _,
            request.reason,
            request.metadata,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Publish event
        self.publish_payment_event(
            "refund.created",
            tenant_id,
            serde_json::to_value(&refund).unwrap(),
        )
        .await?;

        Ok(refund)
    }

    pub async fn get_refund(&self, tenant_id: Uuid, id: Uuid) -> ApiResult<Refund> {
        let refund = sqlx::query_as!(
            Refund,
            r#"
            SELECT * FROM commerce.refunds
            WHERE id = $1 AND tenant_id = $2
            "#,
            id,
            tenant_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?
        .ok_or_else(|| ApiError::NotFound("Refund not found".to_string()))?;

        Ok(refund)
    }

    pub async fn list_refunds(
        &self,
        tenant_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> ApiResult<Vec<Refund>> {
        let refunds = sqlx::query_as!(
            Refund,
            r#"
            SELECT * FROM commerce.refunds
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(refunds)
    }

    pub async fn process_refund(&self, tenant_id: Uuid, refund_id: Uuid) -> ApiResult<Refund> {
        let mut refund = self.get_refund(tenant_id, refund_id).await?;

        if refund.status != RefundStatus::Pending {
            return Err(ApiError::ValidationError {
                field: "status".to_string(),
                message: "Refund is not pending".to_string(),
            });
        }

        // Process refund with gateway
        // This would integrate with actual payment gateway
        // For now, simulate success
        refund.status = RefundStatus::Completed;
        refund.processed_at = Some(Utc::now());

        let now = Utc::now();

        // Update refund in database
        let refund = sqlx::query_as!(
            Refund,
            r#"
            UPDATE commerce.refunds
            SET status = $3,
                processed_at = $4,
                updated_at = $4
            WHERE id = $1 AND tenant_id = $2
            RETURNING *
            "#,
            refund_id,
            tenant_id,
            refund.status as _,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Update payment status
        sqlx::query!(
            r#"
            UPDATE commerce.payments
            SET status = $3,
                updated_at = $4
            WHERE id = $1 AND tenant_id = $2
            "#,
            refund.payment_id,
            tenant_id,
            PaymentTransactionStatus::Refunded as _,
            now
        )
        .execute(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        // Publish event
        self.publish_payment_event(
            "refund.processed",
            tenant_id,
            serde_json::to_value(&refund).unwrap(),
        )
        .await?;

        Ok(refund)
    }

    // ========================================================================
    // ANALYTICS
    // ========================================================================

    pub async fn get_payment_summary(
        &self,
        tenant_id: Uuid,
        from_date: Option<String>,
        to_date: Option<String>,
    ) -> ApiResult<PaymentSummary> {
        let summary = sqlx::query_as!(
            PaymentSummary,
            r#"
            SELECT
                COUNT(*) as total_payments,
                COALESCE(SUM(amount), 0) as total_amount,
                COUNT(*) FILTER (WHERE status = 'completed') as successful_payments,
                COUNT(*) FILTER (WHERE status = 'failed') as failed_payments,
                COUNT(*) FILTER (WHERE status = 'pending') as pending_payments,
                COALESCE(SUM(amount) FILTER (WHERE status IN ('refunded', 'partially_refunded')), 0) as refunded_amount,
                COALESCE(AVG(amount), 0) as average_payment
            FROM commerce.payments
            WHERE tenant_id = $1
            "#,
            tenant_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(summary)
    }

    // ========================================================================
    // HELPER METHODS
    // ========================================================================

    async fn process_with_gateway(
        &self,
        payment: &PaymentTransaction,
        payment_type: PaymentType,
    ) -> ApiResult<String> {
        // Select gateway based on payment gateway type
        // This is a simplified version - actual implementation would have proper gateway integration
        match payment.gateway {
            crate::models::PaymentGateway::Stripe => {
                // Process with Stripe
                Ok("stripe_payment_id".to_string())
            }
            crate::models::PaymentGateway::Square => {
                // Process with Square
                Ok("square_payment_id".to_string())
            }
            _ => {
                // Manual or cash payments don't need gateway processing
                Ok("manual_payment".to_string())
            }
        }
    }

    async fn execute_gateway_action(
        &self,
        payment: &PaymentTransaction,
        action: PaymentAction,
    ) -> ApiResult<String> {
        // Execute action on payment gateway
        // This is a simplified version
        match action {
            PaymentAction::Capture => Ok("captured".to_string()),
            PaymentAction::Cancel => Ok("cancelled".to_string()),
            PaymentAction::Refund => Ok("refunded".to_string()),
            PaymentAction::PartialRefund => Ok("partially_refunded".to_string()),
            _ => Ok("processed".to_string()),
        }
    }

    async fn publish_payment_event(
        &self,
        event_type: &str,
        tenant_id: Uuid,
        data: serde_json::Value,
    ) -> ApiResult<()> {
        let event = Event::new(event_type, tenant_id, data);
        self.event_publisher
            .publish(&format!("events:payment:{}", event_type), event)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;
        Ok(())
    }
}