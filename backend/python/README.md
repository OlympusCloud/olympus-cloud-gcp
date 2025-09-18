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

# Run tests
pytest
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
│   ├── api/               # API endpoints
│   ├── core/              # Core configuration
│   ├── models/            # Data models
│   ├── services/          # Business logic
│   │   ├── analytics/     # Analytics engine
│   │   ├── ml/           # Machine learning
│   │   └── nlp/          # Natural language
│   └── utils/            # Utilities
├── tests/                # Test files
├── alembic/             # Database migrations
├── main.py              # Entry point
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
- `events.user.logged_in` - Track user sessions
- `events.order.created` - Order analytics
- `events.payment.processed` - Revenue tracking
- `events.inventory.updated` - Inventory analytics

## Next Steps for OpenAI Codex
1. Set up FastAPI application structure
2. Create analytics data models
3. Implement Redis event subscriber
4. Build NLP service for natural language queries
5. Create recommendation engine foundation
6. Set up BigQuery connection