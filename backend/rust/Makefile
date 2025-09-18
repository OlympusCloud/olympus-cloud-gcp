.PHONY: help build run test clean docker-build docker-run dev migrate setup check fmt lint

# Default target
help:
	@echo "Olympus Rust Services - Available Commands:"
	@echo "  make setup       - Install dependencies and setup development environment"
	@echo "  make build       - Build the Rust services in release mode"
	@echo "  make run         - Run the services locally"
	@echo "  make dev         - Run in development mode with auto-reload"
	@echo "  make test        - Run all tests"
	@echo "  make migrate     - Run database migrations"
	@echo "  make check       - Run cargo check"
	@echo "  make fmt         - Format code with rustfmt"
	@echo "  make lint        - Run clippy linter"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-run  - Run services in Docker"
	@echo "  make docker-compose - Start full stack with docker-compose"

# Setup development environment
setup:
	@echo "🔧 Setting up Rust development environment..."
	@rustup update stable
	@rustup component add rustfmt clippy
	@cargo install cargo-watch sqlx-cli
	@echo "✅ Development environment ready!"

# Build the project
build:
	@echo "🔨 Building Rust services..."
	@cargo build --release
	@echo "✅ Build complete!"

# Run the services
run:
	@echo "🚀 Starting Olympus Rust services..."
	@cargo run --release

# Development mode with auto-reload
dev:
	@echo "🔄 Starting development server with auto-reload..."
	@cargo watch -x run

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test --all-features

# Run integration tests
test-integration:
	@echo "🧪 Running integration tests..."
	@cargo test --test '*' -- --test-threads=1

# Run database migrations
migrate:
	@echo "📦 Running database migrations..."
	@sqlx migrate run --database-url "${DATABASE_URL:-postgresql://olympus:devpassword@localhost:5432/olympus}"

# Check code without building
check:
	@echo "✔️  Checking code..."
	@cargo check --all-features

# Format code
fmt:
	@echo "🎨 Formatting code..."
	@cargo fmt --all

# Lint code
lint:
	@echo "🔍 Linting code..."
	@cargo clippy --all-features -- -D warnings

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@rm -rf target/

# Docker operations
docker-build:
	@echo "🐳 Building Docker image..."
	@docker build -t olympus-rust:latest .

docker-run:
	@echo "🐳 Running Docker container..."
	@docker run -p 8000:8000 \
		-e DATABASE_URL="postgresql://olympus:devpassword@host.docker.internal:5432/olympus" \
		-e REDIS_URL="redis://host.docker.internal:6379" \
		olympus-rust:latest

# Start full stack with docker-compose
docker-compose:
	@echo "🐳 Starting full stack with docker-compose..."
	@docker-compose up -d

# Stop docker-compose services
docker-stop:
	@echo "🛑 Stopping docker-compose services..."
	@docker-compose down

# View logs
logs:
	@docker-compose logs -f rust-services

# Database operations
db-create:
	@echo "📦 Creating database..."
	@createdb olympus -U olympus -h localhost || true

db-drop:
	@echo "🗑️  Dropping database..."
	@dropdb olympus -U olympus -h localhost || true

db-reset: db-drop db-create migrate
	@echo "♻️  Database reset complete!"

# Benchmarks
bench:
	@echo "⚡ Running benchmarks..."
	@cargo bench

# Security audit
audit:
	@echo "🔐 Running security audit..."
	@cargo audit

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	@cargo doc --no-deps --open

# CI/CD targets
ci-test: check fmt lint test
	@echo "✅ CI tests passed!"

# Development database with Docker
dev-db:
	@echo "🐘 Starting PostgreSQL and Redis..."
	@docker run -d --name olympus-postgres \
		-e POSTGRES_USER=olympus \
		-e POSTGRES_PASSWORD=devpassword \
		-e POSTGRES_DB=olympus \
		-p 5432:5432 \
		postgres:15-alpine || true
	@docker run -d --name olympus-redis \
		-p 6379:6379 \
		redis:7-alpine || true
	@echo "✅ Databases started!"

# Stop development databases
stop-db:
	@echo "🛑 Stopping databases..."
	@docker stop olympus-postgres olympus-redis || true
	@docker rm olympus-postgres olympus-redis || true