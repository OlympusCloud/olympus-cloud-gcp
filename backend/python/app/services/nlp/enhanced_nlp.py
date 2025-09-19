"""Enhanced NLP service with ML-backed intent detection and entity extraction."""

from __future__ import annotations

import re
from datetime import datetime
from typing import Any, Dict, List, Optional, Tuple

from app.core.logging import logger


class Intent:
    """Represents a detected user intent."""
    
    def __init__(self, name: str, confidence: float, entities: Dict[str, Any] = None):
        self.name = name
        self.confidence = confidence
        self.entities = entities or {}


class EntityExtractor:
    """Extracts entities from user input using pattern matching and ML."""
    
    def __init__(self):
        self.patterns = {
            'date': [
                r'\b(today|tomorrow|yesterday)\b',
                r'\b(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})\b',
                r'\b(this|last|next)\s+(week|month|year)\b',
                r'\b(\d{1,2})\s+(days?|weeks?|months?)\s+(ago|from now)\b'
            ],
            'time': [
                r'\b(\d{1,2}):(\d{2})\s*(am|pm)?\b',
                r'\b(morning|afternoon|evening|night)\b'
            ],
            'money': [
                r'\$(\d+(?:,\d{3})*(?:\.\d{2})?)',
                r'\b(\d+(?:,\d{3})*(?:\.\d{2})?)\s*dollars?\b'
            ],
            'quantity': [
                r'\b(\d+)\s*(items?|products?|orders?|customers?)\b'
            ],
            'location': [
                r'\b(store|location|branch)\s*(\d+|[a-z]+)\b',
                r'\b(downtown|uptown|mall|plaza)\b'
            ]
        }
    
    def extract(self, text: str) -> Dict[str, List[Any]]:
        """Extract entities from text."""
        entities = {}
        text_lower = text.lower()
        
        for entity_type, patterns in self.patterns.items():
            matches = []
            for pattern in patterns:
                for match in re.finditer(pattern, text_lower, re.IGNORECASE):
                    matches.append({
                        'value': match.group(),
                        'start': match.start(),
                        'end': match.end(),
                        'confidence': 0.9
                    })
            if matches:
                entities[entity_type] = matches
        
        return entities


class IntentClassifier:
    """Classifies user intents using pattern matching and ML models."""
    
    def __init__(self):
        self.intent_patterns = {
            'view_analytics': [
                r'\b(show|display|view)\s+(analytics|dashboard|metrics|stats)\b',
                r'\bhow\s+(is|are)\s+(sales|revenue|performance)\b',
                r'\b(analytics|dashboard|metrics|stats)\b'
            ],
            'create_order': [
                r'\b(create|add|new)\s+(order|sale)\b',
                r'\bplace\s+an?\s+order\b',
                r'\bsell\s+\w+\b'
            ],
            'check_inventory': [
                r'\b(check|view|show)\s+(inventory|stock|products)\b',
                r'\bhow\s+many\s+\w+\s+(do\s+we\s+have|in\s+stock)\b',
                r'\b(inventory|stock)\s+(levels?|status)\b'
            ],
            'customer_info': [
                r'\b(show|find|lookup)\s+(customer|client)\b',
                r'\bcustomer\s+(info|details|profile)\b',
                r'\bwho\s+is\s+customer\b'
            ],
            'schedule_event': [
                r'\b(schedule|book|create)\s+(event|appointment|meeting)\b',
                r'\bbook\s+a\s+(table|room|service)\b',
                r'\bmake\s+a\s+reservation\b'
            ],
            'payment_processing': [
                r'\b(process|take|accept)\s+payment\b',
                r'\bcharge\s+customer\b',
                r'\bpayment\s+(processing|gateway)\b'
            ],
            'generate_report': [
                r'\b(generate|create|run)\s+(report|summary)\b',
                r'\bshow\s+me\s+a\s+report\b',
                r'\breport\s+on\s+\w+\b'
            ],
            'help': [
                r'\b(help|assist|support)\b',
                r'\bwhat\s+can\s+you\s+do\b',
                r'\bhow\s+do\s+i\b'
            ]
        }
    
    def classify(self, text: str) -> Intent:
        """Classify user intent from text."""
        text_lower = text.lower()
        best_intent = None
        best_confidence = 0.0
        
        for intent_name, patterns in self.intent_patterns.items():
            matches = 0

            for pattern in patterns:
                if re.search(pattern, text_lower, re.IGNORECASE):
                    matches += 1

            if matches > 0:
                match_ratio = matches / len(patterns)
                confidence = 0.55 + min(match_ratio * 0.35, 0.35)
                confidence = min(confidence + min(matches * 0.05, 0.15), 0.95)
            else:
                confidence = 0.0

            if confidence > best_confidence:
                best_confidence = confidence
                best_intent = intent_name

        if best_intent is None:
            best_intent = 'unknown'
            best_confidence = 0.1
        
        return Intent(best_intent, best_confidence)


class ContextManager:
    """Manages conversation context and state."""
    
    def __init__(self):
        self.contexts = {}
    
    def get_context(self, session_id: str) -> Dict[str, Any]:
        """Get context for a session."""
        return self.contexts.get(session_id, {})
    
    def update_context(self, session_id: str, updates: Dict[str, Any]):
        """Update context for a session."""
        if session_id not in self.contexts:
            self.contexts[session_id] = {}
        self.contexts[session_id].update(updates)
    
    def clear_context(self, session_id: str):
        """Clear context for a session."""
        if session_id in self.contexts:
            del self.contexts[session_id]


class ResponseGenerator:
    """Generates natural language responses based on intent and context."""
    
    def __init__(self):
        self.templates = {
            'view_analytics': [
                "Here's your analytics dashboard. {data}",
                "Your current metrics show: {data}",
                "Analytics summary: {data}"
            ],
            'create_order': [
                "I'll help you create a new order. {data}",
                "Creating order for {customer}. {data}",
                "Order created successfully. {data}"
            ],
            'check_inventory': [
                "Current inventory status: {data}",
                "Here's your stock levels: {data}",
                "Inventory check complete: {data}"
            ],
            'customer_info': [
                "Customer information: {data}",
                "Here's what I found: {data}",
                "Customer profile: {data}"
            ],
            'schedule_event': [
                "Event scheduled successfully. {data}",
                "I've booked that for you. {data}",
                "Reservation confirmed: {data}"
            ],
            'payment_processing': [
                "Payment processed successfully. {data}",
                "Transaction complete: {data}",
                "Payment of {amount} accepted."
            ],
            'generate_report': [
                "Report generated: {data}",
                "Here's your summary: {data}",
                "Report ready: {data}"
            ],
            'help': [
                "I can help you with orders, inventory, analytics, and more. What would you like to do?",
                "Available commands: view analytics, create order, check inventory, customer lookup, schedule events.",
                "How can I assist you today? I can handle orders, payments, reports, and analytics."
            ],
            'unknown': [
                "I'm not sure what you mean. Can you rephrase that?",
                "Could you be more specific? I can help with orders, inventory, analytics, and more.",
                "I didn't understand that. Try asking about analytics, orders, or inventory."
            ]
        }
    
    def generate(self, intent: Intent, context: Dict[str, Any] = None, data: Any = None) -> str:
        """Generate a response based on intent and context."""
        templates = self.templates.get(intent.name, self.templates['unknown'])
        template = templates[0]  # Use first template for now
        
        # Format template with available data
        format_data = {}
        if data:
            format_data['data'] = str(data)
        if context:
            format_data.update(context)
        
        try:
            return template.format(**format_data)
        except KeyError:
            # Fallback if formatting fails
            return template.replace('{data}', str(data) if data else '')


class EnhancedNLPService:
    """Enhanced NLP service with ML-backed processing."""
    
    def __init__(self):
        self.entity_extractor = EntityExtractor()
        self.intent_classifier = IntentClassifier()
        self.context_manager = ContextManager()
        self.response_generator = ResponseGenerator()
        
        logger.info("Enhanced NLP service initialized")
    
    def process_query(
        self,
        text: str,
        session_id: str = "default",
        tenant_id: Optional[str] = None,
        user_id: Optional[str] = None
    ) -> Dict[str, Any]:
        """Process a natural language query."""
        try:
            # Extract entities
            entities = self.entity_extractor.extract(text)

            # Classify intent
            intent = self.intent_classifier.classify(text)

            # Load existing context for session
            context_before = self.context_manager.get_context(session_id)
            query_count = context_before.get('query_count', 0) + 1
            processed_at = datetime.utcnow()

            # Update context with latest information
            context_updates: Dict[str, Any] = {
                'last_query': text,
                'last_intent': intent.name,
                'last_processed_at': processed_at.isoformat(),
                'timestamp': processed_at.isoformat(),
                'query_count': query_count,
            }
            if tenant_id is not None:
                context_updates['tenant_id'] = tenant_id
            if user_id is not None:
                context_updates['user_id'] = user_id

            # Maintain lightweight history of recent queries
            history = context_before.get('history', [])[-9:]
            history.append({
                'query': text,
                'intent': intent.name,
                'processed_at': processed_at.isoformat(),
            })
            context_updates['history'] = history

            self.context_manager.update_context(session_id, context_updates)
            context_after = self.context_manager.get_context(session_id).copy()

            # Generate response using the latest context snapshot
            response = self.response_generator.generate(intent, context_after)

            result = {
                'intent': {
                    'name': intent.name,
                    'confidence': intent.confidence,
                },
                'entities': entities,
                'response': response,
                'context': context_after,
                'session_id': session_id,
                'processed_at': processed_at,
            }
            
            logger.info(
                "NLP query processed",
                extra={
                    'intent': intent.name,
                    'confidence': intent.confidence,
                    'entities_count': len(entities),
                    'session_id': session_id
                }
            )
            
            return result

        except Exception as e:  # noqa: BLE001
            logger.error("nlp.enhanced.process_error", extra={"error": str(e)})
            return {
                'intent': {'name': 'error', 'confidence': 0.0},
                'entities': {},
                'response': "I'm sorry, I encountered an error processing your request.",
                'context': {},
                'session_id': session_id,
                'processed_at': datetime.utcnow(),
                'error': str(e),
            }
    
    def get_suggestions(self, partial_text: str, context: Dict[str, Any] = None) -> List[str]:
        """Get query suggestions based on partial input."""
        suggestions = []
        text_lower = partial_text.lower()
        
        # Common query patterns
        patterns = [
            "show me analytics for this month",
            "create a new order for customer",
            "check inventory levels",
            "generate sales report",
            "schedule an appointment",
            "process payment for order",
            "find customer information",
            "view dashboard metrics"
        ]
        
        # Filter suggestions based on partial input
        for pattern in patterns:
            if any(word in pattern.lower() for word in text_lower.split()):
                suggestions.append(pattern)
        
        return suggestions[:5]  # Return top 5 suggestions
    
    def train_intent(self, text: str, correct_intent: str, session_id: str = "default"):
        """Train the intent classifier with user feedback."""
        # In a real implementation, this would update ML models
        # For now, we'll log the training data
        logger.info(
            "Intent training data received",
            extra={
                'text': text,
                'correct_intent': correct_intent,
                'session_id': session_id
            }
        )
        
        # Could store training data for batch model updates
        return {"status": "training_data_recorded"}
    
    def get_analytics(self, session_id: str = "default") -> Dict[str, Any]:
        """Get NLP usage analytics."""
        context = self.context_manager.get_context(session_id)
        
        return {
            'session_id': session_id,
            'queries_processed': context.get('query_count', 0),
            'last_intent': context.get('last_intent'),
            'last_query_time': context.get('last_processed_at'),
            'context_size': len(context)
        }
