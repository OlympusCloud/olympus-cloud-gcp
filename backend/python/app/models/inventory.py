from __future__ import annotations

from datetime import datetime
from typing import List, Optional
from enum import Enum
from uuid import UUID, uuid4

from pydantic import BaseModel, Field


class StockStatus(str, Enum):
    IN_STOCK = "in_stock"
    LOW_STOCK = "low_stock"
    OUT_OF_STOCK = "out_of_stock"
    REORDER_NEEDED = "reorder_needed"


class ForecastPeriod(str, Enum):
    WEEKLY = "weekly"
    MONTHLY = "monthly"
    QUARTERLY = "quarterly"


class InventoryItem(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    tenant_id: str
    product_id: str
    location_id: Optional[str] = None
    current_stock: int = 0
    min_stock_level: int = 10
    max_stock_level: int = 100
    reorder_point: int = 20
    status: StockStatus = StockStatus.IN_STOCK
    last_updated: datetime = Field(default_factory=datetime.utcnow)


class StockMovement(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    tenant_id: str
    product_id: str
    location_id: Optional[str] = None
    movement_type: str  # "sale", "purchase", "adjustment", "transfer"
    quantity: int
    reference_id: Optional[str] = None  # order_id, purchase_id, etc.
    created_at: datetime = Field(default_factory=datetime.utcnow)


class InventoryForecast(BaseModel):
    tenant_id: str
    product_id: str
    location_id: Optional[str] = None
    period: ForecastPeriod
    predicted_demand: int
    recommended_stock: int
    confidence: float = Field(ge=0.0, le=1.0)
    forecast_date: datetime = Field(default_factory=datetime.utcnow)


class StockAlert(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    tenant_id: str
    product_id: str
    location_id: Optional[str] = None
    alert_type: StockStatus
    current_stock: int
    threshold: int
    message: str
    created_at: datetime = Field(default_factory=datetime.utcnow)


class InventoryReport(BaseModel):
    tenant_id: str
    location_id: Optional[str] = None
    total_items: int
    low_stock_items: int
    out_of_stock_items: int
    reorder_needed: int
    total_value: float
    alerts: List[StockAlert] = Field(default_factory=list)
    forecasts: List[InventoryForecast] = Field(default_factory=list)
    generated_at: datetime = Field(default_factory=datetime.utcnow)