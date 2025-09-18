# Go API Gateway Integration Guide

## Overview
This document outlines the integration points between the Rust services and the Go API Gateway.

## Authentication Integration

### JWT Configuration
Both services must share the same JWT secret. Configure via environment variable:
```bash
JWT_SECRET=your-shared-secret-here
```

### Token Format
Access tokens use standard JWT with the following claims:
```json
{
  "sub": "user_uuid",
  "tenant_id": "tenant_uuid",
  "email": "user@example.com",
  "exp": 1234567890,
  "iat": 1234567890
}
```

Refresh tokens are opaque UUIDs stored in the database.

### Auth Endpoints
The Go Gateway should proxy these endpoints to the Rust auth service (port 8000):

#### Register New User
```http
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "first_name": "John",
  "last_name": "Doe",
  "tenant_slug": "acme-corp"
}

Response:
{
  "success": true,
  "data": {
    "user": { ... },
    "access_token": "jwt...",
    "refresh_token": "uuid...",
    "expires_in": 3600
  }
}
```

#### Login
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "tenant_slug": "acme-corp"
}

Response: Same as register
```

#### Refresh Token
```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "uuid..."
}

Response:
{
  "success": true,
  "data": {
    "access_token": "new-jwt...",
    "expires_in": 3600
  }
}
```

#### Logout
```http
POST /api/v1/auth/logout
Authorization: Bearer <access_token>
Content-Type: application/json

{
  "refresh_token": "uuid..."
}

Response:
{
  "success": true,
  "data": null
}
```

## Service Routing

### Platform Service Endpoints
Route to Rust platform service (port 8000):
- `GET /api/v1/tenants/{id}` - Get tenant details
- `POST /api/v1/tenants` - Create tenant
- `GET /api/v1/locations` - List locations
- `POST /api/v1/locations` - Create location
- `GET /api/v1/roles` - List roles
- `POST /api/v1/roles` - Create role
- `POST /api/v1/users/{user_id}/roles/{role_id}` - Assign role

### Commerce Service Endpoints
Route to Rust commerce service (port 8000):
- Products: `/api/v1/products/*`
- Orders: `/api/v1/orders/*`
- Inventory: `/api/v1/inventory/*`
- Customers: `/api/v1/customers/*`
- Payments: `/api/v1/payments/*`

## Middleware Requirements

### Authentication Middleware
The Go Gateway should:
1. Extract JWT from `Authorization: Bearer` header
2. Validate JWT signature using shared secret
3. Check token expiration
4. Extract claims and add to request context
5. Pass tenant_id header to downstream services

### Request Headers
Forward these headers to Rust services:
```
X-Tenant-ID: uuid
X-User-ID: uuid
X-Request-ID: uuid
Authorization: Bearer <token>
```

### Response Format
All services return consistent JSON:
```json
{
  "success": boolean,
  "data": T | null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable message"
  } | null,
  "metadata": {
    "request_id": "uuid",
    "timestamp": "2024-01-01T00:00:00Z"
  }
}
```

## Error Handling

### HTTP Status Codes
- 200 OK - Success
- 201 Created - Resource created
- 400 Bad Request - Validation error
- 401 Unauthorized - Invalid/missing token
- 403 Forbidden - Insufficient permissions
- 404 Not Found - Resource not found
- 409 Conflict - Resource already exists
- 500 Internal Server Error - Server error

### Error Response Format
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Email is required",
    "details": {
      "field": "email",
      "reason": "required"
    }
  }
}
```

## Health Checks

### Endpoints
- `/health` - Basic health check
- `/ready` - Readiness check (database connectivity)
- `/live` - Liveness check
- `/metrics` - Prometheus metrics

### Health Response
```json
{
  "status": "ok",
  "services": {
    "database": "ok",
    "redis": "ok"
  },
  "version": "1.0.0",
  "uptime": 3600
}
```

## GraphQL Considerations

If the Go Gateway exposes GraphQL:

### Type Mapping
- Rust UUID → GraphQL ID
- Rust DateTime<Utc> → GraphQL DateTime
- Rust Decimal → GraphQL Float or custom Scalar
- Rust Option<T> → GraphQL nullable field

### Suggested Schema
```graphql
type User {
  id: ID!
  email: String!
  firstName: String
  lastName: String
  tenant: Tenant!
  roles: [Role!]!
  createdAt: DateTime!
}

type AuthPayload {
  user: User!
  accessToken: String!
  refreshToken: String!
  expiresIn: Int!
}

type Mutation {
  login(email: String!, password: String!, tenantSlug: String!): AuthPayload!
  register(input: RegisterInput!): AuthPayload!
  refreshToken(token: String!): TokenPayload!
  logout(refreshToken: String!): Boolean!
}
```

## WebSocket Events

For real-time features, the Go Gateway can subscribe to Redis events:

### Event Channels
- `events:user:*` - User events (login, logout, profile updates)
- `events:order:*` - Order events (created, updated, fulfilled)
- `events:payment:*` - Payment events (processed, failed, refunded)
- `events:inventory:*` - Inventory events (low stock, restocked)

### Event Format
```json
{
  "type": "order.created",
  "tenant_id": "uuid",
  "user_id": "uuid",
  "data": { ... },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Development Setup

1. Start Rust services:
```bash
cd backend/rust
docker-compose up -d  # Start PostgreSQL and Redis
cargo run --release   # Start Rust services on port 8000
```

2. Configure Go Gateway:
```go
// Example proxy configuration
rustServiceURL := "http://localhost:8000"
proxyAuth := httputil.NewSingleHostReverseProxy(rustServiceURL)
```

3. Shared environment variables:
```bash
JWT_SECRET=shared-secret
REDIS_URL=redis://localhost:6379
DATABASE_URL=postgres://olympus:devpassword@localhost:5432/olympus
```

## Testing Integration

1. Test auth flow:
```bash
# Register
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!","tenant_slug":"test"}'

# Login
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!","tenant_slug":"test"}'
```

2. Test authenticated request:
```bash
curl http://localhost:8080/api/v1/products \
  -H "Authorization: Bearer <token>"
```

## Contact
For questions about Rust service integration, see `/backend/rust/STATUS.md`