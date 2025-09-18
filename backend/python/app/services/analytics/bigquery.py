from __future__ import annotations

from typing import Any, Optional

try:
    from google.cloud import bigquery
except ImportError:  # pragma: no cover - optional dependency during local dev
    bigquery = None  # type: ignore

from app.core.logging import logger
from app.core.settings import Settings, get_settings


class BigQueryClient:
    """Lazy BigQuery client wrapper with safe defaults for local development."""

    def __init__(self, settings: Optional[Settings] = None) -> None:
        self._settings = settings or get_settings()
        self._client: Optional["bigquery.Client"] = None

    @property
    def client(self) -> "bigquery.Client":  # type: ignore[name-defined]
        if bigquery is None:
            raise RuntimeError("google-cloud-bigquery is not installed in this environment")

        if self._client is None:
            logger.info(
                "analytics.bigquery.create_client",
                extra={"project": self._settings.bigquery_project_id},
            )
            self._client = bigquery.Client(project=self._settings.bigquery_project_id)
        return self._client

    def ensure_dataset(self) -> None:
        """Ensure the analytics dataset exists (no-op if already created)."""

        if bigquery is None:
            logger.warning("analytics.bigquery.disabled", extra={"reason": "dependency not installed"})
            return

        dataset_id = f"{self._settings.bigquery_project_id}.{self._settings.bigquery_dataset}"
        dataset = bigquery.Dataset(dataset_id)
        dataset.location = "US"
        self.client.create_dataset(dataset, exists_ok=True)

    def insert_rows(self, table: str, rows: list[dict[str, Any]]) -> None:
        """Insert rows into the specified BigQuery table."""

        if bigquery is None:
            logger.warning("analytics.bigquery.disabled", extra={"reason": "dependency not installed"})
            return

        if not rows:
            return

        table_id = f"{self._settings.bigquery_project_id}.{self._settings.bigquery_dataset}.{table}"
        errors = self.client.insert_rows_json(table_id, rows)
        if errors:
            logger.error("analytics.bigquery.insert_failed", extra={"errors": errors})
