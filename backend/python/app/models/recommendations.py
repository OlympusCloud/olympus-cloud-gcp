from __future__ import annotations

from enum import Enum
from typing import List

from pydantic import BaseModel, Field


class RecommendationCategory(str, Enum):
    """Categories of optimization recommendations."""

    INVENTORY = "inventory"
    STAFFING = "staffing"
    MARKETING = "marketing"
    OPERATIONS = "operations"


class RecommendationPriority(str, Enum):
    """Priority levels for recommended actions."""

    HIGH = "high"
    MEDIUM = "medium"
    LOW = "low"


class RecommendationInsight(BaseModel):
    """Supporting metric or observation for a recommendation."""

    metric: str = Field(..., description="Name of the relevant metric")
    value: str = Field(..., description="Metric value rendered for humans")
    context: str = Field(..., description="Explanation of why the metric matters")


class Recommendation(BaseModel):
    """Actionable recommendation surfaced by the analytics service."""

    id: str = Field(..., description="Stable identifier for the recommendation")
    category: RecommendationCategory = Field(..., description="Area of the business impacted")
    priority: RecommendationPriority = Field(..., description="Urgency of the recommended action")
    action: str = Field(..., description="Concise description of the suggested next step")
    impact: str = Field(..., description="Expected business impact when executed")
    rationale: str = Field(..., description="Reasoning behind the recommendation")
    insights: List[RecommendationInsight] = Field(
        default_factory=list,
        description="Supporting data points used to derive the recommendation",
    )


class RecommendationResponse(BaseModel):
    """Response payload for recommendation endpoints."""

    tenant_id: str
    recommendations: List[Recommendation]
