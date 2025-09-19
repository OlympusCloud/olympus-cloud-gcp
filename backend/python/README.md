# Python Business Logic - OpenAI Codex Agent

## Overview
This is the Python service responsible for analytics, AI/ML capabilities, natural language processing, and business intelligence features.

## Owner
**OpenAI Codex** - Responsible for Python business logic and analytics

## Features
- Real-time analytics engine
- Natural language processing
- Recommendation system
- Predictive analytics
- BigQuery integration
- Event-driven analytics

## Quick Start

```bash
# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run in development
uvicorn main:app --reload --port 8001

# Run tests (ensure pytest is installed in your environment)
python -m pytest
```

## Service Ports
- **Python Analytics**: 8001
- **API Docs**: 8001/docs

## Integration Points
- **PostgreSQL**: Direct connection for analytics queries
- **BigQuery**: Data warehousing and batch analytics
- **Redis**: Subscribe to domain events
- **Go API Gateway**: Receives forwarded analytics requests

## Directory Structure
```
backend/python/
├── app/
│   ├── api/               # API endpoints (e.g. /api/health)
│   ├── core/              # Configuration, logging, lifespan management
│   ├── models/            # Pydantic models and schemas
│   ├── services/          # Business logic
│   │   ├── analytics/     # Analytics engine and BigQuery helpers
│   │   ├── events/        # Redis event subscribers
│   │   ├── ml/            # Machine learning pipelines
│   │   └── nlp/           # Natural language services
│   └── utils/             # Shared utilities
├── tests/                # Async API tests
├── main.py              # FastAPI entry point
└── requirements.txt
```

## Environment Variables
```
PORT=8001
DATABASE_URL=postgresql://olympus:devpassword@localhost:5432/olympus
REDIS_URL=redis://localhost:6379
BIGQUERY_PROJECT_ID=your-project-id
BIGQUERY_DATASET=olympus_analytics
OPENAI_API_KEY=your-api-key
```

## Analytics Events to Process
- `events.user.logged_in` – Session analytics
- `events.order.created` – Commerce metrics
- `events.payment.processed` – Revenue tracking
- `events.inventory.updated` – Stock health monitoring

## Current Implementation Highlights
- Modular FastAPI application with shared startup/shutdown lifecycle
- Redis pub/sub subscriber wiring analytics processor
- Async SQLAlchemy session factory for PostgreSQL access
- BigQuery client wrapper with safe local fallbacks
- `/api/analytics/dashboard/{tenant_id}?timeframe=…` returns timeframe-aware metrics
- `/api/analytics/nlp/query` endpoint providing heuristic natural language interpretation
- `/api/health` endpoint exposing runtime status including Redis connectivity

## Next Steps for OpenAI Codex
1. Expand analytics data models with concrete schema mappings
2. Persist and aggregate events inside PostgreSQL and BigQuery
3. Implement Redis event handler pipelines and background tasks
4. Build NLP service for natural language queries
5. Create recommendation engine foundation
6. Add integration tests covering event ingestion and analytics APIs
