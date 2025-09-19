"""Unit tests for anomaly detection service."""

from __future__ import annotations

from datetime import datetime
from typing import Any, Iterable, Sequence

import pytest

from app.services.analytics.anomaly import AnomalyDetectionService


class _StubResult:
    def __init__(self, rows: Sequence[Sequence[Any]]) -> None:
        self._rows = rows

    def fetchall(self) -> Sequence[Sequence[Any]]:
        return self._rows


class _StubSession:
    def __init__(self, rows: Sequence[Sequence[Any]]) -> None:
        self._rows = rows

    async def __aenter__(self) -> "_StubSession":
        return self

    async def __aexit__(self, exc_type, exc, tb) -> None:  # noqa: ANN001
        return None

    async def execute(self, query, params):  # noqa: ANN001
        return _StubResult(self._rows)


class _StubSessionFactory:
    def __init__(self, rows: Sequence[Sequence[Any]]) -> None:
        self._rows = rows

    def __call__(self) -> _StubSession:
        return _StubSession(self._rows)


@pytest.mark.asyncio
async def test_detect_revenue_anomalies_returns_outliers() -> None:
    rows: Iterable[Sequence[Any]] = [
        (datetime(2024, 1, 1), 100.0),
        (datetime(2024, 1, 2), 101.0),
        (datetime(2024, 1, 3), 99.0),
        (datetime(2024, 1, 4), 100.0),
        (datetime(2024, 1, 5), 102.0),
        (datetime(2024, 1, 6), 98.0),
        (datetime(2024, 1, 7), 101.0),
        (datetime(2024, 1, 8), 99.5),
        (datetime(2024, 1, 9), 100.5),
        (datetime(2024, 1, 10), 100.2),
        (datetime(2024, 1, 11), 320.0),  # clear anomaly
    ]

    service = AnomalyDetectionService(session_factory=_StubSessionFactory(rows))

    anomalies = await service.detect_revenue_anomalies("tenant-123", threshold=2.5)

    assert len(anomalies) == 1
    anomaly = anomalies[0]
    assert anomaly["value"] == pytest.approx(320.0)
    assert anomaly["severity"] >= 2.5


@pytest.mark.asyncio
async def test_detect_revenue_anomalies_handles_no_variance() -> None:
    rows: Iterable[Sequence[Any]] = [
        (datetime(2024, 1, 1), 100.0),
        (datetime(2024, 1, 2), 100.0),
        (datetime(2024, 1, 3), 100.0),
    ]

    service = AnomalyDetectionService(session_factory=_StubSessionFactory(rows))

    anomalies = await service.detect_revenue_anomalies("tenant-456")

    assert anomalies == []
