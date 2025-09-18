package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"
	"os/signal"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
	"github.com/redis/go-redis/v9"
)

type Config struct {
	Port            string
	AuthServiceURL  string
	PythonServiceURL string
	RedisURL        string
	DatabaseURL     string
	JWTSecret       string
	Environment     string
}

func loadConfig() *Config {
	// Load .env file
	godotenv.Load()

	return &Config{
		Port:            getEnv("PORT", "8080"),
		AuthServiceURL:  getEnv("AUTH_SERVICE_URL", "http://localhost:8000"),
		PythonServiceURL: getEnv("PYTHON_SERVICE_URL", "http://localhost:8001"),
		RedisURL:        getEnv("REDIS_URL", "redis://localhost:6379"),
		DatabaseURL:     getEnv("DATABASE_URL", "postgresql://olympus:devpassword@localhost:5432/olympus"),
		JWTSecret:       getEnv("JWT_SECRET", "development-secret-key"),
		Environment:     getEnv("ENVIRONMENT", "development"),
	}
}

func getEnv(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

func main() {
	config := loadConfig()

	// Set Gin mode
	if config.Environment == "production" {
		gin.SetMode(gin.ReleaseMode)
	}

	// Create router
	router := gin.New()

	// Middleware
	router.Use(gin.Logger())
	router.Use(gin.Recovery())
	router.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"http://localhost:3000", "http://localhost:8080"},
		AllowMethods:     []string{"GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Type", "Authorization"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: true,
		MaxAge:           12 * time.Hour,
	}))

	// Initialize Redis client
	opt, _ := redis.ParseURL(config.RedisURL)
	redisClient := redis.NewClient(opt)
	ctx := context.Background()
	if err := redisClient.Ping(ctx).Err(); err != nil {
		log.Printf("Warning: Could not connect to Redis: %v", err)
	}

	// Routes
	setupRoutes(router, config, redisClient)

	// Start server
	srv := &http.Server{
		Addr:    ":" + config.Port,
		Handler: router,
	}

	// Graceful shutdown
	go func() {
		if err := srv.ListenAndServe(); err != nil && err != http.ErrServerClosed {
			log.Fatalf("Failed to start server: %v", err)
		}
	}()

	log.Printf("🚀 API Gateway starting on port %s", config.Port)

	// Wait for interrupt signal
	quit := make(chan os.Signal, 1)
	signal.Notify(quit, os.Interrupt)
	<-quit

	log.Println("Shutting down server...")

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := srv.Shutdown(ctx); err != nil {
		log.Fatal("Server forced to shutdown:", err)
	}

	log.Println("Server exited")
}

func setupRoutes(router *gin.Engine, config *Config, redisClient *redis.Client) {
	// Health check
	router.GET("/health", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"status":    "healthy",
			"service":   "api-gateway",
			"timestamp": time.Now(),
		})
	})

	// API v1 routes
	v1 := router.Group("/api/v1")
	{
		// Auth proxy (forward to Rust service)
		auth := v1.Group("/auth")
		{
			auth.POST("/login", createProxy(config.AuthServiceURL + "/auth/login"))
			auth.POST("/register", createProxy(config.AuthServiceURL + "/auth/register"))
			auth.POST("/refresh", createProxy(config.AuthServiceURL + "/auth/refresh"))
			auth.POST("/logout", createProxy(config.AuthServiceURL + "/auth/logout"))
			auth.GET("/me", createProxy(config.AuthServiceURL + "/auth/me"))
		}

		// Analytics proxy (forward to Python service)
		analytics := v1.Group("/analytics")
		{
			analytics.GET("/*path", createProxy(config.PythonServiceURL))
			analytics.POST("/*path", createProxy(config.PythonServiceURL))
		}

		// Platform routes (would forward to Rust platform service)
		platform := v1.Group("/platform")
		{
			platform.GET("/tenants", func(c *gin.Context) {
				c.JSON(200, gin.H{"message": "Platform service not yet implemented"})
			})
		}

		// Commerce routes (would forward to Rust commerce service)
		commerce := v1.Group("/commerce")
		{
			commerce.GET("/products", func(c *gin.Context) {
				c.JSON(200, gin.H{"message": "Commerce service not yet implemented"})
			})
		}
	}

	// GraphQL endpoint (placeholder)
	router.POST("/graphql", func(c *gin.Context) {
		c.JSON(200, gin.H{"message": "GraphQL endpoint coming soon"})
	})

	// WebSocket endpoint (placeholder)
	router.GET("/ws", func(c *gin.Context) {
		c.JSON(200, gin.H{"message": "WebSocket endpoint coming soon"})
	})
}

// Simple proxy function
func createProxy(targetURL string) gin.HandlerFunc {
	return func(c *gin.Context) {
		// Create new request
		req, err := http.NewRequest(c.Request.Method, targetURL, c.Request.Body)
		if err != nil {
			c.JSON(500, gin.H{"error": "Failed to create request"})
			return
		}

		// Copy headers
		for key, values := range c.Request.Header {
			for _, value := range values {
				req.Header.Add(key, value)
			}
		}

		// Make request
		client := &http.Client{Timeout: 30 * time.Second}
		resp, err := client.Do(req)
		if err != nil {
			c.JSON(500, gin.H{"error": "Failed to forward request"})
			return
		}
		defer resp.Body.Close()

		// Copy response headers
		for key, values := range resp.Header {
			for _, value := range values {
				c.Header(key, value)
			}
		}

		// Copy response body
		c.DataFromReader(resp.StatusCode, resp.ContentLength, resp.Header.Get("Content-Type"), resp.Body, nil)
	}
}