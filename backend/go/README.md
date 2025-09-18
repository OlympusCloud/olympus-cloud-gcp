# Go API Gateway - ChatGPT Agent

## Overview
This is the Go API Gateway service that acts as the central entry point for all client requests. It proxies to the appropriate backend services and provides GraphQL and WebSocket support.

## Owner
**ChatGPT** - Responsible for Go API Gateway implementation

## Features
- REST API routing
- GraphQL endpoint with subscriptions
- WebSocket for real-time communication
- JWT authentication middleware
- Request rate limiting
- Service mesh integration

## Quick Start

```bash
# Install dependencies
go mod download

# Run in development
go run cmd/api/main.go

# Or use air for hot reload
air

# Build for production
go build -o bin/api cmd/api/main.go
```

## Service Ports
- **API Gateway**: 8080 (main entry point)
- **GraphQL Playground**: 8080/graphql

## Integration Points
- **Rust Auth Service**: Proxy auth requests to port 8000
- **Python Analytics**: Forward analytics requests to port 8001
- **Redis**: Subscribe to events on port 6379
- **PostgreSQL**: Direct connection for GraphQL queries

## Directory Structure
```
backend/go/
├── cmd/
│   └── api/
│       └── main.go          # Entry point
├── internal/
│   ├── handlers/            # HTTP handlers
│   ├── middleware/          # Auth, CORS, rate limiting
│   ├── graphql/            # GraphQL schema and resolvers
│   └── websocket/          # WebSocket handlers
├── pkg/
│   ├── auth/               # JWT validation
│   └── proxy/              # Service proxy utilities
├── go.mod
└── go.sum
```

## Environment Variables
```
PORT=8080
AUTH_SERVICE_URL=http://localhost:8000
PYTHON_SERVICE_URL=http://localhost:8001
REDIS_URL=redis://localhost:6379
DATABASE_URL=postgresql://olympus:devpassword@localhost:5432/olympus
JWT_SECRET=development-secret-key
```

## Next Steps for ChatGPT
1. Initialize Gin router with middleware
2. Create proxy handlers for auth service
3. Implement GraphQL schema
4. Set up WebSocket for real-time features
5. Add Redis event subscriber