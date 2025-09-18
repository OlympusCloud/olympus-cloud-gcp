# OpenAI Codex - Python Business Logic & AI/ML Lead

> **Your Mission**: Build intelligent business systems that understand, learn, and predict using Python's rich AI/ML ecosystem

## 🎯 Your Primary Responsibilities

### AI-Powered Business Intelligence
- **Analytics Engine**: Real-time business metrics and predictive analytics
- **Natural Language Processing**: Command parsing and intelligent suggestions
- **Machine Learning**: Recommendation systems and behavioral analysis
- **Data Pipeline**: ETL processes and BigQuery integration
- **Business Logic**: Core business rules and workflow automation

### Your Work Environment
```bash
# Your dedicated workspace
cd /Users/scotthoughton/olympus-cloud/olympus-repos/olympus-cloud-gcp
git worktree add -b feat/python-logic worktree-codex
cd worktree-codex/backend/python
```

## 🐍 Python Development Standards

### Project Structure (YOU MUST CREATE)
```
backend/python/
├── pyproject.toml              # Modern Python packaging
├── requirements.txt            # Dependencies
├── requirements-dev.txt        # Development dependencies
├── .python-version            # Python version (3.11)
├── src/
│   └── olympus/               # Main package
│       ├── __init__.py
│       ├── analytics/         # Business analytics
│       │   ├── __init__.py
│       │   ├── models/        # Data models
│       │   ├── services/      # Business logic
│       │   ├── repositories/  # Data access
│       │   └── api/           # FastAPI endpoints
│       ├── ai/                # AI/ML services
│       │   ├── __init__.py
│       │   ├── nlp/           # Natural language processing
│       │   ├── ml/            # Machine learning models
│       │   ├── recommendations/ # Recommendation engine
│       │   └── predictions/   # Predictive analytics
│       ├── integrations/      # External service integrations
│       │   ├── __init__.py
│       │   ├── bigquery/      # BigQuery operations
│       │   ├── gcp/           # GCP services
│       │   └── webhooks/      # Webhook handlers
│       ├── shared/            # Shared utilities
│       │   ├── __init__.py
│       │   ├── database/      # Database utilities
│       │   ├── events/        # Event handling
│       │   ├── security/      # Security utilities
│       │   └── utils/         # Common utilities
│       └── main.py            # FastAPI application
├── tests/                     # Test suite
│   ├── unit/
│   ├── integration/
│   └── e2e/
├── scripts/                   # Utility scripts
├── docs/                      # Documentation
└── data/                      # Sample data and schemas
```

### Required Dependencies
```toml
# pyproject.toml
[build-system]
requires = ["setuptools>=68.0", "wheel"]
build-backend = "setuptools.build_meta"

[project]
name = "olympus-ai"
version = "1.0.0"
description = "Olympus Cloud AI and Business Logic Services"
authors = [{name = "OpenAI Codex", email = "codex@olympuscloud.io"}]
license = {text = "Proprietary"}
requires-python = ">=3.11"

dependencies = [
    # Web Framework
    "fastapi==0.109.0",
    "uvicorn[standard]==0.27.0",
    "pydantic==2.5.3",
    "pydantic-settings==2.1.0",
    
    # Database
    "sqlalchemy==2.0.25",
    "asyncpg==0.29.0",
    "alembic==1.13.1",
    
    # Data Processing
    "pandas==2.1.4",
    "numpy==1.26.3",
    "polars==0.20.3",
    "pyarrow==14.0.2",
    
    # Machine Learning
    "scikit-learn==1.4.0",
    "xgboost==2.0.3",
    "lightgbm==4.3.0",
    "optuna==3.5.0",
    
    # AI/NLP
    "transformers==4.36.2",
    "torch==2.1.2",
    "sentence-transformers==2.2.2",
    "spacy==3.7.2",
    "openai==1.9.0",
    "anthropic==0.11.0",
    
    # GCP Integration
    "google-cloud-bigquery==3.14.1",
    "google-cloud-aiplatform==1.40.0",
    "google-cloud-storage==2.14.0",
    "google-cloud-secretmanager==2.18.1",
    
    # Caching & Queuing
    "redis==5.0.1",
    "celery==5.3.6",
    "aiocache==0.12.2",
    
    # HTTP & API
    "httpx==0.26.0",
    "aiohttp==3.9.1",
    
    # Utilities
    "tenacity==8.2.3",
    "python-dateutil==2.8.2",
    "pytz==2023.3",
    "pydash==7.0.6",
    "structlog==23.2.0",
    
    # Monitoring
    "prometheus-client==0.19.0",
    "sentry-sdk[fastapi]==1.39.2",
]

[project.optional-dependencies]
dev = [
    "pytest==7.4.4",
    "pytest-asyncio==0.23.2",
    "pytest-cov==4.1.0",
    "pytest-mock==3.12.0",
    "httpx==0.26.0",
    "factory-boy==3.3.0",
    "freezegun==1.4.0",
    
    # Code Quality
    "black==23.12.1",
    "isort==5.13.2",
    "flake8==7.0.0",
    "mypy==1.8.0",
    "bandit==1.7.6",
    "pre-commit==3.6.0",
    
    # Documentation
    "mkdocs==1.5.3",
    "mkdocs-material==9.5.3",
]

[tool.setuptools.packages.find]
where = ["src"]

[tool.black]
line-length = 88
target-version = ['py311']

[tool.isort]
profile = "black"
multi_line_output = 3

[tool.mypy]
python_version = "3.11"
warn_return_any = true
warn_unused_configs = true
disallow_untyped_defs = true
```

## 🧠 AI-Powered Analytics Engine

### Core Analytics Service
```python
# src/olympus/analytics/services/analytics_service.py
from datetime import datetime, timedelta
from typing import List, Dict, Any, Optional
from dataclasses import dataclass
import pandas as pd
import numpy as np
from sqlalchemy.ext.asyncio import AsyncSession
from sqlalchemy import text

from olympus.shared.database import get_db_session
from olympus.analytics.models.metrics import BusinessMetric, MetricType
from olympus.ai.predictions.service import PredictionService

@dataclass
class AnalyticsQuery:
    tenant_id: str
    metric_type: MetricType
    start_date: datetime
    end_date: datetime
    filters: Dict[str, Any] = None
    group_by: List[str] = None

@dataclass
class AnalyticsResult:
    data: pd.DataFrame
    insights: List[str]
    predictions: Optional[Dict[str, Any]] = None
    confidence: float = 0.0

class AnalyticsService:
    def __init__(
        self, 
        db_session: AsyncSession,
        prediction_service: PredictionService
    ):
        self.db = db_session
        self.prediction_service = prediction_service
    
    async def get_business_metrics(
        self, 
        query: AnalyticsQuery
    ) -> AnalyticsResult:
        """Get comprehensive business metrics with AI insights."""
        
        # Execute base query
        data = await self._execute_metrics_query(query)
        
        # Generate AI insights
        insights = await self._generate_insights(data, query)
        
        # Get predictions if requested
        predictions = None
        if query.metric_type in [MetricType.REVENUE, MetricType.ORDERS]:
            predictions = await self.prediction_service.predict_metrics(
                data, query.metric_type
            )
        
        return AnalyticsResult(
            data=data,
            insights=insights,
            predictions=predictions,
            confidence=self._calculate_confidence(data)
        )
    
    async def _execute_metrics_query(
        self, 
        query: AnalyticsQuery
    ) -> pd.DataFrame:
        """Execute optimized SQL query for metrics."""
        
        sql_template = """
        WITH daily_metrics AS (
            SELECT 
                DATE(created_at) as metric_date,
                COUNT(*) as count,
                SUM(CASE WHEN status = 'completed' THEN total_amount ELSE 0 END) as revenue,
                AVG(total_amount) as avg_order_value,
                COUNT(DISTINCT customer_id) as unique_customers
            FROM commerce.orders 
            WHERE tenant_id = :tenant_id 
                AND created_at >= :start_date 
                AND created_at <= :end_date
            GROUP BY DATE(created_at)
            ORDER BY metric_date
        )
        SELECT * FROM daily_metrics
        """
        
        result = await self.db.execute(
            text(sql_template),
            {
                'tenant_id': query.tenant_id,
                'start_date': query.start_date,
                'end_date': query.end_date
            }
        )
        
        # Convert to pandas DataFrame
        df = pd.DataFrame(result.fetchall(), columns=result.keys())
        return df
    
    async def _generate_insights(
        self, 
        data: pd.DataFrame, 
        query: AnalyticsQuery
    ) -> List[str]:
        """Generate AI-powered business insights."""
        insights = []
        
        if len(data) < 2:
            return ["Insufficient data for analysis"]
        
        # Trend analysis
        revenue_trend = self._analyze_trend(data['revenue'])
        if revenue_trend['direction'] == 'increasing':
            insights.append(
                f"Revenue is trending upward with {revenue_trend['growth_rate']:.1%} growth"
            )
        elif revenue_trend['direction'] == 'decreasing':
            insights.append(
                f"Revenue is declining at {abs(revenue_trend['growth_rate']):.1%} rate"
            )
        
        # Seasonal patterns
        seasonal_insights = self._detect_seasonality(data)
        insights.extend(seasonal_insights)
        
        # Anomaly detection
        anomalies = self._detect_anomalies(data)
        if anomalies:
            insights.append(f"Detected {len(anomalies)} unusual patterns in the data")
        
        return insights
    
    def _analyze_trend(self, series: pd.Series) -> Dict[str, Any]:
        """Analyze trend direction and growth rate."""
        if len(series) < 2:
            return {'direction': 'unknown', 'growth_rate': 0}
        
        # Simple linear regression for trend
        x = np.arange(len(series))
        y = series.values
        
        # Remove NaN values
        mask = ~np.isnan(y)
        if np.sum(mask) < 2:
            return {'direction': 'unknown', 'growth_rate': 0}
        
        x = x[mask]
        y = y[mask]
        
        slope = np.polyfit(x, y, 1)[0]
        growth_rate = slope / np.mean(y) if np.mean(y) != 0 else 0
        
        direction = 'increasing' if slope > 0 else 'decreasing' if slope < 0 else 'stable'
        
        return {
            'direction': direction,
            'growth_rate': growth_rate,
            'slope': slope
        }
    
    def _detect_seasonality(self, data: pd.DataFrame) -> List[str]:
        """Detect seasonal patterns in the data."""
        insights = []
        
        if len(data) < 7:
            return insights
        
        # Day of week analysis
        data['day_of_week'] = pd.to_datetime(data['metric_date']).dt.day_name()
        
        dow_revenue = data.groupby('day_of_week')['revenue'].mean()
        best_day = dow_revenue.idxmax()
        worst_day = dow_revenue.idxmin()
        
        if dow_revenue[best_day] > dow_revenue[worst_day] * 1.2:
            insights.append(
                f"Strong weekly pattern: {best_day} is the best performing day"
            )
        
        return insights
    
    def _detect_anomalies(self, data: pd.DataFrame) -> List[Dict[str, Any]]:
        """Detect anomalies using statistical methods."""
        anomalies = []
        
        for column in ['revenue', 'count']:
            if column not in data.columns:
                continue
                
            values = data[column].dropna()
            if len(values) < 3:
                continue
            
            # Z-score method
            z_scores = np.abs((values - values.mean()) / values.std())
            anomaly_threshold = 2.5
            
            anomaly_indices = np.where(z_scores > anomaly_threshold)[0]
            
            for idx in anomaly_indices:
                anomalies.append({
                    'date': data.iloc[idx]['metric_date'],
                    'metric': column,
                    'value': values.iloc[idx],
                    'z_score': z_scores.iloc[idx]
                })
        
        return anomalies
    
    def _calculate_confidence(self, data: pd.DataFrame) -> float:
        """Calculate confidence score based on data quality."""
        if len(data) == 0:
            return 0.0
        
        # Factors affecting confidence
        data_points = len(data)
        completeness = 1 - (data.isnull().sum().sum() / (len(data) * len(data.columns)))
        
        # More data points and higher completeness = higher confidence
        confidence = min(1.0, (data_points / 30) * completeness)
        
        return confidence
```

### Natural Language Processing Service
```python
# src/olympus/ai/nlp/command_processor.py
from typing import Dict, Any, List, Optional
from dataclasses import dataclass
from enum import Enum
import re
import spacy
from transformers import pipeline

class CommandType(Enum):
    QUERY = "query"
    ACTION = "action"
    NAVIGATION = "navigation"
    UNKNOWN = "unknown"

class ActionType(Enum):
    CREATE_ORDER = "create_order"
    VIEW_ANALYTICS = "view_analytics"
    SEARCH_CUSTOMERS = "search_customers"
    UPDATE_INVENTORY = "update_inventory"
    GENERATE_REPORT = "generate_report"

@dataclass
class ParsedCommand:
    intent: CommandType
    action: Optional[ActionType]
    entities: Dict[str, Any]
    confidence: float
    response_suggestion: str

class NaturalLanguageProcessor:
    def __init__(self):
        # Load spaCy model for entity recognition
        self.nlp = spacy.load("en_core_web_sm")
        
        # Load sentiment analysis pipeline
        self.sentiment_analyzer = pipeline(
            "sentiment-analysis",
            model="cardiffnlp/twitter-roberta-base-sentiment-latest"
        )
        
        # Command patterns
        self.patterns = {
            ActionType.CREATE_ORDER: [
                r"create.*order",
                r"new order",
                r"add.*order",
                r"make.*order"
            ],
            ActionType.VIEW_ANALYTICS: [
                r"show.*analytics",
                r"view.*reports?",
                r"analytics.*dashboard",
                r"sales.*data",
                r"revenue.*report"
            ],
            ActionType.SEARCH_CUSTOMERS: [
                r"find.*customer",
                r"search.*customer",
                r"customer.*lookup",
                r"who is.*customer"
            ],
            ActionType.UPDATE_INVENTORY: [
                r"update.*inventory",
                r"stock.*level",
                r"inventory.*count",
                r"add.*stock"
            ],
            ActionType.GENERATE_REPORT: [
                r"generate.*report",
                r"create.*report",
                r"export.*data",
                r"download.*report"
            ]
        }
    
    async def process_command(self, text: str) -> ParsedCommand:
        """Process natural language command and extract intent."""
        
        # Clean and normalize text
        cleaned_text = self._preprocess_text(text)
        
        # Extract entities
        entities = self._extract_entities(cleaned_text)
        
        # Determine intent and action
        intent, action, confidence = self._classify_intent(cleaned_text)
        
        # Generate response suggestion
        response = self._generate_response_suggestion(intent, action, entities)
        
        return ParsedCommand(
            intent=intent,
            action=action,
            entities=entities,
            confidence=confidence,
            response_suggestion=response
        )
    
    def _preprocess_text(self, text: str) -> str:
        """Clean and normalize input text."""
        # Convert to lowercase
        text = text.lower().strip()
        
        # Remove extra whitespace
        text = re.sub(r'\s+', ' ', text)
        
        # Handle common contractions
        contractions = {
            "can't": "cannot",
            "won't": "will not",
            "n't": " not",
            "'ll": " will",
            "'re": " are",
            "'ve": " have"
        }
        
        for contraction, expansion in contractions.items():
            text = text.replace(contraction, expansion)
        
        return text
    
    def _extract_entities(self, text: str) -> Dict[str, Any]:
        """Extract named entities and key information."""
        doc = self.nlp(text)
        
        entities = {
            'dates': [],
            'money': [],
            'numbers': [],
            'products': [],
            'customers': [],
            'locations': []
        }
        
        for ent in doc.ents:
            if ent.label_ in ["DATE", "TIME"]:
                entities['dates'].append(ent.text)
            elif ent.label_ == "MONEY":
                entities['money'].append(ent.text)
            elif ent.label_ in ["CARDINAL", "QUANTITY"]:
                entities['numbers'].append(ent.text)
            elif ent.label_ == "PERSON":
                entities['customers'].append(ent.text)
            elif ent.label_ in ["GPE", "LOC"]:
                entities['locations'].append(ent.text)
            elif ent.label_ == "PRODUCT":
                entities['products'].append(ent.text)
        
        # Extract time periods
        time_keywords = ['today', 'yesterday', 'week', 'month', 'year', 'quarter']
        for keyword in time_keywords:
            if keyword in text:
                entities['time_period'] = keyword
                break
        
        return entities
    
    def _classify_intent(self, text: str) -> tuple[CommandType, Optional[ActionType], float]:
        """Classify the intent of the command."""
        best_action = None
        best_confidence = 0.0
        
        # Check action patterns
        for action_type, patterns in self.patterns.items():
            for pattern in patterns:
                if re.search(pattern, text):
                    confidence = self._calculate_pattern_confidence(pattern, text)
                    if confidence > best_confidence:
                        best_confidence = confidence
                        best_action = action_type
        
        # Determine intent type
        if best_action:
            if best_action in [ActionType.CREATE_ORDER, ActionType.UPDATE_INVENTORY]:
                intent = CommandType.ACTION
            elif best_action in [ActionType.VIEW_ANALYTICS, ActionType.GENERATE_REPORT]:
                intent = CommandType.QUERY
            else:
                intent = CommandType.NAVIGATION
        else:
            intent = CommandType.UNKNOWN
            best_confidence = 0.1
        
        return intent, best_action, best_confidence
    
    def _calculate_pattern_confidence(self, pattern: str, text: str) -> float:
        """Calculate confidence score for pattern match."""
        # Simple confidence based on pattern specificity and text length
        pattern_words = len(pattern.split())
        text_words = len(text.split())
        
        # More specific patterns and shorter text = higher confidence
        specificity_score = min(1.0, pattern_words / max(1, text_words - 2))
        base_confidence = 0.7
        
        return base_confidence + (specificity_score * 0.3)
    
    def _generate_response_suggestion(
        self, 
        intent: CommandType, 
        action: Optional[ActionType], 
        entities: Dict[str, Any]
    ) -> str:
        """Generate a helpful response suggestion."""
        
        if intent == CommandType.UNKNOWN:
            return "I'm not sure what you'd like to do. Try asking about orders, customers, or analytics."
        
        if action == ActionType.CREATE_ORDER:
            if entities.get('customers'):
                customer = entities['customers'][0]
                return f"I'll help you create a new order for {customer}. What items would you like to add?"
            else:
                return "I'll help you create a new order. Which customer is this for?"
        
        elif action == ActionType.VIEW_ANALYTICS:
            time_period = entities.get('time_period', 'recent')
            return f"Here are your {time_period} analytics. What specific metrics would you like to see?"
        
        elif action == ActionType.SEARCH_CUSTOMERS:
            if entities.get('customers'):
                customer = entities['customers'][0]
                return f"Searching for customer: {customer}"
            else:
                return "I'll help you find a customer. What's their name or email?"
        
        elif action == ActionType.UPDATE_INVENTORY:
            if entities.get('products'):
                product = entities['products'][0]
                return f"I'll help you update inventory for {product}. What's the new stock level?"
            else:
                return "I'll help you update inventory. Which product are you working with?"
        
        elif action == ActionType.GENERATE_REPORT:
            return "I'll generate a report for you. What time period and metrics would you like to include?"
        
        return "I'm ready to help! What would you like to do next?"
```

## 📋 Your Daily Development Workflow

### Morning Routine (MANDATORY)
```bash
# 1. Sync with main and other agents
cd worktree-codex
git pull origin main
git merge main

# 2. Check coordination docs
cat docs/daily-status.md
cat docs/integration-points.md

# 3. Update your status in docs/daily-status.md

# 4. Activate virtual environment and start services
cd backend/python
source venv/bin/activate
make dev-python
# This runs: uvicorn olympus.main:app --reload --port 8001
```

### Development Cycle
```bash
# Test-driven development
pytest tests/unit/ -v
pytest tests/integration/ -v

# Type checking and linting
mypy src/
flake8 src/
black src/
isort src/

# Security scanning
bandit -r src/

# Run specific ML training or analysis
python scripts/train_recommendation_model.py
python scripts/analyze_customer_behavior.py

# Commit frequently (every 1-2 hours)
git add -p
git commit -m "codex(analytics): implement seasonal pattern detection"
```

### Evening Integration
```bash
# Generate documentation
mkdocs build

# Update dependencies if needed
pip-compile requirements.in

# Push your work
git push origin feat/python-logic

# Update status in docs/daily-status.md
```

## 🎯 Week 1 Implementation Priorities

### Day 1: Project Foundation
```bash
# 1. Setup Python project structure
mkdir -p src/olympus/{analytics,ai,integrations,shared}

# 2. Configure development environment
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
pip install -r requirements-dev.txt

# 3. Setup FastAPI application
# Create main.py with basic FastAPI app

# 4. Database connection setup
# Implement database utilities in shared/database/
```

### Day 2: Analytics Foundation
```python
# Implement these modules in order:
# 1. Analytics models (analytics/models/)
# 2. Database repositories (analytics/repositories/)
# 3. Analytics service (analytics/services/)
# 4. FastAPI endpoints (analytics/api/)
```

### Day 3: AI/ML Services
```python
# Create:
# 1. Natural language processor (ai/nlp/)
# 2. Basic recommendation engine (ai/recommendations/)
# 3. Prediction service foundation (ai/predictions/)
```

### Day 4: Integration & Testing
```bash
# 1. BigQuery integration for data warehousing
# 2. Redis integration for caching
# 3. Integration tests with real data
# 4. Performance optimization
```

## 🔗 Critical Integration Points

### Database Integration with Claude Code
```python
# src/olympus/shared/database/connection.py
from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.orm import sessionmaker
from typing import AsyncGenerator
import os

DATABASE_URL = os.getenv("DATABASE_URL", "postgresql+asyncpg://olympus:devpassword@localhost:5432/olympus")

engine = create_async_engine(
    DATABASE_URL,
    echo=bool(os.getenv("DEBUG", False)),
    pool_size=20,
    max_overflow=0,
    pool_pre_ping=True,
)

async_session_maker = sessionmaker(
    engine, 
    class_=AsyncSession, 
    expire_on_commit=False
)

async def get_db_session() -> AsyncGenerator[AsyncSession, None]:
    async with async_session_maker() as session:
        try:
            yield session
            await session.commit()
        except Exception:
            await session.rollback()
            raise
        finally:
            await session.close()
```

### API Integration with ChatGPT's Go Gateway
```python
# src/olympus/analytics/api/analytics_endpoints.py
from fastapi import APIRouter, Depends, HTTPException, Query
from sqlalchemy.ext.asyncio import AsyncSession
from typing import List, Optional
from datetime import datetime, timedelta

from olympus.shared.database import get_db_session
from olympus.analytics.services.analytics_service import AnalyticsService, AnalyticsQuery
from olympus.analytics.models.schemas import AnalyticsResponse
from olympus.ai.predictions.service import PredictionService

router = APIRouter(prefix="/analytics", tags=["analytics"])

@router.get("/dashboard", response_model=AnalyticsResponse)
async def get_dashboard_analytics(
    tenant_id: str = Query(..., description="Tenant ID"),
    days: int = Query(30, description="Number of days to analyze"),
    db: AsyncSession = Depends(get_db_session)
):
    """Get comprehensive dashboard analytics."""
    
    end_date = datetime.utcnow()
    start_date = end_date - timedelta(days=days)
    
    analytics_service = AnalyticsService(
        db_session=db,
        prediction_service=PredictionService()
    )
    
    query = AnalyticsQuery(
        tenant_id=tenant_id,
        metric_type=MetricType.REVENUE,
        start_date=start_date,
        end_date=end_date
    )
    
    try:
        result = await analytics_service.get_business_metrics(query)
        
        return AnalyticsResponse(
            data=result.data.to_dict('records'),
            insights=result.insights,
            predictions=result.predictions,
            confidence=result.confidence,
            period=f"{days} days",
            generated_at=datetime.utcnow()
        )
    
    except Exception as e:
        raise HTTPException(
            status_code=500,
            detail=f"Failed to generate analytics: {str(e)}"
        )

@router.post("/query")
async def execute_natural_language_query(
    query: str,
    tenant_id: str,
    db: AsyncSession = Depends(get_db_session)
):
    """Execute a natural language analytics query."""
    
    from olympus.ai.nlp.command_processor import NaturalLanguageProcessor
    
    nlp_processor = NaturalLanguageProcessor()
    parsed_command = await nlp_processor.process_command(query)
    
    if parsed_command.action == ActionType.VIEW_ANALYTICS:
        # Convert natural language to analytics query
        analytics_query = await _convert_nlp_to_analytics_query(
            parsed_command, tenant_id
        )
        
        analytics_service = AnalyticsService(
            db_session=db,
            prediction_service=PredictionService()
        )
        
        result = await analytics_service.get_business_metrics(analytics_query)
        
        return {
            "query": query,
            "interpretation": parsed_command.response_suggestion,
            "data": result.data.to_dict('records'),
            "insights": result.insights,
            "confidence": parsed_command.confidence
        }
    
    else:
        return {
            "query": query,
            "interpretation": parsed_command.response_suggestion,
            "confidence": parsed_command.confidence,
            "suggested_action": "This query requires action outside of analytics"
        }
```

### BigQuery Integration for Data Warehousing
```python
# src/olympus/integrations/bigquery/warehouse_service.py
from google.cloud import bigquery
from google.cloud.bigquery import LoadJobConfig, WriteDisposition
import pandas as pd
from typing import List, Dict, Any
import os

class DataWarehouseService:
    def __init__(self):
        self.client = bigquery.Client()
        self.dataset_id = os.getenv("BIGQUERY_DATASET", "olympus_analytics_dev")
        
    async def sync_order_data(self, tenant_id: str, orders_data: List[Dict[str, Any]]):
        """Sync order data to BigQuery for analytics."""
        
        table_id = f"{self.dataset_id}.orders"
        
        # Convert to DataFrame
        df = pd.DataFrame(orders_data)
        
        # Add metadata
        df['synced_at'] = pd.Timestamp.utcnow()
        df['tenant_id'] = tenant_id
        
        # Load to BigQuery
        job_config = LoadJobConfig(
            write_disposition=WriteDisposition.WRITE_APPEND,
            autodetect=True
        )
        
        job = self.client.load_table_from_dataframe(
            df, table_id, job_config=job_config
        )
        
        job.result()  # Wait for job to complete
        
        return f"Synced {len(orders_data)} orders to BigQuery"
    
    async def run_analytics_query(self, sql_query: str) -> pd.DataFrame:
        """Execute analytics query against BigQuery."""
        
        # Add safety checks for SQL injection
        if any(keyword in sql_query.upper() for keyword in ['DROP', 'DELETE', 'INSERT', 'UPDATE']):
            raise ValueError("Only SELECT queries are allowed")
        
        query_job = self.client.query(sql_query)
        results = query_job.result()
        
        # Convert to pandas DataFrame
        df = results.to_dataframe()
        return df
    
    async def get_customer_insights(self, tenant_id: str) -> Dict[str, Any]:
        """Get AI-powered customer insights from BigQuery."""
        
        sql = f"""
        WITH customer_metrics AS (
            SELECT 
                customer_id,
                COUNT(*) as order_count,
                SUM(total_amount) as total_spent,
                AVG(total_amount) as avg_order_value,
                DATE_DIFF(CURRENT_DATE(), MAX(DATE(created_at)), DAY) as days_since_last_order,
                STDDEV(total_amount) as order_value_variance
            FROM `{self.dataset_id}.orders`
            WHERE tenant_id = '{tenant_id}'
                AND status = 'completed'
            GROUP BY customer_id
        ),
        customer_segments AS (
            SELECT 
                *,
                CASE 
                    WHEN total_spent > 1000 AND days_since_last_order < 30 THEN 'VIP_Active'
                    WHEN total_spent > 500 AND days_since_last_order < 60 THEN 'High_Value'
                    WHEN days_since_last_order > 90 THEN 'At_Risk'
                    WHEN order_count = 1 THEN 'New_Customer'
                    ELSE 'Regular'
                END as segment
            FROM customer_metrics
        )
        SELECT 
            segment,
            COUNT(*) as customer_count,
            AVG(total_spent) as avg_total_spent,
            AVG(order_count) as avg_order_count,
            AVG(avg_order_value) as avg_order_value
        FROM customer_segments
        GROUP BY segment
        ORDER BY avg_total_spent DESC
        """
        
        df = await self.run_analytics_query(sql)
        
        return {
            'segments': df.to_dict('records'),
            'total_customers': df['customer_count'].sum(),
            'insights': self._generate_customer_insights(df)
        }
    
    def _generate_customer_insights(self, df: pd.DataFrame) -> List[str]:
        """Generate insights from customer segmentation data."""
        insights = []
        
        if len(df) == 0:
            return ["No customer data available for analysis"]
        
        # VIP analysis
        vip_customers = df[df['segment'] == 'VIP_Active']
        if not vip_customers.empty:
            vip_count = vip_customers['customer_count'].iloc[0]
            vip_value = vip_customers['avg_total_spent'].iloc[0]
            insights.append(
                f"You have {vip_count} VIP customers with average spend of ${vip_value:.2f}"
            )
        
        # At-risk analysis
        at_risk = df[df['segment'] == 'At_Risk']
        if not at_risk.empty:
            at_risk_count = at_risk['customer_count'].iloc[0]
            total_customers = df['customer_count'].sum()
            risk_percentage = (at_risk_count / total_customers) * 100
            
            if risk_percentage > 20:
                insights.append(
                    f"Warning: {risk_percentage:.1f}% of customers are at risk of churning"
                )
        
        # New customer analysis
        new_customers = df[df['segment'] == 'New_Customer']
        if not new_customers.empty:
            new_count = new_customers['customer_count'].iloc[0]
            insights.append(
                f"You have {new_count} new customers - consider follow-up campaigns"
            )
        
        return insights
```

## 🧪 Testing Standards (MANDATORY)

### Unit Testing
```python
# tests/unit/analytics/test_analytics_service.py
import pytest
import pandas as pd
from datetime import datetime, timedelta
from unittest.mock import Mock, AsyncMock

from olympus.analytics.services.analytics_service import (
    AnalyticsService, 
    AnalyticsQuery, 
    MetricType
)
from olympus.ai.predictions.service import PredictionService

@pytest.fixture
def mock_db_session():
    session = Mock()
    session.execute = AsyncMock()
    return session

@pytest.fixture
def mock_prediction_service():
    return Mock(spec=PredictionService)

@pytest.fixture
def analytics_service(mock_db_session, mock_prediction_service):
    return AnalyticsService(mock_db_session, mock_prediction_service)

@pytest.fixture
def sample_data():
    return pd.DataFrame({
        'metric_date': ['2024-01-01', '2024-01-02', '2024-01-03'],
        'revenue': [1000, 1200, 800],
        'count': [10, 12, 8],
        'avg_order_value': [100, 100, 100],
        'unique_customers': [8, 10, 6]
    })

@pytest.mark.asyncio
async def test_analytics_service_basic_metrics(analytics_service, sample_data):
    """Test basic analytics metrics calculation."""
    
    # Mock database response
    mock_result = Mock()
    mock_result.fetchall.return_value = sample_data.to_dict('records')
    mock_result.keys.return_value = sample_data.columns
    
    analytics_service.db.execute.return_value = mock_result
    
    query = AnalyticsQuery(
        tenant_id="test-tenant",
        metric_type=MetricType.REVENUE,
        start_date=datetime(2024, 1, 1),
        end_date=datetime(2024, 1, 3)
    )
    
    result = await analytics_service.get_business_metrics(query)
    
    assert len(result.data) == 3
    assert result.confidence > 0
    assert len(result.insights) > 0

def test_trend_analysis(analytics_service):
    """Test trend analysis functionality."""
    
    # Test increasing trend
    increasing_series = pd.Series([100, 120, 140, 160])
    trend = analytics_service._analyze_trend(increasing_series)
    
    assert trend['direction'] == 'increasing'
    assert trend['growth_rate'] > 0
    
    # Test decreasing trend
    decreasing_series = pd.Series([160, 140, 120, 100])
    trend = analytics_service._analyze_trend(decreasing_series)
    
    assert trend['direction'] == 'decreasing'
    assert trend['growth_rate'] < 0

def test_anomaly_detection(analytics_service, sample_data):
    """Test anomaly detection algorithm."""
    
    # Add an anomaly
    anomaly_data = sample_data.copy()
    anomaly_data.loc[len(anomaly_data)] = {
        'metric_date': '2024-01-04',
        'revenue': 5000,  # Anomalously high
        'count': 50,
        'avg_order_value': 100,
        'unique_customers': 45
    }
    
    anomalies = analytics_service._detect_anomalies(anomaly_data)
    
    assert len(anomalies) > 0
    assert any(a['metric'] == 'revenue' for a in anomalies)

@pytest.mark.asyncio
async def test_nlp_command_processing():
    """Test natural language command processing."""
    
    from olympus.ai.nlp.command_processor import NaturalLanguageProcessor
    
    nlp = NaturalLanguageProcessor()
    
    # Test analytics query
    result = await nlp.process_command("show me last month's sales analytics")
    
    assert result.action == ActionType.VIEW_ANALYTICS
    assert result.confidence > 0.5
    assert 'analytics' in result.response_suggestion.lower()
    
    # Test order creation
    result = await nlp.process_command("create a new order for John Smith")
    
    assert result.action == ActionType.CREATE_ORDER
    assert 'John Smith' in result.entities.get('customers', [])
```

## 🏁 Success Criteria

### Week 1 Deliverables
- [ ] Python project structure with proper packaging
- [ ] FastAPI application with analytics endpoints
- [ ] Natural language processing for command interpretation
- [ ] Basic machine learning models for recommendations
- [ ] BigQuery integration for data warehousing
- [ ] Redis integration for caching and sessions
- [ ] Database integration with Claude Code's schema
- [ ] API integration with ChatGPT's Go gateway
- [ ] Comprehensive test suite with >80% coverage
- [ ] AI-powered business insights generation

### Quality Gates
- [ ] `pytest` - All tests pass
- [ ] `mypy .` - No type errors
- [ ] `flake8 .` - No style violations
- [ ] `bandit -r .` - No security issues
- [ ] `black --check .` - Code properly formatted
- [ ] Performance benchmarks within targets

### AI/ML Capabilities
- [ ] Natural language command processing working
- [ ] Basic recommendation engine operational
- [ ] Anomaly detection in business metrics
- [ ] Predictive analytics for revenue forecasting
- [ ] Customer segmentation and insights
- [ ] Real-time analytics processing

**Remember**: You're the brain of the entire platform. Your AI capabilities will differentiate Olympus from every other business management system. Make it smart, make it intuitive, make it predictive.

**Your motto**: *"Intelligence that understands business."*