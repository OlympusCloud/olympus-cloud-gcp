from datetime import datetime, timedelta
from typing import Dict, List, Optional
from uuid import UUID

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.models.restaurant import (
    RestaurantAnalytics,
    RestaurantRecommendation,
    TableStatus,
    ReservationStatus,
    OrderStatus
)


class RestaurantService:
    def __init__(self, session_factory: async_sessionmaker[AsyncSession]):
        self.session_factory = session_factory

    async def get_table_analytics(
        self, 
        tenant_id: str, 
        location_id: Optional[str] = None,
        date_from: Optional[datetime] = None,
        date_to: Optional[datetime] = None
    ) -> RestaurantAnalytics:
        """Get comprehensive table and service analytics."""
        
        if not date_from:
            date_from = datetime.now() - timedelta(days=30)
        if not date_to:
            date_to = datetime.now()
            
        async with self.session_factory() as session:
            # Table turnover calculation
            turnover_query = text("""
                SELECT 
                    COUNT(DISTINCT o.id) as total_orders,
                    COUNT(DISTINCT DATE(o.created_at)) as days,
                    COUNT(DISTINCT l.id) as total_tables
                FROM commerce.orders o
                JOIN platform.locations l ON o.location_id = l.id
                WHERE o.tenant_id = :tenant_id
                AND (:location_id IS NULL OR o.location_id = :location_id)
                AND o.created_at BETWEEN :date_from AND :date_to
            """)
            
            result = await session.execute(turnover_query, {
                "tenant_id": tenant_id,
                "location_id": location_id,
                "date_from": date_from,
                "date_to": date_to
            })
            turnover_data = result.fetchone()
            
            # Service metrics
            service_query = text("""
                SELECT 
                    AVG(EXTRACT(EPOCH FROM (o.updated_at - o.created_at))/60) as avg_service_time,
                    AVG(o.total_amount) as avg_check_size,
                    COUNT(*) as total_orders
                FROM commerce.orders o
                WHERE o.tenant_id = :tenant_id
                AND (:location_id IS NULL OR o.location_id = :location_id)
                AND o.created_at BETWEEN :date_from AND :date_to
                AND o.status = 'completed'
            """)
            
            result = await session.execute(service_query, {
                "tenant_id": tenant_id,
                "location_id": location_id,
                "date_from": date_from,
                "date_to": date_to
            })
            service_data = result.fetchone()
            
            # Popular items
            popular_items_query = text("""
                SELECT 
                    p.name,
                    SUM(oi.quantity) as total_sold,
                    AVG(oi.unit_price) as avg_price
                FROM commerce.order_items oi
                JOIN commerce.products p ON oi.product_id = p.id
                JOIN commerce.orders o ON oi.order_id = o.id
                WHERE o.tenant_id = :tenant_id
                AND (:location_id IS NULL OR o.location_id = :location_id)
                AND o.created_at BETWEEN :date_from AND :date_to
                GROUP BY p.id, p.name
                ORDER BY total_sold DESC
                LIMIT 10
            """)
            
            result = await session.execute(popular_items_query, {
                "tenant_id": tenant_id,
                "location_id": location_id,
                "date_from": date_from,
                "date_to": date_to
            })
            popular_items = [
                {"name": row[0], "quantity": row[1], "avg_price": float(row[2] or 0)}
                for row in result.fetchall()
            ]
            
            # Peak hours analysis
            peak_hours_query = text("""
                SELECT 
                    EXTRACT(HOUR FROM o.created_at) as hour,
                    COUNT(*) as order_count,
                    AVG(o.total_amount) as avg_amount
                FROM commerce.orders o
                WHERE o.tenant_id = :tenant_id
                AND (:location_id IS NULL OR o.location_id = :location_id)
                AND o.created_at BETWEEN :date_from AND :date_to
                GROUP BY EXTRACT(HOUR FROM o.created_at)
                ORDER BY order_count DESC
                LIMIT 5
            """)
            
            result = await session.execute(peak_hours_query, {
                "tenant_id": tenant_id,
                "location_id": location_id,
                "date_from": date_from,
                "date_to": date_to
            })
            peak_hours = [
                {"hour": int(row[0]), "orders": row[1], "avg_amount": float(row[2] or 0)}
                for row in result.fetchall()
            ]
            
            # Calculate metrics with defaults
            total_orders = turnover_data[0] if turnover_data and turnover_data[0] else 0
            days = turnover_data[1] if turnover_data and turnover_data[1] else 1
            total_tables = turnover_data[2] if turnover_data and turnover_data[2] else 1
            
            table_turnover_rate = total_orders / (days * total_tables) if days > 0 and total_tables > 0 else 0
            avg_service_time = service_data[0] if service_data and service_data[0] else 45.0
            avg_check_size = service_data[1] if service_data and service_data[1] else 0.0
            
            return RestaurantAnalytics(
                tenant_id=UUID(tenant_id),
                location_id=UUID(location_id) if location_id else None,
                date=datetime.now(),
                table_turnover_rate=table_turnover_rate,
                average_dining_duration=avg_service_time,
                table_utilization_rate=min(table_turnover_rate * 0.8, 1.0),
                average_wait_time=max(15.0, avg_service_time * 0.3),
                average_prep_time=max(10.0, avg_service_time * 0.6),
                order_accuracy_rate=0.95,  # Default high accuracy
                revenue_per_table=avg_check_size * table_turnover_rate,
                revenue_per_seat=avg_check_size * table_turnover_rate * 0.25,  # Assume 4 seats per table
                average_check_size=avg_check_size,
                top_menu_items=popular_items,
                peak_hours=peak_hours,
                kitchen_efficiency=0.85,  # Default efficiency
                server_efficiency=0.80   # Default efficiency
            )

    async def get_table_status(self, tenant_id: str, location_id: str) -> Dict[str, int]:
        """Get current table status distribution."""
        
        # Mock implementation - in real system would query actual table management
        return {
            "available": 12,
            "occupied": 8,
            "reserved": 3,
            "cleaning": 2,
            "out_of_order": 1
        }

    async def get_reservation_metrics(
        self, 
        tenant_id: str, 
        location_id: Optional[str] = None,
        date: Optional[datetime] = None
    ) -> Dict:
        """Get reservation system metrics."""
        
        if not date:
            date = datetime.now()
            
        # Mock implementation - would integrate with actual reservation system
        return {
            "total_reservations": 45,
            "confirmed": 38,
            "seated": 32,
            "no_shows": 3,
            "cancellations": 2,
            "walk_ins": 15,
            "average_party_size": 2.8,
            "peak_reservation_time": "19:00",
            "utilization_rate": 0.82
        }

    async def generate_restaurant_recommendations(
        self, 
        tenant_id: str, 
        location_id: Optional[str] = None
    ) -> List[RestaurantRecommendation]:
        """Generate AI-powered restaurant operation recommendations."""
        
        analytics = await self.get_table_analytics(tenant_id, location_id)
        recommendations = []
        
        # Table turnover optimization
        if analytics.table_turnover_rate < 2.0:
            recommendations.append(RestaurantRecommendation(
                type="operations",
                title="Improve Table Turnover",
                description=f"Current turnover rate is {analytics.table_turnover_rate:.1f}. Consider optimizing service speed or table management.",
                impact="high",
                priority=1,
                data={"current_rate": analytics.table_turnover_rate, "target_rate": 2.5}
            ))
        
        # Service time optimization
        if analytics.average_dining_duration > 60:
            recommendations.append(RestaurantRecommendation(
                type="service",
                title="Reduce Service Time",
                description=f"Average dining duration is {analytics.average_dining_duration:.0f} minutes. Streamline order taking and kitchen processes.",
                impact="medium",
                priority=2,
                data={"current_time": analytics.average_dining_duration, "target_time": 45}
            ))
        
        # Menu optimization
        if len(analytics.top_menu_items) > 0:
            top_item = analytics.top_menu_items[0]
            recommendations.append(RestaurantRecommendation(
                type="menu",
                title="Promote Popular Items",
                description=f"'{top_item['name']}' is your top seller. Consider featuring it prominently or creating variations.",
                impact="medium",
                priority=3,
                data={"item": top_item}
            ))
        
        # Peak hours staffing
        if len(analytics.peak_hours) > 0:
            peak_hour = analytics.peak_hours[0]
            recommendations.append(RestaurantRecommendation(
                type="staffing",
                title="Optimize Peak Hour Staffing",
                description=f"Hour {peak_hour['hour']}:00 is your busiest with {peak_hour['orders']} orders. Ensure adequate staffing.",
                impact="high",
                priority=1,
                data={"peak_hour": peak_hour}
            ))
        
        return recommendations

    async def get_kitchen_display_orders(
        self, 
        tenant_id: str, 
        location_id: str
    ) -> List[Dict]:
        """Get orders for kitchen display system."""
        
        async with self.session_factory() as session:
            query = text("""
                SELECT 
                    o.id,
                    o.created_at,
                    o.status,
                    o.total_amount,
                    COALESCE(
                        JSON_AGG(
                            JSON_BUILD_OBJECT(
                                'product_name', p.name,
                                'quantity', oi.quantity,
                                'special_instructions', oi.notes
                            )
                        ) FILTER (WHERE p.id IS NOT NULL), 
                        '[]'::json
                    ) as items
                FROM commerce.orders o
                LEFT JOIN commerce.order_items oi ON o.id = oi.order_id
                LEFT JOIN commerce.products p ON oi.product_id = p.id
                WHERE o.tenant_id = :tenant_id
                AND o.location_id = :location_id
                AND o.status IN ('pending', 'preparing')
                AND o.created_at >= NOW() - INTERVAL '4 hours'
                GROUP BY o.id, o.created_at, o.status, o.total_amount
                ORDER BY o.created_at ASC
            """)
            
            result = await session.execute(query, {
                "tenant_id": tenant_id,
                "location_id": location_id
            })
            
            orders = []
            for row in result.fetchall():
                orders.append({
                    "id": str(row[0]),
                    "created_at": row[1].isoformat(),
                    "status": row[2],
                    "total_amount": float(row[3]),
                    "items": row[4] if row[4] else [],
                    "wait_time_minutes": int((datetime.now() - row[1]).total_seconds() / 60)
                })
            
            return orders