from __future__ import annotations

from sqlalchemy import text
from sqlalchemy.exc import SQLAlchemyError
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.analytics import (
    AnalyticsDashboardResponse,
    AnalyticsMetrics,
    AnalyticsTimeframe,
)
from app.models.events import AnalyticsEvent
from app.services.analytics.bigquery import BigQueryClient


class AnalyticsService:
    """Coordinate analytics queries across PostgreSQL and BigQuery."""

    def __init__(
        self,
        session_factory: async_sessionmaker[AsyncSession],
        bigquery_client: BigQueryClient,
    ) -> None:
        self._session_factory = session_factory
        self._bigquery = bigquery_client

    async def get_dashboard_metrics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
    ) -> AnalyticsDashboardResponse:
        """Return high-level dashboard metrics for a tenant."""

        metrics = AnalyticsMetrics(timeframe=timeframe)

        async with self._session_factory() as session:
            metrics = metrics.model_copy(update=await self._fetch_user_metrics(session, tenant_id))
            metrics = metrics.model_copy(
                update=await self._fetch_order_metrics(session, tenant_id, timeframe)
            )
            metrics = metrics.model_copy(update=await self._fetch_inventory_metrics(session, tenant_id))

        return AnalyticsDashboardResponse(tenant_id=tenant_id, metrics=metrics)

    async def process_event(self, event: AnalyticsEvent) -> None:
        """Placeholder pipeline for ingesting analytics events."""

        logger.info(
            "analytics.service.process_event",
            extra={"event": event.name, "tenant": event.context.source},
        )
        # Future: persist into Postgres, stream to BigQuery, update caches.

    async def _fetch_user_metrics(self, session: AsyncSession, tenant_id: str) -> dict[str, int]:
        try:
            result = await session.execute(
                text(
                    """
                    SELECT COUNT(*) AS active_users
                    FROM auth.users
                    WHERE tenant_id = :tenant_id AND is_active = TRUE AND deleted_at IS NULL
                    """
                ),
                {"tenant_id": tenant_id},
            )
            row = result.one_or_none()
            if row and row.active_users is not None:
                return {"active_users": int(row.active_users)}
        except SQLAlchemyError as exc:
            logger.warning("analytics.metrics.users_failed", extra={"error": str(exc)})
        return {"active_users": 0}

    async def _fetch_order_metrics(
        self,
        session: AsyncSession,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
    ) -> dict[str, float]:
        try:
            timeframe_clause = self._build_timeframe_clause("created_at", timeframe)
            result = await session.execute(
                text(
                    """
                    SELECT
                        COUNT(*) AS orders,
                        COALESCE(SUM(total_amount), 0)::float AS revenue
                    FROM commerce.orders
                    WHERE tenant_id = :tenant_id AND deleted_at IS NULL
                    {timeframe_clause}
                    """
                    .format(timeframe_clause=timeframe_clause)
                ),
                {"tenant_id": tenant_id},
            )
            row = result.one_or_none()
            if row:
                return {
                    "orders": int(row.orders or 0),
                    "revenue": float(row.revenue or 0.0),
                }
        except SQLAlchemyError as exc:
            logger.warning("analytics.metrics.orders_failed", extra={"error": str(exc)})
        return {"orders": 0, "revenue": 0.0}

    async def _fetch_inventory_metrics(self, session: AsyncSession, tenant_id: str) -> dict[str, int]:
        try:
            result = await session.execute(
                text(
                    """
                    SELECT COUNT(*) AS inventory_warnings
                    FROM inventory.items
                    WHERE tenant_id = :tenant_id AND deleted_at IS NULL AND stock_warning = TRUE
                    """
                ),
                {"tenant_id": tenant_id},
            )
            row = result.one_or_none()
            if row and row.inventory_warnings is not None:
                return {"inventory_warnings": int(row.inventory_warnings)}
        except SQLAlchemyError as exc:
            logger.warning("analytics.metrics.inventory_failed", extra={"error": str(exc)})
        return {"inventory_warnings": 0}

    def _build_timeframe_clause(self, column: str, timeframe: AnalyticsTimeframe) -> str:
        if timeframe == AnalyticsTimeframe.ALL_TIME:
            return ""
        if timeframe == AnalyticsTimeframe.TODAY:
            return f"AND {column}::date = CURRENT_DATE"
        if timeframe == AnalyticsTimeframe.YESTERDAY:
            return f"AND {column}::date = CURRENT_DATE - INTERVAL '1 day'"
        if timeframe == AnalyticsTimeframe.THIS_WEEK:
            return f"AND {column} >= date_trunc('week', CURRENT_DATE::timestamp)"
        if timeframe == AnalyticsTimeframe.LAST_WEEK:
            return (
                "AND {column} >= date_trunc('week', CURRENT_DATE::timestamp) - INTERVAL '1 week' "
                "AND {column} < date_trunc('week', CURRENT_DATE::timestamp)"
            ).format(column=column)
        if timeframe == AnalyticsTimeframe.THIS_MONTH:
            return f"AND {column} >= date_trunc('month', CURRENT_DATE::timestamp)"
        if timeframe == AnalyticsTimeframe.LAST_MONTH:
            return (
                "AND {column} >= date_trunc('month', CURRENT_DATE::timestamp) - INTERVAL '1 month' "
                "AND {column} < date_trunc('month', CURRENT_DATE::timestamp)"
            ).format(column=column)
        if timeframe == AnalyticsTimeframe.YEAR_TO_DATE:
            return f"AND {column} >= date_trunc('year', CURRENT_DATE::timestamp)"
        return ""

    def ensure_bigquery_dataset(self) -> None:
        """Ensure the BigQuery dataset exists; safe to call repeatedly."""

        try:
            self._bigquery.ensure_dataset()
        except RuntimeError as exc:
            logger.info("analytics.bigquery.skipped", extra={"reason": str(exc)})
