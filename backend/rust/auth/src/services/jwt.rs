use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use uuid::Uuid;
use chrono::{Duration, Utc};
use rand::Rng;
use crate::error::{AuthError, Result};
use crate::models::{User, Claims, RefreshClaims};

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub fn generate_access_token(&self, user: &User, tenant_id: Uuid, session_id: Uuid) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(1);

        let claims = Claims {
            sub: user.id,
            tenant_id,
            email: user.email.clone(),
            roles: user.roles.clone(),
            permissions: user.permissions.clone(),
            session_id,
            iat: now.timestamp(),
            exp: expiration.timestamp(),
        };

        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AuthError::JwtError(e.to_string()))
    }

    pub fn generate_refresh_token(&self) -> Result<String> {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
        Ok(base64_utils::encode(random_bytes))
    }

    pub fn verify_access_token(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(Algorithm::HS256);

        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            })
    }

    pub fn generate_email_verification_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(24);

        let claims = serde_json::json!({
            "sub": user_id,
            "email": email,
            "type": "email_verification",
            "iat": now.timestamp(),
            "exp": expiration.timestamp(),
        });

        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AuthError::JwtError(e.to_string()))
    }

    pub fn generate_password_reset_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        let now = Utc::now();
        let expiration = now + Duration::hours(1);

        let claims = serde_json::json!({
            "sub": user_id,
            "email": email,
            "type": "password_reset",
            "iat": now.timestamp(),
            "exp": expiration.timestamp(),
        });

        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &self.encoding_key)
            .map_err(|e| AuthError::JwtError(e.to_string()))
    }

    pub fn verify_email_token(&self, token: &str) -> Result<(Uuid, String)> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<serde_json::Value>(token, &self.decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            })?;

        let claims = token_data.claims;

        if claims.get("type").and_then(|v| v.as_str()) != Some("email_verification") {
            return Err(AuthError::InvalidToken);
        }

        let user_id = claims.get("sub")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or(AuthError::InvalidToken)?;

        let email = claims.get("email")
            .and_then(|v| v.as_str())
            .ok_or(AuthError::InvalidToken)?
            .to_string();

        Ok((user_id, email))
    }

    pub fn verify_password_reset_token(&self, token: &str) -> Result<(Uuid, String)> {
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<serde_json::Value>(token, &self.decoding_key, &validation)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::TokenExpired,
                _ => AuthError::InvalidToken,
            })?;

        let claims = token_data.claims;

        if claims.get("type").and_then(|v| v.as_str()) != Some("password_reset") {
            return Err(AuthError::InvalidToken);
        }

        let user_id = claims.get("sub")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or(AuthError::InvalidToken)?;

        let email = claims.get("email")
            .and_then(|v| v.as_str())
            .ok_or(AuthError::InvalidToken)?
            .to_string();

        Ok((user_id, email))
    }
}

// External base64 encoding/decoding
mod base64_utils {
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

    pub fn encode(input: Vec<u8>) -> String {
        URL_SAFE_NO_PAD.encode(input)
    }

    pub fn decode(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
        URL_SAFE_NO_PAD.decode(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_generation_and_verification() {
        let service = JwtService::new(b"test-secret-key-must-be-at-least-32-bytes-long!");

        let user = User {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: String::new(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            display_name: None,
            phone: None,
            avatar_url: None,
            roles: vec!["user".to_string()],
            permissions: vec![],
            is_active: true,
            email_verified: false,
            phone_verified: false,
            two_factor_enabled: false,
            last_login: None,
            failed_login_attempts: 0,
            locked_until: None,
            password_changed_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let session_id = Uuid::new_v4();
        let token = service.generate_access_token(&user, user.tenant_id, session_id).unwrap();
        assert!(!token.is_empty());

        let claims = service.verify_access_token(&token).unwrap();
        assert_eq!(claims.sub, user.id);
        assert_eq!(claims.email, user.email);
    }

    #[test]
    fn test_refresh_token_generation() {
        let service = JwtService::new(b"test-secret");
        let token = service.generate_refresh_token().unwrap();
        assert!(!token.is_empty());
    }
}