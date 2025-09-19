"""Cohort analytics utilities."""

from __future__ import annotations

from datetime import datetime, timedelta
from typing import Dict, List, Sequence

import pandas as pd
from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.enhanced_analytics import (
    CohortAnalysis,
    CohortAnalyticsResponse,
    CohortPeriodMetrics,
)

_PERIOD_FREQ_MAP: Dict[str, str] = {
    "day": "D",
    "week": "W",
    "month": "M",
}

_LABEL_FORMAT: Dict[str, str] = {
    "day": "%Y-%m-%d",
    "week": "%Y-W%V",
    "month": "%Y-%m",
}


class CohortAnalyticsService:
    """Provide customer cohort retention analytics."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]) -> None:
        self._session_factory = session_factory

    async def generate_cohort_analysis(
        self,
        tenant_id: str,
        *,
        start_date: datetime | None = None,
        end_date: datetime | None = None,
        granularity: str = "month",
        max_periods: int = 6,
    ) -> CohortAnalyticsResponse:
        """Return cohort retention metrics for the requested tenant."""

        start_date = start_date or datetime.utcnow() - timedelta(days=180)
        end_date = end_date or datetime.utcnow()
        granularity = granularity.lower()
        period_freq = _PERIOD_FREQ_MAP.get(granularity, "M")

        orders = await self._fetch_order_history(
            tenant_id,
            start_date=start_date,
            end_date=end_date,
        )

        if not orders:
            return CohortAnalyticsResponse(
                tenant_id=tenant_id,
                period_granularity=granularity,
                cohorts=[],
                average_retention_rate=0.0,
                period_labels=[f"Period {idx}" for idx in range(max_periods)],
            )

        df = self._build_dataframe(orders, period_freq)
        cohort_response = self._build_response(
            df,
            tenant_id=tenant_id,
            granularity=granularity,
            max_periods=max_periods,
        )

        logger.info(
            "analytics.cohort.generated",
            extra={
                "tenant": tenant_id,
                "granularity": granularity,
                "cohorts": len(cohort_response.cohorts),
            },
        )

        return cohort_response

    async def _fetch_order_history(
        self,
        tenant_id: str,
        *,
        start_date: datetime,
        end_date: datetime,
    ) -> List[Dict[str, object]]:
        query = text(
            """
            SELECT customer_id,
                   created_at::timestamp AS order_date,
                   COALESCE(total_amount, 0)::numeric AS total_amount
            FROM commerce.orders
            WHERE tenant_id = :tenant_id
              AND status NOT IN ('cancelled', 'refunded')
              AND created_at >= :start_date
              AND created_at <= :end_date
            ORDER BY created_at
            """
        )

        async with self._session_factory() as session:
            result = await session.execute(
                query,
                {
                    "tenant_id": tenant_id,
                    "start_date": start_date,
                    "end_date": end_date,
                },
            )
            rows = result.fetchall()

        orders: List[Dict[str, object]] = []
        for row in rows:
            orders.append(
                {
                    "customer_id": row.customer_id,
                    "order_date": row.order_date,
                    "total_amount": float(row.total_amount or 0.0),
                }
            )
        return orders

    def _build_dataframe(
        self,
        orders: Sequence[Dict[str, object]],
        period_freq: str,
    ) -> pd.DataFrame:
        df = pd.DataFrame(orders)
        df["order_date"] = pd.to_datetime(df["order_date"], utc=True).dt.tz_localize(None)
        df["order_period"] = df["order_date"].dt.to_period(period_freq)

        first_purchase = (
            df.groupby("customer_id")["order_period"].min().rename("cohort_period")
        )
        df = df.join(first_purchase, on="customer_id")

        order_ord = df["order_period"].astype("int64")
        cohort_ord = df["cohort_period"].astype("int64")
        df["period_index"] = (order_ord - cohort_ord).astype(int)
        df = df[df["period_index"] >= 0]

        df["cohort_label"] = df["cohort_period"].dt.to_timestamp()

        return df

    def _build_response(
        self,
        df: pd.DataFrame,
        *,
        tenant_id: str,
        granularity: str,
        max_periods: int,
    ) -> CohortAnalyticsResponse:
        cohort_sizes = df.groupby("cohort_label")["customer_id"].nunique()
        retention = (
            df.pivot_table(
                index="cohort_label",
                columns="period_index",
                values="customer_id",
                aggfunc=pd.Series.nunique,
                fill_value=0,
            )
            .sort_index(axis=1)
        )
        revenue = (
            df.pivot_table(
                index="cohort_label",
                columns="period_index",
                values="total_amount",
                aggfunc="sum",
                fill_value=0.0,
            )
            .sort_index(axis=1)
        )

        if retention.empty:
            return CohortAnalyticsResponse(
                tenant_id=tenant_id,
                period_granularity=granularity,
                cohorts=[],
                average_retention_rate=0.0,
                period_labels=[f"Period {idx}" for idx in range(max_periods)],
            )

        period_indices = [idx for idx in retention.columns if idx < max_periods]
        if not period_indices:
            period_indices = [0]
        period_labels = [f"Period {int(idx)}" for idx in period_indices]

        retention_rates = retention.divide(cohort_sizes, axis=0).fillna(0.0)

        cohorts: List[CohortAnalysis] = []
        label_format = _LABEL_FORMAT.get(granularity, "%Y-%m")
        for cohort_ts, cohort_size in cohort_sizes.items():
            cohort_key = cohort_ts.strftime(label_format)
            period_metrics: List[CohortPeriodMetrics] = []
            retention_row = (
                retention.loc[cohort_ts].reindex(period_indices, fill_value=0)
                if cohort_ts in retention.index
                else pd.Series(index=period_indices, data=0)
            )
            revenue_row = (
                revenue.loc[cohort_ts].reindex(period_indices, fill_value=0.0)
                if cohort_ts in revenue.index
                else pd.Series(index=period_indices, data=0.0)
            )
            rate_row = (
                retention_rates.loc[cohort_ts].reindex(period_indices, fill_value=0.0)
                if cohort_ts in retention_rates.index
                else pd.Series(index=period_indices, data=0.0)
            )

            for idx in period_indices:
                customers_active = int(retention_row.loc[idx])
                revenue_value = float(revenue_row.loc[idx])
                retention_rate = float(rate_row.loc[idx])
                period_metrics.append(
                    CohortPeriodMetrics(
                        period_index=int(idx),
                        period_label=f"Period {idx}",
                        customers_active=customers_active,
                        revenue=round(revenue_value, 2),
                        retention_rate=min(max(retention_rate, 0.0), 1.0),
                    )
                )

            avg_retention = (
                float(rate_row.mean()) if not rate_row.empty else 0.0
            )
            lifetime_value = float(revenue_row.sum()) if not revenue_row.empty else 0.0

            cohorts.append(
                CohortAnalysis(
                    cohort_key=cohort_key,
                    cohort_size=int(cohort_size),
                    periods=period_metrics,
                    average_retention=min(max(avg_retention, 0.0), 1.0),
                    lifetime_value=round(lifetime_value, 2),
                )
            )

        retention_subset = retention_rates.reindex(columns=period_indices, fill_value=0.0)
        overall_retention = (
            float(retention_subset.to_numpy().mean())
            if period_indices and not retention_subset.empty
            else 0.0
        )
        best_cohort = max(
            cohorts,
            key=lambda c: (c.average_retention, c.lifetime_value),
            default=None,
        )

        return CohortAnalyticsResponse(
            tenant_id=tenant_id,
            period_granularity=granularity,
            cohorts=cohorts,
            average_retention_rate=min(max(overall_retention, 0.0), 1.0),
            best_cohort=best_cohort.cohort_key if best_cohort else None,
            period_labels=period_labels,
        )
