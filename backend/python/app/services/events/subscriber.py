from __future__ import annotations

import asyncio
from contextlib import asynccontextmanager
from typing import Awaitable, Callable, Iterable, Optional

from redis.asyncio import Redis

from app.core.logging import logger

EventHandler = Callable[[str, object], Awaitable[None]]


class EventSubscriber:
    """Background subscriber for Redis pub/sub analytics channels."""

    def __init__(
        self,
        redis: Redis,
        channels: Iterable[str],
        handler: EventHandler,
        poll_interval: float = 0.5,
    ) -> None:
        self._redis = redis
        self._channels = list(channels)
        self._handler = handler
        self._poll_interval = poll_interval
        self._task: Optional[asyncio.Task] = None
        self._stop_event = asyncio.Event()

    @property
    def running(self) -> bool:
        return self._task is not None and not self._task.done()

    async def start(self) -> None:
        """Start the subscriber background task."""

        if self.running:
            return
        self._stop_event.clear()
        self._task = asyncio.create_task(self._listen(), name="redis-event-subscriber")

    async def stop(self) -> None:
        """Stop the subscriber background task and release resources."""

        if not self._task:
            return
        self._stop_event.set()
        self._task.cancel()
        try:
            await self._task
        except asyncio.CancelledError:
            pass
        finally:
            self._task = None

    async def _listen(self) -> None:
        try:
            async with self._pubsub() as pubsub:
                await pubsub.psubscribe(*self._channels)
                logger.info("analytics.redis.subscribed", extra={"channels": self._channels})

                while not self._stop_event.is_set():
                    message = await pubsub.get_message(
                        ignore_subscribe_messages=True,
                        timeout=self._poll_interval,
                    )
                    if message is None:
                        continue

                    channel = message.get("channel", "")
                    data = message.get("data")
                    try:
                        await self._handler(str(channel), data)
                    except Exception as exc:  # noqa: BLE001
                        logger.exception(
                            "analytics.redis.handler_error", extra={"error": str(exc), "channel": channel}
                        )
        except Exception as exc:  # noqa: BLE001
            logger.exception("analytics.redis.connection_error", extra={"error": str(exc)})
        finally:
            logger.info("analytics.redis.unsubscribed", extra={"channels": self._channels})

    @asynccontextmanager
    async def _pubsub(self):
        pubsub = self._redis.pubsub()
        try:
            yield pubsub
        finally:
            await pubsub.aclose()
