from __future__ import annotations

from datetime import datetime
from types import SimpleNamespace
from unittest.mock import AsyncMock, MagicMock
from uuid import uuid4

import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings
from app.models.analytics import AnalyticsTimeframe
from app.models.retail import (
    RetailAnalytics,
    RetailChannelPerformance,
    RetailProductPerformance,
    RetailPromotionPerformance,
    RetailRecommendation,
    RetailStockAlert,
)
from app.services.retail.service import RetailService


@pytest.fixture
def mock_session_factory():
    session = AsyncMock()
    session_factory = MagicMock()
    session_factory.return_value.__aenter__.return_value = session
    return session_factory, session


@pytest.fixture
def retail_service(mock_session_factory):
    session_factory, _ = mock_session_factory
    return RetailService(session_factory)


@pytest.mark.asyncio
async def test_get_retail_analytics_calculates_kpis(retail_service, mock_session_factory):
    session_factory, session = mock_session_factory

    metrics_row = SimpleNamespace(
        total_revenue=50000.0,
        average_order_value=125.0,
        total_orders=400,
        repeat_orders=120,
        unique_customers=350,
        units_sold=1500,
        inventory_units=500,
        total_cost=30000.0,
        barcode_scans=800,
    )

    product_rows = [
        SimpleNamespace(
            product_id=str(uuid4()),
            product_name="Signature Roast Coffee",
            category="Beverages",
            units_sold=600,
            revenue=18000.0,
            stock_on_hand=100,
            gross_margin=8.5,
        ),
        SimpleNamespace(
            product_id=str(uuid4()),
            product_name="Organic Granola",
            category="Grocery",
            units_sold=250,
            revenue=7500.0,
            stock_on_hand=120,
            gross_margin=5.2,
        ),
    ]

    channel_rows = [
        SimpleNamespace(
            sales_channel="online",
            revenue=30000.0,
            orders=220,
            previous_revenue=20000.0,
            average_order_value=136.36,
        ),
        SimpleNamespace(
            sales_channel="in_store",
            revenue=20000.0,
            orders=180,
            previous_revenue=15000.0,
            average_order_value=111.11,
        ),
    ]

    promotion_rows = [
        SimpleNamespace(
            promotion_id=str(uuid4()),
            name="Weekend Flash Sale",
            type="percentage_discount",
            status="active",
            revenue=12000.0,
            uplift=0.18,
        )
    ]

    stock_alert_rows = [
        SimpleNamespace(
            product_id=str(uuid4()),
            sku="SKU-001",
            product_name="Organic Granola",
            status="low_stock",
            current_stock=12,
            threshold=40,
            days_of_cover=2.5,
        )
    ]

    session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=metrics_row)),
        MagicMock(fetchall=MagicMock(return_value=product_rows)),
        MagicMock(fetchall=MagicMock(return_value=channel_rows)),
        MagicMock(fetchall=MagicMock(return_value=promotion_rows)),
        MagicMock(fetchall=MagicMock(return_value=stock_alert_rows)),
    ]

    tenant_id = str(uuid4())

    analytics = await retail_service.get_retail_analytics(
        tenant_id,
        AnalyticsTimeframe.ALL_TIME,
    )

    assert isinstance(analytics, RetailAnalytics)
    assert str(analytics.tenant_id) == tenant_id
    assert analytics.total_revenue == pytest.approx(50000.0)
    assert analytics.repeat_purchase_rate == pytest.approx(0.3)
    assert analytics.sell_through_rate == pytest.approx(0.75)
    assert analytics.inventory_turnover == pytest.approx(3.0)
    assert analytics.customer_conversion_rate == pytest.approx(0.5)
    assert len(analytics.top_products) == 2
    assert analytics.top_products[0].sell_through_rate == pytest.approx(600 / 700)
    assert len(analytics.channel_performance) == 2
    assert analytics.channel_performance[0].growth_rate == pytest.approx(0.5)
    assert len(analytics.stock_alerts) == 1
    assert analytics.stock_alerts[0].status == "low_stock"


@pytest.mark.asyncio
async def test_generate_promotions_prioritizes_actions(retail_service, mock_session_factory):
    session_factory, session = mock_session_factory

    metrics_row = SimpleNamespace(
        total_revenue=18000.0,
        average_order_value=90.0,
        total_orders=200,
        repeat_orders=20,
        unique_customers=180,
        units_sold=100,
        inventory_units=400,
        total_cost=12000.0,
        barcode_scans=500,
    )

    product_rows = [
        SimpleNamespace(
            product_id=str(uuid4()),
            product_name="Premium Headphones",
            category="Electronics",
            units_sold=80,
            revenue=9600.0,
            stock_on_hand=10,
            gross_margin=12.0,
        )
    ]

    channel_rows = [
        SimpleNamespace(
            sales_channel="online",
            revenue=6000.0,
            orders=90,
            previous_revenue=9000.0,
            average_order_value=66.66,
        )
    ]

    promotion_rows = [
        SimpleNamespace(
            promotion_id=str(uuid4()),
            name="Spring Bundle",
            type="bundle",
            status="planned",
            revenue=1500.0,
            uplift=0.05,
        )
    ]

    stock_alert_rows = [
        SimpleNamespace(
            product_id=str(uuid4()),
            sku="SKU-ALERT",
            product_name="Premium Headphones",
            status="low_stock",
            current_stock=5,
            threshold=20,
            days_of_cover=1.0,
        )
    ]

    session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=metrics_row)),
        MagicMock(fetchall=MagicMock(return_value=product_rows)),
        MagicMock(fetchall=MagicMock(return_value=channel_rows)),
        MagicMock(fetchall=MagicMock(return_value=promotion_rows)),
        MagicMock(fetchall=MagicMock(return_value=stock_alert_rows)),
    ]

    tenant_id = str(uuid4())

    recommendations = await retail_service.generate_promotions(tenant_id)

    assert recommendations
    assert recommendations[0].priority == 0  # Stock alert should be highest priority
    types = {rec.type for rec in recommendations}
    assert {"inventory", "loyalty", "channel", "cross_sell"}.issubset(types)


class StubRetailService:
    async def get_retail_analytics(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: str | None = None,
        start_date: date | None = None,
        end_date: date | None = None,
    ) -> RetailAnalytics:
        return RetailAnalytics(
            tenant_id=uuid4(),
            location_id=None,
            date=datetime.utcnow(),
            total_revenue=25000.0,
            average_order_value=98.5,
            total_orders=180,
            unique_customers=160,
            repeat_purchase_rate=0.28,
            inventory_turnover=2.4,
            sell_through_rate=0.62,
            gross_margin_rate=0.34,
            customer_conversion_rate=0.42,
            barcode_scans=430,
            top_products=[
                RetailProductPerformance(
                    product_id=str(uuid4()),
                    name="Smart Watch",
                    category="Electronics",
                    revenue=8200.0,
                    units_sold=85,
                    stock_on_hand=20,
                    sell_through_rate=0.81,
                    gross_margin=14.0,
                )
            ],
            channel_performance=[
                RetailChannelPerformance(
                    channel="online",
                    revenue=16000.0,
                    orders=110,
                    growth_rate=0.25,
                    average_order_value=145.45,
                )
            ],
            promotion_performance=[
                RetailPromotionPerformance(
                    promotion_id=str(uuid4()),
                    name="Holiday Promo",
                    type="percentage_discount",
                    status="active",
                    revenue=5000.0,
                    uplift=0.12,
                )
            ],
            stock_alerts=[
                RetailStockAlert(
                    product_id=str(uuid4()),
                    sku="SKU-123",
                    product_name="Smart Watch",
                    status="low_stock",
                    current_stock=8,
                    threshold=25,
                    days_of_cover=1.8,
                )
            ],
        )

    async def generate_promotions(
        self,
        tenant_id: str,
        timeframe: AnalyticsTimeframe,
        *,
        location_id: str | None = None,
        start_date: date | None = None,
        end_date: date | None = None,
    ) -> list[RetailRecommendation]:
        return [
            RetailRecommendation(
                type="inventory",
                title="Restock Smart Watch",
                description="Inventory is trending low; initiate replenishment order.",
                impact="high",
                priority=0,
                data={"product": "Smart Watch"},
            )
        ]


@pytest.mark.asyncio
async def test_retail_analytics_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.retail_service = StubRetailService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get("/api/retail/analytics", params={"tenant_id": str(uuid4())})

    assert response.status_code == 200
    payload = response.json()
    assert payload["total_revenue"] == 25000.0
    assert payload["total_orders"] == 180
    assert payload["channel_performance"][0]["channel"] == "online"

    get_settings.cache_clear()


@pytest.mark.asyncio
async def test_retail_promotions_endpoint(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.retail_service = StubRetailService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get("/api/retail/promotions", params={"tenant_id": str(uuid4())})

    assert response.status_code == 200
    payload = response.json()
    assert isinstance(payload, list)
    assert payload[0]["type"] == "inventory"

    get_settings.cache_clear()
