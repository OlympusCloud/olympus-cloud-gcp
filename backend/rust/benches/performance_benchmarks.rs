//! Performance benchmarks for Olympus Cloud services

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use olympus_auth::utils::password;
use olympus_auth::services::TokenService;
use olympus_shared::database::DatabaseConnection;
use sqlx::PgPool;
use uuid::Uuid;
use tokio::runtime::Runtime;

fn benchmark_password_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_hashing");

    for password_len in [8, 16, 32, 64].iter() {
        let password = "a".repeat(*password_len);

        group.bench_with_input(
            BenchmarkId::new("hash", password_len),
            &password,
            |b, pwd| {
                b.iter(|| {
                    password::hash(black_box(pwd)).unwrap()
                });
            },
        );
    }

    let password = "TestPassword123!";
    let hash = password::hash(password).unwrap();

    group.bench_function("verify", |b| {
        b.iter(|| {
            password::verify(black_box(password), black_box(&hash)).unwrap()
        });
    });

    group.finish();
}

fn benchmark_jwt_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("jwt_operations");

    let token_service = TokenService::new("test_secret_key_32_bytes_long!!!!");
    let user_id = Uuid::new_v4();
    let tenant_id = Uuid::new_v4();
    let roles = vec!["user".to_string(), "admin".to_string()];

    group.bench_function("generate_access_token", |b| {
        b.iter(|| {
            token_service.generate_access_token(
                black_box(user_id),
                black_box(tenant_id),
                black_box(&roles),
            ).unwrap()
        });
    });

    let token = token_service.generate_access_token(user_id, tenant_id, &roles).unwrap();

    group.bench_function("validate_access_token", |b| {
        b.iter(|| {
            token_service.validate_access_token(black_box(&token)).unwrap()
        });
    });

    group.finish();
}

fn benchmark_database_queries(c: &mut Criterion) {
    let runtime = Runtime::new().unwrap();
    let pool = runtime.block_on(async {
        PgPool::connect(&std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:postgres@localhost:5432/olympus_bench".to_string()
        }))
        .await
        .expect("Failed to connect to database")
    });

    let mut group = c.benchmark_group("database_queries");

    // Benchmark simple SELECT
    group.bench_function("simple_select", |b| {
        b.to_async(&runtime).iter(|| async {
            sqlx::query!("SELECT 1 as value")
                .fetch_one(&pool)
                .await
                .unwrap();
        });
    });

    // Benchmark user lookup
    let user_id = Uuid::new_v4();
    group.bench_function("user_lookup", |b| {
        b.to_async(&runtime).iter(|| async {
            sqlx::query!(
                "SELECT id, email, tenant_id FROM auth.users WHERE id = $1",
                user_id
            )
            .fetch_optional(&pool)
            .await
            .unwrap();
        });
    });

    // Benchmark complex join query
    group.bench_function("complex_join", |b| {
        b.to_async(&runtime).iter(|| async {
            sqlx::query!(
                r#"
                SELECT
                    o.id,
                    o.order_number,
                    o.total,
                    COUNT(oi.id) as item_count
                FROM commerce.orders o
                LEFT JOIN commerce.order_items oi ON o.id = oi.order_id
                WHERE o.created_at > NOW() - INTERVAL '7 days'
                GROUP BY o.id, o.order_number, o.total
                LIMIT 100
                "#
            )
            .fetch_all(&pool)
            .await
            .unwrap();
        });
    });

    group.finish();
}

fn benchmark_order_calculations(c: &mut Criterion) {
    use olympus_commerce::utils::{calculate_price_with_tax, calculate_discount, DiscountType};
    use rust_decimal_macros::dec;

    let mut group = c.benchmark_group("order_calculations");

    group.bench_function("tax_calculation", |b| {
        let price = dec!(99.99);
        let tax_rate = dec!(0.08);

        b.iter(|| {
            calculate_price_with_tax(black_box(price), black_box(tax_rate))
        });
    });

    group.bench_function("percentage_discount", |b| {
        let subtotal = dec!(150.00);
        let discount = dec!(15.00); // 15%

        b.iter(|| {
            calculate_discount(
                black_box(subtotal),
                black_box(DiscountType::Percentage),
                black_box(discount),
            )
        });
    });

    // Benchmark order total calculation with multiple items
    group.bench_function("order_total_calculation", |b| {
        use olympus_commerce::models::Cart;

        b.iter(|| {
            let mut cart = Cart::new(Uuid::new_v4(), Uuid::new_v4());

            // Add multiple items
            for i in 0..10 {
                cart.add_item(
                    Uuid::new_v4(),
                    2,
                    dec!(10.00) + dec!(i),
                    None,
                );
            }

            let subtotal = cart.subtotal();
            let tax = subtotal * dec!(0.08);
            let total = subtotal + tax;

            black_box(total)
        });
    });

    group.finish();
}

fn benchmark_serialization(c: &mut Criterion) {
    use serde_json;
    use olympus_commerce::models::Product;
    use chrono::Utc;

    let mut group = c.benchmark_group("serialization");

    let product = Product {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        sku: "PROD-001".to_string(),
        name: "Test Product".to_string(),
        description: Some("A test product for benchmarking".to_string()),
        category: "test".to_string(),
        price: rust_decimal::Decimal::from(99.99),
        tax_rate: rust_decimal::Decimal::from(0.08),
        is_active: true,
        metadata: serde_json::json!({"tags": ["new", "featured"]}),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    group.bench_function("product_serialize", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&product)).unwrap()
        });
    });

    let json = serde_json::to_string(&product).unwrap();

    group.bench_function("product_deserialize", |b| {
        b.iter(|| {
            let _: Product = serde_json::from_str(black_box(&json)).unwrap();
        });
    });

    group.finish();
}

fn benchmark_event_publishing(c: &mut Criterion) {
    use olympus_shared::events::{DomainEvent, EventPublisher};
    use std::sync::Arc;

    let runtime = Runtime::new().unwrap();
    let redis_client = runtime.block_on(async {
        redis::Client::open(
            std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string())
        ).expect("Failed to create Redis client")
    });

    let publisher = Arc::new(EventPublisher::new(redis_client.clone()));

    let mut group = c.benchmark_group("event_publishing");

    let event = DomainEvent::UserRegistered {
        user_id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        email: "test@example.com".to_string(),
    };

    group.bench_function("publish_single_event", |b| {
        b.to_async(&runtime).iter(|| {
            let pub_clone = publisher.clone();
            let evt_clone = event.clone();
            async move {
                pub_clone.publish(black_box(evt_clone)).await.unwrap();
            }
        });
    });

    group.bench_function("publish_batch_events", |b| {
        let events: Vec<DomainEvent> = (0..10).map(|_| {
            DomainEvent::OrderCreated {
                order_id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                customer_id: Some(Uuid::new_v4()),
                total: rust_decimal::Decimal::from(100.00),
            }
        }).collect();

        b.to_async(&runtime).iter(|| {
            let pub_clone = publisher.clone();
            let evts_clone = events.clone();
            async move {
                pub_clone.publish_batch(black_box(evts_clone)).await.unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_password_hashing,
    benchmark_jwt_operations,
    benchmark_database_queries,
    benchmark_order_calculations,
    benchmark_serialization,
    benchmark_event_publishing
);

criterion_main!(benches);