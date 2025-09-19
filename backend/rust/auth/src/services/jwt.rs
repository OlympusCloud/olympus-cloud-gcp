// ============================================================================
// OLYMPUS CLOUD - JWT TOKEN SERVICE
// ============================================================================
// Module: auth/src/services/jwt.rs
// Description: JWT token generation, validation, and management
// Author: Claude Code Agent
// Date: 2025-01-18
// ============================================================================

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use olympus_shared::{Error, Result};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// JWT token service for handling authentication tokens
#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    algorithm: Algorithm,
    issuer: String,
    audience: String,
    access_token_duration: i64,  // seconds
    refresh_token_duration: i64, // seconds
    leeway: i64,                 // seconds for clock skew
}

/// JWT claims for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,        // Subject (user ID)
    pub tenant_id: String,  // Tenant ID
    pub email: String,      // User email
    pub roles: Vec<String>, // User roles
    pub permissions: Vec<String>, // User permissions
    pub session_id: String, // Session ID
    pub iat: i64,          // Issued at
    pub exp: i64,          // Expiration time
    pub nbf: i64,          // Not before
    pub iss: String,       // Issuer
    pub aud: String,       // Audience
    pub jti: String,       // JWT ID
    pub token_type: String, // "access"
}

/// JWT claims for refresh tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,        // Subject (user ID)
    pub tenant_id: String,  // Tenant ID
    pub session_id: String, // Session ID
    pub device_id: Option<String>, // Device ID
    pub iat: i64,          // Issued at
    pub exp: i64,          // Expiration time
    pub nbf: i64,          // Not before
    pub iss: String,       // Issuer
    pub aud: String,       // Audience
    pub jti: String,       // JWT ID
    pub token_type: String, // "refresh"
}

/// Token pair containing access and refresh tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub scope: Option<String>,
}

/// Token validation result
#[derive(Debug)]
pub struct TokenValidation {
    pub claims: AccessTokenClaims,
    pub is_valid: bool,
    pub is_expired: bool,
    pub remaining_seconds: Option<i64>,
}

/// Device information for token generation
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub device_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl JwtService {
    /// Create a new JWT service
    pub fn new(
        secret: &str,
        issuer: String,
        audience: String,
        access_token_duration: i64,
        refresh_token_duration: i64,
    ) -> Result<Self> {
        if secret.len() < 32 {
            return Err(Error::Configuration(
                "JWT secret must be at least 32 characters".to_string(),
            ));
        }

        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        Ok(Self {
            encoding_key,
            decoding_key,
            algorithm: Algorithm::HS256,
            issuer,
            audience,
            access_token_duration,
            refresh_token_duration,
            leeway: 30, // 30 seconds clock skew tolerance
        })
    }

    /// Generate a token pair (access + refresh)
    pub fn generate_token_pair(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        email: String,
        roles: Vec<String>,
        permissions: Vec<String>,
        session_id: String,
        device_info: DeviceInfo,
    ) -> Result<TokenPair> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::Internal(format!("System time error: {}", e)))?
            .as_secs() as i64;

        // Generate access token
        let access_jti = Uuid::new_v4().to_string();
        let access_claims = AccessTokenClaims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            email,
            roles,
            permissions,
            session_id: session_id.clone(),
            iat: now,
            exp: now + self.access_token_duration,
            nbf: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti: access_jti,
            token_type: "access".to_string(),
        };

        let access_token = encode(&Header::default(), &access_claims, &self.encoding_key)
            .map_err(|e| Error::Jwt(e))?;

        // Generate refresh token
        let refresh_jti = Uuid::new_v4().to_string();
        let refresh_claims = RefreshTokenClaims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            session_id,
            device_id: device_info.device_id,
            iat: now,
            exp: now + self.refresh_token_duration,
            nbf: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti: refresh_jti,
            token_type: "refresh".to_string(),
        };

        let refresh_token = encode(&Header::default(), &refresh_claims, &self.encoding_key)
            .map_err(|e| Error::Jwt(e))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_duration,
            refresh_expires_in: self.refresh_token_duration,
            scope: Some("read write".to_string()),
        })
    }

    /// Validate an access token
    pub fn validate_access_token(&self, token: &str) -> Result<TokenValidation> {
        let mut validation = Validation::new(self.algorithm);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.leeway = self.leeway as u64;

        match decode::<AccessTokenClaims>(token, &self.decoding_key, &validation) {
            Ok(token_data) => {
                let claims = token_data.claims;

                // Check token type
                if claims.token_type != "access" {
                    return Err(Error::Jwt(jsonwebtoken::errors::Error::from(
                        jsonwebtoken::errors::ErrorKind::InvalidToken,
                    )));
                }

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|e| Error::Internal(format!("System time error: {}", e)))?
                    .as_secs() as i64;

                let is_expired = claims.exp <= now;
                let remaining_seconds = if is_expired {
                    None
                } else {
                    Some(claims.exp - now)
                };

                Ok(TokenValidation {
                    claims,
                    is_valid: !is_expired,
                    is_expired,
                    remaining_seconds,
                })
            }
            Err(e) => {
                let is_expired = matches!(e.kind(), jsonwebtoken::errors::ErrorKind::ExpiredSignature);

                if is_expired {
                    // Try to decode without validation to get claims for expired token
                    let mut no_validation = Validation::new(self.algorithm);
                    no_validation.validate_exp = false;
                    no_validation.validate_nbf = false;
                    no_validation.set_issuer(&[&self.issuer]);
                    no_validation.set_audience(&[&self.audience]);

                    if let Ok(token_data) = decode::<AccessTokenClaims>(token, &self.decoding_key, &no_validation) {
                        return Ok(TokenValidation {
                            claims: token_data.claims,
                            is_valid: false,
                            is_expired: true,
                            remaining_seconds: None,
                        });
                    }
                }

                Err(Error::Jwt(e))
            }
        }
    }

    /// Validate a refresh token
    pub fn validate_refresh_token(&self, token: &str) -> Result<RefreshTokenClaims> {
        let mut validation = Validation::new(self.algorithm);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);
        validation.leeway = self.leeway as u64;

        let token_data = decode::<RefreshTokenClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| Error::Jwt(e))?;

        let claims = token_data.claims;

        // Check token type
        if claims.token_type != "refresh" {
            return Err(Error::Jwt(jsonwebtoken::errors::Error::from(
                jsonwebtoken::errors::ErrorKind::InvalidToken,
            )));
        }

        Ok(claims)
    }

    /// Refresh an access token using a refresh token
    pub fn refresh_access_token(
        &self,
        refresh_token: &str,
        email: String,
        roles: Vec<String>,
        permissions: Vec<String>,
    ) -> Result<String> {
        let refresh_claims = self.validate_refresh_token(refresh_token)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::Internal(format!("System time error: {}", e)))?
            .as_secs() as i64;

        let access_jti = Uuid::new_v4().to_string();
        let access_claims = AccessTokenClaims {
            sub: refresh_claims.sub,
            tenant_id: refresh_claims.tenant_id,
            email,
            roles,
            permissions,
            session_id: refresh_claims.session_id,
            iat: now,
            exp: now + self.access_token_duration,
            nbf: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti: access_jti,
            token_type: "access".to_string(),
        };

        encode(&Header::default(), &access_claims, &self.encoding_key)
            .map_err(|e| Error::Jwt(e))
    }

    /// Extract user ID from token without validation (for logging/debugging)
    pub fn extract_user_id(&self, token: &str) -> Option<String> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.insecure_disable_signature_validation();

        decode::<AccessTokenClaims>(token, &self.decoding_key, &validation)
            .ok()
            .map(|token_data| token_data.claims.sub)
    }

    /// Get token expiration time
    pub fn get_token_expiration(&self, token: &str) -> Result<i64> {
        let mut validation = Validation::new(self.algorithm);
        validation.validate_exp = false;
        validation.validate_nbf = false;
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);

        let token_data = decode::<AccessTokenClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| Error::Jwt(e))?;

        Ok(token_data.claims.exp)
    }

    /// Check if token is close to expiration (within 5 minutes)
    pub fn is_token_expiring_soon(&self, token: &str) -> bool {
        if let Ok(exp) = self.get_token_expiration(token) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0);

            // Token expires within 5 minutes
            exp - now < 300
        } else {
            false
        }
    }

    /// Generate a single-use API token (short-lived)
    pub fn generate_api_token(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        purpose: String,
        duration_seconds: i64,
    ) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::Internal(format!("System time error: {}", e)))?
            .as_secs() as i64;

        let jti = Uuid::new_v4().to_string();
        let claims = AccessTokenClaims {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            email: format!("api-token-{}", purpose),
            roles: vec!["api".to_string()],
            permissions: vec![],
            session_id: jti.clone(),
            iat: now,
            exp: now + duration_seconds,
            nbf: now,
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
            jti,
            token_type: "api".to_string(),
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| Error::Jwt(e))
    }

    /// Generate email verification token
    pub fn generate_email_verification_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        self.generate_api_token(user_id, Uuid::new_v4(), "email_verification".to_string(), 86400) // 24 hours
    }

    /// Generate password reset token
    pub fn generate_password_reset_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        self.generate_api_token(user_id, Uuid::new_v4(), "password_reset".to_string(), 3600) // 1 hour
    }

    /// Verify special purpose token (email verification, password reset)
    pub fn verify_special_token(&self, token: &str, expected_purpose: &str) -> Result<Uuid> {
        let validation = self.validate_access_token(token)?;

        if !validation.is_valid {
            return Err(Error::SessionExpired);
        }

        if !validation.claims.email.starts_with(&format!("api-token-{}", expected_purpose)) {
            return Err(Error::InvalidInput("Invalid token purpose".to_string()));
        }

        Uuid::parse_str(&validation.claims.sub)
            .map_err(|_| Error::InvalidInput("Invalid user ID in token".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_jwt_service() -> JwtService {
        JwtService::new(
            "test-secret-key-that-is-at-least-32-characters-long",
            "olympus-test".to_string(),
            "olympus-api".to_string(),
            3600, // 1 hour
            86400, // 24 hours
        ).unwrap()
    }

    #[test]
    fn test_jwt_service_creation() {
        let service = create_test_jwt_service();
        assert_eq!(service.issuer, "olympus-test");
        assert_eq!(service.audience, "olympus-api");
        assert_eq!(service.access_token_duration, 3600);
        assert_eq!(service.refresh_token_duration, 86400);
    }

    #[test]
    fn test_jwt_service_short_secret() {
        let result = JwtService::new(
            "short",
            "test".to_string(),
            "test".to_string(),
            3600,
            86400,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_token_generation() {
        let service = create_test_jwt_service();
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let session_id = Uuid::new_v4().to_string();

        let token_pair = service.generate_token_pair(
            user_id,
            tenant_id,
            "test@example.com".to_string(),
            vec!["user".to_string()],
            vec!["read".to_string()],
            session_id,
            DeviceInfo {
                device_id: Some("test-device".to_string()),
                ip_address: Some("127.0.0.1".to_string()),
                user_agent: Some("test-agent".to_string()),
            },
        ).unwrap();

        assert!(!token_pair.access_token.is_empty());
        assert!(!token_pair.refresh_token.is_empty());
        assert_eq!(token_pair.token_type, "Bearer");
        assert_eq!(token_pair.expires_in, 3600);
    }

    #[test]
    fn test_token_validation() {
        let service = create_test_jwt_service();
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let session_id = Uuid::new_v4().to_string();

        let token_pair = service.generate_token_pair(
            user_id,
            tenant_id,
            "test@example.com".to_string(),
            vec!["user".to_string()],
            vec!["read".to_string()],
            session_id.clone(),
            DeviceInfo {
                device_id: Some("test-device".to_string()),
                ip_address: None,
                user_agent: None,
            },
        ).unwrap();

        let validation = service.validate_access_token(&token_pair.access_token).unwrap();
        assert!(validation.is_valid);
        assert!(!validation.is_expired);
        assert!(validation.remaining_seconds.is_some());
        assert_eq!(validation.claims.sub, user_id.to_string());
        assert_eq!(validation.claims.tenant_id, tenant_id.to_string());
        assert_eq!(validation.claims.session_id, session_id);
    }

    #[test]
    fn test_refresh_token_validation() {
        let service = create_test_jwt_service();
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let session_id = Uuid::new_v4().to_string();

        let token_pair = service.generate_token_pair(
            user_id,
            tenant_id,
            "test@example.com".to_string(),
            vec!["user".to_string()],
            vec!["read".to_string()],
            session_id.clone(),
            DeviceInfo {
                device_id: Some("test-device".to_string()),
                ip_address: None,
                user_agent: None,
            },
        ).unwrap();

        let refresh_claims = service.validate_refresh_token(&token_pair.refresh_token).unwrap();
        assert_eq!(refresh_claims.sub, user_id.to_string());
        assert_eq!(refresh_claims.tenant_id, tenant_id.to_string());
        assert_eq!(refresh_claims.session_id, session_id);
        assert_eq!(refresh_claims.device_id, Some("test-device".to_string()));
    }

    #[test]
    fn test_token_refresh() {
        let service = create_test_jwt_service();
        let user_id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let session_id = Uuid::new_v4().to_string();

        let token_pair = service.generate_token_pair(
            user_id,
            tenant_id,
            "test@example.com".to_string(),
            vec!["user".to_string()],
            vec!["read".to_string()],
            session_id,
            DeviceInfo {
                device_id: Some("test-device".to_string()),
                ip_address: None,
                user_agent: None,
            },
        ).unwrap();

        let new_access_token = service.refresh_access_token(
            &token_pair.refresh_token,
            "test@example.com".to_string(),
            vec!["user".to_string()],
            vec!["read".to_string()],
        ).unwrap();

        assert!(!new_access_token.is_empty());
        assert_ne!(new_access_token, token_pair.access_token);

        let validation = service.validate_access_token(&new_access_token).unwrap();
        assert!(validation.is_valid);
    }

    #[test]
    fn test_special_tokens() {
        let service = create_test_jwt_service();
        let user_id = Uuid::new_v4();

        let email_token = service.generate_email_verification_token(user_id, "test@example.com").unwrap();
        let password_token = service.generate_password_reset_token(user_id, "test@example.com").unwrap();

        assert!(!email_token.is_empty());
        assert!(!password_token.is_empty());

        let verified_user_id = service.verify_special_token(&email_token, "email_verification").unwrap();
        assert_eq!(verified_user_id, user_id);

        let verified_user_id = service.verify_special_token(&password_token, "password_reset").unwrap();
        assert_eq!(verified_user_id, user_id);
    }
}