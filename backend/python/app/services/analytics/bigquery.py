from __future__ import annotations

from datetime import datetime
from typing import Any, Optional

try:
    from google.cloud import bigquery
except ImportError:  # pragma: no cover - optional dependency during local dev
    bigquery = None  # type: ignore

from app.core.logging import logger
from app.core.settings import Settings, get_settings
from app.models.events import AnalyticsEvent


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

    def record_event(self, event: AnalyticsEvent) -> None:
        """Persist a domain analytics event to BigQuery."""

        if bigquery is None:
            logger.info(
                "analytics.bigquery.event_skipped",
                extra={"reason": "dependency not installed", "event": event.name},
            )
            return

        payload = {
            "name": event.name,
            "occurred_at": event.occurred_at.isoformat(),
            "ingested_at": datetime.utcnow().isoformat(),
            "context": {
                "request_id": event.context.request_id,
                "source": event.context.source,
            },
            "payload": event.payload,
        }

        tenant_id = event.payload.get("tenant_id")
        if tenant_id:
            payload["tenant_id"] = tenant_id

        try:
            self.insert_rows("events", [payload])
        except Exception as exc:  # noqa: BLE001
            logger.warning(
                "analytics.bigquery.record_event_failed",
                extra={"error": str(exc), "event": event.name},
            )
