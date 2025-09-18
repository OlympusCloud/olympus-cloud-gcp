from fastapi import APIRouter


api_router = APIRouter()


@api_router.get("/health", tags=["monitoring"])
async def health_check() -> dict[str, str]:
    """Report basic service health."""

    return {"status": "ok"}
