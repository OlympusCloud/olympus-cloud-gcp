from __future__ import annotations

from datetime import date, datetime
from typing import Any, List, Optional

from fastapi import APIRouter, Depends, HTTPException, Query, Request, status
from pydantic import BaseModel, Field

from app.api.dependencies import (
    get_analytics_service,
    get_crm_service,
    get_enhanced_analytics_service,
    get_events_service,
    get_hospitality_service,
    get_inventory_service,
    get_nlp_service,
    get_recommendation_service,
    get_restaurant_service,
    get_retail_service,
    get_snapshot_service,
)
from app.core.settings import get_settings
from app.core.state import RuntimeState
from app.models.analytics import AnalyticsDashboardResponse, AnalyticsTimeframe
from app.models.crm import Campaign
from app.models.enhanced_analytics import (
    EnhancedAnalyticsResponse,
    AnalyticsFilter
)
from app.models.inventory import StockMovement
from app.models.nlp import NLPQueryResponse
from app.models.recommendations import RecommendationResponse
from app.models.restaurant import RestaurantAnalytics, RestaurantRecommendation
from app.models.retail import RetailAnalytics, RetailRecommendation
from app.models.hospitality import HospitalityAnalytics, HospitalityRecommendation
import app.models.events_industry as events_models
from app.models.events_industry import EventsAnalytics, EventsRecommendation


class _EventsModelsShim:
    EventsAnalytics = EventsAnalytics
    EventsRecommendation = EventsRecommendation


events_models = _EventsModelsShim()
EventsAnalytics = events_models.EventsAnalytics
EventsRecommendation = events_models.EventsRecommendation
EventsAnalytics = events_models.EventsAnalytics
EventsRecommendation = events_models.EventsRecommendation
from app.models.snapshots import (
    MetricsSnapshot,
    SnapshotHistoryRequest,
    SnapshotHistoryResponse
)
from app.services.analytics.service import AnalyticsService
from app.services.analytics.enhanced_service import EnhancedAnalyticsService
from app.services.analytics.snapshots import SnapshotService
from app.services.crm.service import CRMService
from app.services.inventory.service import InventoryService
from app.services.ml.recommendation import RecommendationContext, RecommendationService
from app.services.nlp.query_service import NaturalLanguageQueryService
from app.services.restaurant.service import RestaurantService
from app.services.retail.service import RetailService
from app.services.hospitality.service import HospitalityService
from app.services.events_industry.service import EventsService

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


@api_router.get(
    "/crm/segments",
    tags=["crm"]
)
async def get_customer_segments(
    tenant_id: str = Query(..., description="Tenant identifier"),
    crm_service: CRMService = Depends(get_crm_service),
):
    """Get customer segmentation analysis."""
    return await crm_service.segment_customers(tenant_id)


@api_router.post(
    "/crm/campaigns",
    tags=["crm"],
    status_code=status.HTTP_201_CREATED
)
async def create_campaign(
    campaign: Campaign,
    crm_service: CRMService = Depends(get_crm_service),
) -> Campaign:
    """Create a new marketing campaign."""
    return await crm_service.create_campaign(campaign)


@api_router.get(
    "/inventory/report",
    tags=["inventory"]
)
async def get_inventory_report(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: Optional[str] = Query(None, description="Location filter"),
    inventory_service: InventoryService = Depends(get_inventory_service),
):
    """Get comprehensive inventory report with forecasts."""
    return await inventory_service.get_inventory_report(tenant_id, location_id)


@api_router.post(
    "/inventory/movements",
    tags=["inventory"],
    status_code=status.HTTP_201_CREATED
)
async def track_stock_movement(
    movement: StockMovement,
    inventory_service: InventoryService = Depends(get_inventory_service),
) -> StockMovement:
    """Track inventory stock movement."""
    return await inventory_service.track_stock_movement(movement)


@api_router.get(
    "/restaurant/analytics",
    tags=["restaurant"],
    response_model=RestaurantAnalytics
)
async def get_restaurant_analytics(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: Optional[str] = Query(None, description="Location filter"),
    restaurant_service: RestaurantService = Depends(get_restaurant_service),
) -> RestaurantAnalytics:
    """Get comprehensive restaurant analytics including table turnover and service metrics."""
    return await restaurant_service.get_table_analytics(tenant_id, location_id)


@api_router.get(
    "/restaurant/recommendations",
    tags=["restaurant"]
)
async def get_restaurant_recommendations(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: Optional[str] = Query(None, description="Location filter"),
    restaurant_service: RestaurantService = Depends(get_restaurant_service),
) -> List[RestaurantRecommendation]:
    """Get AI-powered restaurant operation recommendations."""
    return await restaurant_service.generate_restaurant_recommendations(tenant_id, location_id)


@api_router.get(
    "/restaurant/kitchen/orders",
    tags=["restaurant"]
)
async def get_kitchen_display_orders(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: str = Query(..., description="Location identifier"),
    restaurant_service: RestaurantService = Depends(get_restaurant_service),
):
    """Get orders for kitchen display system."""
    return await restaurant_service.get_kitchen_display_orders(tenant_id, location_id)


@api_router.get(
    "/restaurant/tables/status",
    tags=["restaurant"]
)
async def get_table_status(
    tenant_id: str = Query(..., description="Tenant identifier"),
    location_id: str = Query(..., description="Location identifier"),
    restaurant_service: RestaurantService = Depends(get_restaurant_service),
):
    """Get current table status distribution."""
    return await restaurant_service.get_table_status(tenant_id, location_id)


@api_router.get(
    "/retail/analytics",
    tags=["retail"],
    response_model=RetailAnalytics,
)
async def get_retail_analytics(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    retail_service: RetailService = Depends(get_retail_service),
) -> RetailAnalytics:
    """Return retail analytics including channel and promotion performance."""

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

    return await retail_service.get_retail_analytics(
        tenant_id,
        timeframe,
        location_id=location_id,
        start_date=from_date,
        end_date=to_date,
    )


@api_router.get(
    "/retail/promotions",
    tags=["retail"],
    response_model=List[RetailRecommendation],
)
async def get_retail_recommendations(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    retail_service: RetailService = Depends(get_retail_service),
) -> List[RetailRecommendation]:
    """Generate retail-specific promotional recommendations."""

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

    return await retail_service.generate_promotions(
        tenant_id,
        timeframe,
        location_id=location_id,
        start_date=from_date,
        end_date=to_date,
    )


@api_router.get(
    "/hospitality/analytics",
    tags=["hospitality"],
    response_model=HospitalityAnalytics,
)
async def get_hospitality_analytics(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    hospitality_service: HospitalityService = Depends(get_hospitality_service),
) -> HospitalityAnalytics:
    """Return hospitality analytics including room, booking, and service metrics."""

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

    return await hospitality_service.get_hospitality_analytics(
        tenant_id,
        timeframe,
        location_id=location_id,
        start_date=from_date,
        end_date=to_date,
    )


@api_router.get(
    "/hospitality/recommendations",
    tags=["hospitality"],
    response_model=List[HospitalityRecommendation],
)
async def get_hospitality_recommendations(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    hospitality_service: HospitalityService = Depends(get_hospitality_service),
) -> List[HospitalityRecommendation]:
    """Generate hospitality-specific operational recommendations."""

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

    return await hospitality_service.generate_recommendations(
        tenant_id,
        timeframe,
        location_id=location_id,
        start_date=from_date,
        end_date=to_date,
    )


@api_router.get(
    "/events/analytics",
    tags=["events"],
    response_model=EventsAnalytics,
)
async def get_events_analytics(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    events_service: EventsService = Depends(get_events_service),
) -> EventsAnalytics:
    """Return events analytics including ticket and vendor performance."""

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

    start_dt = datetime.combine(from_date, datetime.min.time()) if from_date else None
    end_dt = datetime.combine(to_date, datetime.max.time()) if to_date else None

    return await events_service.get_events_analytics(
        tenant_id,
        timeframe,
        location_id=location_id,
        start_date=start_dt,
        end_date=end_dt,
    )


@api_router.get(
    "/events/recommendations",
    tags=["events"],
    response_model=List[EventsRecommendation],
)
async def get_events_recommendations(
    tenant_id: str = Query(..., description="Tenant identifier"),
    date_range: Optional[str] = Query(None, description="Predefined range selector"),
    location_id: Optional[str] = Query(None, description="Optional location filter"),
    from_date: Optional[date] = Query(None, description="Custom range start (if date_range=custom)"),
    to_date: Optional[date] = Query(None, description="Custom range end (if date_range=custom)"),
    events_service: EventsService = Depends(get_events_service),
) -> List[EventsRecommendation]:
    """Generate events-specific operational recommendations."""

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

    start_dt = datetime.combine(from_date, datetime.min.time()) if from_date else None
    end_dt = datetime.combine(to_date, datetime.max.time()) if to_date else None

    return await events_service.generate_recommendations(
        tenant_id,
        timeframe,
        location_id=location_id,
        start_date=start_dt,
        end_date=end_dt,
    )
