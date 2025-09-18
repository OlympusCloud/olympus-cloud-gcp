# Python Business Logic - OpenAI Codex Agent

## Overview
This service delivers Olympus Cloud's analytics, AI/ML, and natural language experiences. It aggregates operational data, powers dashboards, and interprets conversational queries for decision makers.

## Owner
**OpenAI Codex** – Python business logic & analytics

## Features
- Real-time analytics pipelines
- Natural language query interpretation
- Recommendation and forecasting foundations
- BigQuery data warehousing integrations
- Event-driven architecture (Redis pub/sub)
- Async PostgreSQL access with SQLAlchemy

## Quick Start
```bash
# Create virtual environment
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run in development
uvicorn main:app --reload --port 8001

# Run tests (ensure pytest is installed)
python -m pytest
```

## Service Ports
- **Python Analytics API**: 8001
- **OpenAPI docs**: 8001/docs

## Integration Points
- **PostgreSQL** via async SQLAlchemy engine (operational analytics)
- **BigQuery** for warehousing and long-term analytics
- **Redis** pub/sub for domain event ingestion
- **Go API Gateway** (port 8080) as upstream consumer

## Directory Structure
```
backend/python/
├── app/
│   ├── api/               # API routers and dependency wiring
│   ├── core/              # Config, logging, lifespan, database helpers
│   ├── models/            # Pydantic schemas (events, DTOs)
│   ├── services/          # Business logic modules
│   │   ├── analytics/     # Metric aggregation & BigQuery helpers
│   │   ├── events/        # Redis subscriber orchestration
│   │   ├── ml/            # ML & forecasting pipelines (future)
│   │   └── nlp/           # Natural language query service
│   └── utils/             # Shared utilities (placeholder)
├── tests/                # Async API and service tests
├── main.py               # FastAPI entry point
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

## Analytics Dashboard Filters
- `GET /api/analytics/dashboard/{tenant_id}?timeframe=` supports `all_time`, `today`, `yesterday`, `this_week`, `last_week`, `this_month`, `last_month`, `year_to_date`.

## Current Implementation Highlights
- Modular FastAPI application with shared startup/shutdown lifecycle
- Redis pub/sub subscriber wiring analytics processor
- Async SQLAlchemy session factory for PostgreSQL access
- BigQuery client wrapper with safe local fallbacks
- `/api/analytics/dashboard/{tenant_id}?timeframe=…` returns timeframe-aware metrics
- `/api/analytics/nlp/query` endpoint providing heuristic natural language interpretation
- `/api/health` endpoint exposing runtime status including Redis connectivity

## Next Steps for OpenAI Codex
1. Model concrete analytics schemas and persistence strategy
2. Persist events in PostgreSQL and stream to BigQuery for warehousing
3. Expand Redis processing pipelines with background workers
4. Integrate Vertex AI / OpenAI for advanced NLP interpretation
5. Implement recommendation and forecasting services
6. Add integration tests covering event ingestion and cross-service flows
