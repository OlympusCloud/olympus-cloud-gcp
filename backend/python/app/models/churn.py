"""Pydantic models for churn prediction results."""

from __future__ import annotations

from datetime import datetime
from typing import List, Optional

from pydantic import BaseModel, Field


class ChurnSignal(BaseModel):
    """Indicators contributing to a customer's churn risk."""

    reason: str
    severity: float = Field(ge=0.0, le=1.0)


class ChurnRecommendation(BaseModel):
    """Recommended action to reduce churn risk."""

    title: str
    description: str


class CustomerChurnPrediction(BaseModel):
    """Churn risk prediction for a single customer."""

    customer_id: str
    email: Optional[str] = None
    risk_score: float = Field(ge=0.0, le=1.0)
    risk_level: str
    last_order_at: Optional[datetime] = None
    total_orders: int = 0
    recent_orders: int = 0
    total_revenue: float = 0.0
    average_order_value: float = 0.0
    signals: List[ChurnSignal] = Field(default_factory=list)
    recommendations: List[ChurnRecommendation] = Field(default_factory=list)


class ChurnSummary(BaseModel):
    """Aggregate insight for churn predictions."""

    high_risk: int
    medium_risk: int
    low_risk: int
    generated_at: datetime = Field(default_factory=datetime.utcnow)


class ChurnPredictionResponse(BaseModel):
    """API response wrapper for churn predictions."""

    tenant_id: str
    summary: ChurnSummary
    predictions: List[CustomerChurnPrediction]
