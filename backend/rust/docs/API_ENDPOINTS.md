# Rust Services API Endpoints Reference

This document provides a comprehensive reference for all API endpoints exposed by the Rust services that the Go API Gateway should proxy and integrate with.

## Service Overview

| Service | Port | Base Path | Description |
|---------|------|-----------|-------------|
| Auth Service | 8000 | `/api/v1/auth` | Authentication and user management |
| Platform Service | 8001 | `/api/v1/platform` | Tenant and role management |
| Commerce Service | 8002 | `/api/v1/commerce` | Orders, products, inventory |

## Authentication Service (Port 8000)

### Auth Endpoints

#### POST /api/v1/auth/register
Register a new user account.

**Request:**
```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "first_name": "John",
  "last_name": "Doe",
  "phone": "+1234567890",
  "tenant_slug": "acme-corp"
}
```

**Response (201):**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "roles": ["user"],
      "is_active": true
    },
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "refresh_token": "550e8400-e29b-41d4-a716-446655440000",
    "expires_in": 3600
  }
}
```

#### POST /api/v1/auth/login
Authenticate user and return tokens.

**Request:**
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "tenant_slug": "acme-corp",
  "device_id": "device-123",
  "device_name": "iPhone 15"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "user": { /* User object */ },
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "refresh_token": "550e8400-e29b-41d4-a716-446655440000",
    "expires_in": 3600
  }
}
```

#### POST /api/v1/auth/refresh
Refresh access token using refresh token.

**Request:**
```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "access_token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
    "expires_in": 3600
  }
}
```

#### POST /api/v1/auth/logout
Invalidate current session and refresh token.

**Request:**
```http
POST /api/v1/auth/logout
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
Content-Type: application/json

{
  "refresh_token": "550e8400-e29b-41d4-a716-446655440000"
}
```

**Response (200):**
```json
{
  "success": true,
  "message": "Logged out successfully"
}
```

#### GET /api/v1/auth/me
Get current user profile.

**Request:**
```http
GET /api/v1/auth/me
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "uuid",
      "tenant_id": "uuid",
      "email": "user@example.com",
      "first_name": "John",
      "last_name": "Doe",
      "display_name": "John Doe",
      "phone": "+1234567890",
      "roles": ["user", "admin"],
      "permissions": ["read", "write"],
      "is_active": true,
      "email_verified": true,
      "created_at": "2023-01-01T00:00:00Z"
    }
  }
}
```

## Platform Service (Port 8001)

### Tenant Management

#### GET /api/v1/platform/tenants/current
Get current tenant information.

**Request:**
```http
GET /api/v1/platform/tenants/current
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "tenant": {
      "id": "uuid",
      "slug": "acme-corp",
      "name": "Acme Corporation",
      "industry": "Technology",
      "subscription_tier": "premium",
      "is_active": true,
      "settings": {
        "timezone": "UTC",
        "currency": "USD"
      }
    }
  }
}
```

### Location Management

#### GET /api/v1/platform/locations
List all locations for current tenant.

**Request:**
```http
GET /api/v1/platform/locations
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "locations": [
      {
        "id": "uuid",
        "tenant_id": "uuid",
        "name": "Main Store",
        "address": {
          "street": "123 Main St",
          "city": "Anytown",
          "state": "CA",
          "postal_code": "12345",
          "country": "US"
        },
        "settings": {},
        "is_active": true
      }
    ]
  }
}
```

#### POST /api/v1/platform/locations
Create new location.

**Request:**
```http
POST /api/v1/platform/locations
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
Content-Type: application/json

{
  "name": "New Store",
  "address": {
    "street": "456 Oak Ave",
    "city": "Somewhere",
    "state": "NY",
    "postal_code": "67890",
    "country": "US"
  },
  "phone": "+1987654321",
  "email": "newstore@acme.com"
}
```

### Role Management

#### GET /api/v1/platform/roles
List available roles for current tenant.

**Request:**
```http
GET /api/v1/platform/roles
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "roles": [
      {
        "id": "uuid",
        "name": "admin",
        "display_name": "Administrator",
        "description": "Full system access",
        "permissions": ["read", "write", "delete", "admin"]
      },
      {
        "id": "uuid",
        "name": "manager",
        "display_name": "Manager",
        "description": "Location management access",
        "permissions": ["read", "write"]
      }
    ]
  }
}
```

## Commerce Service (Port 8002)

### Product Management

#### GET /api/v1/commerce/products
List products with filtering and pagination.

**Request:**
```http
GET /api/v1/commerce/products?location_id=uuid&category=electronics&page=1&limit=20
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "products": [
      {
        "id": "uuid",
        "tenant_id": "uuid",
        "location_id": "uuid",
        "name": "iPhone 15",
        "description": "Latest smartphone",
        "sku": "IPH15-128",
        "price": 799.99,
        "cost": 500.00,
        "category": "electronics",
        "tags": ["smartphone", "apple"],
        "is_active": true,
        "created_at": "2023-01-01T00:00:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "limit": 20,
      "total": 150,
      "total_pages": 8
    }
  }
}
```

#### POST /api/v1/commerce/products
Create new product.

**Request:**
```http
POST /api/v1/commerce/products
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
Content-Type: application/json

{
  "location_id": "uuid",
  "name": "MacBook Pro",
  "description": "Professional laptop",
  "sku": "MBP-16-512",
  "price": 2499.99,
  "cost": 1800.00,
  "category": "computers",
  "tags": ["laptop", "apple", "professional"]
}
```

### Order Management

#### GET /api/v1/commerce/orders
List orders with filtering.

**Request:**
```http
GET /api/v1/commerce/orders?status=pending&location_id=uuid&page=1&limit=10
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "orders": [
      {
        "id": "uuid",
        "tenant_id": "uuid",
        "location_id": "uuid",
        "customer_id": "uuid",
        "order_number": "ORD-20240101-001",
        "status": "pending",
        "total_amount": 1299.98,
        "items": [
          {
            "product_id": "uuid",
            "name": "iPhone 15",
            "quantity": 1,
            "unit_price": 799.99,
            "total": 799.99
          }
        ],
        "created_at": "2023-01-01T12:00:00Z"
      }
    ]
  }
}
```

#### POST /api/v1/commerce/orders
Create new order.

**Request:**
```http
POST /api/v1/commerce/orders
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
Content-Type: application/json

{
  "location_id": "uuid",
  "customer_id": "uuid",
  "items": [
    {
      "product_id": "uuid",
      "quantity": 2,
      "unit_price": 799.99
    }
  ],
  "payment_method": "card"
}
```

### Inventory Management

#### GET /api/v1/commerce/inventory/stock
Get inventory levels.

**Request:**
```http
GET /api/v1/commerce/inventory/stock?location_id=uuid&product_id=uuid
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
```

**Response (200):**
```json
{
  "success": true,
  "data": {
    "stock": [
      {
        "id": "uuid",
        "product_id": "uuid",
        "location_id": "uuid",
        "quantity_available": 50,
        "quantity_reserved": 5,
        "min_stock_level": 10,
        "reorder_point": 15,
        "updated_at": "2023-01-01T12:00:00Z"
      }
    ]
  }
}
```

## Health & Status Endpoints

All services expose health check endpoints:

#### GET /health
Service health check.

**Response (200):**
```json
{
  "status": "healthy",
  "service": "auth-service",
  "version": "0.1.0",
  "timestamp": "2023-01-01T12:00:00Z"
}
```

## Error Handling

All endpoints use consistent error response format:

**Error Response (4xx/5xx):**
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid email format",
    "details": {
      "field": "email",
      "value": "invalid-email"
    }
  }
}
```

## Authentication Headers

Most endpoints require authentication. The Go Gateway should:

1. Extract JWT from `Authorization: Bearer {token}` header
2. Verify JWT signature using shared secret
3. Forward valid requests with original Authorization header
4. Return 401 for invalid/missing tokens

## Request/Response Middleware

The Go Gateway should implement:

1. **CORS handling** for browser requests
2. **Rate limiting** per user/tenant
3. **Request logging** for audit trails
4. **Response compression** for large payloads
5. **Request ID propagation** for tracing

## Integration Testing

Test these endpoints using the provided integration tests in each service:

```bash
# Auth service tests
cargo test --package olympus-auth --test integration_tests

# Platform service tests
cargo test --package olympus-platform --test integration_tests

# Commerce service tests
cargo test --package olympus-commerce --test integration_tests
```

## Environment Configuration

Configure service URLs in Go Gateway:

```env
RUST_AUTH_SERVICE_URL=http://localhost:8000
RUST_PLATFORM_SERVICE_URL=http://localhost:8001
RUST_COMMERCE_SERVICE_URL=http://localhost:8002
JWT_SECRET=your-shared-secret-key
```

## Next Steps

1. Implement Go middleware for JWT validation
2. Create service proxy handlers
3. Add request/response transformation if needed
4. Implement circuit breakers for resilience
5. Add comprehensive logging and metrics