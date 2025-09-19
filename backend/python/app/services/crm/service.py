from __future__ import annotations

from datetime import datetime
from typing import List

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker
from sqlalchemy.exc import SQLAlchemyError

from app.core.logging import logger
from app.models.crm import Customer, Campaign, CustomerSegment, SegmentationResult


class CRMService:
    """Customer relationship management service."""
    
    def __init__(self, session_factory: async_sessionmaker[AsyncSession]):
        self._session_factory = session_factory
    
    async def segment_customers(self, tenant_id: str) -> SegmentationResult:
        """Segment customers based on behavior and value."""
        
        async with self._session_factory() as session:
            try:
                query = text("""
                    WITH customer_stats AS (
                        SELECT 
                            customer_id,
                            COUNT(*) as order_count,
                            SUM(total_amount) as total_spent,
                            EXTRACT(DAYS FROM NOW() - MAX(created_at)) as days_since_last_order
                        FROM commerce.orders 
                        WHERE tenant_id = :tenant_id 
                        AND status = 'completed'
                        GROUP BY customer_id
                    ),
                    segments AS (
                        SELECT 
                            CASE 
                                WHEN total_spent >= 1000 AND days_since_last_order <= 30 THEN 'vip'
                                WHEN total_spent >= 500 THEN 'high_value'
                                WHEN days_since_last_order > 90 THEN 'at_risk'
                                WHEN order_count = 1 THEN 'new'
                                ELSE 'regular'
                            END as segment
                        FROM customer_stats
                    )
                    SELECT segment, COUNT(*) as count
                    FROM segments
                    GROUP BY segment
                """)
                
                result = await session.execute(query, {"tenant_id": tenant_id})
                rows = result.fetchall()
                
                segments = {}
                total = 0
                for row in rows:
                    segment = CustomerSegment(row.segment)
                    count = int(row.count)
                    segments[segment] = count
                    total += count
                
                return SegmentationResult(
                    tenant_id=tenant_id,
                    segments=segments,
                    total_customers=total
                )
                
            except Exception as e:
                logger.error("crm.segmentation.error", extra={"error": str(e)})
                return SegmentationResult(
                    tenant_id=tenant_id,
                    segments={},
                    total_customers=0
                )
    
    async def create_campaign(self, campaign: Campaign) -> Campaign:
        """Create a new marketing campaign."""
        
        async with self._session_factory() as session:
            try:
                await session.execute(text("""
                    CREATE TABLE IF NOT EXISTS crm.campaigns (
                        id UUID PRIMARY KEY,
                        tenant_id VARCHAR(255) NOT NULL,
                        name VARCHAR(255) NOT NULL,
                        type VARCHAR(50) NOT NULL,
                        status VARCHAR(50) NOT NULL,
                        target_segments TEXT[] NOT NULL,
                        message TEXT NOT NULL,
                        sent_count INTEGER DEFAULT 0,
                        created_at TIMESTAMPTZ DEFAULT NOW()
                    )
                """))
                
                await session.execute(text("""
                    INSERT INTO crm.campaigns (
                        id, tenant_id, name, type, status, target_segments, message, created_at
                    ) VALUES (
                        :id, :tenant_id, :name, :type, :status, :target_segments, :message, :created_at
                    )
                """), {
                    "id": campaign.id,
                    "tenant_id": campaign.tenant_id,
                    "name": campaign.name,
                    "type": campaign.type.value,
                    "status": campaign.status.value,
                    "target_segments": [s.value for s in campaign.target_segments],
                    "message": campaign.message,
                    "created_at": campaign.created_at
                })
                
                await session.commit()
                return campaign
                
            except SQLAlchemyError as e:
                await session.rollback()
                logger.error("crm.campaign.create.error", extra={"error": str(e)})
                raise