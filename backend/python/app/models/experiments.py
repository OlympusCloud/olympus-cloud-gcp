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
    """A variant within an experiment."""

    name: str
    description: Optional[str] = None
    weight: float = Field(ge=0, le=1)
    is_control: bool = False


class ParticipantAssignment(BaseModel):
    """Assignment of a participant to a variant."""

    participant_id: str
    experiment_id: str
    variant: str
    assigned_at: datetime = Field(default_factory=datetime.utcnow)


class ParticipantRecord(BaseModel):
    """Record of a participant in an experiment."""

    participant_id: str
    experiment_id: str
    variant: str
    has_converted: bool = False
    conversion_value: Optional[float] = None
    conversion_time: Optional[datetime] = None


class ConversionUpdate(BaseModel):
    """Update for a conversion event."""

    participant_id: str
    experiment_id: str
    variant: str
    value: Optional[float] = None
    occurred_at: datetime = Field(default_factory=datetime.utcnow)


class ExperimentDefinition(BaseModel):
    """Definition for creating an experiment."""

    name: str
    description: Optional[str] = None
    variants: List[ExperimentVariant]
    status: ExperimentStatus = ExperimentStatus.DRAFT
    starts_at: Optional[datetime] = None
    ends_at: Optional[datetime] = None


class Experiment(BaseModel):
    """Full experiment model."""

    id: str
    name: str
    description: Optional[str] = None
    variants: List[ExperimentVariant]
    status: ExperimentStatus
    starts_at: Optional[datetime] = None
    ends_at: Optional[datetime] = None
    created_at: datetime
    updated_at: datetime


class ExperimentSummary(BaseModel):
    """Summary of an experiment."""

    id: str
    name: str
    status: ExperimentStatus
    variants_count: int
    participants_count: int
    conversions_count: int


class ExperimentDetail(BaseModel):
    """Detailed experiment information."""

    experiment: Experiment
    summary: ExperimentSummary
    results: Optional["ExperimentResults"] = None


class VariantResult(BaseModel):
    """Results for a single variant."""

    name: str
    participants: int
    conversions: int
    conversion_rate: float
    average_value: float
    confidence_level: Optional[float] = None


class VariantComparison(BaseModel):
    """Comparison between two variants."""

    variant_a: str
    variant_b: str
    conversion_rate_diff: float
    relative_improvement: float
    confidence_level: float
    is_significant: bool


class ExperimentResults(BaseModel):
    """Complete results of an experiment."""

    experiment_id: str
    status: ExperimentStatus
    variants: List[VariantResult]
    comparisons: List[VariantComparison]
    winner: Optional[str] = None
    confidence_level: float


class ExperimentSuccessMetric(BaseModel):
    """Success metric for an experiment."""

    name: str
    target_value: float
    current_value: float
    is_met: bool
