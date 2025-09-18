from __future__ import annotations

from datetime import date
from typing import Any, Optional

from fastapi import APIRouter, Depends, HTTPException, Query, Request, status
from pydantic import BaseModel, Field

from app.api.dependencies import (
    get_analytics_service,
    get_nlp_service,
    get_recommendation_service,
)
from app.core.settings import get_settings
from app.core.state import RuntimeState
from app.models.analytics import AnalyticsDashboardResponse, AnalyticsTimeframe
from app.models.nlp import NLPQueryResponse
from app.models.recommendations import RecommendationResponse
from app.services.analytics.service import AnalyticsService
from app.services.ml.recommendation import RecommendationContext, RecommendationService
from app.services.nlp.query_service import NaturalLanguageQueryService

api_router = APIRouter()


class NLPQueryRequest(BaseModel):
    query: str = Field(min_length=1, max_length=500, description="Natural language analytics question")


_DATE_RANGE_ALIASES: dict[str, AnalyticsTimeframe] = {
    "today": AnalyticsTimeframe.TODAY,
    "yesterday": AnalyticsTimeframe.YESTERDAY,
    "week": AnalyticsTimeframe.THIS_WEEK,
    "this_week": AnalyticsTimeframe.THIS_WEEK,
    "month": AnalyticsTimeframe.THIS_MONTH,
    "this_month": AnalyticsTimeframe.THIS_MONTH,
    "quarter": AnalyticsTimeframe.THIS_QUARTER,
    "this_quarter": AnalyticsTimeframe.THIS_QUARTER,
    "year": AnalyticsTimeframe.THIS_YEAR,
    "this_year": AnalyticsTimeframe.THIS_YEAR,
    "custom": AnalyticsTimeframe.CUSTOM,
    "all_time": AnalyticsTimeframe.ALL_TIME,
}


def _resolve_timeframe(value: Optional[str]) -> AnalyticsTimeframe:
    if value is None:
        return AnalyticsTimeframe.ALL_TIME

    normalized = value.strip().lower()
    if normalized in _DATE_RANGE_ALIASES:
        return _DATE_RANGE_ALIASES[normalized]

    try:
        return AnalyticsTimeframe(normalized)
    except ValueError as exc:  # noqa: B904
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Invalid date_range value",
        ) from exc


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


@api_router.get("/analytics/dashboard", tags=["analytics"], response_model=AnalyticsDashboardResponse)
async def get_dashboard_metrics(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    analytics_service: AnalyticsService = Depends(get_analytics_service),
) -> AnalyticsDashboardResponse:
    """Return dashboard metrics for the given tenant."""

    timeframe = _resolve_timeframe(date_range)

    if timeframe == AnalyticsTimeframe.CUSTOM:
        if not (from_date and to_date):
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="from_date and to_date are required when date_range is custom",
            )
        if from_date > to_date:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="from_date must be before or equal to to_date",
            )

    return await analytics_service.get_dashboard_metrics(
        tenant_id,
        timeframe=timeframe,
        location_id=location_id,
        start_date=from_date,
        end_date=to_date,
    )


@api_router.get(
    "/analytics/recommendations",
    tags=["analytics"],
    response_model=RecommendationResponse,
)
async def get_recommendations(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    limit: Optional[int] = Query(
        None,
        ge=1,
        le=10,
        description="Maximum number of recommendations to return",
    ),
    recommendation_service: RecommendationService = Depends(get_recommendation_service),
) -> RecommendationResponse:
    """Return AI-assisted recommendations for the tenant."""

    timeframe = _resolve_timeframe(date_range)

    if timeframe == AnalyticsTimeframe.CUSTOM:
        if not (from_date and to_date):
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="from_date and to_date are required when date_range is custom",
            )
        if from_date > to_date:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="from_date must be before or equal to to_date",
            )

    context = RecommendationContext(
        tenant_id=tenant_id,
        timeframe=timeframe,
        location_id=location_id,
        start_date=from_date,
        end_date=to_date,
        limit=limit,
    )

    return await recommendation_service.generate(context)


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
