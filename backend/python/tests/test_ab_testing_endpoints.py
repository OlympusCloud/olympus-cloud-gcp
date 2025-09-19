"""Endpoint tests for analytics A/B testing routes."""

from __future__ import annotations

from datetime import datetime
from typing import List
from uuid import uuid4

import pytest
from httpx import AsyncClient

from fastapi import FastAPI
from app.core.settings import get_settings
from app.api.experiments_routes import router as experiments_router
from app.models.experiments import (
    Experiment,
    ExperimentDetail,
    ExperimentResults,
    ExperimentStatus,
    ExperimentSummary,
    ExperimentVariant,
    ExperimentSuccessMetric,
    ParticipantRecord,
    VariantComparison,
    VariantResult,
)


class StubABTestingService:
    def __init__(self) -> None:
        self.recorded_assignments: List = []
        self.recorded_conversions: List = []

    async def list_experiments(self, tenant_id: str) -> List[ExperimentSummary]:
        now = datetime.utcnow()
        return [
            ExperimentSummary(
                id="exp-1",
                name="Homepage CTA",
                status=ExperimentStatus.RUNNING,
                start_date=now,
                end_date=None,
                winner=None,
                conversions=5,
                created_at=now,
            )
        ]

    async def get_experiment_detail(self, tenant_id: str, experiment_id: str):
        if experiment_id != "exp-1":
            return None
        now = datetime.utcnow()
        experiment = Experiment(
            id="exp-1",
            tenant_id=tenant_id,
            name="Homepage CTA",
            description=None,
            hypothesis=None,
            status=ExperimentStatus.RUNNING,
            variants=[
                ExperimentVariant(key="control", name="Control", allocation=0.5, description=None, metadata={}),
                ExperimentVariant(key="variant-b", name="Variant B", allocation=0.5, description=None, metadata={}),
            ],
            success_metrics=[
                ExperimentSuccessMetric(name="conversion_rate", goal="increase", target=None, weight=1.0)
            ],
            traffic_allocation={"Control": 0.5, "Variant B": 0.5},
            start_date=now,
            end_date=None,
            created_by=str(uuid4()),
            created_at=now,
            updated_at=now,
            results={},
        )
        results = ExperimentResults(
            baseline_variant="Control",
            variants=[
                VariantResult(
                    name="Control",
                    participants=100,
                    conversions=10,
                    conversion_rate=0.10,
                    total_conversion_value=500.0,
                    avg_conversion_value=50.0,
                    lift=None,
                ),
                VariantResult(
                    name="Variant B",
                    participants=100,
                    conversions=15,
                    conversion_rate=0.15,
                    total_conversion_value=900.0,
                    avg_conversion_value=60.0,
                    lift=0.5,
                ),
            ],
            comparisons=[
                VariantComparison(
                    baseline="Control",
                    variant="Variant B",
                    lift=0.5,
                    p_value=0.04,
                    confidence=0.96,
                    is_significant=True,
                )
            ],
            suggested_winner="Variant B",
            overall_confidence=0.96,
        )
        return ExperimentDetail(experiment=experiment, results=results)

    async def record_assignment(self, assignment):
        self.recorded_assignments.append(assignment)
        now = datetime.utcnow()
        return ParticipantRecord(
            id=str(uuid4()),
            experiment_id=assignment.experiment_id,
            variant_name=assignment.variant_name,
            user_id=assignment.user_id,
            customer_id=assignment.customer_id,
            session_id=assignment.session_id,
            assigned_at=assignment.assigned_at,
            converted_at=None,
            conversion_value=None,
        )

    async def record_conversion(self, update):
        self.recorded_conversions.append(update)
        now = datetime.utcnow()
        return ParticipantRecord(
            id=str(uuid4()),
            experiment_id="exp-1",
            variant_name="Control",
            user_id=None,
            customer_id=None,
            session_id="session-1",
            assigned_at=now,
            converted_at=update.converted_at,
            conversion_value=update.conversion_value,
        )


@pytest.mark.asyncio
async def test_list_experiments_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = FastAPI()
    app.include_router(experiments_router)
    service = StubABTestingService()
    app.state.ab_testing_service = service

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.get("/api/analytics/experiments", params={"tenant_id": str(uuid4())})

    assert response.status_code == 200
    payload = response.json()
    assert len(payload) == 1
    assert payload[0]["name"] == "Homepage CTA"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_get_experiment_detail_not_found(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = FastAPI()
    app.include_router(experiments_router)
    service = StubABTestingService()
    app.state.ab_testing_service = service

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.get(
            "/api/analytics/experiments/unknown",
            params={"tenant_id": str(uuid4())},
        )

    assert response.status_code == 404
    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_assign_experiment_participant(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = FastAPI()
    app.include_router(experiments_router)
    service = StubABTestingService()
    app.state.ab_testing_service = service

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/api/analytics/experiments/exp-1/participants",
            json={"variant_name": "Control", "session_id": "session-1"},
        )

    assert response.status_code == 201
    assert service.recorded_assignments
    assignment = service.recorded_assignments[0]
    assert assignment.variant_name == "Control"
    assert assignment.session_id == "session-1"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_record_conversion_validates_experiment(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = FastAPI()
    app.include_router(experiments_router)
    service = StubABTestingService()
    app.state.ab_testing_service = service

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/api/analytics/experiments/exp-1/conversions",
            json={"participant_id": str(uuid4()), "conversion_value": 42.0},
        )

    assert response.status_code == 200
    assert service.recorded_conversions

    get_settings.cache_clear()
