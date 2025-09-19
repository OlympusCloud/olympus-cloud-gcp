from __future__ import annotations

from datetime import date, datetime
from typing import Any, List, Optional

from fastapi import APIRouter, Depends, HTTPException, Query, Request, status
from pydantic import BaseModel, Field

from app.api.dependencies import (
    get_analytics_service,
    get_enhanced_analytics_service,
    get_nlp_service,
    get_recommendation_service,
    get_snapshot_service,
)
from app.core.settings import get_settings
from app.core.state import RuntimeState
from app.models.analytics import AnalyticsDashboardResponse, AnalyticsTimeframe
from app.models.enhanced_analytics import (
    EnhancedAnalyticsResponse,
    AnalyticsFilter
)
from app.models.nlp import NLPQueryResponse
from app.models.recommendations import RecommendationResponse
from app.models.snapshots import (
    MetricsSnapshot,
    SnapshotHistoryRequest,
    SnapshotHistoryResponse
)
from app.services.analytics.service import AnalyticsService
from app.services.analytics.enhanced_service import EnhancedAnalyticsService
from app.services.analytics.snapshots import SnapshotService
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


@api_router.post(
    "/analytics/snapshots",
    tags=["analytics"],
    response_model=MetricsSnapshot,
    status_code=status.HTTP_201_CREATED
)
async def create_metrics_snapshot(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    timeframe: AnalyticsTimeframe = Query(AnalyticsTimeframe.TODAY, description="Timeframe for snapshot"),
    snapshot_service: SnapshotService = Depends(get_snapshot_service),
) -> MetricsSnapshot:
    """Create a new metrics snapshot for the specified tenant."""
    
    return await snapshot_service.create_snapshot(
        tenant_id=tenant_id,
        timeframe=timeframe,
        location_id=location_id
    )


@api_router.get(
    "/analytics/snapshots/history",
    tags=["analytics"],
    response_model=SnapshotHistoryResponse
)
async def get_snapshot_history(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    timeframe: AnalyticsTimeframe = Query(AnalyticsTimeframe.LAST_MONTH, description="Timeframe filter"),
    limit: int = Query(100, ge=1, le=1000, description="Maximum number of snapshots to return"),
    snapshot_service: SnapshotService = Depends(get_snapshot_service),
) -> SnapshotHistoryResponse:
    """Retrieve historical metrics snapshots with trend analysis."""
    
    request = SnapshotHistoryRequest(
        tenant_id=tenant_id,
        location_id=location_id,
        timeframe=timeframe,
        limit=limit
    )
    
    return await snapshot_service.get_snapshot_history(request)


@api_router.post(
    "/analytics/snapshots/backfill",
    tags=["analytics"],
    status_code=status.HTTP_202_ACCEPTED
)
async def backfill_snapshots(
    tenant_id: str = Query(..., description="Tenant identifier"),
    days_back: int = Query(30, ge=1, le=365, description="Number of days to backfill"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    snapshot_service: SnapshotService = Depends(get_snapshot_service),
) -> dict[str, Any]:
    """Backfill historical snapshots for the specified tenant."""
    
    snapshots_created = await snapshot_service.backfill_snapshots(
        tenant_id=tenant_id,
        days_back=days_back,
        location_id=location_id
    )
    
    return {
        "message": "Backfill completed",
        "tenant_id": tenant_id,
        "snapshots_created": snapshots_created,
        "days_back": days_back
    }


@api_router.get(
    "/analytics/enhanced/dashboard",
    tags=["analytics"],
    response_model=EnhancedAnalyticsResponse
)
async def get_enhanced_dashboard(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_ids: Optional[List[str]] = Query(None, description="Location filters"),
    timeframe: AnalyticsTimeframe = Query(AnalyticsTimeframe.THIS_MONTH, description="Analysis timeframe"),
    start_date: Optional[datetime] = Query(None, description="Custom start date"),
    end_date: Optional[datetime] = Query(None, description="Custom end date"),
    include_trends: bool = Query(True, description="Include trend analysis"),
    include_forecasts: bool = Query(False, description="Include predictive analytics"),
    enhanced_service: EnhancedAnalyticsService = Depends(get_enhanced_analytics_service),
) -> EnhancedAnalyticsResponse:
    """Get comprehensive dashboard with enhanced metrics and insights."""
    
    analytics_filter = AnalyticsFilter(
        tenant_id=tenant_id,
        location_ids=location_ids,
        timeframe=timeframe,
        start_date=start_date,
        end_date=end_date,
        include_trends=include_trends,
        include_forecasts=include_forecasts
    )
    
    return await enhanced_service.get_enhanced_dashboard(analytics_filter)
