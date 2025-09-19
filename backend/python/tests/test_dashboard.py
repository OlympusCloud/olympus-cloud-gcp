from datetime import date
from typing import Optional

import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings
from app.models.analytics import (
    AnalyticsDashboardResponse,
    AnalyticsMetrics,
    AnalyticsTimeframe,
    CustomerMetrics,
    InventoryMetrics,
    SalesForecast,
    SalesMetrics,
    SalesSnapshot,
    SalesTrend,
    StaffMetrics,
)


def _stub_metrics(timeframe: AnalyticsTimeframe) -> AnalyticsMetrics:
    sales_today = SalesSnapshot(revenue=123.45, orders=2, avg_order=61.725)
    sales = SalesMetrics(today=sales_today, trend=SalesTrend(revenue=123.45, orders=2), forecast=SalesForecast())
    customers = CustomerMetrics(total=5, new_today=0, active=5)
    inventory = InventoryMetrics(low_stock_items=1, stockout_risk=1.0, total_value=0.0)
    staff = StaffMetrics()
    return AnalyticsMetrics(
        timeframe=timeframe,
        sales=sales,
        customers=customers,
        inventory=inventory,
        staff=staff,
    )


class StubAnalyticsService:
    async def get_dashboard_metrics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> AnalyticsDashboardResponse:
        return AnalyticsDashboardResponse(
            tenant_id=tenant_id,
            metrics=_stub_metrics(timeframe),
        )


@pytest.mark.asyncio
async def test_dashboard_endpoint_returns_metrics(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.analytics_service = StubAnalyticsService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/dashboard", params={"tenant_id": "tenant-123"}
        )

    assert response.status_code == 200
    payload = response.json()
    assert payload["tenant_id"] == "tenant-123"
    assert payload["metrics"]["customers"]["active"] == 5
    assert payload["metrics"]["sales"]["today"]["orders"] == 2
    assert payload["metrics"]["timeframe"] == "all_time"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_dashboard_endpoint_accepts_timeframe(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    captured: dict[str, str] = {}

    class TimeframeAwareStub(StubAnalyticsService):
        async def get_dashboard_metrics(
            self,
            tenant_id: str,
            timeframe: AnalyticsTimeframe,
            *,
            location_id: Optional[str] = None,
            start_date: Optional[date] = None,
            end_date: Optional[date] = None,
        ) -> AnalyticsDashboardResponse:
            captured["tenant"] = tenant_id
            captured["timeframe"] = timeframe.value
            return await super().get_dashboard_metrics(
                tenant_id,
                timeframe,
                location_id=location_id,
                start_date=start_date,
                end_date=end_date,
            )

    app = create_app()
    app.state.analytics_service = TimeframeAwareStub()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/dashboard",
            params={"tenant_id": "tenant-123", "date_range": "today"},
        )

    assert response.status_code == 200
    payload = response.json()
    assert payload["metrics"]["timeframe"] == "today"
    assert captured == {"tenant": "tenant-123", "timeframe": "today"}

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_dashboard_endpoint_requires_custom_bounds(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.analytics_service = StubAnalyticsService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/dashboard",
            params={"tenant_id": "tenant-123", "date_range": "custom"},
        )

    assert response.status_code == 400

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_dashboard_endpoint_forwards_location_and_dates(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    captured: dict[str, object] = {}

    class RecordingStub(StubAnalyticsService):
        async def get_dashboard_metrics(
            self,
            tenant_id: str,
            timeframe: AnalyticsTimeframe,
            *,
            location_id: Optional[str] = None,
            start_date: Optional[date] = None,
            end_date: Optional[date] = None,
        ) -> AnalyticsDashboardResponse:
            captured.update(
                tenant=tenant_id,
                timeframe=timeframe.value,
                location=location_id,
                start=start_date,
                end=end_date,
            )
            return await super().get_dashboard_metrics(
                tenant_id,
                timeframe,
                location_id=location_id,
                start_date=start_date,
                end_date=end_date,
            )

    app = create_app()
    app.state.analytics_service = RecordingStub()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/dashboard",
            params={
                "tenant_id": "tenant-123",
                "date_range": "custom",
                "from_date": "2024-01-01",
                "to_date": "2024-01-31",
                "location_id": "location-42",
            },
        )

    assert response.status_code == 200
    assert captured["tenant"] == "tenant-123"
    assert captured["timeframe"] == "custom"
    assert captured["location"] == "location-42"
    assert captured["start"] == date(2024, 1, 1)
    assert captured["end"] == date(2024, 1, 31)

    get_settings.cache_clear()
