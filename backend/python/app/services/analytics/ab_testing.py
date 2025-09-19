"""A/B testing analytics service."""

from __future__ import annotations

import json
import math
import uuid
from dataclasses import dataclass
from datetime import datetime
from typing import Dict, Iterable, List, Optional

from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker
from sqlalchemy.exc import SQLAlchemyError

from app.core.logging import logger
from app.models.experiments import (
    ConversionUpdate,
    Experiment,
    ExperimentDefinition,
    ExperimentDetail,
    ExperimentResults,
    ExperimentStatus,
    ExperimentSuccessMetric,
    ExperimentSummary,
    ExperimentVariant,
    ParticipantAssignment,
    ParticipantRecord,
    VariantComparison,
    VariantResult,
)

_SIGNIFICANCE_THRESHOLD = 0.95


@dataclass
class _VariantAggregate:
    """Internal representation of variant metrics."""

    name: str
    participants: int = 0
    conversions: int = 0
    total_value: float = 0.0

    @property
    def conversion_rate(self) -> float:
        return (self.conversions / self.participants) if self.participants else 0.0

    @property
    def average_value(self) -> float:
        return (self.total_value / self.conversions) if self.conversions else 0.0


class ABTestingService:
    """Provides experiment lifecycle management and analytics."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]) -> None:
        self._session_factory = session_factory

    async def create_experiment(self, definition: ExperimentDefinition) -> Experiment:
        """Persist a new experiment definition."""

        experiment_id = uuid.uuid4()
        variants_payload = [variant.model_dump() for variant in definition.variants]
        allocation_payload = {
            variant.name: round(variant.allocation, 4) for variant in definition.variants
        }
        metrics_payload = [metric.model_dump() for metric in definition.success_metrics]

        query = text(
            """
            INSERT INTO analytics.experiments (
                id,
                tenant_id,
                name,
                description,
                hypothesis,
                variants,
                traffic_allocation,
                success_metrics,
                start_date,
                end_date,
                status,
                created_by
            )
            VALUES (
                :id,
                :tenant_id,
                :name,
                :description,
                :hypothesis,
                :variants::jsonb,
                :allocation::jsonb,
                :metrics::jsonb,
                :start_date,
                :end_date,
                :status,
                :created_by
            )
            RETURNING id, tenant_id, name, description, hypothesis,
                      variants, traffic_allocation, success_metrics,
                      start_date, end_date, status, created_by,
                      created_at, updated_at, results
            """
        )

        params = {
            "id": experiment_id,
            "tenant_id": uuid.UUID(definition.tenant_id),
            "name": definition.name,
            "description": definition.description,
            "hypothesis": definition.hypothesis,
            "variants": json.dumps(variants_payload),
            "allocation": json.dumps(allocation_payload),
            "metrics": json.dumps(metrics_payload),
            "start_date": definition.start_date,
            "end_date": definition.end_date,
            "status": definition.status.value,
            "created_by": uuid.UUID(definition.created_by),
        }

        async with self._session_factory() as session:
            result = await session.execute(query, params)
            await session.commit()
            row = result.one()

        experiment = self._row_to_experiment(row)
        logger.info(
            "analytics.experiments.created",
            extra={"experiment": experiment.id, "tenant": experiment.tenant_id},
        )
        return experiment

    async def list_experiments(self, tenant_id: str) -> List[ExperimentSummary]:
        """Return summaries for a tenant's experiments."""

        query = text(
            """
            SELECT e.id,
                   e.name,
                   e.status,
                   e.start_date,
                   e.end_date,
                   e.results,
                   e.created_at,
                   COALESCE(SUM(CASE WHEN p.converted_at IS NOT NULL THEN 1 ELSE 0 END), 0) AS conversions
            FROM analytics.experiments e
            LEFT JOIN analytics.experiment_participants p ON p.experiment_id = e.id
            WHERE e.tenant_id = :tenant_id
            GROUP BY e.id
            ORDER BY e.created_at DESC
            """
        )

        async with self._session_factory() as session:
            result = await session.execute(query, {"tenant_id": uuid.UUID(tenant_id)})
            rows = result.fetchall()

        summaries: List[ExperimentSummary] = []
        for row in rows:
            results_payload = self._deserialize_json(row.results)
            summaries.append(
                ExperimentSummary(
                    id=str(row.id),
                    name=row.name,
                    status=row.status if isinstance(row.status, ExperimentStatus) else ExperimentStatus(row.status),
                    start_date=row.start_date,
                    end_date=row.end_date,
                    winner=results_payload.get("winner") if isinstance(results_payload, dict) else None,
                    conversions=int(row.conversions or 0),
                    created_at=row.created_at,
                )
            )
        return summaries

    async def get_experiment_detail(
        self,
        tenant_id: str,
        experiment_id: str,
    ) -> Optional[ExperimentDetail]:
        """Fetch experiment configuration and computed results."""

        experiment_query = text(
            """
            SELECT id, tenant_id, name, description, hypothesis,
                   variants, traffic_allocation, success_metrics,
                   start_date, end_date, status, created_by,
                   created_at, updated_at, results
            FROM analytics.experiments
            WHERE tenant_id = :tenant_id AND id = :experiment_id
            """
        )

        participants_query = text(
            """
            SELECT variant_name,
                   COUNT(*) AS participants,
                   COUNT(converted_at) AS conversions,
                   COALESCE(SUM(CASE WHEN converted_at IS NOT NULL THEN conversion_value ELSE 0 END), 0) AS total_value
            FROM analytics.experiment_participants
            WHERE experiment_id = :experiment_id
            GROUP BY variant_name
            """
        )

        async with self._session_factory() as session:
            experiment_result = await session.execute(
                experiment_query,
                {
                    "tenant_id": uuid.UUID(tenant_id),
                    "experiment_id": uuid.UUID(experiment_id),
                },
            )
            experiment_row = experiment_result.one_or_none()
            if experiment_row is None:
                return None

            participant_result = await session.execute(
                participants_query,
                {"experiment_id": uuid.UUID(experiment_id)},
            )
            participant_rows = participant_result.fetchall()

        experiment = self._row_to_experiment(experiment_row)
        variant_results = self._build_variant_results(experiment.variants, participant_rows)
        comparisons = self._build_comparisons(variant_results)

        baseline = variant_results[0].name if variant_results else experiment.variants[0].name
        suggested = next(
            (cmp.variant for cmp in comparisons if cmp.is_significant and cmp.lift and cmp.lift > 0),
            None,
        )
        overall_confidence = max((cmp.confidence or 0.0 for cmp in comparisons), default=None)
        if overall_confidence == 0.0:
            overall_confidence = None

        results = ExperimentResults(
            baseline_variant=baseline,
            variants=variant_results,
            comparisons=comparisons,
            suggested_winner=suggested,
            overall_confidence=overall_confidence,
        )

        return ExperimentDetail(experiment=experiment, results=results)

    async def record_assignment(
        self,
        assignment: ParticipantAssignment,
    ) -> ParticipantRecord:
        """Record or update a participant assignment."""

        params = {
            "id": uuid.uuid4(),
            "experiment_id": uuid.UUID(assignment.experiment_id),
            "user_id": uuid.UUID(assignment.user_id) if assignment.user_id else None,
            "customer_id": uuid.UUID(assignment.customer_id) if assignment.customer_id else None,
            "session_id": assignment.session_id,
            "variant_name": assignment.variant_name,
            "assigned_at": assignment.assigned_at,
        }

        query = text(
            """
            INSERT INTO analytics.experiment_participants (
                id,
                experiment_id,
                user_id,
                customer_id,
                session_id,
                variant_name,
                assigned_at
            ) VALUES (
                :id,
                :experiment_id,
                :user_id,
                :customer_id,
                :session_id,
                :variant_name,
                :assigned_at
            )
            ON CONFLICT ON CONSTRAINT unique_participant
            DO UPDATE SET
                variant_name = EXCLUDED.variant_name,
                assigned_at = EXCLUDED.assigned_at
            RETURNING id, experiment_id, user_id, customer_id, session_id,
                      variant_name, assigned_at, converted_at, conversion_value
            """
        )

        async with self._session_factory() as session:
            try:
                result = await session.execute(query, params)
                await session.commit()
            except SQLAlchemyError as exc:  # pragma: no cover - handled by API layer
                await session.rollback()
                logger.error(
                    "analytics.experiments.assignment_failed",
                    extra={"experiment": assignment.experiment_id, "error": str(exc)},
                )
                raise

        row = result.one()
        return self._row_to_participant(row)

    async def record_conversion(self, update: ConversionUpdate) -> ParticipantRecord:
        """Mark a participant as converted with optional value."""

        query = text(
            """
            UPDATE analytics.experiment_participants
            SET converted_at = :converted_at,
                conversion_value = :conversion_value
            WHERE id = :participant_id
            RETURNING id, experiment_id, user_id, customer_id, session_id,
                      variant_name, assigned_at, converted_at, conversion_value
            """
        )

        params = {
            "participant_id": uuid.UUID(update.participant_id),
            "converted_at": update.converted_at,
            "conversion_value": update.conversion_value,
        }

        async with self._session_factory() as session:
            result = await session.execute(query, params)
            await session.commit()
            row = result.one()

        record = self._row_to_participant(row)
        logger.info(
            "analytics.experiments.conversion_recorded",
            extra={"participant": record.id, "experiment": record.experiment_id},
        )
        return record

    def _row_to_participant(self, row) -> ParticipantRecord:
        return ParticipantRecord(
            id=str(row.id),
            experiment_id=str(row.experiment_id),
            user_id=str(row.user_id) if row.user_id else None,
            customer_id=str(row.customer_id) if row.customer_id else None,
            session_id=row.session_id,
            variant_name=row.variant_name,
            assigned_at=row.assigned_at,
            converted_at=row.converted_at,
            conversion_value=float(row.conversion_value) if row.conversion_value is not None else None,
        )

    def _row_to_experiment(self, row) -> Experiment:
        variants_payload = self._deserialize_json(row.variants) or []
        metrics_payload = self._deserialize_json(row.success_metrics) or []
        allocation_payload = self._deserialize_json(row.traffic_allocation) or {}
        results_payload = self._deserialize_json(row.results)

        variants = [ExperimentVariant(**variant) for variant in variants_payload]

        if not allocation_payload:
            allocation_payload = {variant.name: variant.allocation for variant in variants}

        return Experiment(
            id=str(row.id),
            tenant_id=str(row.tenant_id),
            name=row.name,
            description=row.description,
            hypothesis=row.hypothesis,
            status=row.status if isinstance(row.status, ExperimentStatus) else ExperimentStatus(row.status),
            variants=variants,
            success_metrics=[
                ExperimentSuccessMetric(**metric) for metric in metrics_payload
            ],
            traffic_allocation={str(k): float(v) for k, v in allocation_payload.items()},
            start_date=row.start_date,
            end_date=row.end_date,
            created_by=str(row.created_by),
            created_at=row.created_at,
            updated_at=row.updated_at,
            results=results_payload if isinstance(results_payload, dict) else {},
        )

    def _build_variant_results(
        self,
        variants: List[ExperimentVariant],
        participant_rows: Iterable,
    ) -> List[VariantResult]:
        aggregates: Dict[str, _VariantAggregate] = {
            variant.name: _VariantAggregate(name=variant.name) for variant in variants
        }

        for row in participant_rows:
            aggregate = aggregates.get(row.variant_name)
            if not aggregate:
                aggregates[row.variant_name] = aggregate = _VariantAggregate(name=row.variant_name)
            aggregate.participants = int(row.participants or 0)
            aggregate.conversions = int(row.conversions or 0)
            aggregate.total_value = float(row.total_value or 0.0)

        ordered_results: List[VariantResult] = []
        for variant in variants:
            aggregate = aggregates[variant.name]
            ordered_results.append(
                VariantResult(
                    name=aggregate.name,
                    participants=aggregate.participants,
                    conversions=aggregate.conversions,
                    conversion_rate=aggregate.conversion_rate,
                    total_conversion_value=aggregate.total_value,
                    avg_conversion_value=aggregate.average_value,
                    lift=None,
                )
            )

        baseline_rate = ordered_results[0].conversion_rate if ordered_results else 0.0
        for result in ordered_results:
            if result.name == ordered_results[0].name:
                result.lift = None
                continue
            if baseline_rate > 0:
                result.lift = (result.conversion_rate - baseline_rate) / baseline_rate
            elif result.conversion_rate > 0:
                result.lift = float("inf")
            else:
                result.lift = 0.0

        return ordered_results

    def _build_comparisons(self, results: List[VariantResult]) -> List[VariantComparison]:
        if not results:
            return []
        baseline = results[0]
        comparisons: List[VariantComparison] = []
        for variant in results[1:]:
            p_value = self._proportion_z_test(
                baseline.participants,
                baseline.conversions,
                variant.participants,
                variant.conversions,
            )
            confidence = (1 - p_value) if p_value is not None else None
            is_significant = bool(confidence and confidence >= _SIGNIFICANCE_THRESHOLD)
            comparisons.append(
                VariantComparison(
                    baseline=baseline.name,
                    variant=variant.name,
                    lift=variant.lift,
                    p_value=p_value,
                    confidence=confidence,
                    is_significant=is_significant,
                )
            )
        return comparisons

    def _proportion_z_test(
        self,
        participants_a: int,
        conversions_a: int,
        participants_b: int,
        conversions_b: int,
    ) -> Optional[float]:
        if participants_a == 0 or participants_b == 0:
            return None
        p_a = conversions_a / participants_a
        p_b = conversions_b / participants_b
        pooled = (conversions_a + conversions_b) / (participants_a + participants_b)
        if pooled in (0.0, 1.0):
            return None
        std_error = math.sqrt(pooled * (1 - pooled) * ((1 / participants_a) + (1 / participants_b)))
        if std_error == 0:
            return None
        z_score = (p_a - p_b) / std_error
        p_value = 2 * (1 - self._normal_cdf(abs(z_score)))
        return max(min(p_value, 1.0), 0.0)

    @staticmethod
    def _normal_cdf(value: float) -> float:
        return 0.5 * (1 + math.erf(value / math.sqrt(2)))

    @staticmethod
    def _deserialize_json(payload) -> Dict | List | None:
        if payload is None:
            return None
        if isinstance(payload, (bytes, bytearray, memoryview)):
            payload = bytes(payload).decode()
        if isinstance(payload, str):
            payload = payload.strip()
            if not payload:
                return None
            return json.loads(payload)
        return payload
