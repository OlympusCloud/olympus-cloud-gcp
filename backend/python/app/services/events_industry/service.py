from __future__ import annotations

from datetime import datetime, timedelta
from typing import Any, Dict, List, Optional, Tuple
from uuid import UUID

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.analytics import AnalyticsTimeframe
from app.models.events_industry import (
    ChannelPerformance,
    EventSegmentBreakdown,
    EventTimelinePoint,
    EventsAnalytics,
    EventsRecommendation,
    EventPerformance,
    VendorPerformance,
)


class EventsService:
    """Analytics and recommendations for events & ticketing tenants."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]):
        self._session_factory = session_factory

    async def get_events_analytics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None,
    ) -> EventsAnalytics:
        clause, params = self._build_clause(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )

        async with self._session_factory() as session:
            totals_row = await self._fetch_one(
                session,
                text(
                    f"""
                    SELECT
                        COUNT(DISTINCT e.id) AS total_events,
                        COUNT(DISTINCT e.id) FILTER (WHERE e.start_time >= NOW()) AS upcoming_events,
                        COALESCE(SUM(e.capacity), 0) AS total_capacity,
                        COALESCE(SUM(t.sold), 0) AS tickets_sold,
                        COALESCE(SUM(t.revenue), 0) AS gross_revenue,
                        COALESCE(AVG(t.average_price), 0) AS average_price,
                        COALESCE(AVG(feedback.rating), 0) AS avg_satisfaction
                    FROM events.events e
                    LEFT JOIN (
                        SELECT
                            event_id,
                            SUM(quantity) AS sold,
                            SUM(quantity * price) AS revenue,
                            AVG(price) AS average_price
                        FROM events.ticket_sales
                        WHERE {clause}
                        GROUP BY event_id
                    ) AS t ON t.event_id = e.id
                    LEFT JOIN events.guest_feedback feedback ON feedback.event_id = e.id
                    WHERE e.tenant_id = :tenant_id
                      AND (:location_id IS NULL OR e.location_id = :location_id)
                    """
                ),
                params,
            )

            attendance_row = await self._fetch_one(
                session,
                text(
                    f"""
                    SELECT
                        COALESCE(SUM(att.attended), 0) AS attendees,
                        COALESCE(SUM(att.registered), 0) AS registered
                    FROM (
                        SELECT
                            event_id,
                            COUNT(*) FILTER (WHERE checked_in_at IS NOT NULL) AS attended,
                            COUNT(*) AS registered
                        FROM events.attendance
                        WHERE {clause}
                        GROUP BY event_id
                    ) att
                    """
                ),
                params,
            )

            segment_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        COALESCE(segment, 'general') AS segment,
                        SUM(quantity) AS tickets_sold,
                        SUM(quantity * price) AS revenue,
                        COALESCE(AVG(conversion_rate), 0) AS conversion_rate
                    FROM events.ticket_sales
                    WHERE {clause}
                    GROUP BY segment
                    ORDER BY revenue DESC
                    LIMIT 10
                    """
                ),
                params,
            )

            channel_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        COALESCE(channel, 'unknown') AS channel,
                        SUM(quantity) AS tickets_sold,
                        SUM(quantity * price) AS revenue,
                        AVG(price) AS avg_order_value,
                        COALESCE(
                            SUM(quantity * price) FILTER (
                                WHERE sale_time BETWEEN NOW() - INTERVAL '14 days' AND NOW() - INTERVAL '7 days'
                            ),
                            0
                        ) AS previous_revenue
                    FROM events.ticket_sales
                    WHERE {clause}
                    GROUP BY channel
                    ORDER BY revenue DESC
                    """
                ),
                params,
            )

            event_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        e.id,
                        e.name,
                        COALESCE(e.venue, 'Main Venue') AS venue,
                        e.start_time,
                        e.end_time,
                        e.capacity,
                        COALESCE(SUM(ts.quantity), 0) AS tickets_sold,
                        COALESCE(SUM(ts.quantity * ts.price), 0) AS revenue,
                        COALESCE(AVG(gf.rating), 0) AS satisfaction
                    FROM events.events e
                    LEFT JOIN events.ticket_sales ts ON ts.event_id = e.id
                    LEFT JOIN events.guest_feedback gf ON gf.event_id = e.id
                    WHERE {clause}
                      AND e.tenant_id = :tenant_id
                      AND (:location_id IS NULL OR e.location_id = :location_id)
                    GROUP BY e.id, e.name, e.venue, e.start_time, e.end_time, e.capacity
                    ORDER BY revenue DESC
                    LIMIT 10
                    """
                ),
                params,
            )

            vendor_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        v.id,
                        v.name,
                        COALESCE(SUM(o.total_amount), 0) AS revenue,
                        COUNT(o.id) AS orders,
                        COALESCE(AVG(o.satisfaction_score), 0) AS satisfaction
                    FROM events.vendors v
                    LEFT JOIN events.vendor_orders o ON o.vendor_id = v.id
                    WHERE v.tenant_id = :tenant_id
                      AND (:location_id IS NULL OR v.location_id = :location_id)
                      AND ({clause})
                    GROUP BY v.id, v.name
                    ORDER BY revenue DESC
                    LIMIT 10
                    """
                ),
                {
                    **params,
                    "tenant_id": tenant_id,
                },
            )

        total_events = int(self._row_value(totals_row, "total_events", 0, 0))
        upcoming_events = int(self._row_value(totals_row, "upcoming_events", 1, 0))
        total_capacity = int(self._row_value(totals_row, "total_capacity", 2, 0))
        tickets_sold = int(self._row_value(totals_row, "tickets_sold", 3, 0))
        gross_revenue = float(self._row_value(totals_row, "gross_revenue", 4, 0.0))
        average_price = float(self._row_value(totals_row, "average_price", 5, 0.0))
        satisfaction = float(self._row_value(totals_row, "avg_satisfaction", 6, 0.0))

        attendees = int(self._row_value(attendance_row, "attendees", 0, 0))
        registered = int(self._row_value(attendance_row, "registered", 1, 0))
        attendance_rate = self._safe_div(attendees, registered) if registered else 0.0

        segment_breakdown = [
            EventSegmentBreakdown(
                segment=str(self._row_value(row, "segment", 0, "general")),
                tickets_sold=int(self._row_value(row, "tickets_sold", 1, 0)),
                revenue=float(self._row_value(row, "revenue", 2, 0.0)),
                conversion_rate=self._clamp_rate(self._row_value(row, "conversion_rate", 3, 0.0)),
            )
            for row in segment_rows
        ]

        channel_performance = [
            self._build_channel_performance(row)
            for row in channel_rows
        ]

        top_events = [
            self._build_event_performance(row)
            for row in event_rows
        ]

        vendor_performance = [
            VendorPerformance(
                vendor_id=UUID(str(self._row_value(row, "id", 0, UUID(int=0)))),
                name=str(self._row_value(row, "name", 1, "")),
                revenue=float(self._row_value(row, "revenue", 2, 0.0)),
                orders=int(self._row_value(row, "orders", 3, 0)),
                satisfaction_score=self._clamp_score(self._row_value(row, "satisfaction", 4, 0.0)),
            )
            for row in vendor_rows
        ]

        return EventsAnalytics(
            tenant_id=UUID(tenant_id),
            location_id=UUID(location_id) if location_id else None,
            generated_at=datetime.utcnow(),
            total_events=total_events,
            upcoming_events=upcoming_events,
            total_tickets_available=total_capacity,
            total_tickets_sold=tickets_sold,
            gross_revenue=gross_revenue,
            average_ticket_price=average_price,
            attendance_rate=self._clamp_rate(attendance_rate),
            average_satisfaction=self._clamp_score(satisfaction),
            segment_breakdown=segment_breakdown,
            channel_performance=channel_performance,
            top_events=top_events,
            vendor_performance=vendor_performance,
        )

    async def generate_recommendations(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None,
    ) -> List[EventsRecommendation]:
        analytics = await self.get_events_analytics(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )

        recommendations: List[EventsRecommendation] = []

        if analytics.attendance_rate < 0.7:
            recommendations.append(
                EventsRecommendation(
                    type="attendance",
                    title="Improve Attendance",
                    description="Attendance is below 70%. Run reminder campaigns and offer incentives for early check-in.",
                    impact="high",
                    priority=1,
                    data={"attendance_rate": analytics.attendance_rate},
                )
            )

        if analytics.average_ticket_price < 40 and analytics.gross_revenue < 10000:
            recommendations.append(
                EventsRecommendation(
                    type="pricing",
                    title="Review Ticket Pricing",
                    description="Average ticket price and revenue are trending low. Evaluate tiered pricing or premium add-ons.",
                    impact="medium",
                    priority=2,
                    data={
                        "average_ticket_price": analytics.average_ticket_price,
                        "gross_revenue": analytics.gross_revenue,
                    },
                )
            )

        if analytics.segment_breakdown:
            weakest_segment = min(analytics.segment_breakdown, key=lambda s: s.conversion_rate)
            if weakest_segment.conversion_rate < 0.2:
                recommendations.append(
                    EventsRecommendation(
                        type="marketing",
                        title=f"Boost {weakest_segment.segment.title()} Segment",
                        description="Conversion is lagging for this segment. Tailor content and promotions to increase engagement.",
                        impact="medium",
                        priority=3,
                        data={
                            "segment": weakest_segment.segment,
                            "conversion_rate": weakest_segment.conversion_rate,
                        },
                    )
                )

        if analytics.channel_performance:
            declining = [c for c in analytics.channel_performance if c.growth_rate < 0]
            if declining:
                poorest_channel = min(declining, key=lambda c: c.growth_rate)
                recommendations.append(
                    EventsRecommendation(
                        type="channel",
                        title=f"Reinvigorate {poorest_channel.channel.title()} Channel",
                        description="Channel revenue is declining. Experiment with targeted ads or partner promotions.",
                        impact="medium",
                        priority=2,
                        data={
                            "channel": poorest_channel.channel,
                            "growth_rate": poorest_channel.growth_rate,
                        },
                    )
                )

        if analytics.vendor_performance:
            vendor = min(analytics.vendor_performance, key=lambda v: v.satisfaction_score)
            if vendor.satisfaction_score < 4.0:
                recommendations.append(
                    EventsRecommendation(
                        type="vendor",
                        title=f"Review Vendor {vendor.name}",
                        description="Vendor satisfaction is falling. Conduct a quality review and renegotiate expectations.",
                        impact="medium",
                        priority=2,
                        data={
                            "vendor_id": str(vendor.vendor_id),
                            "satisfaction": vendor.satisfaction_score,
                        },
                    )
                )

        if not recommendations:
            recommendations.append(
                EventsRecommendation(
                    type="insight",
                    title="Events Operations Healthy",
                    description="Key events metrics are on track. Keep monitoring trends for new growth opportunities.",
                    impact="low",
                    priority=5,
                    data={},
                )
            )

        return sorted(recommendations, key=lambda rec: rec.priority)

    async def get_event_timeline(
        self,
        tenant_id: str,
        *,
        location_id: Optional[str] = None,
        start_date: datetime,
        end_date: datetime,
    ) -> List[EventTimelinePoint]:
        params = {
            "tenant_id": tenant_id,
            "location_id": location_id,
            "start_date": start_date,
            "end_date": end_date,
        }
        async with self._session_factory() as session:
            rows = await self._fetch_all(
                session,
                text(
                    """
                    SELECT
                        date_trunc('day', sale_time) AS day,
                        SUM(quantity) AS tickets_sold,
                        SUM(quantity * price) AS revenue,
                        COALESCE(SUM(new_registrations), 0) AS registrations
                    FROM events.ticket_sales
                    WHERE tenant_id = :tenant_id
                      AND (:location_id IS NULL OR location_id = :location_id)
                      AND sale_time BETWEEN :start_date AND :end_date
                    GROUP BY day
                    ORDER BY day ASC
                    """
                ),
                params,
            )

        return [
            EventTimelinePoint(
                period_start=row[0],
                period_end=row[0] + timedelta(days=1),
                tickets_sold=int(row[1]),
                revenue=float(row[2]),
                new_registrations=int(row[3]),
            )
            for row in rows
        ]

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

    def _build_clause(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: Optional[str],
        start_date: Optional[datetime],
        end_date: Optional[datetime],
    ) -> Tuple[str, Dict[str, Any]]:
        params: Dict[str, Any] = {
            "tenant_id": tenant_id,
            "location_id": location_id,
        }
        conditions = [
            "tenant_id = :tenant_id",
            "(:location_id IS NULL OR location_id = :location_id)",
        ]

        clause, clause_params = self._build_timeframe_clause("sale_time", timeframe, start_date, end_date)
        if clause:
            conditions.append(clause)
            params.update(clause_params)

        return " AND ".join(conditions), params

    def _build_timeframe_clause(
        self,
        column: str,
        timeframe: AnalyticsTimeframe,
        start: Optional[datetime],
        end: Optional[datetime],
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
            if not start or not end:
                logger.warning(
                    "events.analytics.custom_range_missing_bounds",
                    extra={"start": start, "end": end},
                )
                return "", params
            params.update({"start_date": start, "end_date": end})
            return f"{column} BETWEEN :start_date AND :end_date", params
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

    def _build_channel_performance(self, row: Any) -> ChannelPerformance:
        revenue = float(self._row_value(row, "revenue", 2, 0.0))
        previous_revenue = float(self._row_value(row, "previous_revenue", 4, 0.0))
        growth = 0.0
        if previous_revenue > 0:
            growth = (revenue - previous_revenue) / previous_revenue
        elif revenue > 0:
            growth = 1.0
        return ChannelPerformance(
            channel=str(self._row_value(row, "channel", 0, "unknown")),
            tickets_sold=int(self._row_value(row, "tickets_sold", 1, 0)),
            revenue=revenue,
            average_order_value=float(self._row_value(row, "avg_order_value", 3, 0.0)),
            growth_rate=growth,
        )

    def _build_event_performance(self, row: Any) -> EventPerformance:
        tickets_sold = int(self._row_value(row, "tickets_sold", 6, 0))
        capacity = int(self._row_value(row, "capacity", 5, 0))
        attendance = self._clamp_rate(self._safe_div(tickets_sold, capacity) if capacity else 0.0)
        return EventPerformance(
            event_id=UUID(str(self._row_value(row, "id", 0, UUID(int=0)))),
            name=str(self._row_value(row, "name", 1, "")),
            venue=str(self._row_value(row, "venue", 2, "")),
            start_time=self._row_value(row, "start_time", 3, datetime.utcnow()),
            end_time=self._row_value(row, "end_time", 4, datetime.utcnow()),
            tickets_available=capacity,
            tickets_sold=tickets_sold,
            revenue=float(self._row_value(row, "revenue", 7, 0.0)),
            attendance_rate=attendance,
            satisfaction_score=self._clamp_score(self._row_value(row, "satisfaction", 8, 0.0)),
        )
