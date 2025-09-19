//! Security middleware and hardening for production

use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;

/// Security headers middleware
pub async fn security_headers(
    request: Request,
    next: Next,
) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();

    // Prevent clickjacking
    headers.insert(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    );

    // Prevent MIME type sniffing
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );

    // Enable XSS protection
    headers.insert(
        "X-XSS-Protection",
        HeaderValue::from_static("1; mode=block"),
    );

    // Strict Transport Security
    headers.insert(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    // Content Security Policy
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self'; \
            script-src 'self' 'unsafe-inline'; \
            style-src 'self' 'unsafe-inline'; \
            img-src 'self' data: https:; \
            font-src 'self' data:; \
            connect-src 'self'; \
            frame-ancestors 'none';"
        ),
    );

    // Referrer Policy
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );

    // Permissions Policy
    headers.insert(
        "Permissions-Policy",
        HeaderValue::from_static(
            "accelerometer=(), camera=(), geolocation=(), \
            gyroscope=(), magnetometer=(), microphone=(), \
            payment=(), usb=()"
        ),
    );

    response
}

/// Configure CORS for production
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "https://app.olympuscloud.io".parse::<HeaderValue>().unwrap(),
            "https://www.olympuscloud.io".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(86400))
}

/// Rate limiting configuration
pub mod rate_limiting {
    use std::collections::HashMap;
    use std::net::IpAddr;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use tokio::time::{Duration, Instant};

    #[derive(Clone)]
    pub struct RateLimiter {
        requests: Arc<RwLock<HashMap<IpAddr, Vec<Instant>>>>,
        max_requests: usize,
        window: Duration,
    }

    impl RateLimiter {
        pub fn new(max_requests: usize, window_seconds: u64) -> Self {
            Self {
                requests: Arc::new(RwLock::new(HashMap::new())),
                max_requests,
                window: Duration::from_secs(window_seconds),
            }
        }

        pub async fn check_rate_limit(&self, ip: IpAddr) -> bool {
            let now = Instant::now();
            let mut requests = self.requests.write().await;

            let timestamps = requests.entry(ip).or_insert_with(Vec::new);

            // Remove old timestamps outside the window
            timestamps.retain(|&t| now.duration_since(t) < self.window);

            if timestamps.len() < self.max_requests {
                timestamps.push(now);
                true
            } else {
                false
            }
        }

        pub async fn cleanup(&self) {
            let now = Instant::now();
            let mut requests = self.requests.write().await;

            requests.retain(|_, timestamps| {
                timestamps.retain(|&t| now.duration_since(t) < self.window);
                !timestamps.is_empty()
            });
        }
    }
}

/// Input validation and sanitization
pub mod validation {
    use regex::Regex;
    use lazy_static::lazy_static;

    lazy_static! {
        // SQL injection patterns
        static ref SQL_INJECTION_PATTERN: Regex = Regex::new(
            r"(?i)(union|select|insert|update|delete|drop|create|alter|exec|execute|script|javascript|<script|</script)"
        ).unwrap();

        // XSS patterns
        static ref XSS_PATTERN: Regex = Regex::new(
            r"(?i)(<script|</script|javascript:|on\w+\s*=|<iframe|</iframe)"
        ).unwrap();

        // Path traversal patterns
        static ref PATH_TRAVERSAL_PATTERN: Regex = Regex::new(
            r"(\.\./|\.\.\\|%2e%2e%2f|%2e%2e/|\.%2e/|%2e\./)"
        ).unwrap();
    }

    pub fn validate_input(input: &str) -> Result<(), String> {
        if SQL_INJECTION_PATTERN.is_match(input) {
            return Err("Potential SQL injection detected".to_string());
        }

        if XSS_PATTERN.is_match(input) {
            return Err("Potential XSS attack detected".to_string());
        }

        if PATH_TRAVERSAL_PATTERN.is_match(input) {
            return Err("Potential path traversal detected".to_string());
        }

        Ok(())
    }

    pub fn sanitize_html(input: &str) -> String {
        ammonia::clean(input)
    }

    pub fn validate_email(email: &str) -> Result<(), String> {
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        ).unwrap();

        if email_regex.is_match(email) {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
}

/// Encryption utilities
pub mod encryption {
    use aes_gcm::{
        aead::{Aead, KeyInit, OsRng},
        Aes256Gcm, Key, Nonce,
    };
    use base64::{engine::general_purpose::STANDARD, Engine};
    use rand::RngCore;

    pub struct Encryptor {
        cipher: Aes256Gcm,
    }

    impl Encryptor {
        pub fn new(key: &[u8; 32]) -> Self {
            let key = Key::<Aes256Gcm>::from_slice(key);
            let cipher = Aes256Gcm::new(key);
            Self { cipher }
        }

        pub fn encrypt(&self, plaintext: &[u8]) -> Result<String, String> {
            let mut nonce_bytes = [0u8; 12];
            OsRng.fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);

            let ciphertext = self
                .cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| format!("Encryption failed: {}", e))?;

            let mut combined = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
            combined.extend_from_slice(&nonce_bytes);
            combined.extend_from_slice(&ciphertext);

            Ok(STANDARD.encode(combined))
        }

        pub fn decrypt(&self, ciphertext: &str) -> Result<Vec<u8>, String> {
            let combined = STANDARD
                .decode(ciphertext)
                .map_err(|e| format!("Base64 decode failed: {}", e))?;

            if combined.len() < 12 {
                return Err("Invalid ciphertext".to_string());
            }

            let (nonce_bytes, ciphertext) = combined.split_at(12);
            let nonce = Nonce::from_slice(nonce_bytes);

            self.cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| format!("Decryption failed: {}", e))
        }
    }
}

/// Security audit logging
pub mod audit {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use std::net::IpAddr;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AuditLog {
        pub id: Uuid,
        pub timestamp: DateTime<Utc>,
        pub user_id: Option<Uuid>,
        pub tenant_id: Option<Uuid>,
        pub action: String,
        pub resource: String,
        pub ip_address: IpAddr,
        pub user_agent: String,
        pub success: bool,
        pub details: Option<serde_json::Value>,
    }

    impl AuditLog {
        pub fn new(
            action: String,
            resource: String,
            ip_address: IpAddr,
            user_agent: String,
            success: bool,
        ) -> Self {
            Self {
                id: Uuid::new_v4(),
                timestamp: Utc::now(),
                user_id: None,
                tenant_id: None,
                action,
                resource,
                ip_address,
                user_agent,
                success,
                details: None,
            }
        }

        pub fn with_user(mut self, user_id: Uuid, tenant_id: Uuid) -> Self {
            self.user_id = Some(user_id);
            self.tenant_id = Some(tenant_id);
            self
        }

        pub fn with_details(mut self, details: serde_json::Value) -> Self {
            self.details = Some(details);
            self
        }
    }
}