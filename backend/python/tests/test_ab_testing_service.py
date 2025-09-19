"""Tests for the A/B testing analytics service."""

from __future__ import annotations

import json
import uuid
from datetime import datetime
from decimal import Decimal
from types import SimpleNamespace
from unittest.mock import AsyncMock, Mock

import pytest

from app.models.experiments import (
    ConversionUpdate,
    ParticipantAssignment,
)
from app.services.analytics.ab_testing import ABTestingService


@pytest.fixture
def mock_session_factory() -> Mock:
    session = Mock()
    session.execute = AsyncMock()
    session.commit = AsyncMock()
    session.rollback = AsyncMock()

    factory = Mock()
    factory.return_value.__aenter__ = AsyncMock(return_value=session)
    factory.return_value.__aexit__ = AsyncMock(return_value=None)
    return factory


@pytest.mark.asyncio
async def test_get_experiment_detail_returns_computed_results(mock_session_factory: Mock) -> None:
    service = ABTestingService(mock_session_factory)

    experiment_id = uuid.uuid4()
    tenant_id = uuid.uuid4()
    now = datetime.utcnow()

    experiment_row = SimpleNamespace(
        id=experiment_id,
        tenant_id=tenant_id,
        name="Homepage CTA",
        description="Testing CTA variants",
        hypothesis="Variant B increases conversions",
        variants=json.dumps([
            {"key": "control", "name": "Control", "allocation": 0.5, "description": None, "metadata": {}},
            {"key": "variant_b", "name": "Variant B", "allocation": 0.5, "description": None, "metadata": {}},
        ]),
        traffic_allocation=json.dumps({"Control": 0.5, "Variant B": 0.5}),
        success_metrics=json.dumps([
            {"name": "conversion_rate", "goal": "increase", "target": None, "weight": 1.0}
        ]),
        start_date=now,
        end_date=None,
        status="running",
        created_by=uuid.uuid4(),
        created_at=now,
        updated_at=now,
        results=json.dumps({}),
    )

    participants_rows = [
        SimpleNamespace(variant_name="Control", participants=200, conversions=20, total_value=Decimal("400")),
        SimpleNamespace(variant_name="Variant B", participants=190, conversions=30, total_value=Decimal("750")),
    ]

    experiment_result = Mock()
    experiment_result.one_or_none.return_value = experiment_row

    participant_result = Mock()
    participant_result.fetchall.return_value = participants_rows

    session = mock_session_factory.return_value.__aenter__.return_value
    session.execute.side_effect = [experiment_result, participant_result]

    detail = await service.get_experiment_detail(str(tenant_id), str(experiment_id))

    assert detail is not None
    assert detail.results.baseline_variant == "Control"
    assert len(detail.results.variants) == 2
    variant_b = next(variant for variant in detail.results.variants if variant.name == "Variant B")
    assert pytest.approx(variant_b.conversion_rate, rel=1e-3) == 30 / 190
    assert detail.results.comparisons[0].is_significant is False or detail.results.comparisons[0].confidence is not None


@pytest.mark.asyncio
async def test_record_assignment_upsert(mock_session_factory: Mock) -> None:
    service = ABTestingService(mock_session_factory)

    now = datetime.utcnow()
    assignment = ParticipantAssignment(
        experiment_id=str(uuid.uuid4()),
        variant_name="Control",
        session_id="session-123",
        assigned_at=now,
    )

    stored_row = SimpleNamespace(
        id=uuid.uuid4(),
        experiment_id=uuid.UUID(assignment.experiment_id),
        user_id=None,
        customer_id=None,
        session_id=assignment.session_id,
        variant_name=assignment.variant_name,
        assigned_at=now,
        converted_at=None,
        conversion_value=None,
    )

    session = mock_session_factory.return_value.__aenter__.return_value
    result = Mock()
    result.one.return_value = stored_row
    session.execute.return_value = result

    record = await service.record_assignment(assignment)

    session.commit.assert_awaited_once()
    assert record.variant_name == "Control"
    assert record.session_id == "session-123"


@pytest.mark.asyncio
async def test_record_conversion_updates_participant(mock_session_factory: Mock) -> None:
    service = ABTestingService(mock_session_factory)

    participant_id = uuid.uuid4()
    experiment_id = uuid.uuid4()
    now = datetime.utcnow()

    updated_row = SimpleNamespace(
        id=participant_id,
        experiment_id=experiment_id,
        user_id=None,
        customer_id=None,
        session_id="session-123",
        variant_name="Control",
        assigned_at=now,
        converted_at=now,
        conversion_value=Decimal("199.99"),
    )

    session = mock_session_factory.return_value.__aenter__.return_value
    result = Mock()
    result.one.return_value = updated_row
    session.execute.return_value = result

    update = ConversionUpdate(participant_id=str(participant_id), conversion_value=199.99, converted_at=now)
    record = await service.record_conversion(update)

    session.commit.assert_awaited_once()
    assert record.conversion_value == pytest.approx(199.99)
    assert record.converted_at == now
