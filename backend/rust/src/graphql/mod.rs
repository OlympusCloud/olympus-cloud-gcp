//! GraphQL API layer for complex queries

use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub mod resolvers;
pub mod types;

use crate::auth::services::AuthService;
use crate::commerce::services::{OrderService, ProductService};
use crate::platform::services::{LocationService, TenantService};

/// Root query object for GraphQL schema
pub struct Query;

#[Object]
impl Query {
    /// Get current user information
    async fn me(&self, ctx: &Context<'_>) -> async_graphql::Result<User> {
        let user_id = ctx.data::<Uuid>()?;
        let pool = ctx.data::<PgPool>()?;

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, tenant_id, email, first_name, last_name,
                   roles as "roles: Vec<String>", created_at, updated_at
            FROM auth.users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Get tenant information
    async fn tenant(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<Tenant> {
        let tenant_id = Uuid::parse_str(&id)?;
        let pool = ctx.data::<PgPool>()?;

        let tenant = sqlx::query_as!(
            Tenant,
            r#"
            SELECT id, slug, name, industry, tier,
                   settings, features, branding,
                   created_at, updated_at
            FROM platform.tenants
            WHERE id = $1
            "#,
            tenant_id
        )
        .fetch_one(pool)
        .await?;

        Ok(tenant)
    }

    /// List products with filtering
    async fn products(
        &self,
        ctx: &Context<'_>,
        tenant_id: ID,
        category: Option<String>,
        search: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> async_graphql::Result<ProductConnection> {
        let tid = Uuid::parse_str(&tenant_id)?;
        let pool = ctx.data::<PgPool>()?;
        let limit = limit.unwrap_or(20).min(100);
        let offset = offset.unwrap_or(0);

        let mut query = sqlx::query_builder::QueryBuilder::new(
            "SELECT * FROM commerce.products WHERE tenant_id = "
        );
        query.push_bind(tid);

        if let Some(cat) = category {
            query.push(" AND category = ");
            query.push_bind(cat);
        }

        if let Some(search_term) = search {
            query.push(" AND (name ILIKE ");
            query.push_bind(format!("%{}%", search_term));
            query.push(" OR description ILIKE ");
            query.push_bind(format!("%{}%", search_term));
            query.push(")");
        }

        query.push(" ORDER BY created_at DESC");
        query.push(" LIMIT ");
        query.push_bind(limit);
        query.push(" OFFSET ");
        query.push_bind(offset);

        let products: Vec<Product> = query
            .build_query_as()
            .fetch_all(pool)
            .await?;

        // Get total count
        let count: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM commerce.products WHERE tenant_id = $1",
            tid
        )
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

        Ok(ProductConnection {
            edges: products.into_iter().map(|p| ProductEdge {
                cursor: p.id.to_string(),
                node: p,
            }).collect(),
            page_info: PageInfo {
                has_next_page: (offset + limit as i32) < count as i32,
                has_previous_page: offset > 0,
                start_cursor: Some(offset.to_string()),
                end_cursor: Some((offset + limit as i32).to_string()),
            },
            total_count: count as i32,
        })
    }

    /// Get order with all details
    async fn order(&self, ctx: &Context<'_>, id: ID) -> async_graphql::Result<OrderDetail> {
        let order_id = Uuid::parse_str(&id)?;
        let pool = ctx.data::<PgPool>()?;

        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id, tenant_id, location_id, customer_id,
                   order_number, status, subtotal, tax, total,
                   payment_status, notes, created_at, updated_at
            FROM commerce.orders
            WHERE id = $1
            "#,
            order_id
        )
        .fetch_one(pool)
        .await?;

        let items = sqlx::query_as!(
            OrderItem,
            r#"
            SELECT oi.id, oi.order_id, oi.product_id, oi.quantity,
                   oi.unit_price, oi.total, oi.modifiers,
                   p.name as product_name, p.sku as product_sku
            FROM commerce.order_items oi
            JOIN commerce.products p ON oi.product_id = p.id
            WHERE oi.order_id = $1
            "#,
            order_id
        )
        .fetch_all(pool)
        .await?;

        let payments = sqlx::query_as!(
            Payment,
            r#"
            SELECT id, order_id, amount, method, status,
                   transaction_id, metadata, created_at
            FROM commerce.payments
            WHERE order_id = $1
            "#,
            order_id
        )
        .fetch_all(pool)
        .await?;

        Ok(OrderDetail {
            order,
            items,
            payments,
        })
    }

    /// Analytics dashboard data
    async fn analytics(
        &self,
        ctx: &Context<'_>,
        tenant_id: ID,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> async_graphql::Result<AnalyticsSummary> {
        let tid = Uuid::parse_str(&tenant_id)?;
        let pool = ctx.data::<PgPool>()?;

        // Revenue metrics
        let revenue = sqlx::query!(
            r#"
            SELECT
                SUM(total) as total_revenue,
                AVG(total) as avg_order_value,
                COUNT(*) as order_count
            FROM commerce.orders
            WHERE tenant_id = $1
                AND created_at BETWEEN $2 AND $3
                AND status = 'completed'
            "#,
            tid,
            start_date,
            end_date
        )
        .fetch_one(pool)
        .await?;

        // Top products
        let top_products = sqlx::query_as!(
            TopProduct,
            r#"
            SELECT
                p.id,
                p.name,
                p.sku,
                SUM(oi.quantity) as quantity_sold,
                SUM(oi.total) as revenue
            FROM commerce.order_items oi
            JOIN commerce.orders o ON oi.order_id = o.id
            JOIN commerce.products p ON oi.product_id = p.id
            WHERE o.tenant_id = $1
                AND o.created_at BETWEEN $2 AND $3
                AND o.status = 'completed'
            GROUP BY p.id, p.name, p.sku
            ORDER BY revenue DESC
            LIMIT 10
            "#,
            tid,
            start_date,
            end_date
        )
        .fetch_all(pool)
        .await?;

        // Customer metrics
        let customers = sqlx::query!(
            r#"
            SELECT
                COUNT(DISTINCT customer_id) as unique_customers,
                COUNT(DISTINCT customer_id) FILTER (
                    WHERE customer_id IN (
                        SELECT customer_id
                        FROM commerce.orders
                        WHERE created_at < $2
                        GROUP BY customer_id
                    )
                ) as returning_customers
            FROM commerce.orders
            WHERE tenant_id = $1
                AND created_at BETWEEN $2 AND $3
                AND customer_id IS NOT NULL
            "#,
            tid,
            start_date,
            end_date
        )
        .fetch_one(pool)
        .await?;

        Ok(AnalyticsSummary {
            revenue: RevenueMetrics {
                total: revenue.total_revenue.unwrap_or(Decimal::ZERO),
                average_order_value: revenue.avg_order_value.unwrap_or(Decimal::ZERO),
                order_count: revenue.order_count.unwrap_or(0) as i32,
            },
            top_products,
            customers: CustomerMetrics {
                unique_count: customers.unique_customers.unwrap_or(0) as i32,
                returning_count: customers.returning_customers.unwrap_or(0) as i32,
            },
            period: AnalyticsPeriod {
                start: start_date,
                end: end_date,
            },
        })
    }
}

// GraphQL Types
#[derive(SimpleObject, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct Tenant {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub industry: String,
    pub tier: String,
    pub settings: serde_json::Value,
    pub features: serde_json::Value,
    pub branding: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Decimal,
    pub tax_rate: Decimal,
    pub is_active: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
pub struct ProductConnection {
    pub edges: Vec<ProductEdge>,
    pub page_info: PageInfo,
    pub total_count: i32,
}

#[derive(SimpleObject)]
pub struct ProductEdge {
    pub cursor: String,
    pub node: Product,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub location_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub order_number: String,
    pub status: String,
    pub subtotal: Decimal,
    pub tax: Decimal,
    pub total: Decimal,
    pub payment_status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_sku: String,
    pub quantity: i32,
    pub unit_price: Decimal,
    pub total: Decimal,
    pub modifiers: serde_json::Value,
}

#[derive(SimpleObject)]
pub struct OrderDetail {
    pub order: Order,
    pub items: Vec<OrderItem>,
    pub payments: Vec<Payment>,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub amount: Decimal,
    pub method: String,
    pub status: String,
    pub transaction_id: Option<String>,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
pub struct AnalyticsSummary {
    pub revenue: RevenueMetrics,
    pub top_products: Vec<TopProduct>,
    pub customers: CustomerMetrics,
    pub period: AnalyticsPeriod,
}

#[derive(SimpleObject)]
pub struct RevenueMetrics {
    pub total: Decimal,
    pub average_order_value: Decimal,
    pub order_count: i32,
}

#[derive(SimpleObject, sqlx::FromRow)]
pub struct TopProduct {
    pub id: Uuid,
    pub name: String,
    pub sku: String,
    pub quantity_sold: i64,
    pub revenue: Decimal,
}

#[derive(SimpleObject)]
pub struct CustomerMetrics {
    pub unique_count: i32,
    pub returning_count: i32,
}

#[derive(SimpleObject)]
pub struct AnalyticsPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

/// Create GraphQL schema
pub fn create_schema(pool: PgPool) -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(pool)
        .finish()
}