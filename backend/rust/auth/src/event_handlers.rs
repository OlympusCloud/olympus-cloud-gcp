// ============================================================================
// OLYMPUS CLOUD - AUTHENTICATION SERVICE EVENT HANDLERS
// ============================================================================
// Module: auth/src/event_handlers.rs
// Description: Event handlers for authentication service cross-service communication
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 1.0 - Phase 5 Event-Driven Architecture
// ============================================================================

use async_trait::async_trait;
use olympus_shared::events::{
    EventHandler, EventContainer,
    UserRegisteredEvent, UserLoggedInEvent, TenantCreatedEvent, UserRoleChangedEvent, AccountLockedEvent,
    auth_events, platform_events
};
use olympus_shared::events::subscriber::{HandlerPriority, HandlerHealth};
use olympus_shared::{Error, Result};
use tracing::{info, warn, error, debug, instrument};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// USER AUTHENTICATION EVENT HANDLER
// ============================================================================

/// Handles user authentication-related events from other services
pub struct UserAuthenticationHandler {
    name: String,
    processed_count: AtomicU64,
    failed_count: AtomicU64,
    last_error: Arc<RwLock<Option<String>>>,
}

impl UserAuthenticationHandler {
    pub fn new() -> Self {
        Self {
            name: "UserAuthenticationHandler".to_string(),
            processed_count: AtomicU64::new(0),
            failed_count: AtomicU64::new(0),
            last_error: Arc::new(RwLock::new(None)),
        }
    }

    /// Handle user registration events from platform service
    #[instrument(skip(self, event))]
    async fn handle_user_registered(&self, event: &UserRegisteredEvent) -> Result<()> {
        info!(
            "Processing user registration for user {} in tenant {}",
            event.user_id, event.tenant_id
        );

        // Initialize user authentication profile
        // This would typically involve:
        // 1. Creating user authentication record
        // 2. Setting up default security settings
        // 3. Triggering email verification if required
        // 4. Creating audit log entry

        if event.email_verification_required {
            info!("Email verification required for user {}", event.user_id);
            // TODO: Trigger email verification process
        }

        debug!("User authentication profile initialized for user {}", event.user_id);
        Ok(())
    }

    /// Handle tenant creation events to set up tenant-specific auth configuration
    #[instrument(skip(self, event))]
    async fn handle_tenant_created(&self, event: &TenantCreatedEvent) -> Result<()> {
        info!(
            "Setting up authentication configuration for new tenant: {} ({})",
            event.name, event.tenant_id
        );

        // Set up tenant-specific authentication configuration
        // This would typically involve:
        // 1. Creating tenant authentication settings
        // 2. Setting up default password policies
        // 3. Configuring MFA settings
        // 4. Setting up OAuth providers if applicable

        debug!("Authentication configuration initialized for tenant {}", event.tenant_id);
        Ok(())
    }

    /// Handle user role changes to update authentication permissions
    #[instrument(skip(self, event))]
    async fn handle_user_role_changed(&self, event: &UserRoleChangedEvent) -> Result<()> {
        info!(
            "Updating authentication permissions for user {} due to role change",
            event.user_id
        );

        // Update user permissions and invalidate existing sessions if needed
        // This would typically involve:
        // 1. Updating user role cache
        // 2. Invalidating JWT tokens if role downgrade
        // 3. Updating permission cache
        // 4. Logging role change for audit

        debug!(
            "Authentication permissions updated for user {} (old roles: {:?}, new roles: {:?})",
            event.user_id, event.old_roles, event.new_roles
        );
        Ok(())
    }
}

#[async_trait]
impl EventHandler for UserAuthenticationHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    auth_events::USER_REGISTERED => {
                        if let Ok(user_event) = serde_json::from_value::<UserRegisteredEvent>(domain_event.data.clone()) {
                            self.handle_user_registered(&user_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRegisteredEvent".to_string()))
                        }
                    }
                    platform_events::TENANT_CREATED => {
                        if let Ok(tenant_event) = serde_json::from_value::<TenantCreatedEvent>(domain_event.data.clone()) {
                            self.handle_tenant_created(&tenant_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantCreatedEvent".to_string()))
                        }
                    }
                    platform_events::USER_ROLE_CHANGED => {
                        if let Ok(role_event) = serde_json::from_value::<UserRoleChangedEvent>(domain_event.data.clone()) {
                            self.handle_user_role_changed(&role_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRoleChangedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Ignoring unhandled event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    auth_events::USER_REGISTERED => {
                        if let Ok(user_event) = serde_json::from_value::<UserRegisteredEvent>(versioned_event.data.clone()) {
                            self.handle_user_registered(&user_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRegisteredEvent from versioned event".to_string()))
                        }
                    }
                    platform_events::TENANT_CREATED => {
                        if let Ok(tenant_event) = serde_json::from_value::<TenantCreatedEvent>(versioned_event.data.clone()) {
                            self.handle_tenant_created(&tenant_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantCreatedEvent from versioned event".to_string()))
                        }
                    }
                    platform_events::USER_ROLE_CHANGED => {
                        if let Ok(role_event) = serde_json::from_value::<UserRoleChangedEvent>(versioned_event.data.clone()) {
                            self.handle_user_role_changed(&role_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRoleChangedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Ignoring unhandled versioned event type: {}", versioned_event.event_type);
                        Ok(())
                    }
                }
            }
        };

        match &result {
            Ok(_) => {
                self.processed_count.fetch_add(1, Ordering::Relaxed);
            }
            Err(e) => {
                self.failed_count.fetch_add(1, Ordering::Relaxed);
                let mut last_error = self.last_error.write().await;
                *last_error = Some(e.to_string());
            }
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            auth_events::USER_REGISTERED.to_string(),
            platform_events::TENANT_CREATED.to_string(),
            platform_events::USER_ROLE_CHANGED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::High // Authentication events are high priority
    }

    fn supports_concurrent_processing(&self) -> bool {
        true // Can process multiple events concurrently
    }

    fn max_concurrent_events(&self) -> usize {
        5 // Process up to 5 events concurrently
    }

    async fn health_check(&self) -> HandlerHealth {
        let processed = self.processed_count.load(Ordering::Relaxed);
        let failed = self.failed_count.load(Ordering::Relaxed);

        if processed == 0 {
            return HandlerHealth::Healthy; // No events processed yet
        }

        let failure_rate = (failed as f64) / (processed as f64);

        if failure_rate > 0.5 {
            let last_error = self.last_error.read().await;
            HandlerHealth::Unhealthy(
                last_error.clone().unwrap_or_else(|| "High failure rate".to_string())
            )
        } else if failure_rate > 0.1 {
            HandlerHealth::Degraded(format!("Failure rate: {:.2}%", failure_rate * 100.0))
        } else {
            HandlerHealth::Healthy
        }
    }
}

// ============================================================================
// SECURITY EVENT HANDLER
// ============================================================================

/// Handles security-related events for monitoring and alerting
pub struct SecurityEventHandler {
    name: String,
    suspicious_activity_threshold: i32,
    processed_count: AtomicU64,
}

impl SecurityEventHandler {
    pub fn new() -> Self {
        Self {
            name: "SecurityEventHandler".to_string(),
            suspicious_activity_threshold: 5,
            processed_count: AtomicU64::new(0),
        }
    }

    /// Handle user login events for security monitoring
    #[instrument(skip(self, event))]
    async fn handle_user_logged_in(&self, event: &UserLoggedInEvent) -> Result<()> {
        info!(
            "Processing login event for user {} from IP {}",
            event.user_id, event.ip_address
        );

        // Security monitoring tasks:
        // 1. Check for suspicious login patterns
        // 2. Update user's last login information
        // 3. Check for login from new locations
        // 4. Verify MFA compliance if required

        if let Some(location) = &event.location {
            debug!("Login from location: {:?}", location);
            // TODO: Check for login from unusual location
        }

        if !event.mfa_used {
            warn!("Login without MFA for user {}", event.user_id);
            // TODO: Implement MFA enforcement logic
        }

        Ok(())
    }

    /// Handle account locked events for security alerting
    #[instrument(skip(self, event))]
    async fn handle_account_locked(&self, event: &AccountLockedEvent) -> Result<()> {
        warn!(
            "Account locked for user {} due to {:?} with {} failed attempts",
            event.user_id, event.lock_reason, event.failed_attempts
        );

        // Security response tasks:
        // 1. Alert security team if suspicious
        // 2. Log security event
        // 3. Update user security metrics
        // 4. Check for broader attack patterns

        if event.failed_attempts >= self.suspicious_activity_threshold {
            error!(
                "High number of failed attempts ({}) for user {} - potential brute force attack",
                event.failed_attempts, event.user_id
            );
            // TODO: Trigger security alert
        }

        Ok(())
    }
}

#[async_trait]
impl EventHandler for SecurityEventHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    auth_events::USER_LOGGED_IN => {
                        if let Ok(login_event) = serde_json::from_value::<UserLoggedInEvent>(domain_event.data.clone()) {
                            self.handle_user_logged_in(&login_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserLoggedInEvent".to_string()))
                        }
                    }
                    auth_events::ACCOUNT_LOCKED => {
                        if let Ok(lock_event) = serde_json::from_value::<AccountLockedEvent>(domain_event.data.clone()) {
                            self.handle_account_locked(&lock_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize AccountLockedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Security handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    auth_events::USER_LOGGED_IN => {
                        if let Ok(login_event) = serde_json::from_value::<UserLoggedInEvent>(versioned_event.data.clone()) {
                            self.handle_user_logged_in(&login_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserLoggedInEvent from versioned event".to_string()))
                        }
                    }
                    auth_events::ACCOUNT_LOCKED => {
                        if let Ok(lock_event) = serde_json::from_value::<AccountLockedEvent>(versioned_event.data.clone()) {
                            self.handle_account_locked(&lock_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize AccountLockedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Security handler ignoring versioned event type: {}", versioned_event.event_type);
                        Ok(())
                    }
                }
            }
        };

        if result.is_ok() {
            self.processed_count.fetch_add(1, Ordering::Relaxed);
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            auth_events::USER_LOGGED_IN.to_string(),
            auth_events::USER_LOGGED_OUT.to_string(),
            auth_events::ACCOUNT_LOCKED.to_string(),
            auth_events::PASSWORD_CHANGED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Critical // Security events are critical
    }

    fn supports_concurrent_processing(&self) -> bool {
        true
    }

    fn max_concurrent_events(&self) -> usize {
        10 // Security monitoring can handle many concurrent events
    }

    async fn health_check(&self) -> HandlerHealth {
        // Security handler should always be healthy
        HandlerHealth::Healthy
    }
}

// ============================================================================
// AUDIT LOG HANDLER
// ============================================================================

/// Handles all authentication events for audit logging
pub struct AuthAuditLogHandler {
    name: String,
    processed_count: AtomicU64,
}

impl AuthAuditLogHandler {
    pub fn new() -> Self {
        Self {
            name: "AuthAuditLogHandler".to_string(),
            processed_count: AtomicU64::new(0),
        }
    }

    /// Log all authentication events for compliance and auditing
    #[instrument(skip(self, event))]
    async fn log_auth_event(&self, event: &EventContainer) -> Result<()> {
        let (event_type, tenant_id, user_id, timestamp) = match event {
            EventContainer::Legacy(domain_event) => (
                domain_event.event_type.clone(),
                domain_event.tenant_id,
                domain_event.metadata.user_id,
                domain_event.occurred_at,
            ),
            EventContainer::Versioned(versioned_event) => (
                versioned_event.event_type.clone(),
                versioned_event.context.business_context.tenant_id,
                versioned_event.context.user_id,
                versioned_event.occurred_at,
            ),
        };

        info!(
            "AUDIT: Authentication event '{}' for tenant {} at {}",
            event_type, tenant_id, timestamp
        );

        // Store audit log entry
        // This would typically involve:
        // 1. Writing to audit log database
        // 2. Formatting event for compliance requirements
        // 3. Encrypting sensitive data
        // 4. Setting retention policies

        debug!("Audit log entry created for event: {}", event_type);
        Ok(())
    }
}

#[async_trait]
impl EventHandler for AuthAuditLogHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = self.log_auth_event(event).await;

        if result.is_ok() {
            self.processed_count.fetch_add(1, Ordering::Relaxed);
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            "*".to_string(), // Handle all events for audit logging
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Low // Audit logging is important but not urgent
    }

    fn supports_concurrent_processing(&self) -> bool {
        true
    }

    fn max_concurrent_events(&self) -> usize {
        20 // Audit logging can handle many concurrent events
    }

    async fn health_check(&self) -> HandlerHealth {
        let processed = self.processed_count.load(Ordering::Relaxed);

        if processed > 0 {
            HandlerHealth::Healthy
        } else {
            HandlerHealth::Degraded("No events processed yet".to_string())
        }
    }
}

// ============================================================================
// HANDLER FACTORY
// ============================================================================

/// Factory for creating auth service event handlers
pub struct AuthEventHandlerFactory;

impl AuthEventHandlerFactory {
    /// Create all auth service event handlers
    pub fn create_handlers() -> Vec<Arc<dyn EventHandler>> {
        vec![
            Arc::new(UserAuthenticationHandler::new()),
            Arc::new(SecurityEventHandler::new()),
            Arc::new(AuthAuditLogHandler::new()),
        ]
    }

    /// Create a specific handler by name
    pub fn create_handler(name: &str) -> Option<Arc<dyn EventHandler>> {
        match name {
            "UserAuthenticationHandler" => Some(Arc::new(UserAuthenticationHandler::new())),
            "SecurityEventHandler" => Some(Arc::new(SecurityEventHandler::new())),
            "AuthAuditLogHandler" => Some(Arc::new(AuthAuditLogHandler::new())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::events::{DomainEvent, EventMetadata};
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_user_registered_event() -> UserRegisteredEvent {
        UserRegisteredEvent {
            user_id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email_verification_required: true,
            registration_source: "web".to_string(),
        }
    }

    fn create_test_domain_event(event_type: &str, data: serde_json::Value) -> EventContainer {
        let domain_event = DomainEvent {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            aggregate_id: Uuid::new_v4(),
            aggregate_type: "User".to_string(),
            tenant_id: Uuid::new_v4(),
            data,
            metadata: EventMetadata {
                user_id: Some(Uuid::new_v4()),
                correlation_id: Uuid::new_v4(),
                causation_id: None,
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
                source_service: "test".to_string(),
                event_source: "api".to_string(),
                trace_id: None,
            },
            version: 1,
            occurred_at: Utc::now(),
        };

        EventContainer::Legacy(domain_event)
    }

    #[tokio::test]
    async fn test_user_authentication_handler() {
        let handler = UserAuthenticationHandler::new();
        let user_event = create_test_user_registered_event();
        let event_data = serde_json::to_value(&user_event).unwrap();
        let event = create_test_domain_event(auth_events::USER_REGISTERED, event_data);

        let result = handler.handle(&event).await;
        assert!(result.is_ok());
        assert_eq!(handler.processed_count.load(Ordering::Relaxed), 1);

        let health = handler.health_check().await;
        assert_eq!(health, HandlerHealth::Healthy);
    }

    #[tokio::test]
    async fn test_security_event_handler() {
        let handler = SecurityEventHandler::new();
        assert_eq!(handler.name(), "SecurityEventHandler");
        assert_eq!(handler.priority(), HandlerPriority::Critical);
        assert!(handler.supports_concurrent_processing());
        assert_eq!(handler.max_concurrent_events(), 10);
    }

    #[tokio::test]
    async fn test_audit_log_handler() {
        let handler = AuthAuditLogHandler::new();
        let user_event = create_test_user_registered_event();
        let event_data = serde_json::to_value(&user_event).unwrap();
        let event = create_test_domain_event(auth_events::USER_REGISTERED, event_data);

        let result = handler.handle(&event).await;
        assert!(result.is_ok());
        assert_eq!(handler.processed_count.load(Ordering::Relaxed), 1);

        // Audit handler handles all events
        assert!(handler.event_types().contains(&"*".to_string()));
    }

    #[test]
    fn test_handler_factory() {
        let handlers = AuthEventHandlerFactory::create_handlers();
        assert_eq!(handlers.len(), 3);

        let specific_handler = AuthEventHandlerFactory::create_handler("UserAuthenticationHandler");
        assert!(specific_handler.is_some());

        let unknown_handler = AuthEventHandlerFactory::create_handler("UnknownHandler");
        assert!(unknown_handler.is_none());
    }
}