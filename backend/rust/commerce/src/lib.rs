pub mod models;
pub mod services;
pub mod handlers;

use axum::{
    routing::{get, post, put, delete, patch},
    Router,
};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::services::CommerceService;

pub fn create_router(commerce_service: Arc<CommerceService>) -> Router {
    Router::new()
        // Product management
        .route("/products", get(handlers::list_products))
        .route("/products", post(handlers::create_product))
        .route("/products/:id", get(handlers::get_product))
        .route("/products/:id", put(handlers::update_product))
        .route("/products/:id", delete(handlers::delete_product))
        .route("/products/:id/variants", get(handlers::list_variants))
        .route("/products/:id/variants", post(handlers::create_variant))
        // Category management
        .route("/categories", get(handlers::list_categories))
        .route("/categories", post(handlers::create_category))
        .route("/categories/:id", get(handlers::get_category))
        .route("/categories/:id", put(handlers::update_category))
        .route("/categories/:id", delete(handlers::delete_category))
        // Inventory management
        .route("/inventory", get(handlers::list_inventory))
        .route("/inventory/:product_id", get(handlers::get_inventory))
        .route("/inventory/:product_id", put(handlers::update_inventory))
        .route("/inventory/adjust", post(handlers::adjust_inventory))
        .route("/inventory/transfer", post(handlers::transfer_inventory))
        // Order management
        .route("/orders", get(handlers::list_orders))
        .route("/orders", post(handlers::create_order))
        .route("/orders/:id", get(handlers::get_order))
        .route("/orders/:id", put(handlers::update_order))
        .route("/orders/:id/cancel", post(handlers::cancel_order))
        .route("/orders/:id/fulfill", post(handlers::fulfill_order))
        .route("/orders/:id/refund", post(handlers::refund_order))
        // Cart management
        .route("/carts", post(handlers::create_cart))
        .route("/carts/:id", get(handlers::get_cart))
        .route("/carts/:id/items", post(handlers::add_to_cart))
        .route("/carts/:id/items/:item_id", put(handlers::update_cart_item))
        .route("/carts/:id/items/:item_id", delete(handlers::remove_from_cart))
        .route("/carts/:id/checkout", post(handlers::checkout))
        // Payment management
        .route("/payments", get(handlers::list_payments))
        .route("/payments/:id", get(handlers::get_payment))
        .route("/payments/process", post(handlers::process_payment))
        .route("/payments/:id/capture", post(handlers::capture_payment))
        .route("/payments/:id/void", post(handlers::void_payment))
        // Customer management
        .route("/customers", get(handlers::list_customers))
        .route("/customers", post(handlers::create_customer))
        .route("/customers/:id", get(handlers::get_customer))
        .route("/customers/:id", put(handlers::update_customer))
        .route("/customers/:id/orders", get(handlers::get_customer_orders))
        // Discount & Promotion management
        .route("/discounts", get(handlers::list_discounts))
        .route("/discounts", post(handlers::create_discount))
        .route("/discounts/:id", get(handlers::get_discount))
        .route("/discounts/:id", put(handlers::update_discount))
        .route("/discounts/:id", delete(handlers::delete_discount))
        .route("/discounts/validate", post(handlers::validate_discount))
        // Reports
        .route("/reports/sales", get(handlers::sales_report))
        .route("/reports/inventory", get(handlers::inventory_report))
        .route("/reports/customers", get(handlers::customer_report))
        // Middleware
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive())
                .layer(axum::Extension(commerce_service)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_compilation() {
        assert!(true);
    }
}