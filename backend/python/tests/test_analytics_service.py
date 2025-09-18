import pytest

from app.models.events import AnalyticsEvent, EventContext
from app.services.analytics.service import AnalyticsService


class _RecordingBigQueryClient:
    def __init__(self) -> None:
        self.recorded = []

    def ensure_dataset(self) -> None:  # pragma: no cover - not used in this test
        pass

    def record_event(self, event: AnalyticsEvent) -> None:
        self.recorded.append(event)


class _FailingBigQueryClient:
    def ensure_dataset(self) -> None:  # pragma: no cover - not used in this test
        pass

    def record_event(self, event: AnalyticsEvent) -> None:  # noqa: D401 - test double
        raise RuntimeError("boom")


@pytest.mark.asyncio
async def test_process_event_streams_to_bigquery():
    bigquery_client = _RecordingBigQueryClient()
    service = AnalyticsService(lambda: None, bigquery_client)

    event = AnalyticsEvent(
        name="events.order.created",
        payload={"tenant_id": "tenant-123", "order_id": "order-1"},
        context=EventContext(request_id="req-1", source="rust-service"),
    )

    await service.process_event(event)

    assert bigquery_client.recorded == [event]


@pytest.mark.asyncio
async def test_process_event_swallows_bigquery_errors():
    service = AnalyticsService(lambda: None, _FailingBigQueryClient())

    event = AnalyticsEvent(
        name="events.order.created",
        payload={"tenant_id": "tenant-123"},
        context=EventContext(request_id="req-2", source="rust-service"),
    )

    await service.process_event(event)
