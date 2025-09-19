"""Tests for enhanced NLP service."""

import pytest
from unittest.mock import MagicMock

from app.api.nlp_routes import NLPQuery, NLPResponse, get_suggestions, process_query
from app.services.nlp.enhanced_nlp import (
    EnhancedNLPService,
    EntityExtractor,
    IntentClassifier,
    ContextManager,
    ResponseGenerator,
    Intent
)


class TestEntityExtractor:
    """Test entity extraction functionality."""
    
    def test_extract_date_entities(self):
        extractor = EntityExtractor()
        
        text = "Show me sales from yesterday and next week"
        entities = extractor.extract(text)
        
        assert 'date' in entities
        assert len(entities['date']) >= 1
        assert any('yesterday' in match['value'] for match in entities['date'])
    
    def test_extract_money_entities(self):
        extractor = EntityExtractor()
        
        text = "Orders over $500 and 1000 dollars"
        entities = extractor.extract(text)
        
        assert 'money' in entities
        assert len(entities['money']) >= 1
    
    def test_extract_quantity_entities(self):
        extractor = EntityExtractor()
        
        text = "We sold 50 items and 100 products"
        entities = extractor.extract(text)
        
        assert 'quantity' in entities
        assert len(entities['quantity']) >= 1


class TestIntentClassifier:
    """Test intent classification functionality."""
    
    def test_classify_analytics_intent(self):
        classifier = IntentClassifier()
        
        intent = classifier.classify("show me analytics dashboard")
        
        assert intent.name == 'view_analytics'
        assert intent.confidence > 0.5
    
    def test_classify_order_intent(self):
        classifier = IntentClassifier()
        
        intent = classifier.classify("create a new order for customer")
        
        assert intent.name == 'create_order'
        assert intent.confidence > 0.5
    
    def test_classify_inventory_intent(self):
        classifier = IntentClassifier()
        
        intent = classifier.classify("check inventory levels")
        
        assert intent.name == 'check_inventory'
        assert intent.confidence > 0.5
    
    def test_classify_unknown_intent(self):
        classifier = IntentClassifier()
        
        intent = classifier.classify("random gibberish text")
        
        assert intent.name == 'unknown'
        assert intent.confidence < 0.5


class TestContextManager:
    """Test context management functionality."""
    
    def test_get_empty_context(self):
        manager = ContextManager()
        
        context = manager.get_context("session1")
        
        assert context == {}
    
    def test_update_context(self):
        manager = ContextManager()
        
        manager.update_context("session1", {"key": "value"})
        context = manager.get_context("session1")
        
        assert context["key"] == "value"
    
    def test_clear_context(self):
        manager = ContextManager()
        
        manager.update_context("session1", {"key": "value"})
        manager.clear_context("session1")
        context = manager.get_context("session1")
        
        assert context == {}


class TestResponseGenerator:
    """Test response generation functionality."""
    
    def test_generate_analytics_response(self):
        generator = ResponseGenerator()
        intent = Intent("view_analytics", 0.9)
        
        response = generator.generate(intent, data="test data")
        
        assert "analytics" in response.lower()
        assert "test data" in response
    
    def test_generate_help_response(self):
        generator = ResponseGenerator()
        intent = Intent("help", 0.9)
        
        response = generator.generate(intent)
        
        assert "help" in response.lower() or "assist" in response.lower()
    
    def test_generate_unknown_response(self):
        generator = ResponseGenerator()
        intent = Intent("unknown", 0.1)
        
        response = generator.generate(intent)
        
        assert "not sure" in response.lower() or "understand" in response.lower()


class TestEnhancedNLPService:
    """Test the main enhanced NLP service."""
    
    @pytest.fixture
    def nlp_service(self):
        return EnhancedNLPService()
    
    def test_process_analytics_query(self, nlp_service):
        result = nlp_service.process_query(
            "show me dashboard analytics",
            session_id="test_session",
            tenant_id="test_tenant"
        )
        
        assert result['intent']['name'] == 'view_analytics'
        assert result['intent']['confidence'] > 0.5
        assert result['session_id'] == 'test_session'
        assert 'processed_at' in result
    
    def test_process_order_query(self, nlp_service):
        result = nlp_service.process_query(
            "create new order for customer",
            session_id="test_session"
        )
        
        assert result['intent']['name'] == 'create_order'
        assert result['intent']['confidence'] > 0.5
    
    def test_process_inventory_query(self, nlp_service):
        result = nlp_service.process_query(
            "check stock levels for products",
            session_id="test_session"
        )
        
        assert result['intent']['name'] == 'check_inventory'
        assert result['intent']['confidence'] > 0.5
    
    def test_get_suggestions(self, nlp_service):
        suggestions = nlp_service.get_suggestions("show me")
        
        assert len(suggestions) > 0
        assert any("analytics" in suggestion for suggestion in suggestions)
    
    def test_train_intent(self, nlp_service):
        result = nlp_service.train_intent(
            "display metrics",
            "view_analytics",
            "test_session"
        )
        
        assert result['status'] == 'training_data_recorded'
    
    def test_get_analytics(self, nlp_service):
        # First process a query to create context
        nlp_service.process_query("test query", "test_session")
        
        analytics = nlp_service.get_analytics("test_session")
        
        assert analytics['session_id'] == 'test_session'
        assert 'last_intent' in analytics
        assert 'last_query_time' in analytics
    
    def test_context_persistence(self, nlp_service):
        # Process first query
        result1 = nlp_service.process_query("show analytics", "session1")
        
        # Process second query in same session
        result2 = nlp_service.process_query("create order", "session1")
        
        # Context should contain information from both queries
        assert result2['context']['last_intent'] == 'create_order'
        assert 'timestamp' in result2['context']
    
    def test_error_handling(self, nlp_service):
        # Mock an error in intent classification
        nlp_service.intent_classifier.classify = MagicMock(side_effect=Exception("Test error"))
        
        result = nlp_service.process_query("test query")
        
        assert result['intent']['name'] == 'error'
        assert 'error' in result
        assert "error processing" in result['response'].lower()


@pytest.mark.asyncio
class TestNLPAPIIntegration:
    """Test NLP API integration."""
    
    def test_nlp_service_initialization(self):
        service = EnhancedNLPService()
        
        assert service.entity_extractor is not None
        assert service.intent_classifier is not None
        assert service.context_manager is not None
        assert service.response_generator is not None
    
    def test_multiple_sessions(self):
        service = EnhancedNLPService()
        
        # Process queries in different sessions
        result1 = service.process_query("analytics", "session1")
        result2 = service.process_query("inventory", "session2")
        
        # Sessions should be independent
        assert result1['session_id'] != result2['session_id']
        assert result1['intent']['name'] != result2['intent']['name']
    
    def test_entity_and_intent_combination(self):
        service = EnhancedNLPService()
        
        result = service.process_query(
            "show me sales analytics for yesterday with revenue over $1000"
        )
        
        assert result['intent']['name'] == 'view_analytics'
        assert len(result['entities']) > 0
        # Should extract date and money entities
        assert any(entity_type in ['date', 'money'] for entity_type in result['entities'].keys())


@pytest.mark.asyncio
class TestEnhancedNLPRoutes:
    """Validate FastAPI route functions for enhanced NLP."""

    @pytest.fixture
    def nlp_service(self) -> EnhancedNLPService:
        return EnhancedNLPService()

    async def test_process_query_route(self, nlp_service):
        payload = NLPQuery(text="show analytics", session_id="route-session")

        response: NLPResponse = await process_query(payload, nlp_service=nlp_service)

        assert response.intent['name'] == 'view_analytics'
        assert response.session_id == 'route-session'
        assert response.processed_at is not None

    async def test_get_suggestions_route(self, nlp_service):
        # Prime context with a prior query so suggestions can consider history
        nlp_service.process_query("generate report", session_id="route-session")

        response = await get_suggestions(
            partial_text="generate",
            session_id="route-session",
            nlp_service=nlp_service,
        )

        assert 'suggestions' in response
        assert any('report' in suggestion.lower() for suggestion in response['suggestions'])
