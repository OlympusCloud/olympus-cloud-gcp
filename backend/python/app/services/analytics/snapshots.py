from __future__ import annotations

from datetime import datetime, timedelta
from typing import List, Optional, Tuple
import statistics

from sqlalchemy import text, select, insert, and_
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker
from sqlalchemy.exc import SQLAlchemyError

from app.core.logging import logger
from app.models.analytics import AnalyticsTimeframe
from app.models.snapshots import (
    MetricsSnapshot,
    SnapshotHistoryRequest,
    SnapshotHistoryResponse,
    SnapshotTrend,
    SnapshotInsights
)
from app.services.analytics.service import AnalyticsService


class SnapshotService:
    """Service for managing historical metrics snapshots."""
    
    def __init__(
        self,
        session_factory: async_sessionmaker[AsyncSession],
        analytics_service: AnalyticsService
    ):
        self._session_factory = session_factory
        self._analytics_service = analytics_service
    
    async def create_snapshot(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe = AnalyticsTimeframe.TODAY,
        location_id: Optional[str] = None
    ) -> MetricsSnapshot:
        """Create a new metrics snapshot for the specified tenant and timeframe."""
        
        # Get current metrics from analytics service
        dashboard_response = await self._analytics_service.get_dashboard_metrics(
            tenant_id=tenant_id,
            timeframe=timeframe,
            location_id=location_id
        )
        
        # Create snapshot from current metrics
        snapshot = MetricsSnapshot(
            tenant_id=tenant_id,
            location_id=location_id,
            snapshot_date=datetime.utcnow(),
            timeframe=timeframe,
            active_users=dashboard_response.metrics.active_users,
            orders=dashboard_response.metrics.orders,
            revenue=dashboard_response.metrics.revenue,
            inventory_warnings=dashboard_response.metrics.inventory_warnings,
            avg_order_value=(
                dashboard_response.metrics.revenue / dashboard_response.metrics.orders
                if dashboard_response.metrics.orders > 0 else 0.0
            )
        )
        
        # Persist to database
        await self._persist_snapshot(snapshot)
        
        logger.info(
            "snapshot.created",
            extra={
                "tenant_id": tenant_id,
                "timeframe": timeframe.value,
                "location_id": location_id,
                "snapshot_id": str(snapshot.id)
            }
        )
        
        return snapshot
    
    async def get_snapshot_history(
        self,
        request: SnapshotHistoryRequest
    ) -> SnapshotHistoryResponse:
        """Retrieve historical snapshots with trend analysis."""
        
        async with self._session_factory() as session:
            snapshots = await self._fetch_snapshots(session, request)
            total_count = await self._count_snapshots(session, request)
            
            # Generate insights from snapshot data
            insights = await self._generate_insights(snapshots)
            
            return SnapshotHistoryResponse(
                tenant_id=request.tenant_id,
                location_id=request.location_id,
                timeframe=request.timeframe,
                snapshots=snapshots,
                total_count=total_count,
                insights=insights
            )
    
    async def backfill_snapshots(
        self,
        tenant_id: str,
        days_back: int = 30,
        location_id: Optional[str] = None
    ) -> int:
        """Backfill historical snapshots for the specified number of days."""
        
        snapshots_created = 0
        end_date = datetime.utcnow().date()
        
        for i in range(days_back):
            snapshot_date = end_date - timedelta(days=i)
            
            # Check if snapshot already exists for this date
            if await self._snapshot_exists(tenant_id, snapshot_date, location_id):
                continue
            
            # Create snapshot for this historical date
            try:
                snapshot = MetricsSnapshot(
                    tenant_id=tenant_id,
                    location_id=location_id,
                    snapshot_date=datetime.combine(snapshot_date, datetime.min.time()),
                    timeframe=AnalyticsTimeframe.TODAY,
                    # Note: In a real implementation, we'd calculate historical metrics
                    # For now, we'll use placeholder values
                    active_users=0,
                    orders=0,
                    revenue=0.0,
                    inventory_warnings=0
                )
                
                await self._persist_snapshot(snapshot)
                snapshots_created += 1
                
            except Exception as e:
                logger.warning(
                    "snapshot.backfill.error",
                    extra={
                        "tenant_id": tenant_id,
                        "date": str(snapshot_date),
                        "error": str(e)
                    }
                )
        
        logger.info(
            "snapshot.backfill.completed",
            extra={
                "tenant_id": tenant_id,
                "snapshots_created": snapshots_created,
                "days_back": days_back
            }
        )
        
        return snapshots_created
    
    async def _persist_snapshot(self, snapshot: MetricsSnapshot) -> None:
        """Persist a snapshot to the database."""
        
        async with self._session_factory() as session:
            try:
                # Create table if it doesn't exist
                await session.execute(text("""
                    CREATE TABLE IF NOT EXISTS analytics.metrics_snapshots (
                        id UUID PRIMARY KEY,
                        tenant_id VARCHAR(255) NOT NULL,
                        location_id VARCHAR(255),
                        snapshot_date TIMESTAMPTZ NOT NULL,
                        timeframe VARCHAR(50) NOT NULL,
                        active_users INTEGER DEFAULT 0,
                        orders INTEGER DEFAULT 0,
                        revenue DECIMAL(12,2) DEFAULT 0.0,
                        inventory_warnings INTEGER DEFAULT 0,
                        avg_order_value DECIMAL(10,2) DEFAULT 0.0,
                        new_customers INTEGER DEFAULT 0,
                        returning_customers INTEGER DEFAULT 0,
                        conversion_rate DECIMAL(5,4) DEFAULT 0.0,
                        created_at TIMESTAMPTZ DEFAULT NOW(),
                        UNIQUE(tenant_id, location_id, snapshot_date, timeframe)
                    )
                """))
                
                # Insert snapshot
                await session.execute(text("""
                    INSERT INTO analytics.metrics_snapshots (
                        id, tenant_id, location_id, snapshot_date, timeframe,
                        active_users, orders, revenue, inventory_warnings,
                        avg_order_value, new_customers, returning_customers,
                        conversion_rate, created_at
                    ) VALUES (
                        :id, :tenant_id, :location_id, :snapshot_date, :timeframe,
                        :active_users, :orders, :revenue, :inventory_warnings,
                        :avg_order_value, :new_customers, :returning_customers,
                        :conversion_rate, :created_at
                    ) ON CONFLICT (tenant_id, location_id, snapshot_date, timeframe)
                    DO UPDATE SET
                        active_users = EXCLUDED.active_users,
                        orders = EXCLUDED.orders,
                        revenue = EXCLUDED.revenue,
                        inventory_warnings = EXCLUDED.inventory_warnings,
                        avg_order_value = EXCLUDED.avg_order_value,
                        new_customers = EXCLUDED.new_customers,
                        returning_customers = EXCLUDED.returning_customers,
                        conversion_rate = EXCLUDED.conversion_rate
                """), {
                    "id": snapshot.id,
                    "tenant_id": snapshot.tenant_id,
                    "location_id": snapshot.location_id,
                    "snapshot_date": snapshot.snapshot_date,
                    "timeframe": snapshot.timeframe.value,
                    "active_users": snapshot.active_users,
                    "orders": snapshot.orders,
                    "revenue": snapshot.revenue,
                    "inventory_warnings": snapshot.inventory_warnings,
                    "avg_order_value": snapshot.avg_order_value,
                    "new_customers": snapshot.new_customers,
                    "returning_customers": snapshot.returning_customers,
                    "conversion_rate": snapshot.conversion_rate,
                    "created_at": snapshot.created_at
                })
                
                await session.commit()
                
            except SQLAlchemyError as e:
                await session.rollback()
                logger.error(
                    "snapshot.persist.error",
                    extra={"error": str(e), "snapshot_id": str(snapshot.id)}
                )
                raise
    
    async def _fetch_snapshots(
        self,
        session: AsyncSession,
        request: SnapshotHistoryRequest
    ) -> List[MetricsSnapshot]:
        """Fetch snapshots from database based on request parameters."""
        
        try:
            conditions = ["tenant_id = :tenant_id"]
            params = {"tenant_id": request.tenant_id}
            
            if request.location_id:
                conditions.append("location_id = :location_id")
                params["location_id"] = request.location_id
            
            if request.start_date and request.end_date:
                conditions.append("snapshot_date BETWEEN :start_date AND :end_date")
                params["start_date"] = request.start_date
                params["end_date"] = request.end_date
            
            query = text(f"""
                SELECT * FROM analytics.metrics_snapshots
                WHERE {' AND '.join(conditions)}
                ORDER BY snapshot_date DESC
                LIMIT :limit
            """)
            params["limit"] = request.limit
            
            result = await session.execute(query, params)
            rows = result.fetchall()
            
            snapshots = []
            for row in rows:
                snapshot = MetricsSnapshot(
                    id=row.id,
                    tenant_id=row.tenant_id,
                    location_id=row.location_id,
                    snapshot_date=row.snapshot_date,
                    timeframe=AnalyticsTimeframe(row.timeframe),
                    active_users=row.active_users,
                    orders=row.orders,
                    revenue=float(row.revenue),
                    inventory_warnings=row.inventory_warnings,
                    avg_order_value=float(row.avg_order_value),
                    new_customers=row.new_customers,
                    returning_customers=row.returning_customers,
                    conversion_rate=float(row.conversion_rate),
                    created_at=row.created_at
                )
                snapshots.append(snapshot)
            
            return snapshots
            
        except SQLAlchemyError as e:
            logger.error("snapshot.fetch.error", extra={"error": str(e)})
            return []
    
    async def _count_snapshots(
        self,
        session: AsyncSession,
        request: SnapshotHistoryRequest
    ) -> int:
        """Count total snapshots matching the request criteria."""
        
        try:
            conditions = ["tenant_id = :tenant_id"]
            params = {"tenant_id": request.tenant_id}
            
            if request.location_id:
                conditions.append("location_id = :location_id")
                params["location_id"] = request.location_id
            
            if request.start_date and request.end_date:
                conditions.append("snapshot_date BETWEEN :start_date AND :end_date")
                params["start_date"] = request.start_date
                params["end_date"] = request.end_date
            
            query = text(f"""
                SELECT COUNT(*) as total FROM analytics.metrics_snapshots
                WHERE {' AND '.join(conditions)}
            """)
            
            result = await session.execute(query, params)
            row = result.one_or_none()
            return int(row.total) if row else 0
            
        except SQLAlchemyError as e:
            logger.error("snapshot.count.error", extra={"error": str(e)})
            return 0
    
    async def _snapshot_exists(
        self,
        tenant_id: str,
        snapshot_date: datetime,
        location_id: Optional[str] = None
    ) -> bool:
        """Check if a snapshot already exists for the given parameters."""
        
        async with self._session_factory() as session:
            try:
                conditions = [
                    "tenant_id = :tenant_id",
                    "DATE(snapshot_date) = :snapshot_date"
                ]
                params = {
                    "tenant_id": tenant_id,
                    "snapshot_date": snapshot_date
                }
                
                if location_id:
                    conditions.append("location_id = :location_id")
                    params["location_id"] = location_id
                
                query = text(f"""
                    SELECT COUNT(*) as count FROM analytics.metrics_snapshots
                    WHERE {' AND '.join(conditions)}
                """)
                
                result = await session.execute(query, params)
                row = result.one_or_none()
                return int(row.count) > 0 if row else False
                
            except SQLAlchemyError:
                return False
    
    async def _generate_insights(self, snapshots: List[MetricsSnapshot]) -> List[str]:
        """Generate AI-powered insights from snapshot data."""
        
        if len(snapshots) < 2:
            return ["Insufficient historical data for trend analysis"]
        
        insights = []
        
        # Revenue trend analysis
        revenue_values = [s.revenue for s in snapshots]
        revenue_trend = self._calculate_trend(revenue_values)
        
        if revenue_trend.direction == "increasing":
            insights.append(
                f"Revenue is trending upward with {revenue_trend.change_percentage:.1f}% growth"
            )
        elif revenue_trend.direction == "decreasing":
            insights.append(
                f"Revenue is declining by {abs(revenue_trend.change_percentage):.1f}%"
            )
        
        # Order volume analysis
        order_values = [s.orders for s in snapshots]
        order_trend = self._calculate_trend(order_values)
        
        if order_trend.direction == "increasing":
            insights.append(
                f"Order volume is growing by {order_trend.change_percentage:.1f}%"
            )
        
        # Average order value analysis
        aov_values = [s.avg_order_value for s in snapshots if s.avg_order_value > 0]
        if len(aov_values) >= 2:
            aov_trend = self._calculate_trend(aov_values)
            if abs(aov_trend.change_percentage) > 5:
                direction = "increased" if aov_trend.direction == "increasing" else "decreased"
                insights.append(
                    f"Average order value has {direction} by {abs(aov_trend.change_percentage):.1f}%"
                )
        
        # Inventory warnings
        warning_values = [s.inventory_warnings for s in snapshots]
        if any(w > 0 for w in warning_values):
            avg_warnings = statistics.mean(warning_values)
            insights.append(
                f"Average inventory warnings: {avg_warnings:.1f} items need attention"
            )
        
        return insights
    
    def _calculate_trend(self, values: List[float]) -> SnapshotTrend:
        """Calculate trend direction and percentage change."""
        
        if len(values) < 2:
            return SnapshotTrend(
                metric_name="unknown",
                direction="stable",
                change_percentage=0.0,
                current_value=0.0,
                previous_value=0.0,
                confidence=0.0
            )
        
        current = values[0]  # Most recent (first in DESC order)
        previous = values[-1]  # Oldest
        
        if previous == 0:
            change_percentage = 100.0 if current > 0 else 0.0
        else:
            change_percentage = ((current - previous) / previous) * 100
        
        direction = "stable"
        if abs(change_percentage) > 1:  # 1% threshold
            direction = "increasing" if change_percentage > 0 else "decreasing"
        
        # Simple confidence based on data consistency
        confidence = min(1.0, len(values) / 10.0)
        
        return SnapshotTrend(
            metric_name="trend",
            direction=direction,
            change_percentage=change_percentage,
            current_value=current,
            previous_value=previous,
            confidence=confidence
        )