from fastapi.responses import RedirectResponse

from app.core.app import create_app

app = create_app()


@app.get("/", include_in_schema=False)
async def root() -> RedirectResponse:
    """Redirect root requests to the interactive API documentation."""

    return RedirectResponse(url="/docs", status_code=307)
