from __future__ import annotations

import json
from typing import Any, Optional

from app.core.logging import logger
from app.models.events import AnalyticsEvent
from app.services.analytics.service import AnalyticsService


class EventProcessor:
    """Dispatch events to analytics pipelines."""

    def __init__(self, analytics_service: Optional[AnalyticsService] = None) -> None:
        self._analytics_service = analytics_service

    async def handle_raw_event(self, channel: str, message: Any) -> None:
        """Parse and process an event message from Redis."""

        try:
            if isinstance(message, (bytes, bytearray)):
                payload = message.decode("utf-8")
            else:
                payload = str(message)

            data = json.loads(payload)
            event = AnalyticsEvent.model_validate(data)
        except (json.JSONDecodeError, ValueError) as exc:
            logger.warning("analytics.event.invalid", extra={"error": str(exc), "channel": channel})
            return

        await self.handle_event(event)

    async def handle_event(self, event: AnalyticsEvent) -> None:
        """Handle a validated analytics event (placeholder implementation)."""

        logger.info(
            "analytics.event.received",
            extra={"event": event.name, "occurred_at": event.occurred_at.isoformat()},
        )
        if self._analytics_service is not None:
            await self._analytics_service.process_event(event)
