//! Comprehensive unit tests for the commerce service

use olympus_commerce::{
    models::{Product, Order, OrderItem, Payment, InventoryItem},
    services::{ProductService, OrderService, PaymentService, InventoryService},
};
use olympus_shared::Result;
use rust_decimal::Decimal;
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;
use chrono::Utc;

#[cfg(test)]
mod unit_tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_price_calculation() {
        use olympus_commerce::utils::calculate_price_with_tax;

        let price = dec!(100.00);
        let tax_rate = dec!(0.08);
        let total = calculate_price_with_tax(price, tax_rate);
        assert_eq!(total, dec!(108.00));

        let price = dec!(49.99);
        let tax_rate = dec!(0.0875);
        let total = calculate_price_with_tax(price, tax_rate);
        assert_eq!(total.round_dp(2), dec!(54.36));
    }

    #[tokio::test]
    async fn test_sku_generation() {
        use olympus_commerce::utils::generate_sku;

        let sku = generate_sku("Restaurant", "Burger", Some("001"));
        assert!(sku.starts_with("RES-BUR-001"));

        let auto_sku = generate_sku("Retail", "T-Shirt", None);
        assert!(auto_sku.starts_with("RET-TSH"));
    }

    #[tokio::test]
    async fn test_order_number_generation() {
        use olympus_commerce::utils::generate_order_number;

        let order_num = generate_order_number();
        assert!(order_num.starts_with("ORD-"));
        assert!(order_num.len() > 4);

        // Test uniqueness
        let order_num2 = generate_order_number();
        assert_ne!(order_num, order_num2);
    }

    #[tokio::test]
    async fn test_order_status_transitions() {
        use olympus_commerce::models::OrderStatus;

        let status = OrderStatus::Pending;
        assert!(status.can_transition_to(OrderStatus::Processing));
        assert!(status.can_transition_to(OrderStatus::Cancelled));
        assert!(!status.can_transition_to(OrderStatus::Completed));

        let status = OrderStatus::Processing;
        assert!(status.can_transition_to(OrderStatus::Ready));
        assert!(status.can_transition_to(OrderStatus::Cancelled));
        assert!(!status.can_transition_to(OrderStatus::Pending));

        let status = OrderStatus::Completed;
        assert!(!status.can_transition_to(OrderStatus::Processing));
        assert!(status.can_transition_to(OrderStatus::Refunded));
    }

    #[tokio::test]
    async fn test_payment_validation() {
        use olympus_commerce::models::PaymentMethod;

        assert!(PaymentMethod::is_valid("cash"));
        assert!(PaymentMethod::is_valid("card"));
        assert!(PaymentMethod::is_valid("wallet"));
        assert!(PaymentMethod::is_valid("bank_transfer"));
        assert!(!PaymentMethod::is_valid("bitcoin"));

        assert!(PaymentMethod::requires_processing("card"));
        assert!(PaymentMethod::requires_processing("wallet"));
        assert!(!PaymentMethod::requires_processing("cash"));
    }

    #[tokio::test]
    async fn test_inventory_calculations() {
        use olympus_commerce::models::InventoryItem;

        let item = InventoryItem {
            id: Uuid::new_v4(),
            product_id: Uuid::new_v4(),
            location_id: Uuid::new_v4(),
            quantity_on_hand: 100,
            quantity_reserved: 20,
            reorder_point: 30,
            reorder_quantity: 50,
            last_restock: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(item.available_quantity(), 80);
        assert!(!item.needs_reorder());

        let low_stock_item = InventoryItem {
            quantity_on_hand: 25,
            quantity_reserved: 10,
            ..item
        };

        assert_eq!(low_stock_item.available_quantity(), 15);
        assert!(low_stock_item.needs_reorder());
    }

    #[tokio::test]
    async fn test_discount_calculation() {
        use olympus_commerce::utils::{calculate_discount, DiscountType};

        let subtotal = dec!(100.00);

        let percentage_discount = calculate_discount(
            subtotal,
            DiscountType::Percentage,
            dec!(10.00),
        );
        assert_eq!(percentage_discount, dec!(10.00));

        let fixed_discount = calculate_discount(
            subtotal,
            DiscountType::Fixed,
            dec!(15.00),
        );
        assert_eq!(fixed_discount, dec!(15.00));

        // Test max discount
        let large_discount = calculate_discount(
            subtotal,
            DiscountType::Fixed,
            dec!(150.00),
        );
        assert_eq!(large_discount, subtotal);
    }

    #[tokio::test]
    async fn test_product_category_validation() {
        use olympus_commerce::models::ProductCategory;

        assert!(ProductCategory::is_valid("food"));
        assert!(ProductCategory::is_valid("beverage"));
        assert!(ProductCategory::is_valid("merchandise"));
        assert!(ProductCategory::is_valid("service"));
        assert!(!ProductCategory::is_valid("invalid"));

        assert_eq!(ProductCategory::default_tax_rate("food"), dec!(0.05));
        assert_eq!(ProductCategory::default_tax_rate("beverage"), dec!(0.08));
        assert_eq!(ProductCategory::default_tax_rate("merchandise"), dec!(0.08));
    }

    #[tokio::test]
    async fn test_cart_operations() {
        use olympus_commerce::models::Cart;

        let mut cart = Cart::new(Uuid::new_v4(), Uuid::new_v4());

        cart.add_item(
            Uuid::new_v4(),
            2,
            dec!(25.00),
            Some(json!({"size": "large"})),
        );

        cart.add_item(
            Uuid::new_v4(),
            1,
            dec!(15.00),
            None,
        );

        assert_eq!(cart.item_count(), 2);
        assert_eq!(cart.total_quantity(), 3);
        assert_eq!(cart.subtotal(), dec!(65.00));

        cart.remove_item(0);
        assert_eq!(cart.item_count(), 1);
        assert_eq!(cart.subtotal(), dec!(15.00));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use testcontainers::{clients::Cli, images::postgres::Postgres};

    async fn setup_test_db() -> PgPool {
        let docker = Cli::default();
        let postgres_image = Postgres::default();
        let node = docker.run(postgres_image);

        let connection_string = format!(
            "postgresql://postgres:postgres@localhost:{}/postgres",
            node.get_host_port_ipv4(5432)
        );

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .unwrap();

        // Run migrations
        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_product_creation() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let product = product_service.create_product(
            tenant_id,
            "Cheeseburger",
            "Delicious beef burger with cheese",
            "food",
            dec!(12.99),
            dec!(0.08),
        ).await.unwrap();

        assert_eq!(product.name, "Cheeseburger");
        assert_eq!(product.price, dec!(12.99));
        assert_eq!(product.tax_rate, dec!(0.08));
        assert!(product.is_active);
    }

    #[tokio::test]
    async fn test_product_search() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());

        let tenant_id = Uuid::new_v4();

        // Create multiple products
        product_service.create_product(
            tenant_id,
            "Coffee",
            "Fresh brewed coffee",
            "beverage",
            dec!(3.50),
            dec!(0.08),
        ).await.unwrap();

        product_service.create_product(
            tenant_id,
            "Iced Coffee",
            "Cold brew coffee",
            "beverage",
            dec!(4.50),
            dec!(0.08),
        ).await.unwrap();

        product_service.create_product(
            tenant_id,
            "Sandwich",
            "Turkey sandwich",
            "food",
            dec!(8.99),
            dec!(0.05),
        ).await.unwrap();

        // Search by name
        let coffee_products = product_service.search_products(
            tenant_id,
            Some("coffee"),
            None,
        ).await.unwrap();
        assert_eq!(coffee_products.len(), 2);

        // Search by category
        let food_products = product_service.search_products(
            tenant_id,
            None,
            Some("food"),
        ).await.unwrap();
        assert_eq!(food_products.len(), 1);
    }

    #[tokio::test]
    async fn test_order_creation_flow() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());
        let order_service = OrderService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        // Create products
        let product1 = product_service.create_product(
            tenant_id,
            "Item 1",
            "Description 1",
            "food",
            dec!(10.00),
            dec!(0.08),
        ).await.unwrap();

        let product2 = product_service.create_product(
            tenant_id,
            "Item 2",
            "Description 2",
            "beverage",
            dec!(5.00),
            dec!(0.08),
        ).await.unwrap();

        // Create order
        let order_items = vec![
            (product1.id, 2, None),
            (product2.id, 3, None),
        ];

        let order = order_service.create_order(
            tenant_id,
            location_id,
            None, // No customer
            order_items,
        ).await.unwrap();

        assert_eq!(order.subtotal, dec!(35.00));
        assert_eq!(order.tax, dec!(2.80));
        assert_eq!(order.total, dec!(37.80));
        assert_eq!(order.items.len(), 2);
    }

    #[tokio::test]
    async fn test_order_status_updates() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());
        let order_service = OrderService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        let product = product_service.create_product(
            tenant_id,
            "Test Product",
            "Test",
            "food",
            dec!(10.00),
            dec!(0.08),
        ).await.unwrap();

        let order = order_service.create_order(
            tenant_id,
            location_id,
            None,
            vec![(product.id, 1, None)],
        ).await.unwrap();

        assert_eq!(order.status, "pending");

        // Update to processing
        let updated = order_service.update_order_status(
            order.id,
            "processing",
        ).await.unwrap();
        assert_eq!(updated.status, "processing");

        // Update to ready
        let updated = order_service.update_order_status(
            order.id,
            "ready",
        ).await.unwrap();
        assert_eq!(updated.status, "ready");

        // Update to completed
        let updated = order_service.update_order_status(
            order.id,
            "completed",
        ).await.unwrap();
        assert_eq!(updated.status, "completed");
    }

    #[tokio::test]
    async fn test_payment_processing() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());
        let order_service = OrderService::new(pool.clone());
        let payment_service = PaymentService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        let product = product_service.create_product(
            tenant_id,
            "Payment Test Product",
            "Test",
            "food",
            dec!(50.00),
            dec!(0.08),
        ).await.unwrap();

        let order = order_service.create_order(
            tenant_id,
            location_id,
            None,
            vec![(product.id, 1, None)],
        ).await.unwrap();

        // Process cash payment
        let payment = payment_service.process_payment(
            order.id,
            order.total,
            "cash",
            json!({"received": 60.00, "change": 6.00}),
        ).await.unwrap();

        assert_eq!(payment.amount, order.total);
        assert_eq!(payment.method, "cash");
        assert_eq!(payment.status, "completed");

        // Check order payment status
        let updated_order = order_service.get_order(order.id).await.unwrap();
        assert_eq!(updated_order.payment_status, "paid");
    }

    #[tokio::test]
    async fn test_inventory_management() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());
        let inventory_service = InventoryService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        let product = product_service.create_product(
            tenant_id,
            "Inventory Product",
            "Test",
            "merchandise",
            dec!(25.00),
            dec!(0.08),
        ).await.unwrap();

        // Initialize inventory
        let inventory = inventory_service.initialize_inventory(
            product.id,
            location_id,
            100,
            20,
            50,
        ).await.unwrap();

        assert_eq!(inventory.quantity_on_hand, 100);

        // Adjust inventory
        let adjusted = inventory_service.adjust_inventory(
            product.id,
            location_id,
            -10,
            "sale",
        ).await.unwrap();

        assert_eq!(adjusted.quantity_on_hand, 90);

        // Reserve inventory
        let reserved = inventory_service.reserve_inventory(
            product.id,
            location_id,
            15,
        ).await.unwrap();

        assert_eq!(reserved.quantity_reserved, 15);
        assert_eq!(reserved.available_quantity(), 75);
    }

    #[tokio::test]
    async fn test_low_stock_detection() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());
        let inventory_service = InventoryService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        // Create multiple products with different stock levels
        for i in 0..3 {
            let product = product_service.create_product(
                tenant_id,
                format!("Product {}", i),
                "Test",
                "merchandise",
                dec!(10.00),
                dec!(0.08),
            ).await.unwrap();

            let quantity = match i {
                0 => 5,   // Below reorder point
                1 => 25,  // At reorder point
                2 => 100, // Well stocked
                _ => 50,
            };

            inventory_service.initialize_inventory(
                product.id,
                location_id,
                quantity,
                20, // reorder point
                50, // reorder quantity
            ).await.unwrap();
        }

        let low_stock = inventory_service.get_low_stock_items(
            location_id,
        ).await.unwrap();

        assert_eq!(low_stock.len(), 2);
    }

    #[tokio::test]
    async fn test_refund_processing() {
        let pool = setup_test_db().await;
        let product_service = ProductService::new(pool.clone());
        let order_service = OrderService::new(pool.clone());
        let payment_service = PaymentService::new(pool.clone());

        let tenant_id = Uuid::new_v4();
        let location_id = Uuid::new_v4();

        let product = product_service.create_product(
            tenant_id,
            "Refund Test Product",
            "Test",
            "food",
            dec!(30.00),
            dec!(0.08),
        ).await.unwrap();

        let order = order_service.create_order(
            tenant_id,
            location_id,
            None,
            vec![(product.id, 2, None)],
        ).await.unwrap();

        let payment = payment_service.process_payment(
            order.id,
            order.total,
            "card",
            json!({"card_number": "****1234"}),
        ).await.unwrap();

        // Process refund
        let refund = payment_service.process_refund(
            payment.id,
            order.total,
            "Customer complaint",
        ).await.unwrap();

        assert_eq!(refund.amount, order.total);
        assert_eq!(refund.status, "completed");
        assert_eq!(refund.payment_type, "refund");

        // Check order status
        let refunded_order = order_service.get_order(order.id).await.unwrap();
        assert_eq!(refunded_order.status, "refunded");
        assert_eq!(refunded_order.payment_status, "refunded");
    }
}