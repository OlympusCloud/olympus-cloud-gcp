package config

import (
	"os"
	"strconv"

	"github.com/joho/godotenv"
	"github.com/spf13/viper"
)

type Config struct {
	// Server settings
	Port        int    `mapstructure:"PORT"`
	Environment string `mapstructure:"ENVIRONMENT"`
	LogLevel    string `mapstructure:"LOG_LEVEL"`

	// Database settings
	DatabaseURL      string `mapstructure:"DATABASE_URL"`
	DatabaseHost     string `mapstructure:"DATABASE_HOST"`
	DatabasePort     int    `mapstructure:"DATABASE_PORT"`
	DatabaseUser     string `mapstructure:"DATABASE_USER"`
	DatabasePassword string `mapstructure:"DATABASE_PASSWORD"`
	DatabaseName     string `mapstructure:"DATABASE_NAME"`

	// Redis settings
	RedisURL      string `mapstructure:"REDIS_URL"`
	RedisHost     string `mapstructure:"REDIS_HOST"`
	RedisPort     int    `mapstructure:"REDIS_PORT"`
	RedisPassword string `mapstructure:"REDIS_PASSWORD"`
	RedisDB       int    `mapstructure:"REDIS_DB"`

	// JWT settings
	JWTSecret           string `mapstructure:"JWT_SECRET"`
	JWTExpirationHours  int    `mapstructure:"JWT_EXPIRATION_HOURS"`
	JWTRefreshDays      int    `mapstructure:"JWT_REFRESH_DAYS"`

	// External service URLs
	RustAuthServiceURL    string `mapstructure:"RUST_AUTH_SERVICE_URL"`
	PythonAnalyticsURL    string `mapstructure:"PYTHON_ANALYTICS_URL"`

	// Rate limiting
	RateLimitRequests int `mapstructure:"RATE_LIMIT_REQUESTS"`
	RateLimitWindow   int `mapstructure:"RATE_LIMIT_WINDOW"`

	// CORS settings
	CorsAllowedOrigins []string `mapstructure:"CORS_ALLOWED_ORIGINS"`
	CorsAllowedMethods []string `mapstructure:"CORS_ALLOWED_METHODS"`
	CorsAllowedHeaders []string `mapstructure:"CORS_ALLOWED_HEADERS"`
}

func Load() (*Config, error) {
	// Load .env file if it exists
	_ = godotenv.Load()

	// Set default values
	viper.SetDefault("PORT", 8080)
	viper.SetDefault("ENVIRONMENT", "development")
	viper.SetDefault("LOG_LEVEL", "info")
	viper.SetDefault("DATABASE_HOST", "localhost")
	viper.SetDefault("DATABASE_PORT", 5432)
	viper.SetDefault("DATABASE_NAME", "olympus_dev")
	viper.SetDefault("REDIS_HOST", "localhost")
	viper.SetDefault("REDIS_PORT", 6379)
	viper.SetDefault("REDIS_DB", 0)
	viper.SetDefault("JWT_EXPIRATION_HOURS", 24)
	viper.SetDefault("JWT_REFRESH_DAYS", 30)
	viper.SetDefault("RUST_AUTH_SERVICE_URL", "http://localhost:8000")
	viper.SetDefault("PYTHON_ANALYTICS_URL", "http://localhost:8001")
	viper.SetDefault("RATE_LIMIT_REQUESTS", 100)
	viper.SetDefault("RATE_LIMIT_WINDOW", 60)
	viper.SetDefault("CORS_ALLOWED_ORIGINS", []string{"*"})
	viper.SetDefault("CORS_ALLOWED_METHODS", []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"})
	viper.SetDefault("CORS_ALLOWED_HEADERS", []string{"*"})

	// Bind environment variables
	viper.AutomaticEnv()

	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		return nil, err
	}

	// Override with environment variables if they exist
	if port := os.Getenv("PORT"); port != "" {
		if p, err := strconv.Atoi(port); err == nil {
			config.Port = p
		}
	}

	// Validate required configuration
	if config.JWTSecret == "" {
		config.JWTSecret = "dev-secret-key-change-in-production"
	}

	return &config, nil
}