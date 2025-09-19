"""Tests for anomaly detection service."""

from datetime import datetime

import pytest

from app.services.analytics.anomaly import AnomalyDetectionService


class DummyResult:
    def __init__(self, rows):
        self._rows = rows

    def fetchall(self):
        return self._rows


class DummySession:
    def __init__(self, rows):
        self._rows = rows

    async def __aenter__(self):
        return self

    async def __aexit__(self, exc_type, exc, tb):
        return False

    async def execute(self, *_args, **_kwargs):
        return DummyResult(self._rows)


class DummyFactory:
    def __init__(self, rows):
        self._rows = rows

    def __call__(self):
        return DummySession(self._rows)


@pytest.mark.asyncio
async def test_detect_revenue_anomalies_returns_outliers():
    rows = [
        (datetime(2024, 1, day), amount)
        for day, amount in (
            (1, 100.0),
            (2, 105.0),
            (3, 110.0),
            (4, 500.0),
            (5, 95.0),
        )
    ]

    service = AnomalyDetectionService(DummyFactory(rows))
    anomalies = await service.detect_revenue_anomalies("tenant-1", threshold=2.0)

    assert len(anomalies) == 1
    anomaly = anomalies[0]
    assert anomaly["value"] == 500.0
    assert anomaly["severity"] >= 2.0


@pytest.mark.asyncio
async def test_detect_revenue_anomalies_no_variance_returns_empty():
    rows = [
        (datetime(2024, 1, 1), 100.0),
        (datetime(2024, 1, 2), 100.0),
    ]

    service = AnomalyDetectionService(DummyFactory(rows))
    anomalies = await service.detect_revenue_anomalies("tenant-1")
    assert anomalies == []
