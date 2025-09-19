"""Anomaly detection for analytics metrics."""

from __future__ import annotations

from datetime import datetime, timedelta
from typing import List, Optional

import pandas as pd
from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger


class AnomalyDetectionService:
    """Provides simple statistical anomaly detection for business metrics."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]) -> None:
        self._session_factory = session_factory

    async def detect_revenue_anomalies(
        self,
        tenant_id: str,
        *,
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None,
        threshold: float = 2.5,
    ) -> List[dict[str, object]]:
        """Detect anomalous daily revenue values for the tenant."""

        end_date = end_date or datetime.utcnow()
        start_date = start_date or end_date - timedelta(days=30)

        query = text(
            """
            SELECT date_trunc('day', created_at)::date AS bucket,
                   COALESCE(SUM(total_amount), 0)::numeric AS revenue
            FROM commerce.orders
            WHERE tenant_id = :tenant_id
              AND status NOT IN ('cancelled', 'refunded')
              AND created_at >= :start_date
              AND created_at <= :end_date
            GROUP BY 1
            ORDER BY 1
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

        if not rows:
            return []

        df = pd.DataFrame(rows, columns=["bucket", "revenue"])
        df["bucket"] = pd.to_datetime(df["bucket"], utc=True).dt.tz_localize(None)
        df["revenue"] = df["revenue"].astype(float)

        mean = df["revenue"].mean()
        std = df["revenue"].std(ddof=0)
        if std == 0:
            logger.info(
                "analytics.anomaly.no_variance",
                extra={"tenant": tenant_id, "period_days": (end_date - start_date).days},
            )
            return []

        df["z_score"] = (df["revenue"] - mean) / std
        df["severity"] = df["z_score"].abs()

        anomalies = df[df["z_score"].abs() >= threshold].copy()
        logger.info(
            "analytics.anomaly.detected",
            extra={
                "tenant": tenant_id,
                "count": len(anomalies),
                "threshold": threshold,
            },
        )

        return [
            {
                "timestamp": row.bucket.isoformat(),
                "value": round(row.revenue, 2),
                "z_score": round(row.z_score, 3),
                "severity": round(row.severity, 3),
            }
            for row in anomalies.itertuples()
        ]
