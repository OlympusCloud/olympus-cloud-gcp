from datetime import datetime, time
from enum import Enum
from typing import List, Optional
from uuid import UUID

from pydantic import BaseModel, Field


class TableStatus(str, Enum):
    AVAILABLE = "available"
    OCCUPIED = "occupied"
    RESERVED = "reserved"
    CLEANING = "cleaning"
    OUT_OF_ORDER = "out_of_order"


class ReservationStatus(str, Enum):
    CONFIRMED = "confirmed"
    SEATED = "seated"
    COMPLETED = "completed"
    CANCELLED = "cancelled"
    NO_SHOW = "no_show"


class OrderStatus(str, Enum):
    PENDING = "pending"
    PREPARING = "preparing"
    READY = "ready"
    SERVED = "served"
    CANCELLED = "cancelled"


class Table(BaseModel):
    id: UUID
    tenant_id: UUID
    location_id: UUID
    table_number: str
    capacity: int
    status: TableStatus
    section: Optional[str] = None
    server_id: Optional[UUID] = None
    current_order_id: Optional[UUID] = None
    last_cleaned: Optional[datetime] = None


class Reservation(BaseModel):
    id: UUID
    tenant_id: UUID
    location_id: UUID
    customer_name: str
    customer_phone: Optional[str] = None
    customer_email: Optional[str] = None
    party_size: int
    reservation_time: datetime
    status: ReservationStatus
    table_id: Optional[UUID] = None
    special_requests: Optional[str] = None
    created_at: datetime
    updated_at: datetime


class MenuItem(BaseModel):
    id: UUID
    tenant_id: UUID
    name: str
    description: Optional[str] = None
    category: str
    price: float
    cost: Optional[float] = None
    is_available: bool = True
    prep_time_minutes: Optional[int] = None
    allergens: List[str] = Field(default_factory=list)
    dietary_tags: List[str] = Field(default_factory=list)


class KitchenOrder(BaseModel):
    id: UUID
    tenant_id: UUID
    location_id: UUID
    table_id: UUID
    order_id: UUID
    items: List[dict]
    status: OrderStatus
    priority: int = 1
    estimated_ready_time: Optional[datetime] = None
    actual_ready_time: Optional[datetime] = None
    created_at: datetime


class RestaurantAnalytics(BaseModel):
    tenant_id: UUID
    location_id: Optional[UUID] = None
    date: datetime
    
    # Table metrics
    table_turnover_rate: float
    average_dining_duration: float
    table_utilization_rate: float
    
    # Service metrics
    average_wait_time: float
    average_prep_time: float
    order_accuracy_rate: float
    
    # Revenue metrics
    revenue_per_table: float
    revenue_per_seat: float
    average_check_size: float
    
    # Popular items
    top_menu_items: List[dict]
    peak_hours: List[dict]
    
    # Efficiency metrics
    kitchen_efficiency: float
    server_efficiency: float


class RestaurantRecommendation(BaseModel):
    type: str
    title: str
    description: str
    impact: str
    priority: int
    data: dict