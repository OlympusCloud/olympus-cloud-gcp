from __future__ import annotations

from datetime import datetime
from typing import Any, Literal, Optional

from pydantic import BaseModel, Field


class EventContext(BaseModel):
    request_id: Optional[str] = Field(default=None, description="Correlation identifier")
    source: Optional[str] = Field(default=None, description="Originating service")


class AnalyticsEvent(BaseModel):
    """Generic event envelope consumed by the analytics service."""

    name: Literal[
        "events.user.logged_in",
        "events.user.created",
        "events.order.created",
        "events.order.updated",
        "events.payment.processed",
        "events.inventory.updated",
    ]
    payload: dict[str, Any]
    occurred_at: datetime = Field(default_factory=datetime.utcnow)
    context: EventContext = Field(default_factory=EventContext)


class UserLoggedInPayload(BaseModel):
    user_id: str
    session_id: str
    login_method: str
    happened_at: datetime


class OrderCreatedPayload(BaseModel):
    order_id: str
    user_id: str
    total: float
    currency: str = "USD"
    happened_at: datetime
