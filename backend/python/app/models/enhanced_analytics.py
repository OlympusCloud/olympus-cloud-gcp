from __future__ import annotations

from datetime import datetime
from typing import Any, Dict, List, Optional
from enum import Enum

from pydantic import BaseModel, Field

from app.models.analytics import AnalyticsTimeframe


class MetricCategory(str, Enum):
    """Categories for organizing metrics."""
    REVENUE = "revenue"
    CUSTOMERS = "customers"
    INVENTORY = "inventory"
    OPERATIONS = "operations"
    MARKETING = "marketing"


class TrendDirection(str, Enum):
    """Trend direction indicators."""
    UP = "up"
    DOWN = "down"
    STABLE = "stable"
    VOLATILE = "volatile"


class MetricTrend(BaseModel):
    """Trend information for a specific metric."""
    direction: TrendDirection
    percentage_change: float
    period_comparison: str  # "vs last week", "vs last month", etc.
    confidence: float = Field(ge=0.0, le=1.0)


class DetailedMetric(BaseModel):
    """Enhanced metric with trend and context."""
    name: str
    value: float
    formatted_value: str
    category: MetricCategory
    trend: Optional[MetricTrend] = None
    target: Optional[float] = None
    unit: str = ""
    description: str = ""


class CustomerSegment(BaseModel):
    """Customer segmentation data."""
    segment_name: str
    customer_count: int
    percentage_of_total: float
    avg_order_value: float
    total_revenue: float
    last_purchase_days: Optional[int] = None


class ProductPerformance(BaseModel):
    """Product performance metrics."""
    product_id: str
    product_name: str
    units_sold: int
    revenue: float
    profit_margin: Optional[float] = None
    inventory_turns: Optional[float] = None
    trend: Optional[MetricTrend] = None


class LocationMetrics(BaseModel):
    """Location-specific performance metrics."""
    location_id: str
    location_name: str
    revenue: float
    orders: int
    customers: int
    avg_order_value: float
    performance_rank: int


class TimeSeriesPoint(BaseModel):
    """Single point in time series data."""
    timestamp: datetime
    value: float
    label: Optional[str] = None


class TimeSeriesData(BaseModel):
    """Time series data for charts."""
    metric_name: str
    data_points: List[TimeSeriesPoint]
    timeframe: AnalyticsTimeframe
    unit: str = ""


class EnhancedDashboardMetrics(BaseModel):
    """Comprehensive dashboard metrics with rich insights."""
    
    # Basic info
    tenant_id: str
    location_id: Optional[str] = None
    timeframe: AnalyticsTimeframe
    generated_at: datetime = Field(default_factory=datetime.utcnow)
    
    # Core KPIs
    key_metrics: List[DetailedMetric] = Field(default_factory=list)
    
    # Revenue breakdown
    revenue_by_category: Dict[str, float] = Field(default_factory=dict)
    revenue_by_payment_method: Dict[str, float] = Field(default_factory=dict)
    
    # Customer insights
    customer_segments: List[CustomerSegment] = Field(default_factory=list)
    new_vs_returning: Dict[str, int] = Field(default_factory=dict)
    
    # Product performance
    top_products: List[ProductPerformance] = Field(default_factory=list)
    low_stock_alerts: List[Dict[str, Any]] = Field(default_factory=list)
    
    # Location comparison (if multi-location)
    location_performance: List[LocationMetrics] = Field(default_factory=list)
    
    # Time series for charts
    time_series: List[TimeSeriesData] = Field(default_factory=list)
    
    # Operational metrics
    avg_order_fulfillment_time: Optional[float] = None
    order_accuracy_rate: Optional[float] = None
    customer_satisfaction_score: Optional[float] = None
    
    # Alerts and notifications
    alerts: List[str] = Field(default_factory=list)
    recommendations: List[str] = Field(default_factory=list)


class AnalyticsFilter(BaseModel):
    """Advanced filtering options for analytics."""
    
    tenant_id: str
    location_ids: Optional[List[str]] = None
    timeframe: AnalyticsTimeframe = AnalyticsTimeframe.THIS_MONTH
    start_date: Optional[datetime] = None
    end_date: Optional[datetime] = None
    
    # Category filters
    product_categories: Optional[List[str]] = None
    customer_segments: Optional[List[str]] = None
    payment_methods: Optional[List[str]] = None
    
    # Comparison options
    compare_to_previous: bool = False
    include_trends: bool = True
    include_forecasts: bool = False
    
    # Aggregation options
    group_by: Optional[str] = None  # "day", "week", "month", "product", "location"
    limit: int = Field(default=100, ge=1, le=1000)


class ForecastData(BaseModel):
    """Predictive analytics data."""
    
    metric_name: str
    forecast_points: List[TimeSeriesPoint]
    confidence_interval: Dict[str, List[float]]  # "upper" and "lower" bounds
    accuracy_score: float = Field(ge=0.0, le=1.0)
    model_used: str = "linear_regression"


class BusinessHealthScore(BaseModel):
    """Overall business health assessment."""
    
    overall_score: float = Field(ge=0.0, le=100.0)
    category_scores: Dict[MetricCategory, float] = Field(default_factory=dict)
    
    strengths: List[str] = Field(default_factory=list)
    concerns: List[str] = Field(default_factory=list)
    action_items: List[str] = Field(default_factory=list)
    
    last_updated: datetime = Field(default_factory=datetime.utcnow)


class CompetitiveAnalysis(BaseModel):
    """Industry benchmarking data."""
    
    industry_type: str
    benchmarks: Dict[str, float] = Field(default_factory=dict)
    percentile_ranking: Dict[str, float] = Field(default_factory=dict)
    
    above_average: List[str] = Field(default_factory=list)
    below_average: List[str] = Field(default_factory=list)


class EnhancedAnalyticsResponse(BaseModel):
    """Complete analytics response with all enhanced data."""
    
    dashboard_metrics: EnhancedDashboardMetrics
    business_health: Optional[BusinessHealthScore] = None
    forecasts: List[ForecastData] = Field(default_factory=list)
    competitive_analysis: Optional[CompetitiveAnalysis] = None


class CohortPeriodMetrics(BaseModel):
    """Metrics for a single retention period within a cohort."""

    period_index: int = Field(ge=0)
    period_label: str
    customers_active: int = Field(ge=0)
    revenue: float = Field(ge=0.0)
    retention_rate: float = Field(ge=0.0, le=1.0)


class CohortAnalysis(BaseModel):
    """Retention and revenue metrics for a cohort."""

    cohort_key: str
    cohort_size: int = Field(ge=0)
    periods: List[CohortPeriodMetrics] = Field(default_factory=list)
    average_retention: float = Field(ge=0.0, le=1.0)
    lifetime_value: float = Field(ge=0.0)


class CohortAnalyticsResponse(BaseModel):
    """Full cohort analytics payload."""

    tenant_id: str
    period_granularity: str
    cohorts: List[CohortAnalysis] = Field(default_factory=list)
    average_retention_rate: float = Field(ge=0.0, le=1.0)
    best_cohort: Optional[str] = None
    period_labels: List[str] = Field(default_factory=list)
    
    # Metadata
    processing_time_ms: float
    data_freshness: str  # "real-time", "5 minutes ago", etc.
    cache_hit: bool = False
