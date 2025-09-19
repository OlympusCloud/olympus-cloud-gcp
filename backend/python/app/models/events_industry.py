from __future__ import annotations

from datetime import datetime
from typing import List, Optional
from uuid import UUID

from pydantic import BaseModel, Field


class EventSegmentBreakdown(BaseModel):
    segment: str
    tickets_sold: int
    revenue: float
    conversion_rate: float = Field(ge=0.0, le=1.0)


class EventPerformance(BaseModel):
    event_id: UUID
    name: str
    venue: str
    start_time: datetime
    end_time: datetime
    tickets_available: int
    tickets_sold: int
    revenue: float
    attendance_rate: float = Field(ge=0.0, le=1.0)
    satisfaction_score: float = Field(ge=0.0, le=5.0)


class ChannelPerformance(BaseModel):
    channel: str
    tickets_sold: int
    revenue: float
    average_order_value: float
    growth_rate: float


class VendorPerformance(BaseModel):
    vendor_id: UUID
    name: str
    revenue: float
    orders: int
    satisfaction_score: float = Field(ge=0.0, le=5.0)


class EventsAnalytics(BaseModel):
    tenant_id: UUID
    location_id: Optional[UUID]
    generated_at: datetime

    total_events: int
    upcoming_events: int
    total_tickets_available: int
    total_tickets_sold: int
    gross_revenue: float
    average_ticket_price: float
    attendance_rate: float = Field(ge=0.0, le=1.0)
    average_satisfaction: float = Field(ge=0.0, le=5.0)

    segment_breakdown: List[EventSegmentBreakdown] = Field(default_factory=list)
    channel_performance: List[ChannelPerformance] = Field(default_factory=list)
    top_events: List[EventPerformance] = Field(default_factory=list)
    vendor_performance: List[VendorPerformance] = Field(default_factory=list)


class EventsRecommendation(BaseModel):
    type: str
    title: str
    description: str
    impact: str
    priority: int
    data: dict = Field(default_factory=dict)


class EventTimelinePoint(BaseModel):
    period_start: datetime
    period_end: datetime
    tickets_sold: int
    revenue: float
    new_registrations: int
