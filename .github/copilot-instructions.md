# Olympus Cloud GCP - AI Coding Agent Instructions

## Project Overview

This is a **modular monolith Cloud Business AI OS** targeting multi-industry business management (restaurants, retail, salons, events). The architecture emphasizes human-centric design with natural language interfaces and dramatic cost reduction over traditional microservices.

**Status**: Greenfield project in foundation phase - detailed documentation exists but code implementation has not yet begun.

**Mission**: 100% AI-driven development with minimal human oversight using coordinated multi-agent vibe coding.

## AI Agent Coordination Protocol

### Autonomous Development Rules
1. **Work Independently**: Each agent operates in isolation using git worktrees
2. **Documentation-First**: All decisions must reference existing docs (`docs/` folder)
3. **Continuous Integration**: Push code frequently, integrate through well-defined APIs
4. **Quality Gates**: All code must pass tests, linting, and security scans before merge
5. **Human Escalation**: Only escalate to humans for architectural decisions or external dependencies

### Git Worktree Workflow for AI Agents
```bash
# MANDATORY: Each agent must work in their dedicated worktree
git worktree add -b feat/rust-core worktree-claude       # Claude Code
git worktree add -b feat/flutter-ui worktree-copilot     # GitHub Copilot  
git worktree add -b feat/gcp-infra worktree-gemini       # Google Gemini
git worktree add -b feat/python-logic worktree-codex     # OpenAI Codex
git worktree add -b feat/go-api worktree-chatgpt         # ChatGPT
```

### Daily Agent Workflow
```yaml
Morning_Routine:
  1. cd your-worktree && git pull origin main
  2. Read docs/daily-status.md for updates
  3. Check docs/integration-points.md for dependencies
  4. Update your status in docs/daily-status.md
  5. Begin autonomous development

Development_Cycle:
  1. Implement features from docs/02-AI-AGENT-TASK-ASSIGNMENTS.md
  2. Write tests first (TDD approach)
  3. Commit every 1-2 hours with conventional commits
  4. Run quality checks: make fmt && make lint && make test
  5. Document integration points and API changes

Evening_Integration:
  1. Push all changes to your feature branch
  2. Create PR if module/feature is complete
  3. Update docs/daily-status.md with progress
  4. Note any blockers in docs/integration-points.md
```

## Architecture & Tech Stack

### Multi-Language Backend Strategy
- **Rust** (`/backend/rust/`): Core services (auth, platform, commerce) - memory safety & performance
- **Go** (`/backend/go/`): API gateway & GraphQL - excellent HTTP concurrency  
- **Python** (`/backend/python/`): Business logic, AI/ML, analytics - rich ecosystem
- **Flutter** (`/frontend/`): Universal app (iOS, Android, Web, Desktop, Watch)

### Infrastructure (GCP + Cloudflare)
- **GCP**: Cloud Run, PostgreSQL, Redis, BigQuery, Vertex AI
- **Cloudflare**: Workers for <50ms global response times
- **Database**: Single PostgreSQL with schema-per-module design
- **Deployment**: Docker containers on Cloud Run

## Agent-Specific Autonomous Instructions

### Claude Code (Rust Core Services) - `/backend/rust/`
**Primary Mission**: Build rock-solid core services with memory safety and performance
```rust
// Your autonomous development priorities:
1. Implement authentication system (JWT, OAuth2, device auth)
2. Create modular workspace: auth, platform, commerce, shared
3. Build database layer with sqlx and strong typing
4. Implement event-driven communication via Redis
5. Ensure 100% test coverage and security-first design

// Quality gates you MUST meet:
- cargo clippy -- -D warnings (zero warnings)
- cargo test (100% pass rate)
- cargo audit (zero vulnerabilities)
- Documentation for all public APIs
```

### GitHub Copilot (Flutter Frontend) - `/frontend/`
**Primary Mission**: Create universal app for all platforms with human-centric UX
```dart
// Your autonomous development priorities:
1. Initialize Flutter project with flavors/branding system
2. Implement Riverpod state management architecture
3. Build authentication flows and responsive layouts
4. Create adaptive UI components for all screen sizes
5. Implement watch apps (Apple Watch, Wear OS, Garmin)

// Quality gates you MUST meet:
- flutter analyze (zero issues)
- flutter test (>80% coverage)
- Build success on iOS, Android, Web, Desktop
- Accessibility compliance (a11y)
```

### Google Gemini (GCP Infrastructure) - `/infrastructure/`
**Primary Mission**: Build production-ready cloud infrastructure
```hcl
// Your autonomous development priorities:
1. Setup GCP project with required APIs enabled
2. Implement Terraform for Cloud SQL, Redis, Cloud Run
3. Configure Cloudflare Workers for edge computing
4. Setup CI/CD pipeline with GitHub Actions
5. Implement monitoring with Cloud Operations

// Quality gates you MUST meet:
- terraform validate && terraform plan (zero errors)
- Infrastructure as Code best practices
- Cost optimization (<$100/month dev environment)
- Security hardening and zero-trust architecture
```

### OpenAI Codex (Python Business Logic) - `/backend/python/`
**Primary Mission**: Build AI/ML capabilities and business intelligence
```python
# Your autonomous development priorities:
1. Setup FastAPI with async/await patterns
2. Implement analytics engine with pandas/NumPy
3. Build natural language processing layer
4. Create recommendation and suggestion engines
5. Integrate with BigQuery for data warehousing

# Quality gates you MUST meet:
- python -m flake8 && mypy . (zero issues)
- pytest --cov=. --cov-report=html (>80% coverage)
- bandit -r . (zero security issues)
- Load testing for high-performance analytics
```

### ChatGPT (Go API Gateway) - `/backend/go/`
**Primary Mission**: Build high-performance API gateway and GraphQL layer
```go
// Your autonomous development priorities:
1. Setup Gin/Fiber with middleware pipeline
2. Implement GraphQL with gqlgen
3. Build WebSocket real-time communication
4. Create rate limiting and request routing
5. Integrate with Rust services via HTTP/gRPC

// Quality gates you MUST meet:
- golangci-lint run (zero issues)
- go test -v ./... (100% pass rate)  
- gosec ./... (zero security vulnerabilities)
- Load testing for >1000 concurrent connections
```

## Coordination & Integration Points

### API Contract Management
- **Owner**: ChatGPT (Go API Gateway)
- **Contract**: `docs/06-API-SPECIFICATION.yaml` - MUST be updated for any API changes
- **Integration**: All services communicate through well-defined REST/GraphQL APIs
- **Versioning**: Use semantic versioning (v1.0.0) and maintain backward compatibility

### Database Schema Coordination  
- **Owner**: Claude Code (Rust Core)
- **Schema**: `docs/05-COMPLETE-DATABASE-SCHEMA.sql` - Single source of truth
- **Migrations**: All schema changes go through this file first, then implement in code
- **RLS**: Row-level security enforced at database level, not application level

### Event-Driven Communication
- **Medium**: Redis pub/sub channels
- **Pattern**: Domain events (order.created, payment.processed, etc.)
- **Schema**: Events defined in `/backend/rust/shared/events`
- **Guarantee**: At-least-once delivery with idempotent handlers

### Authentication Flow
```yaml
Flow: Frontend → Go API → Rust Auth → Database
Token: JWT with tenant_id, user_id, roles, permissions
Refresh: Automatic token refresh in frontend
Device: Device fingerprinting for security
Sessions: Redis-backed session management
```

## Autonomous Development Workflow

### Phase 1: Foundation (Week 1-2) - CRITICAL PATH
**ALL AGENTS START IMMEDIATELY**

#### Day 1: Project Initialization
```bash
# Each agent in parallel:
git worktree add -b feat/your-module worktree-your-name
cd worktree-your-name
# Follow your specific setup from docs/04-QUICK-START-GUIDE.md
```

#### Day 2-3: Core Implementation
- **Claude Code**: Auth service with JWT + database models
- **ChatGPT**: API gateway with auth middleware integration  
- **Copilot**: Login/signup screens with state management
- **Gemini**: GCP project + Cloud SQL + Redis provisioning
- **Codex**: Analytics service foundation + database connections

#### Day 4-5: Integration Testing
- All agents push to feature branches
- Create integration PRs with documentation updates
- Run full test suite: `make test`
- Performance benchmarking

### Phase 2: Core Modules (Week 3-8) - PARALLEL DEVELOPMENT

#### Module Development Assignments
```yaml
Commerce_Module:
  Owner: Claude Code (Rust)
  Integrations: [ChatGPT API, Copilot UI, Codex Analytics]
  Timeline: Week 3-4

Product_Catalog:  
  Owner: ChatGPT (Go GraphQL)
  Integrations: [Claude Database, Copilot UI, Codex Search]
  Timeline: Week 3-4

Inventory_System:
  Owner: Claude Code + Codex (Rust + Python)
  Integrations: [Real-time updates via WebSocket]
  Timeline: Week 5-6

Customer_Management:
  Owner: Codex (Python ML/CRM)
  Integrations: [Claude Auth, ChatGPT API, Copilot UI]
  Timeline: Week 5-6

Analytics_Dashboard:
  Owner: Codex + Copilot (Python + Flutter)
  Integrations: [BigQuery, Real-time charts]
  Timeline: Week 7-8
```

### Autonomous Decision Making Framework

#### When to Proceed Independently
- Implementation details within your module
- Code structure and patterns
- Testing strategies and coverage
- Performance optimizations
- Documentation updates

#### When to Coordinate with Team
- API contract changes (update `docs/06-API-SPECIFICATION.yaml`)
- Database schema modifications (update `docs/05-COMPLETE-DATABASE-SCHEMA.sql`)
- New dependencies or external services
- Security model changes
- Breaking changes to integration points

#### Conflict Resolution Protocol
1. **Try to resolve in code**: Use feature flags, graceful degradation
2. **Document the issue**: Update `docs/integration-points.md`
3. **Async coordination**: Leave detailed comments in PR descriptions
4. **Emergency only**: Tag humans with @urgent in GitHub issues

## Quality Assurance Automation

### Mandatory Quality Gates (All Agents)
```bash
# BEFORE every commit:
make fmt          # Auto-format code
make lint         # Check code quality  
make test         # Run all tests
make security     # Security vulnerability scan

# BEFORE every PR:
make build        # Ensure builds succeed
make coverage     # Check test coverage >80%
make docs         # Generate/update documentation
```

### Performance Requirements
- API endpoints: <100ms response time (p99)
- Database queries: <50ms
- Frontend initial load: <2s
- Test suite execution: <5 minutes
- Docker build time: <3 minutes

### Security Requirements (Zero Tolerance)
- No hardcoded secrets or credentials
- All inputs validated and sanitized
- SQL injection prevention mandatory
- XSS protection on all user inputs
- Rate limiting on all public endpoints
- Audit logging for all data modifications

## AI Agent Communication Protocols

### Daily Status Updates
Create/update `docs/daily-status.md`:
```markdown
# Daily Status - [DATE]

## Claude Code (Rust)
- ✅ Completed: Auth module basic structure
- 🔄 In Progress: JWT token validation
- 🎯 Next: User registration endpoint  
- 🚫 Blockers: None

## GitHub Copilot (Flutter)
- ✅ Completed: Project setup with Riverpod
- 🔄 In Progress: Login screen UI
- 🎯 Next: Authentication state management
- 🚫 Blockers: Waiting for auth API endpoints

## [Other agents follow same format]
```

### Integration Points Documentation
Maintain `docs/integration-points.md`:
```markdown
# Integration Points

## Authentication Flow
- **Status**: 🔄 In Development
- **Owner**: Claude Code
- **Dependencies**: ChatGPT (API), Copilot (UI)
- **API**: POST /auth/login, GET /auth/me
- **Testing**: Unit tests complete, integration pending

## Order Processing
- **Status**: 📋 Planned
- **Owner**: Claude Code + ChatGPT
- **Timeline**: Week 3
- **API**: REST + GraphQL endpoints
```

### Commit Message Standards
```bash
# Format: <agent>(<scope>): <description>
claude(auth): implement JWT token generation
copilot(ui): add responsive login form
gemini(infra): setup Cloud SQL with read replicas
codex(analytics): implement user behavior tracking  
chatgpt(api): add GraphQL schema for products
```

### Branch Naming Convention
```bash
feat/rust-auth-jwt        # Claude Code
feat/flutter-login-ui     # GitHub Copilot  
feat/gcp-cloud-sql       # Google Gemini
feat/python-analytics    # OpenAI Codex
feat/go-graphql-api      # ChatGPT
```

## Human-Centric Development Principles

### 1. Natural Language First
Every UI component should support natural language input:
```dart
// Example: Search/command bar in every screen
SearchField(
  placeholder: "What would you like to do?",
  onSubmit: (query) => processNaturalLanguage(query),
)
```

### 2. Context-Aware Interfaces
UIs adapt based on:
- Time of day (morning rush vs evening close)
- User role (owner vs employee vs customer)
- Business type (restaurant vs retail vs salon)
- Historical usage patterns

### 3. Predictive Assistance
- Show relevant suggestions before users ask
- Learn from user patterns and preferences
- Automate repetitive tasks with user approval
- Provide contextual help and tips

## Technical Implementation Patterns

### Database Design (Claude Code)
```sql
-- Multi-tenant with row-level security
CREATE POLICY tenant_isolation ON users 
FOR ALL USING (tenant_id = current_setting('app.tenant_id')::uuid);

-- Event sourcing for audit trails
CREATE TABLE events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  aggregate_id UUID NOT NULL,
  event_type TEXT NOT NULL,
  event_data JSONB NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### API Design (ChatGPT)
```go
// GraphQL schema with real-time subscriptions
type Subscription {
  orderUpdates(tenantId: ID!): Order
  inventoryChanges(locationId: ID!): InventoryItem
  notifications(userId: ID!): Notification
}

// REST with consistent error handling
func (h *Handler) CreateOrder(c *gin.Context) {
  if err := h.service.CreateOrder(ctx, req); err != nil {
    c.JSON(http.StatusBadRequest, ErrorResponse{
      Code: "ORDER_CREATION_FAILED",
      Message: err.Error(),
    })
    return
  }
}
```

### State Management (GitHub Copilot)
```dart
// Riverpod with async state and error handling
@riverpod
class OrderNotifier extends _$OrderNotifier {
  @override
  FutureOr<List<Order>> build() async {
    return ref.watch(orderRepositoryProvider).getOrders();
  }

  Future<void> createOrder(CreateOrderRequest request) async {
    state = const AsyncValue.loading();
    state = await AsyncValue.guard(
      () => ref.read(orderRepositoryProvider).createOrder(request),
    );
  }
}
```

### Analytics Pipeline (OpenAI Codex)
```python
# Real-time analytics with streaming
@dataclass
class UserEvent:
    user_id: str
    event_type: str
    properties: Dict[str, Any]
    timestamp: datetime

class AnalyticsProcessor:
    async def process_event(self, event: UserEvent):
        # Real-time processing
        await self.update_metrics(event)
        await self.check_anomalies(event)
        await self.trigger_predictions(event)
```

### Infrastructure as Code (Google Gemini)
```hcl
# Auto-scaling with cost optimization
resource "google_cloud_run_service" "api" {
  template {
    metadata {
      annotations = {
        "autoscaling.knative.dev/minScale" = "1"
        "autoscaling.knative.dev/maxScale" = "100"
        "run.googleapis.com/cpu-throttling" = "false"
      }
    }
    spec {
      containers {
        resources {
          limits = {
            cpu    = "2000m"
            memory = "2Gi"
          }
        }
      }
    }
  }
}
```

## Essential Development Commands

### Quick Start Commands
```bash
# Start everything
make dev

# Agent-specific development
make dev-rust      # Claude Code
make dev-go        # ChatGPT  
make dev-python    # OpenAI Codex
make dev-flutter   # GitHub Copilot

# Quality assurance
make test          # All tests
make fmt           # Format code
make lint          # Check issues  
make security      # Security scan
```

### Database Operations
```bash
make db-reset      # Reset to clean state
make db-console    # PostgreSQL console
make redis-cli     # Redis CLI
make migrate-up    # Apply migrations
```

### Monitoring & Debugging
```bash
make logs          # View service logs
make health        # Check service health
make metrics       # Performance metrics
```

## Documentation-First Development

### Required Reading Order
1. `docs/00-EXECUTIVE-SUMMARY-ROADMAP.md` - Vision & timeline
2. `docs/01-MASTER-IMPLEMENTATION-GUIDE.md` - Agent coordination
3. `docs/02-AI-AGENT-TASK-ASSIGNMENTS.md` - Specific tasks
4. `docs/05-COMPLETE-DATABASE-SCHEMA.sql` - Data model
5. `docs/06-API-SPECIFICATION.yaml` - API contracts

### Integration Points
Monitor `docs/integration-points.md` and `docs/daily-status.md` for cross-agent dependencies and blockers.

## Common Pitfalls to Avoid

### 1. **Don't Build Microservices**
This is a modular monolith. Services share databases and deploy together for cost efficiency.

### 2. **Language Boundaries**
Respect agent ownership. Don't modify Rust code if you're the Go agent, etc.

### 3. **Database Migrations**
Schema changes go through `docs/05-COMPLETE-DATABASE-SCHEMA.sql` first, then implement in individual services.

### 4. **Coordination First**
For cross-service features, update documentation and coordinate with other agents before coding.

## Quick Reference

| Need | Command/Location |
|------|------------------|
| Start everything | `make dev` |
| Reset database | `make db-reset` |
| API documentation | `docs/06-API-SPECIFICATION.yaml` |
| Agent coordination | `docs/01-MASTER-IMPLEMENTATION-GUIDE.md` |
| Current tasks | `docs/02-AI-AGENT-TASK-ASSIGNMENTS.md` |
| Database schema | `docs/05-COMPLETE-DATABASE-SCHEMA.sql` |
| Health check | `make health` |

## Human-Centric Development Philosophy

Remember: We're building technology that adapts to humans, not the other way around. Every feature should prioritize natural language interfaces, context awareness, and predictive assistance over rigid automation.

---

*This is a living document. Update as patterns emerge and architecture evolves.*