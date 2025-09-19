import pytest
from datetime import datetime, timedelta
from unittest.mock import AsyncMock, MagicMock
from uuid import uuid4

from app.services.restaurant.service import RestaurantService
from app.models.restaurant import RestaurantAnalytics, RestaurantRecommendation


@pytest.fixture
def mock_session_factory():
    session = AsyncMock()
    session_factory = MagicMock()
    session_factory.return_value.__aenter__.return_value = session
    return session_factory, session


@pytest.fixture
def restaurant_service(mock_session_factory):
    session_factory, _ = mock_session_factory
    return RestaurantService(session_factory)


@pytest.mark.asyncio
async def test_get_table_analytics(restaurant_service, mock_session_factory):
    """Test restaurant table analytics calculation."""
    _, mock_session = mock_session_factory
    
    # Mock database responses
    mock_session.execute.side_effect = [
        # Turnover query result
        MagicMock(fetchone=MagicMock(return_value=(100, 30, 20))),  # 100 orders, 30 days, 20 tables
        # Service query result  
        MagicMock(fetchone=MagicMock(return_value=(45.0, 25.50, 100))),  # 45min avg, $25.50 avg check
        # Popular items query result
        MagicMock(fetchall=MagicMock(return_value=[
            ("Burger", 50, 12.99),
            ("Pizza", 35, 18.99)
        ])),
        # Peak hours query result
        MagicMock(fetchall=MagicMock(return_value=[
            (19, 25, 28.50),  # 7PM, 25 orders, $28.50 avg
            (18, 20, 24.00)   # 6PM, 20 orders, $24.00 avg
        ]))
    ]
    
    tenant_id = str(uuid4())
    location_id = str(uuid4())
    
    result = await restaurant_service.get_table_analytics(tenant_id, location_id)
    
    assert isinstance(result, RestaurantAnalytics)
    assert str(result.tenant_id) == tenant_id
    assert str(result.location_id) == location_id
    assert result.table_turnover_rate == 100 / (30 * 20)  # orders / (days * tables)
    assert result.average_dining_duration == 45.0
    assert result.average_check_size == 25.50
    assert len(result.top_menu_items) == 2
    assert result.top_menu_items[0]["name"] == "Burger"
    assert len(result.peak_hours) == 2
    assert result.peak_hours[0]["hour"] == 19


@pytest.mark.asyncio
async def test_get_table_status(restaurant_service):
    """Test table status distribution."""
    tenant_id = str(uuid4())
    location_id = str(uuid4())
    
    result = await restaurant_service.get_table_status(tenant_id, location_id)
    
    assert isinstance(result, dict)
    assert "available" in result
    assert "occupied" in result
    assert "reserved" in result
    assert "cleaning" in result
    assert "out_of_order" in result
    assert sum(result.values()) > 0


@pytest.mark.asyncio
async def test_get_reservation_metrics(restaurant_service):
    """Test reservation system metrics."""
    tenant_id = str(uuid4())
    location_id = str(uuid4())
    
    result = await restaurant_service.get_reservation_metrics(tenant_id, location_id)
    
    assert isinstance(result, dict)
    assert "total_reservations" in result
    assert "confirmed" in result
    assert "no_shows" in result
    assert "utilization_rate" in result
    assert result["utilization_rate"] <= 1.0


@pytest.mark.asyncio
async def test_generate_restaurant_recommendations(restaurant_service, mock_session_factory):
    """Test restaurant recommendation generation."""
    _, mock_session = mock_session_factory
    
    # Mock analytics data for low turnover scenario
    mock_session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=(30, 30, 20))),  # Low turnover: 1.0
        MagicMock(fetchone=MagicMock(return_value=(75.0, 20.00, 30))),  # Long service time
        MagicMock(fetchall=MagicMock(return_value=[("Pasta", 40, 15.99)])),
        MagicMock(fetchall=MagicMock(return_value=[(20, 30, 25.00)]))  # Peak at 8PM
    ]
    
    tenant_id = str(uuid4())
    
    recommendations = await restaurant_service.generate_restaurant_recommendations(tenant_id)
    
    assert isinstance(recommendations, list)
    assert len(recommendations) > 0
    
    # Should recommend improving table turnover (rate < 2.0)
    turnover_rec = next((r for r in recommendations if "turnover" in r.title.lower()), None)
    assert turnover_rec is not None
    assert turnover_rec.impact == "high"
    
    # Should recommend reducing service time (> 60 minutes)
    service_rec = next((r for r in recommendations if "service time" in r.title.lower()), None)
    assert service_rec is not None
    assert service_rec.impact == "medium"


@pytest.mark.asyncio
async def test_get_kitchen_display_orders(restaurant_service, mock_session_factory):
    """Test kitchen display system orders."""
    _, mock_session = mock_session_factory
    
    # Mock kitchen orders
    order_time = datetime.now() - timedelta(minutes=15)
    mock_result = MagicMock()
    mock_result.fetchall.return_value = [
        (
            uuid4(),  # order_id
            order_time,  # created_at
            "preparing",  # status
            25.50,  # total_amount
            [{"product_name": "Burger", "quantity": 2, "special_instructions": "No onions"}]  # items
        )
    ]
    mock_session.execute.return_value = mock_result
    
    tenant_id = str(uuid4())
    location_id = str(uuid4())
    
    result = await restaurant_service.get_kitchen_display_orders(tenant_id, location_id)
    
    assert isinstance(result, list)
    assert len(result) == 1
    
    order = result[0]
    assert "id" in order
    assert "status" in order
    assert order["status"] == "preparing"
    assert "items" in order
    assert "wait_time_minutes" in order
    assert order["wait_time_minutes"] >= 15  # Should be at least 15 minutes


@pytest.mark.asyncio
async def test_restaurant_analytics_with_no_data(restaurant_service, mock_session_factory):
    """Test analytics when no data is available."""
    _, mock_session = mock_session_factory
    
    # Mock empty results
    mock_session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=(0, 0, 0))),  # No orders
        MagicMock(fetchone=MagicMock(return_value=(None, None, 0))),  # No service data
        MagicMock(fetchall=MagicMock(return_value=[])),  # No popular items
        MagicMock(fetchall=MagicMock(return_value=[]))   # No peak hours
    ]
    
    tenant_id = str(uuid4())
    
    result = await restaurant_service.get_table_analytics(tenant_id)
    
    assert isinstance(result, RestaurantAnalytics)
    assert result.table_turnover_rate == 0.0
    assert result.average_check_size == 0.0
    assert len(result.top_menu_items) == 0
    assert len(result.peak_hours) == 0
    # Should still have default values for other metrics
    assert result.order_accuracy_rate == 0.95
    assert result.kitchen_efficiency == 0.85


@pytest.mark.asyncio
async def test_restaurant_recommendations_high_performance(restaurant_service, mock_session_factory):
    """Test recommendations for high-performing restaurant."""
    _, mock_session = mock_session_factory
    
    # Mock high-performance analytics - should result in turnover rate of 2.5 (150/(30*20))
    mock_session.execute.side_effect = [
        MagicMock(fetchone=MagicMock(return_value=(1500, 30, 20))),  # Very high turnover: 2.5
        MagicMock(fetchone=MagicMock(return_value=(35.0, 45.00, 150))),  # Fast service, high check
        MagicMock(fetchall=MagicMock(return_value=[("Steak", 60, 35.99)])),
        MagicMock(fetchall=MagicMock(return_value=[(19, 40, 50.00)]))
    ]
    
    tenant_id = str(uuid4())
    
    recommendations = await restaurant_service.generate_restaurant_recommendations(tenant_id)
    
    # Should still get menu and staffing recommendations
    assert len(recommendations) >= 2
    
    # Should not recommend turnover improvement (rate >= 2.0)
    turnover_rec = next((r for r in recommendations if "turnover" in r.title.lower()), None)
    assert turnover_rec is None
    
    # Should not recommend service time reduction (< 60 minutes)
    service_rec = next((r for r in recommendations if "service time" in r.title.lower()), None)
    assert service_rec is None