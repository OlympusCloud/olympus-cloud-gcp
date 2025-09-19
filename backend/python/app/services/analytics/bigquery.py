from __future__ import annotations

from datetime import date, datetime
from typing import Any, Dict, Optional

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
        """Ensure the analytics dataset and core tables exist."""

        if bigquery is None:
            logger.warning("analytics.bigquery.disabled", extra={"reason": "dependency not installed"})
            return

        dataset_id = f"{self._settings.bigquery_project_id}.{self._settings.bigquery_dataset}"
        dataset = bigquery.Dataset(dataset_id)
        dataset.location = "US"

        client = self.client
        client.create_dataset(dataset, exists_ok=True)

        self._ensure_events_table(client, dataset_id)
        self._ensure_snapshots_table(client, dataset_id)

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

    def record_snapshot(self, snapshot: Dict[str, Any]) -> None:
        """Persist aggregated metrics snapshot to BigQuery."""

        if bigquery is None:
            logger.info(
                "analytics.bigquery.snapshot_skipped",
                extra={"reason": "dependency not installed", "tenant": snapshot.get("tenant_id")},
            )
            return

        serialized: Dict[str, Any] = dict(snapshot)

        captured_at = serialized.get("captured_at")
        if isinstance(captured_at, datetime):
            serialized["captured_at"] = captured_at.isoformat()

        for bound in ("start_date", "end_date"):
            value = serialized.get(bound)
            if isinstance(value, (datetime, date)):
                serialized[bound] = value.isoformat()

        try:
            self.insert_rows("metrics_snapshots", [serialized])
        except Exception as exc:  # noqa: BLE001
            logger.warning(
                "analytics.bigquery.record_snapshot_failed",
                extra={"error": str(exc), "tenant": snapshot.get("tenant_id")},
            )

    def _ensure_events_table(
        self, client: "bigquery.Client", dataset_id: str
    ) -> None:  # type: ignore[name-defined]
        table_id = f"{dataset_id}.events"
        schema = [
            bigquery.SchemaField("name", "STRING", mode="REQUIRED"),
            bigquery.SchemaField("occurred_at", "TIMESTAMP", mode="REQUIRED"),
            bigquery.SchemaField("ingested_at", "TIMESTAMP", mode="REQUIRED"),
            bigquery.SchemaField(
                "context",
                "RECORD",
                mode="NULLABLE",
                fields=[
                    bigquery.SchemaField("request_id", "STRING"),
                    bigquery.SchemaField("source", "STRING"),
                ],
            ),
            bigquery.SchemaField("payload", "JSON", mode="NULLABLE"),
            bigquery.SchemaField("tenant_id", "STRING", mode="NULLABLE"),
        ]

        table = bigquery.Table(table_id, schema=schema)
        table.time_partitioning = bigquery.TimePartitioning(field="occurred_at")
        try:
            client.create_table(table, exists_ok=True)
        except Exception as exc:  # noqa: BLE001
            logger.warning("analytics.bigquery.ensure_events_failed", extra={"error": str(exc)})

    def _ensure_snapshots_table(
        self, client: "bigquery.Client", dataset_id: str
    ) -> None:  # type: ignore[name-defined]
        table_id = f"{dataset_id}.metrics_snapshots"
        schema = [
            bigquery.SchemaField("tenant_id", "STRING", mode="REQUIRED"),
            bigquery.SchemaField("location_id", "STRING", mode="NULLABLE"),
            bigquery.SchemaField("timeframe", "STRING", mode="REQUIRED"),
            bigquery.SchemaField("active_users", "INT64", mode="REQUIRED"),
            bigquery.SchemaField("orders", "INT64", mode="REQUIRED"),
            bigquery.SchemaField("revenue", "NUMERIC", mode="REQUIRED"),
            bigquery.SchemaField("inventory_warnings", "INT64", mode="REQUIRED"),
            bigquery.SchemaField("start_date", "DATE", mode="NULLABLE"),
            bigquery.SchemaField("end_date", "DATE", mode="NULLABLE"),
            bigquery.SchemaField("captured_at", "TIMESTAMP", mode="REQUIRED"),
        ]

        table = bigquery.Table(table_id, schema=schema)
        table.time_partitioning = bigquery.TimePartitioning(field="captured_at")
        try:
            client.create_table(table, exists_ok=True)
        except Exception as exc:  # noqa: BLE001
            logger.warning("analytics.bigquery.ensure_snapshots_failed", extra={"error": str(exc)})
