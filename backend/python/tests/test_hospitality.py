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
from app.models.hospitality import (
    BookingWindow,
    HospitalityAnalytics,
    HospitalityRecommendation,
    HousekeepingSummary,
    RoomPerformance,
    ServiceRequestSummary,
)
from app.services.hospitality.service import HospitalityService


@pytest.fixture
def mock_session_factory():
    session = AsyncMock()
    session_factory = MagicMock()
    session_factory.return_value.__aenter__.return_value = session
    return session_factory, session


@pytest.fixture
def hospitality_service(mock_session_factory):
    session_factory, _ = mock_session_factory
    return HospitalityService(session_factory)


@pytest.mark.asyncio
async def test_get_hospitality_analytics_computes_metrics(hospitality_service, mock_session_factory):
    _, session = mock_session_factory

    rooms_row = SimpleNamespace(available_rooms=120, occupied_rooms=90)
    bookings_row = SimpleNamespace(
        total_bookings=320,
        cancellations=25,
        average_rate=185.0,
        total_revenue=42000.0,
        avg_stay_length=2.4,
        upcoming_checkins=30,
        upcoming_checkouts=18,
    )
    satisfaction_row = SimpleNamespace(satisfaction=4.3)

    service_rows = [
        SimpleNamespace(
            category="housekeeping",
            total_requests=45,
            avg_response_minutes=28.0,
            satisfaction=4.2,
        ),
        SimpleNamespace(
            category="concierge",
            total_requests=15,
            avg_response_minutes=15.0,
            satisfaction=4.8,
        ),
    ]

    housekeeping_rows = [
        SimpleNamespace(
            team="Team A",
            tasks_completed=80,
            avg_completion_minutes=35.0,
            overdue_tasks=10,
            completion_rate=0.88,
        ),
        SimpleNamespace(
            team="Team B",
            tasks_completed=60,
            avg_completion_minutes=42.0,
            overdue_tasks=5,
            completion_rate=0.9,
        ),
    ]

    room_rows = [
        SimpleNamespace(
            room_id=str(uuid4()),
            room_number="1205",
            room_type="suite",
            room_nights=24.0,
            average_rate=220.0,
            revenue=5280.0,
        )
    ]

    session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=rooms_row)),
        MagicMock(fetchone=MagicMock(return_value=bookings_row)),
        MagicMock(fetchone=MagicMock(return_value=satisfaction_row)),
        MagicMock(fetchall=MagicMock(return_value=service_rows)),
        MagicMock(fetchall=MagicMock(return_value=housekeeping_rows)),
        MagicMock(fetchall=MagicMock(return_value=room_rows)),
    ]

    tenant_id = str(uuid4())
    analytics = await hospitality_service.get_hospitality_analytics(tenant_id)

    assert isinstance(analytics, HospitalityAnalytics)
    assert analytics.bookings_created == 320
    assert analytics.cancellations == 25
    assert analytics.occupancy_rate == pytest.approx(0.75)
    assert analytics.revenue_per_available_room == pytest.approx(350.0)
    expected_completion = (80 + 60) / (80 + 60 + 10 + 5)
    assert analytics.housekeeping_completion_rate == pytest.approx(expected_completion, rel=1e-2)
    assert analytics.service_request_volume == 60
    assert analytics.guest_satisfaction_score == pytest.approx(4.3)
    assert analytics.top_rooms[0].room_number == "1205"


@pytest.mark.asyncio
async def test_generate_recommendations_prioritises_actions(hospitality_service, mock_session_factory):
    _, session = mock_session_factory

    rooms_row = SimpleNamespace(available_rooms=150, occupied_rooms=70)
    bookings_row = SimpleNamespace(
        total_bookings=120,
        cancellations=10,
        average_rate=160.0,
        total_revenue=19200.0,
        avg_stay_length=1.8,
        upcoming_checkins=40,
        upcoming_checkouts=15,
    )
    satisfaction_row = SimpleNamespace(satisfaction=3.6)
    service_rows = [
        SimpleNamespace(
            category="room_service",
            total_requests=40,
            avg_response_minutes=45.0,
            satisfaction=3.8,
        )
    ]
    housekeeping_rows = [
        SimpleNamespace(
            team="Team C",
            tasks_completed=30,
            avg_completion_minutes=60.0,
            overdue_tasks=20,
            completion_rate=0.6,
        )
    ]
    room_rows = [
        SimpleNamespace(
            room_id=str(uuid4()),
            room_number="903",
            room_type="deluxe",
            room_nights=10.0,
            average_rate=150.0,
            revenue=1500.0,
        )
    ]

    session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=rooms_row)),
        MagicMock(fetchone=MagicMock(return_value=bookings_row)),
        MagicMock(fetchone=MagicMock(return_value=satisfaction_row)),
        MagicMock(fetchall=MagicMock(return_value=service_rows)),
        MagicMock(fetchall=MagicMock(return_value=housekeeping_rows)),
        MagicMock(fetchall=MagicMock(return_value=room_rows)),
    ]

    recs = await hospitality_service.generate_recommendations(str(uuid4()))

    assert isinstance(recs, list)
    types = {rec.type for rec in recs}
    assert {"occupancy", "operations", "guest_experience", "guest_services", "front_desk"}.issubset(types)
    assert recs[0].priority == 1  # Highest priority first


class StubHospitalityService:
    async def get_hospitality_analytics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: str | None = None,
        start_date: date | None = None,
        end_date: date | None = None,
    ) -> HospitalityAnalytics:
        return HospitalityAnalytics(
            tenant_id=uuid4(),
            location_id=None,
            generated_at=datetime.utcnow(),
            occupancy_rate=0.78,
            average_daily_rate=210.0,
            revenue_per_available_room=164.0,
            available_rooms=180,
            occupied_rooms=140,
            bookings_created=260,
            cancellations=18,
            guest_satisfaction_score=4.5,
            average_stay_length=2.1,
            upcoming_check_ins=34,
            upcoming_check_outs=28,
            service_request_volume=50,
            housekeeping_completion_rate=0.87,
            top_rooms=[
                RoomPerformance(
                    room_id=uuid4(),
                    room_number="1507",
                    room_type="suite",
                    occupancy_rate=0.82,
                    average_rate=250.0,
                    revenue=3750.0,
                )
            ],
            service_requests=[
                ServiceRequestSummary(
                    category="room_service",
                    total_requests=30,
                    average_response_minutes=22.0,
                    satisfaction_score=4.6,
                )
            ],
            housekeeping=[
                HousekeepingSummary(
                    team="Team Alpha",
                    tasks_completed=70,
                    average_completion_minutes=38.0,
                    overdue_tasks=5,
                    completion_rate=0.92,
                )
            ],
        )

    async def generate_recommendations(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: str | None = None,
        start_date: date | None = None,
        end_date: date | None = None,
    ) -> list[HospitalityRecommendation]:
        return [
            HospitalityRecommendation(
                type="operations",
                title="Balance Staffing",
                description="Align staffing with expected check-in volume.",
                impact="medium",
                priority=2,
                data={"upcoming_check_ins": 34},
            )
        ]


@pytest.mark.asyncio
async def test_hospitality_analytics_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.hospitality_service = StubHospitalityService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get("/api/hospitality/analytics", params={"tenant_id": str(uuid4())})

    assert response.status_code == 200
    payload = response.json()
    assert payload["occupancy_rate"] == pytest.approx(0.78)
    assert payload["bookings_created"] == 260
    assert payload["housekeeping_completion_rate"] == pytest.approx(0.87)

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_hospitality_recommendations_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.hospitality_service = StubHospitalityService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get("/api/hospitality/recommendations", params={"tenant_id": str(uuid4())})

    assert response.status_code == 200
    payload = response.json()
    assert isinstance(payload, list)
    assert payload[0]["type"] == "operations"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_booking_window_fetch(hospitality_service, mock_session_factory):
    _, session = mock_session_factory

    row = SimpleNamespace(total_bookings=40, average_rate=190.0, conversion_rate=0.42)
    session.execute.return_value = MagicMock(fetchone=MagicMock(return_value=row))

    start = date.today()
    end = start + timedelta(days=30)

    window = await hospitality_service.get_booking_window(str(uuid4()), start_date=start, end_date=end)

    assert isinstance(window, BookingWindow)
    assert window.total_bookings == 40
    assert window.conversion_rate == pytest.approx(0.42)

    session.execute.assert_awaited()
