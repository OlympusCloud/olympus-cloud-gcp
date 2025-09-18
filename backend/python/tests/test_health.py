import pytest
from httpx import AsyncClient

from app.core.app import create_app
from app.core.settings import get_settings


@pytest.mark.asyncio
async def test_health_endpoint_returns_service_status(monkeypatch):
    monkeypatch.setenv("REDIS_URL", "redis://localhost:0")
    get_settings.cache_clear()

    app = create_app()

    async with AsyncClient(app=app, base_url="http://testserver") as client:
        response = await client.get("/api/health")

    assert response.status_code == 200
    payload = response.json()
    assert payload["status"] == "ok"
    assert payload["service"]
    assert "redis" in payload

    get_settings.cache_clear()
