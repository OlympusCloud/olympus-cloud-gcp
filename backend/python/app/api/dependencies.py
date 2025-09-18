from __future__ import annotations

from typing import Optional

from fastapi import Request
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker, create_async_engine

from app.core.settings import Settings, get_settings
from app.core.state import RuntimeState
from app.services.analytics.bigquery import BigQueryClient
from app.services.analytics.service import AnalyticsService
from app.services.nlp.query_service import NaturalLanguageQueryService


def get_runtime_state(request: Request) -> RuntimeState:
    """Return the shared runtime state for the application."""

    if not hasattr(request.app.state, "runtime") or request.app.state.runtime is None:
        request.app.state.runtime = RuntimeState()
    return request.app.state.runtime


def _get_session_factory(request: Request, settings: Optional[Settings] = None) -> async_sessionmaker[AsyncSession]:
    if hasattr(request.app.state, "session_factory") and request.app.state.session_factory is not None:
        return request.app.state.session_factory

    settings = settings or get_settings()
    engine = create_async_engine(settings.database_url, echo=settings.debug, future=True)
    session_factory = async_sessionmaker(engine, expire_on_commit=False)
    request.app.state.session_factory = session_factory
    return session_factory


def _get_bigquery_client(request: Request, settings: Optional[Settings] = None) -> BigQueryClient:
    if hasattr(request.app.state, "bigquery_client") and request.app.state.bigquery_client is not None:
        return request.app.state.bigquery_client

    settings = settings or get_settings()
    client = BigQueryClient(project_id=settings.bigquery_project_id, dataset=settings.bigquery_dataset)
    request.app.state.bigquery_client = client
    return client


async def get_analytics_service(request: Request) -> AnalyticsService:
    """Provide the analytics service instance from application state."""

    if hasattr(request.app.state, "analytics_service") and request.app.state.analytics_service is not None:
        return request.app.state.analytics_service

    session_factory = _get_session_factory(request)
    bigquery_client = _get_bigquery_client(request)

    service = AnalyticsService(session_factory=session_factory, bigquery_client=bigquery_client)
    request.app.state.analytics_service = service
    return service


def get_nlp_service(request: Request) -> NaturalLanguageQueryService:
    """Return the NLP service, creating a lightweight instance if needed."""

    if hasattr(request.app.state, "nlp_service") and request.app.state.nlp_service is not None:
        return request.app.state.nlp_service

    service = NaturalLanguageQueryService()
    request.app.state.nlp_service = service
    return service
