from __future__ import annotations

from app.core.logging import logger


class BigQueryClient:
    """Lightweight wrapper around the BigQuery client SDK."""

    def __init__(self, project_id: str, dataset: str, location: str = "US") -> None:
        self.project_id = project_id
        self.dataset = dataset
        self.location = location
        self._client = None
        self._bigquery_module = None

        try:
            from google.cloud import bigquery  # type: ignore
        except ImportError:
            logger.info(
                "analytics.bigquery.sdk_missing",
                extra={"project_id": project_id, "dataset": dataset},
            )
            return

        self._bigquery_module = bigquery
        try:
            self._client = bigquery.Client(project=project_id)
        except Exception as exc:  # noqa: BLE001
            logger.info(
                "analytics.bigquery.client_unavailable",
                extra={"error": str(exc), "project_id": project_id},
            )
            self._client = None

    @property
    def dataset_id(self) -> str:
        return f"{self.project_id}.{self.dataset}"

    def ensure_dataset(self) -> None:
        """Ensure the configured dataset exists."""

        if self._client is None or self._bigquery_module is None:
            raise RuntimeError("BigQuery client not available")

        dataset_ref = self.dataset_id
        try:
            self._client.get_dataset(dataset_ref)
            return
        except Exception:  # noqa: BLE001
            pass

        dataset = self._bigquery_module.Dataset(dataset_ref)
        dataset.location = self.location

        try:
            self._client.create_dataset(dataset)
            logger.info(
                "analytics.bigquery.dataset_created",
                extra={"dataset": dataset_ref, "location": self.location},
            )
        except Exception as exc:  # noqa: BLE001
            logger.info(
                "analytics.bigquery.dataset_create_failed",
                extra={"dataset": dataset_ref, "error": str(exc)},
            )
            raise RuntimeError("Failed to ensure BigQuery dataset") from exc
