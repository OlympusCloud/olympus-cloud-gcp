# ChatGPT - Go API Gateway & Integration Lead

> **Your Mission**: Build the high-performance API gateway that seamlessly connects all services and provides lightning-fast responses to clients

## 🎯 Your Primary Responsibilities

### API Gateway Excellence
- **HTTP Performance**: Ultra-fast request routing and response handling
- **GraphQL Implementation**: Flexible data querying with real-time subscriptions
- **Service Integration**: Orchestrate communication between Rust, Python, and Flutter
- **WebSocket Services**: Real-time features for live updates and notifications
- **Middleware Pipeline**: Authentication, rate limiting, logging, and monitoring

### Your Work Environment
```bash
# Your dedicated workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/go-api worktree-chatgpt
cd worktree-chatgpt/backend/go
```

## 🐹 Go Development Standards

### Project Structure (YOU MUST CREATE)
```
backend/go/
├── go.mod                      # Go module file
├── go.sum                      # Dependency checksums
├── Dockerfile                  # Container build
├── .air.toml                   # Hot reload config
├── cmd/
│   └── api/
│       └── main.go             # Application entry point
├── internal/                   # Private application code
│   ├── api/                    # HTTP handlers and routes
│   │   ├── handlers/           # Request handlers
│   │   ├── middleware/         # HTTP middleware
│   │   └── routes/             # Route definitions
│   ├── auth/                   # Authentication logic
│   │   ├── jwt.go
│   │   ├── middleware.go
│   │   └── service.go
│   ├── graphql/                # GraphQL implementation
│   │   ├── generated/          # Generated GraphQL code
│   │   ├── resolvers/          # GraphQL resolvers
│   │   ├── schema/             # GraphQL schemas
│   │   └── server.go
│   ├── websocket/              # WebSocket implementation
│   │   ├── hub.go
│   │   ├── client.go
│   │   └── handlers.go
│   ├── services/               # Business logic
│   │   ├── analytics.go
│   │   ├── orders.go
│   │   ├── products.go
│   │   └── customers.go
│   ├── models/                 # Data models
│   │   ├── requests.go
│   │   ├── responses.go
│   │   └── entities.go
│   └── config/                 # Configuration
│       ├── config.go
│       └── database.go
├── pkg/                        # Public library code
│   ├── errors/                 # Error handling
│   ├── logger/                 # Logging utilities
│   ├── metrics/                # Prometheus metrics
│   └── utils/                  # Shared utilities
├── tests/                      # Test files
│   ├── integration/
│   ├── unit/
│   └── fixtures/
├── docs/                       # API documentation
├── scripts/                    # Build and deployment scripts
└── tools/                      # Development tools
```

### Required Dependencies (go.mod)
```go
module github.com/olympuscloud/olympus-gcp

go 1.21

require (
    // Web Framework
    github.com/gin-gonic/gin v1.9.1
    github.com/gin-contrib/cors v1.5.0
    github.com/gin-contrib/gzip v1.0.0
    
    // GraphQL
    github.com/99designs/gqlgen v0.17.41
    github.com/vektah/gqlparser/v2 v2.5.10
    
    // WebSocket
    github.com/gorilla/websocket v1.5.1
    
    // Authentication & Security
    github.com/golang-jwt/jwt/v5 v5.2.0
    github.com/casbin/casbin/v2 v2.81.0
    golang.org/x/crypto v0.17.0
    
    // Database
    github.com/jmoiron/sqlx v1.3.5
    github.com/lib/pq v1.10.9
    github.com/golang-migrate/migrate/v4 v4.17.0
    
    // Redis
    github.com/redis/go-redis/v9 v9.3.1
    
    // HTTP Client
    github.com/go-resty/resty/v2 v2.11.0
    
    // Configuration
    github.com/spf13/viper v1.18.2
    github.com/joho/godotenv v1.5.1
    
    // Monitoring & Logging
    github.com/prometheus/client_golang v1.18.0
    github.com/sirupsen/logrus v1.9.3
    go.opentelemetry.io/otel v1.21.0
    
    // Validation
    github.com/go-playground/validator/v10 v10.16.0
    
    // Utilities
    github.com/google/uuid v1.5.0
    github.com/shopspring/decimal v1.3.1
    
    // Testing
    github.com/stretchr/testify v1.8.4
    github.com/testcontainers/testcontainers-go v0.26.0
    github.com/golang/mock v1.6.0
)

require (
    // Development tools
    github.com/cosmtrek/air v1.49.0
    github.com/golangci/golangci-lint v1.55.2
    github.com/swaggo/swag v1.16.2
    github.com/swaggo/gin-swagger v1.6.0
    github.com/swaggo/files v1.0.1
)
```

## 🚀 High-Performance API Gateway

### Main Application Setup
```go
// cmd/api/main.go
package main

import (
    "context"
    "fmt"
    "net/http"
    "os"
    "os/signal"
    "syscall"
    "time"

    "github.com/gin-gonic/gin"
    "github.com/olympuscloud/olympus-gcp/internal/api/routes"
    "github.com/olympuscloud/olympus-gcp/internal/config"
    "github.com/olympuscloud/olympus-gcp/pkg/logger"
    "github.com/prometheus/client_golang/prometheus/promhttp"
)

func main() {
    // Initialize configuration
    cfg, err := config.Load()
    if err != nil {
        logger.Fatal("Failed to load configuration", "error", err)
    }

    // Initialize logger
    logger.Init(cfg.LogLevel, cfg.Environment)

    // Set Gin mode
    if cfg.Environment == "production" {
        gin.SetMode(gin.ReleaseMode)
    }

    // Create Gin router
    router := gin.New()

    // Add middleware
    router.Use(gin.Recovery())
    router.Use(logger.GinLogger())

    // Health check endpoint
    router.GET("/health", healthCheckHandler)
    router.GET("/metrics", gin.WrapH(promhttp.Handler()))

    // Setup routes
    routes.SetupRoutes(router, cfg)

    // Create HTTP server
    server := &http.Server{
        Addr:         fmt.Sprintf(":%d", cfg.Port),
        Handler:      router,
        ReadTimeout:  30 * time.Second,
        WriteTimeout: 30 * time.Second,
        IdleTimeout:  120 * time.Second,
    }

    // Start server in goroutine
    go func() {
        logger.Info("Starting server", "port", cfg.Port)
        if err := server.ListenAndServe(); err != nil && err != http.ErrServerClosed {
            logger.Fatal("Failed to start server", "error", err)
        }
    }()

    // Wait for interrupt signal to gracefully shutdown
    quit := make(chan os.Signal, 1)
    signal.Notify(quit, syscall.SIGINT, syscall.SIGTERM)
    <-quit

    logger.Info("Shutting down server...")

    // Graceful shutdown with timeout
    ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
    defer cancel()

    if err := server.Shutdown(ctx); err != nil {
        logger.Fatal("Server forced to shutdown", "error", err)
    }

    logger.Info("Server exited")
}

func healthCheckHandler(c *gin.Context) {
    c.JSON(http.StatusOK, gin.H{
        "status":    "healthy",
        "timestamp": time.Now().UTC(),
        "version":   os.Getenv("VERSION"),
    })
}
```

### Authentication Middleware
```go
// internal/auth/middleware.go
package auth

import (
    "net/http"
    "strings"

    "github.com/gin-gonic/gin"
    "github.com/golang-jwt/jwt/v5"
    "github.com/olympuscloud/olympus-gcp/internal/models"
    "github.com/olympuscloud/olympus-gcp/pkg/errors"
    "github.com/olympuscloud/olympus-gcp/pkg/logger"
)

type AuthMiddleware struct {
    jwtSecret []byte
    authService *Service
}

func NewAuthMiddleware(jwtSecret []byte, authService *Service) *AuthMiddleware {
    return &AuthMiddleware{
        jwtSecret:   jwtSecret,
        authService: authService,
    }
}

func (am *AuthMiddleware) RequireAuth() gin.HandlerFunc {
    return func(c *gin.Context) {
        // Extract token from Authorization header
        authHeader := c.GetHeader("Authorization")
        if authHeader == "" {
            c.JSON(http.StatusUnauthorized, errors.NewAPIError(
                "missing_authorization",
                "Authorization header is required",
                nil,
            ))
            c.Abort()
            return
        }

        // Check Bearer prefix
        tokenString := strings.TrimPrefix(authHeader, "Bearer ")
        if tokenString == authHeader {
            c.JSON(http.StatusUnauthorized, errors.NewAPIError(
                "invalid_authorization_format",
                "Authorization header must start with 'Bearer '",
                nil,
            ))
            c.Abort()
            return
        }

        // Parse and validate JWT token
        token, err := jwt.ParseWithClaims(tokenString, &models.JWTClaims{}, func(token *jwt.Token) (interface{}, error) {
            if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
                return nil, errors.New("invalid signing method")
            }
            return am.jwtSecret, nil
        })

        if err != nil || !token.Valid {
            logger.Warn("Invalid JWT token", "error", err)
            c.JSON(http.StatusUnauthorized, errors.NewAPIError(
                "invalid_token",
                "Token is invalid or expired",
                nil,
            ))
            c.Abort()
            return
        }

        // Extract claims
        claims, ok := token.Claims.(*models.JWTClaims)
        if !ok {
            c.JSON(http.StatusUnauthorized, errors.NewAPIError(
                "invalid_claims",
                "Token claims are invalid",
                nil,
            ))
            c.Abort()
            return
        }

        // Validate user still exists and is active
        user, err := am.authService.GetUserByID(c.Request.Context(), claims.UserID)
        if err != nil {
            logger.Error("Failed to get user from token", "user_id", claims.UserID, "error", err)
            c.JSON(http.StatusUnauthorized, errors.NewAPIError(
                "user_not_found",
                "User associated with token not found",
                nil,
            ))
            c.Abort()
            return
        }

        if !user.IsActive {
            c.JSON(http.StatusUnauthorized, errors.NewAPIError(
                "user_inactive",
                "User account is inactive",
                nil,
            ))
            c.Abort()
            return
        }

        // Set user context
        c.Set("user_id", claims.UserID)
        c.Set("tenant_id", claims.TenantID)
        c.Set("user_roles", claims.Roles)
        c.Set("user", user)

        c.Next()
    }
}

func (am *AuthMiddleware) RequireRole(requiredRole string) gin.HandlerFunc {
    return func(c *gin.Context) {
        userRoles, exists := c.Get("user_roles")
        if !exists {
            c.JSON(http.StatusForbidden, errors.NewAPIError(
                "no_roles",
                "User roles not found in context",
                nil,
            ))
            c.Abort()
            return
        }

        roles, ok := userRoles.([]string)
        if !ok {
            c.JSON(http.StatusForbidden, errors.NewAPIError(
                "invalid_roles",
                "Invalid user roles format",
                nil,
            ))
            c.Abort()
            return
        }

        // Check if user has required role
        hasRole := false
        for _, role := range roles {
            if role == requiredRole || role == "super_admin" {
                hasRole = true
                break
            }
        }

        if !hasRole {
            c.JSON(http.StatusForbidden, errors.NewAPIError(
                "insufficient_permissions",
                fmt.Sprintf("Role '%s' is required", requiredRole),
                nil,
            ))
            c.Abort()
            return
        }

        c.Next()
    }
}
```

### GraphQL Implementation
```go
// internal/graphql/server.go
package graphql

import (
    "context"
    "time"

    "github.com/99designs/gqlgen/graphql/handler"
    "github.com/99designs/gqlgen/graphql/handler/extension"
    "github.com/99designs/gqlgen/graphql/handler/lru"
    "github.com/99designs/gqlgen/graphql/handler/transport"
    "github.com/99designs/gqlgen/graphql/playground"
    "github.com/gin-gonic/gin"
    
    "github.com/olympuscloud/olympus-gcp/internal/graphql/generated"
    "github.com/olympuscloud/olympus-gcp/internal/graphql/resolvers"
    "github.com/olympuscloud/olympus-gcp/internal/services"
)

func NewGraphQLHandler(services *services.Container) gin.HandlerFunc {
    resolver := &resolvers.Resolver{
        OrderService:    services.OrderService,
        ProductService:  services.ProductService,
        CustomerService: services.CustomerService,
        AnalyticsService: services.AnalyticsService,
    }

    config := generated.Config{Resolvers: resolver}
    
    srv := handler.New(generated.NewExecutableSchema(config))

    // Add transports
    srv.AddTransport(transport.Websocket{
        KeepAlivePingInterval: 10 * time.Second,
    })
    srv.AddTransport(transport.Options{})
    srv.AddTransport(transport.GET{})
    srv.AddTransport(transport.POST{})
    srv.AddTransport(transport.MultipartForm{})

    // Add extensions
    srv.SetQueryCache(lru.New(1000))
    srv.Use(extension.Introspection{})
    srv.Use(extension.AutomaticPersistedQuery{
        Cache: lru.New(100),
    })

    // Add complexity limit
    srv.Use(&extension.ComplexityLimit{
        Func: func(ctx context.Context, rc *graphql.RequestContext) int {
            return 300  // Maximum query complexity
        },
    })

    return gin.WrapH(srv)
}

func PlaygroundHandler() gin.HandlerFunc {
    h := playground.Handler("GraphQL Playground", "/graphql")
    return gin.WrapH(h)
}

// internal/graphql/resolvers/order.go
package resolvers

import (
    "context"
    "fmt"

    "github.com/olympuscloud/olympus-gcp/internal/graphql/generated"
    "github.com/olympuscloud/olympus-gcp/internal/models"
)

func (r *queryResolver) Orders(ctx context.Context, filter *generated.OrderFilter, pagination *generated.Pagination) (*generated.OrderConnection, error) {
    // Extract tenant from context
    tenantID, err := getTenantIDFromContext(ctx)
    if err != nil {
        return nil, err
    }

    // Convert GraphQL filter to service filter
    serviceFilter := &models.OrderFilter{
        TenantID: tenantID,
        Status:   filter.Status,
        DateFrom: filter.DateFrom,
        DateTo:   filter.DateTo,
    }

    // Set pagination defaults
    limit := 20
    offset := 0
    if pagination != nil {
        if pagination.Limit != nil {
            limit = *pagination.Limit
        }
        if pagination.Offset != nil {
            offset = *pagination.Offset
        }
    }

    // Get orders from service
    orders, total, err := r.OrderService.GetOrders(ctx, serviceFilter, limit, offset)
    if err != nil {
        return nil, fmt.Errorf("failed to get orders: %w", err)
    }

    // Convert to GraphQL types
    edges := make([]*generated.OrderEdge, len(orders))
    for i, order := range orders {
        edges[i] = &generated.OrderEdge{
            Node:   convertOrderToGraphQL(order),
            Cursor: encodeCursor(order.ID),
        }
    }

    return &generated.OrderConnection{
        Edges: edges,
        PageInfo: &generated.PageInfo{
            HasNextPage:     offset+limit < total,
            HasPreviousPage: offset > 0,
            StartCursor:     edges[0].Cursor,
            EndCursor:       edges[len(edges)-1].Cursor,
        },
        TotalCount: total,
    }, nil
}

func (r *mutationResolver) CreateOrder(ctx context.Context, input generated.CreateOrderInput) (*generated.Order, error) {
    tenantID, err := getTenantIDFromContext(ctx)
    if err != nil {
        return nil, err
    }

    userID, err := getUserIDFromContext(ctx)
    if err != nil {
        return nil, err
    }

    // Convert GraphQL input to service model
    serviceInput := &models.CreateOrderInput{
        TenantID:     tenantID,
        CustomerID:   input.CustomerID,
        LocationID:   input.LocationID,
        Items:        convertOrderItemsFromGraphQL(input.Items),
        Notes:        input.Notes,
        CreatedBy:    userID,
    }

    // Create order
    order, err := r.OrderService.CreateOrder(ctx, serviceInput)
    if err != nil {
        return nil, fmt.Errorf("failed to create order: %w", err)
    }

    return convertOrderToGraphQL(order), nil
}

// Subscription resolver for real-time updates
func (r *subscriptionResolver) OrderUpdates(ctx context.Context, tenantID string) (<-chan *generated.Order, error) {
    // Validate tenant access
    contextTenantID, err := getTenantIDFromContext(ctx)
    if err != nil {
        return nil, err
    }

    if contextTenantID != tenantID {
        return nil, fmt.Errorf("unauthorized to subscribe to tenant %s", tenantID)
    }

    // Create channel for order updates
    orderChan := make(chan *generated.Order)

    // Subscribe to Redis events
    go func() {
        defer close(orderChan)
        
        pubsub := r.RedisClient.Subscribe(ctx, fmt.Sprintf("orders:%s", tenantID))
        defer pubsub.Close()

        for {
            select {
            case <-ctx.Done():
                return
            case msg := <-pubsub.Channel():
                // Parse order from Redis message
                var order models.Order
                if err := json.Unmarshal([]byte(msg.Payload), &order); err != nil {
                    continue
                }
                
                // Convert and send to GraphQL subscriber
                orderChan <- convertOrderToGraphQL(&order)
            }
        }
    }()

    return orderChan, nil
}
```

### WebSocket Real-Time Services
```go
// internal/websocket/hub.go
package websocket

import (
    "context"
    "encoding/json"
    "sync"
    "time"

    "github.com/gorilla/websocket"
    "github.com/olympuscloud/olympus-gcp/pkg/logger"
)

type Hub struct {
    clients    map[*Client]bool
    broadcast  chan []byte
    register   chan *Client
    unregister chan *Client
    mutex      sync.RWMutex
}

type Client struct {
    hub      *Hub
    conn     *websocket.Conn
    send     chan []byte
    userID   string
    tenantID string
    rooms    map[string]bool
}

type Message struct {
    Type      string      `json:"type"`
    Room      string      `json:"room,omitempty"`
    Data      interface{} `json:"data"`
    Timestamp time.Time   `json:"timestamp"`
}

func NewHub() *Hub {
    return &Hub{
        clients:    make(map[*Client]bool),
        broadcast:  make(chan []byte),
        register:   make(chan *Client),
        unregister: make(chan *Client),
    }
}

func (h *Hub) Run(ctx context.Context) {
    for {
        select {
        case <-ctx.Done():
            return
            
        case client := <-h.register:
            h.mutex.Lock()
            h.clients[client] = true
            h.mutex.Unlock()
            
            logger.Info("Client connected", 
                "user_id", client.userID, 
                "tenant_id", client.tenantID,
                "total_clients", len(h.clients))

        case client := <-h.unregister:
            h.mutex.Lock()
            if _, ok := h.clients[client]; ok {
                delete(h.clients, client)
                close(client.send)
            }
            h.mutex.Unlock()
            
            logger.Info("Client disconnected", 
                "user_id", client.userID,
                "total_clients", len(h.clients))

        case message := <-h.broadcast:
            h.mutex.RLock()
            for client := range h.clients {
                select {
                case client.send <- message:
                default:
                    delete(h.clients, client)
                    close(client.send)
                }
            }
            h.mutex.RUnlock()
        }
    }
}

func (h *Hub) BroadcastToRoom(room string, message interface{}) {
    msg := Message{
        Type:      "room_message",
        Room:      room,
        Data:      message,
        Timestamp: time.Now(),
    }

    data, err := json.Marshal(msg)
    if err != nil {
        logger.Error("Failed to marshal WebSocket message", "error", err)
        return
    }

    h.mutex.RLock()
    for client := range h.clients {
        if client.rooms[room] {
            select {
            case client.send <- data:
            default:
                delete(h.clients, client)
                close(client.send)
            }
        }
    }
    h.mutex.RUnlock()
}

func (h *Hub) BroadcastToTenant(tenantID string, message interface{}) {
    msg := Message{
        Type:      "tenant_message",
        Data:      message,
        Timestamp: time.Now(),
    }

    data, err := json.Marshal(msg)
    if err != nil {
        logger.Error("Failed to marshal WebSocket message", "error", err)
        return
    }

    h.mutex.RLock()
    for client := range h.clients {
        if client.tenantID == tenantID {
            select {
            case client.send <- data:
            default:
                delete(h.clients, client)
                close(client.send)
            }
        }
    }
    h.mutex.RUnlock()
}

// Client methods
func (c *Client) readPump() {
    defer func() {
        c.hub.unregister <- c
        c.conn.Close()
    }()

    c.conn.SetReadLimit(512)
    c.conn.SetReadDeadline(time.Now().Add(60 * time.Second))
    c.conn.SetPongHandler(func(string) error {
        c.conn.SetReadDeadline(time.Now().Add(60 * time.Second))
        return nil
    })

    for {
        _, message, err := c.conn.ReadMessage()
        if err != nil {
            if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
                logger.Error("WebSocket error", "error", err)
            }
            break
        }

        // Handle client messages (join rooms, etc.)
        var msg Message
        if err := json.Unmarshal(message, &msg); err == nil {
            c.handleMessage(&msg)
        }
    }
}

func (c *Client) writePump() {
    ticker := time.NewTicker(54 * time.Second)
    defer func() {
        ticker.Stop()
        c.conn.Close()
    }()

    for {
        select {
        case message, ok := <-c.send:
            c.conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
            if !ok {
                c.conn.WriteMessage(websocket.CloseMessage, []byte{})
                return
            }

            if err := c.conn.WriteMessage(websocket.TextMessage, message); err != nil {
                return
            }

        case <-ticker.C:
            c.conn.SetWriteDeadline(time.Now().Add(10 * time.Second))
            if err := c.conn.WriteMessage(websocket.PingMessage, nil); err != nil {
                return
            }
        }
    }
}

func (c *Client) handleMessage(msg *Message) {
    switch msg.Type {
    case "join_room":
        if room, ok := msg.Data.(string); ok {
            c.rooms[room] = true
            logger.Info("Client joined room", "user_id", c.userID, "room", room)
        }
    case "leave_room":
        if room, ok := msg.Data.(string); ok {
            delete(c.rooms, room)
            logger.Info("Client left room", "user_id", c.userID, "room", room)
        }
    }
}
```

## 📋 Your Daily Development Workflow

### Morning Routine (MANDATORY)
```bash
# 1. Sync with main and other agents
cd worktree-chatgpt
git pull origin main
git merge main

# 2. Check coordination docs
cat docs/daily-status.md
cat docs/integration-points.md

# 3. Update your status in docs/daily-status.md

# 4. Start development environment with hot reload
cd backend/go
make dev-go
# This runs: air (hot reload) or go run cmd/api/main.go
```

### Development Cycle
```bash
# Run tests frequently
go test ./... -v -race

# Check code quality
golangci-lint run
go vet ./...

# Format code
go fmt ./...

# Generate GraphQL code (when schemas change)
go generate ./...

# Security scanning
gosec ./...

# Commit frequently (every 1-2 hours)
git add -p
git commit -m "chatgpt(api): implement GraphQL order subscriptions"
```

### Evening Integration
```bash
# Build for production to ensure compatibility
go build -o bin/api cmd/api/main.go

# Run integration tests
go test ./tests/integration/... -v

# Push your work
git push origin feat/go-api

# Update status in docs/daily-status.md
```

## 🎯 Week 1 Implementation Priorities

### Day 1: API Foundation
```bash
# 1. Initialize Go module and project structure
go mod init github.com/olympuscloud/olympus-gcp

# 2. Setup basic Gin server with middleware
# 3. Implement configuration management
# 4. Add health check and metrics endpoints
```

### Day 2: Authentication Integration
```go
// Implement these in order:
// 1. JWT middleware (auth/middleware.go)
// 2. Auth service integration with Claude's Rust service
// 3. Role-based access control
// 4. Session management
```

### Day 3: GraphQL Implementation
```bash
# 1. Setup gqlgen and generate schemas
# 2. Implement resolvers for orders and products
# 3. Add real-time subscriptions
# 4. Test GraphQL playground
```

### Day 4: WebSocket & Integration
```bash
# 1. Implement WebSocket hub and client management
# 2. Real-time order updates
# 3. Integration with Python analytics service
# 4. Performance testing and optimization
```

## 🔗 Critical Integration Points

### Service-to-Service Communication
```go
// internal/services/rust_client.go
package services

import (
    "context"
    "encoding/json"
    "fmt"
    "net/http"
    
    "github.com/go-resty/resty/v2"
    "github.com/olympuscloud/olympus-gcp/internal/models"
)

type RustAuthClient struct {
    client  *resty.Client
    baseURL string
}

func NewRustAuthClient(baseURL string) *RustAuthClient {
    client := resty.New()
    client.SetBaseURL(baseURL)
    client.SetHeader("Content-Type", "application/json")
    
    return &RustAuthClient{
        client:  client,
        baseURL: baseURL,
    }
}

func (c *RustAuthClient) ValidateToken(ctx context.Context, token string) (*models.User, error) {
    var user models.User
    
    resp, err := c.client.R().
        SetContext(ctx).
        SetAuthToken(token).
        SetResult(&user).
        Get("/auth/me")
    
    if err != nil {
        return nil, fmt.Errorf("failed to validate token: %w", err)
    }
    
    if resp.StatusCode() != http.StatusOK {
        return nil, fmt.Errorf("invalid token: status %d", resp.StatusCode())
    }
    
    return &user, nil
}

func (c *RustAuthClient) CreateUser(ctx context.Context, req *models.CreateUserRequest) (*models.User, error) {
    var user models.User
    
    resp, err := c.client.R().
        SetContext(ctx).
        SetBody(req).
        SetResult(&user).
        Post("/auth/users")
    
    if err != nil {
        return nil, fmt.Errorf("failed to create user: %w", err)
    }
    
    if resp.StatusCode() != http.StatusCreated {
        return nil, fmt.Errorf("failed to create user: status %d", resp.StatusCode())
    }
    
    return &user, nil
}

// internal/services/python_client.go
type PythonAnalyticsClient struct {
    client  *resty.Client
    baseURL string
}

func NewPythonAnalyticsClient(baseURL string) *PythonAnalyticsClient {
    client := resty.New()
    client.SetBaseURL(baseURL)
    client.SetHeader("Content-Type", "application/json")
    
    return &PythonAnalyticsClient{
        client:  client,
        baseURL: baseURL,
    }
}

func (c *PythonAnalyticsClient) GetDashboardAnalytics(ctx context.Context, tenantID string, days int) (*models.AnalyticsResponse, error) {
    var analytics models.AnalyticsResponse
    
    resp, err := c.client.R().
        SetContext(ctx).
        SetQueryParams(map[string]string{
            "tenant_id": tenantID,
            "days":      fmt.Sprintf("%d", days),
        }).
        SetResult(&analytics).
        Get("/analytics/dashboard")
    
    if err != nil {
        return nil, fmt.Errorf("failed to get analytics: %w", err)
    }
    
    if resp.StatusCode() != http.StatusOK {
        return nil, fmt.Errorf("analytics request failed: status %d", resp.StatusCode())
    }
    
    return &analytics, nil
}

func (c *PythonAnalyticsClient) ProcessNaturalLanguageQuery(ctx context.Context, query string, tenantID string) (*models.NLQueryResponse, error) {
    var response models.NLQueryResponse
    
    reqBody := map[string]string{
        "query":     query,
        "tenant_id": tenantID,
    }
    
    resp, err := c.client.R().
        SetContext(ctx).
        SetBody(reqBody).
        SetResult(&response).
        Post("/analytics/query")
    
    if err != nil {
        return nil, fmt.Errorf("failed to process NL query: %w", err)
    }
    
    if resp.StatusCode() != http.StatusOK {
        return nil, fmt.Errorf("NL query failed: status %d", resp.StatusCode())
    }
    
    return &response, nil
}
```

### Rate Limiting and Security
```go
// internal/api/middleware/rate_limit.go
package middleware

import (
    "net/http"
    "strconv"
    "time"

    "github.com/gin-gonic/gin"
    "github.com/redis/go-redis/v9"
    "github.com/olympuscloud/olympus-gcp/pkg/errors"
)

type RateLimiter struct {
    redisClient *redis.Client
    requests    int
    window      time.Duration
}

func NewRateLimiter(redisClient *redis.Client, requests int, window time.Duration) *RateLimiter {
    return &RateLimiter{
        redisClient: redisClient,
        requests:    requests,
        window:      window,
    }
}

func (rl *RateLimiter) Middleware() gin.HandlerFunc {
    return func(c *gin.Context) {
        // Use IP address as key (could also use user ID for authenticated requests)
        key := "rate_limit:" + c.ClientIP()
        
        // Get current request count
        current, err := rl.redisClient.Get(c.Request.Context(), key).Int()
        if err != nil && err != redis.Nil {
            // If Redis is down, allow the request
            c.Next()
            return
        }
        
        if current >= rl.requests {
            c.Header("X-RateLimit-Limit", strconv.Itoa(rl.requests))
            c.Header("X-RateLimit-Remaining", "0")
            c.Header("X-RateLimit-Reset", strconv.FormatInt(time.Now().Add(rl.window).Unix(), 10))
            
            c.JSON(http.StatusTooManyRequests, errors.NewAPIError(
                "rate_limit_exceeded",
                "Too many requests",
                map[string]interface{}{
                    "limit":  rl.requests,
                    "window": rl.window.String(),
                },
            ))
            c.Abort()
            return
        }
        
        // Increment counter
        pipe := rl.redisClient.Pipeline()
        pipe.Incr(c.Request.Context(), key)
        pipe.Expire(c.Request.Context(), key, rl.window)
        pipe.Exec(c.Request.Context())
        
        // Set headers
        c.Header("X-RateLimit-Limit", strconv.Itoa(rl.requests))
        c.Header("X-RateLimit-Remaining", strconv.Itoa(rl.requests-current-1))
        c.Header("X-RateLimit-Reset", strconv.FormatInt(time.Now().Add(rl.window).Unix(), 10))
        
        c.Next()
    }
}
```

## 🧪 Testing Standards (MANDATORY)

### Unit Testing
```go
// tests/unit/auth_test.go
package unit

import (
    "net/http"
    "net/http/httptest"
    "testing"
    "time"

    "github.com/gin-gonic/gin"
    "github.com/golang-jwt/jwt/v5"
    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/mock"
    
    "github.com/olympuscloud/olympus-gcp/internal/auth"
    "github.com/olympuscloud/olympus-gcp/internal/models"
    "github.com/olympuscloud/olympus-gcp/tests/mocks"
)

func TestAuthMiddleware_RequireAuth_Success(t *testing.T) {
    // Create mock auth service
    mockAuthService := &mocks.AuthService{}
    user := &models.User{
        ID:       "user-123",
        Email:    "test@example.com",
        IsActive: true,
    }
    mockAuthService.On("GetUserByID", mock.Anything, "user-123").Return(user, nil)

    // Create middleware
    jwtSecret := []byte("test-secret")
    middleware := auth.NewAuthMiddleware(jwtSecret, mockAuthService)

    // Create test JWT token
    claims := &models.JWTClaims{
        UserID:   "user-123",
        TenantID: "tenant-123",
        Roles:    []string{"user"},
        RegisteredClaims: jwt.RegisteredClaims{
            ExpiresAt: jwt.NewNumericDate(time.Now().Add(time.Hour)),
            IssuedAt:  jwt.NewNumericDate(time.Now()),
        },
    }

    token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
    tokenString, err := token.SignedString(jwtSecret)
    assert.NoError(t, err)

    // Setup Gin test context
    gin.SetMode(gin.TestMode)
    router := gin.New()
    router.Use(middleware.RequireAuth())
    router.GET("/test", func(c *gin.Context) {
        c.JSON(http.StatusOK, gin.H{"message": "success"})
    })

    // Make request with valid token
    req := httptest.NewRequest("GET", "/test", nil)
    req.Header.Set("Authorization", "Bearer "+tokenString)
    w := httptest.NewRecorder()

    router.ServeHTTP(w, req)

    assert.Equal(t, http.StatusOK, w.Code)
    mockAuthService.AssertExpectations(t)
}

func TestAuthMiddleware_RequireAuth_InvalidToken(t *testing.T) {
    mockAuthService := &mocks.AuthService{}
    jwtSecret := []byte("test-secret")
    middleware := auth.NewAuthMiddleware(jwtSecret, mockAuthService)

    gin.SetMode(gin.TestMode)
    router := gin.New()
    router.Use(middleware.RequireAuth())
    router.GET("/test", func(c *gin.Context) {
        c.JSON(http.StatusOK, gin.H{"message": "success"})
    })

    // Make request with invalid token
    req := httptest.NewRequest("GET", "/test", nil)
    req.Header.Set("Authorization", "Bearer invalid-token")
    w := httptest.NewRecorder()

    router.ServeHTTP(w, req)

    assert.Equal(t, http.StatusUnauthorized, w.Code)
}

// Integration test with real database
func TestOrderAPI_Integration(t *testing.T) {
    // Setup test database
    db := setupTestDB(t)
    defer db.Close()

    // Setup test server
    server := setupTestServer(db)
    defer server.Close()

    // Create test user and tenant
    user := createTestUser(t, db)
    token := generateTestToken(user)

    // Test create order
    orderData := map[string]interface{}{
        "customer_id": "customer-123",
        "items": []map[string]interface{}{
            {
                "product_id": "product-123",
                "quantity":   2,
                "unit_price": 10.99,
            },
        },
    }

    body, _ := json.Marshal(orderData)
    req := httptest.NewRequest("POST", "/api/v1/orders", bytes.NewBuffer(body))
    req.Header.Set("Authorization", "Bearer "+token)
    req.Header.Set("Content-Type", "application/json")

    w := httptest.NewRecorder()
    server.Handler.ServeHTTP(w, req)

    assert.Equal(t, http.StatusCreated, w.Code)

    var response map[string]interface{}
    err := json.Unmarshal(w.Body.Bytes(), &response)
    assert.NoError(t, err)
    assert.NotEmpty(t, response["id"])
}
```

### Load Testing
```go
// tests/load/load_test.go
package load

import (
    "context"
    "sync"
    "testing"
    "time"

    "github.com/stretchr/testify/assert"
)

func TestAPIPerformance_ConcurrentRequests(t *testing.T) {
    if testing.Short() {
        t.Skip("Skipping load test in short mode")
    }

    // Test parameters
    numClients := 100
    requestsPerClient := 10
    timeout := 30 * time.Second

    // Setup test server
    server := setupTestServer(nil)
    defer server.Close()

    // Create test token
    token := generateTestToken(createTestUser(t, nil))

    ctx, cancel := context.WithTimeout(context.Background(), timeout)
    defer cancel()

    var wg sync.WaitGroup
    results := make(chan time.Duration, numClients*requestsPerClient)
    errors := make(chan error, numClients*requestsPerClient)

    // Launch concurrent clients
    for i := 0; i < numClients; i++ {
        wg.Add(1)
        go func() {
            defer wg.Done()
            
            for j := 0; j < requestsPerClient; j++ {
                start := time.Now()
                
                req := httptest.NewRequest("GET", "/api/v1/orders", nil)
                req.Header.Set("Authorization", "Bearer "+token)
                w := httptest.NewRecorder()
                
                server.Handler.ServeHTTP(w, req)
                
                duration := time.Since(start)
                
                if w.Code == http.StatusOK {
                    results <- duration
                } else {
                    errors <- fmt.Errorf("request failed with status %d", w.Code)
                }
            }
        }()
    }

    // Wait for all requests to complete
    wg.Wait()
    close(results)
    close(errors)

    // Analyze results
    var durations []time.Duration
    for duration := range results {
        durations = append(durations, duration)
    }

    var errorCount int
    for range errors {
        errorCount++
    }

    // Calculate statistics
    if len(durations) > 0 {
        sort.Slice(durations, func(i, j int) bool {
            return durations[i] < durations[j]
        })

        p50 := durations[len(durations)*50/100]
        p95 := durations[len(durations)*95/100]
        p99 := durations[len(durations)*99/100]

        t.Logf("Performance Results:")
        t.Logf("Total requests: %d", len(durations))
        t.Logf("Errors: %d", errorCount)
        t.Logf("P50: %v", p50)
        t.Logf("P95: %v", p95)
        t.Logf("P99: %v", p99)

        // Assert performance targets
        assert.Less(t, p99, 100*time.Millisecond, "P99 should be less than 100ms")
        assert.Less(t, float64(errorCount)/float64(len(durations)), 0.01, "Error rate should be less than 1%")
    }
}
```

## 🏁 Success Criteria

### Week 1 Deliverables
- [ ] Go API gateway with Gin framework
- [ ] JWT authentication middleware integrated with Rust auth service
- [ ] GraphQL implementation with real-time subscriptions
- [ ] WebSocket services for live updates
- [ ] Rate limiting and security middleware
- [ ] Service-to-service communication with Rust and Python
- [ ] Comprehensive error handling and logging
- [ ] Prometheus metrics and health checks
- [ ] Integration tests with >80% coverage
- [ ] Load testing demonstrating performance targets

### Quality Gates
- [ ] `go test ./...` - All tests pass
- [ ] `golangci-lint run` - No issues
- [ ] `go vet ./...` - No warnings
- [ ] `gosec ./...` - No security vulnerabilities
- [ ] Load testing: P99 < 100ms for simple endpoints
- [ ] Concurrent users: >1000 without degradation

### Integration Requirements
- [ ] Authentication flow working with Claude's Rust service
- [ ] Real-time data from OpenAI Codex's Python analytics
- [ ] API consumed successfully by GitHub Copilot's Flutter app
- [ ] WebSocket connections stable under load
- [ ] GraphQL queries optimized and performant

**Remember**: You are the central nervous system of the entire platform. Every request flows through you. Performance, reliability, and seamless integration are your primary concerns.

**Your motto**: *"Fast, reliable, seamlessly connected."*