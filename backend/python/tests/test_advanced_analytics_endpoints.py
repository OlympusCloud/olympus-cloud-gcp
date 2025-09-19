"""API tests for advanced analytics endpoints."""

from datetime import datetime

import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings
from app.models.enhanced_analytics import (
    CohortAnalysis,
    CohortAnalyticsResponse,
    CohortPeriodMetrics,
    ForecastData,
    TimeSeriesPoint,
)


class StubCohortService:
    async def generate_cohort_analysis(self, tenant_id: str, **kwargs):  # noqa: ANN001
        return CohortAnalyticsResponse(
            tenant_id=tenant_id,
            period_granularity="month",
            period_labels=["Period 0", "Period 1"],
            cohorts=[
                CohortAnalysis(
                    cohort_key="2024-01",
                    cohort_size=10,
                    periods=[
                        CohortPeriodMetrics(
                            period_index=0,
                            period_label="Period 0",
                            customers_active=10,
                            revenue=1000.0,
                            retention_rate=1.0,
                        ),
                        CohortPeriodMetrics(
                            period_index=1,
                            period_label="Period 1",
                            customers_active=7,
                            revenue=650.0,
                            retention_rate=0.7,
                        ),
                    ],
                    average_retention=0.85,
                    lifetime_value=1650.0,
                )
            ],
            average_retention_rate=0.85,
            best_cohort="2024-01",
        )


class StubForecastingService:
    async def revenue_forecast(self, tenant_id: str, **kwargs):  # noqa: ANN001
        return ForecastData(
            metric_name="revenue",
            forecast_points=[
                TimeSeriesPoint(
                    timestamp=datetime(2024, 7, 1),
                    value=250.0,
                    label="Forecast 1",
                )
            ],
            confidence_interval={"upper": [275.0], "lower": [225.0]},
            accuracy_score=0.92,
            model_used="linear_trend",
        )


@pytest.mark.asyncio
async def test_get_cohort_analytics_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.cohort_service = StubCohortService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/cohorts",
            params={"tenant_id": "tenant-123", "granularity": "month", "periods": 2},
        )

    assert response.status_code == 200
    payload = response.json()
    assert payload["tenant_id"] == "tenant-123"
    assert payload["cohorts"][0]["cohort_key"] == "2024-01"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_get_revenue_forecast_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.forecasting_service = StubForecastingService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/forecast",
            params={"tenant_id": "tenant-123", "periods": 1},
        )

    assert response.status_code == 200
    payload = response.json()
    assert payload["metric_name"] == "revenue"
    assert payload["forecast_points"][0]["value"] == 250.0

    get_settings.cache_clear()
