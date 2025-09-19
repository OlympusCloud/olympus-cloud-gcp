"""Endpoint tests for A/B testing routes."""  

from __future__ import annotations

from datetime import datetime
from uuid import uuid4

import pytest
from fastapi import FastAPI
from httpx import AsyncClient

from app.api.experiments_routes import router as experiments_router
from app.models.experiments import (
    Experiment,
    ExperimentDefinition,
    ExperimentDetail,
    ExperimentStatus,
    ExperimentSummary,
    ExperimentVariant,
    ParticipantAssignment,
    ParticipantRecord,
)


class StubABTestingService:
    def __init__(self) -> None:
        now = datetime.utcnow()
        self.summary = ExperimentSummary(
            id="exp-1",
            name="Homepage CTA",
            status=ExperimentStatus.RUNNING,
            start_date=now,
            end_date=None,
            winner=None,
            conversions=5,
            created_at=now,
        )
        experiment = Experiment(
            id="exp-1",
            tenant_id="tenant-1",
            name="Homepage CTA",
            description=None,
            hypothesis=None,
            status=ExperimentStatus.RUNNING,
            variants=[
                ExperimentVariant(key="control", name="Control", allocation=0.5, metadata={}),
                ExperimentVariant(key="variant_b", name="Variant B", allocation=0.5, metadata={}),
            ],
            success_metrics=[],
            traffic_allocation={"Control": 0.5, "Variant B": 0.5},
            start_date=now,
            end_date=None,
            created_by="user-1",
            created_at=now,
            updated_at=now,
            results={},
        )
        self.detail = ExperimentDetail(
            experiment=experiment,
            results=None,
        )

    async def list_experiments(self, tenant_id: str):  # noqa: D401
        return [self.summary]

    async def create_experiment(self, definition: ExperimentDefinition):  # noqa: D401
        return Experiment(
            id="exp-2",
            tenant_id=definition.tenant_id,
            name=definition.name,
            description=definition.description,
            hypothesis=definition.hypothesis,
            status=definition.status,
            variants=definition.variants,
            success_metrics=definition.success_metrics,
            traffic_allocation={v.name: v.allocation for v in definition.variants},
            start_date=definition.start_date,
            end_date=definition.end_date,
            created_by=definition.created_by,
            created_at=datetime.utcnow(),
            updated_at=datetime.utcnow(),
            results={},
        )

    async def get_experiment_detail(self, tenant_id: str, experiment_id: str):  # noqa: D401
        return self.detail if experiment_id == "exp-1" else None

    async def record_assignment(self, assignment: ParticipantAssignment):  # noqa: D401
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

    async def record_conversion(self, update):  # noqa: ANN001,D401
        return ParticipantRecord(
            id=str(uuid4()),
            experiment_id="exp-1",
            variant_name="Control",
            user_id=None,
            customer_id=None,
            session_id="session-1",
            assigned_at=datetime.utcnow(),
            converted_at=update.converted_at,
            conversion_value=update.conversion_value,
        )


@pytest.mark.asyncio
async def test_list_experiments_endpoint(monkeypatch):
    app = FastAPI()
    app.include_router(experiments_router)
    app.state.ab_testing_service = StubABTestingService()

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.get("/api/analytics/experiments", params={"tenant_id": str(uuid4())})

    assert response.status_code == 200
    payload = response.json()
    assert payload[0]["name"] == "Homepage CTA"


@pytest.mark.asyncio
async def test_assign_experiment_participant(monkeypatch):
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
    assert service.detail is not None


@pytest.mark.asyncio
async def test_record_experiment_conversion(monkeypatch):
    app = FastAPI()
    app.include_router(experiments_router)
    app.state.ab_testing_service = StubABTestingService()

    async with AsyncClient(app=app, base_url="http://test") as client:
        response = await client.post(
            "/api/analytics/experiments/exp-1/conversions",
            json={"participant_id": str(uuid4()), "conversion_value": 42.0},
        )

    assert response.status_code == 200
