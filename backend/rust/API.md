# Olympus Rust Core Services API Documentation

## Overview
The Rust core services provide the foundational business logic for Olympus Cloud, including authentication, platform management, and commerce operations.

## Base URL
- Development: `http://localhost:8000/api/v1`
- Production: `https://api.olympuscloud.io/api/v1`

## Authentication

All endpoints except `/auth/register` and `/auth/login` require a valid JWT token in the Authorization header:

```
Authorization: Bearer <token>
```

## Service Endpoints

### Health Check
- **GET** `/health` - Service health status
- **GET** `/status` - Detailed service status with all endpoints

---

## Authentication Service (`/auth`)

### Register
**POST** `/auth/register`
```json
{
  "tenant_id": "uuid",
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "first_name": "John",
  "last_name": "Doe"
}
```

### Login
**POST** `/auth/login`
```json
{
  "tenant_id": "uuid",
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
```

### Refresh Token
**POST** `/auth/refresh`
```json
{
  "refresh_token": "token_string"
}
```

### Logout
**POST** `/auth/logout`
```json
{
  "refresh_token": "token_string"
}
```

### Verify Email
**POST** `/auth/verify`
```json
{
  "token": "verification_token"
}
```

### Request Password Reset
**POST** `/auth/reset-password/request`
```json
{
  "email": "user@example.com"
}
```

### Reset Password
**POST** `/auth/reset-password/confirm`
```json
{
  "token": "reset_token",
  "new_password": "NewSecurePassword123!"
}
```

---

## Platform Service (`/platform`)

### Tenants

#### List Tenants
**GET** `/tenants?page=1&per_page=20`

#### Create Tenant
**POST** `/tenants`
```json
{
  "slug": "company-slug",
  "name": "Company Name",
  "industry": "Technology",
  "subscription_tier": "STARTER",
  "billing_email": "billing@company.com"
}
```

#### Get Tenant
**GET** `/tenants/:id`

#### Update Tenant
**PUT** `/tenants/:id`
```json
{
  "name": "Updated Name",
  "display_name": "Updated Display",
  "description": "Updated description"
}
```

#### Delete Tenant
**DELETE** `/tenants/:id`

### Locations

#### List Locations
**GET** `/locations?tenant_id=uuid`

#### Create Location
**POST** `/locations?tenant_id=uuid`
```json
{
  "name": "Main Office",
  "code": "HQ001",
  "description": "Headquarters",
  "address": {
    "street": "123 Main St",
    "city": "San Francisco",
    "state": "CA",
    "zip": "94105"
  },
  "timezone": "America/Los_Angeles"
}
```

#### Get Location
**GET** `/locations/:id`

#### Update Location
**PUT** `/locations/:id`

#### Delete Location
**DELETE** `/locations/:id`

### Roles

#### List Roles
**GET** `/roles?tenant_id=uuid`

#### Create Role
**POST** `/roles?tenant_id=uuid`
```json
{
  "name": "manager",
  "display_name": "Manager",
  "description": "Manager role",
  "permissions": ["users.read", "users.write", "orders.read"]
}
```

#### Get Role
**GET** `/roles/:id`

#### Update Role
**PUT** `/roles/:id`

#### Delete Role
**DELETE** `/roles/:id`

### Permissions

#### List All Permissions
**GET** `/permissions`

#### Assign Role to User
**POST** `/users/:user_id/roles`
```json
{
  "role_id": "uuid"
}
```

#### Remove Role from User
**DELETE** `/users/:user_id/roles/:role_id`

---

## Commerce Service (`/commerce`)

### Products

#### List Products
**GET** `/products?tenant_id=uuid&page=1&per_page=20`

#### Create Product
**POST** `/products`
```json
{
  "tenant_id": "uuid",
  "sku": "PROD-001",
  "name": "Product Name",
  "description": "Product description",
  "unit_price": 99.99,
  "category_id": "uuid",
  "track_inventory": true
}
```

#### Get Product
**GET** `/products/:id`

#### Update Product
**PUT** `/products/:id`
```json
{
  "name": "Updated Name",
  "description": "Updated description",
  "unit_price": 89.99
}
```

#### Delete Product
**DELETE** `/products/:id`

### Inventory

#### Get Inventory
**GET** `/inventory/:product_id?location_id=uuid`

#### Update Inventory
**PUT** `/inventory/:product_id`
```json
{
  "location_id": "uuid",
  "quantity": 100
}
```

#### Adjust Inventory
**POST** `/inventory/adjust`
```json
{
  "product_id": "uuid",
  "location_id": "uuid",
  "adjustment_type": "ADD",
  "quantity": 50,
  "reason": "Restock"
}
```

### Orders

#### List Orders
**GET** `/orders?tenant_id=uuid&page=1&per_page=20`

#### Create Order
**POST** `/orders`
```json
{
  "tenant_id": "uuid",
  "customer_id": "uuid",
  "location_id": "uuid",
  "items": [
    {
      "product_id": "uuid",
      "sku": "PROD-001",
      "name": "Product Name",
      "quantity": 2,
      "unit_price": 99.99
    }
  ],
  "shipping_address": {
    "street": "123 Main St",
    "city": "San Francisco",
    "state": "CA",
    "zip": "94105"
  }
}
```

#### Get Order
**GET** `/orders/:id`

#### Cancel Order
**POST** `/orders/:id/cancel`

#### Fulfill Order
**POST** `/orders/:id/fulfill`

#### Refund Order
**POST** `/orders/:id/refund`

### Customers

#### List Customers
**GET** `/customers?tenant_id=uuid&page=1&per_page=20`

#### Create Customer
**POST** `/customers`
```json
{
  "tenant_id": "uuid",
  "email": "customer@example.com",
  "first_name": "Jane",
  "last_name": "Smith",
  "phone": "+1-555-0123"
}
```

#### Get Customer
**GET** `/customers/:id`

#### Update Customer
**PUT** `/customers/:id`

#### Get Customer Orders
**GET** `/customers/:id/orders`

### Payments

#### Process Payment
**POST** `/payments/process`
```json
{
  "tenant_id": "uuid",
  "order_id": "uuid",
  "amount": 199.98,
  "payment_method": "CREDIT_CARD",
  "payment_type": "SALE",
  "currency": "USD",
  "gateway": "stripe",
  "card_last_four": "4242",
  "card_brand": "Visa"
}
```

#### Get Payment
**GET** `/payments/:id`

#### Capture Payment
**POST** `/payments/:id/capture`

#### Void Payment
**POST** `/payments/:id/void`

---

## Response Format

All responses follow this format:

### Success Response
```json
{
  "success": true,
  "data": { ... },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

### Error Response
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error message"
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

## Status Codes

- `200 OK` - Request successful
- `201 Created` - Resource created
- `400 Bad Request` - Invalid request parameters
- `401 Unauthorized` - Authentication required or failed
- `403 Forbidden` - Access denied
- `404 Not Found` - Resource not found
- `409 Conflict` - Resource already exists
- `500 Internal Server Error` - Server error

## Rate Limiting

- 100 requests per minute per IP for unauthenticated endpoints
- 1000 requests per minute per user for authenticated endpoints

## Event Publishing

The following events are published to Redis for cross-service communication:

- `events.user.registered`
- `events.user.logged_in`
- `events.tenant.created`
- `events.product.created`
- `events.product.updated`
- `events.product.deleted`
- `events.order.created`
- `events.order.cancelled`
- `events.order.fulfilled`
- `events.payment.processed`
- `events.inventory.updated`