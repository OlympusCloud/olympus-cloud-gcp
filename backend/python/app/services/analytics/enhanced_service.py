from __future__ import annotations

import time
from datetime import datetime, timedelta
from typing import Dict, List, Optional

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker
from sqlalchemy.exc import SQLAlchemyError

from app.core.logging import logger
from app.models.enhanced_analytics import (
    EnhancedDashboardMetrics,
    EnhancedAnalyticsResponse,
    DetailedMetric,
    MetricCategory,
    MetricTrend,
    TrendDirection,
    CustomerSegment,
    ProductPerformance,
    LocationMetrics,
    TimeSeriesData,
    TimeSeriesPoint,
    BusinessHealthScore,
    AnalyticsFilter
)
from app.models.analytics import AnalyticsTimeframe
from app.services.analytics.service import AnalyticsService


class EnhancedAnalyticsService:
    """Advanced analytics service with rich insights and detailed metrics."""
    
    def __init__(
        self,
        session_factory: async_sessionmaker[AsyncSession],
        base_analytics_service: AnalyticsService
    ):
        self._session_factory = session_factory
        self._base_service = base_analytics_service
    
    async def get_enhanced_dashboard(
        self,
        analytics_filter: AnalyticsFilter
    ) -> EnhancedAnalyticsResponse:
        """Get comprehensive dashboard with enhanced metrics and insights."""
        
        start_time = time.time()
        
        async with self._session_factory() as session:
            # Get enhanced dashboard metrics
            dashboard_metrics = await self._build_enhanced_dashboard(session, analytics_filter)
            
            # Calculate business health score
            business_health = await self._calculate_business_health(session, analytics_filter)
            
            # Generate time series data for charts
            time_series = await self._generate_time_series(session, analytics_filter)
            dashboard_metrics.time_series = time_series
            
            processing_time = (time.time() - start_time) * 1000
            
            return EnhancedAnalyticsResponse(
                dashboard_metrics=dashboard_metrics,
                business_health=business_health,
                processing_time_ms=processing_time,
                data_freshness="real-time",
                cache_hit=False
            )
    
    async def _build_enhanced_dashboard(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> EnhancedDashboardMetrics:
        """Build comprehensive dashboard metrics."""
        
        dashboard = EnhancedDashboardMetrics(
            tenant_id=analytics_filter.tenant_id,
            location_id=analytics_filter.location_ids[0] if analytics_filter.location_ids else None,
            timeframe=analytics_filter.timeframe
        )
        
        # Build key metrics with trends
        dashboard.key_metrics = await self._build_key_metrics(session, analytics_filter)
        
        # Get customer segments
        dashboard.customer_segments = await self._get_customer_segments(session, analytics_filter)
        
        # Get top products
        dashboard.top_products = await self._get_top_products(session, analytics_filter)
        
        # Get revenue breakdowns
        dashboard.revenue_by_category = await self._get_revenue_by_category(session, analytics_filter)
        dashboard.revenue_by_payment_method = await self._get_revenue_by_payment_method(session, analytics_filter)
        
        # Get location performance (if multi-location)
        if not analytics_filter.location_ids:
            dashboard.location_performance = await self._get_location_performance(session, analytics_filter)
        
        # Get operational metrics
        dashboard.avg_order_fulfillment_time = await self._get_avg_fulfillment_time(session, analytics_filter)
        dashboard.order_accuracy_rate = await self._get_order_accuracy_rate(session, analytics_filter)
        
        # Generate alerts and recommendations
        dashboard.alerts = await self._generate_alerts(session, analytics_filter)
        dashboard.recommendations = await self._generate_recommendations(session, analytics_filter)
        
        return dashboard
    
    async def _build_key_metrics(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[DetailedMetric]:
        """Build key performance indicators with trends."""
        
        metrics = []
        
        try:
            # Revenue metric
            revenue_data = await self._get_revenue_with_trend(session, analytics_filter)
            if revenue_data:
                metrics.append(DetailedMetric(
                    name="Total Revenue",
                    value=revenue_data["current"],
                    formatted_value=f"${revenue_data['current']:,.2f}",
                    category=MetricCategory.REVENUE,
                    trend=MetricTrend(
                        direction=TrendDirection.UP if revenue_data["change"] > 0 else TrendDirection.DOWN,
                        percentage_change=revenue_data["change"],
                        period_comparison="vs previous period",
                        confidence=0.9
                    ),
                    unit="USD",
                    description="Total revenue for the selected period"
                ))
            
            # Orders metric
            orders_data = await self._get_orders_with_trend(session, analytics_filter)
            if orders_data:
                metrics.append(DetailedMetric(
                    name="Total Orders",
                    value=orders_data["current"],
                    formatted_value=f"{orders_data['current']:,}",
                    category=MetricCategory.OPERATIONS,
                    trend=MetricTrend(
                        direction=TrendDirection.UP if orders_data["change"] > 0 else TrendDirection.DOWN,
                        percentage_change=orders_data["change"],
                        period_comparison="vs previous period",
                        confidence=0.95
                    ),
                    unit="orders",
                    description="Total number of orders processed"
                ))
            
            # Average Order Value
            if revenue_data and orders_data and orders_data["current"] > 0:
                aov = revenue_data["current"] / orders_data["current"]
                prev_aov = revenue_data["previous"] / orders_data["previous"] if orders_data["previous"] > 0 else 0
                aov_change = ((aov - prev_aov) / prev_aov * 100) if prev_aov > 0 else 0
                
                metrics.append(DetailedMetric(
                    name="Average Order Value",
                    value=aov,
                    formatted_value=f"${aov:.2f}",
                    category=MetricCategory.REVENUE,
                    trend=MetricTrend(
                        direction=TrendDirection.UP if aov_change > 0 else TrendDirection.DOWN,
                        percentage_change=aov_change,
                        period_comparison="vs previous period",
                        confidence=0.85
                    ),
                    unit="USD",
                    description="Average value per order"
                ))
            
            # Customer metrics
            customers_data = await self._get_customers_with_trend(session, analytics_filter)
            if customers_data:
                metrics.append(DetailedMetric(
                    name="Active Customers",
                    value=customers_data["current"],
                    formatted_value=f"{customers_data['current']:,}",
                    category=MetricCategory.CUSTOMERS,
                    trend=MetricTrend(
                        direction=TrendDirection.UP if customers_data["change"] > 0 else TrendDirection.DOWN,
                        percentage_change=customers_data["change"],
                        period_comparison="vs previous period",
                        confidence=0.8
                    ),
                    unit="customers",
                    description="Number of active customers"
                ))
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.key_metrics.error", extra={"error": str(e)})
        
        return metrics
    
    async def _get_revenue_with_trend(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[Dict[str, float]]:
        """Get revenue data with trend comparison."""
        
        try:
            # Current period
            current_query = text("""
                SELECT COALESCE(SUM(total_amount), 0) as revenue
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status NOT IN ('cancelled', 'refunded')
                AND created_at >= :start_date 
                AND created_at <= :end_date
            """)
            
            # Previous period (same duration)
            duration = self._get_period_duration(analytics_filter.timeframe)
            prev_start = analytics_filter.start_date - duration if analytics_filter.start_date else datetime.utcnow() - duration
            prev_end = analytics_filter.end_date - duration if analytics_filter.end_date else datetime.utcnow()
            
            prev_query = text("""
                SELECT COALESCE(SUM(total_amount), 0) as revenue
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status NOT IN ('cancelled', 'refunded')
                AND created_at >= :prev_start 
                AND created_at <= :prev_end
            """)
            
            # Execute queries
            current_result = await session.execute(current_query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            
            prev_result = await session.execute(prev_query, {
                "tenant_id": analytics_filter.tenant_id,
                "prev_start": prev_start,
                "prev_end": prev_end
            })
            
            current_revenue = float(current_result.scalar() or 0)
            prev_revenue = float(prev_result.scalar() or 0)
            
            change = ((current_revenue - prev_revenue) / prev_revenue * 100) if prev_revenue > 0 else 0
            
            return {
                "current": current_revenue,
                "previous": prev_revenue,
                "change": change
            }
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.revenue_trend.error", extra={"error": str(e)})
            return None
    
    async def _get_orders_with_trend(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[Dict[str, float]]:
        """Get orders data with trend comparison."""
        
        try:
            # Similar logic to revenue but counting orders
            current_query = text("""
                SELECT COUNT(*) as orders
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status NOT IN ('cancelled', 'refunded')
                AND created_at >= :start_date 
                AND created_at <= :end_date
            """)
            
            duration = self._get_period_duration(analytics_filter.timeframe)
            prev_start = analytics_filter.start_date - duration if analytics_filter.start_date else datetime.utcnow() - duration
            prev_end = analytics_filter.end_date - duration if analytics_filter.end_date else datetime.utcnow()
            
            prev_query = text("""
                SELECT COUNT(*) as orders
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status NOT IN ('cancelled', 'refunded')
                AND created_at >= :prev_start 
                AND created_at <= :prev_end
            """)
            
            current_result = await session.execute(current_query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            
            prev_result = await session.execute(prev_query, {
                "tenant_id": analytics_filter.tenant_id,
                "prev_start": prev_start,
                "prev_end": prev_end
            })
            
            current_orders = int(current_result.scalar() or 0)
            prev_orders = int(prev_result.scalar() or 0)
            
            change = ((current_orders - prev_orders) / prev_orders * 100) if prev_orders > 0 else 0
            
            return {
                "current": current_orders,
                "previous": prev_orders,
                "change": change
            }
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.orders_trend.error", extra={"error": str(e)})
            return None
    
    async def _get_customers_with_trend(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[Dict[str, float]]:
        """Get customer data with trend comparison."""
        
        try:
            current_query = text("""
                SELECT COUNT(DISTINCT customer_id) as customers
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND created_at >= :start_date 
                AND created_at <= :end_date
            """)
            
            duration = self._get_period_duration(analytics_filter.timeframe)
            prev_start = analytics_filter.start_date - duration if analytics_filter.start_date else datetime.utcnow() - duration
            prev_end = analytics_filter.end_date - duration if analytics_filter.end_date else datetime.utcnow()
            
            prev_query = text("""
                SELECT COUNT(DISTINCT customer_id) as customers
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND created_at >= :prev_start 
                AND created_at <= :prev_end
            """)
            
            current_result = await session.execute(current_query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            
            prev_result = await session.execute(prev_query, {
                "tenant_id": analytics_filter.tenant_id,
                "prev_start": prev_start,
                "prev_end": prev_end
            })
            
            current_customers = int(current_result.scalar() or 0)
            prev_customers = int(prev_result.scalar() or 0)
            
            change = ((current_customers - prev_customers) / prev_customers * 100) if prev_customers > 0 else 0
            
            return {
                "current": current_customers,
                "previous": prev_customers,
                "change": change
            }
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.customers_trend.error", extra={"error": str(e)})
            return None
    
    async def _get_customer_segments(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[CustomerSegment]:
        """Get customer segmentation data."""
        
        segments = []
        
        try:
            query = text("""
                WITH customer_stats AS (
                    SELECT 
                        customer_id,
                        COUNT(*) as order_count,
                        SUM(total_amount) as total_spent,
                        AVG(total_amount) as avg_order_value,
                        MAX(created_at) as last_order_date
                    FROM commerce.orders 
                    WHERE tenant_id = :tenant_id 
                    AND status NOT IN ('cancelled', 'refunded')
                    GROUP BY customer_id
                ),
                segments AS (
                    SELECT 
                        CASE 
                            WHEN total_spent >= 1000 THEN 'VIP'
                            WHEN total_spent >= 500 THEN 'High Value'
                            WHEN order_count = 1 THEN 'New Customer'
                            ELSE 'Regular'
                        END as segment,
                        COUNT(*) as customer_count,
                        AVG(avg_order_value) as avg_aov,
                        SUM(total_spent) as segment_revenue
                    FROM customer_stats
                    GROUP BY segment
                )
                SELECT 
                    segment,
                    customer_count,
                    avg_aov,
                    segment_revenue,
                    (customer_count * 100.0 / SUM(customer_count) OVER()) as percentage
                FROM segments
                ORDER BY segment_revenue DESC
            """)
            
            result = await session.execute(query, {"tenant_id": analytics_filter.tenant_id})
            rows = result.fetchall()
            
            for row in rows:
                segments.append(CustomerSegment(
                    segment_name=row.segment,
                    customer_count=int(row.customer_count),
                    percentage_of_total=float(row.percentage),
                    avg_order_value=float(row.avg_aov),
                    total_revenue=float(row.segment_revenue)
                ))
                
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.customer_segments.error", extra={"error": str(e)})
        
        return segments
    
    async def _get_top_products(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[ProductPerformance]:
        """Get top performing products."""
        
        products = []
        
        try:
            query = text("""
                SELECT 
                    oi.product_id,
                    p.name as product_name,
                    SUM(oi.quantity) as units_sold,
                    SUM(oi.quantity * oi.unit_price) as revenue
                FROM commerce.order_items oi
                JOIN commerce.orders o ON oi.order_id = o.id
                LEFT JOIN commerce.products p ON oi.product_id = p.id
                WHERE o.tenant_id = :tenant_id 
                AND o.status NOT IN ('cancelled', 'refunded')
                AND o.created_at >= :start_date 
                AND o.created_at <= :end_date
                GROUP BY oi.product_id, p.name
                ORDER BY revenue DESC
                LIMIT 10
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            rows = result.fetchall()
            
            for row in rows:
                products.append(ProductPerformance(
                    product_id=str(row.product_id),
                    product_name=row.product_name or "Unknown Product",
                    units_sold=int(row.units_sold),
                    revenue=float(row.revenue)
                ))
                
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.top_products.error", extra={"error": str(e)})
        
        return products
    
    async def _get_revenue_by_category(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Dict[str, float]:
        """Get revenue breakdown by product category."""
        
        try:
            query = text("""
                SELECT 
                    COALESCE(p.category, 'Uncategorized') as category,
                    SUM(oi.quantity * oi.unit_price) as revenue
                FROM commerce.order_items oi
                JOIN commerce.orders o ON oi.order_id = o.id
                LEFT JOIN commerce.products p ON oi.product_id = p.id
                WHERE o.tenant_id = :tenant_id 
                AND o.status NOT IN ('cancelled', 'refunded')
                AND o.created_at >= :start_date 
                AND o.created_at <= :end_date
                GROUP BY p.category
                ORDER BY revenue DESC
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            rows = result.fetchall()
            
            return {row.category: float(row.revenue) for row in rows}
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.revenue_by_category.error", extra={"error": str(e)})
            return {}
    
    async def _get_revenue_by_payment_method(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Dict[str, float]:
        """Get revenue breakdown by payment method."""
        
        try:
            query = text("""
                SELECT 
                    COALESCE(payment_method, 'Unknown') as method,
                    SUM(amount) as revenue
                FROM commerce.payments p
                JOIN commerce.orders o ON p.order_id = o.id
                WHERE o.tenant_id = :tenant_id 
                AND p.status = 'completed'
                AND o.created_at >= :start_date 
                AND o.created_at <= :end_date
                GROUP BY payment_method
                ORDER BY revenue DESC
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            rows = result.fetchall()
            
            return {row.method: float(row.revenue) for row in rows}
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.revenue_by_payment.error", extra={"error": str(e)})
            return {}
    
    async def _get_location_performance(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[LocationMetrics]:
        """Get performance metrics by location."""
        
        locations = []
        
        try:
            query = text("""
                SELECT 
                    o.location_id,
                    l.name as location_name,
                    COUNT(*) as orders,
                    COUNT(DISTINCT o.customer_id) as customers,
                    SUM(o.total_amount) as revenue,
                    AVG(o.total_amount) as avg_order_value
                FROM commerce.orders o
                LEFT JOIN platform.locations l ON o.location_id = l.id
                WHERE o.tenant_id = :tenant_id 
                AND o.status NOT IN ('cancelled', 'refunded')
                AND o.created_at >= :start_date 
                AND o.created_at <= :end_date
                GROUP BY o.location_id, l.name
                ORDER BY revenue DESC
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            rows = result.fetchall()
            
            for rank, row in enumerate(rows, 1):
                locations.append(LocationMetrics(
                    location_id=str(row.location_id) if row.location_id else "unknown",
                    location_name=row.location_name or "Unknown Location",
                    revenue=float(row.revenue),
                    orders=int(row.orders),
                    customers=int(row.customers),
                    avg_order_value=float(row.avg_order_value),
                    performance_rank=rank
                ))
                
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.location_performance.error", extra={"error": str(e)})
        
        return locations
    
    async def _generate_time_series(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[TimeSeriesData]:
        """Generate time series data for charts."""
        
        time_series = []
        
        try:
            # Revenue time series
            revenue_series = await self._get_revenue_time_series(session, analytics_filter)
            if revenue_series:
                time_series.append(revenue_series)
            
            # Orders time series
            orders_series = await self._get_orders_time_series(session, analytics_filter)
            if orders_series:
                time_series.append(orders_series)
                
        except Exception as e:
            logger.error("enhanced_analytics.time_series.error", extra={"error": str(e)})
        
        return time_series
    
    async def _get_revenue_time_series(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[TimeSeriesData]:
        """Get revenue time series data."""
        
        try:
            query = text("""
                SELECT 
                    DATE(created_at) as date,
                    SUM(total_amount) as revenue
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status NOT IN ('cancelled', 'refunded')
                AND created_at >= :start_date 
                AND created_at <= :end_date
                GROUP BY DATE(created_at)
                ORDER BY date
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            rows = result.fetchall()
            
            data_points = [
                TimeSeriesPoint(
                    timestamp=datetime.combine(row.date, datetime.min.time()),
                    value=float(row.revenue),
                    label=f"${row.revenue:,.2f}"
                )
                for row in rows
            ]
            
            return TimeSeriesData(
                metric_name="Daily Revenue",
                data_points=data_points,
                timeframe=analytics_filter.timeframe,
                unit="USD"
            )
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.revenue_time_series.error", extra={"error": str(e)})
            return None
    
    async def _get_orders_time_series(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[TimeSeriesData]:
        """Get orders time series data."""
        
        try:
            query = text("""
                SELECT 
                    DATE(created_at) as date,
                    COUNT(*) as orders
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status NOT IN ('cancelled', 'refunded')
                AND created_at >= :start_date 
                AND created_at <= :end_date
                GROUP BY DATE(created_at)
                ORDER BY date
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            rows = result.fetchall()
            
            data_points = [
                TimeSeriesPoint(
                    timestamp=datetime.combine(row.date, datetime.min.time()),
                    value=float(row.orders),
                    label=f"{row.orders} orders"
                )
                for row in rows
            ]
            
            return TimeSeriesData(
                metric_name="Daily Orders",
                data_points=data_points,
                timeframe=analytics_filter.timeframe,
                unit="orders"
            )
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.orders_time_series.error", extra={"error": str(e)})
            return None
    
    def _get_period_duration(self, timeframe: AnalyticsTimeframe) -> timedelta:
        """Get the duration for a given timeframe."""
        
        duration_map = {
            AnalyticsTimeframe.TODAY: timedelta(days=1),
            AnalyticsTimeframe.YESTERDAY: timedelta(days=1),
            AnalyticsTimeframe.THIS_WEEK: timedelta(weeks=1),
            AnalyticsTimeframe.LAST_WEEK: timedelta(weeks=1),
            AnalyticsTimeframe.THIS_MONTH: timedelta(days=30),
            AnalyticsTimeframe.LAST_MONTH: timedelta(days=30),
            AnalyticsTimeframe.THIS_QUARTER: timedelta(days=90),
            AnalyticsTimeframe.THIS_YEAR: timedelta(days=365),
            AnalyticsTimeframe.YEAR_TO_DATE: timedelta(days=365),
        }
        
        return duration_map.get(timeframe, timedelta(days=30))
    
    async def _calculate_business_health(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> BusinessHealthScore:
        """Calculate overall business health score."""
        
        # Simplified business health calculation
        # In a real implementation, this would be more sophisticated
        
        category_scores = {
            MetricCategory.REVENUE: 75.0,
            MetricCategory.CUSTOMERS: 80.0,
            MetricCategory.OPERATIONS: 85.0,
            MetricCategory.INVENTORY: 70.0,
            MetricCategory.MARKETING: 65.0
        }
        
        overall_score = sum(category_scores.values()) / len(category_scores)
        
        strengths = []
        concerns = []
        action_items = []
        
        for category, score in category_scores.items():
            if score >= 80:
                strengths.append(f"{category.value.title()} performance is strong")
            elif score < 70:
                concerns.append(f"{category.value.title()} needs attention")
                action_items.append(f"Improve {category.value} metrics")
        
        return BusinessHealthScore(
            overall_score=overall_score,
            category_scores=category_scores,
            strengths=strengths,
            concerns=concerns,
            action_items=action_items
        )
    
    async def _get_avg_fulfillment_time(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[float]:
        """Get average order fulfillment time in hours."""
        
        try:
            query = text("""
                SELECT AVG(EXTRACT(EPOCH FROM (updated_at - created_at))/3600) as avg_hours
                FROM commerce.orders 
                WHERE tenant_id = :tenant_id 
                AND status = 'completed'
                AND created_at >= :start_date 
                AND created_at <= :end_date
            """)
            
            result = await session.execute(query, {
                "tenant_id": analytics_filter.tenant_id,
                "start_date": analytics_filter.start_date or datetime.utcnow() - timedelta(days=30),
                "end_date": analytics_filter.end_date or datetime.utcnow()
            })
            
            avg_hours = result.scalar()
            return float(avg_hours) if avg_hours else None
            
        except SQLAlchemyError as e:
            logger.error("enhanced_analytics.fulfillment_time.error", extra={"error": str(e)})
            return None
    
    async def _get_order_accuracy_rate(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> Optional[float]:
        """Get order accuracy rate as a percentage."""
        
        # Simplified calculation - in reality this would track actual accuracy metrics
        return 95.5  # Placeholder
    
    async def _generate_alerts(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[str]:
        """Generate business alerts."""
        
        alerts = []
        
        # Check for low stock items
        try:
            query = text("""
                SELECT COUNT(*) as low_stock_count
                FROM inventory.stock 
                WHERE tenant_id = :tenant_id 
                AND quantity_available <= COALESCE(min_stock_level, 5)
            """)
            
            result = await session.execute(query, {"tenant_id": analytics_filter.tenant_id})
            low_stock_count = result.scalar() or 0
            
            if low_stock_count > 0:
                alerts.append(f"{low_stock_count} items are running low on stock")
                
        except SQLAlchemyError:
            pass
        
        return alerts
    
    async def _generate_recommendations(
        self,
        session: AsyncSession,
        analytics_filter: AnalyticsFilter
    ) -> List[str]:
        """Generate business recommendations."""
        
        recommendations = [
            "Consider running a promotion for slow-moving inventory",
            "Focus marketing efforts on high-value customer segments",
            "Optimize order fulfillment process to reduce delivery times"
        ]
        
        return recommendations