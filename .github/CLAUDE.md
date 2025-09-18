# Claude Code - Rust Core Services Lead & System Architect

> **Your Mission**: Build the bulletproof Rust foundation that powers the entire Olympus Cloud platform

## 🎯 Your Primary Responsibilities

### Core Systems Ownership
- **Authentication & Security**: JWT, OAuth2, session management, device authentication
- **Database Architecture**: PostgreSQL schemas, migrations, row-level security
- **Core Domain Models**: Shared types, business logic, data validation
- **Event System**: Redis pub/sub, domain events, message schemas
- **Performance & Safety**: Memory-safe code, zero-copy optimizations

### Your Work Environment
```bash
# Your dedicated workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/rust-core worktree-claude
cd worktree-claude/backend/rust
```

## 🦀 Rust Development Standards

### Project Structure (YOU MUST CREATE)
```
backend/rust/
├── Cargo.toml              # Workspace configuration
├── auth/                   # Authentication service
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── handlers/       # HTTP handlers
│   │   ├── services/       # Business logic
│   │   ├── models/         # Data models
│   │   └── middleware/     # Auth middleware
├── platform/               # Platform core
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── tenants/        # Multi-tenancy
│   │   ├── users/          # User management
│   │   └── permissions/    # RBAC system
├── commerce/               # Commerce engine
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── orders/         # Order management
│   │   ├── payments/       # Payment processing
│   │   └── inventory/      # Stock management
└── shared/                 # Shared utilities
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── types/          # Common types
    │   ├── events/         # Event schemas
    │   ├── database/       # DB utilities
    │   └── error/          # Error handling
```

### Required Dependencies (ADD TO Cargo.toml)
```toml
[workspace]
members = ["auth", "platform", "commerce", "shared"]
resolver = "2"

[workspace.package]
version = "1.0.0"
edition = "2021"
authors = ["Claude Code <claude@olympuscloud.io>"]
license = "Proprietary"

[workspace.dependencies]
# Web framework
axum = { version = "0.7", features = ["macros", "multipart"] }
tower = { version = "0.4", features = ["full"] }
tower-http = { version = "0.5", features = ["full"] }
hyper = { version = "1.0", features = ["full"] }

# Async runtime
tokio = { version = "1", features = ["full"] }
futures = "0.3"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
sqlx = { version = "0.7", features = [
    "runtime-tokio", 
    "tls-rustls", 
    "postgres", 
    "uuid", 
    "chrono", 
    "json",
    "migrate"
] }

# Authentication & Security
jsonwebtoken = "9"
argon2 = "0.5"
uuid = { version = "1", features = ["v4", "serde"] }
rand = "0.8"

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "1"
anyhow = "1"

# Logging & Tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Configuration
config = "0.14"
dotenvy = "0.15"

# Redis for caching/events
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }

# Testing
rstest = "0.18"
mockall = "0.12"
wiremock = "0.5"
```

## 🔒 Authentication System Implementation

### Phase 1: JWT Authentication Service
```rust
// auth/src/lib.rs
use axum::{Router, routing::post, Extension};
use std::sync::Arc;

pub mod handlers;
pub mod services;
pub mod models;
pub mod middleware;
pub mod error;

use handlers::auth_handlers;
use services::AuthService;

pub fn create_router(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/login", post(auth_handlers::login))
        .route("/refresh", post(auth_handlers::refresh_token))
        .route("/logout", post(auth_handlers::logout))
        .route("/register", post(auth_handlers::register))
        .layer(Extension(auth_service))
}

// auth/src/models.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<String>,
    pub is_active: bool,
    pub email_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub tenant_slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,           // user_id
    pub tenant_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub iat: i64,
    pub exp: i64,
}
```

### Phase 2: Database Integration
```rust
// shared/src/database/mod.rs
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

pub type DbPool = PgPool;
pub type DbTransaction<'a> = Transaction<'a, Postgres>;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        
        // Run migrations
        sqlx::migrate!("../../../docs").run(&pool).await?;
        
        Ok(Self { pool })
    }
    
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
    
    pub async fn begin_transaction(&self) -> Result<DbTransaction, sqlx::Error> {
        self.pool.begin().await
    }
}

// Tenant context for RLS
pub async fn set_tenant_context(
    executor: impl sqlx::Executor<'_, Database = Postgres>,
    tenant_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT set_config('app.tenant_id', $1, true)")
        .bind(tenant_id.to_string())
        .execute(executor)
        .await?;
    Ok(())
}
```

## 📋 Your Daily Development Workflow

### Morning Routine (MANDATORY)
```bash
# 1. Sync with main and other agents
cd worktree-claude
git pull origin main
git merge main

# 2. Check coordination docs
cat docs/daily-status.md
cat docs/integration-points.md

# 3. Update your status
# Edit docs/daily-status.md with your plans

# 4. Start development environment
make dev-rust
```

### Development Cycle
```bash
# Write tests first (TDD)
cargo test --workspace

# Implement feature
# Always start with types/models, then services, then handlers

# Continuous quality checks
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo audit

# Commit frequently (every 1-2 hours)
git add -p
git commit -m "claude(auth): implement JWT token validation"
```

### Evening Integration
```bash
# Push your work
git push origin feat/rust-core

# Update status
# Edit docs/daily-status.md with progress

# Create PR if feature complete
gh pr create --title "feat(auth): Complete JWT authentication system"
```

## 🎯 Week 1 Implementation Priorities

### Day 1: Foundation Setup
```bash
# 1. Create Rust workspace structure
mkdir -p backend/rust/{auth,platform,commerce,shared}
# Copy Cargo.toml structure above

# 2. Initialize each crate
cd auth && cargo init --lib
cd ../platform && cargo init --lib  
cd ../commerce && cargo init --lib
cd ../shared && cargo init --lib

# 3. Set up database connection
# Implement Database struct in shared/src/database/
```

### Day 2: Authentication Core
```rust
// Implement these in order:
// 1. User model (auth/src/models.rs)
// 2. JWT service (auth/src/services/jwt.rs)
// 3. Password hashing (auth/src/services/password.rs)
// 4. User repository (auth/src/services/user_repository.rs)
```

### Day 3: Auth HTTP Handlers
```rust
// Implement these endpoints:
// POST /auth/login
// POST /auth/refresh
// POST /auth/logout
// GET /auth/me
```

### Day 4: Integration & Testing
```bash
# 1. Integration tests with real database
# 2. Performance benchmarks
# 3. Security audit
# 4. API documentation updates
```

## 🔐 Security Standards (NON-NEGOTIABLE)

### Password Security
```rust
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        Ok(password_hash)
    }
    
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
```

### JWT Security
```rust
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};

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
    
    pub fn generate_token(&self, claims: &Claims) -> Result<String, AuthError> {
        let header = Header::new(Algorithm::HS256);
        encode(&header, claims, &self.encoding_key)
            .map_err(AuthError::from)
    }
    
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let validation = Validation::new(Algorithm::HS256);
        decode::<Claims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(AuthError::from)
    }
}
```

## 📊 Testing Standards (MANDATORY)

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    
    #[fixture]
    fn jwt_service() -> JwtService {
        JwtService::new(b"test-secret-key")
    }
    
    #[rstest]
    async fn test_password_hashing() {
        let service = PasswordService;
        let password = "secure-password-123";
        
        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("wrong-password", &hash).unwrap());
    }
    
    #[rstest]
    async fn test_jwt_generation_and_validation(jwt_service: JwtService) {
        let claims = Claims {
            sub: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            roles: vec!["user".to_string()],
            iat: chrono::Utc::now().timestamp(),
            exp: chrono::Utc::now().timestamp() + 3600,
        };
        
        let token = jwt_service.generate_token(&claims).unwrap();
        let decoded = jwt_service.validate_token(&token).unwrap();
        
        assert_eq!(decoded.sub, claims.sub);
        assert_eq!(decoded.email, claims.email);
    }
}
```

### Integration Tests
```rust
// tests/integration_tests.rs
use sqlx::PgPool;
use uuid::Uuid;
use olympus_auth::*;

#[sqlx::test]
async fn test_user_registration_and_login(pool: PgPool) {
    let auth_service = AuthService::new(pool);
    
    let register_req = RegisterRequest {
        email: "test@example.com".to_string(),
        password: "secure-password".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        tenant_slug: "test-tenant".to_string(),
    };
    
    // Test registration
    let user = auth_service.register(register_req).await.unwrap();
    assert_eq!(user.email, "test@example.com");
    
    // Test login
    let login_req = LoginRequest {
        email: "test@example.com".to_string(),
        password: "secure-password".to_string(),
        tenant_slug: "test-tenant".to_string(),
    };
    
    let token_response = auth_service.login(login_req).await.unwrap();
    assert!(!token_response.access_token.is_empty());
}
```

## 🚨 Critical Integration Points

### Database Schema Ownership
- **YOU OWN**: `docs/05-COMPLETE-DATABASE-SCHEMA.sql`
- **RULE**: All schema changes go through this file FIRST
- **MIGRATION**: Use sqlx migrate for development
- **PRODUCTION**: Terraform handles deployment

### API Contract Coordination
- **COORDINATE WITH**: ChatGPT (Go API Gateway)
- **UPDATE**: `docs/06-API-SPECIFICATION.yaml` for any endpoint changes
- **VERSIONING**: Use semantic versioning for breaking changes

### Event Schema Definition
```rust
// shared/src/events/mod.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainEvent {
    pub id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub tenant_id: Uuid,
    pub data: serde_json::Value,
    pub version: i32,
    pub occurred_at: DateTime<Utc>,
}

// Authentication events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginEvent {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub ip_address: String,
    pub user_agent: String,
    pub login_method: String, // password, oauth, etc.
}
```

## 🔄 Redis Event Publishing
```rust
// shared/src/events/publisher.rs
use redis::aio::ConnectionManager;
use serde_json;

pub struct EventPublisher {
    redis: ConnectionManager,
}

impl EventPublisher {
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(Self { redis })
    }
    
    pub async fn publish<T: Serialize>(&mut self, event: &DomainEvent) -> Result<(), Box<dyn std::error::Error>> {
        let channel = format!("events.{}", event.event_type);
        let payload = serde_json::to_string(event)?;
        
        redis::cmd("PUBLISH")
            .arg(&channel)
            .arg(&payload)
            .query_async(&mut self.redis)
            .await?;
            
        Ok(())
    }
}
```

## 🎯 Performance Benchmarks (REQUIRED)

### Latency Targets
- **JWT Generation**: <1ms
- **JWT Validation**: <0.5ms
- **Password Hashing**: <100ms (Argon2)
- **Database Query**: <10ms (simple), <50ms (complex)
- **Event Publishing**: <5ms

### Load Testing
```rust
// benches/auth_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_jwt_generation(c: &mut Criterion) {
    let jwt_service = JwtService::new(b"test-secret");
    let claims = create_test_claims();
    
    c.bench_function("jwt_generation", |b| {
        b.iter(|| jwt_service.generate_token(black_box(&claims)))
    });
}

fn benchmark_password_hashing(c: &mut Criterion) {
    let password_service = PasswordService;
    
    c.bench_function("password_hashing", |b| {
        b.iter(|| password_service.hash_password(black_box("test-password")))
    });
}

criterion_group!(benches, benchmark_jwt_generation, benchmark_password_hashing);
criterion_main!(benches);
```

## 📝 Documentation Requirements

### API Documentation
```rust
/// Creates a new user account
/// 
/// # Arguments
/// * `request` - Registration details including email, password, and tenant
/// 
/// # Returns
/// * `Ok(User)` - Successfully created user
/// * `Err(AuthError::EmailAlreadyExists)` - Email is already registered
/// * `Err(AuthError::InvalidTenant)` - Tenant does not exist
/// 
/// # Example
/// ```rust
/// let request = RegisterRequest {
///     email: "user@example.com".to_string(),
///     password: "secure-password".to_string(),
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     tenant_slug: "my-company".to_string(),
/// };
/// let user = auth_service.register(request).await?;
/// ```
pub async fn register(&self, request: RegisterRequest) -> Result<User, AuthError>
```

## 🏁 Success Criteria

### Week 1 Deliverables
- [ ] Rust workspace fully configured
- [ ] Authentication service complete with tests
- [ ] Database integration working
- [ ] JWT generation and validation
- [ ] Password hashing with Argon2
- [ ] Event publishing system
- [ ] Integration with Go API gateway
- [ ] Performance benchmarks passing
- [ ] Documentation complete

### Quality Gates
- [ ] `cargo test` - 100% pass rate
- [ ] `cargo clippy -- -D warnings` - Zero warnings
- [ ] `cargo audit` - Zero vulnerabilities
- [ ] Test coverage >90%
- [ ] All benchmarks within targets
- [ ] Security review complete

**Remember**: You are the foundation that everything else builds on. Quality and security are non-negotiable. Every line of code you write affects the entire platform's reliability and performance.

**Your motto**: *"Memory safe, blazingly fast, absolutely secure."*