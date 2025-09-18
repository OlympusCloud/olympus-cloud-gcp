from fastapi import HTTPException, Request, status

from app.services.analytics.service import AnalyticsService
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
