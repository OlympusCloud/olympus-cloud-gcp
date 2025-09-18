#!/bin/bash

# Olympus Cloud GCP - Quick Setup Script
# Run this to get started immediately!

set -e

echo "🌩️  Olympus Cloud GCP - Development Setup"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Check prerequisites
echo "📋 Checking prerequisites..."

check_command() {
    if command -v $1 &> /dev/null; then
        print_status "$1 is installed"
        return 0
    else
        print_error "$1 is not installed"
        return 1
    fi
}

# Check required tools
MISSING_TOOLS=0

check_command git || MISSING_TOOLS=1
check_command docker || MISSING_TOOLS=1
check_command make || MISSING_TOOLS=1

# Check language-specific tools
if check_command cargo; then
    print_status "Rust is installed ($(cargo --version))"
else
    print_warning "Rust not installed - needed for backend/rust development"
    echo "    Install from: https://rustup.rs"
fi

if check_command go; then
    print_status "Go is installed ($(go version | cut -d' ' -f3))"
else
    print_warning "Go not installed - needed for backend/go development"
    echo "    Install from: https://golang.org/dl"
fi

if check_command python3; then
    print_status "Python is installed ($(python3 --version))"
else
    print_warning "Python not installed - needed for backend/python development"
    echo "    Install Python 3.11+"
fi

if check_command flutter; then
    print_status "Flutter is installed ($(flutter --version | head -1))"
else
    print_warning "Flutter not installed - needed for frontend development"
    echo "    Install from: https://flutter.dev/docs/get-started/install"
fi

if [ $MISSING_TOOLS -eq 1 ]; then
    echo ""
    print_error "Missing required tools. Please install them and run this script again."
    exit 1
fi

echo ""
echo "🔧 Setting up development environment..."

# Create necessary directories
print_status "Creating directory structure..."
mkdir -p backend/rust backend/go backend/python frontend
mkdir -p infrastructure/terraform edge/cloudflare
mkdir -p database/migrations database/seeds
mkdir -p tests/unit tests/integration tests/e2e
mkdir -p scripts monitoring/prometheus monitoring/grafana
mkdir -p .github/workflows .github/actions

# Setup environment file
if [ ! -f .env ]; then
    print_status "Creating .env file from template..."
    cp .env.example .env
    print_warning "Please update .env with your configuration"
else
    print_status ".env file already exists"
fi

# Initialize git if needed
if [ ! -d .git ]; then
    print_status "Initializing git repository..."
    git init
    git add .
    git commit -m "Initial commit: Olympus Cloud GCP"
fi

# Setup git worktrees for AI agents
echo ""
echo "🌳 Setting up git worktrees for AI agents..."

setup_worktree() {
    local branch=$1
    local dir=$2
    local agent=$3
    
    if [ ! -d "$dir" ]; then
        git worktree add -b "$branch" "$dir" 2>/dev/null || true
        print_status "Created worktree for $agent at $dir"
    else
        print_status "Worktree for $agent already exists"
    fi
}

setup_worktree "feat/rust-core" "worktree-claude" "Claude Code"
setup_worktree "feat/flutter-ui" "worktree-copilot" "GitHub Copilot"
setup_worktree "feat/gcp-infra" "worktree-gemini" "Google Gemini"
setup_worktree "feat/python-logic" "worktree-codex" "OpenAI Codex"
setup_worktree "feat/go-api" "worktree-chatgpt" "ChatGPT"

# Start Docker services
echo ""
echo "🐳 Starting Docker services..."

if docker info &> /dev/null; then
    docker-compose up -d postgres redis
    print_status "PostgreSQL and Redis containers started"
    
    # Wait for services to be ready
    echo "⏳ Waiting for services to be healthy..."
    sleep 5
    
    # Check if services are running
    if docker-compose ps | grep -q "postgres.*healthy"; then
        print_status "PostgreSQL is ready"
    else
        print_warning "PostgreSQL might not be fully ready yet"
    fi
    
    if docker-compose ps | grep -q "redis.*healthy"; then
        print_status "Redis is ready"
    else
        print_warning "Redis might not be fully ready yet"
    fi
else
    print_error "Docker is not running. Please start Docker Desktop."
fi

# Create initial project structure for each agent
echo ""
echo "📦 Creating initial project structures..."

# Rust structure
if [ ! -f backend/rust/Cargo.toml ]; then
    cat > backend/rust/Cargo.toml << 'EOF'
[workspace]
members = ["auth", "platform", "commerce", "shared"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"

[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.7"
EOF
    print_status "Created Rust workspace configuration"
fi

# Go structure
if [ ! -f backend/go/go.mod ]; then
    cd backend/go
    go mod init github.com/olympuscloud/olympus-gcp 2>/dev/null || true
    cd ../..
    print_status "Created Go module"
fi

# Python structure
if [ ! -f backend/python/requirements.txt ]; then
    cat > backend/python/requirements.txt << 'EOF'
fastapi==0.109.0
uvicorn[standard]==0.27.0
sqlalchemy==2.0.25
pandas==2.1.4
redis==5.0.1
EOF
    print_status "Created Python requirements file"
fi

# Flutter structure
if [ ! -f frontend/pubspec.yaml ]; then
    print_warning "Run 'flutter create' in frontend/ directory to initialize Flutter project"
fi

echo ""
echo "=================================="
echo "✅ Setup Complete!"
echo "=================================="
echo ""
echo "📚 Next Steps for Each Agent:"
echo ""
echo "🦀 Claude Code (Rust):"
echo "   cd worktree-claude/backend/rust"
echo "   cargo build"
echo ""
echo "🎨 GitHub Copilot (Flutter):"
echo "   cd worktree-copilot/frontend"
echo "   flutter create --org io.olympuscloud ."
echo "   flutter pub get"
echo ""
echo "☁️ Google Gemini (Infrastructure):"
echo "   cd worktree-gemini/infrastructure/terraform"
echo "   terraform init"
echo ""
echo "🐍 OpenAI Codex (Python):"
echo "   cd worktree-codex/backend/python"
echo "   python3 -m venv venv"
echo "   source venv/bin/activate"
echo "   pip install -r requirements.txt"
echo ""
echo "🐹 ChatGPT (Go):"
echo "   cd worktree-chatgpt/backend/go"
echo "   go mod tidy"
echo ""
echo "=================================="
echo "📖 Documentation: docs/00-EXECUTIVE-SUMMARY-ROADMAP.md"
echo "🚀 Quick Commands: make help"
echo "=================================="
echo ""
echo "Happy coding! 🚀"