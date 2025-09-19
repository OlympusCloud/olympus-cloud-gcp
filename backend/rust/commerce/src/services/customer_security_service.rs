// ============================================================================
// OLYMPUS CLOUD - CUSTOMER SECURITY SERVICE
// ============================================================================
// Module: commerce/src/services/customer_security_service.rs
// Description: Enhanced customer security and privacy service implementation
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use crate::models::customer_security::*;
use olympus_shared::{Result, Error};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::net::IpAddr;
use tracing::{info, warn, error};
use serde_json;

#[derive(Clone)]
pub struct CustomerSecurityService {
    db: PgPool,
}

impl CustomerSecurityService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    // ============================================================================
    // CUSTOMER SECURITY OPERATIONS
    // ============================================================================

    /// Get secure customer data with privacy controls
    pub async fn get_secure_customer(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        accessed_by: Option<Uuid>,
        access_purpose: Option<String>,
    ) -> Result<SecureCustomer> {
        // Log data access for compliance
        if let Some(user_id) = accessed_by {
            self.log_data_access(
                customer_id,
                tenant_id,
                user_id,
                DataAccessType::Read,
                vec!["customer_profile".to_string()],
                access_purpose,
                Some(GdprLegalBasis::LegitimateInterest),
                None,
                None,
            ).await?;
        }

        let customer = sqlx::query_as!(
            SecureCustomer,
            r#"
            SELECT
                id, tenant_id, email, first_name, last_name, phone, company,
                addresses, default_address_id, accepts_marketing, tax_exempt,
                notes, tags, metadata, total_spent, order_count, last_order_at,
                email_verified, email_verification_token, email_verification_expires_at,
                password_reset_token, password_reset_expires_at, login_attempts,
                locked_until, last_login_at, last_login_ip as "last_login_ip: IpAddr",
                privacy_consent_given, privacy_consent_date,
                marketing_consent_given, marketing_consent_date,
                gdpr_deletion_requested, gdpr_deletion_requested_at,
                data_retention_expires_at,
                data_classification as "data_classification: DataClassification",
                created_at, updated_at, deleted_at
            FROM customers
            WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            customer_id,
            tenant_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(customer)
    }

    /// Update customer security settings
    pub async fn update_customer_security(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        request: UpdateCustomerSecurityRequest,
        updated_by: Option<Uuid>,
    ) -> Result<SecureCustomer> {
        let mut tx = self.db.begin().await?;

        // Update customer record
        if let Some(email) = &request.email {
            sqlx::query!(
                r#"
                UPDATE customers
                SET email = $1, email_verified = false, updated_at = NOW()
                WHERE id = $2 AND tenant_id = $3
                "#,
                email,
                customer_id,
                tenant_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // Update privacy consent
        if let Some(privacy_consent) = request.privacy_consent_given {
            let consent_id = self.record_consent_internal(
                &mut tx,
                customer_id,
                tenant_id,
                "privacy".to_string(),
                privacy_consent,
                ConsentMethod::Explicit,
                None,
                None,
                None,
                None,
            ).await?;

            sqlx::query!(
                r#"
                UPDATE customers
                SET privacy_consent_given = $1, privacy_consent_date = NOW()
                WHERE id = $2 AND tenant_id = $3
                "#,
                privacy_consent,
                customer_id,
                tenant_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // Update marketing consent
        if let Some(marketing_consent) = request.marketing_consent_given {
            let consent_id = self.record_consent_internal(
                &mut tx,
                customer_id,
                tenant_id,
                "marketing".to_string(),
                marketing_consent,
                ConsentMethod::Explicit,
                None,
                None,
                None,
                None,
            ).await?;

            sqlx::query!(
                r#"
                UPDATE customers
                SET marketing_consent_given = $1, marketing_consent_date = NOW()
                WHERE id = $2 AND tenant_id = $3
                "#,
                marketing_consent,
                customer_id,
                tenant_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // Set data retention period
        if let Some(retention_years) = request.data_retention_years {
            let retention_date = Utc::now() + Duration::days(retention_years as i64 * 365);
            sqlx::query!(
                r#"
                UPDATE customers
                SET data_retention_expires_at = $1
                WHERE id = $2 AND tenant_id = $3
                "#,
                retention_date,
                customer_id,
                tenant_id
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        // Log the security update
        if let Some(user_id) = updated_by {
            self.log_data_access(
                customer_id,
                tenant_id,
                user_id,
                DataAccessType::Modify,
                vec!["security_settings".to_string()],
                Some("Customer security settings update".to_string()),
                Some(GdprLegalBasis::LegitimateInterest),
                None,
                None,
            ).await?;
        }

        self.get_secure_customer(tenant_id, customer_id, updated_by, Some("Security update verification".to_string())).await
    }

    /// Handle customer login attempt
    pub async fn handle_login_attempt(
        &self,
        tenant_id: Uuid,
        email: &str,
        success: bool,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> Result<Option<Uuid>> {
        let mut tx = self.db.begin().await?;

        let customer = sqlx::query!(
            r#"
            SELECT id, login_attempts, locked_until
            FROM customers
            WHERE email = $1 AND tenant_id = $2 AND deleted_at IS NULL
            "#,
            email,
            tenant_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(customer) = customer {
            if success {
                // Reset login attempts on successful login
                sqlx::query!(
                    r#"
                    UPDATE customers
                    SET login_attempts = 0, locked_until = NULL,
                        last_login_at = NOW(), last_login_ip = $1
                    WHERE id = $2
                    "#,
                    ip_address,
                    customer.id
                )
                .execute(&mut *tx)
                .await?;

                info!("Successful login for customer {}", customer.id);
            } else {
                // Increment failed login attempts
                let new_attempts = customer.login_attempts + 1;
                let lock_threshold = 5; // Lock after 5 failed attempts
                let lock_duration = Duration::minutes(30); // Lock for 30 minutes

                let locked_until = if new_attempts >= lock_threshold {
                    Some(Utc::now() + lock_duration)
                } else {
                    customer.locked_until
                };

                sqlx::query!(
                    r#"
                    UPDATE customers
                    SET login_attempts = $1, locked_until = $2
                    WHERE id = $3
                    "#,
                    new_attempts,
                    locked_until,
                    customer.id
                )
                .execute(&mut *tx)
                .await?;

                // Log security incident
                if new_attempts >= lock_threshold {
                    self.log_security_incident_internal(
                        &mut tx,
                        customer.id,
                        tenant_id,
                        SecurityIncidentType::AccountLocked,
                        IncidentSeverity::Medium,
                        format!("Account locked after {} failed login attempts", new_attempts),
                        ip_address,
                        user_agent,
                    ).await?;

                    warn!("Customer {} account locked after {} failed attempts", customer.id, new_attempts);
                } else {
                    self.log_security_incident_internal(
                        &mut tx,
                        customer.id,
                        tenant_id,
                        SecurityIncidentType::FailedLogin,
                        IncidentSeverity::Low,
                        format!("Failed login attempt {} of {}", new_attempts, lock_threshold),
                        ip_address,
                        user_agent,
                    ).await?;
                }
            }

            tx.commit().await?;
            Ok(Some(customer.id))
        } else {
            tx.commit().await?;
            Ok(None)
        }
    }

    // ============================================================================
    // DATA ACCESS AND AUDIT LOGGING
    // ============================================================================

    /// Log customer data access for GDPR compliance
    pub async fn log_data_access(
        &self,
        customer_id: Uuid,
        tenant_id: Uuid,
        accessed_by: Uuid,
        access_type: DataAccessType,
        data_fields: Vec<String>,
        purpose: Option<String>,
        legal_basis: Option<GdprLegalBasis>,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> Result<Uuid> {
        let log_id = sqlx::query_scalar!(
            r#"
            SELECT log_customer_data_access($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            customer_id,
            tenant_id,
            accessed_by,
            access_type as DataAccessType,
            &data_fields,
            purpose,
            legal_basis as Option<GdprLegalBasis>,
            ip_address,
            user_agent
        )
        .fetch_one(&self.db)
        .await?;

        Ok(log_id)
    }

    /// Record customer consent
    pub async fn record_consent(
        &self,
        customer_id: Uuid,
        tenant_id: Uuid,
        request: RecordConsentRequest,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> Result<Uuid> {
        let consent_id = sqlx::query_scalar!(
            r#"
            SELECT record_customer_consent($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            customer_id,
            tenant_id,
            request.consent_type,
            request.status,
            request.consent_method as ConsentMethod,
            request.consent_text,
            ip_address,
            user_agent,
            request.valid_until
        )
        .fetch_one(&self.db)
        .await?;

        info!(
            "Recorded {} consent for customer {}: {}",
            request.consent_type, customer_id, request.status
        );

        Ok(consent_id)
    }

    /// Get customer consent history
    pub async fn get_consent_history(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
    ) -> Result<Vec<CustomerConsent>> {
        let consents = sqlx::query_as!(
            CustomerConsent,
            r#"
            SELECT
                id, customer_id, tenant_id, consent_type, status,
                consent_method as "consent_method: ConsentMethod",
                consent_text, ip_address as "ip_address: Option<IpAddr>",
                user_agent, valid_from, valid_until, revoked_at, created_at
            FROM customer_consent
            WHERE customer_id = $1 AND tenant_id = $2
            ORDER BY created_at DESC
            "#,
            customer_id,
            tenant_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(consents)
    }

    // ============================================================================
    // GDPR COMPLIANCE OPERATIONS
    // ============================================================================

    /// Export all customer data for GDPR compliance
    pub async fn export_customer_data(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        requested_by: Option<Uuid>,
    ) -> Result<GdprExportData> {
        let export_data = sqlx::query_scalar!(
            r#"SELECT export_customer_gdpr_data($1, $2)"#,
            customer_id,
            tenant_id
        )
        .fetch_one(&self.db)
        .await?;

        let export_result = GdprExportData {
            customer: export_data["customer"].clone(),
            orders: export_data["orders"].clone(),
            consents: export_data["consents"].clone(),
            exported_at: export_data["exported_at"]
                .as_str()
                .unwrap()
                .parse::<DateTime<Utc>>()
                .unwrap(),
            export_id: Uuid::new_v4(),
        };

        info!("GDPR data export completed for customer {}", customer_id);

        Ok(export_result)
    }

    /// Anonymize customer data for GDPR right to be forgotten
    pub async fn anonymize_customer_data(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        performed_by: Option<Uuid>,
    ) -> Result<AnonymizationResult> {
        let success = sqlx::query_scalar!(
            r#"SELECT anonymize_customer_data($1, $2, $3)"#,
            customer_id,
            tenant_id,
            performed_by
        )
        .fetch_one(&self.db)
        .await?;

        if success {
            let result = AnonymizationResult {
                customer_id,
                anonymized_at: Utc::now(),
                data_removed: vec![
                    "email".to_string(),
                    "first_name".to_string(),
                    "last_name".to_string(),
                    "phone".to_string(),
                    "addresses".to_string(),
                    "encrypted_data".to_string(),
                ],
                retention_period_ended: true,
                legal_hold_active: false,
            };

            info!("Customer {} data anonymized successfully", customer_id);
            Ok(result)
        } else {
            Err(Error::InternalServerError("Failed to anonymize customer data".to_string()))
        }
    }

    /// Check customers eligible for data retention cleanup
    pub async fn get_customers_for_retention_cleanup(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<Uuid>> {
        let customer_ids = sqlx::query_scalar!(
            r#"
            SELECT id
            FROM customers
            WHERE tenant_id = $1
              AND data_retention_expires_at IS NOT NULL
              AND data_retention_expires_at <= NOW()
              AND gdpr_deletion_requested = false
              AND deleted_at IS NULL
            "#,
            tenant_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(customer_ids)
    }

    // ============================================================================
    // SECURITY METRICS AND MONITORING
    // ============================================================================

    /// Get customer security metrics
    pub async fn get_security_metrics(&self, tenant_id: Uuid) -> Result<CustomerSecurityMetrics> {
        let metrics = sqlx::query!(
            r#"
            SELECT
                COUNT(*) as total_customers,
                COUNT(*) FILTER (WHERE email_verified = true) as verified_customers,
                COUNT(*) FILTER (WHERE locked_until IS NOT NULL AND locked_until > NOW()) as locked_customers,
                COUNT(*) FILTER (WHERE gdpr_deletion_requested = true) as gdpr_deletion_requests,
                AVG(CASE WHEN privacy_consent_given THEN 1.0 ELSE 0.0 END) as privacy_consent_rate,
                AVG(CASE WHEN marketing_consent_given THEN 1.0 ELSE 0.0 END) as marketing_consent_rate
            FROM customers
            WHERE tenant_id = $1 AND deleted_at IS NULL
            "#,
            tenant_id
        )
        .fetch_one(&self.db)
        .await?;

        let access_events_today = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM customer_data_access_log
            WHERE tenant_id = $1 AND DATE(created_at) = CURRENT_DATE
            "#,
            tenant_id
        )
        .fetch_one(&self.db)
        .await?
        .unwrap_or(0);

        let security_incidents_week = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM customer_audit_log
            WHERE tenant_id = $1
              AND action IN ('security_incident', 'account_locked')
              AND created_at >= NOW() - INTERVAL '7 days'
            "#,
            tenant_id
        )
        .fetch_one(&self.db)
        .await?
        .unwrap_or(0);

        Ok(CustomerSecurityMetrics {
            total_customers: metrics.total_customers.unwrap_or(0),
            verified_customers: metrics.verified_customers.unwrap_or(0),
            locked_customers: metrics.locked_customers.unwrap_or(0),
            gdpr_deletion_requests: metrics.gdpr_deletion_requests.unwrap_or(0),
            privacy_consent_rate: metrics.privacy_consent_rate.unwrap_or(0.0),
            marketing_consent_rate: metrics.marketing_consent_rate.unwrap_or(0.0),
            data_access_events_today: access_events_today,
            security_incidents_this_week: security_incidents_week,
        })
    }

    // ============================================================================
    // INTERNAL HELPER METHODS
    // ============================================================================

    /// Internal method to record consent within a transaction
    async fn record_consent_internal(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        customer_id: Uuid,
        tenant_id: Uuid,
        consent_type: String,
        status: bool,
        consent_method: ConsentMethod,
        consent_text: Option<String>,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        valid_until: Option<DateTime<Utc>>,
    ) -> Result<Uuid> {
        let consent_id = sqlx::query_scalar!(
            r#"
            INSERT INTO customer_consent (
                customer_id, tenant_id, consent_type, status, consent_method,
                consent_text, ip_address, user_agent, valid_until
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
            customer_id,
            tenant_id,
            consent_type,
            status,
            consent_method as ConsentMethod,
            consent_text,
            ip_address,
            user_agent,
            valid_until
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(consent_id)
    }

    /// Internal method to log security incidents
    async fn log_security_incident_internal(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        customer_id: Uuid,
        tenant_id: Uuid,
        incident_type: SecurityIncidentType,
        severity: IncidentSeverity,
        description: String,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
    ) -> Result<Uuid> {
        let incident_id = sqlx::query_scalar!(
            r#"
            INSERT INTO customer_audit_log (
                customer_id, tenant_id, action, entity_type, entity_id,
                performed_by_type, compliance_reason, ip_address, user_agent
            ) VALUES ($1, $2, 'security_incident', 'customer', $1, 'system', $3, $4, $5)
            RETURNING id
            "#,
            customer_id,
            tenant_id,
            description,
            ip_address,
            user_agent
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(incident_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_test_db() -> PgPool {
        // Test database setup would go here
        todo!("Implement test database setup")
    }

    #[tokio::test]
    async fn test_customer_security_metrics() {
        // Test implementation would go here
        todo!("Implement security metrics test")
    }

    #[tokio::test]
    async fn test_gdpr_data_export() {
        // Test implementation would go here
        todo!("Implement GDPR export test")
    }

    #[tokio::test]
    async fn test_customer_anonymization() {
        // Test implementation would go here
        todo!("Implement anonymization test")
    }
}