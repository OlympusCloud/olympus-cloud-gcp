"""Tests for the A/B testing analytics service."""

from __future__ import annotations

from datetime import datetime
from types import SimpleNamespace
from unittest.mock import AsyncMock, Mock

import pytest

from app.models.experiments import ExperimentDefinition, ExperimentVariant
from app.services.analytics.ab_testing import ABTestingService


class _MockSessionFactory:
    def __init__(self, experiment_rows=None, participant_rows=None):
        self._experiment_rows = experiment_rows or []
        self._participant_rows = participant_rows or []

    def __call__(self):
        session = AsyncMock()
        session.__aenter__.return_value = session
        session.__aexit__.return_value = AsyncMock()

        async def execute(query, params):  # noqa: ANN001
            if "FROM analytics.experiments" in str(query):
                return _SimpleResult(self._experiment_rows)
            return _SimpleResult(self._participant_rows)

        session.execute.side_effect = execute
        return session


class _SimpleResult:
    def __init__(self, payload):
        self._payload = payload

    def fetchall(self):
        return self._payload

    def one_or_none(self):
        return self._payload[0] if self._payload else None

    def one(self):
        if not self._payload:
            raise RuntimeError("No rows")
        return self._payload[0]


@pytest.mark.asyncio
async def test_list_experiments_returns_summary():
    now = datetime.utcnow()
    experiment_rows = [
        SimpleNamespace(
            id="exp-1",
            tenant_id="tenant-1",
            name="CTA",
            description=None,
            hypothesis=None,
            variants="[]",
            traffic_allocation="{}",
            success_metrics="[]",
            start_date=now,
            end_date=None,
            status="running",
            created_by="user-1",
            created_at=now,
            updated_at=now,
            results="{}",
        )
    ]
    factory = _MockSessionFactory(experiment_rows=experiment_rows)
    service = ABTestingService(factory)

    summaries = await service.list_experiments("tenant-1")

    assert len(summaries) == 1
    assert summaries[0].name == "CTA"


@pytest.mark.asyncio
async def test_create_experiment_persists_definition():
    factory = _MockSessionFactory()
    service = ABTestingService(factory)

    definition = ExperimentDefinition(
        tenant_id="tenant-1",
        name="Homepage CTA",
        description=None,
        hypothesis="Variant B converts better",
        variants=[
            ExperimentVariant(key="control", name="Control", allocation=0.5, description=None, metadata={}),
            ExperimentVariant(key="variant_b", name="Variant B", allocation=0.5, description=None, metadata={}),
        ],
        success_metrics=[{"name": "conversion_rate", "goal": "increase", "weight": 1.0}],
        created_by="user-1",
    )

    session = factory()
    session.execute.return_value.one.return_value = SimpleNamespace(
        id="exp-123",
        tenant_id="tenant-1",
        name="Homepage CTA",
        description=None,
        hypothesis="Variant B converts better",
        variants="[]",
        traffic_allocation="{}",
        success_metrics="[]",
        start_date=None,
        end_date=None,
        status="draft",
        created_by="user-1",
        created_at=datetime.utcnow(),
        updated_at=datetime.utcnow(),
        results="{}",
    )

    experiment = await service.create_experiment(definition)

    assert experiment.name == "Homepage CTA"
    session.execute.assert_awaited()
