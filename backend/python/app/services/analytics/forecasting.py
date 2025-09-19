"""Forecasting utilities for analytics metrics."""

from __future__ import annotations

from datetime import datetime, timedelta
from typing import Dict, List, Sequence, Tuple

import numpy as np
import pandas as pd
from sqlalchemy import text
from sqlalchemy.ext.asyncio import AsyncSession, async_sessionmaker

from app.core.logging import logger
from app.models.enhanced_analytics import ForecastData, TimeSeriesPoint

_ALLOWED_GRANULARITY = {"day", "week", "month"}
_GRANULARITY_TO_FREQ = {"day": "D", "week": "W", "month": "M"}


class ForecastingService:
    """Generate lightweight forecasts for key business metrics."""

    def __init__(self, session_factory: async_sessionmaker[AsyncSession]) -> None:
        self._session_factory = session_factory

    async def revenue_forecast(
        self,
        tenant_id: str,
        *,
        periods: int = 6,
        granularity: str = "month",
        start_date: datetime | None = None,
        end_date: datetime | None = None,
    ) -> ForecastData:
        """Forecast revenue for upcoming periods using a linear trend model."""

        granularity = granularity.lower()
        if granularity not in _ALLOWED_GRANULARITY:
            granularity = "month"

        start_date = start_date or datetime.utcnow() - timedelta(days=365)
        end_date = end_date or datetime.utcnow()

        series = await self._fetch_revenue_series(
            tenant_id,
            granularity=granularity,
            start_date=start_date,
            end_date=end_date,
        )

        if len(series) < 2:
            logger.info(
                "analytics.forecast.insufficient_data",
                extra={"tenant": tenant_id, "granularity": granularity},
            )
            return self._empty_forecast(granularity)

        history_index = np.arange(len(series))
        values = series["revenue"].to_numpy(dtype=float)

        slope, intercept = np.polyfit(history_index, values, 1)
        predictions, ci = self._predict_future(
            values=values,
            slope=slope,
            intercept=intercept,
            periods=periods,
        )

        future_points = self._build_future_points(
            series,
            predictions,
            granularity=granularity,
        )

        accuracy = self._coefficient_of_determination(values, slope, intercept)

        return ForecastData(
            metric_name="revenue",
            forecast_points=future_points,
            confidence_interval={
                "upper": [round(val, 2) for val in ci["upper"]],
                "lower": [round(val, 2) for val in ci["lower"]],
            },
            accuracy_score=round(accuracy, 4),
            model_used="linear_trend",
        )

    async def _fetch_revenue_series(
        self,
        tenant_id: str,
        *,
        granularity: str,
        start_date: datetime,
        end_date: datetime,
    ) -> pd.DataFrame:
        freq = _GRANULARITY_TO_FREQ[granularity]
        trunc_unit = {"day": "day", "week": "week", "month": "month"}[granularity]

        query = text(
            f"""
            SELECT date_trunc('{trunc_unit}', created_at)::date AS bucket,
                   COALESCE(SUM(total_amount), 0)::numeric AS revenue
            FROM commerce.orders
            WHERE tenant_id = :tenant_id
              AND status NOT IN ('cancelled', 'refunded')
              AND created_at >= :start_date
              AND created_at <= :end_date
            GROUP BY 1
            ORDER BY 1
            """
        )

        async with self._session_factory() as session:
            result = await session.execute(
                query,
                {
                    "tenant_id": tenant_id,
                    "start_date": start_date,
                    "end_date": end_date,
                },
            )
            rows = result.fetchall()

        if not rows:
            return pd.DataFrame(columns=["bucket", "revenue"])

        data = [
            (getattr(row, "bucket"), float(getattr(row, "revenue", 0.0) or 0.0))
            for row in rows
        ]
        df = pd.DataFrame(data, columns=["bucket", "revenue"])
        df["bucket"] = pd.to_datetime(df["bucket"], utc=True)
        df = df.set_index("bucket").asfreq(freq, fill_value=0.0)
        df["revenue"] = df["revenue"].astype(float)
        return df

    def _predict_future(
        self,
        *,
        values: np.ndarray,
        slope: float,
        intercept: float,
        periods: int,
    ) -> Tuple[np.ndarray, Dict[str, np.ndarray]]:
        history_index = np.arange(len(values))
        predictions_index = np.arange(len(values), len(values) + periods)

        predictions = intercept + slope * predictions_index

        residuals = values - (intercept + slope * history_index)
        residual_std = float(np.std(residuals)) if residuals.size else 0.0
        margin = 1.96 * residual_std

        upper = predictions + margin
        lower = np.maximum(predictions - margin, 0.0)

        return predictions, {"upper": upper, "lower": lower}

    def _build_future_points(
        self,
        history: pd.DataFrame,
        predictions: np.ndarray,
        *,
        granularity: str,
    ) -> List[TimeSeriesPoint]:
        if history.empty:
            return []

        last_index = history.index[-1]
        freq = _GRANULARITY_TO_FREQ[granularity]

        points: List[TimeSeriesPoint] = []
        for step, value in enumerate(predictions, start=1):
            timestamp = (last_index + step * pd.tseries.frequencies.to_offset(freq)).to_pydatetime()
            points.append(
                TimeSeriesPoint(
                    timestamp=timestamp,
                    value=round(float(value), 2),
                    label=f"Forecast {step}",
                )
            )
        return points

    def _coefficient_of_determination(
        self,
        values: np.ndarray,
        slope: float,
        intercept: float,
    ) -> float:
        if values.size < 2:
            return 0.0
        fitted = intercept + slope * np.arange(len(values))
        ss_tot = float(((values - values.mean()) ** 2).sum())
        ss_res = float(((values - fitted) ** 2).sum())
        if ss_tot == 0:
            return 0.0
        return max(0.0, 1 - ss_res / ss_tot)

    def _empty_forecast(self, granularity: str) -> ForecastData:
        return ForecastData(
            metric_name="revenue",
            forecast_points=[],
            confidence_interval={"upper": [], "lower": []},
            accuracy_score=0.0,
            model_used="insufficient_data",
        )
