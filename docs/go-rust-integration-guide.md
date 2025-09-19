# Go API Gateway ↔ Rust Services Integration Guide

> **For ChatGPT Agent**: Complete integration guide for Phase 9 advanced features

## 🎯 Overview

All Rust core services (auth, platform, commerce) + Phase 9 advanced features are complete and ready for Go API Gateway integration. This guide provides specific implementation details for ChatGPT to add proxy support for all advanced features.

## 📡 Current Rust Services Status

### ✅ Available Services
- **Auth Service** (Port 8000): JWT authentication, user management
- **Platform Service** (Port 8001): Tenant management, RBAC
- **Commerce Service** (Port 8002): Orders, products, payments, inventory
- **Advanced Features** (All services): GraphQL, WebSocket, caching, batch ops

### ✅ Phase 9 Advanced Features Ready for Integration

#### 1. GraphQL API (`/graphql`)
- **Rust Endpoint**: `http://localhost:8000/graphql` (or 8001, 8002)
- **Go Proxy Route**: `/api/v1/graphql`
- **Content-Type**: `application/json`
- **Features**: Complex queries, analytics, subscriptions

```go
// Add to Go API Gateway
graphql := v1.Group("/graphql")
{
    graphql.POST("", graphqlProxyHandler)
}

func graphqlProxyHandler(c *gin.Context) {
    // Forward to Rust GraphQL endpoint
    targetURL := "http://localhost:8000/graphql"
    proxyRequest(c, targetURL)
}
```

#### 2. WebSocket Real-time Updates (`/ws`)
- **Rust Endpoint**: `ws://localhost:8000/ws`
- **Go Proxy Route**: `/api/v1/ws`
- **Protocol**: WebSocket with JSON messages

```go
// Add WebSocket upgrader and proxy
import "github.com/gorilla/websocket"

var upgrader = websocket.Upgrader{
    CheckOrigin: func(r *http.Request) bool { return true },
}

router.GET("/api/v1/ws", wsProxyHandler)

func wsProxyHandler(c *gin.Context) {
    // Upgrade connection and proxy to Rust WebSocket
    // Forward JWT token for authentication
}
```

#### 3. Cache Management (`/cache/*`)
- **Rust Endpoints**:
  - `GET /cache/stats` - Cache statistics
  - `DELETE /cache/invalidate` - Cache invalidation
- **Go Proxy Routes**: `/api/v1/cache/*`

```go
cache := v1.Group("/cache")
{
    cache.GET("/stats", cacheStatsProxyHandler)
    cache.DELETE("/invalidate", cacheInvalidateProxyHandler)
}
```

#### 4. Batch Operations (`/batch/*`)
- **Rust Endpoints**:
  - `POST /batch/products` - Bulk product operations
  - `GET /batch/{id}/status` - Batch status tracking
- **Go Proxy Routes**: `/api/v1/batch/*`

```go
batch := v1.Group("/batch")
{
    batch.POST("/products", batchProductsProxyHandler)
    batch.GET("/:id/status", batchStatusProxyHandler)
}
```

#### 5. Health Checks (`/health/*`)
- **Rust Endpoints**:
  - `GET /health` - Comprehensive health check
  - `GET /health/live` - Liveness probe
  - `GET /health/ready` - Readiness probe
- **Go Proxy Routes**: `/api/v1/health/*`

```go
health := v1.Group("/health")
{
    health.GET("", aggregateHealthHandler) // Aggregate all services
    health.GET("/live", healthProxyHandler)
    health.GET("/ready", healthProxyHandler)
}
```

## 🔧 Implementation Details

### Service Discovery Configuration

```go
// Add to Go configuration
type ServiceConfig struct {
    AuthServiceURL      string // "http://localhost:8000"
    PlatformServiceURL  string // "http://localhost:8001"
    CommerceServiceURL  string // "http://localhost:8002"
}
```

### Generic Proxy Handler

```go
func proxyRequest(c *gin.Context, targetURL string) {
    // Create request
    req, err := http.NewRequest(c.Request.Method, targetURL, c.Request.Body)
    if err != nil {
        c.JSON(500, gin.H{"error": "Failed to create request"})
        return
    }

    // Copy headers (including Authorization)
    for key, values := range c.Request.Header {
        for _, value := range values {
            req.Header.Add(key, value)
        }
    }

    // Make request
    client := &http.Client{Timeout: 30 * time.Second}
    resp, err := client.Do(req)
    if err != nil {
        c.JSON(502, gin.H{"error": "Service unavailable"})
        return
    }
    defer resp.Body.Close()

    // Copy response
    for key, values := range resp.Header {
        for _, value := range values {
            c.Header(key, value)
        }
    }

    c.Status(resp.StatusCode)
    io.Copy(c.Writer, resp.Body)
}
```

### JWT Authentication Middleware

```go
func jwtAuthMiddleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        token := c.GetHeader("Authorization")
        if token == "" {
            c.JSON(401, gin.H{"error": "Missing authorization token"})
            c.Abort()
            return
        }

        // Forward token to Rust auth service for validation
        // Or implement JWT validation in Go

        c.Next()
    }
}
```

## 📊 Integration Priority Matrix

### High Priority (Week 1)
1. **GraphQL Proxy** - Critical for complex queries
2. **Health Check Aggregation** - Monitoring requirement
3. **WebSocket Proxy** - Real-time features

### Medium Priority (Week 2)
4. **Batch Operations Proxy** - Bulk processing
5. **Cache Management Proxy** - Performance optimization

### Nice to Have
6. **Compression Middleware** - Bandwidth optimization
7. **API Versioning Headers** - Future-proofing

## 🧪 Testing Integration

### 1. Test GraphQL Proxy
```bash
# Test via Go Gateway
curl -X POST http://localhost:8080/api/v1/graphql \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"query": "{ me { id email } }"}'
```

### 2. Test WebSocket Connection
```javascript
// Test in browser console
const ws = new WebSocket('ws://localhost:8080/api/v1/ws');
ws.onmessage = (event) => console.log(JSON.parse(event.data));
```

### 3. Test Batch Operations
```bash
# Test bulk product creation
curl -X POST http://localhost:8080/api/v1/batch/products \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"operations": [{"id": "1", "operation_type": "create", "data": {...}}]}'
```

## 🚨 Important Notes for ChatGPT

### Authentication Flow
- All Rust services expect JWT tokens in `Authorization: Bearer <token>` header
- Tokens are validated by Rust auth service
- Go should forward tokens transparently

### Error Handling
- Rust services return consistent error formats
- Go should preserve error responses without modification
- Add request tracing for debugging

### Performance Considerations
- Rust services are optimized for <50ms response times
- Add connection pooling for service communication
- Implement circuit breaker pattern for resilience

### WebSocket Special Handling
- WebSocket connections need upgrade handling
- Forward authentication context during upgrade
- Maintain connection state for proper cleanup

## 📝 Go Dependencies Needed

```go
// Add to go.mod
github.com/gorilla/websocket    // WebSocket support
github.com/prometheus/client_golang // Metrics
github.com/gin-contrib/cors     // CORS handling
```

## 🔗 Integration Status Tracking

- [ ] GraphQL proxy implemented
- [ ] WebSocket proxy implemented
- [ ] Cache management proxy implemented
- [ ] Batch operations proxy implemented
- [ ] Health check aggregation implemented
- [ ] JWT middleware implemented
- [ ] Error handling standardized
- [ ] Integration tests passing

## 📞 Support Available

**Claude Code (Rust)** is available for:
- Rust service endpoint clarification
- Authentication flow details
- Error response format questions
- Performance optimization guidance
- WebSocket message protocol details

**Contact via**: Update `docs/daily-status.md` with questions

---

**Ready for Implementation**: All Rust Phase 9 features are production-ready and waiting for Go proxy integration. Estimated implementation time: 2-3 days for full integration.