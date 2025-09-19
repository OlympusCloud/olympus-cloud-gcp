"""Tests for forecasting service."""

from datetime import datetime
from unittest.mock import MagicMock

import pandas as pd
import pytest

from app.models.enhanced_analytics import ForecastData
from app.services.analytics.forecasting import ForecastingService


@pytest.mark.asyncio
async def test_revenue_forecast_returns_points(monkeypatch):
    service = ForecastingService(session_factory=MagicMock())

    index = pd.date_range("2024-01-01", periods=6, freq="M", tz="UTC")
    df = pd.DataFrame({"revenue": [100, 120, 140, 160, 180, 210]}, index=index)

    async def fake_fetch(*args, **kwargs):  # noqa: ANN001
        return df

    monkeypatch.setattr(service, "_fetch_revenue_series", fake_fetch)

    forecast = await service.revenue_forecast("tenant-1", periods=3, granularity="month")

    assert isinstance(forecast, ForecastData)
    assert forecast.metric_name == "revenue"
    assert len(forecast.forecast_points) == 3
    assert len(forecast.confidence_interval["upper"]) == 3
    assert forecast.accuracy_score >= 0.0


@pytest.mark.asyncio
async def test_revenue_forecast_handles_limited_history(monkeypatch):
    service = ForecastingService(session_factory=MagicMock())

    index = pd.date_range("2024-01-01", periods=1, freq="M", tz="UTC")
    df = pd.DataFrame({"revenue": [100]}, index=index)

    async def fake_fetch(*args, **kwargs):  # noqa: ANN001
        return df

    monkeypatch.setattr(service, "_fetch_revenue_series", fake_fetch)

    forecast = await service.revenue_forecast("tenant-1")
    assert forecast.forecast_points == []
    assert forecast.model_used == "insufficient_data"
