from __future__ import annotations

from enum import Enum

from pydantic import BaseModel


class AnalyticsTimeframe(str, Enum):
    ALL_TIME = "all_time"
    TODAY = "today"
    YESTERDAY = "yesterday"
    THIS_WEEK = "this_week"
    LAST_WEEK = "last_week"
    THIS_MONTH = "this_month"
    LAST_MONTH = "last_month"
    YEAR_TO_DATE = "year_to_date"


class AnalyticsMetrics(BaseModel):
    active_users: int = 0
    orders: int = 0
    revenue: float = 0.0
    inventory_warnings: int = 0
    timeframe: AnalyticsTimeframe = AnalyticsTimeframe.ALL_TIME


class AnalyticsDashboardResponse(BaseModel):
    tenant_id: str
    metrics: AnalyticsMetrics
