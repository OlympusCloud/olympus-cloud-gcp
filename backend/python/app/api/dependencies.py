from fastapi import HTTPException, Request, status

from app.services.analytics.service import AnalyticsService
from app.services.analytics.enhanced_service import EnhancedAnalyticsService
from app.services.analytics.snapshots import SnapshotService
from app.services.crm.service import CRMService
from app.services.inventory.service import InventoryService
from app.services.ml.recommendation import RecommendationService
from app.services.nlp.query_service import NaturalLanguageQueryService


def get_analytics_service(request: Request) -> AnalyticsService:
    service = getattr(request.app.state, "analytics_service", None)
    if service is None:
        raise HTTPException(status_code=status.HTTP_503_SERVICE_UNAVAILABLE, detail="Analytics service unavailable")
    return service


def get_nlp_service(request: Request) -> NaturalLanguageQueryService:
    service = getattr(request.app.state, "nlp_service", None)
    if service is None:
        raise HTTPException(status_code=status.HTTP_503_SERVICE_UNAVAILABLE, detail="NLP service unavailable")
    return service


def get_recommendation_service(request: Request) -> RecommendationService:
    service = getattr(request.app.state, "recommendation_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Recommendation service unavailable",
        )
    return service


def get_snapshot_service(request: Request) -> SnapshotService:
    service = getattr(request.app.state, "snapshot_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Snapshot service unavailable",
        )
    return service


def get_enhanced_analytics_service(request: Request) -> EnhancedAnalyticsService:
    service = getattr(request.app.state, "enhanced_analytics_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Enhanced analytics service unavailable",
        )
    return service


def get_crm_service(request: Request) -> CRMService:
    service = getattr(request.app.state, "crm_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CRM service unavailable",
        )
    return service


def get_inventory_service(request: Request) -> InventoryService:
    service = getattr(request.app.state, "inventory_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Inventory service unavailable",
        )
    return service
