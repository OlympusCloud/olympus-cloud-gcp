from __future__ import annotations

from datetime import date, datetime
from typing import Any, Dict, Iterable, List, Optional, Tuple
from uuid import UUID

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.analytics import AnalyticsTimeframe
from app.models.retail import (
    RetailAnalytics,
    RetailChannelPerformance,
    RetailProductPerformance,
    RetailPromotionPerformance,
    RetailRecommendation,
    RetailStockAlert,
)


class RetailService:
    """Business intelligence and insight generator for retail tenants."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]):
        self._session_factory = session_factory

    async def get_retail_analytics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> RetailAnalytics:
        """Aggregate retail KPIs including product, channel, and promotion performance."""

        order_clause, order_params = self._build_order_clause(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )

        async with self._session_factory() as session:
            metrics_row = await self._fetch_one(
                session,
                text(
                    f"""
                    SELECT
                        COALESCE(SUM(o.total_amount), 0) AS total_revenue,
                        COALESCE(AVG(o.total_amount), 0) AS average_order_value,
                        COUNT(DISTINCT o.id) AS total_orders,
                        COALESCE(SUM(CASE WHEN o.is_repeat_customer THEN 1 ELSE 0 END), 0) AS repeat_orders,
                        COALESCE(COUNT(DISTINCT o.customer_id), 0) AS unique_customers,
                        COALESCE(SUM(oi.quantity), 0) AS units_sold,
                        COALESCE(SUM(p.stock_on_hand), 0) AS inventory_units,
                        COALESCE(SUM(o.total_cost), 0) AS total_cost,
                        COALESCE(SUM(b.scans), 0) AS barcode_scans
                    FROM commerce.orders o
                    LEFT JOIN commerce.order_items oi ON oi.order_id = o.id
                    LEFT JOIN commerce.products p ON p.id = oi.product_id
                    LEFT JOIN analytics.daily_barcode_activity b
                        ON b.tenant_id = o.tenant_id
                        AND b.location_id = o.location_id
                        AND b.date = o.created_at::date
                    WHERE {order_clause}
                    """
                ),
                order_params,
            )

            product_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        CAST(p.id AS TEXT) AS product_id,
                        p.name AS product_name,
                        p.category,
                        COALESCE(SUM(oi.quantity), 0) AS units_sold,
                        COALESCE(SUM(oi.quantity * oi.unit_price), 0) AS revenue,
                        COALESCE(MAX(p.stock_on_hand), 0) AS stock_on_hand,
                        COALESCE(AVG(oi.unit_price - COALESCE(oi.unit_cost, 0)), 0) AS gross_margin
                    FROM commerce.orders o
                    JOIN commerce.order_items oi ON oi.order_id = o.id
                    JOIN commerce.products p ON p.id = oi.product_id
                    WHERE {order_clause}
                    GROUP BY p.id, p.name, p.category
                    ORDER BY revenue DESC
                    LIMIT 10
                    """
                ),
                order_params,
            )

            channel_rows = await self._fetch_all(
                session,
                text(
                    f"""
                    SELECT
                        COALESCE(o.sales_channel, 'unknown') AS sales_channel,
                        COALESCE(SUM(o.total_amount), 0) AS revenue,
                        COUNT(*) AS orders,
                        COALESCE(
                            SUM(o.total_amount) FILTER (
                                WHERE o.created_at >= CURRENT_DATE - INTERVAL '14 days'
                                  AND o.created_at < CURRENT_DATE - INTERVAL '7 days'
                            ),
                            0
                        ) AS previous_revenue,
                        COALESCE(AVG(o.total_amount), 0) AS average_order_value
                    FROM commerce.orders o
                    WHERE {order_clause}
                    GROUP BY sales_channel
                    ORDER BY revenue DESC
                    """
                ),
                order_params,
            )

            promotion_rows = await self._fetch_all(
                session,
                text(
                    """
                    SELECT
                        CAST(pr.id AS TEXT) AS promotion_id,
                        pr.name,
                        pr.type,
                        COALESCE(pr.status, 'active') AS status,
                        COALESCE(SUM(o.total_amount), 0) AS revenue,
                        COALESCE(AVG(pp.uplift_percentage), 0) AS uplift
                    FROM marketing.promotions pr
                    LEFT JOIN marketing.promotion_performance pp ON pp.promotion_id = pr.id
                    LEFT JOIN commerce.orders o ON o.promotion_id = pr.id
                    WHERE pr.tenant_id = :tenant_id
                      AND (:location_id IS NULL OR pr.location_id = :location_id)
                    GROUP BY pr.id, pr.name, pr.type, pr.status
                    ORDER BY revenue DESC
                    LIMIT 10
                    """
                ),
                {"tenant_id": tenant_id, "location_id": location_id},
            )

            stock_alert_rows = await self._fetch_all(
                session,
                text(
                    """
                    SELECT
                        CAST(sa.product_id AS TEXT) AS product_id,
                        sa.sku,
                        sa.product_name,
                        sa.status,
                        COALESCE(sa.current_stock, 0) AS current_stock,
                        COALESCE(sa.threshold, 0) AS threshold,
                        COALESCE(sa.days_of_cover, 0) AS days_of_cover
                    FROM inventory.stock_alerts sa
                    WHERE sa.tenant_id = :tenant_id
                      AND (:location_id IS NULL OR sa.location_id = :location_id)
                    ORDER BY sa.days_of_cover ASC
                    LIMIT 10
                    """
                ),
                {"tenant_id": tenant_id, "location_id": location_id},
            )

        total_revenue = float(self._row_value(metrics_row, "total_revenue", 0, 0.0))
        average_order_value = float(self._row_value(metrics_row, "average_order_value", 1, 0.0))
        total_orders = int(self._row_value(metrics_row, "total_orders", 2, 0))
        repeat_orders = int(self._row_value(metrics_row, "repeat_orders", 3, 0))
        unique_customers = int(self._row_value(metrics_row, "unique_customers", 4, 0))
        units_sold = int(self._row_value(metrics_row, "units_sold", 5, 0))
        inventory_units = int(self._row_value(metrics_row, "inventory_units", 6, 0))
        total_cost = float(self._row_value(metrics_row, "total_cost", 7, 0.0))
        barcode_scans = int(self._row_value(metrics_row, "barcode_scans", 8, 0))

        repeat_purchase_rate = self._safe_div(repeat_orders, total_orders)
        sell_through_rate = self._safe_div(units_sold, units_sold + inventory_units)
        inventory_turnover = self._safe_div(units_sold, inventory_units or 1)
        gross_margin_rate = self._safe_div(total_revenue - total_cost, total_revenue)
        customer_conversion_rate = self._safe_div(total_orders, barcode_scans) if barcode_scans else 0.0

        top_products = [
            self._build_product_performance(row)
            for row in product_rows
        ]

        channel_performance = [
            self._build_channel_performance(row)
            for row in channel_rows
        ]

        promotion_performance = [
            self._build_promotion_performance(row)
            for row in promotion_rows
        ]

        stock_alerts = [
            self._build_stock_alert(row)
            for row in stock_alert_rows
        ]

        return RetailAnalytics(
            tenant_id=UUID(tenant_id),
            location_id=UUID(location_id) if location_id else None,
            date=datetime.utcnow(),
            total_revenue=total_revenue,
            average_order_value=average_order_value,
            total_orders=total_orders,
            unique_customers=unique_customers,
            repeat_purchase_rate=self._clamp_rate(repeat_purchase_rate),
            inventory_turnover=inventory_turnover,
            sell_through_rate=self._clamp_rate(sell_through_rate),
            gross_margin_rate=gross_margin_rate,
            customer_conversion_rate=self._clamp_rate(customer_conversion_rate),
            barcode_scans=barcode_scans,
            top_products=top_products,
            channel_performance=channel_performance,
            promotion_performance=promotion_performance,
            stock_alerts=stock_alerts,
        )

    async def generate_promotions(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> List[RetailRecommendation]:
        """Generate actionable recommendations based on retail analytics."""

        analytics = await self.get_retail_analytics(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )

        recommendations: List[RetailRecommendation] = []

        if analytics.sell_through_rate < 0.45:
            recommendations.append(
                RetailRecommendation(
                    type="inventory",
                    title="Slow Moving Inventory",
                    description=(
                        "Sell-through rate is below 45%. Launch clearance promotions or product bundles "
                        "to accelerate sell-through."
                    ),
                    impact="medium",
                    priority=2,
                    data={"sell_through_rate": analytics.sell_through_rate},
                )
            )

        if analytics.repeat_purchase_rate < 0.30:
            recommendations.append(
                RetailRecommendation(
                    type="loyalty",
                    title="Launch Loyalty Incentives",
                    description=(
                        "Repeat purchase rate is under 30%. Consider loyalty rewards or targeted email "
                        "campaigns to improve retention."
                    ),
                    impact="high",
                    priority=1,
                    data={"repeat_rate": analytics.repeat_purchase_rate},
                )
            )

        declining_channels = [c for c in analytics.channel_performance if c.growth_rate < 0]
        if declining_channels:
            weakest_channel = min(declining_channels, key=lambda c: c.growth_rate)
            recommendations.append(
                RetailRecommendation(
                    type="channel",
                    title=f"Revitalize {weakest_channel.channel.title()} Channel",
                    description=(
                        f"{weakest_channel.channel.title()} revenue is declining. Allocate marketing spend or run "
                        "exclusive promotions to recover growth."
                    ),
                    impact="high",
                    priority=1,
                    data={
                        "channel": weakest_channel.channel,
                        "growth_rate": weakest_channel.growth_rate,
                        "revenue": weakest_channel.revenue,
                    },
                )
            )

        critical_alerts = [a for a in analytics.stock_alerts if a.status in {"low_stock", "out_of_stock", "reorder_needed"}]
        if critical_alerts:
            alert = critical_alerts[0]
            recommendations.append(
                RetailRecommendation(
                    type="inventory",
                    title="Resolve Stock Alert",
                    description=(
                        f"{alert.product_name} is {alert.status.replace('_', ' ')}. Expedite replenishment to avoid lost sales."
                    ),
                    impact="high",
                    priority=0,
                    data={
                        "product_id": alert.product_id,
                        "current_stock": alert.current_stock,
                        "threshold": alert.threshold,
                    },
                )
            )

        if analytics.top_products:
            top_product = analytics.top_products[0]
            if top_product.sell_through_rate > 0.7:
                recommendations.append(
                    RetailRecommendation(
                        type="cross_sell",
                        title=f"Promote {top_product.name} Bundles",
                        description=(
                            f"{top_product.name} is performing exceptionally well. Create bundles or featured "
                            "placements to maximize momentum."
                        ),
                        impact="medium",
                        priority=3,
                        data={
                            "product_id": top_product.product_id,
                            "sell_through_rate": top_product.sell_through_rate,
                            "revenue": top_product.revenue,
                        },
                    )
                )

        if not recommendations:
            recommendations.append(
                RetailRecommendation(
                    type="insight",
                    title="Retail Operations Performing Well",
                    description="Key metrics are healthy. Continue monitoring performance for emerging trends.",
                    impact="low",
                    priority=5,
                    data={},
                )
            )

        return sorted(recommendations, key=lambda rec: rec.priority)

    async def get_channel_performance(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME,
        *,
        location_id: Optional[str] = None,
        start_date: Optional[date] = None,
        end_date: Optional[date] = None,
    ) -> List[RetailChannelPerformance]:
        """Expose channel performance as a standalone helper for dashboards."""

        analytics = await self.get_retail_analytics(
            tenant_id,
            timeframe,
            location_id=location_id,
            start_date=start_date,
            end_date=end_date,
        )
        return analytics.channel_performance

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

    def _build_order_clause(
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
        conditions: List[str] = [
            "o.tenant_id = :tenant_id",
            "(:location_id IS NULL OR o.location_id = :location_id)",
        ]

        timeframe_clause, timeframe_params = self._build_timeframe_clause(
            "o.created_at",
            timeframe,
            start_date,
            end_date,
        )
        if timeframe_clause:
            conditions.append(timeframe_clause)
            params.update(timeframe_params)

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
                    "retail.analytics.custom_range_missing_bounds",
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

    def _build_product_performance(self, row: Any) -> RetailProductPerformance:
        units_sold = int(self._row_value(row, "units_sold", 3, 0))
        stock_on_hand = int(self._row_value(row, "stock_on_hand", 5, 0))
        sell_through_rate = self._safe_div(units_sold, units_sold + stock_on_hand)
        return RetailProductPerformance(
            product_id=str(self._row_value(row, "product_id", 0, "")),
            name=str(self._row_value(row, "product_name", 1, "")),
            category=self._row_value(row, "category", 2, None),
            revenue=float(self._row_value(row, "revenue", 4, 0.0)),
            units_sold=units_sold,
            stock_on_hand=stock_on_hand,
            sell_through_rate=self._clamp_rate(sell_through_rate),
            gross_margin=float(self._row_value(row, "gross_margin", 6, 0.0)),
        )

    def _build_channel_performance(self, row: Any) -> RetailChannelPerformance:
        revenue = float(self._row_value(row, "revenue", 1, 0.0))
        previous_revenue = float(self._row_value(row, "previous_revenue", 3, 0.0))
        growth_rate = 0.0
        if previous_revenue > 0:
            growth_rate = (revenue - previous_revenue) / previous_revenue
        elif revenue > 0:
            growth_rate = 1.0
        return RetailChannelPerformance(
            channel=str(self._row_value(row, "sales_channel", 0, "unknown")),
            revenue=revenue,
            orders=int(self._row_value(row, "orders", 2, 0)),
            growth_rate=growth_rate,
            average_order_value=float(self._row_value(row, "average_order_value", 4, 0.0)),
        )

    def _build_promotion_performance(self, row: Any) -> RetailPromotionPerformance:
        return RetailPromotionPerformance(
            promotion_id=str(self._row_value(row, "promotion_id", 0, "")),
            name=str(self._row_value(row, "name", 1, "")),
            type=str(self._row_value(row, "type", 2, "")),
            status=str(self._row_value(row, "status", 3, "active")),
            revenue=float(self._row_value(row, "revenue", 4, 0.0)),
            uplift=float(self._row_value(row, "uplift", 5, 0.0)),
        )

    def _build_stock_alert(self, row: Any) -> RetailStockAlert:
        return RetailStockAlert(
            product_id=str(self._row_value(row, "product_id", 0, "")),
            sku=self._row_value(row, "sku", 1, None),
            product_name=str(self._row_value(row, "product_name", 2, "")),
            status=str(self._row_value(row, "status", 3, "low_stock")),
            current_stock=int(self._row_value(row, "current_stock", 4, 0)),
            threshold=int(self._row_value(row, "threshold", 5, 0)),
            days_of_cover=float(self._row_value(row, "days_of_cover", 6, 0.0)),
        )

