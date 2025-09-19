from __future__ import annotations

from contextlib import asynccontextmanager
from typing import AsyncIterator, Optional

from fastapi import FastAPI

from app.core.database import create_engine, create_session_factory
from app.core.logging import logger
from app.core.redis import create_redis_client
from app.core.settings import get_settings
from app.core.state import RuntimeState
from app.services.analytics.bigquery import BigQueryClient
from app.services.analytics.processor import EventProcessor
from app.services.analytics.service import AnalyticsService
from app.services.analytics.enhanced_service import EnhancedAnalyticsService
from app.services.analytics.snapshots import SnapshotService
from app.services.crm.service import CRMService
from app.services.events.subscriber import EventSubscriber
from app.services.inventory.service import InventoryService
from app.services.ml.recommendation import RecommendationService
from app.services.nlp.query_service import NaturalLanguageQueryService
from app.services.restaurant.service import RestaurantService
from app.services.retail.service import RetailService

EVENT_CHANNEL_PATTERNS = [
    "events.user.*",
    "events.order.*",
    "events.payment.*",
    "events.inventory.*",
]


@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncIterator[None]:
    """Manage application startup and shutdown hooks."""

    settings = get_settings()
    runtime_state = RuntimeState()
    app.state.runtime = runtime_state

    engine = create_engine(settings)
    session_factory = create_session_factory(engine)
    app.state.db_engine = engine
    app.state.db_session_factory = session_factory

    analytics_service = getattr(app.state, "analytics_service", None)

    if analytics_service is None:
        bigquery_client = BigQueryClient(settings)
        analytics_service = AnalyticsService(session_factory, bigquery_client)
        app.state.analytics_service = analytics_service
        analytics_service.ensure_bigquery_dataset()
    elif hasattr(analytics_service, "ensure_bigquery_dataset"):
        try:
            analytics_service.ensure_bigquery_dataset()
        except RuntimeError as exc:  # pragma: no cover - optional dependency scenario
            logger.info("analytics.bigquery.skipped", extra={"reason": str(exc)})

    if not hasattr(app.state, "recommendation_service"):
        app.state.recommendation_service = RecommendationService(analytics_service)
    
    if not hasattr(app.state, "snapshot_service"):
        app.state.snapshot_service = SnapshotService(session_factory, analytics_service)
    
    if not hasattr(app.state, "enhanced_analytics_service"):
        app.state.enhanced_analytics_service = EnhancedAnalyticsService(session_factory, analytics_service)
    
    if not hasattr(app.state, "crm_service"):
        app.state.crm_service = CRMService(session_factory)
    
    if not hasattr(app.state, "inventory_service"):
        app.state.inventory_service = InventoryService(session_factory)
    
    if not hasattr(app.state, "restaurant_service"):
        app.state.restaurant_service = RestaurantService(session_factory)
    
    if not hasattr(app.state, "retail_service"):
        app.state.retail_service = RetailService(session_factory)

    processor = EventProcessor(analytics_service)
    app.state.event_processor = processor

    if not hasattr(app.state, "nlp_service"):
        app.state.nlp_service = NaturalLanguageQueryService()

    redis = create_redis_client(settings)
    subscriber: Optional[EventSubscriber] = None

    try:
        await redis.ping()
        runtime_state.redis_connected = True
        logger.info("analytics.redis.connected")

        subscriber = EventSubscriber(
            redis=redis,
            channels=EVENT_CHANNEL_PATTERNS,
            handler=processor.handle_raw_event,
        )
        await subscriber.start()
        runtime_state.event_subscriber_running = True
        app.state.event_subscriber = subscriber
        logger.info("analytics.events.subscriber_started")
    except Exception as exc:  # noqa: BLE001
        runtime_state.redis_connected = False
        logger.warning("analytics.redis.unavailable", extra={"error": str(exc)})
        await redis.close()
        subscriber = None
        redis = None

    try:
        yield
    finally:
        if subscriber and subscriber.running:
            await subscriber.stop()
        if redis is not None:
            await redis.close()
        analytics_service = getattr(app.state, "analytics_service", None)
        if analytics_service and hasattr(analytics_service, "ensure_bigquery_dataset"):
            analytics_service.ensure_bigquery_dataset()
        if engine is not None:
            await engine.dispose()
        logger.info("analytics.app.shutdown")
