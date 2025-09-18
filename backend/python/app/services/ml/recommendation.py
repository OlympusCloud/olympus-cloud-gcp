from __future__ import annotations

from dataclasses import dataclass
from datetime import date
from typing import Iterable, List, Optional
from uuid import uuid4

from app.models.analytics import (
    AnalyticsDashboardResponse,
    AnalyticsMetrics,
    AnalyticsTimeframe,
)
from app.models.recommendations import (
    Recommendation,
    RecommendationCategory,
    RecommendationInsight,
    RecommendationPriority,
    RecommendationResponse,
)
from app.services.analytics.service import AnalyticsService


@dataclass
class RecommendationContext:
    """Parameters controlling recommendation generation."""

    tenant_id: str
    timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME
    location_id: Optional[str] = None
    start_date: Optional[date] = None
    end_date: Optional[date] = None
    limit: Optional[int] = None


class RecommendationService:
    """Generate actionable business recommendations from analytics metrics."""

    def __init__(self, analytics_service: AnalyticsService) -> None:
        self._analytics_service = analytics_service

    async def generate(self, context: RecommendationContext) -> RecommendationResponse:
        """Produce recommendations for the supplied tenant and filters."""

        dashboard = await self._analytics_service.get_dashboard_metrics(
            context.tenant_id,
            timeframe=context.timeframe,
            location_id=context.location_id,
            start_date=context.start_date,
            end_date=context.end_date,
        )

        recommendations = self._build_recommendations(dashboard)
        if context.limit is not None and context.limit >= 0:
            recommendations = recommendations[: context.limit]

        return RecommendationResponse(tenant_id=context.tenant_id, recommendations=recommendations)

    def _build_recommendations(self, dashboard: AnalyticsDashboardResponse) -> List[Recommendation]:
        metrics = dashboard.metrics
        recommendations: List[Recommendation] = []

        recommendations.extend(self._inventory_recommendations(metrics))
        recommendations.extend(self._conversion_recommendations(metrics))
        recommendations.extend(self._growth_recommendations(metrics))

        if not recommendations:
            recommendations.append(self._baseline_recommendation(metrics))

        return recommendations

    def _inventory_recommendations(self, metrics: AnalyticsMetrics) -> Iterable[Recommendation]:
        if metrics.inventory_warnings <= 0:
            return []

        return [
            Recommendation(
                id=self._make_id("inventory"),
                category=RecommendationCategory.INVENTORY,
                priority=RecommendationPriority.HIGH,
                action="Prioritize replenishment for at-risk items",
                impact="Prevent stockouts and lost revenue",
                rationale="Inventory alerts indicate items are below safe stock levels.",
                insights=[
                    RecommendationInsight(
                        metric="Inventory warnings",
                        value=str(metrics.inventory_warnings),
                        context="Number of SKUs flagged for low inventory",
                    ),
                    RecommendationInsight(
                        metric="Orders",
                        value=str(metrics.orders),
                        context="Recent demand signals to inform replenishment volume",
                    ),
                ],
            )
        ]

    def _conversion_recommendations(self, metrics: AnalyticsMetrics) -> Iterable[Recommendation]:
        if metrics.active_users == 0:
            return []

        conversion_rate = metrics.orders / max(metrics.active_users, 1)
        if metrics.active_users >= 20 and conversion_rate < 0.15:
            return [
                Recommendation(
                    id=self._make_id("marketing"),
                    category=RecommendationCategory.MARKETING,
                    priority=RecommendationPriority.MEDIUM,
                    action="Launch targeted campaign to improve conversion",
                    impact="Lift conversion from engaged customers",
                    rationale="Large engaged audience with muted order volume signals a conversion gap.",
                    insights=[
                        RecommendationInsight(
                            metric="Active users",
                            value=str(metrics.active_users),
                            context="Audience currently engaging with the platform",
                        ),
                        RecommendationInsight(
                            metric="Orders",
                            value=str(metrics.orders),
                            context="Orders in the selected timeframe",
                        ),
                        RecommendationInsight(
                            metric="Conversion rate",
                            value=f"{conversion_rate:.2%}",
                            context="Orders divided by active users",
                        ),
                    ],
                )
            ]

        return []

    def _growth_recommendations(self, metrics: AnalyticsMetrics) -> Iterable[Recommendation]:
        if metrics.revenue <= 0:
            return []

        if metrics.revenue >= 100000 and metrics.orders >= 100:
            priority = RecommendationPriority.LOW
            rationale = "Healthy revenue trend supports investment in operational efficiencies."
            action = "Automate fulfillment workflows to sustain growth"
        else:
            priority = RecommendationPriority.MEDIUM
            rationale = "Revenue momentum can be amplified with cross-sell programs."
            action = "Introduce personalized bundles for repeat customers"

        return [
            Recommendation(
                id=self._make_id("operations"),
                category=RecommendationCategory.OPERATIONS,
                priority=priority,
                action=action,
                impact="Improve margin while maintaining customer satisfaction",
                rationale=rationale,
                insights=[
                    RecommendationInsight(
                        metric="Revenue",
                        value=self._format_currency(metrics.revenue),
                        context="Gross revenue for the selected timeframe",
                    ),
                    RecommendationInsight(
                        metric="Orders",
                        value=str(metrics.orders),
                        context="Order count supporting the revenue figure",
                    ),
                ],
            )
        ]

    def _baseline_recommendation(self, metrics: AnalyticsMetrics) -> Recommendation:
        return Recommendation(
            id=self._make_id("baseline"),
            category=RecommendationCategory.OPERATIONS,
            priority=RecommendationPriority.LOW,
            action="Monitor analytics dashboard for new opportunities",
            impact="Stay alert to shifts in customer behaviour",
            rationale="Current metrics do not surface urgent actions, but continued monitoring is advised.",
            insights=[
                RecommendationInsight(
                    metric="Active users",
                    value=str(metrics.active_users),
                    context="Engagement level for the timeframe",
                ),
                RecommendationInsight(
                    metric="Revenue",
                    value=self._format_currency(metrics.revenue),
                    context="Overall sales performance",
                ),
            ],
        )

    def _format_currency(self, value: float) -> str:
        return f"${value:,.2f}"

    def _make_id(self, prefix: str) -> str:
        return f"rec-{prefix}-{uuid4().hex[:8]}"
