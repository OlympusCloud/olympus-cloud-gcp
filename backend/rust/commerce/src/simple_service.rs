use std::sync::Arc;
use uuid::Uuid;
use sqlx::PgPool;
use chrono::Utc;

use crate::simple_models::*;
use olympus_shared::error::{Result, Error};

#[derive(Clone)]
pub struct SimpleCommerceService {
    db: Arc<PgPool>,
}

impl SimpleCommerceService {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    // Product operations
    pub async fn create_product(
        &self,
        tenant_id: Uuid,
        request: CreateProductRequest,
    ) -> Result<Product> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            INSERT INTO commerce.products (tenant_id, name, description, price)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, description, price, is_active, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(&request.name)
        .bind(&request.description)
        .bind(&request.price)
        .fetch_one(&*self.db)
        .await?;

        Ok(product)
    }

    pub async fn get_product(&self, tenant_id: Uuid, product_id: Uuid) -> Result<Product> {
        let product = sqlx::query_as::<_, Product>(
            r#"
            SELECT id, tenant_id, name, description, price, is_active, created_at, updated_at
            FROM commerce.products
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(product_id)
        .bind(tenant_id)
        .fetch_optional(&*self.db)
        .await?
        .ok_or_else(|| Error::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    pub async fn list_products(
        &self,
        tenant_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<(Vec<Product>, i64)> {
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        let products = sqlx::query_as::<_, Product>(
            r#"
            SELECT id, tenant_id, name, description, price, is_active, created_at, updated_at
            FROM commerce.products
            WHERE tenant_id = $1 AND is_active = true
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(tenant_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&*self.db)
        .await?;

        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM commerce.products WHERE tenant_id = $1 AND is_active = true"
        )
        .bind(tenant_id)
        .fetch_one(&*self.db)
        .await?;

        Ok((products, total))
    }

    pub async fn update_product(
        &self,
        tenant_id: Uuid,
        product_id: Uuid,
        request: UpdateProductRequest,
    ) -> Result<Product> {
        // Get current product
        let mut product = self.get_product(tenant_id, product_id).await?;

        // Update fields if provided
        if let Some(name) = request.name {
            product.name = name;
        }
        if let Some(description) = request.description {
            product.description = Some(description);
        }
        if let Some(price) = request.price {
            product.price = Some(price);
        }
        if let Some(is_active) = request.is_active {
            product.is_active = is_active;
        }

        product.updated_at = Utc::now();

        let updated_product = sqlx::query_as::<_, Product>(
            r#"
            UPDATE commerce.products
            SET name = $3, description = $4, price = $5, is_active = $6, updated_at = $7
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, name, description, price, is_active, created_at, updated_at
            "#
        )
        .bind(product_id)
        .bind(tenant_id)
        .bind(&product.name)
        .bind(&product.description)
        .bind(&product.price)
        .bind(product.is_active)
        .bind(product.updated_at)
        .fetch_one(&*self.db)
        .await?;

        Ok(updated_product)
    }

    // Order operations
    pub async fn create_order(
        &self,
        tenant_id: Uuid,
        request: CreateOrderRequest,
    ) -> Result<Order> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            INSERT INTO commerce.orders (tenant_id, user_id, status)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, user_id, status, total, created_at, updated_at
            "#
        )
        .bind(tenant_id)
        .bind(&request.user_id)
        .bind(&request.status.unwrap_or(OrderStatus::Draft))
        .fetch_one(&*self.db)
        .await?;

        Ok(order)
    }

    pub async fn get_order(&self, tenant_id: Uuid, order_id: Uuid) -> Result<Order> {
        let order = sqlx::query_as::<_, Order>(
            r#"
            SELECT id, tenant_id, user_id, status, total, created_at, updated_at
            FROM commerce.orders
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(order_id)
        .bind(tenant_id)
        .fetch_optional(&*self.db)
        .await?
        .ok_or_else(|| Error::NotFound("Order not found".to_string()))?;

        Ok(order)
    }

    pub async fn list_orders(
        &self,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<(Vec<Order>, i64)> {
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        let (orders, total) = if let Some(user_id) = user_id {
            let orders = sqlx::query_as::<_, Order>(
                r#"
                SELECT id, tenant_id, user_id, status, total, created_at, updated_at
                FROM commerce.orders
                WHERE tenant_id = $1 AND user_id = $2
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
                "#
            )
            .bind(tenant_id)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.db)
            .await?;

            let total: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM commerce.orders WHERE tenant_id = $1 AND user_id = $2"
            )
            .bind(tenant_id)
            .bind(user_id)
            .fetch_one(&*self.db)
            .await?;

            (orders, total)
        } else {
            let orders = sqlx::query_as::<_, Order>(
                r#"
                SELECT id, tenant_id, user_id, status, total, created_at, updated_at
                FROM commerce.orders
                WHERE tenant_id = $1
                ORDER BY created_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(tenant_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&*self.db)
            .await?;

            let total: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM commerce.orders WHERE tenant_id = $1"
            )
            .bind(tenant_id)
            .fetch_one(&*self.db)
            .await?;

            (orders, total)
        };

        Ok((orders, total))
    }

    pub async fn update_order(
        &self,
        tenant_id: Uuid,
        order_id: Uuid,
        request: UpdateOrderRequest,
    ) -> Result<Order> {
        // Get current order
        let mut order = self.get_order(tenant_id, order_id).await?;

        // Update fields if provided
        if let Some(status) = request.status {
            order.status = status;
        }
        if let Some(total) = request.total {
            order.total = total;
        }

        order.updated_at = Utc::now();

        let updated_order = sqlx::query_as::<_, Order>(
            r#"
            UPDATE commerce.orders
            SET status = $3, total = $4, updated_at = $5
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, user_id, status, total, created_at, updated_at
            "#
        )
        .bind(order_id)
        .bind(tenant_id)
        .bind(&order.status)
        .bind(&order.total)
        .bind(order.updated_at)
        .fetch_one(&*self.db)
        .await?;

        Ok(updated_order)
    }
}