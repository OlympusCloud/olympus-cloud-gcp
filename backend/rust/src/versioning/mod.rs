//! API versioning strategy and middleware

use axum::{
    extract::{Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use tracing::{debug, warn};

/// API version definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiVersion {
    V1,
    V2,
    V3,
}

impl ApiVersion {
    /// Get the current stable version
    pub fn current() -> Self {
        ApiVersion::V1
    }

    /// Get the latest available version
    pub fn latest() -> Self {
        ApiVersion::V1
    }

    /// Check if this version is deprecated
    pub fn is_deprecated(&self) -> bool {
        match self {
            ApiVersion::V1 => false,
            ApiVersion::V2 => false,
            ApiVersion::V3 => false,
        }
    }

    /// Get deprecation info if applicable
    pub fn deprecation_info(&self) -> Option<DeprecationInfo> {
        match self {
            ApiVersion::V1 => None,
            ApiVersion::V2 => None,
            ApiVersion::V3 => None,
        }
    }

    /// Get version string for headers
    pub fn as_header_value(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "application/vnd.olympus.v1+json",
            ApiVersion::V2 => "application/vnd.olympus.v2+json",
            ApiVersion::V3 => "application/vnd.olympus.v3+json",
        }
    }

    /// Get version prefix for URLs
    pub fn as_url_prefix(&self) -> &'static str {
        match self {
            ApiVersion::V1 => "/api/v1",
            ApiVersion::V2 => "/api/v2",
            ApiVersion::V3 => "/api/v3",
        }
    }
}

impl FromStr for ApiVersion {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "v1" | "1" => Ok(ApiVersion::V1),
            "v2" | "2" => Ok(ApiVersion::V2),
            "v3" | "3" => Ok(ApiVersion::V3),
            _ => Err(VersionError::UnsupportedVersion(s.to_string())),
        }
    }
}

impl std::fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiVersion::V1 => write!(f, "v1"),
            ApiVersion::V2 => write!(f, "v2"),
            ApiVersion::V3 => write!(f, "v3"),
        }
    }
}

/// Version deprecation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeprecationInfo {
    pub deprecated_on: chrono::DateTime<chrono::Utc>,
    pub sunset_date: Option<chrono::DateTime<chrono::Utc>>,
    pub replacement_version: ApiVersion,
    pub migration_guide_url: Option<String>,
    pub breaking_changes: Vec<String>,
}

/// Version negotiation strategy
#[derive(Debug, Clone)]
pub enum VersionStrategy {
    /// Use URL path versioning (/api/v1/...)
    UrlPath,
    /// Use Accept header versioning (Accept: application/vnd.olympus.v1+json)
    AcceptHeader,
    /// Use custom header versioning (X-API-Version: v1)
    CustomHeader(String),
    /// Use query parameter versioning (?version=v1)
    QueryParameter(String),
}

impl Default for VersionStrategy {
    fn default() -> Self {
        VersionStrategy::UrlPath
    }
}

/// Version resolution result
#[derive(Debug, Clone)]
pub struct VersionResolution {
    pub version: ApiVersion,
    pub strategy_used: VersionStrategy,
    pub deprecation_warning: Option<DeprecationInfo>,
}

/// Version resolver for extracting API version from requests
#[derive(Debug, Clone)]
pub struct VersionResolver {
    pub strategies: Vec<VersionStrategy>,
    pub default_version: ApiVersion,
    pub strict_mode: bool,
}

impl Default for VersionResolver {
    fn default() -> Self {
        Self {
            strategies: vec![
                VersionStrategy::UrlPath,
                VersionStrategy::AcceptHeader,
                VersionStrategy::CustomHeader("X-API-Version".to_string()),
            ],
            default_version: ApiVersion::current(),
            strict_mode: false,
        }
    }
}

impl VersionResolver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure strategies in order of preference
    pub fn with_strategies(mut self, strategies: Vec<VersionStrategy>) -> Self {
        self.strategies = strategies;
        self
    }

    /// Set default version when none specified
    pub fn with_default_version(mut self, version: ApiVersion) -> Self {
        self.default_version = version;
        self
    }

    /// Enable strict mode (reject requests without version)
    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    /// Resolve API version from request
    pub fn resolve(&self, request: &Request) -> Result<VersionResolution, VersionError> {
        for strategy in &self.strategies {
            if let Some(version) = self.try_strategy(request, strategy)? {
                let deprecation_warning = version.deprecation_info();
                return Ok(VersionResolution {
                    version,
                    strategy_used: strategy.clone(),
                    deprecation_warning,
                });
            }
        }

        if self.strict_mode {
            Err(VersionError::NoVersionSpecified)
        } else {
            Ok(VersionResolution {
                version: self.default_version,
                strategy_used: VersionStrategy::UrlPath,
                deprecation_warning: None,
            })
        }
    }

    fn try_strategy(
        &self,
        request: &Request,
        strategy: &VersionStrategy,
    ) -> Result<Option<ApiVersion>, VersionError> {
        match strategy {
            VersionStrategy::UrlPath => {
                let path = request.uri().path();
                if let Some(captures) = regex::Regex::new(r"/api/v(\d+)")
                    .unwrap()
                    .captures(path)
                {
                    let version_str = &captures[1];
                    return Ok(Some(ApiVersion::from_str(&format!("v{}", version_str))?));
                }
            }
            VersionStrategy::AcceptHeader => {
                if let Some(accept) = request.headers().get(header::ACCEPT) {
                    if let Ok(accept_str) = accept.to_str() {
                        if let Some(captures) = regex::Regex::new(r"application/vnd\.olympus\.v(\d+)\+json")
                            .unwrap()
                            .captures(accept_str)
                        {
                            let version_str = &captures[1];
                            return Ok(Some(ApiVersion::from_str(&format!("v{}", version_str))?));
                        }
                    }
                }
            }
            VersionStrategy::CustomHeader(header_name) => {
                if let Some(version_header) = request.headers().get(header_name) {
                    if let Ok(version_str) = version_header.to_str() {
                        return Ok(Some(ApiVersion::from_str(version_str)?));
                    }
                }
            }
            VersionStrategy::QueryParameter(param_name) => {
                if let Some(query) = request.uri().query() {
                    let params: HashMap<&str, &str> = form_urlencoded::parse(query.as_bytes())
                        .into_owned()
                        .collect();
                    if let Some(version_str) = params.get(param_name.as_str()) {
                        return Ok(Some(ApiVersion::from_str(version_str)?));
                    }
                }
            }
        }
        Ok(None)
    }
}

/// Version validation middleware
pub async fn version_middleware(
    State(resolver): State<VersionResolver>,
    mut request: Request,
    next: Next,
) -> Result<Response, VersionError> {
    let resolution = resolver.resolve(&request)?;

    debug!(
        "API version resolved: {} using {:?}",
        resolution.version, resolution.strategy_used
    );

    // Add version info to request extensions
    request.extensions_mut().insert(resolution.version);

    // Call next middleware/handler
    let mut response = next.run(request).await;

    // Add version headers to response
    let headers = response.headers_mut();
    headers.insert(
        "X-API-Version",
        HeaderValue::from_static(match resolution.version {
            ApiVersion::V1 => "v1",
            ApiVersion::V2 => "v2",
            ApiVersion::V3 => "v3",
        }),
    );

    // Add deprecation warning if applicable
    if let Some(deprecation) = resolution.deprecation_warning {
        headers.insert(
            "X-API-Deprecated",
            HeaderValue::from_str("true").unwrap(),
        );

        if let Some(sunset) = deprecation.sunset_date {
            headers.insert(
                "Sunset",
                HeaderValue::from_str(&sunset.to_rfc3339()).unwrap(),
            );
        }

        headers.insert(
            "X-API-Replacement-Version",
            HeaderValue::from_static(match deprecation.replacement_version {
                ApiVersion::V1 => "v1",
                ApiVersion::V2 => "v2",
                ApiVersion::V3 => "v3",
            }),
        );

        if let Some(guide_url) = deprecation.migration_guide_url {
            headers.insert(
                "X-API-Migration-Guide",
                HeaderValue::from_str(&guide_url).unwrap(),
            );
        }

        warn!(
            "Deprecated API version {} used, replacement: {}",
            resolution.version, deprecation.replacement_version
        );
    }

    Ok(response)
}

/// Version-aware route handler trait
pub trait VersionedHandler {
    async fn handle_v1(&self) -> Result<Response, VersionError>;
    async fn handle_v2(&self) -> Result<Response, VersionError> {
        // Default to v1 implementation
        self.handle_v1().await
    }
    async fn handle_v3(&self) -> Result<Response, VersionError> {
        // Default to v2 implementation
        self.handle_v2().await
    }

    async fn handle(&self, version: ApiVersion) -> Result<Response, VersionError> {
        match version {
            ApiVersion::V1 => self.handle_v1().await,
            ApiVersion::V2 => self.handle_v2().await,
            ApiVersion::V3 => self.handle_v3().await,
        }
    }
}

/// Version transformation utilities
pub mod transforms {
    use super::*;
    use serde_json::Value;

    /// Transform response data between API versions
    pub trait VersionTransform {
        fn transform_v1_to_v2(&self, data: Value) -> Result<Value, VersionError>;
        fn transform_v2_to_v3(&self, data: Value) -> Result<Value, VersionError>;
        fn transform_v2_to_v1(&self, data: Value) -> Result<Value, VersionError>;
        fn transform_v3_to_v2(&self, data: Value) -> Result<Value, VersionError>;
    }

    /// Standard transformations for common data structures
    pub struct StandardTransforms;

    impl VersionTransform for StandardTransforms {
        fn transform_v1_to_v2(&self, mut data: Value) -> Result<Value, VersionError> {
            // Example: Add new fields, rename fields, etc.
            if let Value::Object(ref mut obj) = data {
                // Add API version metadata
                obj.insert("_version".to_string(), Value::String("v2".to_string()));

                // Transform date formats if needed
                if let Some(created_at) = obj.get("created_at") {
                    if let Value::String(date_str) = created_at {
                        // Convert from ISO 8601 to RFC 3339 format
                        obj.insert("created_at".to_string(), Value::String(date_str.clone()));
                    }
                }
            }
            Ok(data)
        }

        fn transform_v2_to_v3(&self, mut data: Value) -> Result<Value, VersionError> {
            if let Value::Object(ref mut obj) = data {
                obj.insert("_version".to_string(), Value::String("v3".to_string()));

                // Add new fields for v3
                obj.insert("_links".to_string(), Value::Object(serde_json::Map::new()));
            }
            Ok(data)
        }

        fn transform_v2_to_v1(&self, mut data: Value) -> Result<Value, VersionError> {
            if let Value::Object(ref mut obj) = data {
                // Remove v2-specific fields
                obj.remove("_version");
                obj.remove("_metadata");
            }
            Ok(data)
        }

        fn transform_v3_to_v2(&self, mut data: Value) -> Result<Value, VersionError> {
            if let Value::Object(ref mut obj) = data {
                // Remove v3-specific fields
                obj.remove("_links");
                obj.insert("_version".to_string(), Value::String("v2".to_string()));
            }
            Ok(data)
        }
    }
}

/// Version compatibility matrix
pub struct CompatibilityMatrix {
    compatibility: HashMap<(ApiVersion, ApiVersion), bool>,
}

impl CompatibilityMatrix {
    pub fn new() -> Self {
        let mut compatibility = HashMap::new();

        // Define version compatibility
        compatibility.insert((ApiVersion::V1, ApiVersion::V1), true);
        compatibility.insert((ApiVersion::V1, ApiVersion::V2), true); // v1 can serve v2 requests
        compatibility.insert((ApiVersion::V2, ApiVersion::V1), true); // v2 can serve v1 requests
        compatibility.insert((ApiVersion::V2, ApiVersion::V2), true);
        compatibility.insert((ApiVersion::V3, ApiVersion::V3), true);
        compatibility.insert((ApiVersion::V3, ApiVersion::V2), true);
        compatibility.insert((ApiVersion::V2, ApiVersion::V3), false); // v2 cannot serve v3 requests

        Self { compatibility }
    }

    pub fn is_compatible(&self, handler_version: ApiVersion, request_version: ApiVersion) -> bool {
        self.compatibility
            .get(&(handler_version, request_version))
            .copied()
            .unwrap_or(false)
    }
}

/// Version error types
#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("Unsupported API version: {0}")]
    UnsupportedVersion(String),

    #[error("No API version specified and strict mode is enabled")]
    NoVersionSpecified,

    #[error("Version incompatibility: handler {handler} cannot serve request {request}")]
    IncompatibleVersion {
        handler: ApiVersion,
        request: ApiVersion,
    },

    #[error("Invalid version format: {0}")]
    InvalidFormat(String),

    #[error("Version transformation failed: {0}")]
    TransformationFailed(String),
}

impl IntoResponse for VersionError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            VersionError::UnsupportedVersion(_) | VersionError::InvalidFormat(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            VersionError::NoVersionSpecified => {
                (StatusCode::BAD_REQUEST, "API version must be specified".to_string())
            }
            VersionError::IncompatibleVersion { .. } => {
                (StatusCode::NOT_ACCEPTABLE, self.to_string())
            }
            VersionError::TransformationFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Version transformation failed".to_string())
            }
        };

        let body = serde_json::json!({
            "error": {
                "code": "VERSION_ERROR",
                "message": message,
                "supported_versions": ["v1", "v2", "v3"]
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

/// Macros for version-aware handlers
pub mod macros {
    /// Generate version-aware route handlers
    #[macro_export]
    macro_rules! versioned_handler {
        ($name:ident, $v1:expr, $v2:expr, $v3:expr) => {
            pub async fn $name(
                axum::extract::Extension(version): axum::extract::Extension<ApiVersion>,
            ) -> Result<axum::response::Response, VersionError> {
                match version {
                    ApiVersion::V1 => $v1().await,
                    ApiVersion::V2 => $v2().await,
                    ApiVersion::V3 => $v3().await,
                }
            }
        };
    }

    /// Generate version-specific response transformations
    #[macro_export]
    macro_rules! version_transform {
        ($data:expr, $from:expr, $to:expr) => {
            match ($from, $to) {
                (ApiVersion::V1, ApiVersion::V2) => transforms::StandardTransforms.transform_v1_to_v2($data),
                (ApiVersion::V2, ApiVersion::V3) => transforms::StandardTransforms.transform_v2_to_v3($data),
                (ApiVersion::V2, ApiVersion::V1) => transforms::StandardTransforms.transform_v2_to_v1($data),
                (ApiVersion::V3, ApiVersion::V2) => transforms::StandardTransforms.transform_v3_to_v2($data),
                _ => Ok($data),
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderMap, Method, Uri};

    #[test]
    fn test_version_parsing() {
        assert_eq!(ApiVersion::from_str("v1").unwrap(), ApiVersion::V1);
        assert_eq!(ApiVersion::from_str("2").unwrap(), ApiVersion::V2);
        assert!(ApiVersion::from_str("v99").is_err());
    }

    #[test]
    fn test_compatibility_matrix() {
        let matrix = CompatibilityMatrix::new();
        assert!(matrix.is_compatible(ApiVersion::V1, ApiVersion::V1));
        assert!(matrix.is_compatible(ApiVersion::V2, ApiVersion::V1));
        assert!(!matrix.is_compatible(ApiVersion::V2, ApiVersion::V3));
    }

    #[tokio::test]
    async fn test_version_resolution() {
        let resolver = VersionResolver::new();

        // Create a mock request with version in URL
        let mut request = Request::builder()
            .uri("/api/v1/products")
            .method(Method::GET)
            .body(axum::body::Body::empty())
            .unwrap();

        let resolution = resolver.resolve(&request).unwrap();
        assert_eq!(resolution.version, ApiVersion::V1);
    }
}