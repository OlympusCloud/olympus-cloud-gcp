// ============================================================================
// OLYMPUS CLOUD - INVENTORY SERVICE TESTS
// ============================================================================
// Module: commerce/src/tests/inventory_tests.rs
// Description: Unit tests for inventory management functionality
// Author: Claude Code Agent
// Date: 2025-01-19
// ============================================================================

use std::sync::Arc;
use uuid::Uuid;
use rust_decimal::Decimal;
use sqlx::PgPool;

use crate::models::InventoryAdjustmentType;
use crate::services::inventory::{InventoryService, StockAdjustmentRequest, StockReservationRequest, ReservationItem};
use olympus_shared::events::EventPublisher;

/// Mock event publisher for testing
pub struct MockEventPublisher;

impl MockEventPublisher {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl EventPublisher for MockEventPublisher {
    async fn publish(&self, _event: &olympus_shared::events::DomainEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Mock implementation - just return Ok
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create test inventory service
    fn create_test_service(pool: PgPool) -> InventoryService {
        let event_publisher = Arc::new(MockEventPublisher::new());
        InventoryService::new(pool, event_publisher)
    }

    #[sqlx::test]
    async fn test_get_stock_level_nonexistent(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();

        let result = service
            .get_stock_level(tenant_id, product_id, None, None)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[sqlx::test]
    async fn test_adjust_stock_increase(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Setup test data
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        let request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 10,
            reason: "Initial stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1500, 2)), // $15.00
        };

        let result = service.adjust_stock(tenant_id, request, user_id).await;

        assert!(result.is_ok());
        let stock_level = result.unwrap();
        assert_eq!(stock_level.on_hand, 10);
        assert_eq!(stock_level.available, 10);
        assert_eq!(stock_level.reserved, 0);
    }

    #[sqlx::test]
    async fn test_adjust_stock_decrease(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Setup test data
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        // First, increase stock
        let increase_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 20,
            reason: "Initial stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1500, 2)),
        };

        service.adjust_stock(tenant_id, increase_request, user_id).await.unwrap();

        // Then decrease stock
        let decrease_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Sale,
            quantity_change: 5,
            reason: "Sale".to_string(),
            reference_id: None,
            cost_per_unit: None,
        };

        let result = service.adjust_stock(tenant_id, decrease_request, user_id).await;

        assert!(result.is_ok());
        let stock_level = result.unwrap();
        assert_eq!(stock_level.on_hand, 15);
        assert_eq!(stock_level.available, 15);
    }

    #[sqlx::test]
    async fn test_reserve_stock_success(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        // Setup test data with stock
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        // Add initial stock
        let stock_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 10,
            reason: "Initial stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1000, 2)),
        };

        service.adjust_stock(tenant_id, stock_request, user_id).await.unwrap();

        // Reserve stock
        let reservation_request = StockReservationRequest {
            items: vec![ReservationItem {
                product_id,
                variant_id: None,
                location_id: None,
                quantity: 3,
            }],
            reference_id: order_id,
            expires_at: None,
        };

        let result = service.reserve_stock(tenant_id, reservation_request, user_id).await;

        assert!(result.is_ok());
        let allocation_result = result.unwrap();
        assert!(allocation_result.success);
        assert_eq!(allocation_result.allocated_items.len(), 1);
        assert_eq!(allocation_result.allocated_items[0].quantity_allocated, 3);
        assert!(allocation_result.insufficient_stock.is_empty());

        // Verify stock levels after reservation
        let stock_level = service.get_stock_level(tenant_id, product_id, None, None).await.unwrap().unwrap();
        assert_eq!(stock_level.on_hand, 10);
        assert_eq!(stock_level.reserved, 3);
        assert_eq!(stock_level.available, 7);
    }

    #[sqlx::test]
    async fn test_reserve_stock_insufficient(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        // Setup test data with limited stock
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        // Add minimal stock
        let stock_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 2,
            reason: "Initial stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1000, 2)),
        };

        service.adjust_stock(tenant_id, stock_request, user_id).await.unwrap();

        // Try to reserve more than available
        let reservation_request = StockReservationRequest {
            items: vec![ReservationItem {
                product_id,
                variant_id: None,
                location_id: None,
                quantity: 5,
            }],
            reference_id: order_id,
            expires_at: None,
        };

        let result = service.reserve_stock(tenant_id, reservation_request, user_id).await;

        assert!(result.is_ok());
        let allocation_result = result.unwrap();
        assert!(!allocation_result.success);
        assert!(allocation_result.allocated_items.is_empty());
        assert_eq!(allocation_result.insufficient_stock.len(), 1);
        assert_eq!(allocation_result.insufficient_stock[0].requested_quantity, 5);
        assert_eq!(allocation_result.insufficient_stock[0].available_quantity, 2);
    }

    #[sqlx::test]
    async fn test_release_reservation(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let order_id = Uuid::new_v4();

        // Setup test data
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        // Add stock and reserve some
        let stock_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 10,
            reason: "Initial stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1000, 2)),
        };

        service.adjust_stock(tenant_id, stock_request, user_id).await.unwrap();

        let reservation_request = StockReservationRequest {
            items: vec![ReservationItem {
                product_id,
                variant_id: None,
                location_id: None,
                quantity: 4,
            }],
            reference_id: order_id,
            expires_at: None,
        };

        service.reserve_stock(tenant_id, reservation_request, user_id).await.unwrap();

        // Release the reservation
        let result = service.release_reservation(tenant_id, order_id, user_id).await;

        assert!(result.is_ok());

        // Verify stock is released
        let stock_level = service.get_stock_level(tenant_id, product_id, None, None).await.unwrap().unwrap();
        assert_eq!(stock_level.on_hand, 10);
        assert_eq!(stock_level.reserved, 0);
        assert_eq!(stock_level.available, 10);
    }

    #[sqlx::test]
    async fn test_low_stock_threshold(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Setup test data
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        // Set low stock threshold
        service.update_low_stock_threshold(tenant_id, product_id, None, None, Some(5), user_id).await.unwrap();

        // Add stock below threshold
        let stock_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 3,
            reason: "Low stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1000, 2)),
        };

        service.adjust_stock(tenant_id, stock_request, user_id).await.unwrap();

        // Check low stock alerts
        let alerts = service.get_low_stock_alerts(tenant_id, None).await;
        assert!(alerts.is_ok());
        let alerts = alerts.unwrap();
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].current_stock, 3);
        assert_eq!(alerts[0].low_stock_threshold, 5);
    }

    #[sqlx::test]
    async fn test_inventory_valuation(pool: PgPool) {
        let service = create_test_service(pool);
        let tenant_id = Uuid::new_v4();
        let product_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Setup test data
        setup_test_tenant(&pool, tenant_id).await;
        setup_test_product(&pool, tenant_id, product_id).await;

        // Add stock with cost
        let stock_request = StockAdjustmentRequest {
            product_id,
            variant_id: None,
            location_id: None,
            adjustment_type: InventoryAdjustmentType::Increase,
            quantity_change: 10,
            reason: "Initial stock".to_string(),
            reference_id: None,
            cost_per_unit: Some(Decimal::new(1500, 2)), // $15.00 per unit
        };

        service.adjust_stock(tenant_id, stock_request, user_id).await.unwrap();

        // Calculate valuation
        let valuation = service.calculate_inventory_valuation(tenant_id, None).await;
        assert!(valuation.is_ok());
        let valuation = valuation.unwrap();
        assert_eq!(valuation.total_quantity, 10);
        assert_eq!(valuation.total_value_cost, Decimal::new(15000, 2)); // $150.00 total cost
    }

    // Helper functions for setting up test data
    async fn setup_test_tenant(pool: &PgPool, tenant_id: Uuid) {
        sqlx::query!(
            "INSERT INTO platform.tenants (id, slug, name, industry, tier)
             VALUES ($1, $2, $3, $4, $5)",
            tenant_id,
            format!("test-{}", tenant_id),
            "Test Tenant",
            "test",
            "free"
        )
        .execute(pool)
        .await
        .unwrap();
    }

    async fn setup_test_product(pool: &PgPool, tenant_id: Uuid, product_id: Uuid) {
        sqlx::query!(
            "INSERT INTO commerce.products
             (id, tenant_id, sku, name, product_type, status, base_price, price_type,
              requires_shipping, is_digital, track_inventory, created_by, updated_by)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
            product_id,
            tenant_id,
            format!("TEST-{}", product_id),
            "Test Product",
            "simple" as &str,
            "active" as &str,
            Decimal::new(2000, 2), // $20.00
            "fixed" as &str,
            true,
            false,
            true,
            tenant_id, // Using tenant_id as user_id for simplicity
            tenant_id
        )
        .execute(pool)
        .await
        .unwrap();
    }
}