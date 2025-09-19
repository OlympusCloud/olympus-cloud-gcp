import pytest

from app.models.analytics import AnalyticsDashboardResponse, AnalyticsMetrics, AnalyticsTimeframe
from app.services.ml.recommendation import RecommendationContext, RecommendationService


class StubAnalyticsService:
    def __init__(self, metrics: AnalyticsMetrics) -> None:
        self._metrics = metrics

    async def get_dashboard_metrics(  # noqa: D401 - match service signature
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id=None,
        start_date=None,
        end_date=None,
    ) -> AnalyticsDashboardResponse:
        return AnalyticsDashboardResponse(tenant_id=tenant_id, metrics=self._metrics)


@pytest.mark.asyncio
async def test_generate_produces_inventory_and_marketing_recommendations():
    metrics = AnalyticsMetrics(
        active_users=200,
        orders=10,
        revenue=5000.0,
        inventory_warnings=3,
        timeframe=AnalyticsTimeframe.THIS_WEEK,
    )
    service = RecommendationService(StubAnalyticsService(metrics))

    response = await service.generate(
        RecommendationContext(tenant_id="tenant-1", timeframe=AnalyticsTimeframe.THIS_WEEK)
    )

    actions = {rec.action for rec in response.recommendations}
    assert any(rec.category.value == "inventory" for rec in response.recommendations)
    assert any(rec.category.value == "marketing" for rec in response.recommendations)
    assert "Prioritize replenishment for at-risk items" in actions
    assert response.tenant_id == "tenant-1"


@pytest.mark.asyncio
async def test_generate_respects_limit_parameter():
    metrics = AnalyticsMetrics(
        active_users=500,
        orders=200,
        revenue=150000.0,
        inventory_warnings=0,
        timeframe=AnalyticsTimeframe.THIS_MONTH,
    )
    service = RecommendationService(StubAnalyticsService(metrics))

    response = await service.generate(
        RecommendationContext(
            tenant_id="tenant-2",
            timeframe=AnalyticsTimeframe.THIS_MONTH,
            limit=1,
        )
    )

    assert len(response.recommendations) == 1
    assert response.recommendations[0].priority.value in {"medium", "low"}
