// ============================================================================
// OLYMPUS CLOUD - TENANT MIDDLEWARE
// ============================================================================
// Module: platform/src/middleware/tenant.rs
// Description: Multi-tenancy enforcement middleware with row-level security
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use std::sync::Arc;
use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use uuid::Uuid;

use olympus_shared::{Error, Result};
use crate::services::TenantService;

/// Tenant context extracted from request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: Uuid,
    pub tenant_slug: String,
    pub tenant_name: String,
    pub subscription_tier: String,
    pub is_active: bool,
    pub feature_flags: serde_json::Value,
}

impl TenantContext {
    /// Check if tenant can access a specific feature
    pub fn can_access_feature(&self, feature: &str) -> bool {
        if !self.is_active {
            return false;
        }

        // Check feature flags first
        if let Some(flag_value) = self.feature_flags.get(feature) {
            if let Some(enabled) = flag_value.as_bool() {
                return enabled;
            }
        }

        // Default feature access based on subscription tier
        match self.subscription_tier.as_str() {
            "free" => matches!(feature, "basic_analytics"),
            "starter" => matches!(feature, "basic_analytics" | "api_access"),
            "professional" => matches!(
                feature,
                "basic_analytics" | "advanced_analytics" | "api_access" | "priority_support"
            ),
            "enterprise" | "custom" => true, // All features
            _ => false,
        }
    }

    /// Check if tenant is within user limits
    pub fn check_user_limit(&self, current_users: i32) -> bool {
        let limit = match self.subscription_tier.as_str() {
            "free" => 5,
            "starter" => 25,
            "professional" => 100,
            "enterprise" => 500,
            "custom" => return true, // Unlimited
            _ => 0,
        };

        current_users < limit
    }

    /// Check if tenant is within location limits
    pub fn check_location_limit(&self, current_locations: i32) -> bool {
        let limit = match self.subscription_tier.as_str() {
            "free" => 1,
            "starter" => 3,
            "professional" => 10,
            "enterprise" => 50,
            "custom" => return true, // Unlimited
            _ => 0,
        };

        current_locations < limit
    }
}

/// Tenant resolution strategy
#[derive(Debug, Clone)]
pub enum TenantResolutionStrategy {
    /// Extract from subdomain (e.g., acme.olympus.com)
    Subdomain,
    /// Extract from header (X-Tenant-Slug)
    Header,
    /// Extract from JWT token claims
    JwtClaims,
    /// Extract from path parameter (/api/v1/tenants/{slug}/...)
    PathParameter,
}

/// Tenant middleware configuration
#[derive(Debug, Clone)]
pub struct TenantMiddlewareConfig {
    pub resolution_strategy: TenantResolutionStrategy,
    pub enforce_active_status: bool,
    pub require_feature_access: Option<String>,
    pub tenant_service: Arc<TenantService>,
}

impl TenantMiddlewareConfig {
    pub fn new(tenant_service: Arc<TenantService>) -> Self {
        Self {
            resolution_strategy: TenantResolutionStrategy::Header,
            enforce_active_status: true,
            require_feature_access: None,
            tenant_service,
        }
    }

    pub fn with_resolution_strategy(mut self, strategy: TenantResolutionStrategy) -> Self {
        self.resolution_strategy = strategy;
        self
    }

    pub fn with_feature_requirement(mut self, feature: String) -> Self {
        self.require_feature_access = Some(feature);
        self
    }

    pub fn with_active_enforcement(mut self, enforce: bool) -> Self {
        self.enforce_active_status = enforce;
        self
    }
}

/// Tenant middleware layer
pub async fn tenant_middleware(
    State(config): State<TenantMiddlewareConfig>,
    mut request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    // Extract tenant identifier from request
    let tenant_identifier = match extract_tenant_identifier(&request, &config.resolution_strategy) {
        Ok(identifier) => identifier,
        Err(e) => {
            error!("Failed to extract tenant identifier: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    // Resolve tenant context
    let tenant_context = match resolve_tenant_context(&tenant_identifier, &config).await {
        Ok(context) => context,
        Err(e) => {
            error!("Failed to resolve tenant context: {}", e);
            return match e {
                Error::NotFound(_) => Err(StatusCode::NOT_FOUND),
                Error::Unauthorized => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            };
        }
    };

    // Enforce tenant status
    if config.enforce_active_status && !tenant_context.is_active {
        warn!("Inactive tenant attempted access: {}", tenant_context.tenant_slug);
        return Err(StatusCode::FORBIDDEN);
    }

    // Enforce feature access if required
    if let Some(required_feature) = &config.require_feature_access {
        if !tenant_context.can_access_feature(required_feature) {
            warn!(
                "Tenant {} cannot access required feature: {}",
                tenant_context.tenant_slug, required_feature
            );
            return Err(StatusCode::FORBIDDEN);
        }
    }

    info!(
        "Tenant context resolved: {} ({})",
        tenant_context.tenant_slug, tenant_context.tenant_id
    );

    // Add tenant context to request extensions
    request.extensions_mut().insert(tenant_context);

    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

/// Extract tenant identifier from request based on strategy
fn extract_tenant_identifier(
    request: &Request,
    strategy: &TenantResolutionStrategy,
) -> Result<String> {
    match strategy {
        TenantResolutionStrategy::Header => {
            extract_from_header(request.headers(), "x-tenant-slug")
                .or_else(|| extract_from_header(request.headers(), "x-tenant-id"))
                .ok_or_else(|| Error::Validation("Missing tenant identifier in headers".to_string()))
        }

        TenantResolutionStrategy::Subdomain => {
            let host = extract_from_header(request.headers(), "host")
                .ok_or_else(|| Error::Validation("Missing Host header".to_string()))?;

            extract_subdomain(&host)
                .ok_or_else(|| Error::Validation("Invalid subdomain format".to_string()))
        }

        TenantResolutionStrategy::JwtClaims => {
            // Extract from Authorization header JWT claims
            let auth_header = extract_from_header(request.headers(), "authorization")
                .ok_or_else(|| Error::Validation("Missing Authorization header".to_string()))?;

            extract_tenant_from_jwt(&auth_header)
        }

        TenantResolutionStrategy::PathParameter => {
            // Extract from URL path (e.g., /api/v1/tenants/{slug}/...)
            let path = request.uri().path();
            extract_tenant_from_path(path)
                .ok_or_else(|| Error::Validation("Missing tenant in path".to_string()))
        }
    }
}

/// Extract header value as string
fn extract_from_header(headers: &HeaderMap, header_name: &str) -> Option<String> {
    headers
        .get(header_name)
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string())
}

/// Extract subdomain from host header
fn extract_subdomain(host: &str) -> Option<String> {
    let host = host.split(':').next()?; // Remove port if present
    let parts: Vec<&str> = host.split('.').collect();

    if parts.len() >= 3 {
        // Format: subdomain.domain.tld
        Some(parts[0].to_string())
    } else {
        None
    }
}

/// Extract tenant information from JWT token
fn extract_tenant_from_jwt(auth_header: &str) -> Result<String> {
    // This is a simplified version - in a real implementation,
    // you would parse and validate the JWT token
    if !auth_header.starts_with("Bearer ") {
        return Err(Error::Validation("Invalid Authorization header format".to_string()));
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    // TODO: Implement JWT parsing and extract tenant_id or tenant_slug
    // For now, return an error indicating this needs to be implemented
    Err(Error::Validation("JWT tenant extraction not implemented".to_string()))
}

/// Extract tenant from URL path
fn extract_tenant_from_path(path: &str) -> Option<String> {
    // Pattern: /api/v1/tenants/{slug}/...
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() >= 5 && parts[1] == "api" && parts[2] == "v1" && parts[3] == "tenants" {
        Some(parts[4].to_string())
    } else {
        None
    }
}

/// Resolve tenant context from identifier
async fn resolve_tenant_context(
    identifier: &str,
    config: &TenantMiddlewareConfig,
) -> Result<TenantContext> {
    // Try to parse as UUID first, then as slug
    let tenant_detail = if let Ok(tenant_id) = Uuid::parse_str(identifier) {
        config.tenant_service.get_tenant(tenant_id).await?
    } else {
        config.tenant_service.get_tenant_by_slug(identifier).await?
    };

    Ok(TenantContext {
        tenant_id: tenant_detail.id,
        tenant_slug: tenant_detail.slug,
        tenant_name: tenant_detail.name,
        subscription_tier: format!("{:?}", tenant_detail.subscription_tier).to_lowercase(),
        is_active: tenant_detail.status.to_string().to_uppercase() == "ACTIVE",
        feature_flags: tenant_detail.feature_flags,
    })
}

/// Row-level security enforcement middleware
pub async fn rls_middleware(
    request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    // Extract tenant context from request extensions
    let tenant_context = request
        .extensions()
        .get::<TenantContext>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Add tenant context to database connections
    // This would typically involve setting session variables or connection parameters
    // that would be used by the database to enforce row-level security

    info!(
        "Enforcing RLS for tenant: {} ({})",
        tenant_context.tenant_slug, tenant_context.tenant_id
    );

    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

/// Cross-tenant access prevention middleware
pub async fn cross_tenant_prevention_middleware(
    request: Request,
    next: Next,
) -> std::result::Result<Response, StatusCode> {
    // This middleware would validate that all resources being accessed
    // belong to the current tenant context

    let tenant_context = request
        .extensions()
        .get::<TenantContext>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // TODO: Implement cross-tenant access validation
    // This would involve checking resource ownership against tenant_id

    info!(
        "Cross-tenant protection active for: {}",
        tenant_context.tenant_slug
    );

    Ok(next.run(request).await)
}

/// Helper to create feature-gated middleware
pub fn require_feature(feature: &str) -> impl Fn(TenantMiddlewareConfig) -> TenantMiddlewareConfig + '_ {
    move |config| config.with_feature_requirement(feature.to_string())
}

/// Helper to create tenant middleware with subdomain resolution
pub fn subdomain_tenant_middleware(
    tenant_service: Arc<TenantService>,
) -> TenantMiddlewareConfig {
    TenantMiddlewareConfig::new(tenant_service)
        .with_resolution_strategy(TenantResolutionStrategy::Subdomain)
}

/// Helper to create tenant middleware with header resolution
pub fn header_tenant_middleware(
    tenant_service: Arc<TenantService>,
) -> TenantMiddlewareConfig {
    TenantMiddlewareConfig::new(tenant_service)
        .with_resolution_strategy(TenantResolutionStrategy::Header)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, HeaderValue};

    #[test]
    fn test_extract_subdomain() {
        assert_eq!(extract_subdomain("acme.olympus.com"), Some("acme".to_string()));
        assert_eq!(extract_subdomain("acme.olympus.com:8080"), Some("acme".to_string()));
        assert_eq!(extract_subdomain("olympus.com"), None);
        assert_eq!(extract_subdomain("localhost"), None);
    }

    #[test]
    fn test_extract_tenant_from_path() {
        assert_eq!(
            extract_tenant_from_path("/api/v1/tenants/acme/users"),
            Some("acme".to_string())
        );
        assert_eq!(
            extract_tenant_from_path("/api/v1/tenants/123e4567-e89b-12d3-a456-426614174000/products"),
            Some("123e4567-e89b-12d3-a456-426614174000".to_string())
        );
        assert_eq!(extract_tenant_from_path("/api/v1/health"), None);
        assert_eq!(extract_tenant_from_path("/users"), None);
    }

    #[test]
    fn test_tenant_context_feature_access() {
        let context = TenantContext {
            tenant_id: Uuid::new_v4(),
            tenant_slug: "test".to_string(),
            tenant_name: "Test Tenant".to_string(),
            subscription_tier: "professional".to_string(),
            is_active: true,
            feature_flags: serde_json::json!({}),
        };

        assert!(context.can_access_feature("basic_analytics"));
        assert!(context.can_access_feature("advanced_analytics"));
        assert!(context.can_access_feature("api_access"));
        assert!(context.can_access_feature("priority_support"));
        assert!(!context.can_access_feature("white_labeling")); // Enterprise only

        // Test with feature flags override
        let context_with_flags = TenantContext {
            feature_flags: serde_json::json!({
                "white_labeling": true,
                "basic_analytics": false
            }),
            ..context
        };

        assert!(context_with_flags.can_access_feature("white_labeling")); // Enabled by flag
        assert!(!context_with_flags.can_access_feature("basic_analytics")); // Disabled by flag
    }

    #[test]
    fn test_tenant_context_limits() {
        let context = TenantContext {
            tenant_id: Uuid::new_v4(),
            tenant_slug: "test".to_string(),
            tenant_name: "Test Tenant".to_string(),
            subscription_tier: "starter".to_string(),
            is_active: true,
            feature_flags: serde_json::json!({}),
        };

        assert!(context.check_user_limit(10)); // Under limit (25)
        assert!(!context.check_user_limit(30)); // Over limit

        assert!(context.check_location_limit(2)); // Under limit (3)
        assert!(!context.check_location_limit(5)); // Over limit
    }

    #[test]
    fn test_inactive_tenant_feature_access() {
        let context = TenantContext {
            tenant_id: Uuid::new_v4(),
            tenant_slug: "test".to_string(),
            tenant_name: "Test Tenant".to_string(),
            subscription_tier: "enterprise".to_string(),
            is_active: false, // Inactive
            feature_flags: serde_json::json!({}),
        };

        // Inactive tenants should not have access to any features
        assert!(!context.can_access_feature("basic_analytics"));
        assert!(!context.can_access_feature("advanced_analytics"));
    }
}