# Python Analytics Events Integration

## Overview
This document defines the event schemas and integration patterns for the Python analytics service to consume events from Rust services via Redis pub/sub.

## Redis Configuration
```python
# Python connection
redis_client = redis.Redis(
    host='localhost',
    port=6379,
    decode_responses=True
)

# Subscribe to event channels
pubsub = redis_client.pubsub()
pubsub.psubscribe('events:*')
```

## Event Schema

### Base Event Structure
All events follow this base structure:
```json
{
  "event_id": "uuid-v4",
  "event_type": "category.action",
  "tenant_id": "uuid-v4",
  "user_id": "uuid-v4",
  "timestamp": "2024-01-01T00:00:00Z",
  "data": { ... },
  "metadata": {
    "ip_address": "192.168.1.1",
    "user_agent": "Mozilla/5.0...",
    "session_id": "uuid-v4",
    "request_id": "uuid-v4"
  }
}
```

## User Events

### Channel: `events:user:*`

#### user.registered
Fired when a new user completes registration.
```json
{
  "event_type": "user.registered",
  "data": {
    "user_id": "uuid",
    "email": "user@example.com",
    "tenant_slug": "acme-corp",
    "registration_method": "email",
    "referral_source": "organic"
  }
}
```

#### user.logged_in
```json
{
  "event_type": "user.logged_in",
  "data": {
    "user_id": "uuid",
    "login_method": "password",
    "device_type": "web",
    "location": {
      "country": "US",
      "city": "San Francisco"
    }
  }
}
```

#### user.logged_out
```json
{
  "event_type": "user.logged_out",
  "data": {
    "user_id": "uuid",
    "session_duration_seconds": 3600
  }
}
```

#### user.profile_updated
```json
{
  "event_type": "user.profile_updated",
  "data": {
    "user_id": "uuid",
    "fields_updated": ["first_name", "last_name", "phone"],
    "previous_values": { ... },
    "new_values": { ... }
  }
}
```

## Order Events

### Channel: `events:order:*`

#### order.created
```json
{
  "event_type": "order.created",
  "data": {
    "order_id": "uuid",
    "order_number": "ORD-2024-001",
    "customer_id": "uuid",
    "location_id": "uuid",
    "items_count": 3,
    "subtotal": 149.99,
    "tax_amount": 15.00,
    "shipping_amount": 10.00,
    "total_amount": 174.99,
    "currency": "USD",
    "items": [
      {
        "product_id": "uuid",
        "sku": "PROD-001",
        "name": "Product Name",
        "quantity": 2,
        "unit_price": 49.99,
        "total_price": 99.98
      }
    ]
  }
}
```

#### order.updated
```json
{
  "event_type": "order.updated",
  "data": {
    "order_id": "uuid",
    "order_number": "ORD-2024-001",
    "status": "processing",
    "previous_status": "pending",
    "updated_fields": ["status", "notes"]
  }
}
```

#### order.cancelled
```json
{
  "event_type": "order.cancelled",
  "data": {
    "order_id": "uuid",
    "order_number": "ORD-2024-001",
    "cancellation_reason": "customer_request",
    "cancelled_by": "uuid",
    "refund_amount": 174.99
  }
}
```

#### order.fulfilled
```json
{
  "event_type": "order.fulfilled",
  "data": {
    "order_id": "uuid",
    "order_number": "ORD-2024-001",
    "fulfillment_method": "shipping",
    "tracking_number": "1Z999AA10123456784",
    "carrier": "UPS",
    "estimated_delivery": "2024-01-05"
  }
}
```

## Payment Events

### Channel: `events:payment:*`

#### payment.processed
```json
{
  "event_type": "payment.processed",
  "data": {
    "payment_id": "uuid",
    "order_id": "uuid",
    "amount": 174.99,
    "currency": "USD",
    "payment_method": "credit_card",
    "gateway": "stripe",
    "transaction_id": "ch_1234567890",
    "card_last_four": "4242",
    "card_brand": "visa"
  }
}
```

#### payment.failed
```json
{
  "event_type": "payment.failed",
  "data": {
    "payment_id": "uuid",
    "order_id": "uuid",
    "amount": 174.99,
    "currency": "USD",
    "failure_reason": "insufficient_funds",
    "error_code": "card_declined",
    "retry_count": 1
  }
}
```

#### payment.refunded
```json
{
  "event_type": "payment.refunded",
  "data": {
    "payment_id": "uuid",
    "order_id": "uuid",
    "refund_amount": 174.99,
    "refund_reason": "customer_request",
    "refund_type": "full",
    "refund_transaction_id": "re_1234567890"
  }
}
```

## Inventory Events

### Channel: `events:inventory:*`

#### inventory.low_stock
```json
{
  "event_type": "inventory.low_stock",
  "data": {
    "product_id": "uuid",
    "variant_id": "uuid",
    "sku": "PROD-001",
    "location_id": "uuid",
    "current_quantity": 5,
    "reorder_point": 10,
    "reorder_quantity": 50
  }
}
```

#### inventory.out_of_stock
```json
{
  "event_type": "inventory.out_of_stock",
  "data": {
    "product_id": "uuid",
    "variant_id": "uuid",
    "sku": "PROD-001",
    "location_id": "uuid",
    "last_sold_at": "2024-01-01T00:00:00Z"
  }
}
```

#### inventory.restocked
```json
{
  "event_type": "inventory.restocked",
  "data": {
    "product_id": "uuid",
    "variant_id": "uuid",
    "sku": "PROD-001",
    "location_id": "uuid",
    "quantity_added": 100,
    "new_quantity": 105,
    "restocked_by": "uuid",
    "supplier": "Supplier Name"
  }
}
```

#### inventory.adjusted
```json
{
  "event_type": "inventory.adjusted",
  "data": {
    "inventory_id": "uuid",
    "product_id": "uuid",
    "location_id": "uuid",
    "adjustment_type": "manual",
    "previous_quantity": 50,
    "new_quantity": 48,
    "adjustment_reason": "damaged_goods",
    "adjusted_by": "uuid"
  }
}
```

## Python Consumer Example

```python
import json
import redis
from datetime import datetime
from typing import Dict, Any
import asyncio

class EventProcessor:
    def __init__(self, redis_url: str):
        self.redis_client = redis.from_url(redis_url)
        self.pubsub = self.redis_client.pubsub()

    async def process_events(self):
        """Main event processing loop"""
        # Subscribe to all event channels
        self.pubsub.psubscribe('events:*')

        for message in self.pubsub.listen():
            if message['type'] == 'pmessage':
                await self.handle_event(
                    channel=message['channel'],
                    data=json.loads(message['data'])
                )

    async def handle_event(self, channel: str, data: Dict[str, Any]):
        """Route events to appropriate handlers"""
        event_type = data.get('event_type', '')

        handlers = {
            'user.registered': self.handle_user_registered,
            'user.logged_in': self.handle_user_login,
            'order.created': self.handle_order_created,
            'payment.processed': self.handle_payment_processed,
            'inventory.low_stock': self.handle_low_stock,
        }

        handler = handlers.get(event_type)
        if handler:
            await handler(data)
        else:
            print(f"Unhandled event type: {event_type}")

    async def handle_user_registered(self, data: Dict[str, Any]):
        """Process user registration for analytics"""
        tenant_id = data['tenant_id']
        user_data = data['data']

        # Update user acquisition metrics
        await self.update_metric(
            tenant_id=tenant_id,
            metric='user_registrations',
            value=1,
            dimensions={
                'source': user_data.get('referral_source', 'organic'),
                'method': user_data.get('registration_method', 'email')
            }
        )

        # Track cohort data
        await self.add_to_cohort(
            tenant_id=tenant_id,
            cohort_date=datetime.fromisoformat(data['timestamp']),
            user_id=user_data['user_id']
        )

    async def handle_order_created(self, data: Dict[str, Any]):
        """Process order creation for analytics"""
        tenant_id = data['tenant_id']
        order_data = data['data']

        # Update revenue metrics
        await self.update_metric(
            tenant_id=tenant_id,
            metric='revenue',
            value=order_data['total_amount'],
            dimensions={
                'location': order_data['location_id'],
                'currency': order_data['currency']
            }
        )

        # Update order count
        await self.update_metric(
            tenant_id=tenant_id,
            metric='order_count',
            value=1
        )

        # Track product performance
        for item in order_data['items']:
            await self.update_product_metrics(
                tenant_id=tenant_id,
                product_id=item['product_id'],
                quantity=item['quantity'],
                revenue=item['total_price']
            )

    async def handle_low_stock(self, data: Dict[str, Any]):
        """Generate alerts for low stock"""
        tenant_id = data['tenant_id']
        inventory_data = data['data']

        # Create alert
        await self.create_alert(
            tenant_id=tenant_id,
            alert_type='low_stock',
            severity='warning',
            data=inventory_data,
            message=f"Low stock alert: {inventory_data['sku']} at location {inventory_data['location_id']}"
        )

        # Update inventory metrics
        await self.update_metric(
            tenant_id=tenant_id,
            metric='low_stock_items',
            value=1,
            dimensions={
                'location': inventory_data['location_id']
            }
        )

    # Helper methods (implement based on your storage strategy)
    async def update_metric(self, tenant_id: str, metric: str, value: float, dimensions: Dict = None):
        """Update analytics metrics in database"""
        pass

    async def add_to_cohort(self, tenant_id: str, cohort_date: datetime, user_id: str):
        """Add user to cohort for retention analysis"""
        pass

    async def update_product_metrics(self, tenant_id: str, product_id: str, quantity: int, revenue: float):
        """Update product performance metrics"""
        pass

    async def create_alert(self, tenant_id: str, alert_type: str, severity: str, data: Dict, message: str):
        """Create alert for dashboard"""
        pass

# Usage
if __name__ == "__main__":
    processor = EventProcessor(redis_url="redis://localhost:6379")
    asyncio.run(processor.process_events())
```

## BigQuery Integration

For long-term analytics storage:

```python
from google.cloud import bigquery
from datetime import datetime

class BigQueryEventWriter:
    def __init__(self, project_id: str, dataset_id: str):
        self.client = bigquery.Client(project=project_id)
        self.dataset_id = dataset_id
        self.buffer = []
        self.buffer_size = 1000  # Batch size

    async def write_event(self, event: Dict[str, Any]):
        """Buffer events for batch insertion"""
        # Transform event for BigQuery
        row = {
            'event_id': event['event_id'],
            'event_type': event['event_type'],
            'tenant_id': event['tenant_id'],
            'user_id': event['user_id'],
            'timestamp': event['timestamp'],
            'data': json.dumps(event['data']),
            'metadata': json.dumps(event.get('metadata', {})),
            'inserted_at': datetime.utcnow().isoformat()
        }

        self.buffer.append(row)

        if len(self.buffer) >= self.buffer_size:
            await self.flush()

    async def flush(self):
        """Write buffered events to BigQuery"""
        if not self.buffer:
            return

        table_id = f"{self.dataset_id}.events"
        table = self.client.get_table(table_id)

        errors = self.client.insert_rows_json(table, self.buffer)
        if errors:
            print(f"BigQuery insertion errors: {errors}")
        else:
            print(f"Inserted {len(self.buffer)} events to BigQuery")

        self.buffer = []
```

## Metrics Aggregation

Key metrics to calculate from events:

### User Metrics
- Daily/Monthly Active Users (DAU/MAU)
- User retention cohorts
- Session duration
- Login frequency

### Commerce Metrics
- Revenue by timeframe
- Average Order Value (AOV)
- Conversion rate
- Product performance
- Cart abandonment rate

### Inventory Metrics
- Stock turnover rate
- Out-of-stock frequency
- Restock cycle time
- Dead stock identification

### Operational Metrics
- API response times (from metadata)
- Error rates
- Peak usage hours
- Geographic distribution

## Dashboard Queries

Example queries for the analytics dashboard:

```sql
-- Revenue by day
SELECT
  DATE(timestamp) as date,
  tenant_id,
  SUM(JSON_EXTRACT_SCALAR(data, '$.total_amount')) as revenue
FROM events
WHERE event_type = 'order.created'
  AND tenant_id = @tenant_id
  AND timestamp >= @start_date
GROUP BY date, tenant_id
ORDER BY date DESC;

-- Top products by revenue
SELECT
  JSON_EXTRACT_SCALAR(item, '$.product_id') as product_id,
  JSON_EXTRACT_SCALAR(item, '$.name') as product_name,
  SUM(JSON_EXTRACT_SCALAR(item, '$.quantity')) as units_sold,
  SUM(JSON_EXTRACT_SCALAR(item, '$.total_price')) as revenue
FROM events,
  UNNEST(JSON_EXTRACT_ARRAY(data, '$.items')) as item
WHERE event_type = 'order.created'
  AND tenant_id = @tenant_id
  AND timestamp >= @start_date
GROUP BY product_id, product_name
ORDER BY revenue DESC
LIMIT 10;

-- User activity heatmap
SELECT
  EXTRACT(HOUR FROM timestamp) as hour,
  COUNT(DISTINCT user_id) as active_users,
  COUNT(*) as events
FROM events
WHERE tenant_id = @tenant_id
  AND timestamp >= CURRENT_TIMESTAMP - INTERVAL 7 DAY
GROUP BY hour
ORDER BY hour;
```

## Testing Integration

1. Publish test event from Rust:
```bash
curl -X POST http://localhost:8000/test/publish-event \
  -H "Content-Type: application/json" \
  -d '{
    "event_type": "order.created",
    "tenant_id": "test-tenant",
    "data": {
      "order_id": "test-order",
      "total_amount": 99.99
    }
  }'
```

2. Monitor Redis:
```bash
redis-cli
> PSUBSCRIBE events:*
```

3. Check Python consumer logs for processed events

## Contact
For questions about event schemas or Rust service integration, see `/backend/rust/STATUS.md`