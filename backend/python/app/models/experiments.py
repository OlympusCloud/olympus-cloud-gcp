"""Pydantic models for analytics experiments and A/B testing."""

from __future__ import annotations

from datetime import datetime
from enum import Enum
from typing import Any, Dict, List, Optional

from pydantic import BaseModel, Field, model_validator


class ExperimentStatus(str, Enum):
    """Lifecycle states for experiments."""

    DRAFT = "draft"
    RUNNING = "running"
    PAUSED = "paused"
    COMPLETED = "completed"
    ARCHIVED = "archived"


class ExperimentVariant(BaseModel):
    """Configuration for a single experiment variant."""

    key: str = Field(..., min_length=1, description="Unique identifier for the variant")
    name: str = Field(..., min_length=1, description="Human readable name")
    allocation: float = Field(..., gt=0.0, le=1.0, description="Traffic allocation percentage (0-1 range)")
    description: Optional[str] = Field(None, description="Optional details about the variant")
    metadata: Dict[str, Any] = Field(default_factory=dict, description="Arbitrary variant metadata")


class ExperimentSuccessMetric(BaseModel):
    """Metric definitions tracked during the experiment."""

    name: str = Field(..., min_length=1)
    goal: str = Field("increase", description="Desired direction of change (increase/decrease)")
    target: Optional[float] = Field(None, description="Optional target value for the metric")
    weight: float = Field(1.0, gt=0.0, description="Relative weight when combining metrics")


class ExperimentDefinition(BaseModel):
    """Payload required to create or update an experiment."""

    tenant_id: str = Field(..., min_length=1)
    name: str = Field(..., min_length=1)
    description: Optional[str] = None
    hypothesis: Optional[str] = None
    variants: List[ExperimentVariant] = Field(..., min_length=2)
    success_metrics: List[ExperimentSuccessMetric] = Field(..., min_length=1)
    start_date: Optional[datetime] = None
    end_date: Optional[datetime] = None
    status: ExperimentStatus = ExperimentStatus.DRAFT
    created_by: str = Field(..., min_length=1)

    @model_validator(mode="after")
    def validate_allocation(cls, values: "ExperimentDefinition") -> "ExperimentDefinition":  # type: ignore[override]
        total_allocation = sum(variant.allocation for variant in values.variants)
        if abs(total_allocation - 1.0) > 0.01:
            raise ValueError("Variant allocations must sum to approximately 1.0")
        return values


class Experiment(BaseModel):
    """Stored experiment with metadata and configuration."""

    id: str
    tenant_id: str
    name: str
    description: Optional[str]
    hypothesis: Optional[str]
    status: ExperimentStatus
    variants: List[ExperimentVariant]
    success_metrics: List[ExperimentSuccessMetric]
    traffic_allocation: Dict[str, float]
    start_date: Optional[datetime]
    end_date: Optional[datetime]
    created_by: str
    created_at: datetime
    updated_at: datetime
    results: Dict[str, Any] = Field(default_factory=dict)


class ExperimentSummary(BaseModel):
    """Lightweight summary used for listings."""

    id: str
    name: str
    status: ExperimentStatus
    start_date: Optional[datetime]
    end_date: Optional[datetime]
    winner: Optional[str] = None
    conversions: Optional[int] = None
    created_at: datetime


class VariantResult(BaseModel):
    """Aggregated performance metrics for a variant."""

    name: str
    participants: int
    conversions: int
    conversion_rate: float
    total_conversion_value: float
    avg_conversion_value: float
    lift: Optional[float] = None


class VariantComparison(BaseModel):
    """Pairwise comparison between baseline and another variant."""

    baseline: str
    variant: str
    lift: Optional[float]
    p_value: Optional[float]
    confidence: Optional[float]
    is_significant: bool


class ExperimentResults(BaseModel):
    """Computed experiment analytics including statistical tests."""

    baseline_variant: str
    variants: List[VariantResult]
    comparisons: List[VariantComparison]
    suggested_winner: Optional[str] = None
    overall_confidence: Optional[float] = None


class ExperimentDetail(BaseModel):
    """Complete experiment detail including computed statistics."""

    experiment: Experiment
    results: ExperimentResults


class ParticipantAssignment(BaseModel):
    """Represents an assignment of a participant to a variant."""

    experiment_id: str
    variant_name: str
    user_id: Optional[str] = None
    customer_id: Optional[str] = None
    session_id: Optional[str] = None
    assigned_at: datetime = Field(default_factory=datetime.utcnow)

    @model_validator(mode="after")
    def validate_identity(cls, values: "ParticipantAssignment") -> "ParticipantAssignment":  # type: ignore[override]
        if not (values.user_id or values.customer_id or values.session_id):
            raise ValueError("At least one participant identifier must be provided")
        return values


class ConversionUpdate(BaseModel):
    """Conversion payload for a participant assignment."""

    participant_id: str
    converted_at: datetime = Field(default_factory=datetime.utcnow)
    conversion_value: Optional[float] = None


class ParticipantRecord(BaseModel):
    """Participant row persisted in the database."""

    id: str
    experiment_id: str
    variant_name: str
    user_id: Optional[str]
    customer_id: Optional[str]
    session_id: Optional[str]
    assigned_at: datetime
    converted_at: Optional[datetime]
    conversion_value: Optional[float]
