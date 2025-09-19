"""Enhanced NLP API routes."""

from datetime import datetime
from typing import Any, Dict, List, Optional

from fastapi import APIRouter, Depends, HTTPException, Query
from pydantic import BaseModel

from app.api.dependencies import get_enhanced_nlp_service
from app.services.nlp.enhanced_nlp import EnhancedNLPService

router = APIRouter(prefix="/nlp", tags=["nlp"])


class NLPQuery(BaseModel):
    """NLP query request model."""
    text: str
    session_id: Optional[str] = "default"
    tenant_id: Optional[str] = None
    user_id: Optional[str] = None


class TrainingData(BaseModel):
    """Training data for intent classification."""
    text: str
    correct_intent: str
    session_id: Optional[str] = "default"


class NLPResponse(BaseModel):
    """NLP processing response model."""

    intent: Dict[str, Any]
    entities: Dict[str, List[Dict[str, Any]]]
    response: str
    context: Dict[str, Any]
    session_id: str
    processed_at: datetime


@router.post("/process", response_model=NLPResponse)
async def process_query(
    query: NLPQuery,
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> NLPResponse:
    """Process a natural language query."""
    try:
        result = nlp_service.process_query(
            text=query.text,
            session_id=query.session_id,
            tenant_id=query.tenant_id,
            user_id=query.user_id
        )
        return NLPResponse.model_validate(result)
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error processing query: {str(e)}")


@router.get("/suggestions")
async def get_suggestions(
    partial_text: str = Query(..., description="Partial text input"),
    session_id: str = Query("default", description="Session ID for context"),
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> Dict[str, List[str]]:
    """Get query suggestions based on partial input."""
    try:
        context = nlp_service.context_manager.get_context(session_id)
        suggestions = nlp_service.get_suggestions(partial_text, context)
        return {"suggestions": suggestions}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error getting suggestions: {str(e)}")


@router.post("/train")
async def train_intent(
    training_data: TrainingData,
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> Dict[str, str]:
    """Train the intent classifier with user feedback."""
    try:
        result = nlp_service.train_intent(
            text=training_data.text,
            correct_intent=training_data.correct_intent,
            session_id=training_data.session_id
        )
        return result
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error training intent: {str(e)}")


@router.get("/analytics/{session_id}")
async def get_nlp_analytics(
    session_id: str,
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> Dict[str, Any]:
    """Get NLP usage analytics for a session."""
    try:
        analytics = nlp_service.get_analytics(session_id)
        return analytics
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error getting analytics: {str(e)}")


@router.delete("/context/{session_id}")
async def clear_context(
    session_id: str,
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> Dict[str, str]:
    """Clear context for a session."""
    try:
        nlp_service.context_manager.clear_context(session_id)
        return {"status": "context_cleared", "session_id": session_id}
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Error clearing context: {str(e)}")


@router.get("/intents")
async def get_available_intents(
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> Dict[str, List[str]]:
    """Get list of available intents."""
    intents = list(nlp_service.intent_classifier.intent_patterns.keys())
    return {"intents": intents}


@router.get("/entities")
async def get_available_entities(
    nlp_service: EnhancedNLPService = Depends(get_enhanced_nlp_service),
) -> Dict[str, List[str]]:
    """Get list of available entity types."""
    entities = list(nlp_service.entity_extractor.patterns.keys())
    return {"entity_types": entities}


@router.get("/health")
async def nlp_health_check():
    """Health check for NLP service."""
    return {
        "status": "healthy",
        "service": "enhanced_nlp",
        "version": "1.0.0",
        "features": [
            "intent_classification",
            "entity_extraction", 
            "context_management",
            "response_generation",
            "query_suggestions",
            "training_feedback"
        ]
    }
