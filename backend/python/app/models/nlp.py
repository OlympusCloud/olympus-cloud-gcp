from __future__ import annotations

from typing import Optional

from pydantic import BaseModel


class NLPInterpretation(BaseModel):
    intent: str
    metric: str
    timeframe: str
    group_by: list[str]
    limit: Optional[int] = None
    confidence: float
    raw: str


class NLPQueryResponse(BaseModel):
    query: str
    result: NLPInterpretation
