"""Machine learning related API routes."""

from __future__ import annotations

from fastapi import APIRouter, Depends, Query

from app.api.dependencies import get_churn_service
from app.models.churn import ChurnPredictionResponse
from app.services.ml.churn import ChurnPredictionService

router = APIRouter(prefix="/api/ml", tags=["ml"])


@router.get("/churn", response_model=ChurnPredictionResponse)
async def get_churn_predictions(
    tenant_id: str = Query(..., description="Tenant identifier"),
    limit: int = Query(50, ge=1, le=500, description="Maximum customers to score"),
    churn_service: ChurnPredictionService = Depends(get_churn_service),
) -> ChurnPredictionResponse:
    """Return churn predictions for the given tenant."""

    return await churn_service.predict(tenant_id, limit=limit)
