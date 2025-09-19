from __future__ import annotations

from datetime import datetime
from typing import List, Optional
from enum import Enum
from uuid import UUID, uuid4

from pydantic import BaseModel, Field


class CustomerSegment(str, Enum):
    VIP = "vip"
    HIGH_VALUE = "high_value"
    REGULAR = "regular"
    NEW = "new"
    AT_RISK = "at_risk"


class CampaignType(str, Enum):
    EMAIL = "email"
    SMS = "sms"
    PUSH = "push"
    IN_APP = "in_app"


class CampaignStatus(str, Enum):
    DRAFT = "draft"
    ACTIVE = "active"
    PAUSED = "paused"
    COMPLETED = "completed"


class Customer(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    tenant_id: str
    email: str
    phone: Optional[str] = None
    first_name: Optional[str] = None
    last_name: Optional[str] = None
    segment: CustomerSegment = CustomerSegment.NEW
    total_spent: float = 0.0
    order_count: int = 0
    last_order_date: Optional[datetime] = None
    created_at: datetime = Field(default_factory=datetime.utcnow)


class Campaign(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    tenant_id: str
    name: str
    type: CampaignType
    status: CampaignStatus = CampaignStatus.DRAFT
    target_segments: List[CustomerSegment]
    message: str
    sent_count: int = 0
    open_rate: float = 0.0
    click_rate: float = 0.0
    created_at: datetime = Field(default_factory=datetime.utcnow)


class SegmentationResult(BaseModel):
    tenant_id: str
    segments: dict[CustomerSegment, int]
    total_customers: int
    updated_at: datetime = Field(default_factory=datetime.utcnow)