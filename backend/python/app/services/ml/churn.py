"""Churn prediction heuristics."""

from __future__ import annotations

import math
from datetime import datetime, timedelta
from typing import List, Optional

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.churn import (
    ChurnPredictionResponse,
    ChurnRecommendation,
    ChurnSignal,
    ChurnSummary,
    CustomerChurnPrediction,
)


class ChurnPredictionService:
    """Estimate churn risk for customers using heuristic scoring."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]) -> None:
        self._session_factory = session_factory

    async def predict(
        self,
        tenant_id: str,
        *,
        limit: int = 100,
        lookback_days: int = 180,
    ) -> ChurnPredictionResponse:
        """Generate churn predictions for a tenant."""

        now = datetime.utcnow()
        lookback_start = now - timedelta(days=lookback_days)

        query = text(
            """
            SELECT
                c.id AS customer_id,
                c.email AS email,
                MAX(o.created_at) AS last_order_at,
                COUNT(o.id) AS total_orders,
                COUNT(o.id) FILTER (WHERE o.created_at >= :lookback_start) AS recent_orders,
                COALESCE(SUM(o.total_amount), 0)::numeric AS total_revenue,
                COALESCE(AVG(o.total_amount), 0)::numeric AS avg_order_value
            FROM customer.customers c
            LEFT JOIN commerce.orders o
              ON o.customer_id = c.id
             AND o.tenant_id = c.tenant_id
            WHERE c.tenant_id = :tenant_id
            GROUP BY c.id, c.email
            ORDER BY MAX(o.created_at) ASC NULLS FIRST
            LIMIT :limit
            """
        )

        async with self._session_factory() as session:
            result = await session.execute(
                query,
                {
                    "tenant_id": tenant_id,
                    "lookback_start": lookback_start,
                    "limit": limit,
                },
            )
            rows = result.fetchall()

        predictions: List[CustomerChurnPrediction] = []
        high = medium = low = 0

        for row in rows:
            last_order_at: Optional[datetime] = row.last_order_at
            days_since_last = (now - last_order_at).days if last_order_at else 999
            recent_orders = int(row.recent_orders or 0)
            total_orders = int(row.total_orders or 0)
            total_revenue = float(row.total_revenue or 0.0)
            avg_order_value = float(row.avg_order_value or 0.0)

            risk_score = self._score_customer(
                days_since_last=days_since_last,
                recent_orders=recent_orders,
                total_revenue=total_revenue,
                avg_order_value=avg_order_value,
            )
            risk_level = self._risk_level(risk_score)

            if risk_level == "high":
                high += 1
            elif risk_level == "medium":
                medium += 1
            else:
                low += 1

            signals = self._build_signals(days_since_last, recent_orders, avg_order_value)
            recommendations = self._build_recommendations(risk_level, avg_order_value)

            predictions.append(
                CustomerChurnPrediction(
                    customer_id=str(row.customer_id),
                    email=getattr(row, "email", None),
                    risk_score=risk_score,
                    risk_level=risk_level,
                    last_order_at=last_order_at,
                    total_orders=total_orders,
                    recent_orders=recent_orders,
                    total_revenue=round(total_revenue, 2),
                    average_order_value=round(avg_order_value, 2),
                    signals=signals,
                    recommendations=recommendations,
                )
            )

        logger.info(
            "ml.churn.predicted",
            extra={"tenant": tenant_id, "count": len(predictions)},
        )

        summary = ChurnSummary(
            high_risk=high,
            medium_risk=medium,
            low_risk=low,
        )

        return ChurnPredictionResponse(
            tenant_id=tenant_id,
            summary=summary,
            predictions=predictions,
        )

    def _score_customer(
        self,
        *,
        days_since_last: int,
        recent_orders: int,
        total_revenue: float,
        avg_order_value: float,
    ) -> float:
        recency_component = min(days_since_last / 120.0, 3.0)
        frequency_component = -0.6 * min(recent_orders, 6)
        revenue_component = -0.002 * min(total_revenue, 2000.0)
        basket_component = -0.004 * min(avg_order_value, 400.0)

        raw_score = 0.65 + 0.25 * recency_component + frequency_component + revenue_component + basket_component
        probability = 1 / (1 + math.exp(-raw_score))
        return max(0.0, min(probability, 1.0))

    def _risk_level(self, score: float) -> str:
        if score >= 0.66:
            return "high"
        if score >= 0.35:
            return "medium"
        return "low"

    def _build_signals(
        self,
        days_since_last: int,
        recent_orders: int,
        avg_order_value: float,
    ) -> List[ChurnSignal]:
        signals: List[ChurnSignal] = []
        if days_since_last > 45:
            severity = min(days_since_last / 180.0, 1.0)
            signals.append(
                ChurnSignal(
                    reason=f"Last order {days_since_last} days ago",
                    severity=round(severity, 2),
                )
            )
        if recent_orders == 0:
            signals.append(ChurnSignal(reason="No orders in the recent window", severity=0.85))
        elif recent_orders <= 1:
            signals.append(ChurnSignal(reason="Only one recent order", severity=0.55))
        if avg_order_value < 30:
            signals.append(ChurnSignal(reason="Low average order value", severity=0.4))
        return signals

    def _build_recommendations(
        self,
        risk_level: str,
        avg_order_value: float,
    ) -> List[ChurnRecommendation]:
        if risk_level == "low":
            return []

        recommendations: List[ChurnRecommendation] = []
        if risk_level == "high":
            recommendations.append(
                ChurnRecommendation(
                    title="Launch win-back campaign",
                    description="Send a personalized offer with an incentive to return before the next billing cycle.",
                )
            )
        if avg_order_value < 30:
            recommendations.append(
                ChurnRecommendation(
                    title="Promote bundle upgrades",
                    description="Suggest higher-value bundles or add-ons to increase engagement and order value.",
                )
            )
        if not recommendations:
            recommendations.append(
                ChurnRecommendation(
                    title="Schedule success call",
                    description="Have customer success reach out to reinforce value and gather feedback.",
                )
            )
        return recommendations
