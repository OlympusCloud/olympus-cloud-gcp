from __future__ import annotations

from typing import Any

from fastapi import APIRouter, Depends, Request
from pydantic import BaseModel, Field

from app.api.dependencies import get_analytics_service, get_nlp_service
from app.core.settings import get_settings
from app.core.state import RuntimeState
from app.models.analytics import AnalyticsDashboardResponse, AnalyticsTimeframe
from app.models.nlp import NLPQueryResponse
from app.services.analytics.service import AnalyticsService
from app.services.nlp.query_service import NaturalLanguageQueryService

api_router = APIRouter(prefix="/api")


class NLPQueryRequest(BaseModel):
    query: str = Field(min_length=1, max_length=500, description="Natural language analytics question")


@api_router.get("/health", tags=["monitoring"])
async def health_check(request: Request) -> dict[str, Any]:
    """Report service health and runtime information."""

    settings = get_settings()
    runtime_state: RuntimeState = getattr(request.app.state, "runtime", RuntimeState())

    return {
        "status": "ok",
        "service": settings.app_name,
        "environment": settings.environment,
        "version": request.app.version,
        "redis": {
            "connected": runtime_state.redis_connected,
            "subscriber_running": runtime_state.event_subscriber_running,
        },
    }


@api_router.get(
    "/analytics/dashboard/{tenant_id}",
    tags=["analytics"],
    response_model=AnalyticsDashboardResponse,
)
async def get_dashboard_metrics(
    tenant_id: str,
    timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
    analytics_service: AnalyticsService = Depends(get_analytics_service),
) -> AnalyticsDashboardResponse:
    """Return dashboard metrics for the given tenant."""

    return await analytics_service.get_dashboard_metrics(tenant_id, timeframe=timeframe)


@api_router.post(
    "/analytics/nlp/query",
    tags=["analytics"],
    response_model=NLPQueryResponse,
)
async def interpret_nlp_query(
    request_body: NLPQueryRequest,
    nlp_service: NaturalLanguageQueryService = Depends(get_nlp_service),
) -> NLPQueryResponse:
    """Interpret a natural language analytics question."""

    result = await nlp_service.interpret(request_body.query)
    return NLPQueryResponse(query=request_body.query, result=result)
