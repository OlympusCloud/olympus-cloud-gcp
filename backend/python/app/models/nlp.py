from __future__ import annotations

from typing import List, Optional

from pydantic import BaseModel, Field


class NLPQueryInterpretation(BaseModel):
    """Structured representation of a natural language analytics query."""

    intent: str = Field(default="metric_summary")
    metric: str = Field(default="general")
    timeframe: str = Field(default="all_time")
    group_by: List[str] = Field(default_factory=list)
    limit: Optional[int] = None
    confidence: float = Field(default=0.5, ge=0.0, le=1.0)
    raw: str = Field(description="Original user query")


class NLPQueryResponse(BaseModel):
    """API response envelope for NLP query interpretations."""

    query: str
    result: NLPQueryInterpretation
