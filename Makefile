# Olympus Cloud GCP - Master Makefile
# One command to rule them all

.PHONY: help
help: ## Show this help message
	@echo "🌩️  Olympus Cloud GCP - Development Commands"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ============================================
# Setup & Installation
# ============================================

.PHONY: install-all
install-all: install-rust install-go install-python install-flutter install-tools ## Install all dependencies

.PHONY: install-rust
install-rust: ## Install Rust dependencies
	@echo "🦀 Installing Rust dependencies..."
	cd backend/rust && cargo fetch

.PHONY: install-go
install-go: ## Install Go dependencies
	@echo "🐹 Installing Go dependencies..."
	cd backend/go && go mod download

.PHONY: install-python
install-python: ## Install Python dependencies
	@echo "🐍 Installing Python dependencies..."
	cd backend/python && python3 -m venv venv && . venv/bin/activate && pip install -r requirements.txt

.PHONY: install-flutter
install-flutter: ## Install Flutter dependencies
	@echo "🎨 Installing Flutter dependencies..."
	cd frontend && flutter pub get

.PHONY: install-tools
install-tools: ## Install development tools
	@echo "🛠️  Installing development tools..."
	@which docker || echo "❌ Docker not installed - please install Docker Desktop"
	@which terraform || brew install terraform
	@which gcloud || echo "❌ gcloud not installed - please install Google Cloud SDK"
	@which air || go install github.com/cosmtrek/air@latest
	@which migrate || go install -tags 'postgres' github.com/golang-migrate/migrate/v4/cmd/migrate@latest

.PHONY: setup-dev
setup-dev: ## Setup development environment
	@echo "🔧 Setting up development environment..."
	@cp .env.example .env 2>/dev/null || echo "✓ .env already exists"
	@mkdir -p data/postgres data/redis
	@echo "✅ Development environment ready!"

# ============================================
# Development
# ============================================

.PHONY: dev
dev: ## Run all services in development mode
	@echo "🚀 Starting development environment..."
	docker-compose up -d postgres redis
	@sleep 3
	@make migrate-up
	@make -j4 dev-api dev-python dev-go dev-flutter

.PHONY: dev-api
dev-api: ## Run Rust API in development
	@echo "🦀 Starting Rust API..."
	cd backend/rust && cargo watch -x run

.PHONY: dev-go
dev-go: ## Run Go API Gateway in development
	@echo "🐹 Starting Go API Gateway..."
	cd backend/go && air

.PHONY: dev-python
dev-python: ## Run Python services in development
	@echo "🐍 Starting Python services..."
	cd backend/python && . venv/bin/activate && uvicorn main:app --reload --port 8001

.PHONY: dev-flutter
dev-flutter: ## Run Flutter in development
	@echo "🎨 Starting Flutter..."
	cd frontend && flutter run -d chrome --web-port 3000

.PHONY: dev-docker
dev-docker: ## Run everything in Docker
	@echo "🐳 Starting Docker Compose..."
	docker-compose up

# ============================================
# Database
# ============================================

.PHONY: db-up
db-up: ## Start database containers
	docker-compose up -d postgres redis

.PHONY: db-down
db-down: ## Stop database containers
	docker-compose down

.PHONY: db-reset
db-reset: ## Reset database
	docker-compose down -v
	docker-compose up -d postgres redis
	@sleep 3
	@make migrate-up
	@make db-seed

.PHONY: migrate-up
migrate-up: ## Run database migrations
	@echo "🗄️  Running migrations..."
	psql postgresql://olympus:devpassword@localhost:5432/olympus < docs/05-COMPLETE-DATABASE-SCHEMA.sql

.PHONY: migrate-down
migrate-down: ## Rollback database migrations
	@echo "🗄️  Rolling back migrations..."
	migrate -database "postgresql://olympus:devpassword@localhost:5432/olympus?sslmode=disable" -path database/migrations down 1

.PHONY: db-seed
db-seed: ## Seed database with test data
	@echo "🌱 Seeding database..."
	cd backend/rust && cargo run --bin seed

.PHONY: db-console
db-console: ## Open PostgreSQL console
	psql postgresql://olympus:devpassword@localhost:5432/olympus

.PHONY: redis-cli
redis-cli: ## Open Redis CLI
	docker exec -it olympus-redis redis-cli

# ============================================
# Testing
# ============================================

.PHONY: test
test: test-rust test-go test-python test-flutter ## Run all tests

.PHONY: test-rust
test-rust: ## Run Rust tests
	@echo "🦀 Testing Rust..."
	cd backend/rust && cargo test

.PHONY: test-go
test-go: ## Run Go tests
	@echo "🐹 Testing Go..."
	cd backend/go && go test ./... -v

.PHONY: test-python
test-python: ## Run Python tests
	@echo "🐍 Testing Python..."
	cd backend/python && . venv/bin/activate && pytest

.PHONY: test-flutter
test-flutter: ## Run Flutter tests
	@echo "🎨 Testing Flutter..."
	cd frontend && flutter test

.PHONY: test-integration
test-integration: ## Run integration tests
	@echo "🧪 Running integration tests..."
	docker-compose -f docker-compose.test.yml up --abort-on-container-exit

.PHONY: test-e2e
test-e2e: ## Run end-to-end tests
	@echo "🎯 Running E2E tests..."
	cd tests/e2e && npm test

.PHONY: coverage
coverage: ## Generate test coverage reports
	@echo "📊 Generating coverage reports..."
	cd backend/rust && cargo tarpaulin --out Html
	cd backend/go && go test -coverprofile=coverage.out ./... && go tool cover -html=coverage.out
	cd backend/python && . venv/bin/activate && pytest --cov=. --cov-report=html
	cd frontend && flutter test --coverage

# ============================================
# Building
# ============================================

.PHONY: build
build: build-rust build-go build-python build-flutter ## Build all services

.PHONY: build-rust
build-rust: ## Build Rust services
	@echo "🦀 Building Rust..."
	cd backend/rust && cargo build --release

.PHONY: build-go
build-go: ## Build Go services
	@echo "🐹 Building Go..."
	cd backend/go && CGO_ENABLED=0 go build -o bin/api ./cmd/api

.PHONY: build-python
build-python: ## Build Python services
	@echo "🐍 Building Python..."
	cd backend/python && python -m compileall .

.PHONY: build-flutter
build-flutter: ## Build Flutter app
	@echo "🎨 Building Flutter..."
	cd frontend && flutter build web --release

.PHONY: build-docker
build-docker: ## Build Docker images
	@echo "🐳 Building Docker images..."
	docker build -f backend/Dockerfile.api -t olympus-api:local .
	docker build -f frontend/Dockerfile -t olympus-web:local frontend/

.PHONY: build-prod
build-prod: ## Build for production
	@echo "🏗️  Building for production..."
	@make build-docker
	@echo "✅ Production build complete!"

# ============================================
# Code Quality
# ============================================

.PHONY: fmt
fmt: fmt-rust fmt-go fmt-python fmt-flutter ## Format all code

.PHONY: fmt-rust
fmt-rust: ## Format Rust code
	@echo "🦀 Formatting Rust..."
	cd backend/rust && cargo fmt

.PHONY: fmt-go
fmt-go: ## Format Go code
	@echo "🐹 Formatting Go..."
	cd backend/go && go fmt ./...

.PHONY: fmt-python
fmt-python: ## Format Python code
	@echo "🐍 Formatting Python..."
	cd backend/python && . venv/bin/activate && black . && isort .

.PHONY: fmt-flutter
fmt-flutter: ## Format Flutter code
	@echo "🎨 Formatting Flutter..."
	cd frontend && dart format .

.PHONY: lint
lint: lint-rust lint-go lint-python lint-flutter ## Lint all code

.PHONY: lint-rust
lint-rust: ## Lint Rust code
	@echo "🦀 Linting Rust..."
	cd backend/rust && cargo clippy -- -D warnings

.PHONY: lint-go
lint-go: ## Lint Go code
	@echo "🐹 Linting Go..."
	cd backend/go && golangci-lint run

.PHONY: lint-python
lint-python: ## Lint Python code
	@echo "🐍 Linting Python..."
	cd backend/python && . venv/bin/activate && flake8 . && mypy .

.PHONY: lint-flutter
lint-flutter: ## Lint Flutter code
	@echo "🎨 Linting Flutter..."
	cd frontend && flutter analyze

.PHONY: security
security: ## Run security checks
	@echo "🔐 Running security checks..."
	cd backend/rust && cargo audit
	cd backend/go && gosec ./...
	cd backend/python && . venv/bin/activate && bandit -r .
	cd frontend && flutter pub audit

# ============================================
# Deployment
# ============================================

.PHONY: deploy-dev
deploy-dev: ## Deploy to development environment
	@echo "🚀 Deploying to development..."
	./scripts/deploy.sh dev

.PHONY: deploy-staging
deploy-staging: ## Deploy to staging environment
	@echo "🚀 Deploying to staging..."
	./scripts/deploy.sh staging

.PHONY: deploy-prod
deploy-prod: ## Deploy to production environment
	@echo "🚀 Deploying to production..."
	@read -p "⚠️  Deploy to PRODUCTION? Type 'yes' to confirm: " confirm && [ $$confirm = "yes" ] || exit 1
	./scripts/deploy.sh prod

.PHONY: rollback
rollback: ## Rollback last deployment
	@echo "↩️  Rolling back deployment..."
	gcloud run services update-traffic olympus-api-$(ENV) --to-revisions=LATEST-1=100

# ============================================
# Infrastructure
# ============================================

.PHONY: tf-init
tf-init: ## Initialize Terraform
	@echo "🏗️  Initializing Terraform..."
	cd infrastructure/terraform && terraform init

.PHONY: tf-plan
tf-plan: ## Plan Terraform changes
	@echo "📋 Planning infrastructure changes..."
	cd infrastructure/terraform && terraform plan

.PHONY: tf-apply
tf-apply: ## Apply Terraform changes
	@echo "🚀 Applying infrastructure changes..."
	cd infrastructure/terraform && terraform apply

.PHONY: tf-destroy
tf-destroy: ## Destroy infrastructure (DANGEROUS!)
	@echo "💣 Destroying infrastructure..."
	@read -p "⚠️  DESTROY infrastructure? Type 'destroy' to confirm: " confirm && [ $$confirm = "destroy" ] || exit 1
	cd infrastructure/terraform && terraform destroy

# ============================================
# Monitoring
# ============================================

.PHONY: logs
logs: ## Tail application logs
	docker-compose logs -f

.PHONY: logs-api
logs-api: ## Tail API logs
	docker-compose logs -f api

.PHONY: metrics
metrics: ## Open metrics dashboard
	open http://localhost:3000/grafana

.PHONY: health
health: ## Check service health
	@echo "🏥 Checking service health..."
	@curl -s http://localhost:8080/health | jq '.' || echo "❌ API not responding"
	@docker ps --format "table {{.Names}}\t{{.Status}}"

# ============================================
# Utilities
# ============================================

.PHONY: clean
clean: ## Clean build artifacts
	@echo "🧹 Cleaning build artifacts..."
	cd backend/rust && cargo clean
	cd backend/go && rm -rf bin/
	cd backend/python && find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	cd frontend && flutter clean
	docker-compose down -v

.PHONY: update
update: ## Update all dependencies
	@echo "📦 Updating dependencies..."
	cd backend/rust && cargo update
	cd backend/go && go get -u ./...
	cd backend/python && . venv/bin/activate && pip-review --auto
	cd frontend && flutter pub upgrade

.PHONY: docs
docs: ## Generate documentation
	@echo "📚 Generating documentation..."
	cd backend/rust && cargo doc --open
	cd backend/go && godoc -http=:6060
	@echo "Documentation available at http://localhost:6060"

.PHONY: api-docs
api-docs: ## Open API documentation
	@echo "📖 Opening API documentation..."
	open docs/06-API-SPECIFICATION.yaml

.PHONY: git-setup
git-setup: ## Setup git worktrees for agents
	@echo "🌳 Setting up git worktrees..."
	git worktree add -b feat/rust-core worktree-claude || true
	git worktree add -b feat/flutter-ui worktree-copilot || true
	git worktree add -b feat/gcp-infra worktree-gemini || true
	git worktree add -b feat/python-logic worktree-codex || true
	git worktree add -b feat/go-api worktree-chatgpt || true
	@echo "✅ Git worktrees ready!"

.PHONY: status
status: ## Show project status
	@echo "📊 Project Status"
	@echo "=================="
	@echo "🦀 Rust:" && cd backend/rust && cargo --version
	@echo "🐹 Go:" && go version
	@echo "🐍 Python:" && python3 --version
	@echo "🎨 Flutter:" && flutter --version | head -1
	@echo "🐳 Docker:" && docker --version
	@echo "☁️  GCloud:" && gcloud --version | head -1
	@echo ""
	@echo "Services:"
	@docker ps --format "table {{.Names}}\t{{.Status}}" 2>/dev/null || echo "No services running"

# ============================================
# Quick Commands
# ============================================

.PHONY: up
up: dev ## Alias for 'make dev'

.PHONY: down
down: db-down ## Alias for 'make db-down'

.PHONY: restart
restart: down up ## Restart all services

.PHONY: reset
reset: clean db-reset setup-dev ## Reset everything to clean state