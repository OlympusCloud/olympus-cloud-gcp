"""Endpoint tests for churn predictions."""

from __future__ import annotations

from datetime import datetime
from uuid import uuid4

import pytest
from fastapi import FastAPI
from httpx import AsyncClient

from app.api.ml_routes import router as ml_router
from app.models.churn import ChurnPredictionResponse, ChurnSummary, CustomerChurnPrediction


class StubChurnService:
    async def predict(self, tenant_id: str, *, limit: int = 50) -> ChurnPredictionResponse:  # noqa: D401, ANN001
        summary = ChurnSummary(high_risk=1, medium_risk=0, low_risk=0)
        prediction = CustomerChurnPrediction(
            customer_id="cust-1",
            email="user@example.com",
            risk_score=0.84,
            risk_level="high",
            last_order_at=datetime(2024, 1, 1),
            total_orders=3,
            recent_orders=0,
            total_revenue=210.0,
            average_order_value=70.0,
            signals=[],
            recommendations=[],
        )
        return ChurnPredictionResponse(tenant_id=tenant_id, summary=summary, predictions=[prediction])


@pytest.mark.asyncio
async def test_churn_predictions_endpoint(monkeypatch):
    app = FastAPI()
    app.include_router(ml_router)
    app.state.churn_service = StubChurnService()

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.get("/api/ml/churn", params={"tenant_id": str(uuid4()), "limit": 5})

    assert response.status_code == 200
    data = response.json()
    assert data["summary"]["high_risk"] == 1
    assert len(data["predictions"]) == 1
