from __future__ import annotations

from datetime import datetime, date
from enum import Enum
from typing import List, Optional
from uuid import UUID

from pydantic import BaseModel, Field


class RoomStatus(str, Enum):
    AVAILABLE = "available"
    OCCUPIED = "occupied"
    RESERVED = "reserved"
    OUT_OF_SERVICE = "out_of_service"
    CLEANING = "cleaning"


class BookingStatus(str, Enum):
    CONFIRMED = "confirmed"
    CHECKED_IN = "checked_in"
    CHECKED_OUT = "checked_out"
    CANCELLED = "cancelled"
    NO_SHOW = "no_show"


class GuestSentiment(str, Enum):
    EXCELLENT = "excellent"
    GOOD = "good"
    NEUTRAL = "neutral"
    POOR = "poor"


class RoomPerformance(BaseModel):
    room_id: UUID
    room_number: str
    room_type: str
    occupancy_rate: float = Field(ge=0.0, le=1.0)
    average_rate: float
    revenue: float


class ServiceRequestSummary(BaseModel):
    category: str
    total_requests: int
    average_response_minutes: float
    satisfaction_score: float = Field(ge=0.0, le=5.0)


class HousekeepingSummary(BaseModel):
    team: str
    tasks_completed: int
    average_completion_minutes: float
    overdue_tasks: int
    completion_rate: float = Field(ge=0.0, le=1.0)


class HospitalityAnalytics(BaseModel):
    tenant_id: UUID
    location_id: Optional[UUID]
    generated_at: datetime

    # Core KPIs
    occupancy_rate: float = Field(ge=0.0, le=1.0)
    average_daily_rate: float
    revenue_per_available_room: float
    available_rooms: int
    occupied_rooms: int
    bookings_created: int
    cancellations: int
    guest_satisfaction_score: float = Field(ge=0.0, le=5.0)

    # Operational metrics
    average_stay_length: float
    upcoming_check_ins: int
    upcoming_check_outs: int
    service_request_volume: int
    housekeeping_completion_rate: float = Field(ge=0.0, le=1.0)

    # Detailed breakdowns
    top_rooms: List[RoomPerformance] = Field(default_factory=list)
    service_requests: List[ServiceRequestSummary] = Field(default_factory=list)
    housekeeping: List[HousekeepingSummary] = Field(default_factory=list)


class HospitalityRecommendation(BaseModel):
    type: str
    title: str
    description: str
    impact: str
    priority: int
    data: dict = Field(default_factory=dict)


class BookingWindow(BaseModel):
    start_date: date
    end_date: date
    total_bookings: int
    average_rate: float
    conversion_rate: float = Field(ge=0.0, le=1.0)
