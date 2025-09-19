"""Tests for cohort analytics service."""

from datetime import datetime
from unittest.mock import MagicMock

import pytest

from app.models.enhanced_analytics import CohortAnalyticsResponse
from app.services.analytics.cohort import CohortAnalyticsService


@pytest.mark.asyncio
async def test_cohort_analysis_generates_period_metrics(monkeypatch):
    service = CohortAnalyticsService(session_factory=MagicMock())

    sample_orders = [
        {"customer_id": "c1", "order_date": datetime(2024, 1, 5), "total_amount": 120.0},
        {"customer_id": "c1", "order_date": datetime(2024, 2, 6), "total_amount": 80.0},
        {"customer_id": "c2", "order_date": datetime(2024, 1, 10), "total_amount": 60.0},
        {"customer_id": "c2", "order_date": datetime(2024, 3, 2), "total_amount": 150.0},
        {"customer_id": "c3", "order_date": datetime(2024, 2, 15), "total_amount": 200.0},
        {"customer_id": "c3", "order_date": datetime(2024, 3, 16), "total_amount": 90.0},
    ]

    async def fake_fetch(*args, **kwargs):  # noqa: ANN001
        return sample_orders

    monkeypatch.setattr(service, "_fetch_order_history", fake_fetch)

    result = await service.generate_cohort_analysis(
        "tenant-1",
        granularity="month",
        max_periods=3,
    )

    assert isinstance(result, CohortAnalyticsResponse)
    assert result.tenant_id == "tenant-1"
    assert result.period_granularity == "month"
    assert result.period_labels == ["Period 0", "Period 1", "Period 2"]
    assert len(result.cohorts) == 2

    january_cohort = next(c for c in result.cohorts if c.cohort_key == "2024-01")
    assert january_cohort.cohort_size == 2
    # Period 0 should include all cohort members
    assert january_cohort.periods[0].customers_active == 2
    # Retention rates should be bounded between 0 and 1
    assert 0.0 <= january_cohort.average_retention <= 1.0
    # Lifetime value aggregates revenue
    assert january_cohort.lifetime_value > 0


@pytest.mark.asyncio
async def test_cohort_analysis_empty_dataset(monkeypatch):
    service = CohortAnalyticsService(session_factory=MagicMock())

    async def fake_fetch(*args, **kwargs):  # noqa: ANN001
        return []

    monkeypatch.setattr(service, "_fetch_order_history", fake_fetch)

    result = await service.generate_cohort_analysis("tenant-1")
    assert result.cohorts == []
    assert result.average_retention_rate == 0.0
