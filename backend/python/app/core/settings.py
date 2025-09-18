from functools import lru_cache

from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application configuration loaded from environment."""

    app_name: str = "Olympus Cloud Analytics"
    environment: str = "development"
    debug: bool = True

    port: int = 8001
    database_url: str = "postgresql://olympus:devpassword@localhost:5432/olympus"
    redis_url: str = "redis://localhost:6379/0"

    bigquery_project_id: str = "olympus-cloud"
    bigquery_dataset: str = "analytics"

    openai_api_key: str | None = None

    model_config = SettingsConfigDict(env_file=".env", env_file_encoding="utf-8", extra="ignore")


@lru_cache
def get_settings() -> Settings:
    """Return cached settings instance."""

    return Settings()
