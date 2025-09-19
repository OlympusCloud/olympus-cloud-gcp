from __future__ import annotations

from datetime import date, datetime, timedelta
from types import SimpleNamespace
from unittest.mock import AsyncMock, MagicMock
from uuid import uuid4

import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings
from app.models.analytics import AnalyticsTimeframe
from app.models.events_industry import (
    ChannelPerformance,
    EventPerformance,
    EventSegmentBreakdown,
    EventTimelinePoint,
    EventsAnalytics,
    EventsRecommendation,
    VendorPerformance,
)
from app.services.events_industry.service import EventsService


@pytest.fixture
def mock_session_factory():
    session = AsyncMock()
    factory = MagicMock()
    factory.return_value.__aenter__.return_value = session
    return factory, session


@pytest.fixture
def events_service(mock_session_factory):
    factory, _ = mock_session_factory
    return EventsService(factory)


@pytest.mark.asyncio
async def test_events_analytics_compiles_metrics(events_service, mock_session_factory):
    _, session = mock_session_factory

    totals_row = SimpleNamespace(
        total_events=12,
        upcoming_events=5,
        total_capacity=2500,
        tickets_sold=1800,
        gross_revenue=72000.0,
        average_price=45.0,
        avg_satisfaction=4.4,
    )
    attendance_row = SimpleNamespace(attendees=1500, registered=2000)

    segment_rows = [
        SimpleNamespace(segment="vip", tickets_sold=200, revenue=18000.0, conversion_rate=0.35),
        SimpleNamespace(segment="general", tickets_sold=1600, revenue=54000.0, conversion_rate=0.22),
    ]

    channel_rows = [
        SimpleNamespace(
            channel="web",
            tickets_sold=1000,
            revenue=40000.0,
            avg_order_value=40.0,
            previous_revenue=25000.0,
        ),
        SimpleNamespace(
            channel="partners",
            tickets_sold=500,
            revenue=22000.0,
            avg_order_value=44.0,
            previous_revenue=24000.0,
        ),
    ]

    event_rows = [
        SimpleNamespace(
            id=str(uuid4()),
            name="Tech Expo",
            venue="Hall A",
            start_time=datetime(2025, 10, 1, 10, 0),
            end_time=datetime(2025, 10, 1, 17, 0),
            capacity=500,
            tickets_sold=450,
            revenue=22000.0,
            satisfaction=4.6,
        )
    ]

    vendor_rows = [
        SimpleNamespace(
            id=str(uuid4()),
            name="Catering Co",
            revenue=12000.0,
            orders=60,
            satisfaction=4.1,
        )
    ]

    session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=totals_row)),
        MagicMock(fetchone=MagicMock(return_value=attendance_row)),
        MagicMock(fetchall=MagicMock(return_value=segment_rows)),
        MagicMock(fetchall=MagicMock(return_value=channel_rows)),
        MagicMock(fetchall=MagicMock(return_value=event_rows)),
        MagicMock(fetchall=MagicMock(return_value=vendor_rows)),
    ]

    analytics = await events_service.get_events_analytics(str(uuid4()))

    assert isinstance(analytics, EventsAnalytics)
    assert analytics.total_events == 12
    assert analytics.attendance_rate == pytest.approx(0.75)
    assert analytics.average_ticket_price == pytest.approx(45.0)
    assert analytics.segment_breakdown[0].segment == "vip"
    assert analytics.channel_performance[0].growth_rate == pytest.approx(0.6)
    assert analytics.top_events[0].name == "Tech Expo"
    assert analytics.vendor_performance[0].name == "Catering Co"


@pytest.mark.asyncio
async def test_events_recommendations(events_service, mock_session_factory):
    _, session = mock_session_factory

    totals_row = SimpleNamespace(
        total_events=5,
        upcoming_events=2,
        total_capacity=800,
        tickets_sold=400,
        gross_revenue=12000.0,
        average_price=30.0,
        avg_satisfaction=3.5,
    )
    attendance_row = SimpleNamespace(attendees=250, registered=500)
    segment_rows = [SimpleNamespace(segment="general", tickets_sold=350, revenue=9000.0, conversion_rate=0.18)]
    channel_rows = [SimpleNamespace(channel="web", tickets_sold=200, revenue=7000.0, avg_order_value=35.0, previous_revenue=9000.0)]
    event_rows = [
        SimpleNamespace(
            id=str(uuid4()),
            name="Music Night",
            venue="Arena",
            start_time=datetime.utcnow(),
            end_time=datetime.utcnow() + timedelta(hours=4),
            capacity=400,
            tickets_sold=300,
            revenue=10000.0,
            satisfaction=3.8,
        )
    ]
    vendor_rows = [
        SimpleNamespace(id=str(uuid4()), name="Vendor B", revenue=2000.0, orders=20, satisfaction=3.7)
    ]

    session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=totals_row)),
        MagicMock(fetchone=MagicMock(return_value=attendance_row)),
        MagicMock(fetchall=MagicMock(return_value=segment_rows)),
        MagicMock(fetchall=MagicMock(return_value=channel_rows)),
        MagicMock(fetchall=MagicMock(return_value=event_rows)),
        MagicMock(fetchall=MagicMock(return_value=vendor_rows)),
    ]

    recs = await events_service.generate_recommendations(str(uuid4()))

    assert recs
    types = {rec.type for rec in recs}
    assert {"attendance", "pricing", "marketing", "channel", "vendor"}.issubset(types)
    assert recs[0].priority == 1


class StubEventsService:
    async def get_events_analytics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: str | None = None,
        start_date: datetime | None = None,
        end_date: datetime | None = None,
    ) -> EventsAnalytics:
        return EventsAnalytics(
            tenant_id=uuid4(),
            location_id=None,
            generated_at=datetime.utcnow(),
            total_events=8,
            upcoming_events=3,
            total_tickets_available=1500,
            total_tickets_sold=1100,
            gross_revenue=48000.0,
            average_ticket_price=52.0,
            attendance_rate=0.74,
            average_satisfaction=4.3,
            segment_breakdown=[
                EventSegmentBreakdown(segment="vip", tickets_sold=120, revenue=9600.0, conversion_rate=0.4)
            ],
            channel_performance=[
                ChannelPerformance(channel="web", tickets_sold=700, revenue=32000.0, average_order_value=45.7, growth_rate=0.22)
            ],
            top_events=[
                EventPerformance(
                    event_id=uuid4(),
                    name="Expo",
                    venue="Center",
                    start_time=datetime.utcnow(),
                    end_time=datetime.utcnow(),
                    tickets_available=500,
                    tickets_sold=450,
                    revenue=23000.0,
                    attendance_rate=0.9,
                    satisfaction_score=4.7,
                )
            ],
            vendor_performance=[
                VendorPerformance(
                    vendor_id=uuid4(),
                    name="Vendor A",
                    revenue=6000.0,
                    orders=40,
                    satisfaction_score=4.4,
                )
            ],
        )

    async def generate_recommendations(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: str | None = None,
        start_date: datetime | None = None,
        end_date: datetime | None = None,
    ) -> list[EventsRecommendation]:
        return [
            EventsRecommendation(
                type="insight",
                title="Maintain Momentum",
                description="Events operations performing well.",
                impact="low",
                priority=5,
                data={},
            )
        ]


@pytest.mark.asyncio
async def test_events_endpoints(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.events_service = StubEventsService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        analytics_resp = await client.get(
            "/api/events/analytics",
            params={"tenant_id": str(uuid4()), "date_range": "today"},
        )
        recommendations_resp = await client.get(
            "/api/events/recommendations",
            params={"tenant_id": str(uuid4()), "date_range": "today"},
        )

    assert analytics_resp.status_code == 200
    assert analytics_resp.json()["total_events"] == 8
    assert recommendations_resp.status_code == 200
    assert recommendations_resp.json()[0]["type"] == "insight"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_event_timeline(events_service, mock_session_factory):
    _, session = mock_session_factory
    timeline_rows = [
        (
            datetime(2025, 9, 1, 0, 0),
            120,
            5400.0,
            30,
        ),
        (
            datetime(2025, 9, 2, 0, 0),
            150,
            6750.0,
            45,
        ),
    ]
    session.execute.return_value = MagicMock(fetchall=MagicMock(return_value=timeline_rows))

    start = datetime(2025, 9, 1)
    end = datetime(2025, 9, 3)

    points = await events_service.get_event_timeline(str(uuid4()), start_date=start, end_date=end)

    assert isinstance(points, list)
    assert len(points) == 2
    assert isinstance(points[0], EventTimelinePoint)
    assert points[0].tickets_sold == 120
    session.execute.assert_awaited()
