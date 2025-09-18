# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Olympus Cloud GCP is a **modular monolith Cloud Business AI OS** targeting multi-industry business management (restaurants, retail, salons, events). This is a greenfield project in foundation phase with detailed documentation but no code implementation yet.

**Mission**: 100% AI-driven development with coordinated multi-agent vibe coding.

## Architecture

```
┌─────────────────────────────────────────────┐
│             Edge Layer                       │
│          Cloudflare Workers                  │
│         (<50ms global response)              │
└───────────────┬─────────────────────────────┘
                │
┌───────────────▼─────────────────────────────┐
│           API Gateway                        │
│     Go (GraphQL + REST + WebSocket)         │
│          High concurrency                    │
└───────────────┬─────────────────────────────┘
                │
┌───────────────▼─────────────────────────────┐
│          Core Services                       │
│   ┌──────────────┐  ┌──────────────┐       │
│   │  Rust Core   │  │ Python Logic │       │
│   │  (Auth,      │  │  (Analytics, │       │
│   │  Platform,   │  │   AI/ML)     │       │
│   │  Commerce)   │  │              │       │
│   └──────────────┘  └──────────────┘       │
└───────────────┬─────────────────────────────┘
                │
┌───────────────▼─────────────────────────────┐
│           Data Layer                         │
│  PostgreSQL │ Redis │ BigQuery               │
│  (Single DB with schema-per-module)          │
└─────────────────────────────────────────────┘
```

## Common Development Commands

### Build and Run
```bash
# Install all dependencies
make install-all

# Setup development environment
make setup-dev

# Run everything locally
make dev

# Run specific service
make dev-rust      # Rust core services
make dev-go        # Go API gateway
make dev-python    # Python services
make dev-flutter   # Flutter frontend
```

### Testing
```bash
# Run all tests
make test

# Run specific tests
make test-rust
make test-go
make test-python
make test-flutter

# Generate coverage reports
make coverage
```

### Code Quality
```bash
# Format all code
make fmt

# Lint all code
make lint

# Security scan
make security
```

### Database Operations
```bash
# Reset database to clean state
make db-reset

# Open PostgreSQL console
make db-console

# Open Redis CLI
make redis-cli

# Run migrations
make migrate-up
```

## Project Structure

```
.
├── backend/
│   ├── rust/           # Core services (auth, platform, commerce)
│   │   ├── auth/       # Authentication service
│   │   ├── platform/   # Multi-tenancy, users, permissions
│   │   ├── commerce/   # Orders, payments, inventory
│   │   └── shared/     # Common types, events, utilities
│   ├── go/             # API Gateway (GraphQL + REST)
│   └── python/         # Business logic, analytics, AI/ML
├── frontend/           # Flutter universal app
├── infrastructure/     # Terraform for GCP + Cloudflare
└── docs/               # Complete documentation
```

## Key Files

- **Database Schema**: `docs/05-COMPLETE-DATABASE-SCHEMA.sql` - Single source of truth for database structure
- **API Specification**: `docs/06-API-SPECIFICATION.yaml` - OpenAPI spec for all endpoints
- **Implementation Guide**: `docs/01-MASTER-IMPLEMENTATION-GUIDE.md` - Detailed implementation roadmap
- **Task Assignments**: `docs/02-AI-AGENT-TASK-ASSIGNMENTS.md` - Specific tasks for each AI agent

## Development Workflow

### For Rust Core Services (backend/rust/)
1. Workspace-based structure with `auth`, `platform`, `commerce`, and `shared` crates
2. Use `sqlx` for database operations with compile-time query verification
3. Implement JWT authentication with Argon2 password hashing
4. Publish domain events to Redis for event-driven communication
5. Ensure all code passes: `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`, `cargo audit`

### For Go API Gateway (backend/go/)
1. Use Gin or Fiber for HTTP framework
2. Implement GraphQL with gqlgen
3. Add WebSocket support for real-time features
4. Integrate with Rust services via HTTP/gRPC
5. Ensure all code passes: `golangci-lint run`, `go test -v ./...`

### For Python Services (backend/python/)
1. Use FastAPI with async/await patterns
2. Implement analytics with pandas/NumPy
3. Build ML capabilities with scikit-learn/TensorFlow
4. Integrate with BigQuery for data warehousing
5. Ensure all code passes: `flake8`, `mypy`, `pytest`

### For Flutter Frontend (frontend/)
1. Support iOS, Android, Web, Desktop, and Watch platforms
2. Use Riverpod for state management
3. Implement adaptive UI for all screen sizes
4. Use GoRouter for navigation
5. Ensure all code passes: `flutter analyze`, `flutter test`

## Database Conventions

- **Multi-tenant architecture** with row-level security
- **UUID primary keys** for all tables
- **Soft deletes** with `deleted_at` timestamps
- **Audit columns**: `created_at`, `updated_at`, `created_by`, `updated_by`
- **Event sourcing** for critical business operations
- **Schema-per-module** design within single PostgreSQL database

## API Conventions

- **RESTful endpoints** for CRUD operations
- **GraphQL** for complex queries and real-time subscriptions
- **JWT authentication** with refresh tokens
- **Consistent error responses** with error codes and messages
- **Request/response validation** with OpenAPI schemas
- **Rate limiting** and **request throttling**

## Testing Requirements

- **Unit tests**: Minimum 80% code coverage
- **Integration tests**: Test service interactions
- **E2E tests**: Critical user journeys
- **Performance tests**: Meet latency targets (<100ms API response)
- **Security tests**: OWASP compliance

## Security Standards

- **Zero-trust architecture**
- **Argon2 password hashing**
- **JWT with short-lived access tokens**
- **Row-level security in PostgreSQL**
- **Input validation and sanitization**
- **SQL injection prevention**
- **XSS protection**
- **Rate limiting on all endpoints**
- **Audit logging for all data changes**

## Performance Targets

- **API Response**: <100ms (p99)
- **Database Queries**: <50ms
- **JWT Generation**: <1ms
- **JWT Validation**: <0.5ms
- **Frontend Load**: <2s
- **Global Response**: <50ms (via Cloudflare)

## Git Workflow

```bash
# Create feature branch
git checkout -b feat/your-feature

# Commit with conventional commits
git commit -m "feat(module): add new feature"
# Types: feat, fix, docs, test, chore

# Push and create PR
git push origin feat/your-feature
gh pr create
```

## AI Agent Coordination

This project uses multiple AI agents working in parallel:
- **Claude Code**: Rust core services & system architecture
- **GitHub Copilot**: Flutter frontend
- **Google Gemini**: GCP infrastructure
- **OpenAI Codex**: Python business logic
- **ChatGPT**: Go API gateway

Each agent works in their designated area. Integration points are documented in `docs/integration-points.md` and daily status updates in `docs/daily-status.md`.

## Important Notes

- This is a **modular monolith**, not microservices - services share databases and deploy together
- Focus on **human-centric design** with natural language interfaces
- Prioritize **developer experience** with clear documentation and tooling
- Maintain **cost efficiency** - target <$100/month for development environment
- Follow **documentation-first** approach - update docs before implementing