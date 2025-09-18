from __future__ import annotations

from typing import Any

from fastapi import APIRouter, Request

from app.core.settings import get_settings
from app.core.state import RuntimeState

api_router = APIRouter()


@api_router.get("/health", tags=["monitoring"])
async def health_check(request: Request) -> dict[str, Any]:
    """Report service health and runtime information."""

    settings = get_settings()
    runtime_state: RuntimeState = getattr(request.app.state, "runtime", RuntimeState())

    return {
        "status": "ok",
        "service": settings.app_name,
        "environment": settings.environment,
        "version": request.app.version,
        "redis": {
            "connected": runtime_state.redis_connected,
            "subscriber_running": runtime_state.event_subscriber_running,
        },
    }
