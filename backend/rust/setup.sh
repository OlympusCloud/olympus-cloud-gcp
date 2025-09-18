#!/bin/bash

# Olympus Rust Services - Development Setup Script

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${CYAN}🚀 Olympus Rust Services - Development Setup${NC}"
echo "================================================"

# Check prerequisites
echo -e "\n${YELLOW}📋 Checking prerequisites...${NC}"

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust is not installed${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi
echo -e "${GREEN}✅ Rust is installed${NC}"

# Check for Docker
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed${NC}"
    echo "Please install Docker from https://docker.com/"
    exit 1
fi
echo -e "${GREEN}✅ Docker is installed${NC}"

# Install Rust tools
echo -e "\n${YELLOW}🔧 Installing Rust development tools...${NC}"

# Update Rust
rustup update stable

# Add required components
rustup component add rustfmt clippy

# Install cargo tools
echo "Installing cargo-watch for auto-reload..."
cargo install cargo-watch --locked 2>/dev/null || echo "cargo-watch already installed"

echo "Installing sqlx-cli for database migrations..."
cargo install sqlx-cli --no-default-features --features postgres --locked 2>/dev/null || echo "sqlx-cli already installed"

echo "Installing cargo-audit for security scanning..."
cargo install cargo-audit --locked 2>/dev/null || echo "cargo-audit already installed"

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo -e "\n${YELLOW}📝 Creating .env file...${NC}"
    cat > .env << EOL
# Database
DATABASE_URL=postgresql://olympus:devpassword@localhost:5432/olympus

# Redis
REDIS_URL=redis://localhost:6379

# JWT
JWT_SECRET=your-secret-key-change-in-production

# Server
PORT=8000
RUST_LOG=olympus=debug,tower_http=debug

# Environment
ENVIRONMENT=development
EOL
    echo -e "${GREEN}✅ .env file created${NC}"
else
    echo -e "${GREEN}✅ .env file already exists${NC}"
fi

# Start databases with Docker
echo -e "\n${YELLOW}🐳 Starting PostgreSQL and Redis with Docker...${NC}"

# Start PostgreSQL
if ! docker ps | grep -q olympus-postgres; then
    docker run -d \
        --name olympus-postgres \
        -e POSTGRES_USER=olympus \
        -e POSTGRES_PASSWORD=devpassword \
        -e POSTGRES_DB=olympus \
        -p 5432:5432 \
        postgres:15-alpine
    echo -e "${GREEN}✅ PostgreSQL started${NC}"
else
    echo -e "${GREEN}✅ PostgreSQL already running${NC}"
fi

# Start Redis
if ! docker ps | grep -q olympus-redis; then
    docker run -d \
        --name olympus-redis \
        -p 6379:6379 \
        redis:7-alpine
    echo -e "${GREEN}✅ Redis started${NC}"
else
    echo -e "${GREEN}✅ Redis already running${NC}"
fi

# Wait for databases to be ready
echo -e "\n${YELLOW}⏳ Waiting for databases to be ready...${NC}"
sleep 5

# Check PostgreSQL connection
until docker exec olympus-postgres pg_isready -U olympus > /dev/null 2>&1; do
    echo -n "."
    sleep 1
done
echo -e "\n${GREEN}✅ PostgreSQL is ready${NC}"

# Check Redis connection
until docker exec olympus-redis redis-cli ping > /dev/null 2>&1; do
    echo -n "."
    sleep 1
done
echo -e "${GREEN}✅ Redis is ready${NC}"

# Run database migrations
echo -e "\n${YELLOW}📦 Running database migrations...${NC}"
if [ -d "migrations" ]; then
    sqlx migrate run || echo -e "${YELLOW}⚠️  Migrations might have already been applied${NC}"
else
    echo -e "${YELLOW}⚠️  No migrations directory found${NC}"
fi

# Build the project
echo -e "\n${YELLOW}🔨 Building the project...${NC}"
cargo build

# Run tests
echo -e "\n${YELLOW}🧪 Running tests...${NC}"
cargo test || echo -e "${YELLOW}⚠️  Some tests may require additional setup${NC}"

# Display next steps
echo -e "\n${GREEN}🎉 Setup complete!${NC}"
echo -e "\n${CYAN}Next steps:${NC}"
echo "1. Run the development server:"
echo "   ${GREEN}cargo run${NC} or ${GREEN}make dev${NC}"
echo ""
echo "2. Run with auto-reload:"
echo "   ${GREEN}cargo watch -x run${NC}"
echo ""
echo "3. Access the services:"
echo "   - Health check: ${CYAN}http://localhost:8000/health${NC}"
echo "   - API docs: ${CYAN}http://localhost:8000/api/v1/docs${NC}"
echo ""
echo "4. View available Make commands:"
echo "   ${GREEN}make help${NC}"
echo ""
echo "5. Stop databases when done:"
echo "   ${GREEN}docker stop olympus-postgres olympus-redis${NC}"

echo -e "\n${CYAN}Happy coding! 🚀${NC}"