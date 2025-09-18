# 📋 Olympus Cloud GCP - Quick Reference Card

> **Keep this handy while developing!**

## 🚀 Essential Commands

```bash
# Start everything
make dev

# Run your service only
make dev-rust      # Claude Code
make dev-go        # ChatGPT
make dev-python    # OpenAI Codex
make dev-flutter   # GitHub Copilot

# Database
make db-console    # PostgreSQL console
make redis-cli     # Redis CLI
make db-reset      # Reset database

# Testing
make test          # Run all tests
make test-rust     # Test Rust only
make coverage      # Coverage report

# Code Quality
make fmt           # Format code
make lint          # Check issues
make security      # Security scan
```

## 📁 Your Work Directory

| Agent | Directory | Main Files |
|-------|-----------|------------|
| Claude Code | `/backend/rust/` | `Cargo.toml`, `src/main.rs` |
| GitHub Copilot | `/frontend/` | `pubspec.yaml`, `lib/main.dart` |
| Google Gemini | `/infrastructure/` | `main.tf`, `variables.tf` |
| OpenAI Codex | `/backend/python/` | `requirements.txt`, `main.py` |
| ChatGPT | `/backend/go/` | `go.mod`, `main.go` |

## 🔗 Service URLs (Local)

- **API**: http://localhost:8080
- **Frontend**: http://localhost:3000
- **PostgreSQL**: localhost:5432
- **Redis**: localhost:6379
- **Adminer**: http://localhost:8081
- **Redis Commander**: http://localhost:8082

## 🗄️ Database Connection

```bash
postgresql://olympus:devpassword@localhost:5432/olympus?sslmode=disable
```

## 🔑 Environment Variables

```bash
DATABASE_URL=postgresql://olympus:devpassword@localhost:5432/olympus?sslmode=disable
REDIS_URL=redis://localhost:6379
JWT_SECRET=dev-secret-change-in-production
API_PORT=8080
ENVIRONMENT=development
```

## 📝 Git Workflow

```bash
# Start work
git checkout main && git pull
git checkout -b feat/your-feature

# Save work
git add -p                    # Interactive staging
git commit -m "type: message"  # Conventional commit
git push origin feat/your-feature

# Create PR
gh pr create
```

## 📊 Commit Types

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation
- `test:` Tests
- `refactor:` Code restructuring
- `chore:` Maintenance
- `perf:` Performance
- `security:` Security fix

## 🏗️ Module Structure

```
your-module/
├── api/          # REST endpoints
├── service/      # Business logic
├── repository/   # Data access
├── models/       # Data structures
├── events/       # Event definitions
├── tests/        # Test files
└── docs/         # Documentation
```

## 🔄 Integration Points

### Auth (All → Janus/Claude Code)
```http
POST /auth/login
Authorization: Bearer {token}
```

### Orders (Frontend → API → Services)
```http
POST /api/v1/orders
GET /api/v1/orders/{id}
```

### Events (Publish/Subscribe)
```javascript
// Publish
EventBus.publish("order.created", orderData)

// Subscribe
EventBus.subscribe("order.created", handler)
```

## 📈 Performance Targets

- API Response: <100ms (p99)
- Database Query: <50ms
- Frontend Load: <1s
- Test Coverage: >80%
- Docker Build: <2min

## 🚦 Status Codes

| Code | Meaning | Action |
|------|---------|--------|
| 200 | Success | Continue |
| 201 | Created | Resource created |
| 400 | Bad Request | Check input |
| 401 | Unauthorized | Check auth |
| 403 | Forbidden | Check permissions |
| 404 | Not Found | Check URL/ID |
| 429 | Rate Limited | Slow down |
| 500 | Server Error | Check logs |

## 🧪 Testing Checklist

- [ ] Unit tests written
- [ ] Integration tests written
- [ ] Tests passing locally
- [ ] Coverage >80%
- [ ] No console.log
- [ ] No hardcoded values

## 🔐 Security Checklist

- [ ] Input validation
- [ ] SQL injection prevention
- [ ] XSS protection
- [ ] CSRF protection
- [ ] Secrets in env vars
- [ ] JWT validation
- [ ] Rate limiting
- [ ] Audit logging

## 📚 Documentation Files

1. **Start Here**: `docs/00-EXECUTIVE-SUMMARY-ROADMAP.md`
2. **Your Tasks**: `docs/02-AI-AGENT-TASK-ASSIGNMENTS.md`
3. **Quick Start**: `docs/04-QUICK-START-GUIDE.md`
4. **Database**: `docs/05-COMPLETE-DATABASE-SCHEMA.sql`
5. **API Spec**: `docs/06-API-SPECIFICATION.yaml`

## 🆘 Getting Help

```bash
# Check documentation
ls docs/

# View Makefile help
make help

# Check service health
make health

# View logs
make logs

# Update daily status
echo "Status update" >> docs/daily-status.md
```

## 🎯 Daily Routine

### Morning (5 min)
```bash
git pull
make dev
cat docs/daily-status.md
```

### During Work
```bash
# Make changes
# Test locally
make test-[your-language]
# Commit frequently
git commit -m "feat: add feature"
```

### Evening (5 min)
```bash
git push
# Update docs/daily-status.md
make down
```

## 💡 Pro Tips

1. **Test First**: Write tests before code
2. **Small Commits**: One logical change per commit
3. **Document Now**: Don't leave it for later
4. **Ask Questions**: Better to ask than assume
5. **Check Logs**: `docker-compose logs -f [service]`
6. **Use Make**: `make help` shows all commands
7. **Stay Synced**: Pull changes frequently

## 🏆 Success Metrics

✅ Code compiles  
✅ Tests pass  
✅ Documentation updated  
✅ PR approved  
✅ Deployed to staging  
✅ No security issues  
✅ Performance targets met  

---

**Quick Help**: `make help` | **Docs**: `/docs` | **Status**: `/docs/daily-status.md`

**Remember: Quality > Speed. Always.**