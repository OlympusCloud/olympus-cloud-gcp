from fastapi import HTTPException, Request, status

from app.services.analytics.service import AnalyticsService
from app.services.analytics.enhanced_service import EnhancedAnalyticsService
from app.services.analytics.snapshots import SnapshotService
from app.services.crm.service import CRMService
from app.services.inventory.service import InventoryService
from app.services.retail.service import RetailService
from app.services.hospitality.service import HospitalityService
from app.services.events_industry.service import EventsService
from app.services.ml.recommendation import RecommendationService
from app.services.nlp.enhanced_nlp import EnhancedNLPService
from app.services.nlp.query_service import NaturalLanguageQueryService
from app.services.restaurant.service import RestaurantService


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


def get_enhanced_nlp_service(request: Request) -> EnhancedNLPService:
    service = getattr(request.app.state, "enhanced_nlp_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Enhanced NLP service unavailable",
        )
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


def get_restaurant_service(request: Request) -> RestaurantService:
    service = getattr(request.app.state, "restaurant_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Restaurant service unavailable",
        )
    return service


def get_retail_service(request: Request) -> RetailService:
    service = getattr(request.app.state, "retail_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Retail service unavailable",
        )
    return service


def get_hospitality_service(request: Request) -> HospitalityService:
    service = getattr(request.app.state, "hospitality_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Hospitality service unavailable",
        )
    return service


def get_events_service(request: Request) -> EventsService:
    service = getattr(request.app.state, "events_service", None)
    if service is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Events service unavailable",
        )
    return service
