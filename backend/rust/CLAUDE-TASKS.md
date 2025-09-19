# 🦀 Rust Core Services - Claude Code Agent Task List

> **Agent:** Claude Code | **Services:** Auth, Platform, Commerce | **Ports:** 8000-8002 | **Priority:** CRITICAL

## 📋 Mission Statement
Build production-ready Rust microservices that form the core business logic of Olympus Cloud, providing secure authentication, multi-tenant platform management, and comprehensive commerce functionality with event-driven architecture.

## 🎯 Current Status
- ✅ Workspace structure and dependencies (15% complete)
- ✅ Basic commerce demo implementation (20% complete)
- ❌ Missing all production features and security

## 📝 Complete Task List

### Phase 1: Foundation & Database (Week 1)

#### Task 1.1: Database Schema Implementation
- [ ] **Create complete database migrations** (`migrations/`)
  ```sql
  -- Priority order:
  001_initial_schema.sql      # Core platform tables
  002_auth_system.sql         # Authentication tables
  003_commerce_system.sql     # Commerce tables
  004_analytics_tables.sql    # Analytics and events
  005_indexes_and_constraints.sql
  ```

- [ ] **Implement shared database models** (`shared/src/models/`)
  ```rust
  // Core models needed:
  mod user;           // User entity with validation
  mod tenant;         // Multi-tenant support
  mod session;        // Session management
  mod permission;     // RBAC system
  mod product;        // Product catalog
  mod order;          // Order management
  mod payment;        // Payment processing
  mod event;          // Domain events
  ```

- [ ] **Create database connection management** (`shared/src/database/`)
  - SQLX connection pool configuration
  - Health check implementation
  - Migration runner
  - Transaction management utilities
  - Row-level security (RLS) implementation

#### Task 1.2: Shared Infrastructure
- [ ] **Error handling system** (`shared/src/error.rs`)
  ```rust
  // Comprehensive error types:
  #[derive(thiserror::Error, Debug)]
  pub enum OlympusError {
      #[error("Database error: {0}")]
      Database(#[from] sqlx::Error),

      #[error("Authentication error: {0}")]
      Authentication(String),

      #[error("Authorization error: {0}")]
      Authorization(String),

      #[error("Validation error: {0}")]
      Validation(#[from] validator::ValidationErrors),

      #[error("Business logic error: {0}")]
      Business(String),
  }
  ```

- [ ] **Configuration management** (`shared/src/config.rs`)
  - Environment-based configuration
  - Database connection settings
  - Redis configuration
  - JWT secret management
  - Service discovery settings

- [ ] **Event system foundation** (`shared/src/events/`)
  - Domain event definitions
  - Event publishing infrastructure
  - Event serialization/deserialization
  - Redis integration for event streaming

### Phase 2: Authentication Service (Port 8000) - Week 1-2

#### Task 2.1: User Management Core
- [ ] **User registration system** (`auth/src/handlers/register.rs`)
  ```rust
  // Features needed:
  // - Email validation and verification
  // - Password strength validation
  // - Argon2 password hashing
  // - Duplicate email prevention
  // - Tenant association
  // - Email verification workflow
  ```

- [ ] **User login system** (`auth/src/handlers/login.rs`)
  ```rust
  // Features needed:
  // - Multi-tenant login (email + tenant_slug)
  // - Password verification with Argon2
  // - JWT token generation
  // - Refresh token creation
  // - Account lockout after failed attempts
  // - Login audit logging
  ```

- [ ] **Password management** (`auth/src/handlers/password.rs`)
  - Password reset flow with email verification
  - Password change with current password verification
  - Password strength validation
  - Password history prevention

#### Task 2.2: JWT Token System
- [ ] **JWT implementation** (`auth/src/services/jwt.rs`)
  ```rust
  // Token features needed:
  // - Access token generation (short-lived: 15 minutes)
  // - Refresh token generation (long-lived: 30 days)
  // - Token validation and claims extraction
  // - Token revocation support
  // - Multi-device token management
  // - Token renewal workflow
  ```

- [ ] **Token validation middleware** (`auth/src/middleware/`)
  - JWT signature validation
  - Token expiration checking
  - User context extraction
  - Tenant context validation
  - Rate limiting integration

#### Task 2.3: Session Management
- [ ] **Redis session store** (`auth/src/services/session.rs`)
  - Session creation and storage
  - Session validation and cleanup
  - Multi-device session tracking
  - Session invalidation on logout
  - Session expiry management

- [ ] **Device management** (`auth/src/services/device.rs`)
  - Device registration and tracking
  - Device-specific session management
  - Suspicious device detection
  - Device revocation capabilities

#### Task 2.4: Security Features
- [ ] **Account security** (`auth/src/services/security.rs`)
  - Failed login attempt tracking
  - Account lockout mechanisms
  - IP-based access control
  - Suspicious activity detection
  - Security event logging

- [ ] **Multi-factor authentication** (`auth/src/services/mfa.rs`)
  - TOTP implementation
  - Backup codes generation
  - MFA enrollment process
  - MFA validation
  - Recovery mechanisms

#### Task 2.5: Email Verification System
- [ ] **Email service integration** (`auth/src/services/email.rs`)
  - Email verification token generation
  - Email template system
  - Email sending integration (SendGrid/SES)
  - Email verification workflow
  - Resend verification handling

### Phase 3: Platform Service (Port 8001) - Week 2

#### Task 3.1: Tenant Management
- [ ] **Tenant administration** (`platform/src/handlers/tenants.rs`)
  ```rust
  // Tenant features needed:
  // - Tenant creation and configuration
  // - Subscription plan management
  // - Feature flag configuration
  // - Billing information management
  // - Tenant deletion with data cleanup
  ```

- [ ] **Multi-tenancy enforcement** (`platform/src/middleware/tenant.rs`)
  - Tenant isolation validation
  - Row-level security enforcement
  - Cross-tenant access prevention
  - Tenant context injection

#### Task 3.2: User & Role Management
- [ ] **User management** (`platform/src/handlers/users.rs`)
  - User profile management
  - User role assignments
  - User permissions management
  - User activity tracking
  - User deactivation/reactivation

- [ ] **Role-based access control** (`platform/src/services/rbac.rs`)
  ```rust
  // RBAC features needed:
  // - Role hierarchy system
  // - Permission-based authorization
  // - Resource-level permissions
  // - Dynamic permission checking
  // - Role inheritance
  ```

#### Task 3.3: Location Management
- [ ] **Multi-location support** (`platform/src/handlers/locations.rs`)
  - Location creation and management
  - Location hierarchy relationships
  - Location-specific configurations
  - Location-based user assignments
  - Location analytics integration

#### Task 3.4: Configuration Management
- [ ] **Feature flags** (`platform/src/services/feature_flags.rs`)
  - Per-tenant feature configuration
  - Feature rollout management
  - A/B testing support
  - Feature usage analytics

- [ ] **System configuration** (`platform/src/services/config.rs`)
  - Tenant-specific settings
  - Global system settings
  - Configuration validation
  - Configuration change auditing

### Phase 4: Commerce Service (Port 8002) - Week 2-3

#### Task 4.1: Product Catalog Management
- [ ] **Complete product system** (`commerce/src/handlers/products.rs`)
  ```rust
  // Replace simple demo with full implementation:
  // - Product variants and options
  // - Product categories and tags
  // - Product search and filtering
  // - Product image management
  // - Product pricing rules
  // - Product availability tracking
  ```

- [ ] **Product catalog features** (`commerce/src/services/catalog.rs`)
  - Category hierarchy management
  - Product search implementation
  - Filtering and sorting capabilities
  - Product recommendations
  - Bulk product operations

#### Task 4.2: Advanced Order Management
- [ ] **Complete order system** (`commerce/src/handlers/orders.rs`)
  ```rust
  // Enhanced order features:
  // - Order lifecycle management (draft → confirmed → fulfilled)
  // - Order modification and cancellation
  // - Order item management
  // - Order status tracking
  // - Order history and audit trail
  // - Partial fulfillment support
  ```

- [ ] **Order processing** (`commerce/src/services/order_processor.rs`)
  - Order validation logic
  - Inventory availability checking
  - Order total calculation
  - Tax calculation integration
  - Shipping calculation
  - Order confirmation workflow

#### Task 4.3: Payment Processing
- [ ] **Payment system** (`commerce/src/handlers/payments.rs`)
  - Payment method management
  - Payment processing workflows
  - Payment status tracking
  - Refund processing
  - Chargeback handling
  - Payment retry logic

- [ ] **Payment gateway integration** (`commerce/src/services/payments/`)
  ```rust
  // Payment providers:
  mod stripe;     // Stripe integration
  mod square;     // Square integration
  mod paypal;     // PayPal integration
  mod gateway;    // Unified payment interface
  ```

#### Task 4.4: Inventory Management
- [ ] **Inventory tracking** (`commerce/src/services/inventory.rs`)
  - Stock level management
  - Inventory allocation for orders
  - Low stock alerts
  - Inventory adjustments
  - Inventory history tracking
  - Multi-location inventory

#### Task 4.5: Analytics & Reporting
- [ ] **Commerce analytics** (`commerce/src/services/analytics.rs`)
  - Sales performance metrics
  - Product performance tracking
  - Order analytics
  - Revenue calculations
  - Customer analytics
  - Inventory analytics

### Phase 5: Event-Driven Architecture (Week 3)

#### Task 5.1: Domain Events
- [ ] **Event definitions** (`shared/src/events/mod.rs`)
  ```rust
  // Core events needed:
  #[derive(Debug, Serialize, Deserialize)]
  pub enum DomainEvent {
      // Auth events
      UserRegistered { user_id: Uuid, tenant_id: Uuid },
      UserLoggedIn { user_id: Uuid, device_id: String },
      UserLoggedOut { user_id: Uuid, session_id: String },

      // Commerce events
      ProductCreated { product_id: Uuid, tenant_id: Uuid },
      OrderCreated { order_id: Uuid, user_id: Uuid },
      OrderStatusChanged { order_id: Uuid, old_status: String, new_status: String },
      PaymentProcessed { payment_id: Uuid, order_id: Uuid, amount: Decimal },

      // Platform events
      TenantCreated { tenant_id: Uuid, plan: String },
      UserRoleChanged { user_id: Uuid, old_role: String, new_role: String },
  }
  ```

#### Task 5.2: Event Publishing
- [ ] **Redis event publisher** (`shared/src/events/publisher.rs`)
  - Event serialization and publishing
  - Event ordering guarantees
  - Retry mechanisms for failed publishes
  - Event deduplication
  - Batch event publishing

#### Task 5.3: Event Processing
- [ ] **Event subscribers** (per service)
  - Cross-service event handling
  - Event-driven state updates
  - Async event processing
  - Error handling and dead letter queues
  - Event replay capabilities

### Phase 6: API Integration & Communication (Week 3)

#### Task 6.1: Service-to-Service Communication
- [ ] **HTTP client implementation** (`shared/src/http/`)
  - Async HTTP client setup
  - Request/response serialization
  - Error handling and retries
  - Circuit breaker pattern
  - Load balancing support

#### Task 6.2: Health Checks & Monitoring
- [ ] **Health check endpoints** (all services)
  ```rust
  // Health check features:
  // - Database connectivity check
  // - Redis connectivity check
  // - Dependency service health
  // - Resource utilization metrics
  // - Service-specific health indicators
  ```

- [ ] **Metrics collection** (`shared/src/metrics/`)
  - Request/response metrics
  - Business metrics
  - Performance metrics
  - Error rate tracking
  - Custom metric definitions

### Phase 7: Security & Validation (Week 3-4)

#### Task 7.1: Input Validation
- [ ] **Request validation** (all services)
  ```rust
  // Validation features:
  // - Request body validation with serde
  // - Query parameter validation
  // - Path parameter validation
  // - Custom validation rules
  // - Sanitization of user input
  ```

- [ ] **Business rule validation**
  - Domain-specific validation logic
  - Cross-entity validation
  - Constraint checking
  - Data integrity validation

#### Task 7.2: Security Hardening
- [ ] **SQL injection prevention**
  - Parameterized queries only
  - Input sanitization
  - Query validation
  - Database access auditing

- [ ] **XSS prevention**
  - Output encoding
  - Content Security Policy
  - Input sanitization
  - Safe HTML handling

#### Task 7.3: Audit Logging
- [ ] **Comprehensive audit system** (`shared/src/audit/`)
  - User action logging
  - Data change tracking
  - Security event logging
  - Compliance reporting
  - Log retention policies

### Phase 8: Testing & Quality (Week 4)

#### Task 8.1: Unit Testing
- [ ] **Service unit tests** (90%+ coverage target)
  ```rust
  // Test coverage areas:
  // - Business logic functions
  // - Data validation
  // - Error handling
  // - Edge cases
  // - Security features
  ```

- [ ] **Mock implementations**
  - Database mocking with sqlx-test
  - Redis mocking
  - HTTP client mocking
  - External service mocking

#### Task 8.2: Integration Testing
- [ ] **Database integration tests**
  - CRUD operation testing
  - Transaction testing
  - Constraint validation
  - Migration testing

- [ ] **Service integration tests**
  - Inter-service communication
  - Event processing
  - End-to-end workflows
  - Error recovery testing

#### Task 8.3: Load Testing
- [ ] **Performance testing**
  - Concurrent request handling
  - Database connection pooling
  - Memory usage profiling
  - Response time analysis
  - Bottleneck identification

### Phase 9: Production Readiness (Week 4)

#### Task 9.1: Configuration Management
- [ ] **Environment configuration**
  - Development environment setup
  - Staging environment configuration
  - Production environment hardening
  - Secret management integration

#### Task 9.2: Containerization
- [ ] **Docker implementation** (per service)
  ```dockerfile
  # Multi-stage Dockerfile features:
  # - Optimized build stage
  # - Minimal runtime image
  # - Non-root user execution
  # - Health check configuration
  # - Security scanning
  ```

#### Task 9.3: Deployment Preparation
- [ ] **Graceful shutdown**
  - Signal handling
  - Connection draining
  - Background task completion
  - Resource cleanup

- [ ] **Startup validation**
  - Configuration validation
  - Dependency connectivity
  - Database migration checks
  - Service registration

## 🔧 Development Commands

```bash
# Workspace commands
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --workspace

# Individual service commands
cd auth && cargo run
cd platform && cargo run
cd commerce && cargo run

# Database operations
sqlx migrate run
sqlx prepare --workspace

# Testing
cargo test --workspace --no-fail-fast
cargo test --workspace --release

# Security audit
cargo audit
```

## 📊 Success Metrics

### Technical Metrics
- [ ] API response time < 50ms (p99)
- [ ] Database query time < 25ms (p99)
- [ ] Memory usage < 256MB per service
- [ ] CPU usage < 50% under normal load
- [ ] Unit test coverage > 90%
- [ ] Integration test coverage > 80%

### Security Metrics
- [ ] Zero SQL injection vulnerabilities
- [ ] Zero XSS vulnerabilities
- [ ] All inputs validated and sanitized
- [ ] Comprehensive audit logging
- [ ] Secure password hashing (Argon2)
- [ ] JWT tokens properly secured

### Business Metrics
- [ ] Multi-tenant isolation working
- [ ] Authentication system secure and reliable
- [ ] Role-based access control functional
- [ ] Commerce workflows complete
- [ ] Event-driven communication working
- [ ] Real-time features operational

## 🚨 Critical Dependencies

1. **PostgreSQL Database** - Must be available with proper schema
2. **Redis Server** - Required for sessions and events
3. **SMTP Service** - Needed for email verification
4. **Go API Gateway** - Must accept Rust service connections
5. **Python Analytics** - Must receive events from Rust services

## 📋 Daily Progress Tracking

Create daily updates in format:
```
## [Date] - Claude Code Progress Update

### Auth Service Progress
- [ ] Completed features with details

### Platform Service Progress
- [ ] Completed features with details

### Commerce Service Progress
- [ ] Completed features with details

### Blocked Issues
- [ ] Description and resolution needed

### Next Day Plan
- [ ] Priority tasks for tomorrow
```

## 🎯 Final Deliverables

1. **Three production-ready Rust services** with complete functionality
2. **Comprehensive test suite** with high coverage
3. **Complete database schema** with migrations
4. **Event-driven architecture** fully implemented
5. **Security features** properly implemented
6. **Docker containers** ready for deployment
7. **Documentation** for all APIs and services
8. **Performance benchmarks** meeting target metrics
9. **Security audit** with no critical vulnerabilities
10. **Integration** with Go gateway and Python analytics