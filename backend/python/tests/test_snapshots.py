import pytest
from datetime import datetime, timedelta
from unittest.mock import AsyncMock, Mock
from uuid import uuid4

from app.models.analytics import AnalyticsTimeframe, AnalyticsDashboardResponse, AnalyticsMetrics
from app.models.snapshots import MetricsSnapshot, SnapshotHistoryRequest
from app.services.analytics.snapshots import SnapshotService


@pytest.fixture
def mock_session_factory():
    """Mock session factory for database operations."""
    session = Mock()
    session.execute = AsyncMock()
    session.commit = AsyncMock()
    session.rollback = AsyncMock()
    
    factory = Mock()
    factory.return_value.__aenter__ = AsyncMock(return_value=session)
    factory.return_value.__aexit__ = AsyncMock(return_value=None)
    
    return factory


@pytest.fixture
def mock_analytics_service():
    """Mock analytics service."""
    service = Mock()
    service.get_dashboard_metrics = AsyncMock()
    return service


@pytest.fixture
def snapshot_service(mock_session_factory, mock_analytics_service):
    """Create snapshot service with mocked dependencies."""
    return SnapshotService(mock_session_factory, mock_analytics_service)


@pytest.fixture
def sample_dashboard_response():
    """Sample dashboard response for testing."""
    return AnalyticsDashboardResponse(
        tenant_id="test-tenant",
        metrics=AnalyticsMetrics(
            timeframe=AnalyticsTimeframe.TODAY,
            active_users=100,
            orders=50,
            revenue=2500.0,
            inventory_warnings=5
        )
    )


@pytest.mark.asyncio
async def test_create_snapshot(snapshot_service, mock_analytics_service, sample_dashboard_response):
    """Test creating a new metrics snapshot."""
    
    # Setup mock
    mock_analytics_service.get_dashboard_metrics.return_value = sample_dashboard_response
    
    # Create snapshot
    snapshot = await snapshot_service.create_snapshot(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.TODAY
    )
    
    # Verify snapshot properties
    assert snapshot.tenant_id == "test-tenant"
    assert snapshot.timeframe == AnalyticsTimeframe.TODAY
    assert snapshot.active_users == 100
    assert snapshot.orders == 50
    assert snapshot.revenue == 2500.0
    assert snapshot.inventory_warnings == 5
    assert snapshot.avg_order_value == 50.0  # 2500 / 50
    
    # Verify analytics service was called
    mock_analytics_service.get_dashboard_metrics.assert_called_once_with(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.TODAY,
        location_id=None
    )


@pytest.mark.asyncio
async def test_create_snapshot_with_location(snapshot_service, mock_analytics_service, sample_dashboard_response):
    """Test creating a snapshot with location filter."""
    
    mock_analytics_service.get_dashboard_metrics.return_value = sample_dashboard_response
    
    snapshot = await snapshot_service.create_snapshot(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.TODAY,
        location_id="location-1"
    )
    
    assert snapshot.location_id == "location-1"
    
    mock_analytics_service.get_dashboard_metrics.assert_called_once_with(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.TODAY,
        location_id="location-1"
    )


@pytest.mark.asyncio
async def test_create_snapshot_zero_orders(snapshot_service, mock_analytics_service):
    """Test creating a snapshot when there are zero orders."""
    
    # Setup response with zero orders
    response = AnalyticsDashboardResponse(
        tenant_id="test-tenant",
        metrics=AnalyticsMetrics(
            timeframe=AnalyticsTimeframe.TODAY,
            active_users=10,
            orders=0,
            revenue=0.0,
            inventory_warnings=0
        )
    )
    mock_analytics_service.get_dashboard_metrics.return_value = response
    
    snapshot = await snapshot_service.create_snapshot(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.TODAY
    )
    
    # Should handle division by zero gracefully
    assert snapshot.avg_order_value == 0.0


@pytest.mark.asyncio
async def test_get_snapshot_history(snapshot_service, mock_session_factory):
    """Test retrieving snapshot history."""
    
    # Mock database response
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock snapshot data
    mock_row = Mock()
    mock_row.id = uuid4()
    mock_row.tenant_id = "test-tenant"
    mock_row.location_id = None
    mock_row.snapshot_date = datetime.utcnow()
    mock_row.timeframe = "today"
    mock_row.active_users = 100
    mock_row.orders = 50
    mock_row.revenue = 2500.0
    mock_row.inventory_warnings = 5
    mock_row.avg_order_value = 50.0
    mock_row.new_customers = 10
    mock_row.returning_customers = 40
    mock_row.conversion_rate = 0.25
    mock_row.created_at = datetime.utcnow()
    
    # Mock execute results
    mock_result = Mock()
    mock_result.fetchall.return_value = [mock_row]
    mock_session.execute.return_value = mock_result
    
    # Mock count result
    mock_count_result = Mock()
    mock_count_result.one_or_none.return_value = Mock(total=1)
    mock_session.execute.side_effect = [mock_result, mock_count_result]
    
    # Create request
    request = SnapshotHistoryRequest(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.LAST_MONTH,
        limit=10
    )
    
    # Get history
    response = await snapshot_service.get_snapshot_history(request)
    
    # Verify response
    assert response.tenant_id == "test-tenant"
    assert response.timeframe == AnalyticsTimeframe.LAST_MONTH
    assert len(response.snapshots) == 1
    assert response.total_count == 1
    assert len(response.insights) > 0  # Should have some insights


@pytest.mark.asyncio
async def test_backfill_snapshots(snapshot_service, mock_session_factory):
    """Test backfilling historical snapshots."""
    
    # Mock session
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock that no snapshots exist (so all will be created)
    mock_count_result = Mock()
    mock_count_result.one_or_none.return_value = Mock(count=0)
    mock_session.execute.return_value = mock_count_result
    
    # Backfill 5 days
    snapshots_created = await snapshot_service.backfill_snapshots(
        tenant_id="test-tenant",
        days_back=5
    )
    
    # Should create 5 snapshots
    assert snapshots_created == 5


def test_calculate_trend_increasing(snapshot_service):
    """Test trend calculation for increasing values."""
    
    values = [100.0, 90.0, 80.0, 70.0]  # Increasing (newest first)
    trend = snapshot_service._calculate_trend(values)
    
    assert trend.direction == "increasing"
    assert trend.change_percentage > 0
    assert trend.current_value == 100.0
    assert trend.previous_value == 70.0


def test_calculate_trend_decreasing(snapshot_service):
    """Test trend calculation for decreasing values."""
    
    values = [70.0, 80.0, 90.0, 100.0]  # Decreasing (newest first)
    trend = snapshot_service._calculate_trend(values)
    
    assert trend.direction == "decreasing"
    assert trend.change_percentage < 0
    assert trend.current_value == 70.0
    assert trend.previous_value == 100.0


def test_calculate_trend_stable(snapshot_service):
    """Test trend calculation for stable values."""
    
    values = [100.0, 100.5, 99.5, 100.0]  # Stable (within 1% threshold)
    trend = snapshot_service._calculate_trend(values)
    
    assert trend.direction == "stable"
    assert abs(trend.change_percentage) <= 1.0


def test_calculate_trend_insufficient_data(snapshot_service):
    """Test trend calculation with insufficient data."""
    
    values = [100.0]  # Only one value
    trend = snapshot_service._calculate_trend(values)
    
    assert trend.direction == "stable"
    assert trend.change_percentage == 0.0
    assert trend.confidence == 0.0


@pytest.mark.asyncio
async def test_generate_insights_insufficient_data(snapshot_service):
    """Test insight generation with insufficient data."""
    
    snapshots = [MetricsSnapshot(
        tenant_id="test", 
        snapshot_date=datetime.utcnow(),
        timeframe=AnalyticsTimeframe.TODAY
    )]
    insights = await snapshot_service._generate_insights(snapshots)
    
    assert len(insights) == 1
    assert "Insufficient historical data" in insights[0]


@pytest.mark.asyncio
async def test_generate_insights_with_trends(snapshot_service):
    """Test insight generation with trend data."""
    
    # Create snapshots with increasing revenue trend
    # Note: snapshots are ordered newest first, so we need to reverse the revenue calculation
    base_date = datetime.utcnow()
    snapshots = [
        MetricsSnapshot(
            tenant_id="test",
            snapshot_date=base_date - timedelta(days=i),
            timeframe=AnalyticsTimeframe.TODAY,
            revenue=1000.0 + ((4-i) * 100),  # Increasing revenue (newest has highest)
            orders=10 + (4-i),
            avg_order_value=100.0,
            inventory_warnings=max(0, 5 - (4-i))
        )
        for i in range(5)
    ]
    
    insights = await snapshot_service._generate_insights(snapshots)
    
    # Should have insights about revenue trend
    assert len(insights) > 0
    # Check for either "upward" or "increasing" since the logic might use different wording
    assert any("upward" in insight or "increasing" in insight for insight in insights)