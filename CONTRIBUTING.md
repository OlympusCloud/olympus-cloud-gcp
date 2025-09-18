# 🤝 Contributing to Olympus Cloud GCP

Welcome AI Agents and Developers! This guide ensures we all work together efficiently.

## 📋 Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [Development Process](#development-process)
3. [Coding Standards](#coding-standards)
4. [Commit Guidelines](#commit-guidelines)
5. [Testing Requirements](#testing-requirements)
6. [Documentation Standards](#documentation-standards)
7. [Pull Request Process](#pull-request-process)
8. [Communication](#communication)

## 📜 Code of Conduct

### Our Standards
- **Quality First**: No shortcuts, no technical debt
- **Human-Centric**: Technology adapts to humans
- **Collaboration**: Help each other succeed
- **Documentation**: Write it as you build it
- **Security**: Built-in, not bolted-on

## 🔄 Development Process

### 1. Daily Workflow

```bash
# Morning Sync
1. Pull latest changes from main
2. Read docs/daily-status.md
3. Update your task status
4. Plan your work for the day

# During Development
1. Work in your assigned directories
2. Commit frequently (every 1-2 hours)
3. Write tests as you code
4. Document as you go

# Evening Wrap-up
1. Push all changes
2. Update docs/daily-status.md
3. Note any blockers
4. Clean up temporary files
```

### 2. Branch Strategy

```bash
main                    # Production-ready code
├── staging            # Pre-production testing
└── feat/*             # Feature branches (your work)
    ├── feat/rust-core      # Claude Code
    ├── feat/flutter-ui     # GitHub Copilot
    ├── feat/gcp-infra     # Google Gemini
    ├── feat/python-logic   # OpenAI Codex
    └── feat/go-api        # ChatGPT
```

### 3. Module Ownership

| Module | Owner | Backup |
|--------|-------|--------|
| Auth & Security | Claude Code | ChatGPT |
| API Gateway | ChatGPT | Claude Code |
| Frontend | GitHub Copilot | - |
| Infrastructure | Google Gemini | - |
| Analytics & AI | OpenAI Codex | Claude Code |

## 💻 Coding Standards

### Universal Principles

1. **Clean Code**: Readable, maintainable, testable
2. **DRY**: Don't Repeat Yourself
3. **SOLID**: Follow SOLID principles
4. **KISS**: Keep It Simple, Stupid
5. **YAGNI**: You Aren't Gonna Need It

### Language-Specific Standards

#### Rust (Claude Code)
```rust
// Use clear, descriptive names
pub struct OrderService {
    repository: Arc<OrderRepository>,
    event_bus: Arc<EventBus>,
}

// Always handle errors explicitly
pub async fn create_order(&self, req: CreateOrderRequest) 
    -> Result<Order, OrderError> {
    // Implementation
}

// Document public APIs
/// Creates a new order with inventory validation
/// 
/// # Errors
/// Returns `OrderError::InsufficientStock` if inventory unavailable
pub async fn process_order(&self, order: Order) -> Result<(), OrderError>
```

#### Go (ChatGPT)
```go
// Use interfaces for flexibility
type OrderService interface {
    CreateOrder(ctx context.Context, req *CreateOrderRequest) (*Order, error)
}

// Always use context
func (s *orderService) CreateOrder(ctx context.Context, req *CreateOrderRequest) (*Order, error) {
    // Check context cancellation
    if ctx.Err() != nil {
        return nil, ctx.Err()
    }
    // Implementation
}

// Handle errors properly
if err != nil {
    return nil, fmt.Errorf("failed to create order: %w", err)
}
```

#### Python (OpenAI Codex)
```python
# Use type hints
from typing import List, Optional, Dict, Any

async def process_analytics(
    tenant_id: str,
    date_range: DateRange
) -> AnalyticsResult:
    """Process analytics for the given tenant and date range."""
    # Implementation

# Use dataclasses
from dataclasses import dataclass

@dataclass
class MetricQuery:
    tenant_id: str
    metric_type: str
    date_range: DateRange
    
# Handle exceptions properly
try:
    result = await calculate_metrics(query)
except MetricError as e:
    logger.error(f"Metric calculation failed: {e}")
    raise
```

#### Flutter (GitHub Copilot)
```dart
// Use proper state management
class OrderScreen extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final orders = ref.watch(ordersProvider);
    
    return orders.when(
      data: (data) => OrderList(orders: data),
      loading: () => LoadingIndicator(),
      error: (err, stack) => ErrorWidget(err),
    );
  }
}

// Use consistent naming
class OrderRepository {
  Future<List<Order>> fetchOrders() async {
    // Implementation
  }
}
```

## 📝 Commit Guidelines

### Commit Message Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation
- **style**: Formatting, no code change
- **refactor**: Code restructuring
- **test**: Adding tests
- **chore**: Maintenance
- **perf**: Performance improvement
- **security**: Security fix

### Examples
```bash
# Good commits
git commit -m "feat(auth): implement JWT refresh token rotation"
git commit -m "fix(orders): correct tax calculation for split payments"
git commit -m "docs(api): add order endpoint examples"
git commit -m "test(inventory): add integration tests for stock movements"

# Bad commits (don't do this!)
git commit -m "fixed stuff"
git commit -m "WIP"
git commit -m "updates"
```

### Commit Frequency
- Commit every 1-2 hours
- Each commit should be atomic (one logical change)
- Commits should compile and pass basic tests

## 🧪 Testing Requirements

### Test Coverage Goals
- **Unit Tests**: 80% minimum coverage
- **Integration Tests**: All API endpoints
- **E2E Tests**: Critical user workflows

### Test Structure
```
tests/
├── unit/           # Fast, isolated tests
├── integration/    # Service integration tests
└── e2e/           # Full workflow tests
```

### Writing Tests

#### Rust Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_order() {
        // Arrange
        let service = create_test_service().await;
        let request = create_test_request();
        
        // Act
        let result = service.create_order(request).await;
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, OrderStatus::Pending);
    }
}
```

#### Go Tests
```go
func TestCreateOrder(t *testing.T) {
    // Arrange
    service := NewOrderService(mockRepo)
    req := &CreateOrderRequest{
        Items: []OrderItem{{ProductID: "123", Quantity: 2}},
    }
    
    // Act
    order, err := service.CreateOrder(context.Background(), req)
    
    // Assert
    assert.NoError(t, err)
    assert.Equal(t, OrderStatusPending, order.Status)
}
```

## 📚 Documentation Standards

### Code Documentation
- Document all public APIs
- Include examples for complex functions
- Explain "why" not just "what"

### README Files
Each module should have:
```markdown
# Module Name

## Overview
Brief description of what this module does

## Features
- Feature 1
- Feature 2

## Usage
```example code```

## API Reference
Link to detailed API docs

## Testing
How to run tests

## Dependencies
List of dependencies
```

### API Documentation
- Use OpenAPI/Swagger for REST APIs
- Include request/response examples
- Document error codes

## 🔀 Pull Request Process

### Before Creating PR

- [ ] All tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation is updated
- [ ] No console.log or debug statements
- [ ] No commented-out code
- [ ] Commits are clean and logical

### PR Template
```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings

## Screenshots (if applicable)
Add screenshots for UI changes
```

### Review Process
1. Create PR with descriptive title
2. Fill out PR template
3. Request review from appropriate agent
4. Address feedback
5. Get approval
6. Merge to target branch

## 💬 Communication

### Daily Status Updates
Location: `docs/daily-status.md`

```markdown
## Date: 2024-XX-XX

### [Agent Name]
**Completed:**
- Task 1
- Task 2

**In Progress:**
- Task 3

**Blockers:**
- None

**Tomorrow:**
- Task 4
- Task 5
```

### Asking for Help
1. Check existing documentation
2. Search for similar issues
3. If still blocked, update `docs/blockers.md`:

```markdown
## Blocker: [Description]
**Agent:** [Your Name]
**Module:** [Module Name]
**Description:** Detailed description of the issue
**What I've Tried:**
- Attempt 1
- Attempt 2
**Help Needed:** Specific help required
```

### Integration Points
When your module needs to integrate with another:

1. Document in `docs/integration-points.md`
2. Notify the other agent
3. Agree on interface
4. Write integration tests

## 🎯 Definition of Done

A feature is "done" when:

- [ ] Code is written and working
- [ ] Unit tests written and passing (>80% coverage)
- [ ] Integration tests written and passing
- [ ] Documentation updated
- [ ] Code reviewed by another agent
- [ ] No security vulnerabilities
- [ ] Performance benchmarked
- [ ] Deployed to staging
- [ ] Acceptance criteria met

## 🏆 Best Practices

### Do's ✅
- Write clean, readable code
- Test everything
- Document as you go
- Commit frequently
- Ask for help when stuck
- Review others' code
- Keep learning

### Don'ts ❌
- Don't commit broken code
- Don't skip tests
- Don't ignore warnings
- Don't duplicate code
- Don't hardcode secrets
- Don't work in isolation
- Don't sacrifice quality for speed

## 🚀 Quick Commands

```bash
# Run all tests
make test

# Format your code
make fmt

# Check for issues
make lint

# Build everything
make build

# Start development
make dev

# Get help
make help
```

---

**Remember: We're building the future of business software. Every line of code matters. Every test counts. Every documentation page helps. Build it right, build it together!**