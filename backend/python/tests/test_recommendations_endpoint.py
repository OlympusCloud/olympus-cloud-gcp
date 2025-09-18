import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings
from app.models.analytics import AnalyticsDashboardResponse, AnalyticsMetrics, AnalyticsTimeframe
from app.services.ml.recommendation import RecommendationService


class StubAnalyticsService:
    async def get_dashboard_metrics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id=None,
        start_date=None,
        end_date=None,
    ) -> AnalyticsDashboardResponse:
        return AnalyticsDashboardResponse(
            tenant_id=tenant_id,
            metrics=AnalyticsMetrics(
                active_users=150,
                orders=12,
                revenue=48000.0,
                inventory_warnings=2,
                timeframe=timeframe,
            ),
        )


@pytest.mark.asyncio
async def test_recommendations_endpoint_returns_payload(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    analytics_service = StubAnalyticsService()
    app.state.analytics_service = analytics_service
    app.state.recommendation_service = RecommendationService(analytics_service)

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/recommendations",
            params={"tenant_id": "tenant-xyz", "limit": 1},
        )

    assert response.status_code == 200
    payload = response.json()

    assert payload["tenant_id"] == "tenant-xyz"
    assert len(payload["recommendations"]) == 1
    recommendation = payload["recommendations"][0]
    assert recommendation["category"] in {"inventory", "marketing", "operations"}
    assert "insights" in recommendation

    get_settings.cache_clear()
