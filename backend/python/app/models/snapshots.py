from __future__ import annotations

from datetime import datetime
from typing import Any, Dict, List, Optional
from uuid import UUID, uuid4

from pydantic import BaseModel, Field

from app.models.analytics import AnalyticsTimeframe


class MetricsSnapshot(BaseModel):
    """A point-in-time snapshot of analytics metrics for a tenant."""
    
    id: UUID = Field(default_factory=uuid4)
    tenant_id: str
    location_id: Optional[str] = None
    snapshot_date: datetime
    timeframe: AnalyticsTimeframe
    
    # Core metrics
    active_users: int = 0
    orders: int = 0
    revenue: float = 0.0
    inventory_warnings: int = 0
    
    # Extended metrics
    avg_order_value: float = 0.0
    new_customers: int = 0
    returning_customers: int = 0
    conversion_rate: float = 0.0
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.utcnow)
    
    class Config:
        from_attributes = True


class SnapshotHistoryRequest(BaseModel):
    """Request parameters for historical snapshot data."""
    
    tenant_id: str
    location_id: Optional[str] = None
    timeframe: AnalyticsTimeframe = AnalyticsTimeframe.LAST_MONTH
    start_date: Optional[datetime] = None
    end_date: Optional[datetime] = None
    limit: int = Field(default=100, ge=1, le=1000)


class SnapshotHistoryResponse(BaseModel):
    """Response containing historical snapshot data."""
    
    tenant_id: str
    location_id: Optional[str] = None
    timeframe: AnalyticsTimeframe
    snapshots: List[MetricsSnapshot]
    total_count: int
    insights: List[str] = Field(default_factory=list)
    
    class Config:
        from_attributes = True


class SnapshotTrend(BaseModel):
    """Trend analysis for a specific metric over time."""
    
    metric_name: str
    direction: str  # 'increasing', 'decreasing', 'stable'
    change_percentage: float
    current_value: float
    previous_value: float
    confidence: float = Field(ge=0.0, le=1.0)


class SnapshotInsights(BaseModel):
    """AI-generated insights from snapshot analysis."""
    
    trends: List[SnapshotTrend]
    anomalies: List[Dict[str, Any]] = Field(default_factory=list)
    recommendations: List[str] = Field(default_factory=list)
    confidence_score: float = Field(ge=0.0, le=1.0)