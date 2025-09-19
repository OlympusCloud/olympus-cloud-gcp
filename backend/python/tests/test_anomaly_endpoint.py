"""Tests for analytics anomaly endpoint."""

from datetime import datetime

import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings


class StubAnomalyService:
    async def detect_revenue_anomalies(self, tenant_id, **kwargs):  # noqa: ANN001
        return [
            {
                "timestamp": datetime(2024, 1, 4).isoformat(),
                "value": 500.0,
                "z_score": 3.2,
                "severity": 3.2,
            }
        ]


@pytest.mark.asyncio
async def test_get_analytics_anomalies_returns_payload(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.anomaly_service = StubAnomalyService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get(
            "/api/analytics/anomalies",
            params={"tenant_id": "tenant-123"},
        )

    assert response.status_code == 200
    payload = response.json()
    assert len(payload) == 1
    assert payload[0]["value"] == 500.0

    get_settings.cache_clear()
