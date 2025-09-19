from __future__ import annotations

from datetime import datetime, timedelta
from typing import List
import statistics

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker
from sqlalchemy.exc import SQLAlchemyError

from app.core.logging import logger
from app.models.inventory import (
    InventoryItem, StockMovement, InventoryForecast, StockAlert, 
    InventoryReport, StockStatus, ForecastPeriod
)


class InventoryService:
    """Inventory tracking and forecasting service."""
    
    def __init__(self, session_factory: async_sessionmaker[AsyncSession]):
        self._session_factory = session_factory
    
    async def get_inventory_report(self, tenant_id: str, location_id: str = None) -> InventoryReport:
        """Generate comprehensive inventory report."""
        
        async with self._session_factory() as session:
            try:
                # Get inventory summary
                conditions = ["tenant_id = :tenant_id"]
                params = {"tenant_id": tenant_id}
                
                if location_id:
                    conditions.append("location_id = :location_id")
                    params["location_id"] = location_id
                
                query = text(f"""
                    SELECT 
                        COUNT(*) as total_items,
                        COUNT(CASE WHEN quantity_available <= min_stock_level THEN 1 END) as low_stock,
                        COUNT(CASE WHEN quantity_available = 0 THEN 1 END) as out_of_stock,
                        COUNT(CASE WHEN quantity_available <= reorder_point THEN 1 END) as reorder_needed,
                        COALESCE(SUM(quantity_available * unit_cost), 0) as total_value
                    FROM inventory.stock 
                    WHERE {' AND '.join(conditions)}
                """)
                
                result = await session.execute(query, params)
                row = result.one_or_none()
                
                if not row:
                    return InventoryReport(
                        tenant_id=tenant_id,
                        location_id=location_id,
                        total_items=0,
                        low_stock_items=0,
                        out_of_stock_items=0,
                        reorder_needed=0,
                        total_value=0.0
                    )
                
                # Get alerts
                alerts = await self._get_stock_alerts(session, tenant_id, location_id)
                
                # Get forecasts
                forecasts = await self._generate_forecasts(session, tenant_id, location_id)
                
                return InventoryReport(
                    tenant_id=tenant_id,
                    location_id=location_id,
                    total_items=int(row.total_items),
                    low_stock_items=int(row.low_stock),
                    out_of_stock_items=int(row.out_of_stock),
                    reorder_needed=int(row.reorder_needed),
                    total_value=float(row.total_value),
                    alerts=alerts,
                    forecasts=forecasts
                )
                
            except Exception as e:
                logger.error("inventory.report.error", extra={"error": str(e)})
                return InventoryReport(
                    tenant_id=tenant_id,
                    location_id=location_id,
                    total_items=0,
                    low_stock_items=0,
                    out_of_stock_items=0,
                    reorder_needed=0,
                    total_value=0.0
                )
    
    async def track_stock_movement(self, movement: StockMovement) -> StockMovement:
        """Track inventory stock movement."""
        
        async with self._session_factory() as session:
            try:
                # Create movements table if not exists
                await session.execute(text("""
                    CREATE TABLE IF NOT EXISTS inventory.movements (
                        id UUID PRIMARY KEY,
                        tenant_id VARCHAR(255) NOT NULL,
                        product_id VARCHAR(255) NOT NULL,
                        location_id VARCHAR(255),
                        movement_type VARCHAR(50) NOT NULL,
                        quantity INTEGER NOT NULL,
                        reference_id VARCHAR(255),
                        created_at TIMESTAMPTZ DEFAULT NOW()
                    )
                """))
                
                # Insert movement
                await session.execute(text("""
                    INSERT INTO inventory.movements (
                        id, tenant_id, product_id, location_id, movement_type, quantity, reference_id, created_at
                    ) VALUES (
                        :id, :tenant_id, :product_id, :location_id, :movement_type, :quantity, :reference_id, :created_at
                    )
                """), {
                    "id": movement.id,
                    "tenant_id": movement.tenant_id,
                    "product_id": movement.product_id,
                    "location_id": movement.location_id,
                    "movement_type": movement.movement_type,
                    "quantity": movement.quantity,
                    "reference_id": movement.reference_id,
                    "created_at": movement.created_at
                })
                
                await session.commit()
                return movement
                
            except SQLAlchemyError as e:
                await session.rollback()
                logger.error("inventory.movement.error", extra={"error": str(e)})
                raise
    
    async def _get_stock_alerts(
        self, 
        session: AsyncSession, 
        tenant_id: str, 
        location_id: str = None
    ) -> List[StockAlert]:
        """Get current stock alerts."""
        
        alerts = []
        
        try:
            conditions = ["tenant_id = :tenant_id"]
            params = {"tenant_id": tenant_id}
            
            if location_id:
                conditions.append("location_id = :location_id")
                params["location_id"] = location_id
            
            # Low stock alerts
            query = text(f"""
                SELECT product_id, location_id, quantity_available, min_stock_level, reorder_point
                FROM inventory.stock 
                WHERE {' AND '.join(conditions)}
                AND (quantity_available <= min_stock_level OR quantity_available <= reorder_point)
            """)
            
            result = await session.execute(query, params)
            rows = result.fetchall()
            
            for row in rows:
                if row.quantity_available == 0:
                    alert_type = StockStatus.OUT_OF_STOCK
                    message = f"Product {row.product_id} is out of stock"
                    threshold = 0
                elif row.quantity_available <= row.min_stock_level:
                    alert_type = StockStatus.LOW_STOCK
                    message = f"Product {row.product_id} is low on stock ({row.quantity_available} remaining)"
                    threshold = row.min_stock_level
                else:
                    alert_type = StockStatus.REORDER_NEEDED
                    message = f"Product {row.product_id} needs reordering ({row.quantity_available} remaining)"
                    threshold = row.reorder_point
                
                alerts.append(StockAlert(
                    tenant_id=tenant_id,
                    product_id=row.product_id,
                    location_id=row.location_id,
                    alert_type=alert_type,
                    current_stock=row.quantity_available,
                    threshold=threshold,
                    message=message
                ))
                
        except SQLAlchemyError as e:
            logger.error("inventory.alerts.error", extra={"error": str(e)})
        
        return alerts
    
    async def _generate_forecasts(
        self, 
        session: AsyncSession, 
        tenant_id: str, 
        location_id: str = None
    ) -> List[InventoryForecast]:
        """Generate demand forecasts for products."""
        
        forecasts = []
        
        try:
            conditions = ["o.tenant_id = :tenant_id"]
            params = {"tenant_id": tenant_id}
            
            if location_id:
                conditions.append("o.location_id = :location_id")
                params["location_id"] = location_id
            
            # Get historical sales data for forecasting
            query = text(f"""
                SELECT 
                    oi.product_id,
                    o.location_id,
                    DATE_TRUNC('week', o.created_at) as week,
                    SUM(oi.quantity) as weekly_demand
                FROM commerce.order_items oi
                JOIN commerce.orders o ON oi.order_id = o.id
                WHERE {' AND '.join(conditions)}
                AND o.status = 'completed'
                AND o.created_at >= NOW() - INTERVAL '12 weeks'
                GROUP BY oi.product_id, o.location_id, DATE_TRUNC('week', o.created_at)
                ORDER BY oi.product_id, week
            """)
            
            result = await session.execute(query, params)
            rows = result.fetchall()
            
            # Group by product for forecasting
            product_data = {}
            for row in rows:
                key = (row.product_id, row.location_id)
                if key not in product_data:
                    product_data[key] = []
                product_data[key].append(row.weekly_demand)
            
            # Generate forecasts for each product
            for (product_id, loc_id), demands in product_data.items():
                if len(demands) >= 3:  # Need at least 3 data points
                    # Simple moving average forecast
                    recent_avg = statistics.mean(demands[-4:]) if len(demands) >= 4 else statistics.mean(demands)
                    predicted_demand = max(1, int(recent_avg))
                    
                    # Recommend stock level (2x predicted demand + safety stock)
                    recommended_stock = predicted_demand * 2 + 10
                    
                    # Calculate confidence based on demand consistency
                    if len(demands) > 1:
                        variance = statistics.variance(demands)
                        mean_demand = statistics.mean(demands)
                        cv = variance / mean_demand if mean_demand > 0 else 1
                        confidence = max(0.3, min(0.9, 1 - cv))
                    else:
                        confidence = 0.5
                    
                    forecasts.append(InventoryForecast(
                        tenant_id=tenant_id,
                        product_id=product_id,
                        location_id=loc_id,
                        period=ForecastPeriod.WEEKLY,
                        predicted_demand=predicted_demand,
                        recommended_stock=recommended_stock,
                        confidence=confidence
                    ))
                    
        except SQLAlchemyError as e:
            logger.error("inventory.forecasts.error", extra={"error": str(e)})
        
        return forecasts