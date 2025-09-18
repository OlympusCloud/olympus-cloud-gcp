import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings
from app.models.nlp import NLPInterpretation


class StubNLPService:
    async def interpret(self, query: str):
        return NLPInterpretation(
            intent="metric_summary",
            metric="revenue",
            timeframe="today",
            group_by=[],
            limit=None,
            confidence=0.9,
            raw=query,
        )


@pytest.mark.asyncio
async def test_nlp_query_endpoint_returns_result(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()
    app.state.nlp_service = StubNLPService()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.post("/api/analytics/nlp/query", json={"query": "Show revenue"})

    assert response.status_code == 200
    payload = response.json()
    assert payload["query"] == "Show revenue"
    assert payload["result"]["metric"] == "revenue"
    assert payload["result"]["timeframe"] == "today"

    get_settings.cache_clear()
