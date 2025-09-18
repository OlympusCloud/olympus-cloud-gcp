# 🚀 Olympus Cloud GCP - Master Implementation Guide for AI Agents

> **Coordinated Development Plan for Multiple AI Coding Agents**

## 🎯 Overview

This guide coordinates the efforts of multiple AI coding agents (Claude Code, GitHub Copilot, Google Gemini, OpenAI Codex) to build the Olympus Cloud GCP platform efficiently and without conflicts.

## 🤖 Agent Role Assignments

### Claude Code - Lead Architect & Rust Developer
**Primary Responsibilities:**
- System architecture and design decisions
- Rust core modules (auth, platform, commerce)
- Security implementation
- Database schema design
- Performance optimization

**Work Directory:** `/backend/rust/`

### GitHub Copilot - Flutter & Frontend Specialist
**Primary Responsibilities:**
- Flutter application development
- UI/UX implementation
- Cross-platform compatibility
- Watch app development
- Frontend testing

**Work Directory:** `/frontend/`

### Google Gemini - GCP & Infrastructure Expert
**Primary Responsibilities:**
- GCP service configuration
- Terraform infrastructure
- Cloud Run deployment
- BigQuery analytics
- Vertex AI integration

**Work Directory:** `/infrastructure/` and `/edge/`

### OpenAI Codex - Python & Business Logic Developer
**Primary Responsibilities:**
- Python business modules
- AI/ML implementation
- Data processing pipelines
- Integration services
- Analytics module

**Work Directory:** `/backend/python/`

### ChatGPT - Go API & Integration Specialist
**Primary Responsibilities:**
- Go API gateway
- GraphQL implementation
- WebSocket services
- External integrations
- Middleware development

**Work Directory:** `/backend/go/`

## 📁 Git Workflow for Collaboration

### Initial Setup
```bash
# Each agent creates their own worktree
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp

# Claude Code worktree
git worktree add -b feat/rust-core worktree-claude

# GitHub Copilot worktree
git worktree add -b feat/flutter-ui worktree-copilot

# Google Gemini worktree
git worktree add -b feat/gcp-infra worktree-gemini

# OpenAI Codex worktree
git worktree add -b feat/python-logic worktree-codex

# ChatGPT worktree
git worktree add -b feat/go-api worktree-chatgpt
```

### Daily Workflow
```yaml
Morning_Sync:
  1. Pull latest changes from main
  2. Read updates in /docs/daily-status.md
  3. Check /docs/integration-points.md
  4. Update your task status

Development:
  1. Work in your assigned directories only
  2. Commit frequently with descriptive messages
  3. Push to your feature branch
  4. Update documentation as you go

Evening_Integration:
  1. Create pull request if module complete
  2. Update /docs/daily-status.md
  3. Document any blockers
  4. Plan next day's tasks
```

## 🏗️ Implementation Phases

### Week 1-2: Foundation Phase
**All Agents Parallel Tasks:**

**Claude Code:**
```rust
// Implement core auth module
- JWT token generation/validation
- OAuth2 flows
- Device authentication
- Session management
- Security middleware
```

**GitHub Copilot:**
```dart
// Flutter app foundation
- Project setup with flavors
- Navigation structure
- Authentication screens
- State management setup
- Theming system
```

**Google Gemini:**
```yaml
# GCP infrastructure
- Project setup
- Cloud SQL provisioning
- Cloud Run configuration
- Cloudflare Workers setup
- CI/CD pipeline
```

**OpenAI Codex:**
```python
# Python foundation
- Project structure
- Database models
- Common utilities
- Message bus setup
- Logging configuration
```

**ChatGPT:**
```go
// Go API gateway
- HTTP server setup
- Routing framework
- Middleware pipeline
- Authentication integration
- OpenAPI documentation
```

### Week 3-4: Core Business Logic

**Integration Points:**
```yaml
Database_Schema:
  Owner: Claude Code
  Consumers: All agents
  Location: /database/schema.sql

API_Contracts:
  Owner: ChatGPT
  Format: OpenAPI 3.0
  Location: /docs/api/openapi.yaml

Message_Events:
  Owner: Claude Code
  Format: Protobuf
  Location: /shared/proto/

Authentication:
  Owner: Claude Code
  Interface: REST API
  Endpoint: /api/v1/auth
```

### Week 5-8: Module Development

**Module Ownership:**
```yaml
Commerce_Module:
  Backend: Claude Code (Rust)
  API: ChatGPT (Go)
  Frontend: GitHub Copilot (Flutter)
  Database: Claude Code

Customer_Module:
  Backend: OpenAI Codex (Python)
  API: ChatGPT (Go)
  Frontend: GitHub Copilot (Flutter)
  Analytics: Google Gemini (BigQuery)

Platform_Module:
  Backend: Claude Code (Rust)
  API: ChatGPT (Go)
  Admin_UI: GitHub Copilot (Flutter)
  Infrastructure: Google Gemini
```

## 🔌 Integration Standards

### API Communication
```yaml
REST_API:
  Base_URL: https://api.olympuscloud.io
  Version: /v1
  Auth: Bearer {jwt_token}
  Format: JSON
  
GraphQL:
  Endpoint: /graphql
  Subscriptions: /graphql/ws
  Schema: /docs/graphql/schema.graphql

WebSocket:
  Endpoint: wss://ws.olympuscloud.io
  Protocol: JSON-RPC 2.0
  Heartbeat: 30 seconds
```

### Database Access
```python
# All agents use this connection pattern
DATABASE_URL = "postgresql://user:pass@localhost/olympus"
REDIS_URL = "redis://localhost:6379"

# Schema prefix by module
# auth.* - Authentication tables
# platform.* - Platform management
# commerce.* - Commerce operations
# customer.* - Customer data
```

### Message Bus Events
```rust
// Event naming convention
// module.entity.action
// Example: commerce.order.created

pub struct EventEnvelope {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub tenant_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
}
```

## 📋 Daily Checklist

### For All Agents
- [ ] Pull latest changes from main
- [ ] Read integration updates
- [ ] Work in assigned directories only
- [ ] Write tests for new code
- [ ] Update API documentation
- [ ] Commit with descriptive messages
- [ ] Update daily status
- [ ] Check for merge conflicts

### Code Quality Standards
```yaml
Testing:
  Unit_Tests: Required for all functions
  Integration_Tests: Required for APIs
  Coverage: Minimum 80%

Documentation:
  Code_Comments: For complex logic
  API_Docs: OpenAPI/GraphQL schema
  README: For each module

Security:
  Input_Validation: All user inputs
  SQL_Injection: Use prepared statements
  XSS_Prevention: Sanitize outputs
  Auth_Check: Every endpoint
```

## 🚦 Communication Protocol

### Documentation Updates
```markdown
# /docs/daily-status.md
## Date: YYYY-MM-DD

### Claude Code
- Completed: [List completed tasks]
- In Progress: [Current work]
- Blockers: [Any issues]
- Next: [Tomorrow's plan]

### [Repeat for each agent]
```

### Integration Points
```markdown
# /docs/integration-points.md
## Module: [Module Name]
### API Endpoints
- POST /api/v1/module/action
  - Owner: [Agent name]
  - Status: [Planning/Development/Testing/Complete]
  - Contract: [Link to OpenAPI spec]

### Database Tables
- module.table_name
  - Owner: [Agent name]
  - Schema: [Link to schema file]

### Events Published
- module.entity.action
  - Producer: [Agent name]
  - Consumers: [List of agents]
  - Payload: [Link to schema]
```

## 🎯 Success Criteria

### Module Completion Checklist
- [ ] All endpoints implemented and documented
- [ ] Unit tests written (>80% coverage)
- [ ] Integration tests passing
- [ ] API documentation complete
- [ ] Error handling implemented
- [ ] Logging added
- [ ] Performance benchmarked
- [ ] Security reviewed
- [ ] Code reviewed by another agent
- [ ] Merged to main branch

## 📊 Performance Targets

```yaml
API_Response_Times:
  GET: < 50ms
  POST: < 100ms
  Complex_Queries: < 200ms
  
Database_Queries:
  Simple_Select: < 10ms
  Complex_Join: < 50ms
  Write_Operations: < 20ms
  
Frontend_Performance:
  Initial_Load: < 1s
  Route_Change: < 200ms
  API_Call: < 100ms
  Optimistic_Update: Instant
```

## 🔄 Continuous Integration

### GitHub Actions Workflow
```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main, feat/*]
  pull_request:
    branches: [main]

jobs:
  test:
    strategy:
      matrix:
        service: [rust, go, python, flutter]
    
    steps:
      - uses: actions/checkout@v3
      - name: Run tests
        run: make test-${{ matrix.service }}
      
  build:
    needs: test
    steps:
      - name: Build containers
        run: make build-all
      
  deploy:
    needs: build
    if: github.ref == 'refs/heads/main'
    steps:
      - name: Deploy to staging
        run: make deploy-staging
```

## 🚨 Conflict Resolution

### When Conflicts Occur:
1. **Stop work immediately**
2. **Document in #conflicts channel**
3. **Designated resolver:**
   - Architecture conflicts: Claude Code
   - API conflicts: ChatGPT
   - UI conflicts: GitHub Copilot
   - Infrastructure: Google Gemini
   - Business logic: OpenAI Codex

### Prevention Strategies:
- Clear module boundaries
- Documented interfaces
- Daily status updates
- Regular integration
- Communication first

---

**Remember: We're building a unified platform. Communication and coordination are more important than individual speed. Quality over quantity, always.**