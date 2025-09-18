# 🚀 Olympus Cloud GCP - Quick Start Guide for AI Agents

> **Get started in 5 minutes - Everything you need to begin development**

## 📋 Pre-Development Checklist

Before starting, ensure you have:
- [ ] Access to `/Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp`
- [ ] Git configured with your agent identity
- [ ] Read the architecture documents in `/docs`
- [ ] Your assigned work directory ready

## 🎯 Quick Commands for Each Agent

### Claude Code (Rust Development)
```bash
# Setup your workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/rust-core worktree-claude
cd worktree-claude

# Initialize Rust project
mkdir -p backend/rust
cd backend/rust

# Create workspace structure
cat > Cargo.toml << 'EOF'
[workspace]
members = ["auth", "platform", "commerce", "shared"]
resolver = "2"

[workspace.package]
version = "1.0.0"
edition = "2021"
authors = ["Olympus Cloud Team"]

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
tower = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.7", features = ["runtime-tokio", "tls-rustls", "postgres", "uuid", "chrono", "json"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
jsonwebtoken = "9"
argon2 = "0.5"
tracing = "0.1"
tracing-subscriber = "0.3"
thiserror = "1"
anyhow = "1"
EOF

# Create auth module
cargo new --lib auth
cd auth
# Start implementing authentication
```

### GitHub Copilot (Flutter Development)
```bash
# Setup Flutter workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/flutter-ui worktree-copilot
cd worktree-copilot

# Create Flutter project
flutter create --org io.olympuscloud \
  --project-name olympus_app \
  --platforms ios,android,web,macos,windows,linux \
  frontend

cd frontend

# Add dependencies
cat > pubspec_additions.yaml << 'EOF'
dependencies:
  flutter_riverpod: ^2.4.9
  go_router: ^13.0.0
  dio: ^5.4.0
  get_it: ^7.6.4
  hive: ^2.2.3
  hive_flutter: ^1.1.0
  json_annotation: ^4.8.1
  freezed_annotation: ^2.4.1
  flutter_native_splash: ^2.3.8
  cached_network_image: ^3.3.1
  shimmer: ^3.0.0
  
dev_dependencies:
  build_runner: ^2.4.7
  freezed: ^2.4.6
  json_serializable: ^6.7.1
  flutter_launcher_icons: ^0.13.1
EOF

# Merge dependencies
cat pubspec_additions.yaml >> pubspec.yaml
flutter pub get

# Start building UI
```

### Google Gemini (GCP Infrastructure)
```bash
# Setup Infrastructure workspace  
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/gcp-infra worktree-gemini
cd worktree-gemini

# Create Terraform structure
mkdir -p infrastructure/terraform/{modules,environments}
cd infrastructure/terraform

# Initialize Terraform
cat > main.tf << 'EOF'
terraform {
  required_version = ">= 1.6.0"
  
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "~> 5.10"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.20"
    }
  }
  
  backend "gcs" {
    bucket = "olympus-terraform-state"
    prefix = "terraform/state"
  }
}

provider "google" {
  project = var.project_id
  region  = var.region
}

variable "project_id" {
  description = "GCP Project ID"
  type        = string
}

variable "region" {
  description = "GCP Region"
  type        = string
  default     = "us-central1"
}

variable "environment" {
  description = "Environment name"
  type        = string
}
EOF

# Start creating resources
```

### OpenAI Codex (Python Development)
```bash
# Setup Python workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/python-logic worktree-codex
cd worktree-codex

# Create Python structure
mkdir -p backend/python/{analytics,ai,integrations,shared}
cd backend/python

# Setup virtual environment
python3 -m venv venv
source venv/bin/activate

# Create requirements.txt
cat > requirements.txt << 'EOF'
# Core
fastapi==0.109.0
uvicorn[standard]==0.27.0
pydantic==2.5.3
python-dotenv==1.0.0

# Database
sqlalchemy==2.0.25
asyncpg==0.29.0
alembic==1.13.1

# Data Processing
pandas==2.1.4
numpy==1.26.3
scikit-learn==1.4.0

# Cache
redis==5.0.1
aiocache==0.12.2

# GCP
google-cloud-bigquery==3.14.1
google-cloud-aiplatform==1.40.0
google-cloud-storage==2.14.0

# AI/ML
transformers==4.36.2
torch==2.1.2
openai==1.9.0
anthropic==0.11.0

# Utils
httpx==0.26.0
tenacity==8.2.3
python-dateutil==2.8.2
pytz==2023.3
EOF

pip install -r requirements.txt

# Start implementing services
```

### ChatGPT (Go API Development)
```bash
# Setup Go workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/go-api worktree-chatgpt
cd worktree-chatgpt

# Initialize Go module
mkdir -p backend/go
cd backend/go
go mod init github.com/olympuscloud/olympus-gcp

# Add core dependencies
cat > go.mod << 'EOF'
module github.com/olympuscloud/olympus-gcp

go 1.21

require (
    github.com/gin-gonic/gin v1.9.1
    github.com/99designs/gqlgen v0.17.44
    github.com/gorilla/websocket v1.5.1
    github.com/prometheus/client_golang v1.18.0
    github.com/redis/go-redis/v9 v9.4.0
    github.com/jackc/pgx/v5 v5.5.2
    github.com/golang-jwt/jwt/v5 v5.2.0
    google.golang.org/grpc v1.61.0
    go.opentelemetry.io/otel v1.22.0
)
EOF

go mod tidy

# Create main structure
mkdir -p {api,graphql,websocket,middleware,handlers,services}

# Start implementing API
```

## 🔄 Daily Development Workflow

### Morning Routine (All Agents)
```bash
# 1. Pull latest changes
git checkout main
git pull origin main
git checkout feat/your-branch
git rebase main

# 2. Check for integration updates
cat docs/integration-points.md

# 3. Start development server
# Rust: cargo watch -x run
# Flutter: flutter run
# Python: uvicorn main:app --reload
# Go: air (with hot reload)

# 4. Run tests before starting
# Rust: cargo test
# Flutter: flutter test
# Python: pytest
# Go: go test ./...
```

### Commit Guidelines
```bash
# Make atomic commits with clear messages
git add -p  # Stage changes interactively
git commit -m "feat(module): add specific feature

- Implement X functionality
- Add Y validation
- Update Z documentation

Resolves: #issue-number"

# Push to your branch
git push origin feat/your-branch
```

### End of Day
```bash
# 1. Run all tests
make test

# 2. Update documentation
echo "## $(date +%Y-%m-%d) Progress" >> docs/daily-status.md
echo "- Completed: [list]" >> docs/daily-status.md
echo "- Tomorrow: [list]" >> docs/daily-status.md

# 3. Commit and push all changes
git add .
git commit -m "chore: end of day checkpoint"
git push origin feat/your-branch

# 4. Create PR if ready
gh pr create --title "feat: Your feature" --body "Description"
```

## 🧪 Testing Quick Reference

### Unit Tests
```bash
# Rust
cargo test --lib

# Flutter
flutter test test/unit

# Python
pytest tests/unit -v

# Go
go test ./... -short
```

### Integration Tests
```bash
# Run with Docker Compose
docker-compose -f docker-compose.test.yml up --abort-on-container-exit

# Cleanup
docker-compose -f docker-compose.test.yml down -v
```

### E2E Tests
```bash
# Start all services
docker-compose up -d

# Run E2E suite
npm run test:e2e

# View results
open test-results/report.html
```

## 🐛 Debugging Tips

### Common Issues and Solutions

#### Database Connection Issues
```bash
# Check PostgreSQL is running
docker ps | grep postgres

# Test connection
psql -h localhost -U olympus -d olympus_db

# Reset database
make db-reset
```

#### Port Conflicts
```bash
# Find process using port
lsof -i :8080

# Kill process
kill -9 <PID>

# Use different port
PORT=8081 cargo run
```

#### Dependency Issues
```bash
# Rust: Clear cargo cache
cargo clean
rm -rf ~/.cargo/registry/cache

# Flutter: Clear and get packages
flutter clean
flutter pub cache repair
flutter pub get

# Python: Reinstall dependencies
pip install --upgrade -r requirements.txt

# Go: Clear module cache
go clean -modcache
go mod download
```

## 🚀 Quick Deployment

### Local Development
```bash
# Start everything with Docker Compose
docker-compose up -d

# Watch logs
docker-compose logs -f

# Access services
# API: http://localhost:8080
# Frontend: http://localhost:3000
# PostgreSQL: localhost:5432
# Redis: localhost:6379
```

### Staging Deployment
```bash
# Build and push images
make build-all
make push-staging

# Deploy to GCP
gcloud run deploy olympus-api \
  --image gcr.io/olympus-cloud/api:latest \
  --region us-central1

# Run migrations
make migrate-staging
```

## 📚 Essential Resources

### Documentation
- Architecture: `/docs/01-ARCHITECTURE.md`
- API Spec: `/docs/api/openapi.yaml`
- Database Schema: `/database/schema.sql`

### Environment Variables
```bash
# Create .env file
cat > .env << 'EOF'
# Database
DATABASE_URL=postgresql://olympus:password@localhost:5432/olympus_db

# Redis
REDIS_URL=redis://localhost:6379

# JWT
JWT_SECRET=your-secret-key-here

# GCP
GOOGLE_PROJECT_ID=olympus-cloud
GOOGLE_REGION=us-central1

# API
API_PORT=8080
API_HOST=0.0.0.0
EOF
```

### Makefile Commands
```makefile
# Common commands
make dev          # Start development environment
make test         # Run all tests
make build        # Build all services
make clean        # Clean build artifacts
make db-migrate   # Run database migrations
make db-seed      # Seed test data
make lint         # Run linters
make fmt          # Format code
```

## 🆘 Getting Help

### When Stuck:
1. Check documentation in `/docs`
2. Search for similar patterns in codebase
3. Review integration points
4. Update daily status with blocker
5. Tag issue as `help-wanted`

### Communication Channels:
- Daily Status: `/docs/daily-status.md`
- Integration Points: `/docs/integration-points.md`
- Blockers: `/docs/blockers.md`
- Architecture Questions: `/docs/questions.md`

---

**Start coding! Remember: Quality over speed. Test everything. Document as you go.**