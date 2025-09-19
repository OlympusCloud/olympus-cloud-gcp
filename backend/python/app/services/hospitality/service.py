from __future__ import annotations

from datetime import date, datetime
from typing import Any, Dict, List, Optional, Tuple
from uuid import UUID

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.analytics import AnalyticsTimeframe
from app.models.hospitality import (
    BookingWindow,
    HospitalityAnalytics,
    HospitalityRecommendation,
    HousekeepingSummary,
    RoomPerformance,
    ServiceRequestSummary,
)


class HospitalityService:
    """Analytics and recommendations for hospitality tenants."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]):
        self._session_factory = session_factory

    async def get_hospitality_analytics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> HospitalityAnalytics:
        booking_clause, booking_params = self._build_booking_clause(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )

        base_params = {"tenant_id": tenant_id, "location_id": location_id}

        async with self._session_factory() as session:
            rooms_row = await self._fetch_one(
                session,
                text(
                    """
                    SELECT
                        COUNT(*) FILTER (WHERE status != 'out_of_service') AS available_rooms,
                        COUNT(*) FILTER (WHERE status IN ('occupied', 'reserved')) AS occupied_rooms
                    FROM hospitality.rooms
                    WHERE tenant_id = :tenant_id
                      AND (:location_id IS NULL OR location_id = :location_id)
                    """
                ),
                base_params,
            )

            bookings_row = await self._fetch_one(
                session,
                text(
                    f"""
                    SELECT
                        COUNT(*) AS total_bookings,
                        COUNT(*) FILTER (WHERE status = 'cancelled') AS cancellations,
                        COUNT(*) FILTER (WHERE status = 'confirmed') AS confirmed,
                        COUNT(*) FILTER (WHERE status = 'checked_in') AS checked_in,
                        COUNT(*) FILTER (WHERE status = 'checked_out') AS checked_out,
                        COALESCE(AVG(room_rate), 0) AS average_rate,
                        COALESCE(SUM(room_rate), 0) AS total_revenue,
                        COALESCE(AVG(EXTRACT(EPOCH FROM (checkout_date - checkin_date)) / 86400), 0) AS avg_stay_length,
                        COALESCE(SUM(CASE WHEN checkin_date >= CURRENT_DATE AND checkin_date < CURRENT_DATE + INTERVAL '2 day' THEN 1 ELSE 0 END), 0) AS upcoming_checkins,
                        COALESCE(SUM(CASE WHEN checkout_date >= CURRENT_DATE AND checkout_date < CURRENT_DATE + INTERVAL '2 day' THEN 1 ELSE 0 END), 0) AS upcoming_checkouts
                    FROM hospitality.bookings b
                    WHERE {booking_clause}
                    """
                ),
                booking_params,
            )

            satisfaction_row = await self._fetch_one(
                session,
                text(
                    """
                    SELECT COALESCE(AVG(rating), 0) AS satisfaction
                    FROM hospitality.guest_feedback
                    WHERE tenant_id = :tenant_id
                      AND (:location_id IS NULL OR location_id = :location_id)
                    """
                ),
                base_params,
            )

            service_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        COALESCE(category, 'general') AS category,
                        COUNT(*) AS total_requests,
                        COALESCE(AVG(EXTRACT(EPOCH FROM (completed_at - created_at)) / 60), 0) AS avg_response_minutes,
                        COALESCE(AVG(satisfaction_score), 0) AS satisfaction
                    FROM hospitality.service_requests
                    WHERE {booking_clause}
                    GROUP BY category
                    ORDER BY total_requests DESC
                    LIMIT 8
                    """
                ),
                booking_params,
            )

            housekeeping_rows = await self._fetch_all(
                session,
                text(
                    """
                    SELECT
                        COALESCE(team, 'unassigned') AS team,
                        COUNT(*) FILTER (WHERE status = 'completed') AS tasks_completed,
                        COALESCE(AVG(EXTRACT(EPOCH FROM (completed_at - scheduled_at)) / 60), 0) AS avg_completion_minutes,
                        COUNT(*) FILTER (WHERE status != 'completed' AND scheduled_at < NOW()) AS overdue_tasks,
                        COALESCE(AVG(CASE WHEN status = 'completed' THEN 1.0 ELSE 0.0 END), 0) AS completion_rate
                    FROM hospitality.housekeeping_tasks
                    WHERE tenant_id = :tenant_id
                      AND (:location_id IS NULL OR location_id = :location_id)
                      AND scheduled_at::date >= COALESCE(:start_date, CURRENT_DATE - INTERVAL '30 day')
                    GROUP BY team
                    ORDER BY tasks_completed DESC
                    LIMIT 10
                    """
                ),
                {
                    "tenant_id": tenant_id,
                    "location_id": location_id,
                    "start_date": start_date,
                },
            )

            room_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        CAST(b.room_id AS TEXT) AS room_id,
                        MAX(r.room_number) AS room_number,
                        MAX(r.room_type) AS room_type,
                        COALESCE(SUM(EXTRACT(EPOCH FROM (checkout_date - checkin_date)) / 86400), 0) AS room_nights,
                        COALESCE(AVG(b.room_rate), 0) AS average_rate,
                        COALESCE(SUM(b.room_rate), 0) AS revenue
                    FROM hospitality.bookings b
                    JOIN hospitality.rooms r ON r.id = b.room_id
                    WHERE {booking_clause}
                    GROUP BY b.room_id
                    ORDER BY revenue DESC
                    LIMIT 10
                    """
                ),
                booking_params,
            )

        available_rooms = int(self._row_value(rooms_row, "available_rooms", 0, 0))
        occupied_rooms = int(self._row_value(rooms_row, "occupied_rooms", 1, 0))

        total_bookings = int(self._row_value(bookings_row, "total_bookings", 0, 0))
        cancellations = int(self._row_value(bookings_row, "cancellations", 1, 0))
        average_rate = float(self._row_value(bookings_row, "average_rate", 5, 0.0))
        total_revenue = float(self._row_value(bookings_row, "total_revenue", 6, 0.0))
        avg_stay_length = float(self._row_value(bookings_row, "avg_stay_length", 7, 0.0))
        upcoming_checkins = int(self._row_value(bookings_row, "upcoming_checkins", 8, 0))
        upcoming_checkouts = int(self._row_value(bookings_row, "upcoming_checkouts", 9, 0))

        occupancy_rate = self._clamp_rate(self._safe_div(occupied_rooms, available_rooms) if available_rooms else 0.0)
        revpar = total_revenue / available_rooms if available_rooms else 0.0

        satisfaction = float(self._row_value(satisfaction_row, "satisfaction", 0, 0.0))

        service_summaries = [
            ServiceRequestSummary(
                category=str(self._row_value(row, "category", 0, "general")),
                total_requests=int(self._row_value(row, "total_requests", 1, 0)),
                average_response_minutes=float(self._row_value(row, "avg_response_minutes", 2, 0.0)),
                satisfaction_score=self._clamp_score(self._row_value(row, "satisfaction", 3, 0.0)),
            )
            for row in service_rows
        ]

        housekeeping_summaries = [
            HousekeepingSummary(
                team=str(self._row_value(row, "team", 0, "unassigned")),
                tasks_completed=int(self._row_value(row, "tasks_completed", 1, 0)),
                average_completion_minutes=float(self._row_value(row, "avg_completion_minutes", 2, 0.0)),
                overdue_tasks=int(self._row_value(row, "overdue_tasks", 3, 0)),
                completion_rate=self._clamp_rate(self._row_value(row, "completion_rate", 4, 0.0)),
            )
            for row in housekeeping_rows
        ]

        room_performance = [
            self._build_room_performance(row)
            for row in room_rows
        ]

        housekeeping_total_tasks = sum(item.tasks_completed + item.overdue_tasks for item in housekeeping_summaries)
        housekeeping_completion = self._safe_div(
            sum(item.tasks_completed for item in housekeeping_summaries),
            housekeeping_total_tasks,
        ) if housekeeping_total_tasks else 0.0

        return HospitalityAnalytics(
            tenant_id=UUID(tenant_id),
            location_id=UUID(location_id) if location_id else None,
            generated_at=datetime.utcnow(),
            occupancy_rate=occupancy_rate,
            average_daily_rate=average_rate,
            revenue_per_available_room=revpar,
            available_rooms=available_rooms,
            occupied_rooms=occupied_rooms,
            bookings_created=total_bookings,
            cancellations=cancellations,
            guest_satisfaction_score=self._clamp_score(satisfaction),
            average_stay_length=avg_stay_length,
            upcoming_check_ins=upcoming_checkins,
            upcoming_check_outs=upcoming_checkouts,
            service_request_volume=sum(item.total_requests for item in service_summaries),
            housekeeping_completion_rate=self._clamp_rate(housekeeping_completion),
            top_rooms=room_performance,
            service_requests=service_summaries,
            housekeeping=housekeeping_summaries,
        )

    async def generate_recommendations(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> List[HospitalityRecommendation]:
        analytics = await self.get_hospitality_analytics(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )

        recommendations: List[HospitalityRecommendation] = []

        if analytics.occupancy_rate < 0.65:
            recommendations.append(
                HospitalityRecommendation(
                    type="occupancy",
                    title="Boost Low Occupancy",
                    description=(
                        "Occupancy is below 65%. Launch targeted promotions or adjust pricing to increase bookings."
                    ),
                    impact="high",
                    priority=1,
                    data={"occupancy_rate": analytics.occupancy_rate},
                )
            )

        if analytics.housekeeping_completion_rate < 0.8:
            recommendations.append(
                HospitalityRecommendation(
                    type="operations",
                    title="Improve Housekeeping Turnaround",
                    description=(
                        "Housekeeping completion rate is under 80%. Review staffing levels and scheduling to speed up room turns."
                    ),
                    impact="medium",
                    priority=2,
                    data={"completion_rate": analytics.housekeeping_completion_rate},
                )
            )

        if analytics.guest_satisfaction_score < 4.0:
            recommendations.append(
                HospitalityRecommendation(
                    type="guest_experience",
                    title="Address Guest Satisfaction",
                    description="Guest satisfaction is trending below 4.0. Analyse feedback categories and action quick wins.",
                    impact="high",
                    priority=1,
                    data={"satisfaction": analytics.guest_satisfaction_score},
                )
            )

        if analytics.service_request_volume > 0:
            busiest_category = max(analytics.service_requests, key=lambda s: s.total_requests, default=None)
            if busiest_category and busiest_category.average_response_minutes > 30:
                recommendations.append(
                    HospitalityRecommendation(
                        type="guest_services",
                        title=f"Accelerate {busiest_category.category.title()} Responses",
                        description=(
                            f"Average response time for {busiest_category.category} is over 30 minutes. Add staff coverage or introduce triage workflows."
                        ),
                        impact="medium",
                        priority=2,
                        data={
                            "category": busiest_category.category,
                            "response_minutes": busiest_category.average_response_minutes,
                        },
                    )
                )

        if analytics.upcoming_check_ins - analytics.upcoming_check_outs > analytics.available_rooms * 0.1:
            recommendations.append(
                HospitalityRecommendation(
                    type="front_desk",
                    title="Prepare for Check-in Surge",
                    description="Upcoming check-ins exceed check-outs significantly. Ensure front desk staffing and housekeeping schedules align.",
                    impact="medium",
                    priority=3,
                    data={
                        "check_ins": analytics.upcoming_check_ins,
                        "check_outs": analytics.upcoming_check_outs,
                    },
                )
            )

        if not recommendations:
            recommendations.append(
                HospitalityRecommendation(
                    type="insight",
                    title="Hospitality Operations Healthy",
                    description="Key hospitality metrics are on target. Continue monitoring demand patterns for optimisation opportunities.",
                    impact="low",
                    priority=5,
                    data={},
                )
            )

        return sorted(recommendations, key=lambda rec: rec.priority)

    async def get_booking_window(
        self,
        tenant_id: str,
        *,
        start_date: date,
        end_date: date,
        location_id: Optional[str] = None,
    ) -> BookingWindow:
        params = {
            "tenant_id": tenant_id,
            "location_id": location_id,
            "start_date": start_date,
            "end_date": end_date,
        }
        async with self._session_factory() as session:
            row = await self._fetch_one(
                session,
                text(
                    """
                    SELECT
                        COUNT(*) AS total_bookings,
                        COALESCE(AVG(room_rate), 0) AS average_rate,
                        COALESCE(AVG(conversion_rate), 0) AS conversion_rate
                    FROM hospitality.booking_windows
                    WHERE tenant_id = :tenant_id
                      AND (:location_id IS NULL OR location_id = :location_id)
                      AND window_start >= :start_date
                      AND window_end <= :end_date
                    """
                ),
                params,
            )

        return BookingWindow(
            start_date=start_date,
            end_date=end_date,
            total_bookings=int(self._row_value(row, "total_bookings", 0, 0)),
            average_rate=float(self._row_value(row, "average_rate", 1, 0.0)),
            conversion_rate=self._clamp_rate(self._row_value(row, "conversion_rate", 2, 0.0)),
        )

    async def _fetch_one(
        self,
        session: AsyncSession,
        statement,
        params: Dict[str, Any],
    ) -> Any:
        result = await session.execute(statement, params)
        return result.fetchone()

    async def _fetch_all(
        self,
        session: AsyncSession,
        statement,
        params: Dict[str, Any],
    ) -> List[Any]:
        result = await session.execute(statement, params)
        return list(result.fetchall())

    def _build_booking_clause(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: Optional[str],
        start_date: Optional[date],
        end_date: Optional[date],
    ) -> Tuple[str, Dict[str, Any]]:
        params: Dict[str, Any] = {
            "tenant_id": tenant_id,
            "location_id": location_id,
        }
        conditions = [
            "tenant_id = :tenant_id",
            "(:location_id IS NULL OR location_id = :location_id)",
        ]

        clause, clause_params = self._build_timeframe_clause("created_at", timeframe, start_date, end_date)
        if clause:
            conditions.append(clause)
            params.update(clause_params)

        return " AND ".join(conditions), params

    def _build_timeframe_clause(
        self,
        column: str,
        timeframe: AnalyticsTimeframe,
        start_date: Optional[date],
        end_date: Optional[date],
    ) -> Tuple[str, Dict[str, Any]]:
        params: Dict[str, Any] = {}

        if timeframe == AnalyticsTimeframe.ALL_TIME:
            return "", params
        if timeframe == AnalyticsTimeframe.TODAY:
            return f"{column}::date = CURRENT_DATE", params
        if timeframe == AnalyticsTimeframe.YESTERDAY:
            return f"{column}::date = CURRENT_DATE - INTERVAL '1 day'", params
        if timeframe == AnalyticsTimeframe.THIS_WEEK:
            return f"{column} >= date_trunc('week', CURRENT_DATE::timestamp)", params
        if timeframe == AnalyticsTimeframe.LAST_WEEK:
            clause = (
                "{column} >= date_trunc('week', CURRENT_DATE::timestamp) - INTERVAL '1 week' "
                "AND {column} < date_trunc('week', CURRENT_DATE::timestamp)"
            ).format(column=column)
            return clause, params
        if timeframe == AnalyticsTimeframe.THIS_MONTH:
            return f"{column} >= date_trunc('month', CURRENT_DATE::timestamp)", params
        if timeframe == AnalyticsTimeframe.LAST_MONTH:
            clause = (
                "{column} >= date_trunc('month', CURRENT_DATE::timestamp) - INTERVAL '1 month' "
                "AND {column} < date_trunc('month', CURRENT_DATE::timestamp)"
            ).format(column=column)
            return clause, params
        if timeframe == AnalyticsTimeframe.YEAR_TO_DATE:
            return f"{column} >= date_trunc('year', CURRENT_DATE::timestamp)", params
        if timeframe == AnalyticsTimeframe.THIS_QUARTER:
            return f"{column} >= date_trunc('quarter', CURRENT_DATE::timestamp)", params
        if timeframe == AnalyticsTimeframe.THIS_YEAR:
            return f"{column} >= date_trunc('year', CURRENT_DATE::timestamp)", params
        if timeframe == AnalyticsTimeframe.CUSTOM:
            if not start_date or not end_date:
                logger.warning(
                    "hospitality.analytics.custom_range_missing_bounds",
                    extra={"start": start_date, "end": end_date},
                )
                return "", params
            params.update({"start_date": start_date, "end_date": end_date})
            return f"{column}::date BETWEEN :start_date AND :end_date", params
        return "", params

    @staticmethod
    def _row_value(row: Any, key: str, index: int, default: Any) -> Any:
        if row is None:
            return default
        if hasattr(row, "_mapping") and key in row._mapping:
            value = row._mapping[key]
            return default if value is None else value
        if isinstance(row, dict) and key in row:
            value = row[key]
            return default if value is None else value
        if hasattr(row, key):
            value = getattr(row, key)
            return default if value is None else value
        try:
            value = row[index]
        except (IndexError, KeyError, TypeError):
            return default
        return default if value is None else value

    @staticmethod
    def _safe_div(numerator: float, denominator: float) -> float:
        if not denominator:
            return 0.0
        return float(numerator) / float(denominator)

    @staticmethod
    def _clamp_rate(value: float) -> float:
        return max(0.0, min(float(value), 1.0))

    @staticmethod
    def _clamp_score(value: float) -> float:
        return max(0.0, min(float(value), 5.0))

    def _build_room_performance(self, row: Any) -> RoomPerformance:
        room_nights = float(self._row_value(row, "room_nights", 3, 0.0))
        occupancy_rate = self._clamp_rate(room_nights / 30.0)
        return RoomPerformance(
            room_id=UUID(str(self._row_value(row, "room_id", 0, UUID(int=0)))),
            room_number=str(self._row_value(row, "room_number", 1, "")),
            room_type=str(self._row_value(row, "room_type", 2, "standard")),
            occupancy_rate=occupancy_rate,
            average_rate=float(self._row_value(row, "average_rate", 4, 0.0)),
            revenue=float(self._row_value(row, "revenue", 5, 0.0)),
        )
