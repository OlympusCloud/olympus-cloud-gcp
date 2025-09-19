"""Unit tests for churn prediction service."""

from __future__ import annotations

from datetime import datetime, timedelta
from types import SimpleNamespace
from typing import Sequence

import pytest

from app.models.churn import CustomerChurnPrediction
from app.services.ml.churn import ChurnPredictionService


class _Result:
    def __init__(self, rows: Sequence[SimpleNamespace]) -> None:
        self._rows = rows

    def fetchall(self) -> Sequence[SimpleNamespace]:
        return self._rows


class _Session:
    def __init__(self, rows: Sequence[SimpleNamespace]) -> None:
        self._rows = rows

    async def __aenter__(self) -> "_Session":
        return self

    async def __aexit__(self, exc_type, exc, tb) -> None:  # noqa: ANN001
        return None

    async def execute(self, query, params):  # noqa: ANN001
        return _Result(self._rows)


class _Factory:
    def __init__(self, rows: Sequence[SimpleNamespace]) -> None:
        self._rows = rows

    def __call__(self) -> _Session:
        return _Session(self._rows)


@pytest.mark.asyncio
async def test_predict_scores_customers() -> None:
    now = datetime.utcnow()
    rows = [
        SimpleNamespace(
            customer_id="cust-1",
            email="user@example.com",
            last_order_at=now - timedelta(days=10),
            total_orders=5,
            recent_orders=2,
            total_revenue=300.0,
            avg_order_value=60.0,
        ),
        SimpleNamespace(
            customer_id="cust-2",
            email=None,
            last_order_at=now - timedelta(days=200),
            total_orders=1,
            recent_orders=0,
            total_revenue=50.0,
            avg_order_value=50.0,
        ),
    ]

    service = ChurnPredictionService(_Factory(rows))
    response = await service.predict("tenant-1", limit=10)

    assert response.summary.high_risk >= 1
    assert len(response.predictions) == 2

    high_risk = next(pred for pred in response.predictions if pred.customer_id == "cust-2")
    assert isinstance(high_risk, CustomerChurnPrediction)
    assert high_risk.risk_level == "high"
    assert high_risk.signals
