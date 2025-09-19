// ============================================================================
// OLYMPUS CLOUD - PLATFORM SERVICE EVENT HANDLERS
// ============================================================================
// Module: platform/src/event_handlers.rs
// Description: Event handlers for platform service cross-service communication
// Author: Claude Code Agent
// Date: 2025-01-19
// Version: 1.0 - Phase 5 Event-Driven Architecture
// ============================================================================

use async_trait::async_trait;
use olympus_shared::events::{
    EventHandler, EventContainer,
    TenantCreatedEvent, TenantSubscriptionChangedEvent, UserRoleChangedEvent,
    LocationCreatedEvent, FeatureFlagChangedEvent, UserRegisteredEvent,
    UserLoggedInEvent, OrderCreatedEvent, PaymentProcessedEvent,
    platform_events, auth_events, commerce_events,
};
use olympus_shared::events::subscriber::{HandlerPriority, HandlerHealth};
use olympus_shared::{Error, Result};
use tracing::{info, warn, error, debug, instrument};
use uuid::Uuid;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// ============================================================================
// TENANT LIFECYCLE HANDLER
// ============================================================================

/// Handles tenant lifecycle events and manages multi-tenancy
pub struct TenantLifecycleHandler {
    name: String,
    processed_count: AtomicU64,
    failed_count: AtomicU64,
    tenant_registry: Arc<RwLock<HashMap<Uuid, TenantInfo>>>,
}

#[derive(Debug, Clone)]
struct TenantInfo {
    tenant_id: Uuid,
    slug: String,
    name: String,
    industry: String,
    subscription_tier: String,
    created_at: DateTime<Utc>,
    is_active: bool,
    user_count: u32,
    location_count: u32,
    feature_flags: HashMap<String, bool>,
}

impl TenantLifecycleHandler {
    pub fn new() -> Self {
        Self {
            name: "TenantLifecycleHandler".to_string(),
            processed_count: AtomicU64::new(0),
            failed_count: AtomicU64::new(0),
            tenant_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle new tenant creation
    #[instrument(skip(self, event))]
    async fn handle_tenant_created(&self, event: &TenantCreatedEvent) -> Result<()> {
        info!(
            "Processing new tenant creation: {} ({}) in {} industry",
            event.name, event.slug, event.industry
        );

        // Tenant creation tasks:
        // 1. Set up tenant infrastructure
        // 2. Initialize tenant-specific configurations
        // 3. Create default roles and permissions
        // 4. Set up billing and subscription
        // 5. Initialize feature flags

        let tenant_info = TenantInfo {
            tenant_id: event.tenant_id,
            slug: event.slug.clone(),
            name: event.name.clone(),
            industry: event.industry.clone(),
            subscription_tier: event.subscription_tier.clone(),
            created_at: Utc::now(),
            is_active: true,
            user_count: 0,
            location_count: 0,
            feature_flags: self.get_default_feature_flags(&event.subscription_tier),
        };

        let mut registry = self.tenant_registry.write().await;
        registry.insert(event.tenant_id, tenant_info);

        info!(
            "Tenant {} successfully created with subscription tier: {}",
            event.tenant_id, event.subscription_tier
        );

        // Set up default feature flags based on subscription tier
        self.configure_tenant_features(&event.tenant_id, &event.subscription_tier).await?;

        Ok(())
    }

    /// Handle tenant subscription changes
    #[instrument(skip(self, event))]
    async fn handle_tenant_subscription_changed(&self, event: &TenantSubscriptionChangedEvent) -> Result<()> {
        info!(
            "Processing subscription change for tenant {}: {} -> {}",
            event.tenant_id, event.old_tier, event.new_tier
        );

        // Subscription change tasks:
        // 1. Update tenant subscription information
        // 2. Adjust feature flags based on new tier
        // 3. Update billing configuration
        // 4. Handle tier upgrades/downgrades
        // 5. Notify relevant services

        let mut registry = self.tenant_registry.write().await;
        if let Some(tenant_info) = registry.get_mut(&event.tenant_id) {
            tenant_info.subscription_tier = event.new_tier.clone();
            tenant_info.feature_flags = self.get_default_feature_flags(&event.new_tier);

            info!(
                "Tenant {} subscription updated to {} tier effective {}",
                event.tenant_id, event.new_tier, event.effective_date
            );
        } else {
            warn!("Attempted to update subscription for unknown tenant: {}", event.tenant_id);
        }

        // Update feature flags for the new tier
        self.configure_tenant_features(&event.tenant_id, &event.new_tier).await?;

        Ok(())
    }

    /// Handle user registration to update tenant user count
    #[instrument(skip(self, event))]
    async fn handle_user_registered(&self, event: &UserRegisteredEvent) -> Result<()> {
        debug!("Updating user count for tenant {} due to new user registration", event.tenant_id);

        let mut registry = self.tenant_registry.write().await;
        if let Some(tenant_info) = registry.get_mut(&event.tenant_id) {
            tenant_info.user_count += 1;
            info!("Tenant {} now has {} users", event.tenant_id, tenant_info.user_count);
        }

        Ok(())
    }

    /// Handle location creation to update tenant location count
    #[instrument(skip(self, event))]
    async fn handle_location_created(&self, event: &LocationCreatedEvent) -> Result<()> {
        info!(
            "New location created for tenant {}: {} ({})",
            event.tenant_id, event.name, event.location_id
        );

        let mut registry = self.tenant_registry.write().await;
        if let Some(tenant_info) = registry.get_mut(&event.tenant_id) {
            tenant_info.location_count += 1;
            info!("Tenant {} now has {} locations", event.tenant_id, tenant_info.location_count);
        }

        // Set up location-specific configurations
        self.configure_location_settings(&event.location_id, &event.tenant_id).await?;

        Ok(())
    }

    /// Get default feature flags for a subscription tier
    fn get_default_feature_flags(&self, tier: &str) -> HashMap<String, bool> {
        let mut flags = HashMap::new();

        match tier {
            "basic" => {
                flags.insert("advanced_analytics".to_string(), false);
                flags.insert("api_access".to_string(), true);
                flags.insert("multiple_locations".to_string(), false);
                flags.insert("custom_branding".to_string(), false);
            }
            "professional" => {
                flags.insert("advanced_analytics".to_string(), true);
                flags.insert("api_access".to_string(), true);
                flags.insert("multiple_locations".to_string(), true);
                flags.insert("custom_branding".to_string(), false);
            }
            "enterprise" => {
                flags.insert("advanced_analytics".to_string(), true);
                flags.insert("api_access".to_string(), true);
                flags.insert("multiple_locations".to_string(), true);
                flags.insert("custom_branding".to_string(), true);
                flags.insert("sso_integration".to_string(), true);
                flags.insert("audit_logs".to_string(), true);
            }
            _ => {
                // Default/trial features
                flags.insert("advanced_analytics".to_string(), false);
                flags.insert("api_access".to_string(), false);
                flags.insert("multiple_locations".to_string(), false);
                flags.insert("custom_branding".to_string(), false);
            }
        }

        flags
    }

    /// Configure tenant features based on subscription tier
    async fn configure_tenant_features(&self, tenant_id: &Uuid, tier: &str) -> Result<()> {
        debug!("Configuring features for tenant {} with tier {}", tenant_id, tier);
        // TODO: Actually configure features in the system
        Ok(())
    }

    /// Configure location-specific settings
    async fn configure_location_settings(&self, location_id: &Uuid, tenant_id: &Uuid) -> Result<()> {
        debug!("Configuring settings for location {} in tenant {}", location_id, tenant_id);
        // TODO: Set up location-specific configurations
        Ok(())
    }
}

#[async_trait]
impl EventHandler for TenantLifecycleHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    platform_events::TENANT_CREATED => {
                        if let Ok(tenant_event) = serde_json::from_value::<TenantCreatedEvent>(domain_event.data.clone()) {
                            self.handle_tenant_created(&tenant_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantCreatedEvent".to_string()))
                        }
                    }
                    platform_events::TENANT_SUBSCRIPTION_CHANGED => {
                        if let Ok(subscription_event) = serde_json::from_value::<TenantSubscriptionChangedEvent>(domain_event.data.clone()) {
                            self.handle_tenant_subscription_changed(&subscription_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantSubscriptionChangedEvent".to_string()))
                        }
                    }
                    auth_events::USER_REGISTERED => {
                        if let Ok(user_event) = serde_json::from_value::<UserRegisteredEvent>(domain_event.data.clone()) {
                            self.handle_user_registered(&user_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRegisteredEvent".to_string()))
                        }
                    }
                    platform_events::LOCATION_CREATED => {
                        if let Ok(location_event) = serde_json::from_value::<LocationCreatedEvent>(domain_event.data.clone()) {
                            self.handle_location_created(&location_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize LocationCreatedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Tenant lifecycle handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    platform_events::TENANT_CREATED => {
                        if let Ok(tenant_event) = serde_json::from_value::<TenantCreatedEvent>(versioned_event.data.clone()) {
                            self.handle_tenant_created(&tenant_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantCreatedEvent from versioned event".to_string()))
                        }
                    }
                    platform_events::TENANT_SUBSCRIPTION_CHANGED => {
                        if let Ok(subscription_event) = serde_json::from_value::<TenantSubscriptionChangedEvent>(versioned_event.data.clone()) {
                            self.handle_tenant_subscription_changed(&subscription_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantSubscriptionChangedEvent from versioned event".to_string()))
                        }
                    }
                    auth_events::USER_REGISTERED => {
                        if let Ok(user_event) = serde_json::from_value::<UserRegisteredEvent>(versioned_event.data.clone()) {
                            self.handle_user_registered(&user_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRegisteredEvent from versioned event".to_string()))
                        }
                    }
                    platform_events::LOCATION_CREATED => {
                        if let Ok(location_event) = serde_json::from_value::<LocationCreatedEvent>(versioned_event.data.clone()) {
                            self.handle_location_created(&location_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize LocationCreatedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Tenant lifecycle handler ignoring versioned event type: {}", versioned_event.event_type);
                        Ok(())
                    }
                }
            }
        };

        match &result {
            Ok(_) => {
                self.processed_count.fetch_add(1, Ordering::Relaxed);
            }
            Err(_) => {
                self.failed_count.fetch_add(1, Ordering::Relaxed);
            }
        }

        result
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            platform_events::TENANT_CREATED.to_string(),
            platform_events::TENANT_SUBSCRIPTION_CHANGED.to_string(),
            platform_events::LOCATION_CREATED.to_string(),
            auth_events::USER_REGISTERED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Critical // Tenant management is critical
    }

    fn supports_concurrent_processing(&self) -> bool {
        false // Tenant operations should be sequential for consistency
    }

    fn max_concurrent_events(&self) -> usize {
        1 // Process tenant events sequentially
    }

    async fn health_check(&self) -> HandlerHealth {
        let processed = self.processed_count.load(Ordering::Relaxed);
        let failed = self.failed_count.load(Ordering::Relaxed);

        if processed == 0 {
            return HandlerHealth::Healthy;
        }

        let failure_rate = (failed as f64) / (processed as f64);

        if failure_rate > 0.1 {
            HandlerHealth::Unhealthy(format!("High tenant operation failure rate: {:.2}%", failure_rate * 100.0))
        } else if failure_rate > 0.05 {
            HandlerHealth::Degraded(format!("Elevated tenant operation failure rate: {:.2}%", failure_rate * 100.0))
        } else {
            HandlerHealth::Healthy
        }
    }
}

// ============================================================================
// USER ROLE MANAGEMENT HANDLER
// ============================================================================

/// Handles user role and permission events
pub struct UserRoleManagementHandler {
    name: String,
    processed_count: AtomicU64,
    role_cache: Arc<RwLock<HashMap<Uuid, UserRoleInfo>>>,
}

#[derive(Debug, Clone)]
struct UserRoleInfo {
    user_id: Uuid,
    tenant_id: Uuid,
    roles: Vec<String>,
    permissions: Vec<String>,
    last_updated: DateTime<Utc>,
    updated_by: Uuid,
}

impl UserRoleManagementHandler {
    pub fn new() -> Self {
        Self {
            name: "UserRoleManagementHandler".to_string(),
            processed_count: AtomicU64::new(0),
            role_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Handle user role changes
    #[instrument(skip(self, event))]
    async fn handle_user_role_changed(&self, event: &UserRoleChangedEvent) -> Result<()> {
        info!(
            "Processing role change for user {} in tenant {}: {:?} -> {:?}",
            event.user_id, event.tenant_id, event.old_roles, event.new_roles
        );

        // Role change tasks:
        // 1. Update user role cache
        // 2. Recalculate user permissions
        // 3. Invalidate user sessions if needed
        // 4. Update access control lists
        // 5. Audit role change

        let permissions = self.calculate_permissions(&event.new_roles).await;

        let role_info = UserRoleInfo {
            user_id: event.user_id,
            tenant_id: event.tenant_id,
            roles: event.new_roles.clone(),
            permissions,
            last_updated: Utc::now(),
            updated_by: event.changed_by,
        };

        let mut cache = self.role_cache.write().await;
        cache.insert(event.user_id, role_info);

        // Check if role change requires session invalidation
        if self.requires_session_invalidation(&event.old_roles, &event.new_roles) {
            warn!("Role change requires session invalidation for user {}", event.user_id);
            // TODO: Invalidate user sessions
        }

        info!(
            "Role change completed for user {} - reason: {}",
            event.user_id, event.reason
        );

        Ok(())
    }

    /// Handle user registration to set default roles
    #[instrument(skip(self, event))]
    async fn handle_user_registered(&self, event: &UserRegisteredEvent) -> Result<()> {
        debug!("Setting default roles for newly registered user {}", event.user_id);

        // Set default roles based on registration source and tenant settings
        let default_roles = self.get_default_roles(&event.registration_source, &event.tenant_id).await;
        let permissions = self.calculate_permissions(&default_roles).await;

        let role_info = UserRoleInfo {
            user_id: event.user_id,
            tenant_id: event.tenant_id,
            roles: default_roles,
            permissions,
            last_updated: Utc::now(),
            updated_by: event.user_id, // Self-registration
        };

        let mut cache = self.role_cache.write().await;
        cache.insert(event.user_id, role_info);

        info!("Default roles assigned to new user {}", event.user_id);
        Ok(())
    }

    /// Calculate permissions based on roles
    async fn calculate_permissions(&self, roles: &[String]) -> Vec<String> {
        let mut permissions = Vec::new();

        for role in roles {
            match role.as_str() {
                "admin" => {
                    permissions.extend_from_slice(&[
                        "user.create", "user.read", "user.update", "user.delete",
                        "order.create", "order.read", "order.update", "order.delete",
                        "product.create", "product.read", "product.update", "product.delete",
                        "location.create", "location.read", "location.update", "location.delete",
                        "tenant.read", "tenant.update",
                    ]);
                }
                "manager" => {
                    permissions.extend_from_slice(&[
                        "user.read", "user.update",
                        "order.create", "order.read", "order.update",
                        "product.read", "product.update",
                        "location.read", "location.update",
                    ]);
                }
                "employee" => {
                    permissions.extend_from_slice(&[
                        "order.create", "order.read", "order.update",
                        "product.read",
                    ]);
                }
                "customer" => {
                    permissions.extend_from_slice(&[
                        "order.create", "order.read",
                    ]);
                }
                _ => {
                    // Unknown role, no permissions
                }
            }
        }

        // Remove duplicates
        permissions.sort();
        permissions.dedup();
        permissions.into_iter().map(|s| s.to_string()).collect()
    }

    /// Get default roles for new users
    async fn get_default_roles(&self, registration_source: &str, _tenant_id: &Uuid) -> Vec<String> {
        match registration_source {
            "invite" => vec!["employee".to_string()],
            "admin" => vec!["admin".to_string()],
            _ => vec!["customer".to_string()],
        }
    }

    /// Check if role change requires session invalidation
    fn requires_session_invalidation(&self, old_roles: &[String], new_roles: &[String]) -> bool {
        // Invalidate if roles were removed or if admin role was added/removed
        old_roles.len() > new_roles.len() ||
        (old_roles.contains(&"admin".to_string()) != new_roles.contains(&"admin".to_string()))
    }
}

#[async_trait]
impl EventHandler for UserRoleManagementHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    platform_events::USER_ROLE_CHANGED => {
                        if let Ok(role_event) = serde_json::from_value::<UserRoleChangedEvent>(domain_event.data.clone()) {
                            self.handle_user_role_changed(&role_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRoleChangedEvent".to_string()))
                        }
                    }
                    auth_events::USER_REGISTERED => {
                        if let Ok(user_event) = serde_json::from_value::<UserRegisteredEvent>(domain_event.data.clone()) {
                            self.handle_user_registered(&user_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRegisteredEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Role management handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    platform_events::USER_ROLE_CHANGED => {
                        if let Ok(role_event) = serde_json::from_value::<UserRoleChangedEvent>(versioned_event.data.clone()) {
                            self.handle_user_role_changed(&role_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRoleChangedEvent from versioned event".to_string()))
                        }
                    }
                    auth_events::USER_REGISTERED => {
                        if let Ok(user_event) = serde_json::from_value::<UserRegisteredEvent>(versioned_event.data.clone()) {
                            self.handle_user_registered(&user_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserRegisteredEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Role management handler ignoring versioned event type: {}", versioned_event.event_type);
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
            platform_events::USER_ROLE_CHANGED.to_string(),
            auth_events::USER_REGISTERED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::High // Role management is high priority
    }

    fn supports_concurrent_processing(&self) -> bool {
        false // Role changes should be sequential for consistency
    }

    fn max_concurrent_events(&self) -> usize {
        1 // Process role events sequentially
    }

    async fn health_check(&self) -> HandlerHealth {
        let processed = self.processed_count.load(Ordering::Relaxed);

        if processed > 0 {
            HandlerHealth::Healthy
        } else {
            HandlerHealth::Degraded("No role events processed yet".to_string())
        }
    }
}

// ============================================================================
// PLATFORM ANALYTICS HANDLER
// ============================================================================

/// Handles platform-wide analytics and usage metrics
pub struct PlatformAnalyticsHandler {
    name: String,
    processed_count: AtomicU64,
    analytics_cache: Arc<RwLock<PlatformAnalytics>>,
}

#[derive(Debug, Clone)]
struct PlatformAnalytics {
    tenant_count: u32,
    active_users: u32,
    daily_logins: HashMap<String, u32>, // Date -> Login count
    order_volume: HashMap<String, u32>, // Date -> Order count
    revenue_tracking: HashMap<String, f64>, // Date -> Revenue
    feature_usage: HashMap<String, u32>, // Feature -> Usage count
}

impl PlatformAnalyticsHandler {
    pub fn new() -> Self {
        Self {
            name: "PlatformAnalyticsHandler".to_string(),
            processed_count: AtomicU64::new(0),
            analytics_cache: Arc::new(RwLock::new(PlatformAnalytics {
                tenant_count: 0,
                active_users: 0,
                daily_logins: HashMap::new(),
                order_volume: HashMap::new(),
                revenue_tracking: HashMap::new(),
                feature_usage: HashMap::new(),
            })),
        }
    }

    /// Process tenant creation for analytics
    #[instrument(skip(self, event))]
    async fn process_tenant_analytics(&self, event: &TenantCreatedEvent) -> Result<()> {
        let mut analytics = self.analytics_cache.write().await;
        analytics.tenant_count += 1;

        info!(
            "Platform analytics: Tenant count increased to {}",
            analytics.tenant_count
        );

        Ok(())
    }

    /// Process user login for analytics
    #[instrument(skip(self, event))]
    async fn process_login_analytics(&self, event: &UserLoggedInEvent) -> Result<()> {
        let mut analytics = self.analytics_cache.write().await;

        let date_key = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let login_count = analytics.daily_logins.entry(date_key).or_insert(0);
        *login_count += 1;

        debug!("Platform analytics: Daily login count updated");
        Ok(())
    }

    /// Process order creation for analytics
    #[instrument(skip(self, event))]
    async fn process_order_analytics(&self, event: &OrderCreatedEvent) -> Result<()> {
        let mut analytics = self.analytics_cache.write().await;

        let date_key = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let order_count = analytics.order_volume.entry(date_key).or_insert(0);
        *order_count += 1;

        debug!("Platform analytics: Daily order volume updated");
        Ok(())
    }

    /// Process feature flag usage
    #[instrument(skip(self, event))]
    async fn process_feature_analytics(&self, event: &FeatureFlagChangedEvent) -> Result<()> {
        let mut analytics = self.analytics_cache.write().await;

        let usage_count = analytics.feature_usage.entry(event.feature_name.clone()).or_insert(0);
        *usage_count += 1;

        debug!("Platform analytics: Feature usage tracked for {}", event.feature_name);
        Ok(())
    }
}

#[async_trait]
impl EventHandler for PlatformAnalyticsHandler {
    #[instrument(skip(self, event))]
    async fn handle(&self, event: &EventContainer) -> Result<()> {
        let result = match event {
            EventContainer::Legacy(domain_event) => {
                match domain_event.event_type.as_str() {
                    platform_events::TENANT_CREATED => {
                        if let Ok(tenant_event) = serde_json::from_value::<TenantCreatedEvent>(domain_event.data.clone()) {
                            self.process_tenant_analytics(&tenant_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantCreatedEvent".to_string()))
                        }
                    }
                    auth_events::USER_LOGGED_IN => {
                        if let Ok(login_event) = serde_json::from_value::<UserLoggedInEvent>(domain_event.data.clone()) {
                            self.process_login_analytics(&login_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserLoggedInEvent".to_string()))
                        }
                    }
                    commerce_events::ORDER_CREATED => {
                        if let Ok(order_event) = serde_json::from_value::<OrderCreatedEvent>(domain_event.data.clone()) {
                            self.process_order_analytics(&order_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderCreatedEvent".to_string()))
                        }
                    }
                    platform_events::FEATURE_FLAG_CHANGED => {
                        if let Ok(feature_event) = serde_json::from_value::<FeatureFlagChangedEvent>(domain_event.data.clone()) {
                            self.process_feature_analytics(&feature_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize FeatureFlagChangedEvent".to_string()))
                        }
                    }
                    _ => {
                        debug!("Platform analytics handler ignoring event type: {}", domain_event.event_type);
                        Ok(())
                    }
                }
            }
            EventContainer::Versioned(versioned_event) => {
                match versioned_event.event_type.as_str() {
                    platform_events::TENANT_CREATED => {
                        if let Ok(tenant_event) = serde_json::from_value::<TenantCreatedEvent>(versioned_event.data.clone()) {
                            self.process_tenant_analytics(&tenant_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize TenantCreatedEvent from versioned event".to_string()))
                        }
                    }
                    auth_events::USER_LOGGED_IN => {
                        if let Ok(login_event) = serde_json::from_value::<UserLoggedInEvent>(versioned_event.data.clone()) {
                            self.process_login_analytics(&login_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize UserLoggedInEvent from versioned event".to_string()))
                        }
                    }
                    commerce_events::ORDER_CREATED => {
                        if let Ok(order_event) = serde_json::from_value::<OrderCreatedEvent>(versioned_event.data.clone()) {
                            self.process_order_analytics(&order_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize OrderCreatedEvent from versioned event".to_string()))
                        }
                    }
                    platform_events::FEATURE_FLAG_CHANGED => {
                        if let Ok(feature_event) = serde_json::from_value::<FeatureFlagChangedEvent>(versioned_event.data.clone()) {
                            self.process_feature_analytics(&feature_event).await
                        } else {
                            Err(Error::Internal("Failed to deserialize FeatureFlagChangedEvent from versioned event".to_string()))
                        }
                    }
                    _ => {
                        debug!("Platform analytics handler ignoring versioned event type: {}", versioned_event.event_type);
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
            platform_events::TENANT_CREATED.to_string(),
            platform_events::FEATURE_FLAG_CHANGED.to_string(),
            auth_events::USER_LOGGED_IN.to_string(),
            commerce_events::ORDER_CREATED.to_string(),
        ]
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn priority(&self) -> HandlerPriority {
        HandlerPriority::Low // Analytics is important but not urgent
    }

    fn supports_concurrent_processing(&self) -> bool {
        true // Analytics can be processed concurrently
    }

    fn max_concurrent_events(&self) -> usize {
        10 // Process up to 10 analytics events concurrently
    }

    async fn health_check(&self) -> HandlerHealth {
        HandlerHealth::Healthy // Analytics handler is always healthy
    }
}

// ============================================================================
// HANDLER FACTORY
// ============================================================================

/// Factory for creating platform service event handlers
pub struct PlatformEventHandlerFactory;

impl PlatformEventHandlerFactory {
    /// Create all platform service event handlers
    pub fn create_handlers() -> Vec<Arc<dyn EventHandler>> {
        vec![
            Arc::new(TenantLifecycleHandler::new()),
            Arc::new(UserRoleManagementHandler::new()),
            Arc::new(PlatformAnalyticsHandler::new()),
        ]
    }

    /// Create a specific handler by name
    pub fn create_handler(name: &str) -> Option<Arc<dyn EventHandler>> {
        match name {
            "TenantLifecycleHandler" => Some(Arc::new(TenantLifecycleHandler::new())),
            "UserRoleManagementHandler" => Some(Arc::new(UserRoleManagementHandler::new())),
            "PlatformAnalyticsHandler" => Some(Arc::new(PlatformAnalyticsHandler::new())),
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

    fn create_test_tenant_created_event() -> TenantCreatedEvent {
        TenantCreatedEvent {
            tenant_id: Uuid::new_v4(),
            slug: "test-tenant".to_string(),
            name: "Test Tenant".to_string(),
            industry: "restaurant".to_string(),
            subscription_tier: "professional".to_string(),
            created_by: Uuid::new_v4(),
            trial_ends_at: Some(Utc::now() + chrono::Duration::days(30)),
        }
    }

    fn create_test_domain_event(event_type: &str, data: serde_json::Value) -> EventContainer {
        let domain_event = DomainEvent {
            id: Uuid::new_v4(),
            event_type: event_type.to_string(),
            aggregate_id: Uuid::new_v4(),
            aggregate_type: "Tenant".to_string(),
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
    async fn test_tenant_lifecycle_handler() {
        let handler = TenantLifecycleHandler::new();
        let tenant_event = create_test_tenant_created_event();
        let event_data = serde_json::to_value(&tenant_event).unwrap();
        let event = create_test_domain_event(platform_events::TENANT_CREATED, event_data);

        let result = handler.handle(&event).await;
        assert!(result.is_ok());
        assert_eq!(handler.processed_count.load(Ordering::Relaxed), 1);

        let health = handler.health_check().await;
        assert_eq!(health, HandlerHealth::Healthy);
    }

    #[tokio::test]
    async fn test_user_role_management_handler() {
        let handler = UserRoleManagementHandler::new();
        assert_eq!(handler.name(), "UserRoleManagementHandler");
        assert_eq!(handler.priority(), HandlerPriority::High);
        assert!(!handler.supports_concurrent_processing()); // Sequential processing
        assert_eq!(handler.max_concurrent_events(), 1);
    }

    #[tokio::test]
    async fn test_platform_analytics_handler() {
        let handler = PlatformAnalyticsHandler::new();
        let tenant_event = create_test_tenant_created_event();
        let event_data = serde_json::to_value(&tenant_event).unwrap();
        let event = create_test_domain_event(platform_events::TENANT_CREATED, event_data);

        let result = handler.handle(&event).await;
        assert!(result.is_ok());
        assert_eq!(handler.processed_count.load(Ordering::Relaxed), 1);

        // Analytics handler supports concurrent processing
        assert!(handler.supports_concurrent_processing());
        assert_eq!(handler.priority(), HandlerPriority::Low);
    }

    #[test]
    fn test_handler_factory() {
        let handlers = PlatformEventHandlerFactory::create_handlers();
        assert_eq!(handlers.len(), 3);

        let specific_handler = PlatformEventHandlerFactory::create_handler("TenantLifecycleHandler");
        assert!(specific_handler.is_some());

        let unknown_handler = PlatformEventHandlerFactory::create_handler("UnknownHandler");
        assert!(unknown_handler.is_none());
    }
}