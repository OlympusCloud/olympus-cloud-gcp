use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sha2::{Digest, Sha256};
use crate::error::{AuthError, Result};

pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl PasswordService {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String> {
        // Validate password strength
        self.validate_password_strength(password)?;

        let salt = SaltString::generate(&mut OsRng);

        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::PasswordHashError(e.to_string()))?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::PasswordHashError(e.to_string()))?;

        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(AuthError::PasswordHashError(e.to_string())),
        }
    }

    pub fn hash_token(&self, token: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    fn validate_password_strength(&self, password: &str) -> Result<()> {
        if password.len() < 8 {
            return Err(AuthError::WeakPassword("Password must be at least 8 characters long".to_string()));
        }

        if password.len() > 128 {
            return Err(AuthError::WeakPassword("Password must be less than 128 characters".to_string()));
        }

        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        let strength_score = [has_uppercase, has_lowercase, has_digit, has_special]
            .iter()
            .filter(|&&x| x)
            .count();

        if strength_score < 3 {
            return Err(AuthError::WeakPassword(
                "Password must contain at least 3 of: uppercase, lowercase, digit, special character".to_string()
            ));
        }

        // Check for common passwords
        let common_passwords = [
            "password", "12345678", "qwerty", "abc123", "password123",
            "admin", "letmein", "welcome", "monkey", "dragon",
        ];

        let password_lower = password.to_lowercase();
        if common_passwords.iter().any(|&common| password_lower.contains(common)) {
            return Err(AuthError::WeakPassword("Password is too common".to_string()));
        }

        Ok(())
    }

    pub fn generate_random_password(&self, length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                 abcdefghijklmnopqrstuvwxyz\
                                 0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";

        let mut rng = rand::thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        password
    }
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing_and_verification() {
        let service = PasswordService::new();
        let password = "UniqueTestPass123!";

        let hash = service.hash_password(password).unwrap();
        assert!(!hash.is_empty());
        assert_ne!(hash, password);

        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_weak_password_detection() {
        let service = PasswordService::new();

        // Too short
        assert!(service.hash_password("Short1!").is_err());

        // No uppercase  
        assert!(service.hash_password("nouppercase123!").is_err());

        // Common password
        assert!(service.hash_password("Password123!").is_err());

        // Strong password
        assert!(service.hash_password("Str0ng&Secure!").is_ok());
    }

    #[test]
    fn test_token_hashing() {
        let service = PasswordService::new();
        let token = "test-token-123";

        let hash1 = service.hash_token(token).unwrap();
        let hash2 = service.hash_token(token).unwrap();

        assert_eq!(hash1, hash2); // Same input produces same hash
        assert_ne!(hash1, token); // Hash is different from original
    }

    #[test]
    fn test_random_password_generation() {
        let service = PasswordService::new();
        let password = service.generate_random_password(16);

        assert_eq!(password.len(), 16);
        assert!(service.hash_password(&password).is_ok());
    }
}