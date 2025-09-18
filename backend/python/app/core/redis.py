from __future__ import annotations

from typing import Optional

from redis.asyncio import Redis

from app.core.settings import Settings, get_settings


def create_redis_client(settings: Optional[Settings] = None) -> Redis:
    """Create a Redis client instance using application settings."""

    settings = settings or get_settings()
    return Redis.from_url(settings.redis_url, decode_responses=True)
