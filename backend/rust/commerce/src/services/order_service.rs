use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::Database;
use olympus_shared::types::{PageRequest, PageResponse, Money, Currency};
use crate::models::{Order, CreateOrderRequest, OrderStatus};

pub struct OrderService {
    _db: Arc<Database>,
}

impl OrderService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { _db: db }
    }

    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order, String> {
        let order = Order {
            id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            customer_id: request.customer_id,
            location_id: request.location_id,
            order_number: format!("ORD-{}", Uuid::new_v4().to_string()[..8].to_uppercase()),
            status: OrderStatus::Pending,
            total_amount: Money::new(0, Currency::USD),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        Ok(order)
    }

    pub async fn get_order(&self, order_id: Uuid) -> Result<Order, String> {
        Ok(Order {
            id: order_id,
            tenant_id: Uuid::new_v4(),
            customer_id: Some(Uuid::new_v4()),
            location_id: Uuid::new_v4(),
            order_number: "ORD-12345678".to_string(),
            status: OrderStatus::Pending,
            total_amount: Money::new(2500, Currency::USD),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn list_orders(&self, _page: PageRequest) -> Result<PageResponse<Order>, String> {
        let orders = vec![
            Order {
                id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                customer_id: Some(Uuid::new_v4()),
                location_id: Uuid::new_v4(),
                order_number: "ORD-12345678".to_string(),
                status: OrderStatus::Pending,
                total_amount: Money::new(2500, Currency::USD),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }
        ];
        
        Ok(PageResponse::new(orders, 1, 1, 10))
    }

    pub async fn update_order_status(&self, order_id: Uuid, status: OrderStatus) -> Result<Order, String> {
        Ok(Order {
            id: order_id,
            tenant_id: Uuid::new_v4(),
            customer_id: Some(Uuid::new_v4()),
            location_id: Uuid::new_v4(),
            order_number: "ORD-12345678".to_string(),
            status,
            total_amount: Money::new(2500, Currency::USD),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}