from __future__ import annotations

import re
from typing import Any

from app.core.logging import logger
from app.models.nlp import NLPQueryInterpretation

_METRIC_KEYWORDS: dict[str, tuple[str, ...]] = {
    "revenue": ("revenue", "sales", "income", "turnover"),
    "orders": ("orders", "order", "purchases", "transactions"),
    "customers": ("customers", "users", "signups", "leads"),
    "inventory": ("inventory", "stock", "supply", "items"),
}

_TIMEFRAME_KEYWORDS: dict[str, tuple[str, ...]] = {
    "today": ("today", "last 24 hours", "since this morning"),
    "yesterday": ("yesterday", "last day"),
    "this_week": ("this week", "current week"),
    "last_week": ("last week", "previous week"),
    "this_month": ("this month", "current month"),
    "last_month": ("last month", "previous month"),
    "year_to_date": ("year to date", "ytd"),
}

_GROUP_BY_KEYWORDS: dict[str, tuple[str, ...]] = {
    "location": ("by location", "per location", "each store", "each location"),
    "channel": ("by channel", "per channel", "each channel"),
    "product": ("by product", "per product", "each product", "sku"),
}


class NaturalLanguageQueryService:
    """Translate natural language analytics questions into structured queries."""

    async def interpret(self, query: str) -> NLPQueryInterpretation:
        """Return a lightweight interpretation of the query.

        This heuristic approach identifies the metric, timeframe, and grouping requests
        so downstream services can respond while a full NLP pipeline is under
        construction.
        """

        logger.info("analytics.nlp.interpret", extra={"query": query})
        normalized = query.lower()

        intent = "metric_summary"
        metric = self._detect_metric(normalized)
        timeframe = self._detect_timeframe(normalized)
        group_by = self._detect_group_by(normalized)
        limit = self._detect_limit(normalized)

        confidence = 0.5
        if metric != "general":
            confidence += 0.25
        if timeframe != "all_time":
            confidence += 0.15
        if group_by:
            confidence += 0.1
        confidence = min(confidence, 0.95)

        return NLPQueryInterpretation(
            intent=intent,
            metric=metric,
            timeframe=timeframe,
            group_by=group_by,
            limit=limit,
            confidence=round(confidence, 2),
            raw=query,
        )

    def _detect_metric(self, text: str) -> str:
        for metric, keywords in _METRIC_KEYWORDS.items():
            if any(keyword in text for keyword in keywords):
                return metric
        return "general"

    def _detect_timeframe(self, text: str) -> str:
        for timeframe, keywords in _TIMEFRAME_KEYWORDS.items():
            if any(keyword in text for keyword in keywords):
                return timeframe
        return "all_time"

    def _detect_group_by(self, text: str) -> list[str]:
        groups: list[str] = []
        for group, keywords in _GROUP_BY_KEYWORDS.items():
            if any(keyword in text for keyword in keywords):
                groups.append(group)
        return groups

    def _detect_limit(self, text: str) -> int | None:
        match = re.search(r"top\\s+(\\d+)", text)
        if match:
            return int(match.group(1))
        return None
