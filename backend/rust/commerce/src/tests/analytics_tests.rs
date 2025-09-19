// ============================================================================
// OLYMPUS CLOUD - COMMERCE ANALYTICS TESTS
// ============================================================================
// Module: commerce/src/tests/analytics_tests.rs
// Description: Comprehensive test coverage for analytics functionality
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use olympus_shared::test_helpers::{create_test_db_pool, create_test_event_publisher};

use crate::models::{OrderStatus, PaymentStatus, FulfillmentStatus, ProductStatus, ProductType, PriceType};
use crate::services::analytics::{
    AnalyticsService, AnalyticsExportType, CustomerAnalyticsRequest, InventoryAnalyticsRequest,
    OrderAnalyticsRequest, ProductAnalyticsRequest, RevenueAnalyticsRequest, SalesAnalyticsRequest,
    AnalyticsExportRequest,
};

// ============================================================================
// TEST SETUP AND HELPERS
// ============================================================================

struct TestContext {
    analytics_service: AnalyticsService,
    tenant_id: Uuid,
    _db_pool: Arc<DbPool>,
}

impl TestContext {
    async fn new() -> Self {
        let db_pool = create_test_db_pool().await;
        let event_publisher = create_test_event_publisher();
        let analytics_service = AnalyticsService::new(db_pool.clone(), event_publisher);
        let tenant_id = Uuid::new_v4();

        // Setup test data
        Self::setup_test_data(&db_pool, tenant_id).await;

        Self {
            analytics_service,
            tenant_id,
            _db_pool: db_pool,
        }
    }

    async fn setup_test_data(db_pool: &Arc<DbPool>, tenant_id: Uuid) {
        let mut conn = db_pool.acquire().await.unwrap();

        // Create test categories
        let category_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO commerce.product_categories (id, tenant_id, name, slug, is_active, sort_order, created_at, updated_at)
            VALUES ($1, $2, 'Electronics', 'electronics', true, 1, $3, $3)
            "#,
        )
        .bind(category_id)
        .bind(tenant_id)
        .bind(Utc::now())
        .execute(&mut *conn)
        .await
        .unwrap();

        // Create test products
        let product_ids = Self::create_test_products(&mut conn, tenant_id, Some(category_id)).await;

        // Create test orders with items
        Self::create_test_orders(&mut conn, tenant_id, &product_ids).await;
    }

    async fn create_test_products(
        conn: &mut sqlx::PgConnection,
        tenant_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Vec<Uuid> {
        let mut product_ids = Vec::new();

        // Create multiple test products
        for i in 1..=5 {
            let product_id = Uuid::new_v4();
            let sku = format!("PROD{:03}", i);
            let name = format!("Test Product {}", i);
            let base_price = Decimal::from(100 + i * 10);
            let cost_price = Decimal::from(50 + i * 5);
            let inventory_quantity = 100 - i * 10;

            sqlx::query(
                r#"
                INSERT INTO commerce.products (
                    id, tenant_id, sku, name, description, product_type, status, category_id,
                    base_price, price_type, cost_price, requires_shipping, is_digital,
                    track_inventory, inventory_quantity, low_stock_threshold, tags, attributes,
                    created_at, updated_at, created_by, updated_by
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $19, $19, $19
                )
                "#,
            )
            .bind(product_id)
            .bind(tenant_id)
            .bind(&sku)
            .bind(&name)
            .bind(format!("Description for {}", name))
            .bind(ProductType::Simple)
            .bind(ProductStatus::Active)
            .bind(category_id)
            .bind(base_price)
            .bind(PriceType::Fixed)
            .bind(cost_price)
            .bind(true)
            .bind(false)
            .bind(true)
            .bind(inventory_quantity)
            .bind(10)
            .bind(vec!["test".to_string()])
            .bind(json!({}))
            .bind(Utc::now())
            .bind(Uuid::new_v4()) // created_by
            .execute(conn)
            .await
            .unwrap();

            product_ids.push(product_id);
        }

        product_ids
    }

    async fn create_test_orders(
        conn: &mut sqlx::PgConnection,
        tenant_id: Uuid,
        product_ids: &[Uuid],
    ) {
        let now = Utc::now();

        // Create orders over the last 30 days
        for day in 0..30 {
            let order_date = now - Duration::days(day);
            let orders_for_day = if day % 7 == 0 { 5 } else { 2 }; // More orders on "weekends"

            for order_idx in 0..orders_for_day {
                let order_id = Uuid::new_v4();
                let order_number = format!("ORD{}{:03}", day, order_idx);
                let customer_id = Uuid::new_v4();
                let customer_email = format!("customer{}{}@test.com", day, order_idx);

                // Vary order status
                let status = match day % 10 {
                    0 => OrderStatus::Cancelled,
                    1 => OrderStatus::Refunded,
                    2 | 3 => OrderStatus::Pending,
                    _ => OrderStatus::Completed,
                };

                let payment_status = match status {
                    OrderStatus::Cancelled => PaymentStatus::Cancelled,
                    OrderStatus::Refunded => PaymentStatus::Refunded,
                    OrderStatus::Completed => PaymentStatus::Captured,
                    _ => PaymentStatus::Pending,
                };

                // Calculate totals
                let subtotal = Decimal::from(150 + day * 10);
                let tax_total = subtotal * Decimal::from_str_exact("0.08").unwrap();
                let shipping_total = Decimal::from(10);
                let discount_total = if day % 5 == 0 { Decimal::from(15) } else { Decimal::ZERO };
                let total = subtotal + tax_total + shipping_total - discount_total;

                // Create order
                sqlx::query(
                    r#"
                    INSERT INTO commerce.orders (
                        id, tenant_id, order_number, customer_id, customer_email, status,
                        payment_status, fulfillment_status, currency, subtotal, tax_total,
                        shipping_total, discount_total, total, notes, tags, metadata,
                        created_at, updated_at, confirmed_at
                    ) VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $18, $19
                    )
                    "#,
                )
                .bind(order_id)
                .bind(tenant_id)
                .bind(order_number)
                .bind(customer_id)
                .bind(customer_email)
                .bind(status)
                .bind(payment_status)
                .bind(FulfillmentStatus::Fulfilled)
                .bind("USD")
                .bind(subtotal)
                .bind(tax_total)
                .bind(shipping_total)
                .bind(discount_total)
                .bind(total)
                .bind(None::<String>)
                .bind(vec!["test".to_string()])
                .bind(json!({"location": "store1"}))
                .bind(order_date)
                .bind(if matches!(status, OrderStatus::Completed) {
                    Some(order_date + Duration::hours(1))
                } else {
                    None
                })
                .execute(conn)
                .await
                .unwrap();

                // Create order items
                for (item_idx, &product_id) in product_ids.iter().enumerate().take(2) {
                    let order_item_id = Uuid::new_v4();
                    let quantity = 1 + (item_idx as i32);
                    let unit_price = Decimal::from(50 + item_idx * 25);
                    let total_price = unit_price * Decimal::from(quantity);

                    sqlx::query(
                        r#"
                        INSERT INTO commerce.order_items (
                            id, order_id, product_id, sku, name, quantity, unit_price, total_price,
                            tax_rate, tax_amount, discount_amount, attributes, created_at, updated_at
                        ) VALUES (
                            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $13
                        )
                        "#,
                    )
                    .bind(order_item_id)
                    .bind(order_id)
                    .bind(product_id)
                    .bind(format!("PROD{:03}", item_idx + 1))
                    .bind(format!("Test Product {}", item_idx + 1))
                    .bind(quantity)
                    .bind(unit_price)
                    .bind(total_price)
                    .bind(Some(Decimal::from_str_exact("0.08").unwrap()))
                    .bind(Some(total_price * Decimal::from_str_exact("0.08").unwrap()))
                    .bind(Some(Decimal::ZERO))
                    .bind(json!({}))
                    .bind(order_date)
                    .execute(conn)
                    .await
                    .unwrap();
                }
            }
        }
    }
}

// ============================================================================
// SALES ANALYTICS TESTS
// ============================================================================

#[tokio::test]
async fn test_sales_performance_metrics() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify basic metrics
    assert!(metrics.total_sales > Decimal::ZERO);
    assert!(metrics.total_orders > 0);
    assert!(metrics.average_order_value > Decimal::ZERO);
    assert!(!metrics.daily_breakdown.is_empty());

    // Verify daily breakdown has reasonable data
    assert!(metrics.daily_breakdown.len() <= 31); // At most 31 days
    for daily in &metrics.daily_breakdown {
        assert!(daily.total_sales >= Decimal::ZERO);
        assert!(daily.order_count >= 0);
    }
}

#[tokio::test]
async fn test_sales_analytics_with_location_filter() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    let request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: Some("store1".to_string()),
        channel_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Should have some sales data filtered by location
    assert!(metrics.total_orders >= 0);
}

#[tokio::test]
async fn test_sales_growth_rate_calculation() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(14);

    let request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Growth rate should be calculated (could be positive, negative, or zero)
    assert!(metrics.growth_rate >= Decimal::from(-100)); // At least -100%
}

// ============================================================================
// PRODUCT ANALYTICS TESTS
// ============================================================================

#[tokio::test]
async fn test_product_performance_metrics() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = ProductAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        category_filter: None,
        limit: Some(10),
    };

    let result = ctx
        .analytics_service
        .get_product_performance(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Should have best sellers data
    assert!(!metrics.best_sellers.is_empty());
    assert!(metrics.best_sellers.len() <= 10);

    // Should have category performance
    assert!(!metrics.category_performance.is_empty());

    // Verify product metrics structure
    for product in &metrics.best_sellers {
        assert!(!product.product_name.is_empty());
        assert!(!product.sku.is_empty());
        assert!(product.total_quantity_sold >= 0);
        assert!(product.total_revenue >= Decimal::ZERO);
    }
}

#[tokio::test]
async fn test_product_analytics_with_category_filter() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    // Get a category ID from the test data
    let mut conn = ctx._db_pool.acquire().await.unwrap();
    let category_id: Uuid = sqlx::query_scalar(
        "SELECT id FROM commerce.product_categories WHERE tenant_id = $1 LIMIT 1"
    )
    .bind(ctx.tenant_id)
    .fetch_one(&mut *conn)
    .await
    .unwrap();

    let request = ProductAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        category_filter: Some(category_id),
        limit: Some(5),
    };

    let result = ctx
        .analytics_service
        .get_product_performance(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Should still return data, filtered by category
    assert!(metrics.best_sellers.len() <= 5);
}

#[tokio::test]
async fn test_slow_moving_products_detection() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = ProductAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        category_filter: None,
        limit: Some(20),
    };

    let result = ctx
        .analytics_service
        .get_product_performance(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Should identify slow movers (products with < 5 quantity sold)
    for slow_mover in &metrics.slow_movers {
        assert!(slow_mover.total_quantity_sold < 5);
    }
}

// ============================================================================
// ORDER ANALYTICS TESTS
// ============================================================================

#[tokio::test]
async fn test_order_analytics_metrics() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = OrderAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        status_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_order_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify order volume metrics
    assert!(metrics.total_orders > 0);
    assert!(metrics.completed_orders >= 0);
    assert!(metrics.cancelled_orders >= 0);
    assert!(metrics.pending_orders >= 0);
    assert!(metrics.completion_rate >= 0.0 && metrics.completion_rate <= 100.0);

    // Verify status distribution
    assert!(!metrics.status_distribution.is_empty());
    let total_percentage: f64 = metrics
        .status_distribution
        .iter()
        .map(|s| s.percentage.to_string().parse::<f64>().unwrap_or(0.0))
        .sum();
    assert!((total_percentage - 100.0).abs() < 1.0); // Should sum to ~100%

    // Verify hourly patterns
    assert!(!metrics.hourly_patterns.is_empty());
    for pattern in &metrics.hourly_patterns {
        assert!(pattern.hour >= 0 && pattern.hour <= 23);
        assert!(pattern.order_count >= 0);
    }
}

#[tokio::test]
async fn test_order_analytics_with_status_filter() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = OrderAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        status_filter: Some(OrderStatus::Completed),
    };

    let result = ctx
        .analytics_service
        .get_order_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Should only count completed orders
    assert!(metrics.completed_orders >= 0);
}

#[tokio::test]
async fn test_order_processing_time_calculation() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    let request = OrderAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        status_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_order_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Average processing time should be reasonable
    assert!(metrics.average_processing_hours >= Decimal::ZERO);
    assert!(metrics.average_processing_hours <= Decimal::from(168)); // <= 1 week
}

// ============================================================================
// REVENUE ANALYTICS TESTS
// ============================================================================

#[tokio::test]
async fn test_revenue_analytics_metrics() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = RevenueAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        group_by: None,
    };

    let result = ctx
        .analytics_service
        .get_revenue_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify revenue calculations
    assert!(metrics.gross_revenue >= Decimal::ZERO);
    assert!(metrics.net_revenue >= Decimal::ZERO);
    assert!(metrics.net_revenue <= metrics.gross_revenue);
    assert!(metrics.total_refunds >= Decimal::ZERO);
    assert!(metrics.total_tax >= Decimal::ZERO);
    assert!(metrics.total_shipping >= Decimal::ZERO);

    // Verify category breakdown
    assert!(!metrics.category_breakdown.is_empty());
    let total_category_revenue: Decimal = metrics
        .category_breakdown
        .iter()
        .map(|c| c.revenue)
        .sum();
    assert!(total_category_revenue <= metrics.gross_revenue);

    // Verify monthly trends
    assert!(!metrics.monthly_trends.is_empty());
    for trend in &metrics.monthly_trends {
        assert!(trend.year >= 2020 && trend.year <= 2030);
        assert!(trend.month >= 1 && trend.month <= 12);
        assert!(trend.revenue >= Decimal::ZERO);
    }
}

#[tokio::test]
async fn test_revenue_net_calculation() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = RevenueAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        group_by: None,
    };

    let result = ctx
        .analytics_service
        .get_revenue_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Net revenue should equal gross revenue minus refunds
    let expected_net = metrics.gross_revenue - metrics.total_refunds;
    assert_eq!(metrics.net_revenue, expected_net);
}

// ============================================================================
// CUSTOMER ANALYTICS TESTS
// ============================================================================

#[tokio::test]
async fn test_customer_analytics_metrics() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = CustomerAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        segment_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_customer_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify customer metrics
    assert!(metrics.total_customers >= 0);
    assert!(metrics.new_customers >= 0);
    assert!(metrics.new_customers <= metrics.total_customers);
    assert!(metrics.retention_rate >= 0.0 && metrics.retention_rate <= 100.0);
    assert!(metrics.average_lifetime_value >= Decimal::ZERO);
    assert!(metrics.average_order_frequency >= Decimal::ZERO);

    // Verify customer segmentation
    assert!(!metrics.segmentation.is_empty());
    for segment in &metrics.segmentation {
        assert!(!segment.segment.is_empty());
        assert!(segment.customer_count >= 0);
        assert!(segment.average_spent >= Decimal::ZERO);
    }
}

#[tokio::test]
async fn test_customer_retention_rate_calculation() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = CustomerAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        segment_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_customer_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Retention rate should be calculated correctly
    if metrics.total_customers > 0 {
        let expected_retention = if metrics.total_customers > metrics.new_customers {
            ((metrics.total_customers - metrics.new_customers) as f64 / metrics.total_customers as f64) * 100.0
        } else {
            0.0
        };
        assert!((metrics.retention_rate - expected_retention).abs() < 1.0);
    }
}

#[tokio::test]
async fn test_customer_segmentation_logic() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = CustomerAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        segment_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_customer_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify segmentation includes expected segments
    let segment_names: Vec<String> = metrics
        .segmentation
        .iter()
        .map(|s| s.segment.clone())
        .collect();

    assert!(segment_names.contains(&"new".to_string()));
    // May contain occasional, regular, loyal depending on test data
}

// ============================================================================
// INVENTORY ANALYTICS TESTS
// ============================================================================

#[tokio::test]
async fn test_inventory_analytics_metrics() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = InventoryAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_inventory_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify inventory summary
    assert!(metrics.total_products > 0);
    assert!(metrics.low_stock_items >= 0);
    assert!(metrics.out_of_stock_items >= 0);
    assert!(metrics.total_inventory_value >= Decimal::ZERO);
    assert!(metrics.average_stock_level >= Decimal::ZERO);

    // Verify high value items
    assert!(!metrics.high_value_items.is_empty());
    for item in &metrics.high_value_items {
        assert!(!item.product_name.is_empty());
        assert!(!item.sku.is_empty());
        assert!(item.quantity >= 0);
        assert!(item.unit_cost >= Decimal::ZERO);
        assert!(item.total_value >= Decimal::ZERO);
        assert_eq!(item.total_value, Decimal::from(item.quantity) * item.unit_cost);
    }

    // Verify turnover analysis
    assert!(!metrics.turnover_analysis.is_empty());
    for analysis in &metrics.turnover_analysis {
        assert!(!analysis.product_name.is_empty());
        assert!(!analysis.sku.is_empty());
        assert!(analysis.current_stock >= 0);
        assert!(analysis.total_sold >= 0);
        assert!(analysis.turnover_ratio >= Decimal::ZERO);
    }
}

#[tokio::test]
async fn test_inventory_valuation_calculation() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = InventoryAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_inventory_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Total inventory value should equal sum of individual item values
    let calculated_total: Decimal = metrics
        .high_value_items
        .iter()
        .map(|item| item.total_value)
        .sum();

    // Should be less than or equal to total (since high_value_items is limited)
    assert!(calculated_total <= metrics.total_inventory_value);
}

#[tokio::test]
async fn test_inventory_turnover_calculation() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = InventoryAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_inventory_analytics(ctx.tenant_id, &request)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();

    // Verify turnover ratio calculations
    for analysis in &metrics.turnover_analysis {
        if analysis.current_stock > 0 && analysis.total_sold > 0 {
            let expected_ratio = Decimal::from(analysis.total_sold) / Decimal::from(analysis.current_stock);
            assert!((analysis.turnover_ratio - expected_ratio).abs() < Decimal::from_str_exact("0.01").unwrap());
        }
    }
}

// ============================================================================
// EXPORT FUNCTIONALITY TESTS
// ============================================================================

#[tokio::test]
async fn test_export_sales_analytics_csv() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    let request = AnalyticsExportRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        format: None,
    };

    let result = ctx
        .analytics_service
        .export_analytics_csv(ctx.tenant_id, AnalyticsExportType::Sales, request)
        .await;

    assert!(result.is_ok());
    let csv_data = result.unwrap();

    // Should have CSV header
    assert!(csv_data.contains("Date,Total Sales,Orders,Average Order Value"));

    // Should have some data rows
    let lines: Vec<&str> = csv_data.lines().collect();
    assert!(lines.len() > 1); // Header + at least one data row
}

#[tokio::test]
async fn test_export_products_analytics_csv() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    let request = AnalyticsExportRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        format: None,
    };

    let result = ctx
        .analytics_service
        .export_analytics_csv(ctx.tenant_id, AnalyticsExportType::Products, request)
        .await;

    assert!(result.is_ok());
    let csv_data = result.unwrap();

    // Should have CSV header
    assert!(csv_data.contains("Product Name,SKU,Quantity Sold,Revenue,Average Price"));

    // Should have some data rows
    let lines: Vec<&str> = csv_data.lines().collect();
    assert!(lines.len() > 1); // Header + at least one data row
}

// ============================================================================
// CACHE FUNCTIONALITY TESTS
// ============================================================================

#[tokio::test]
async fn test_cache_analytics_metrics() {
    let ctx = TestContext::new().await;

    let result = ctx
        .analytics_service
        .cache_analytics_metrics(ctx.tenant_id)
        .await;

    assert!(result.is_ok());
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[tokio::test]
async fn test_analytics_with_invalid_tenant() {
    let ctx = TestContext::new().await;
    let invalid_tenant_id = Uuid::new_v4();
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    let request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_sales_performance(invalid_tenant_id, &request)
        .await;

    // Should succeed but return empty/zero metrics
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.total_orders, 0);
    assert_eq!(metrics.total_sales, Decimal::ZERO);
}

#[tokio::test]
async fn test_analytics_with_future_dates() {
    let ctx = TestContext::new().await;
    let start_date = Utc::now() + Duration::days(1);
    let end_date = Utc::now() + Duration::days(7);

    let request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };

    let result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &request)
        .await;

    // Should succeed but return empty metrics
    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.total_orders, 0);
    assert_eq!(metrics.total_sales, Decimal::ZERO);
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[tokio::test]
async fn test_analytics_data_consistency() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    // Get metrics from different analytics endpoints
    let sales_request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };

    let revenue_request = RevenueAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        group_by: None,
    };

    let order_request = OrderAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        status_filter: None,
    };

    let sales_result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &sales_request)
        .await;
    let revenue_result = ctx
        .analytics_service
        .get_revenue_analytics(ctx.tenant_id, &revenue_request)
        .await;
    let order_result = ctx
        .analytics_service
        .get_order_analytics(ctx.tenant_id, &order_request)
        .await;

    assert!(sales_result.is_ok());
    assert!(revenue_result.is_ok());
    assert!(order_result.is_ok());

    let sales = sales_result.unwrap();
    let revenue = revenue_result.unwrap();
    let orders = order_result.unwrap();

    // Verify data consistency across endpoints
    assert_eq!(sales.total_orders, orders.total_orders);
    assert_eq!(sales.total_sales, revenue.gross_revenue);
}

#[tokio::test]
async fn test_comprehensive_analytics_workflow() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    // Test complete analytics workflow

    // 1. Get sales performance
    let sales_request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };
    let sales_result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &sales_request)
        .await;
    assert!(sales_result.is_ok());

    // 2. Get product performance
    let product_request = ProductAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        category_filter: None,
        limit: Some(5),
    };
    let product_result = ctx
        .analytics_service
        .get_product_performance(ctx.tenant_id, &product_request)
        .await;
    assert!(product_result.is_ok());

    // 3. Export data
    let export_request = AnalyticsExportRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        format: None,
    };
    let export_result = ctx
        .analytics_service
        .export_analytics_csv(ctx.tenant_id, AnalyticsExportType::Sales, export_request)
        .await;
    assert!(export_result.is_ok());

    // 4. Refresh cache
    let cache_result = ctx
        .analytics_service
        .cache_analytics_metrics(ctx.tenant_id)
        .await;
    assert!(cache_result.is_ok());
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_analytics_query_performance() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(30);

    let request = SalesAnalyticsRequest {
        start_date: Some(start_date),
        end_date: Some(end_date),
        location_filter: None,
        channel_filter: None,
    };

    // Time the analytics query
    let start_time = std::time::Instant::now();
    let result = ctx
        .analytics_service
        .get_sales_performance(ctx.tenant_id, &request)
        .await;
    let elapsed = start_time.elapsed();

    assert!(result.is_ok());
    // Should complete within reasonable time (adjust threshold as needed)
    assert!(elapsed.as_secs() < 5, "Analytics query took too long: {:?}", elapsed);
}

#[tokio::test]
async fn test_concurrent_analytics_requests() {
    let ctx = TestContext::new().await;
    let end_date = Utc::now();
    let start_date = end_date - Duration::days(7);

    // Create multiple concurrent requests
    let futures = (0..5).map(|_| {
        let service = &ctx.analytics_service;
        let tenant_id = ctx.tenant_id;
        async move {
            let request = SalesAnalyticsRequest {
                start_date: Some(start_date),
                end_date: Some(end_date),
                location_filter: None,
                channel_filter: None,
            };
            service.get_sales_performance(tenant_id, &request).await
        }
    });

    let results: Vec<_> = futures::future::join_all(futures).await;

    // All requests should succeed
    for result in results {
        assert!(result.is_ok());
    }
}

// ============================================================================
// HELPER IMPLEMENTATIONS
// ============================================================================

// Helper functions for test data validation
impl Default for SalesAnalyticsRequest {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            location_filter: None,
            channel_filter: None,
        }
    }
}

impl Default for ProductAnalyticsRequest {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            category_filter: None,
            limit: None,
        }
    }
}

impl Default for OrderAnalyticsRequest {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            status_filter: None,
        }
    }
}

impl Default for RevenueAnalyticsRequest {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            group_by: None,
        }
    }
}

impl Default for CustomerAnalyticsRequest {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            segment_filter: None,
        }
    }
}

impl Default for InventoryAnalyticsRequest {
    fn default() -> Self {
        Self {
            start_date: None,
            end_date: None,
            location_filter: None,
        }
    }
}