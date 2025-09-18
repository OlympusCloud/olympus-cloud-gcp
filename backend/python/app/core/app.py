from fastapi import FastAPI

from app.api.routes import api_router
from app.core.lifespan import lifespan
from app.core.logging import configure_logging
from app.core.settings import get_settings


def create_app() -> FastAPI:
    """Factory for FastAPI application with configuration and routers."""

    settings = get_settings()
    configure_logging("DEBUG" if settings.debug else "INFO")

    app = FastAPI(
        title=settings.app_name,
        version="0.1.0",
        docs_url="/docs",
        redoc_url="/redoc",
        lifespan=lifespan,
    )

    app.include_router(api_router, prefix="/api")

    return app
