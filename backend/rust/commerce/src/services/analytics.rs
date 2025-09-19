// ============================================================================
// OLYMPUS CLOUD - COMMERCE ANALYTICS SERVICE
// ============================================================================
// Module: commerce/src/services/analytics.rs
// Description: Comprehensive analytics service for commerce metrics and reporting
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use chrono::{DateTime, Utc, Duration, Datelike};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use olympus_shared::database::DbPool;
use olympus_shared::events::EventPublisher;
use olympus_shared::error::{Result, OlympusError};

// ============================================================================
// ANALYTICS SERVICE
// ============================================================================

#[derive(Clone)]
pub struct AnalyticsService {
    db: Arc<DbPool>,
    event_publisher: Arc<EventPublisher>,
}

impl AnalyticsService {
    pub fn new(db: Arc<DbPool>, event_publisher: Arc<EventPublisher>) -> Self {
        Self {
            db,
            event_publisher,
        }
    }

    // ========================================================================
    // SALES PERFORMANCE METRICS
    // ========================================================================

    /// Get comprehensive sales performance metrics for a given period
    pub async fn get_sales_performance(
        &self,
        tenant_id: Uuid,
        request: &SalesAnalyticsRequest,
    ) -> Result<SalesPerformanceMetrics> {
        let mut conn = self.db.acquire().await?;

        // Build WHERE conditions
        let mut where_conditions = vec!["o.tenant_id = $1".to_string()];
        let mut param_count = 1;

        let query = if let (Some(start_date), Some(end_date)) = (request.start_date, request.end_date) {
            param_count += 2;
            where_conditions.push(format!("o.created_at >= ${}", param_count - 1));
            where_conditions.push(format!("o.created_at <= ${}", param_count));

            let where_clause = where_conditions.join(" AND ");
            format!(
                r#"
                SELECT
                    COALESCE(SUM(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount ELSE 0 END), 0) as total_sales,
                    COALESCE(SUM(CASE WHEN o.status = 'cancelled' THEN o.total_amount ELSE 0 END), 0) as total_refunds,
                    COUNT(*) as total_orders,
                    COUNT(CASE WHEN o.status NOT IN ('cancelled') THEN 1 END) as completed_orders,
                    COALESCE(AVG(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount END), 0) as average_order_value
                FROM commerce.orders o
                WHERE {}
                "#,
                where_clause
            )
        } else {
            let where_clause = where_conditions.join(" AND ");
            format!(
                r#"
                SELECT
                    COALESCE(SUM(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount ELSE 0 END), 0) as total_sales,
                    COALESCE(SUM(CASE WHEN o.status = 'cancelled' THEN o.total_amount ELSE 0 END), 0) as total_refunds,
                    COUNT(*) as total_orders,
                    COUNT(CASE WHEN o.status NOT IN ('cancelled') THEN 1 END) as completed_orders,
                    COALESCE(AVG(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount END), 0) as average_order_value
                FROM commerce.orders o
                WHERE {}
                "#,
                where_clause
            )
        };

        let sales_summary = if let (Some(start_date), Some(end_date)) = (request.start_date, request.end_date) {
            sqlx::query_as::<_, SalesSummaryRow>(&query)
                .bind(tenant_id)
                .bind(start_date)
                .bind(end_date)
                .fetch_one(&mut *conn)
                .await?
        } else {
            sqlx::query_as::<_, SalesSummaryRow>(&query)
                .bind(tenant_id)
                .fetch_one(&mut *conn)
                .await?
        };

        // Get daily sales breakdown
        let daily_sales = self.get_daily_sales_breakdown(tenant_id, request).await?;

        // Get peak sales periods
        let peak_periods = self.get_peak_sales_periods(tenant_id, request).await?;

        // Calculate growth rate
        let growth_rate = self.calculate_sales_growth_rate(tenant_id, request).await?;

        Ok(SalesPerformanceMetrics {
            total_sales: sales_summary.total_sales,
            net_sales: sales_summary.total_sales - sales_summary.total_refunds,
            total_refunds: sales_summary.total_refunds,
            total_orders: sales_summary.total_orders as i32,
            completed_orders: sales_summary.completed_orders as i32,
            average_order_value: sales_summary.average_order_value,
            growth_rate,
            daily_breakdown: daily_sales,
            peak_periods,
        })
    }

    /// Get daily sales breakdown
    async fn get_daily_sales_breakdown(
        &self,
        tenant_id: Uuid,
        request: &SalesAnalyticsRequest,
    ) -> Result<Vec<DailySalesMetric>> {
        let mut conn = self.db.acquire().await?;

        let query = r#"
            SELECT
                DATE(o.created_at) as date,
                COALESCE(SUM(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount ELSE 0 END), 0) as total_sales,
                COUNT(CASE WHEN o.status NOT IN ('cancelled') THEN 1 END) as order_count,
                COALESCE(AVG(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount END), 0) as avg_order_value
            FROM commerce.orders o
            WHERE o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
            GROUP BY DATE(o.created_at)
            ORDER BY DATE(o.created_at)
        "#;

        let rows = sqlx::query_as::<_, DailySalesRow>(query)
            .bind(tenant_id)
            .bind(request.start_date)
            .bind(request.end_date)
            .fetch_all(&mut *conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| DailySalesMetric {
                date: row.date,
                total_sales: row.total_sales,
                order_count: row.order_count as i32,
                average_order_value: row.avg_order_value,
            })
            .collect())
    }

    /// Get peak sales periods (hourly analysis)
    async fn get_peak_sales_periods(
        &self,
        tenant_id: Uuid,
        request: &SalesAnalyticsRequest,
    ) -> Result<Vec<PeakPeriodMetric>> {
        let mut conn = self.db.acquire().await?;

        let query = r#"
            SELECT
                EXTRACT(hour FROM o.created_at) as hour,
                EXTRACT(dow FROM o.created_at) as day_of_week,
                COALESCE(SUM(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount ELSE 0 END), 0) as total_sales,
                COUNT(CASE WHEN o.status NOT IN ('cancelled') THEN 1 END) as order_count
            FROM commerce.orders o
            WHERE o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
            GROUP BY EXTRACT(hour FROM o.created_at), EXTRACT(dow FROM o.created_at)
            ORDER BY total_sales DESC
            LIMIT 10
        "#;

        let rows = sqlx::query_as::<_, PeakPeriodRow>(query)
            .bind(tenant_id)
            .bind(request.start_date)
            .bind(request.end_date)
            .fetch_all(&mut *conn)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| PeakPeriodMetric {
                hour: row.hour as i32,
                day_of_week: row.day_of_week as i32,
                total_sales: row.total_sales,
                order_count: row.order_count as i32,
            })
            .collect())
    }

    /// Calculate sales growth rate
    async fn calculate_sales_growth_rate(
        &self,
        tenant_id: Uuid,
        request: &SalesAnalyticsRequest,
    ) -> Result<Decimal> {
        let mut conn = self.db.acquire().await?;

        // Get current period sales
        let current_sales = sqlx::query_scalar::<_, Decimal>(
            r#"
            SELECT COALESCE(SUM(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount ELSE 0 END), 0)
            FROM commerce.orders o
            WHERE o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_one(&mut *conn)
        .await?;

        // Calculate previous period dates
        let (prev_start, prev_end) = if let (Some(start), Some(end)) = (request.start_date, request.end_date) {
            let duration = end.signed_duration_since(start);
            (Some(start - duration), Some(end - duration))
        } else {
            (None, None)
        };

        // Get previous period sales
        let previous_sales = sqlx::query_scalar::<_, Decimal>(
            r#"
            SELECT COALESCE(SUM(CASE WHEN o.status NOT IN ('cancelled') THEN o.total_amount ELSE 0 END), 0)
            FROM commerce.orders o
            WHERE o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
            "#,
        )
        .bind(tenant_id)
        .bind(prev_start)
        .bind(prev_end)
        .fetch_one(&mut *conn)
        .await?;

        // Calculate growth rate
        if previous_sales > Decimal::ZERO {
            Ok(((current_sales - previous_sales) / previous_sales) * Decimal::from(100))
        } else {
            Ok(Decimal::ZERO)
        }
    }

    // ========================================================================
    // PRODUCT PERFORMANCE TRACKING
    // ========================================================================

    /// Get product performance metrics
    pub async fn get_product_performance(
        &self,
        tenant_id: Uuid,
        request: &ProductAnalyticsRequest,
    ) -> Result<ProductPerformanceMetrics> {
        let mut conn = self.db.acquire().await?;

        // Best selling products (extracted from order items JSONB)
        let best_sellers = sqlx::query_as::<_, ProductSalesRow>(
            r#"
            SELECT
                p.id,
                p.name,
                p.sku,
                COALESCE(SUM((item->>'quantity')::int), 0) as total_quantity_sold,
                COALESCE(SUM((item->>'total')::decimal), 0) as total_revenue,
                COALESCE(AVG((item->>'unit_price')::decimal), 0) as average_price,
                COUNT(DISTINCT o.id) as order_count
            FROM commerce.products p
            LEFT JOIN commerce.orders o ON o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
                AND o.status NOT IN ('cancelled')
            LEFT JOIN jsonb_array_elements(o.items) as item ON (item->>'product_id')::uuid = p.id
            WHERE p.tenant_id = $1
                AND ($4::text IS NULL OR p.category = $4)
            GROUP BY p.id, p.name, p.sku
            ORDER BY total_quantity_sold DESC
            LIMIT $5
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(&request.category_filter)
        .bind(request.limit.unwrap_or(20))
        .fetch_all(&mut *conn)
        .await?;

        // Slow moving products
        let slow_movers = sqlx::query_as::<_, ProductSalesRow>(
            r#"
            SELECT
                p.id,
                p.name,
                p.sku,
                COALESCE(SUM((item->>'quantity')::int), 0) as total_quantity_sold,
                COALESCE(SUM((item->>'total')::decimal), 0) as total_revenue,
                COALESCE(AVG((item->>'unit_price')::decimal), 0) as average_price,
                COUNT(DISTINCT o.id) as order_count
            FROM commerce.products p
            LEFT JOIN commerce.orders o ON o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
                AND o.status NOT IN ('cancelled')
            LEFT JOIN jsonb_array_elements(o.items) as item ON (item->>'product_id')::uuid = p.id
            WHERE p.tenant_id = $1
            GROUP BY p.id, p.name, p.sku
            HAVING COALESCE(SUM((item->>'quantity')::int), 0) < 5
            ORDER BY total_quantity_sold ASC
            LIMIT $4
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.limit.unwrap_or(20))
        .fetch_all(&mut *conn)
        .await?;

        // Category performance
        let category_performance = sqlx::query_as::<_, CategoryPerformanceRow>(
            r#"
            SELECT
                COALESCE(p.category, 'Uncategorized') as category_name,
                COALESCE(SUM((item->>'quantity')::int), 0) as total_quantity_sold,
                COALESCE(SUM((item->>'total')::decimal), 0) as total_revenue,
                COUNT(DISTINCT p.id) as product_count
            FROM commerce.products p
            LEFT JOIN commerce.orders o ON o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
                AND o.status NOT IN ('cancelled')
            LEFT JOIN jsonb_array_elements(o.items) as item ON (item->>'product_id')::uuid = p.id
            WHERE p.tenant_id = $1
            GROUP BY p.category
            ORDER BY total_revenue DESC
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        Ok(ProductPerformanceMetrics {
            best_sellers: best_sellers
                .into_iter()
                .map(|row| ProductSalesMetric {
                    product_id: row.id,
                    product_name: row.name,
                    sku: row.sku,
                    total_quantity_sold: row.total_quantity_sold as i32,
                    total_revenue: row.total_revenue,
                    average_price: row.average_price,
                    order_count: row.order_count as i32,
                })
                .collect(),
            slow_movers: slow_movers
                .into_iter()
                .map(|row| ProductSalesMetric {
                    product_id: row.id,
                    product_name: row.name,
                    sku: row.sku,
                    total_quantity_sold: row.total_quantity_sold as i32,
                    total_revenue: row.total_revenue,
                    average_price: row.average_price,
                    order_count: row.order_count as i32,
                })
                .collect(),
            category_performance: category_performance
                .into_iter()
                .map(|row| CategoryPerformanceMetric {
                    category_name: row.category_name,
                    total_quantity_sold: row.total_quantity_sold as i32,
                    total_revenue: row.total_revenue,
                    product_count: row.product_count as i32,
                })
                .collect(),
        })
    }

    // ========================================================================
    // ORDER ANALYTICS
    // ========================================================================

    /// Get comprehensive order analytics
    pub async fn get_order_analytics(
        &self,
        tenant_id: Uuid,
        request: &OrderAnalyticsRequest,
    ) -> Result<OrderAnalyticsMetrics> {
        let mut conn = self.db.acquire().await?;

        // Order volume metrics
        let volume_metrics = sqlx::query_as::<_, OrderVolumeRow>(
            r#"
            SELECT
                COUNT(*) as total_orders,
                COUNT(CASE WHEN status = 'completed' THEN 1 END) as completed_orders,
                COUNT(CASE WHEN status = 'cancelled' THEN 1 END) as cancelled_orders,
                COUNT(CASE WHEN status = 'pending' THEN 1 END) as pending_orders,
                COALESCE(AVG(EXTRACT(EPOCH FROM (updated_at - created_at))/3600), 0) as avg_processing_hours
            FROM commerce.orders
            WHERE tenant_id = $1
                AND ($2::timestamptz IS NULL OR created_at >= $2)
                AND ($3::timestamptz IS NULL OR created_at <= $3)
                AND ($4::text IS NULL OR status::text = $4)
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .bind(request.status_filter.as_ref().map(|s| format!("{:?}", s).to_lowercase()))
        .fetch_one(&mut *conn)
        .await?;

        // Order status distribution
        let status_distribution = sqlx::query_as::<_, OrderStatusRow>(
            r#"
            SELECT
                status::text as status,
                COUNT(*) as count,
                ROUND(COUNT(*) * 100.0 / SUM(COUNT(*)) OVER (), 2) as percentage
            FROM commerce.orders
            WHERE tenant_id = $1
                AND ($2::timestamptz IS NULL OR created_at >= $2)
                AND ($3::timestamptz IS NULL OR created_at <= $3)
            GROUP BY status
            ORDER BY count DESC
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        // Order patterns by time
        let hourly_patterns = sqlx::query_as::<_, OrderPatternRow>(
            r#"
            SELECT
                EXTRACT(hour FROM created_at) as hour,
                COUNT(*) as order_count,
                COALESCE(AVG(total_amount), 0) as avg_order_value
            FROM commerce.orders
            WHERE tenant_id = $1
                AND ($2::timestamptz IS NULL OR created_at >= $2)
                AND ($3::timestamptz IS NULL OR created_at <= $3)
                AND status NOT IN ('cancelled')
            GROUP BY EXTRACT(hour FROM created_at)
            ORDER BY hour
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        Ok(OrderAnalyticsMetrics {
            total_orders: volume_metrics.total_orders as i32,
            completed_orders: volume_metrics.completed_orders as i32,
            cancelled_orders: volume_metrics.cancelled_orders as i32,
            pending_orders: volume_metrics.pending_orders as i32,
            average_processing_hours: volume_metrics.avg_processing_hours,
            completion_rate: if volume_metrics.total_orders > 0 {
                (volume_metrics.completed_orders as f64 / volume_metrics.total_orders as f64) * 100.0
            } else {
                0.0
            },
            status_distribution: status_distribution
                .into_iter()
                .map(|row| OrderStatusDistribution {
                    status: row.status,
                    count: row.count as i32,
                    percentage: row.percentage,
                })
                .collect(),
            hourly_patterns: hourly_patterns
                .into_iter()
                .map(|row| OrderPatternMetric {
                    hour: row.hour as i32,
                    order_count: row.order_count as i32,
                    average_order_value: row.avg_order_value,
                })
                .collect(),
        })
    }

    // ========================================================================
    // REVENUE CALCULATIONS
    // ========================================================================

    /// Get comprehensive revenue analytics
    pub async fn get_revenue_analytics(
        &self,
        tenant_id: Uuid,
        request: &RevenueAnalyticsRequest,
    ) -> Result<RevenueAnalyticsMetrics> {
        let mut conn = self.db.acquire().await?;

        // Gross and net revenue
        let revenue_summary = sqlx::query_as::<_, RevenueSummaryRow>(
            r#"
            SELECT
                COALESCE(SUM(CASE WHEN status NOT IN ('cancelled') THEN total_amount ELSE 0 END), 0) as gross_revenue,
                COALESCE(SUM(CASE WHEN status = 'cancelled' THEN total_amount ELSE 0 END), 0) as refunds,
                COALESCE(SUM(CASE WHEN status NOT IN ('cancelled') THEN tax_amount ELSE 0 END), 0) as total_tax,
                COALESCE(SUM(CASE WHEN status NOT IN ('cancelled') THEN delivery_fee ELSE 0 END), 0) as total_shipping,
                COALESCE(SUM(CASE WHEN status NOT IN ('cancelled') THEN discount_amount ELSE 0 END), 0) as total_discounts
            FROM commerce.orders
            WHERE tenant_id = $1
                AND ($2::timestamptz IS NULL OR created_at >= $2)
                AND ($3::timestamptz IS NULL OR created_at <= $3)
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_one(&mut *conn)
        .await?;

        // Revenue by product category
        let category_revenue = sqlx::query_as::<_, CategoryRevenueRow>(
            r#"
            SELECT
                COALESCE(p.category, 'Uncategorized') as category_name,
                COALESCE(SUM((item->>'total')::decimal), 0) as revenue
            FROM commerce.orders o
            LEFT JOIN jsonb_array_elements(o.items) as item ON true
            LEFT JOIN commerce.products p ON (item->>'product_id')::uuid = p.id
            WHERE o.tenant_id = $1
                AND o.status NOT IN ('cancelled')
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
            GROUP BY p.category
            ORDER BY revenue DESC
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        // Monthly revenue trends
        let monthly_trends = sqlx::query_as::<_, MonthlyRevenueRow>(
            r#"
            SELECT
                EXTRACT(year FROM created_at) as year,
                EXTRACT(month FROM created_at) as month,
                COALESCE(SUM(CASE WHEN status NOT IN ('cancelled') THEN total_amount ELSE 0 END), 0) as revenue
            FROM commerce.orders
            WHERE tenant_id = $1
                AND ($2::timestamptz IS NULL OR created_at >= $2)
                AND ($3::timestamptz IS NULL OR created_at <= $3)
            GROUP BY EXTRACT(year FROM created_at), EXTRACT(month FROM created_at)
            ORDER BY year, month
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        let net_revenue = revenue_summary.gross_revenue - revenue_summary.refunds;

        Ok(RevenueAnalyticsMetrics {
            gross_revenue: revenue_summary.gross_revenue,
            net_revenue,
            total_refunds: revenue_summary.refunds,
            total_tax: revenue_summary.total_tax,
            total_shipping: revenue_summary.total_shipping,
            total_discounts: revenue_summary.total_discounts,
            category_breakdown: category_revenue
                .into_iter()
                .map(|row| CategoryRevenueMetric {
                    category_name: row.category_name,
                    revenue: row.revenue,
                })
                .collect(),
            monthly_trends: monthly_trends
                .into_iter()
                .map(|row| MonthlyRevenueMetric {
                    year: row.year as i32,
                    month: row.month as i32,
                    revenue: row.revenue,
                })
                .collect(),
        })
    }

    // ========================================================================
    // CUSTOMER ANALYTICS
    // ========================================================================

    /// Get customer analytics (simplified for current schema)
    pub async fn get_customer_analytics(
        &self,
        tenant_id: Uuid,
        request: &CustomerAnalyticsRequest,
    ) -> Result<CustomerAnalyticsMetrics> {
        let mut conn = self.db.acquire().await?;

        // Customer metrics based on customer_id and guest_email
        let customer_metrics = sqlx::query_as::<_, CustomerMetricsRow>(
            r#"
            SELECT
                COUNT(DISTINCT COALESCE(customer_id::text, guest_email)) FILTER (WHERE COALESCE(customer_id, uuid_generate_v4()) IS NOT NULL OR guest_email IS NOT NULL) as total_customers,
                COUNT(DISTINCT CASE WHEN created_at >= $2 THEN COALESCE(customer_id::text, guest_email) END) as new_customers,
                COALESCE(AVG(customer_stats.total_spent), 0) as avg_lifetime_value,
                COALESCE(AVG(customer_stats.order_count), 0) as avg_order_frequency
            FROM commerce.orders o
            LEFT JOIN (
                SELECT
                    COALESCE(customer_id::text, guest_email) as customer_key,
                    SUM(total_amount) as total_spent,
                    COUNT(*) as order_count
                FROM commerce.orders
                WHERE tenant_id = $1 AND status NOT IN ('cancelled')
                GROUP BY COALESCE(customer_id::text, guest_email)
            ) customer_stats ON COALESCE(o.customer_id::text, o.guest_email) = customer_stats.customer_key
            WHERE o.tenant_id = $1
                AND ($3::timestamptz IS NULL OR o.created_at >= $3)
                AND ($4::timestamptz IS NULL OR o.created_at <= $4)
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date.unwrap_or_else(|| Utc::now() - Duration::days(30)))
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_one(&mut *conn)
        .await?;

        // Customer segmentation by order frequency
        let segmentation = sqlx::query_as::<_, CustomerSegmentRow>(
            r#"
            SELECT
                CASE
                    WHEN order_count = 1 THEN 'new'
                    WHEN order_count BETWEEN 2 AND 5 THEN 'occasional'
                    WHEN order_count BETWEEN 6 AND 15 THEN 'regular'
                    ELSE 'loyal'
                END as segment,
                COUNT(*) as customer_count,
                COALESCE(AVG(total_spent), 0) as avg_spent
            FROM (
                SELECT
                    COALESCE(customer_id::text, guest_email) as customer_key,
                    COUNT(*) as order_count,
                    SUM(total_amount) as total_spent
                FROM commerce.orders
                WHERE tenant_id = $1
                    AND (customer_id IS NOT NULL OR guest_email IS NOT NULL)
                    AND status NOT IN ('cancelled')
                    AND ($2::timestamptz IS NULL OR created_at >= $2)
                    AND ($3::timestamptz IS NULL OR created_at <= $3)
                GROUP BY COALESCE(customer_id::text, guest_email)
            ) customer_stats
            GROUP BY
                CASE
                    WHEN order_count = 1 THEN 'new'
                    WHEN order_count BETWEEN 2 AND 5 THEN 'occasional'
                    WHEN order_count BETWEEN 6 AND 15 THEN 'regular'
                    ELSE 'loyal'
                END
            ORDER BY customer_count DESC
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        // Calculate retention rate
        let retention_rate = if customer_metrics.total_customers > 0 {
            let returning_customers = customer_metrics.total_customers - customer_metrics.new_customers;
            (returning_customers as f64 / customer_metrics.total_customers as f64) * 100.0
        } else {
            0.0
        };

        Ok(CustomerAnalyticsMetrics {
            total_customers: customer_metrics.total_customers as i32,
            new_customers: customer_metrics.new_customers as i32,
            retention_rate,
            average_lifetime_value: customer_metrics.avg_lifetime_value,
            average_order_frequency: customer_metrics.avg_order_frequency,
            segmentation: segmentation
                .into_iter()
                .map(|row| CustomerSegmentMetric {
                    segment: row.segment,
                    customer_count: row.customer_count as i32,
                    average_spent: row.avg_spent,
                })
                .collect(),
        })
    }

    // ========================================================================
    // INVENTORY ANALYTICS (Simplified for current schema)
    // ========================================================================

    /// Get inventory analytics
    pub async fn get_inventory_analytics(
        &self,
        tenant_id: Uuid,
        request: &InventoryAnalyticsRequest,
    ) -> Result<InventoryAnalyticsMetrics> {
        let mut conn = self.db.acquire().await?;

        // Basic inventory metrics
        let inventory_summary = sqlx::query_as::<_, InventorySummaryRow>(
            r#"
            SELECT
                COUNT(*) as total_products,
                COUNT(CASE WHEN current_stock <= low_stock_threshold THEN 1 END) as low_stock_items,
                COUNT(CASE WHEN current_stock = 0 THEN 1 END) as out_of_stock_items,
                COALESCE(SUM(current_stock * COALESCE(cost, price)), 0) as total_inventory_value,
                COALESCE(AVG(current_stock), 0) as avg_stock_level
            FROM commerce.products
            WHERE tenant_id = $1
                AND track_inventory = true
            "#,
        )
        .bind(tenant_id)
        .fetch_one(&mut *conn)
        .await?;

        // High value items
        let high_value_items = sqlx::query_as::<_, InventoryValueRow>(
            r#"
            SELECT
                id,
                name,
                sku,
                current_stock,
                COALESCE(cost, price) as unit_cost,
                current_stock * COALESCE(cost, price) as total_value
            FROM commerce.products
            WHERE tenant_id = $1
                AND track_inventory = true
                AND current_stock > 0
            ORDER BY total_value DESC
            LIMIT 20
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&mut *conn)
        .await?;

        // Simple turnover calculation (based on order data)
        let turnover_metrics = sqlx::query_as::<_, TurnoverMetricsRow>(
            r#"
            SELECT
                p.id,
                p.name,
                p.sku,
                p.current_stock,
                COALESCE(SUM((item->>'quantity')::int), 0) as total_sold,
                CASE
                    WHEN p.current_stock > 0 AND SUM((item->>'quantity')::int) > 0
                    THEN SUM((item->>'quantity')::int)::decimal / p.current_stock
                    ELSE 0
                END as turnover_ratio
            FROM commerce.products p
            LEFT JOIN commerce.orders o ON o.tenant_id = $1
                AND ($2::timestamptz IS NULL OR o.created_at >= $2)
                AND ($3::timestamptz IS NULL OR o.created_at <= $3)
                AND o.status NOT IN ('cancelled')
            LEFT JOIN jsonb_array_elements(o.items) as item ON (item->>'product_id')::uuid = p.id
            WHERE p.tenant_id = $1
                AND p.track_inventory = true
            GROUP BY p.id, p.name, p.sku, p.current_stock
            ORDER BY turnover_ratio DESC
            LIMIT 20
            "#,
        )
        .bind(tenant_id)
        .bind(request.start_date)
        .bind(request.end_date)
        .fetch_all(&mut *conn)
        .await?;

        Ok(InventoryAnalyticsMetrics {
            total_products: inventory_summary.total_products as i32,
            low_stock_items: inventory_summary.low_stock_items as i32,
            out_of_stock_items: inventory_summary.out_of_stock_items as i32,
            total_inventory_value: inventory_summary.total_inventory_value,
            average_stock_level: inventory_summary.avg_stock_level,
            high_value_items: high_value_items
                .into_iter()
                .map(|row| InventoryValueMetric {
                    product_id: row.id,
                    product_name: row.name,
                    sku: row.sku,
                    quantity: row.current_stock.unwrap_or(0),
                    unit_cost: row.unit_cost,
                    total_value: row.total_value,
                })
                .collect(),
            turnover_analysis: turnover_metrics
                .into_iter()
                .map(|row| InventoryTurnoverMetric {
                    product_id: row.id,
                    product_name: row.name,
                    sku: row.sku,
                    current_stock: row.current_stock.unwrap_or(0),
                    total_sold: row.total_sold as i32,
                    turnover_ratio: row.turnover_ratio,
                })
                .collect(),
        })
    }

    // ========================================================================
    // EXPORT AND CACHING
    // ========================================================================

    /// Export analytics data to CSV format
    pub async fn export_analytics_csv(
        &self,
        tenant_id: Uuid,
        export_type: AnalyticsExportType,
        request: AnalyticsExportRequest,
    ) -> Result<String> {
        match export_type {
            AnalyticsExportType::Sales => {
                let metrics = self.get_sales_performance(tenant_id, &request.into()).await?;
                Ok(self.format_sales_csv(metrics))
            }
            AnalyticsExportType::Products => {
                let metrics = self.get_product_performance(tenant_id, &request.into()).await?;
                Ok(self.format_products_csv(metrics))
            }
            AnalyticsExportType::Orders => {
                let metrics = self.get_order_analytics(tenant_id, &request.into()).await?;
                Ok(self.format_orders_csv(metrics))
            }
            AnalyticsExportType::Revenue => {
                let metrics = self.get_revenue_analytics(tenant_id, &request.into()).await?;
                Ok(self.format_revenue_csv(metrics))
            }
            AnalyticsExportType::Customers => {
                let metrics = self.get_customer_analytics(tenant_id, &request.into()).await?;
                Ok(self.format_customers_csv(metrics))
            }
            AnalyticsExportType::Inventory => {
                let metrics = self.get_inventory_analytics(tenant_id, &request.into()).await?;
                Ok(self.format_inventory_csv(metrics))
            }
        }
    }

    /// Cache analytics metrics for real-time dashboards
    pub async fn cache_analytics_metrics(&self, tenant_id: Uuid) -> Result<()> {
        // Publish analytics events to Redis for real-time dashboards
        let event_data = serde_json::json!({
            "tenant_id": tenant_id,
            "timestamp": Utc::now(),
            "event_type": "analytics_refresh"
        });

        self.event_publisher
            .publish("analytics.refresh", &event_data)
            .await?;

        Ok(())
    }

    // ========================================================================
    // PRIVATE HELPER METHODS
    // ========================================================================

    fn format_sales_csv(&self, metrics: SalesPerformanceMetrics) -> String {
        let mut csv = "Date,Total Sales,Orders,Average Order Value\n".to_string();
        for daily in metrics.daily_breakdown {
            csv.push_str(&format!(
                "{},{},{},{}\n",
                daily.date, daily.total_sales, daily.order_count, daily.average_order_value
            ));
        }
        csv
    }

    fn format_products_csv(&self, metrics: ProductPerformanceMetrics) -> String {
        let mut csv = "Product Name,SKU,Quantity Sold,Revenue,Average Price\n".to_string();
        for product in metrics.best_sellers {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                product.product_name,
                product.sku,
                product.total_quantity_sold,
                product.total_revenue,
                product.average_price
            ));
        }
        csv
    }

    fn format_orders_csv(&self, metrics: OrderAnalyticsMetrics) -> String {
        let mut csv = "Hour,Order Count,Average Order Value\n".to_string();
        for pattern in metrics.hourly_patterns {
            csv.push_str(&format!(
                "{},{},{}\n",
                pattern.hour, pattern.order_count, pattern.average_order_value
            ));
        }
        csv
    }

    fn format_revenue_csv(&self, metrics: RevenueAnalyticsMetrics) -> String {
        let mut csv = "Year,Month,Revenue\n".to_string();
        for trend in metrics.monthly_trends {
            csv.push_str(&format!("{},{},{}\n", trend.year, trend.month, trend.revenue));
        }
        csv
    }

    fn format_customers_csv(&self, metrics: CustomerAnalyticsMetrics) -> String {
        let mut csv = "Segment,Customer Count,Average Spent\n".to_string();
        for segment in metrics.segmentation {
            csv.push_str(&format!(
                "{},{},{}\n",
                segment.segment, segment.customer_count, segment.average_spent
            ));
        }
        csv
    }

    fn format_inventory_csv(&self, metrics: InventoryAnalyticsMetrics) -> String {
        let mut csv = "Product Name,SKU,Current Stock,Total Sold,Turnover Ratio\n".to_string();
        for item in metrics.turnover_analysis {
            csv.push_str(&format!(
                "{},{},{},{},{}\n",
                item.product_name,
                item.sku,
                item.current_stock,
                item.total_sold,
                item.turnover_ratio
            ));
        }
        csv
    }
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SalesAnalyticsRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub location_filter: Option<String>,
    pub channel_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ProductAnalyticsRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub category_filter: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct OrderAnalyticsRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub status_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct RevenueAnalyticsRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub group_by: Option<RevenueGroupBy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CustomerAnalyticsRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub segment_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InventoryAnalyticsRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub location_filter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AnalyticsExportRequest {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub format: Option<ExportFormat>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueGroupBy {
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    CSV,
    JSON,
    Excel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnalyticsExportType {
    Sales,
    Products,
    Orders,
    Revenue,
    Customers,
    Inventory,
}

// ============================================================================
// RESPONSE MODELS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesPerformanceMetrics {
    pub total_sales: Decimal,
    pub net_sales: Decimal,
    pub total_refunds: Decimal,
    pub total_orders: i32,
    pub completed_orders: i32,
    pub average_order_value: Decimal,
    pub growth_rate: Decimal,
    pub daily_breakdown: Vec<DailySalesMetric>,
    pub peak_periods: Vec<PeakPeriodMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySalesMetric {
    pub date: chrono::NaiveDate,
    pub total_sales: Decimal,
    pub order_count: i32,
    pub average_order_value: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeakPeriodMetric {
    pub hour: i32,
    pub day_of_week: i32,
    pub total_sales: Decimal,
    pub order_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPerformanceMetrics {
    pub best_sellers: Vec<ProductSalesMetric>,
    pub slow_movers: Vec<ProductSalesMetric>,
    pub category_performance: Vec<CategoryPerformanceMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductSalesMetric {
    pub product_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub total_quantity_sold: i32,
    pub total_revenue: Decimal,
    pub average_price: Decimal,
    pub order_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPerformanceMetric {
    pub category_name: String,
    pub total_quantity_sold: i32,
    pub total_revenue: Decimal,
    pub product_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderAnalyticsMetrics {
    pub total_orders: i32,
    pub completed_orders: i32,
    pub cancelled_orders: i32,
    pub pending_orders: i32,
    pub average_processing_hours: Decimal,
    pub completion_rate: f64,
    pub status_distribution: Vec<OrderStatusDistribution>,
    pub hourly_patterns: Vec<OrderPatternMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusDistribution {
    pub status: String,
    pub count: i32,
    pub percentage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPatternMetric {
    pub hour: i32,
    pub order_count: i32,
    pub average_order_value: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueAnalyticsMetrics {
    pub gross_revenue: Decimal,
    pub net_revenue: Decimal,
    pub total_refunds: Decimal,
    pub total_tax: Decimal,
    pub total_shipping: Decimal,
    pub total_discounts: Decimal,
    pub category_breakdown: Vec<CategoryRevenueMetric>,
    pub monthly_trends: Vec<MonthlyRevenueMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryRevenueMetric {
    pub category_name: String,
    pub revenue: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyRevenueMetric {
    pub year: i32,
    pub month: i32,
    pub revenue: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAnalyticsMetrics {
    pub total_customers: i32,
    pub new_customers: i32,
    pub retention_rate: f64,
    pub average_lifetime_value: Decimal,
    pub average_order_frequency: Decimal,
    pub segmentation: Vec<CustomerSegmentMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSegmentMetric {
    pub segment: String,
    pub customer_count: i32,
    pub average_spent: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryAnalyticsMetrics {
    pub total_products: i32,
    pub low_stock_items: i32,
    pub out_of_stock_items: i32,
    pub total_inventory_value: Decimal,
    pub average_stock_level: Decimal,
    pub high_value_items: Vec<InventoryValueMetric>,
    pub turnover_analysis: Vec<InventoryTurnoverMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryValueMetric {
    pub product_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub quantity: i32,
    pub unit_cost: Decimal,
    pub total_value: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryTurnoverMetric {
    pub product_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub current_stock: i32,
    pub total_sold: i32,
    pub turnover_ratio: Decimal,
}

// ============================================================================
// DATABASE ROW MODELS (Internal)
// ============================================================================

#[derive(sqlx::FromRow)]
struct SalesSummaryRow {
    total_sales: Decimal,
    total_refunds: Decimal,
    total_orders: i64,
    completed_orders: i64,
    average_order_value: Decimal,
}

#[derive(sqlx::FromRow)]
struct DailySalesRow {
    date: chrono::NaiveDate,
    total_sales: Decimal,
    order_count: i64,
    avg_order_value: Decimal,
}

#[derive(sqlx::FromRow)]
struct PeakPeriodRow {
    hour: f64,
    day_of_week: f64,
    total_sales: Decimal,
    order_count: i64,
}

#[derive(sqlx::FromRow)]
struct ProductSalesRow {
    id: Uuid,
    name: String,
    sku: String,
    total_quantity_sold: i64,
    total_revenue: Decimal,
    average_price: Decimal,
    order_count: i64,
}

#[derive(sqlx::FromRow)]
struct CategoryPerformanceRow {
    category_name: String,
    total_quantity_sold: i64,
    total_revenue: Decimal,
    product_count: i64,
}

#[derive(sqlx::FromRow)]
struct OrderVolumeRow {
    total_orders: i64,
    completed_orders: i64,
    cancelled_orders: i64,
    pending_orders: i64,
    avg_processing_hours: Decimal,
}

#[derive(sqlx::FromRow)]
struct OrderStatusRow {
    status: String,
    count: i64,
    percentage: Decimal,
}

#[derive(sqlx::FromRow)]
struct OrderPatternRow {
    hour: f64,
    order_count: i64,
    avg_order_value: Decimal,
}

#[derive(sqlx::FromRow)]
struct RevenueSummaryRow {
    gross_revenue: Decimal,
    refunds: Decimal,
    total_tax: Decimal,
    total_shipping: Decimal,
    total_discounts: Decimal,
}

#[derive(sqlx::FromRow)]
struct CategoryRevenueRow {
    category_name: String,
    revenue: Decimal,
}

#[derive(sqlx::FromRow)]
struct MonthlyRevenueRow {
    year: f64,
    month: f64,
    revenue: Decimal,
}

#[derive(sqlx::FromRow)]
struct CustomerMetricsRow {
    total_customers: i64,
    new_customers: i64,
    avg_lifetime_value: Decimal,
    avg_order_frequency: Decimal,
}

#[derive(sqlx::FromRow)]
struct CustomerSegmentRow {
    segment: String,
    customer_count: i64,
    avg_spent: Decimal,
}

#[derive(sqlx::FromRow)]
struct InventorySummaryRow {
    total_products: i64,
    low_stock_items: i64,
    out_of_stock_items: i64,
    total_inventory_value: Decimal,
    avg_stock_level: Decimal,
}

#[derive(sqlx::FromRow)]
struct InventoryValueRow {
    id: Uuid,
    name: String,
    sku: String,
    current_stock: Option<i32>,
    unit_cost: Decimal,
    total_value: Decimal,
}

#[derive(sqlx::FromRow)]
struct TurnoverMetricsRow {
    id: Uuid,
    name: String,
    sku: String,
    current_stock: Option<i32>,
    total_sold: i64,
    turnover_ratio: Decimal,
}

// ============================================================================
// CONVERSION IMPLEMENTATIONS
// ============================================================================

impl From<AnalyticsExportRequest> for SalesAnalyticsRequest {
    fn from(req: AnalyticsExportRequest) -> Self {
        Self {
            start_date: req.start_date,
            end_date: req.end_date,
            location_filter: None,
            channel_filter: None,
        }
    }
}

impl From<AnalyticsExportRequest> for ProductAnalyticsRequest {
    fn from(req: AnalyticsExportRequest) -> Self {
        Self {
            start_date: req.start_date,
            end_date: req.end_date,
            category_filter: None,
            limit: None,
        }
    }
}

impl From<AnalyticsExportRequest> for OrderAnalyticsRequest {
    fn from(req: AnalyticsExportRequest) -> Self {
        Self {
            start_date: req.start_date,
            end_date: req.end_date,
            status_filter: None,
        }
    }
}

impl From<AnalyticsExportRequest> for RevenueAnalyticsRequest {
    fn from(req: AnalyticsExportRequest) -> Self {
        Self {
            start_date: req.start_date,
            end_date: req.end_date,
            group_by: None,
        }
    }
}

impl From<AnalyticsExportRequest> for CustomerAnalyticsRequest {
    fn from(req: AnalyticsExportRequest) -> Self {
        Self {
            start_date: req.start_date,
            end_date: req.end_date,
            segment_filter: None,
        }
    }
}

impl From<AnalyticsExportRequest> for InventoryAnalyticsRequest {
    fn from(req: AnalyticsExportRequest) -> Self {
        Self {
            start_date: req.start_date,
            end_date: req.end_date,
            location_filter: None,
        }
    }
}