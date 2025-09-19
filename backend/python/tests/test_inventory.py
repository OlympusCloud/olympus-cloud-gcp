import pytest
from unittest.mock import AsyncMock, Mock

from app.models.inventory import StockMovement, StockStatus
from app.services.inventory.service import InventoryService


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
def inventory_service(mock_session_factory):
    """Create inventory service with mocked dependencies."""
    return InventoryService(mock_session_factory)


@pytest.mark.asyncio
async def test_get_inventory_report(inventory_service, mock_session_factory):
    """Test inventory report generation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock inventory summary query
    summary_result = Mock()
    summary_result.one_or_none.return_value = Mock(
        total_items=100,
        low_stock=5,
        out_of_stock=2,
        reorder_needed=8,
        total_value=50000.0
    )
    
    # Mock alerts query
    alerts_result = Mock()
    alerts_result.fetchall.return_value = [
        Mock(
            product_id="prod-1",
            location_id="loc-1",
            quantity_available=0,
            min_stock_level=10,
            reorder_point=20
        ),
        Mock(
            product_id="prod-2",
            location_id="loc-1",
            quantity_available=5,
            min_stock_level=10,
            reorder_point=20
        )
    ]
    
    # Mock forecasts query
    forecasts_result = Mock()
    forecasts_result.fetchall.return_value = [
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-01", weekly_demand=10),
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-08", weekly_demand=12),
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-15", weekly_demand=8)
    ]
    
    mock_session.execute.side_effect = [
        summary_result,
        alerts_result,
        forecasts_result
    ]
    
    # Test report generation
    report = await inventory_service.get_inventory_report("test-tenant", "test-location")
    
    # Verify report
    assert report.tenant_id == "test-tenant"
    assert report.location_id == "test-location"
    assert report.total_items == 100
    assert report.low_stock_items == 5
    assert report.out_of_stock_items == 2
    assert report.reorder_needed == 8
    assert report.total_value == 50000.0
    assert len(report.alerts) == 2
    assert len(report.forecasts) == 1


@pytest.mark.asyncio
async def test_track_stock_movement(inventory_service, mock_session_factory):
    """Test stock movement tracking."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Create test movement
    movement = StockMovement(
        tenant_id="test-tenant",
        product_id="prod-1",
        location_id="loc-1",
        movement_type="sale",
        quantity=-5,
        reference_id="order-123"
    )
    
    # Test movement tracking
    result = await inventory_service.track_stock_movement(movement)
    
    # Verify movement was tracked
    assert result.tenant_id == "test-tenant"
    assert result.product_id == "prod-1"
    assert result.movement_type == "sale"
    assert result.quantity == -5
    
    # Verify database calls
    assert mock_session.execute.call_count == 2  # CREATE TABLE + INSERT
    mock_session.commit.assert_called_once()


@pytest.mark.asyncio
async def test_stock_alerts_generation(inventory_service, mock_session_factory):
    """Test stock alerts generation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock alerts query result
    mock_result = Mock()
    mock_result.fetchall.return_value = [
        Mock(
            product_id="prod-1",
            location_id="loc-1",
            quantity_available=0,
            min_stock_level=10,
            reorder_point=20
        ),
        Mock(
            product_id="prod-2",
            location_id="loc-1",
            quantity_available=5,
            min_stock_level=10,
            reorder_point=20
        )
    ]
    
    mock_session.execute.return_value = mock_result
    
    # Test alerts generation
    alerts = await inventory_service._get_stock_alerts(mock_session, "test-tenant", "test-location")
    
    # Verify alerts
    assert len(alerts) == 2
    
    # Check out of stock alert
    out_of_stock_alert = next((a for a in alerts if a.alert_type == StockStatus.OUT_OF_STOCK), None)
    assert out_of_stock_alert is not None
    assert out_of_stock_alert.product_id == "prod-1"
    assert out_of_stock_alert.current_stock == 0
    
    # Check low stock alert
    low_stock_alert = next((a for a in alerts if a.alert_type == StockStatus.LOW_STOCK), None)
    assert low_stock_alert is not None
    assert low_stock_alert.product_id == "prod-2"
    assert low_stock_alert.current_stock == 5


@pytest.mark.asyncio
async def test_demand_forecasting(inventory_service, mock_session_factory):
    """Test demand forecasting functionality."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock historical sales data
    mock_result = Mock()
    mock_result.fetchall.return_value = [
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-01", weekly_demand=10),
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-08", weekly_demand=12),
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-15", weekly_demand=8),
        Mock(product_id="prod-1", location_id="loc-1", week="2024-01-22", weekly_demand=11)
    ]
    
    mock_session.execute.return_value = mock_result
    
    # Test forecast generation
    forecasts = await inventory_service._generate_forecasts(mock_session, "test-tenant", "test-location")
    
    # Verify forecasts
    assert len(forecasts) == 1
    
    forecast = forecasts[0]
    assert forecast.tenant_id == "test-tenant"
    assert forecast.product_id == "prod-1"
    assert forecast.location_id == "loc-1"
    assert forecast.predicted_demand > 0
    assert forecast.recommended_stock > forecast.predicted_demand
    assert 0.0 <= forecast.confidence <= 1.0