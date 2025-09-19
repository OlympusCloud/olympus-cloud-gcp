from __future__ import annotations

from datetime import date, datetime
from typing import Any, Optional, Tuple

from sqlalchemy import text
from sqlalchemy.exc import SQLAlchemyError
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.analytics import (
    AnalyticsDashboardResponse,
    AnalyticsMetrics,
    AnalyticsTimeframe,
    CustomerMetrics,
    InventoryMetrics,
    SalesForecast,
    SalesMetrics,
    SalesSnapshot,
    SalesTrend,
    StaffMetrics,
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
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> AnalyticsDashboardResponse:
        """Return high-level dashboard metrics for a tenant."""

        async with self._session_factory() as session:
            user_metrics = await self._fetch_user_metrics(session, tenant_id, location_id)
            order_metrics = await self._fetch_order_metrics(
                session,
                tenant_id,
                timeframe,
                location_id=location_id,
                start_date=start_date,
                end_date=end_date,
            )
            inventory_metrics = await self._fetch_inventory_metrics(session, tenant_id, location_id)

            metrics = self._build_metrics(
                timeframe=timeframe,
                active_users=user_metrics["active_users"],
                orders=order_metrics["orders"],
                revenue=order_metrics["revenue"],
                inventory_warnings=inventory_metrics["inventory_warnings"],
            )

            await self._record_metrics_snapshot(
                session,
                tenant_id,
                metrics,
                active_users=user_metrics["active_users"],
                orders=order_metrics["orders"],
                revenue=order_metrics["revenue"],
                inventory_warnings=inventory_metrics["inventory_warnings"],
                location_id=location_id,
                start_date=start_date,
                end_date=end_date,
            )

        return AnalyticsDashboardResponse(tenant_id=tenant_id, metrics=metrics)

    async def process_event(self, event: AnalyticsEvent) -> None:
        """Ingest analytics events and forward them to downstream systems."""

        logger.info(
            "analytics.service.process_event",
            extra={"event": event.name, "tenant": event.context.source},
        )

        try:
            self._bigquery.record_event(event)
        except Exception as exc:  # noqa: BLE001
            logger.warning(
                "analytics.bigquery.record_event_error",
                extra={"error": str(exc), "event": event.name},
            )
        # Future: persist into Postgres, update caches, trigger ML pipelines.

    async def _fetch_user_metrics(
        self,
        session: AsyncSession,
        tenant_id: str,
        location_id: Optional[str] = None,
    ) -> dict[str, int]:
        try:
            conditions = [
                "tenant_id = :tenant_id",
                "is_active = TRUE",
                "deleted_at IS NULL",
            ]
            params: dict[str, Any] = {"tenant_id": tenant_id}

            if location_id:
                conditions.append(":location_id = ANY(location_ids)")
                params["location_id"] = location_id

            query = text(
                "SELECT COUNT(*) AS active_users FROM auth.users WHERE "
                + " AND ".join(conditions)
            )

            result = await session.execute(query, params)
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
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> dict[str, float]:
        try:
            conditions = ["tenant_id = :tenant_id", "status NOT IN ('cancelled', 'refunded')"]
            params: dict[str, Any] = {"tenant_id": tenant_id}

            if location_id:
                conditions.append("location_id = :location_id")
                params["location_id"] = location_id

            timeframe_clause, timeframe_params = self._build_timeframe_clause(
                "created_at", timeframe, start_date, end_date
            )
            if timeframe_clause:
                conditions.append(timeframe_clause)
                params.update(timeframe_params)

            query = text(
                "SELECT COUNT(*) AS orders, COALESCE(SUM(total_amount), 0)::float AS revenue "
                "FROM commerce.orders WHERE "
                + " AND ".join(conditions)
            )

            result = await session.execute(query, params)
            row = result.one_or_none()
            if row:
                return {
                    "orders": int(row.orders or 0),
                    "revenue": float(row.revenue or 0.0),
                }
        except SQLAlchemyError as exc:
            logger.warning("analytics.metrics.orders_failed", extra={"error": str(exc)})
        return {"orders": 0, "revenue": 0.0}

    async def _fetch_inventory_metrics(
        self,
        session: AsyncSession,
        tenant_id: str,
        location_id: Optional[str] = None,
    ) -> dict[str, int]:
        try:
            conditions = ["tenant_id = :tenant_id"]
            params: dict[str, Any] = {"tenant_id": tenant_id}

            if location_id:
                conditions.append("location_id = :location_id")
                params["location_id"] = location_id

            conditions.append(
                "( (min_stock_level IS NOT NULL AND quantity_available <= min_stock_level)"
                " OR (reorder_point IS NOT NULL AND quantity_available <= reorder_point)"
                " OR count_required = TRUE )"
            )

            query = text(
                "SELECT COUNT(*) AS inventory_warnings FROM inventory.stock WHERE "
                + " AND ".join(conditions)
            )

            result = await session.execute(query, params)
            row = result.one_or_none()
            if row and row.inventory_warnings is not None:
                return {"inventory_warnings": int(row.inventory_warnings)}
        except SQLAlchemyError as exc:
            logger.warning("analytics.metrics.inventory_failed", extra={"error": str(exc)})
        return {"inventory_warnings": 0}

    def _build_metrics(
        self,
        *,
        timeframe: AnalyticsTimeframe,
        active_users: int,
        orders: int,
        revenue: float,
        inventory_warnings: int,
    ) -> AnalyticsMetrics:
        avg_order = revenue / orders if orders else 0.0
        stockout_risk = 1.0 if inventory_warnings > 0 else 0.0

        sales = SalesMetrics(
            today=SalesSnapshot(revenue=revenue, orders=orders, avg_order=avg_order),
            trend=SalesTrend(revenue=revenue, orders=orders),
            forecast=SalesForecast(),
        )
        customers = CustomerMetrics(total=active_users, new_today=0, active=active_users)
        inventory = InventoryMetrics(
            low_stock_items=inventory_warnings,
            stockout_risk=stockout_risk,
            total_value=0.0,
        )
        staff = StaffMetrics()

        return AnalyticsMetrics(
            timeframe=timeframe,
            sales=sales,
            customers=customers,
            inventory=inventory,
            staff=staff,
        )

    async def _record_metrics_snapshot(
        self,
        session: AsyncSession,
        tenant_id: str,
        metrics: AnalyticsMetrics,
        *,
        active_users: int,
        orders: int,
        revenue: float,
        inventory_warnings: int,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> None:
        snapshot = {
            "tenant_id": tenant_id,
            "timeframe": metrics.timeframe.value,
            "active_users": active_users,
            "orders": orders,
            "revenue": revenue,
            "inventory_warnings": inventory_warnings,
            "location_id": location_id,
            "start_date": start_date,
            "end_date": end_date,
            "captured_at": datetime.utcnow(),
        }

        await self._write_snapshot_to_postgres(session, snapshot)

        try:
            self._bigquery.record_snapshot(snapshot)
        except Exception as exc:  # noqa: BLE001
            logger.warning(
                "analytics.snapshot.bigquery_failed",
                extra={"error": str(exc), "tenant": tenant_id},
            )

    async def _write_snapshot_to_postgres(
        self,
        session: AsyncSession,
        snapshot: dict[str, Any],
    ) -> None:
        try:
            await session.execute(
                text(
                    """
                    INSERT INTO analytics.metric_snapshots (
                        tenant_id,
                        timeframe,
                        active_users,
                        orders,
                        revenue,
                        inventory_warnings,
                        location_id,
                        start_date,
                        end_date,
                        captured_at
                    ) VALUES (
                        :tenant_id,
                        :timeframe,
                        :active_users,
                        :orders,
                        :revenue,
                        :inventory_warnings,
                        :location_id,
                        :start_date,
                        :end_date,
                        :captured_at
                    )
                    """
                ),
                snapshot,
            )
            await session.commit()
        except SQLAlchemyError as exc:
            await session.rollback()
            logger.warning(
                "analytics.snapshot.postgres_failed",
                extra={"error": str(exc), "tenant": snapshot.get("tenant_id")},
            )

    def _build_timeframe_clause(
        self,
        column: str,
        timeframe: AnalyticsTimeframe,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> Tuple[str, dict[str, Any]]:
        params: dict[str, Any] = {}

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
                    "analytics.metrics.custom_timeframe_missing_bounds",
                    extra={"start": start_date, "end": end_date},
                )
                return "", params
            params.update({"start_date": start_date, "end_date": end_date})
            return f"{column}::date BETWEEN :start_date AND :end_date", params
        return "", params

    def ensure_bigquery_dataset(self) -> None:
        """Ensure the BigQuery dataset exists; safe to call repeatedly."""

        try:
            self._bigquery.ensure_dataset()
        except RuntimeError as exc:
            logger.info("analytics.bigquery.skipped", extra={"reason": str(exc)})
