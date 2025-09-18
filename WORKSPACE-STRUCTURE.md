# Olympus Cloud GCP - Workspace Structure

## 📁 Directory Organization

```
olympus-cloud-gcp/
├── backend/                    # Backend services
│   ├── rust/                  # [Claude Code] Core services
│   │   ├── auth/             # Authentication service ✅
│   │   ├── platform/         # Platform core (pending)
│   │   ├── commerce/         # Commerce engine (pending)
│   │   └── shared/           # Shared utilities ✅
│   ├── go/                   # [ChatGPT] API Gateway
│   │   ├── cmd/              # Entry points
│   │   ├── internal/         # Internal packages
│   │   └── pkg/              # Public packages
│   └── python/               # [OpenAI Codex] Analytics & ML
│       ├── app/              # Application code
│       ├── tests/            # Test files
│       └── alembic/          # Database migrations
│
├── frontend/                   # [GitHub Copilot] Flutter app
│   ├── lib/                  # Dart source code
│   ├── test/                 # Test files
│   ├── assets/               # Images, fonts
│   └── [platform folders]    # iOS, Android, Web, etc.
│
├── infrastructure/             # [Google Gemini] IaC
│   └── terraform/            # Terraform configurations
│       ├── modules/          # Reusable modules
│       ├── environments/     # Environment configs
│       └── *.tf              # Main config files
│
├── docs/                       # Documentation
│   ├── 00-EXECUTIVE-SUMMARY-ROADMAP.md
│   ├── 01-MASTER-IMPLEMENTATION-GUIDE.md
│   ├── 02-AI-AGENT-TASK-ASSIGNMENTS.md
│   ├── 03-NEBUSAI-METHODOLOGY-IMPLEMENTATION.md
│   ├── 04-QUICK-START-GUIDE.md
│   ├── 05-COMPLETE-DATABASE-SCHEMA.sql
│   ├── 06-API-SPECIFICATION.yaml
│   ├── 07-DEPLOYMENT-GUIDE.md
│   ├── AI-AGENT-NEXT-STEPS.md    # ← Start here!
│   ├── daily-status.md           # Daily coordination
│   └── integration-points.md     # Integration tracking
│
├── scripts/                    # Utility scripts
│   ├── setup.sh               # Initial setup
│   ├── deploy.sh              # Deployment script
│   └── backup.sh              # Backup utilities
│
├── .github/                    # GitHub configurations
│   ├── workflows/             # CI/CD pipelines
│   ├── copilot-instructions.md
│   └── CLAUDE.md              # Claude-specific instructions
│
├── worktree-*/                # Git worktrees for each agent
│   ├── worktree-claude        # Claude Code workspace
│   ├── worktree-copilot       # GitHub Copilot workspace
│   ├── worktree-chatgpt       # ChatGPT workspace
│   ├── worktree-codex         # OpenAI Codex workspace
│   └── worktree-gemini        # Google Gemini workspace
│
├── .env.example               # Environment template ✅
├── docker-compose.yml         # Local development ✅
├── Makefile                   # Build automation ✅
├── README.md                  # Project overview ✅
└── CLAUDE.md                  # Claude guidance ✅
```

## 🎯 Service Port Allocation

| Service | Port | Owner | Status |
|---------|------|-------|--------|
| PostgreSQL | 5432 | Infrastructure | ✅ Running |
| Redis | 6379 | Infrastructure | ✅ Running |
| Rust Auth | 8000 | Claude Code | ✅ Ready |
| Python Analytics | 8001 | OpenAI Codex | 🔄 Pending |
| Go API Gateway | 8080 | ChatGPT | 🔄 Pending |
| Flutter Web | 3000 | GitHub Copilot | 🔄 Pending |
| Adminer | 8081 | Dev Tool | ✅ Available |
| Redis Commander | 8082 | Dev Tool | ✅ Available |
| Prometheus | 9090 | Monitoring | Optional |
| Grafana | 3001 | Monitoring | Optional |

## 🚀 Quick Commands

```bash
# Start everything
make dev

# Start databases only
docker-compose up -d postgres redis

# Run specific service
make dev-rust      # Rust services
make dev-go        # Go API gateway
make dev-python    # Python analytics
make dev-flutter   # Flutter frontend

# Run tests
make test

# Clean everything
make clean
```

## 📝 Key Files for Each Agent

### Claude Code (Rust)
- `backend/rust/Cargo.toml` - Dependencies
- `backend/rust/*/src/` - Source code
- Already implemented auth service ✅

### ChatGPT (Go)
- `backend/go/go.mod` - Dependencies
- `backend/go/README.md` - Instructions
- Ready to start implementation

### OpenAI Codex (Python)
- `backend/python/requirements.txt` - Dependencies
- `backend/python/README.md` - Instructions
- Ready to start implementation

### GitHub Copilot (Flutter)
- `frontend/README.md` - Instructions
- Ready for `flutter create`

### Google Gemini (Infrastructure)
- `infrastructure/terraform/variables.tf` - Configuration
- Ready for terraform init

## ✅ Completed Setup

1. **Workspace Structure** - Clean and organized
2. **Rust Auth Service** - Fully implemented with JWT
3. **Database Schema** - Ready in docs/
4. **Docker Compose** - PostgreSQL and Redis configured
5. **Environment Config** - .env.example complete
6. **Agent Instructions** - README in each directory
7. **Coordination Docs** - Updated with next steps

## 🎯 Next Actions

Each AI agent should:
1. Check out their git worktree
2. Read `docs/AI-AGENT-NEXT-STEPS.md`
3. Follow their specific README
4. Start implementing their service
5. Update `docs/daily-status.md` with progress

The workspace is clean, organized, and ready for coordinated development!