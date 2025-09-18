use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::Database;
use olympus_shared::error::Result;
use olympus_shared::events::{EventPublisher, Event};
use crate::models::*;
use rust_decimal::Decimal;

pub struct CommerceService {
    db: Arc<Database>,
    event_publisher: Arc<EventPublisher>,
}

impl CommerceService {
    pub fn new(db: Arc<Database>, event_publisher: Arc<EventPublisher>) -> Self {
        Self { db, event_publisher }
    }

    // Product management
    pub async fn create_product(&self, request: CreateProductRequest) -> Result<Product> {
        let pool = self.db.pool();

        let product = Product {
            id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            sku: request.sku,
            name: request.name,
            description: request.description,
            category_id: request.category_id,
            brand: request.brand,
            unit_price: request.unit_price,
            compare_at_price: request.compare_at_price,
            cost: request.cost,
            tax_rate: request.tax_rate.unwrap_or(Decimal::ZERO),
            weight_value: request.weight_value,
            weight_unit: request.weight_unit,
            dimensions: request.dimensions,
            is_digital: request.is_digital.unwrap_or(false),
            is_active: true,
            requires_shipping: request.requires_shipping.unwrap_or(true),
            track_inventory: request.track_inventory.unwrap_or(true),
            allow_backorder: request.allow_backorder.unwrap_or(false),
            images: vec![],
            attributes: serde_json::json!({}),
            metadata: serde_json::json!({}),
            tags: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let result = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (
                id, tenant_id, sku, name, description, category_id, brand,
                unit_price, compare_at_price, cost, tax_rate, weight_value,
                weight_unit, dimensions, is_digital, is_active, requires_shipping,
                track_inventory, allow_backorder, images, attributes, metadata,
                tags, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25
            )
            RETURNING *
            "#,
            product.id,
            product.tenant_id,
            product.sku,
            product.name,
            product.description,
            product.category_id,
            product.brand,
            product.unit_price,
            product.compare_at_price,
            product.cost,
            product.tax_rate,
            product.weight_value,
            product.weight_unit,
            serde_json::to_value(&product.dimensions)?,
            product.is_digital,
            product.is_active,
            product.requires_shipping,
            product.track_inventory,
            product.allow_backorder,
            &product.images,
            product.attributes,
            product.metadata,
            &product.tags,
            product.created_at,
            product.updated_at
        )
        .fetch_one(pool)
        .await?;

        // Publish event
        self.event_publisher.publish(Event::ProductCreated {
            product_id: result.id,
            tenant_id: result.tenant_id,
            sku: result.sku.clone(),
        }).await?;

        Ok(result)
    }

    pub async fn get_product(&self, product_id: Uuid) -> Result<Product> {
        let pool = self.db.pool();

        let product = sqlx::query_as!(
            Product,
            r#"
            SELECT * FROM products
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            product_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| olympus_shared::error::Error::NotFound("Product not found".to_string()))?;

        Ok(product)
    }

    pub async fn list_products(&self, tenant_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Product>> {
        let pool = self.db.pool();

        let products = sqlx::query_as!(
            Product,
            r#"
            SELECT * FROM products
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(products)
    }

    pub async fn update_product(&self, product_id: Uuid, request: UpdateProductRequest) -> Result<Product> {
        let pool = self.db.pool();

        // Build dynamic update query
        let mut query = String::from("UPDATE products SET updated_at = $1");
        let mut param_count = 1;

        if request.name.is_some() {
            param_count += 1;
            query.push_str(&format!(", name = ${}", param_count));
        }

        if request.description.is_some() {
            param_count += 1;
            query.push_str(&format!(", description = ${}", param_count));
        }

        if request.unit_price.is_some() {
            param_count += 1;
            query.push_str(&format!(", unit_price = ${}", param_count));
        }

        query.push_str(&format!(" WHERE id = ${} AND deleted_at IS NULL RETURNING *", param_count + 1));

        let product = sqlx::query_as::<_, Product>(&query)
            .bind(Utc::now())
            .bind(product_id)
            .fetch_one(pool)
            .await?;

        // Publish event
        self.event_publisher.publish(Event::ProductUpdated {
            product_id: product.id,
            tenant_id: product.tenant_id,
        }).await?;

        Ok(product)
    }

    pub async fn delete_product(&self, product_id: Uuid) -> Result<()> {
        let pool = self.db.pool();

        let product = sqlx::query!(
            r#"
            UPDATE products
            SET deleted_at = $1, updated_at = $1
            WHERE id = $2
            RETURNING tenant_id
            "#,
            Utc::now(),
            product_id
        )
        .fetch_one(pool)
        .await?;

        // Publish event
        self.event_publisher.publish(Event::ProductDeleted {
            product_id,
            tenant_id: product.tenant_id,
        }).await?;

        Ok(())
    }

    // Inventory management
    pub async fn get_inventory(&self, product_id: Uuid, location_id: Option<Uuid>) -> Result<Inventory> {
        let pool = self.db.pool();

        let query = if let Some(loc_id) = location_id {
            sqlx::query_as!(
                Inventory,
                r#"
                SELECT * FROM inventory
                WHERE product_id = $1 AND location_id = $2
                "#,
                product_id,
                loc_id
            )
        } else {
            sqlx::query_as!(
                Inventory,
                r#"
                SELECT * FROM inventory
                WHERE product_id = $1
                "#,
                product_id
            )
        };

        let inventory = query
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| olympus_shared::error::Error::NotFound("Inventory not found".to_string()))?;

        Ok(inventory)
    }

    pub async fn update_inventory(&self, product_id: Uuid, location_id: Uuid, quantity: i32) -> Result<Inventory> {
        let pool = self.db.pool();

        let inventory = sqlx::query_as!(
            Inventory,
            r#"
            INSERT INTO inventory (
                id, product_id, variant_id, location_id, quantity_on_hand,
                quantity_reserved, quantity_available, reorder_point, reorder_quantity,
                updated_at
            ) VALUES (
                $1, $2, NULL, $3, $4, 0, $4, 10, 50, $5
            )
            ON CONFLICT (product_id, location_id)
            DO UPDATE SET
                quantity_on_hand = $4,
                quantity_available = $4,
                updated_at = $5
            RETURNING *
            "#,
            Uuid::new_v4(),
            product_id,
            location_id,
            quantity,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;

        // Publish event
        self.event_publisher.publish(Event::InventoryUpdated {
            product_id,
            location_id,
            quantity,
        }).await?;

        Ok(inventory)
    }

    pub async fn adjust_inventory(&self, request: AdjustInventoryRequest) -> Result<Inventory> {
        let pool = self.db.pool();

        // Start transaction
        let mut tx = pool.begin().await?;

        // Get current inventory
        let current = sqlx::query_as!(
            Inventory,
            r#"
            SELECT * FROM inventory
            WHERE product_id = $1 AND location_id = $2
            FOR UPDATE
            "#,
            request.product_id,
            request.location_id
        )
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| olympus_shared::error::Error::NotFound("Inventory not found".to_string()))?;

        // Calculate new quantity
        let new_quantity = match request.adjustment_type {
            AdjustmentType::Add => current.quantity_on_hand + request.quantity,
            AdjustmentType::Remove => current.quantity_on_hand - request.quantity,
            AdjustmentType::Set => request.quantity,
        };

        // Update inventory
        let inventory = sqlx::query_as!(
            Inventory,
            r#"
            UPDATE inventory
            SET quantity_on_hand = $1,
                quantity_available = $1 - quantity_reserved,
                updated_at = $2
            WHERE product_id = $3 AND location_id = $4
            RETURNING *
            "#,
            new_quantity,
            Utc::now(),
            request.product_id,
            request.location_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Record adjustment
        sqlx::query!(
            r#"
            INSERT INTO inventory_adjustments (
                id, inventory_id, adjustment_type, quantity, reason,
                reference_type, reference_id, user_id, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9
            )
            "#,
            Uuid::new_v4(),
            inventory.id,
            serde_json::to_value(&request.adjustment_type)?,
            request.quantity,
            request.reason,
            request.reference_type,
            request.reference_id,
            request.user_id,
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        Ok(inventory)
    }

    // Order management
    pub async fn create_order(&self, request: CreateOrderRequest) -> Result<Order> {
        let pool = self.db.pool();
        let mut tx = pool.begin().await?;

        // Generate order number
        let order_number = format!("ORD-{}", Uuid::new_v4().to_string()[..8].to_uppercase());

        let order = Order {
            id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            order_number,
            customer_id: request.customer_id,
            location_id: request.location_id,
            status: OrderStatus::Pending,
            subtotal: Decimal::ZERO,
            tax_amount: Decimal::ZERO,
            discount_amount: Decimal::ZERO,
            shipping_amount: request.shipping_amount.unwrap_or(Decimal::ZERO),
            total_amount: Decimal::ZERO,
            currency: request.currency,
            payment_status: PaymentStatus::Pending,
            fulfillment_status: FulfillmentStatus::Unfulfilled,
            shipping_address: request.shipping_address.clone(),
            billing_address: request.billing_address.clone().or(request.shipping_address),
            notes: request.notes,
            tags: vec![],
            metadata: serde_json::json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            cancelled_at: None,
            fulfilled_at: None,
        };

        // Calculate totals from items
        let mut subtotal = Decimal::ZERO;
        for item in &request.items {
            subtotal += item.unit_price * Decimal::from(item.quantity);
        }

        let tax_amount = subtotal * Decimal::from_str_exact("0.08").unwrap(); // 8% tax
        let total_amount = subtotal + tax_amount + order.shipping_amount - order.discount_amount;

        // Insert order
        let order = sqlx::query_as!(
            Order,
            r#"
            INSERT INTO orders (
                id, tenant_id, order_number, customer_id, location_id, status,
                subtotal, tax_amount, discount_amount, shipping_amount, total_amount,
                currency, payment_status, fulfillment_status, shipping_address,
                billing_address, notes, tags, metadata, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21
            )
            RETURNING *
            "#,
            order.id,
            order.tenant_id,
            order.order_number,
            order.customer_id,
            order.location_id,
            serde_json::to_value(&order.status)?,
            subtotal,
            tax_amount,
            order.discount_amount,
            order.shipping_amount,
            total_amount,
            serde_json::to_value(&order.currency)?,
            serde_json::to_value(&order.payment_status)?,
            serde_json::to_value(&order.fulfillment_status)?,
            serde_json::to_value(&order.shipping_address)?,
            serde_json::to_value(&order.billing_address)?,
            order.notes,
            &order.tags,
            order.metadata,
            order.created_at,
            order.updated_at
        )
        .fetch_one(&mut *tx)
        .await?;

        // Insert order items
        for item in request.items {
            sqlx::query!(
                r#"
                INSERT INTO order_items (
                    id, order_id, product_id, variant_id, sku, name,
                    quantity, unit_price, discount_amount, tax_amount,
                    total_amount, fulfillment_status, metadata, created_at
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
                )
                "#,
                Uuid::new_v4(),
                order.id,
                item.product_id,
                item.variant_id,
                item.sku,
                item.name,
                item.quantity,
                item.unit_price,
                Decimal::ZERO,
                item.unit_price * Decimal::from_str_exact("0.08").unwrap(),
                item.unit_price * Decimal::from(item.quantity),
                serde_json::to_value(&FulfillmentStatus::Unfulfilled)?,
                serde_json::json!({}),
                Utc::now()
            )
            .execute(&mut *tx)
            .await?;

            // Reserve inventory
            sqlx::query!(
                r#"
                UPDATE inventory
                SET quantity_reserved = quantity_reserved + $1,
                    quantity_available = quantity_on_hand - (quantity_reserved + $1)
                WHERE product_id = $2 AND location_id = $3
                "#,
                item.quantity,
                item.product_id,
                request.location_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // Commit transaction
        tx.commit().await?;

        // Publish event
        self.event_publisher.publish(Event::OrderCreated {
            order_id: order.id,
            tenant_id: order.tenant_id,
            order_number: order.order_number.clone(),
        }).await?;

        Ok(order)
    }

    pub async fn get_order(&self, order_id: Uuid) -> Result<Order> {
        let pool = self.db.pool();

        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT * FROM orders
            WHERE id = $1
            "#,
            order_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| olympus_shared::error::Error::NotFound("Order not found".to_string()))?;

        Ok(order)
    }

    pub async fn list_orders(&self, tenant_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Order>> {
        let pool = self.db.pool();

        let orders = sqlx::query_as!(
            Order,
            r#"
            SELECT * FROM orders
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(orders)
    }

    pub async fn cancel_order(&self, order_id: Uuid) -> Result<Order> {
        let pool = self.db.pool();
        let mut tx = pool.begin().await?;

        // Update order status
        let order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET status = $1,
                cancelled_at = $2,
                updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
            serde_json::to_value(&OrderStatus::Cancelled)?,
            Utc::now(),
            order_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Release reserved inventory
        sqlx::query!(
            r#"
            UPDATE inventory i
            SET quantity_reserved = quantity_reserved - oi.quantity,
                quantity_available = quantity_on_hand - (quantity_reserved - oi.quantity)
            FROM order_items oi
            WHERE oi.order_id = $1
                AND i.product_id = oi.product_id
                AND i.location_id = $2
            "#,
            order_id,
            order.location_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        // Publish event
        self.event_publisher.publish(Event::OrderCancelled {
            order_id: order.id,
            tenant_id: order.tenant_id,
        }).await?;

        Ok(order)
    }

    pub async fn fulfill_order(&self, order_id: Uuid) -> Result<Order> {
        let pool = self.db.pool();
        let mut tx = pool.begin().await?;

        // Update order status
        let order = sqlx::query_as!(
            Order,
            r#"
            UPDATE orders
            SET fulfillment_status = $1,
                fulfilled_at = $2,
                updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
            serde_json::to_value(&FulfillmentStatus::Fulfilled)?,
            Utc::now(),
            order_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Update inventory (remove reserved and on-hand quantities)
        sqlx::query!(
            r#"
            UPDATE inventory i
            SET quantity_reserved = quantity_reserved - oi.quantity,
                quantity_on_hand = quantity_on_hand - oi.quantity,
                quantity_available = quantity_on_hand - oi.quantity - quantity_reserved
            FROM order_items oi
            WHERE oi.order_id = $1
                AND i.product_id = oi.product_id
                AND i.location_id = $2
            "#,
            order_id,
            order.location_id
        )
        .execute(&mut *tx)
        .await?;

        // Commit transaction
        tx.commit().await?;

        // Publish event
        self.event_publisher.publish(Event::OrderFulfilled {
            order_id: order.id,
            tenant_id: order.tenant_id,
        }).await?;

        Ok(order)
    }

    // Customer management
    pub async fn create_customer(&self, request: CreateCustomerRequest) -> Result<Customer> {
        let pool = self.db.pool();

        let customer = Customer {
            id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            phone: request.phone,
            company: request.company,
            addresses: vec![],
            default_address_id: None,
            accepts_marketing: request.accepts_marketing.unwrap_or(false),
            tax_exempt: request.tax_exempt.unwrap_or(false),
            notes: request.notes,
            tags: vec![],
            metadata: serde_json::json!({}),
            total_spent: Decimal::ZERO,
            order_count: 0,
            last_order_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        };

        let result = sqlx::query_as!(
            Customer,
            r#"
            INSERT INTO customers (
                id, tenant_id, email, first_name, last_name, phone, company,
                addresses, accepts_marketing, tax_exempt, notes, tags, metadata,
                total_spent, order_count, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )
            RETURNING *
            "#,
            customer.id,
            customer.tenant_id,
            customer.email,
            customer.first_name,
            customer.last_name,
            customer.phone,
            customer.company,
            serde_json::to_value(&customer.addresses)?,
            customer.accepts_marketing,
            customer.tax_exempt,
            customer.notes,
            &customer.tags,
            customer.metadata,
            customer.total_spent,
            customer.order_count,
            customer.created_at,
            customer.updated_at
        )
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_customer(&self, customer_id: Uuid) -> Result<Customer> {
        let pool = self.db.pool();

        let customer = sqlx::query_as!(
            Customer,
            r#"
            SELECT * FROM customers
            WHERE id = $1 AND deleted_at IS NULL
            "#,
            customer_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| olympus_shared::error::Error::NotFound("Customer not found".to_string()))?;

        Ok(customer)
    }

    pub async fn list_customers(&self, tenant_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Customer>> {
        let pool = self.db.pool();

        let customers = sqlx::query_as!(
            Customer,
            r#"
            SELECT * FROM customers
            WHERE tenant_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            tenant_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(customers)
    }

    // Payment processing
    pub async fn process_payment(&self, request: ProcessPaymentRequest) -> Result<Payment> {
        let pool = self.db.pool();

        let payment = Payment {
            id: Uuid::new_v4(),
            tenant_id: request.tenant_id,
            order_id: request.order_id,
            transaction_id: Uuid::new_v4().to_string(),
            payment_method: request.payment_method,
            payment_type: request.payment_type,
            status: PaymentStatus::Processing,
            amount: request.amount,
            currency: request.currency,
            gateway: request.gateway,
            gateway_response: None,
            reference_number: None,
            authorization_code: None,
            card_last_four: request.card_last_four,
            card_brand: request.card_brand,
            metadata: serde_json::json!({}),
            processed_at: None,
            cancelled_at: None,
            refunded_amount: Decimal::ZERO,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = sqlx::query_as!(
            Payment,
            r#"
            INSERT INTO payments (
                id, tenant_id, order_id, transaction_id, payment_method, payment_type,
                status, amount, currency, gateway, card_last_four, card_brand,
                metadata, refunded_amount, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16
            )
            RETURNING *
            "#,
            payment.id,
            payment.tenant_id,
            payment.order_id,
            payment.transaction_id,
            serde_json::to_value(&payment.payment_method)?,
            serde_json::to_value(&payment.payment_type)?,
            serde_json::to_value(&payment.status)?,
            payment.amount,
            serde_json::to_value(&payment.currency)?,
            payment.gateway,
            payment.card_last_four,
            payment.card_brand,
            payment.metadata,
            payment.refunded_amount,
            payment.created_at,
            payment.updated_at
        )
        .fetch_one(pool)
        .await?;

        // Simulate payment processing
        // In production, this would integrate with actual payment gateways
        let processed_payment = sqlx::query_as!(
            Payment,
            r#"
            UPDATE payments
            SET status = $1,
                processed_at = $2,
                updated_at = $2
            WHERE id = $3
            RETURNING *
            "#,
            serde_json::to_value(&PaymentStatus::Completed)?,
            Utc::now(),
            result.id
        )
        .fetch_one(pool)
        .await?;

        // Update order payment status
        sqlx::query!(
            r#"
            UPDATE orders
            SET payment_status = $1,
                updated_at = $2
            WHERE id = $3
            "#,
            serde_json::to_value(&PaymentStatus::Completed)?,
            Utc::now(),
            payment.order_id
        )
        .execute(pool)
        .await?;

        // Publish event
        self.event_publisher.publish(Event::PaymentProcessed {
            payment_id: processed_payment.id,
            order_id: processed_payment.order_id,
            amount: processed_payment.amount,
        }).await?;

        Ok(processed_payment)
    }
}

// Helper functions
use rust_decimal::prelude::*;
use std::str::FromStr;