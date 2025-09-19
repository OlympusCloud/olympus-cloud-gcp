import pytest
from datetime import datetime, timedelta
from unittest.mock import AsyncMock, Mock

from app.models.analytics import AnalyticsTimeframe
from app.models.enhanced_analytics import (
    AnalyticsFilter,
    EnhancedDashboardMetrics,
    DetailedMetric,
    MetricCategory,
    TrendDirection
)
from app.services.analytics.enhanced_service import EnhancedAnalyticsService


@pytest.fixture
def mock_session_factory():
    """Mock session factory for database operations."""
    session = Mock()
    session.execute = AsyncMock()
    session.scalar = Mock(return_value=1000.0)
    
    factory = Mock()
    factory.return_value.__aenter__ = AsyncMock(return_value=session)
    factory.return_value.__aexit__ = AsyncMock(return_value=None)
    
    return factory


@pytest.fixture
def mock_base_analytics_service():
    """Mock base analytics service."""
    service = Mock()
    return service


@pytest.fixture
def enhanced_service(mock_session_factory, mock_base_analytics_service):
    """Create enhanced analytics service with mocked dependencies."""
    return EnhancedAnalyticsService(mock_session_factory, mock_base_analytics_service)


@pytest.fixture
def sample_analytics_filter():
    """Sample analytics filter for testing."""
    return AnalyticsFilter(
        tenant_id="test-tenant",
        timeframe=AnalyticsTimeframe.THIS_MONTH,
        start_date=datetime.utcnow() - timedelta(days=30),
        end_date=datetime.utcnow(),
        include_trends=True
    )


@pytest.mark.asyncio
async def test_get_enhanced_dashboard(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test getting enhanced dashboard metrics."""
    
    # Mock database responses
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock revenue query result
    revenue_result = Mock()
    revenue_result.scalar.return_value = 5000.0
    
    # Mock orders query result  
    orders_result = Mock()
    orders_result.scalar.return_value = 50
    
    # Mock customers query result
    customers_result = Mock()
    customers_result.scalar.return_value = 25
    
    # Mock customer segments result
    segments_result = Mock()
    segments_result.fetchall.return_value = [
        Mock(segment="VIP", customer_count=5, avg_aov=200.0, segment_revenue=1000.0, percentage=20.0),
        Mock(segment="Regular", customer_count=20, avg_aov=100.0, segment_revenue=2000.0, percentage=80.0)
    ]
    
    # Mock products result
    products_result = Mock()
    products_result.fetchall.return_value = [
        Mock(product_id="prod-1", product_name="Product 1", units_sold=10, revenue=500.0),
        Mock(product_id="prod-2", product_name="Product 2", units_sold=8, revenue=400.0)
    ]
    
    # Mock time series result
    time_series_result = Mock()
    time_series_result.fetchall.return_value = [
        Mock(date=datetime.utcnow().date(), revenue=100.0),
        Mock(date=(datetime.utcnow() - timedelta(days=1)).date(), revenue=150.0)
    ]
    
    # Configure mock to return different results for different queries
    mock_session.execute.side_effect = [
        revenue_result, revenue_result,  # Current and previous revenue
        orders_result, orders_result,    # Current and previous orders
        customers_result, customers_result,  # Current and previous customers
        segments_result,  # Customer segments
        products_result,  # Top products
        Mock(fetchall=Mock(return_value=[])),  # Revenue by category
        Mock(fetchall=Mock(return_value=[])),  # Revenue by payment method
        Mock(fetchall=Mock(return_value=[])),  # Location performance
        Mock(scalar=Mock(return_value=2.5)),   # Avg fulfillment time
        Mock(scalar=Mock(return_value=0)),     # Low stock count
        time_series_result, time_series_result  # Time series data
    ]
    
    # Get enhanced dashboard
    response = await enhanced_service.get_enhanced_dashboard(sample_analytics_filter)
    
    # Verify response structure
    assert response.dashboard_metrics.tenant_id == "test-tenant"
    assert response.dashboard_metrics.timeframe == AnalyticsTimeframe.THIS_MONTH
    assert len(response.dashboard_metrics.key_metrics) > 0
    assert response.business_health is not None
    assert response.processing_time_ms > 0


@pytest.mark.asyncio
async def test_build_key_metrics(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test building key performance indicators."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock successful revenue trend data
    revenue_result = Mock()
    revenue_result.scalar.side_effect = [5000.0, 4000.0]  # Current and previous
    
    orders_result = Mock()
    orders_result.scalar.side_effect = [50, 40]  # Current and previous
    
    customers_result = Mock()
    customers_result.scalar.side_effect = [25, 20]  # Current and previous
    
    mock_session.execute.side_effect = [
        revenue_result, revenue_result,  # Revenue queries
        orders_result, orders_result,    # Orders queries
        customers_result, customers_result  # Customers queries
    ]
    
    # Build key metrics
    metrics = await enhanced_service._build_key_metrics(mock_session, sample_analytics_filter)
    
    # Verify metrics
    assert len(metrics) >= 3  # Should have revenue, orders, AOV, customers
    
    # Check revenue metric
    revenue_metric = next((m for m in metrics if m.name == "Total Revenue"), None)
    assert revenue_metric is not None
    assert revenue_metric.category == MetricCategory.REVENUE
    assert revenue_metric.value == 5000.0
    assert revenue_metric.trend is not None
    assert revenue_metric.trend.direction == TrendDirection.UP  # 5000 > 4000


@pytest.mark.asyncio
async def test_get_customer_segments(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test customer segmentation analysis."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock customer segments data
    mock_result = Mock()
    mock_result.fetchall.return_value = [
        Mock(
            segment="VIP",
            customer_count=5,
            avg_aov=200.0,
            segment_revenue=1000.0,
            percentage=20.0
        ),
        Mock(
            segment="Regular",
            customer_count=20,
            avg_aov=100.0,
            segment_revenue=2000.0,
            percentage=80.0
        )
    ]
    
    mock_session.execute.return_value = mock_result
    
    # Get customer segments
    segments = await enhanced_service._get_customer_segments(mock_session, sample_analytics_filter)
    
    # Verify segments
    assert len(segments) == 2
    
    vip_segment = next((s for s in segments if s.segment_name == "VIP"), None)
    assert vip_segment is not None
    assert vip_segment.customer_count == 5
    assert vip_segment.avg_order_value == 200.0
    assert vip_segment.percentage_of_total == 20.0


@pytest.mark.asyncio
async def test_get_top_products(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test top products analysis."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock top products data
    mock_result = Mock()
    mock_result.fetchall.return_value = [
        Mock(
            product_id="prod-1",
            product_name="Product 1",
            units_sold=10,
            revenue=500.0
        ),
        Mock(
            product_id="prod-2",
            product_name="Product 2",
            units_sold=8,
            revenue=400.0
        )
    ]
    
    mock_session.execute.return_value = mock_result
    
    # Get top products
    products = await enhanced_service._get_top_products(mock_session, sample_analytics_filter)
    
    # Verify products
    assert len(products) == 2
    
    top_product = products[0]
    assert top_product.product_id == "prod-1"
    assert top_product.product_name == "Product 1"
    assert top_product.units_sold == 10
    assert top_product.revenue == 500.0


@pytest.mark.asyncio
async def test_generate_time_series(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test time series data generation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock time series data
    mock_result = Mock()
    mock_result.fetchall.return_value = [
        Mock(date=datetime.utcnow().date(), revenue=100.0),
        Mock(date=(datetime.utcnow() - timedelta(days=1)).date(), revenue=150.0)
    ]
    
    mock_session.execute.return_value = mock_result
    
    # Generate time series
    time_series = await enhanced_service._generate_time_series(mock_session, sample_analytics_filter)
    
    # Verify time series
    assert len(time_series) >= 1  # Should have at least revenue series
    
    revenue_series = next((ts for ts in time_series if ts.metric_name == "Daily Revenue"), None)
    assert revenue_series is not None
    assert len(revenue_series.data_points) == 2
    assert revenue_series.unit == "USD"


def test_get_period_duration(enhanced_service):
    """Test period duration calculation."""
    
    # Test different timeframes
    assert enhanced_service._get_period_duration(AnalyticsTimeframe.TODAY) == timedelta(days=1)
    assert enhanced_service._get_period_duration(AnalyticsTimeframe.THIS_WEEK) == timedelta(weeks=1)
    assert enhanced_service._get_period_duration(AnalyticsTimeframe.THIS_MONTH) == timedelta(days=30)
    assert enhanced_service._get_period_duration(AnalyticsTimeframe.THIS_YEAR) == timedelta(days=365)


@pytest.mark.asyncio
async def test_calculate_business_health(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test business health score calculation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Calculate business health
    health_score = await enhanced_service._calculate_business_health(mock_session, sample_analytics_filter)
    
    # Verify health score
    assert 0 <= health_score.overall_score <= 100
    assert len(health_score.category_scores) == 5  # All metric categories
    assert isinstance(health_score.strengths, list)
    assert isinstance(health_score.concerns, list)
    assert isinstance(health_score.action_items, list)


@pytest.mark.asyncio
async def test_generate_alerts(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test alert generation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock low stock alert
    mock_result = Mock()
    mock_result.scalar.return_value = 3  # 3 low stock items
    mock_session.execute.return_value = mock_result
    
    # Generate alerts
    alerts = await enhanced_service._generate_alerts(mock_session, sample_analytics_filter)
    
    # Verify alerts
    assert len(alerts) >= 1
    assert "3 items are running low on stock" in alerts


@pytest.mark.asyncio
async def test_generate_recommendations(enhanced_service, mock_session_factory, sample_analytics_filter):
    """Test recommendation generation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Generate recommendations
    recommendations = await enhanced_service._generate_recommendations(mock_session, sample_analytics_filter)
    
    # Verify recommendations
    assert len(recommendations) > 0
    assert all(isinstance(rec, str) for rec in recommendations)