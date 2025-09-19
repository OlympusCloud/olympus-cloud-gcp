"""Experiment A/B testing API endpoints."""

from __future__ import annotations

from datetime import date, datetime
from typing import List, Optional

from fastapi import APIRouter, Depends, HTTPException, Query, status
from pydantic import BaseModel

from app.api.dependencies import get_ab_testing_service
from app.models import experiments as experiments_models
from app.services.analytics.ab_testing import ABTestingService

router = APIRouter(prefix="/api/analytics/experiments", tags=["analytics"])

class ParticipantAssignmentRequest(BaseModel):
    variant_name: str
    user_id: Optional[str] = None
    customer_id: Optional[str] = None
    session_id: Optional[str] = None
    assigned_at: Optional[datetime] = None

    def to_model(self, experiment_id: str) -> experiments_models.ParticipantAssignment:
        payload = {
            "experiment_id": experiment_id,
            "variant_name": self.variant_name,
            "user_id": self.user_id,
            "customer_id": self.customer_id,
            "session_id": self.session_id,
            "assigned_at": self.assigned_at or datetime.utcnow(),
        }
        return experiments_models.ParticipantAssignment(**payload)


def _to_datetime(value: Optional[date], *, start: bool) -> Optional[datetime]:
    if value is None:
        return None
    return datetime.combine(value, datetime.min.time() if start else datetime.max.time())


@router.get("", response_model=List[experiments_models.ExperimentSummary])
async def list_experiments(
    tenant_id: str = Query(..., description="Tenant identifier"),
    ab_testing_service: ABTestingService = Depends(get_ab_testing_service),
) -> List[experiments_models.ExperimentSummary]:
    """List configured experiments for a tenant."""

    return await ab_testing_service.list_experiments(tenant_id)


@router.post(
    "",
    response_model=experiments_models.Experiment,
    status_code=status.HTTP_201_CREATED,
)
async def create_experiment(
    payload: experiments_models.ExperimentDefinition,
    ab_testing_service: ABTestingService = Depends(get_ab_testing_service),
) -> experiments_models.Experiment:
    """Create a new experiment definition."""

    return await ab_testing_service.create_experiment(payload)


@router.get(
    "/{experiment_id}",
    response_model=experiments_models.ExperimentDetail,
)
async def get_experiment_detail(
    experiment_id: str,
    tenant_id: str = Query(..., description="Tenant identifier"),
    ab_testing_service: ABTestingService = Depends(get_ab_testing_service),
) -> experiments_models.ExperimentDetail:
    """Fetch experiment configuration including computed statistics."""

    detail = await ab_testing_service.get_experiment_detail(tenant_id, experiment_id)
    if detail is None:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="Experiment not found")
    return detail


@router.post(
    "/{experiment_id}/participants",
    response_model=experiments_models.ParticipantRecord,
    status_code=status.HTTP_201_CREATED,
)
async def assign_participant(
    experiment_id: str,
    payload: ParticipantAssignmentRequest,
    ab_testing_service: ABTestingService = Depends(get_ab_testing_service),
) -> experiments_models.ParticipantRecord:
    """Assign a participant to a variant, upserting existing assignments."""

    assignment = payload.to_model(experiment_id)
    return await ab_testing_service.record_assignment(assignment)


@router.post(
    "/{experiment_id}/conversions",
    response_model=experiments_models.ParticipantRecord,
)
async def record_conversion(
    experiment_id: str,
    payload: experiments_models.ConversionUpdate,
    ab_testing_service: ABTestingService = Depends(get_ab_testing_service),
) -> experiments_models.ParticipantRecord:
    """Record a conversion for a participant and ensure experiment alignment."""

    record = await ab_testing_service.record_conversion(payload)
    if record.experiment_id != experiment_id:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Participant does not belong to the specified experiment",
        )
    return record
