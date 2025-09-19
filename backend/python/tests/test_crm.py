import pytest
from unittest.mock import AsyncMock, Mock

from app.models.crm import Campaign, CampaignType, CampaignStatus, CustomerSegment
from app.services.crm.service import CRMService


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
def crm_service(mock_session_factory):
    """Create CRM service with mocked dependencies."""
    return CRMService(mock_session_factory)


@pytest.mark.asyncio
async def test_segment_customers(crm_service, mock_session_factory):
    """Test customer segmentation functionality."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Mock segmentation query result
    mock_result = Mock()
    mock_result.fetchall.return_value = [
        Mock(segment="vip", count=5),
        Mock(segment="high_value", count=15),
        Mock(segment="regular", count=50),
        Mock(segment="new", count=10),
        Mock(segment="at_risk", count=8)
    ]
    
    mock_session.execute.return_value = mock_result
    
    # Test segmentation
    result = await crm_service.segment_customers("test-tenant")
    
    # Verify results
    assert result.tenant_id == "test-tenant"
    assert result.total_customers == 88
    assert result.segments[CustomerSegment.VIP] == 5
    assert result.segments[CustomerSegment.HIGH_VALUE] == 15
    assert result.segments[CustomerSegment.REGULAR] == 50


@pytest.mark.asyncio
async def test_create_campaign(crm_service, mock_session_factory):
    """Test campaign creation functionality."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    
    # Create test campaign
    campaign = Campaign(
        tenant_id="test-tenant",
        name="Test Campaign",
        type=CampaignType.EMAIL,
        status=CampaignStatus.DRAFT,
        target_segments=[CustomerSegment.VIP, CustomerSegment.HIGH_VALUE],
        message="Special offer for valued customers!"
    )
    
    # Test campaign creation
    result = await crm_service.create_campaign(campaign)
    
    # Verify campaign was created
    assert result.tenant_id == "test-tenant"
    assert result.name == "Test Campaign"
    assert result.type == CampaignType.EMAIL
    assert len(result.target_segments) == 2
    
    # Verify database calls
    assert mock_session.execute.call_count == 2  # CREATE TABLE + INSERT
    mock_session.commit.assert_called_once()


@pytest.mark.asyncio
async def test_segment_customers_error_handling(crm_service, mock_session_factory):
    """Test error handling in customer segmentation."""
    
    mock_session = mock_session_factory.return_value.__aenter__.return_value
    mock_session.execute.side_effect = Exception("Database error")
    
    # Test segmentation with error
    result = await crm_service.segment_customers("test-tenant")
    
    # Should return empty result on error
    assert result.tenant_id == "test-tenant"
    assert result.total_customers == 0
    assert len(result.segments) == 0