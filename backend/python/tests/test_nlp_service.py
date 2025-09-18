import pytest

from app.services.nlp.query_service import NaturalLanguageQueryService


@pytest.mark.asyncio
async def test_interpret_detects_metric_timeframe_and_group():
    service = NaturalLanguageQueryService()
    result = await service.interpret("Show revenue this week by location")

    assert result["metric"] == "revenue"
    assert result["timeframe"] == "this_week"
    assert "location" in result["group_by"]
    assert result["confidence"] >= 0.7


@pytest.mark.asyncio
async def test_interpret_handles_top_n_queries():
    service = NaturalLanguageQueryService()
    result = await service.interpret("Top 5 products by orders last month")

    assert result["metric"] == "orders"
    assert result["timeframe"] == "last_month"
    assert "product" in result["group_by"]
    assert result["limit"] == 5


@pytest.mark.asyncio
async def test_interpret_falls_back_to_general():
    service = NaturalLanguageQueryService()
    result = await service.interpret("How are things going?")

    assert result["metric"] == "general"
    assert result["timeframe"] == "all_time"
    assert result["group_by"] == []
    assert result["confidence"] <= 0.6
