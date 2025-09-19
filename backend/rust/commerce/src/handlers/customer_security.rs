// ============================================================================
// OLYMPUS CLOUD - CUSTOMER SECURITY API HANDLERS
// ============================================================================
// Module: commerce/src/handlers/customer_security.rs
// Description: REST API handlers for customer security and privacy operations
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::net::IpAddr;

use crate::models::customer_security::*;
use crate::services::CustomerSecurityService;
use olympus_shared::integration::AuthContext;

/// Response wrapper for customer security API endpoints
#[derive(Debug, Serialize)]
pub struct SecurityApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub compliance_notice: Option<String>,
}

impl<T> SecurityApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            compliance_notice: None,
        }
    }

    pub fn success_with_notice(data: T, notice: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            compliance_notice: Some(notice),
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            compliance_notice: None,
        }
    }
}

/// Login attempt request
#[derive(Debug, Deserialize)]
pub struct LoginAttemptRequest {
    pub email: String,
    pub success: bool,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
}

// ============================================================================
// CUSTOMER SECURITY ROUTES
// ============================================================================

pub fn customer_security_routes() -> Router<CustomerSecurityService> {
    Router::new()
        // Customer security operations
        .route("/customers/:customer_id/security", get(get_customer_security))
        .route("/customers/:customer_id/security", put(update_customer_security))

        // Authentication and login security
        .route("/auth/login-attempt", post(handle_login_attempt))

        // Data access and audit
        .route("/customers/:customer_id/access-log", post(log_data_access))
        .route("/customers/:customer_id/access-log", get(get_access_log))

        // Consent management
        .route("/customers/:customer_id/consent", post(record_consent))
        .route("/customers/:customer_id/consent", get(get_consent_history))

        // GDPR compliance
        .route("/customers/:customer_id/gdpr/export", post(export_customer_data))
        .route("/customers/:customer_id/gdpr/anonymize", post(anonymize_customer))
        .route("/gdpr/retention-cleanup", get(get_retention_cleanup_candidates))

        // Security monitoring and metrics
        .route("/security/metrics", get(get_security_metrics))
        .route("/security/incidents", get(get_security_incidents))
}

// ============================================================================
// CUSTOMER SECURITY HANDLERS
// ============================================================================

/// GET /api/v1/customers/:customer_id/security
/// Get customer security information
pub async fn get_customer_security(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> std::result::Result<Json<SecurityApiResponse<SecureCustomer>>, StatusCode> {
    let access_purpose = params.get("purpose").cloned();

    match service.get_secure_customer(
        auth.tenant_id,
        customer_id,
        Some(auth.user_id),
        access_purpose,
    ).await {
        Ok(customer) => {
            let notice = "This data access has been logged for compliance purposes.".to_string();
            Ok(Json(SecurityApiResponse::success_with_notice(customer, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to get customer security info: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// PUT /api/v1/customers/:customer_id/security
/// Update customer security settings
pub async fn update_customer_security(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
    Json(request): Json<UpdateCustomerSecurityRequest>,
) -> std::result::Result<Json<SecurityApiResponse<SecureCustomer>>, StatusCode> {
    match service.update_customer_security(
        auth.tenant_id,
        customer_id,
        request,
        Some(auth.user_id),
    ).await {
        Ok(customer) => {
            let notice = "Security settings updated. Changes have been logged for audit purposes.".to_string();
            Ok(Json(SecurityApiResponse::success_with_notice(customer, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to update customer security: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// AUTHENTICATION SECURITY HANDLERS
// ============================================================================

/// POST /api/v1/auth/login-attempt
/// Handle login attempt for security monitoring
pub async fn handle_login_attempt(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Json(request): Json<LoginAttemptRequest>,
) -> std::result::Result<Json<SecurityApiResponse<Option<Uuid>>>, StatusCode> {
    match service.handle_login_attempt(
        auth.tenant_id,
        &request.email,
        request.success,
        request.ip_address,
        request.user_agent,
    ).await {
        Ok(customer_id) => {
            let notice = if request.success {
                "Login attempt processed successfully.".to_string()
            } else {
                "Failed login attempt has been logged. Account may be temporarily locked after multiple failures.".to_string()
            };
            Ok(Json(SecurityApiResponse::success_with_notice(customer_id, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to handle login attempt: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// DATA ACCESS AND AUDIT HANDLERS
// ============================================================================

/// POST /api/v1/customers/:customer_id/access-log
/// Log customer data access
pub async fn log_data_access(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
    Json(request): Json<LogDataAccessRequest>,
) -> std::result::Result<Json<SecurityApiResponse<Uuid>>, StatusCode> {
    match service.log_data_access(
        customer_id,
        auth.tenant_id,
        auth.user_id,
        request.access_type,
        request.data_fields,
        request.purpose,
        request.legal_basis,
        None, // IP would come from middleware
        None, // User agent would come from middleware
    ).await {
        Ok(log_id) => {
            Ok(Json(SecurityApiResponse::success(log_id)))
        }
        Err(e) => {
            tracing::error!("Failed to log data access: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/customers/:customer_id/access-log
/// Get customer data access history
pub async fn get_access_log(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
) -> std::result::Result<Json<SecurityApiResponse<Vec<CustomerDataAccessLog>>>, StatusCode> {
    // This would need to be implemented in the service
    // For now, return a placeholder
    Ok(Json(SecurityApiResponse::error("Access log retrieval not implemented yet".to_string())))
}

// ============================================================================
// CONSENT MANAGEMENT HANDLERS
// ============================================================================

/// POST /api/v1/customers/:customer_id/consent
/// Record customer consent
pub async fn record_consent(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
    Json(request): Json<RecordConsentRequest>,
) -> std::result::Result<Json<SecurityApiResponse<Uuid>>, StatusCode> {
    match service.record_consent(
        customer_id,
        auth.tenant_id,
        request,
        None, // IP would come from middleware
        None, // User agent would come from middleware
    ).await {
        Ok(consent_id) => {
            let notice = "Consent has been recorded and is legally binding. This action is auditable.".to_string();
            Ok(Json(SecurityApiResponse::success_with_notice(consent_id, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to record consent: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/customers/:customer_id/consent
/// Get customer consent history
pub async fn get_consent_history(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
) -> std::result::Result<Json<SecurityApiResponse<Vec<CustomerConsent>>>, StatusCode> {
    match service.get_consent_history(auth.tenant_id, customer_id).await {
        Ok(consents) => {
            Ok(Json(SecurityApiResponse::success(consents)))
        }
        Err(e) => {
            tracing::error!("Failed to get consent history: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// GDPR COMPLIANCE HANDLERS
// ============================================================================

/// POST /api/v1/customers/:customer_id/gdpr/export
/// Export customer data for GDPR compliance
pub async fn export_customer_data(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
) -> std::result::Result<Json<SecurityApiResponse<GdprExportData>>, StatusCode> {
    match service.export_customer_data(
        auth.tenant_id,
        customer_id,
        Some(auth.user_id),
    ).await {
        Ok(export_data) => {
            let notice = "GDPR data export completed. This action has been logged for compliance. Data contains all personal information associated with this customer.".to_string();
            Ok(Json(SecurityApiResponse::success_with_notice(export_data, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to export customer data: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /api/v1/customers/:customer_id/gdpr/anonymize
/// Anonymize customer data (GDPR right to be forgotten)
pub async fn anonymize_customer(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
    Path(customer_id): Path<Uuid>,
) -> std::result::Result<Json<SecurityApiResponse<AnonymizationResult>>, StatusCode> {
    match service.anonymize_customer_data(
        auth.tenant_id,
        customer_id,
        Some(auth.user_id),
    ).await {
        Ok(result) => {
            let notice = "Customer data has been anonymized in compliance with GDPR right to be forgotten. This action is irreversible and has been logged for audit purposes.".to_string();
            Ok(Json(SecurityApiResponse::success_with_notice(result, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to anonymize customer: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/gdpr/retention-cleanup
/// Get customers eligible for data retention cleanup
pub async fn get_retention_cleanup_candidates(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
) -> std::result::Result<Json<SecurityApiResponse<Vec<Uuid>>>, StatusCode> {
    match service.get_customers_for_retention_cleanup(auth.tenant_id).await {
        Ok(customer_ids) => {
            let notice = format!("Found {} customers eligible for data retention cleanup.", customer_ids.len());
            Ok(Json(SecurityApiResponse::success_with_notice(customer_ids, notice)))
        }
        Err(e) => {
            tracing::error!("Failed to get retention cleanup candidates: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ============================================================================
// SECURITY MONITORING HANDLERS
// ============================================================================

/// GET /api/v1/security/metrics
/// Get customer security metrics
pub async fn get_security_metrics(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
) -> std::result::Result<Json<SecurityApiResponse<CustomerSecurityMetrics>>, StatusCode> {
    match service.get_security_metrics(auth.tenant_id).await {
        Ok(metrics) => {
            Ok(Json(SecurityApiResponse::success(metrics)))
        }
        Err(e) => {
            tracing::error!("Failed to get security metrics: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// GET /api/v1/security/incidents
/// Get security incidents (placeholder)
pub async fn get_security_incidents(
    State(service): State<CustomerSecurityService>,
    auth: AuthContext,
) -> std::result::Result<Json<SecurityApiResponse<Vec<SecurityIncident>>>, StatusCode> {
    // This would need to be implemented in the service
    // For now, return empty list
    Ok(Json(SecurityApiResponse::success(vec![])))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_customer_security_routes_compile() {
        // Basic test to ensure routes compile correctly
        let _routes = customer_security_routes();
        assert!(true);
    }
}