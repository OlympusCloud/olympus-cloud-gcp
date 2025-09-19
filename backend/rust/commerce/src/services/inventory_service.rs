use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use olympus_shared::database::Database;
use olympus_shared::types::Money;
use crate::models::{StockTransaction, StockLevel, TransactionType};

pub struct InventoryService {
    _db: Arc<Database>,
}

impl InventoryService {
    pub fn new(db: Arc<Database>) -> Self {
        Self { _db: db }
    }

    pub async fn process_stock_transaction(&self, transaction: StockTransaction) -> Result<StockLevel, String> {
        // Validate transaction
        if transaction.quantity == 0 {
            return Err("Transaction quantity cannot be zero".to_string());
        }

        // Get current stock level
        let mut stock_level = self.get_stock_level(transaction.product_id, transaction.location_id).await?;

        // Process transaction based on type
        match transaction.transaction_type {
            TransactionType::Sale | TransactionType::Waste | TransactionType::Damage => {
                if stock_level.quantity_on_hand < transaction.quantity.abs() {
                    return Err("Insufficient stock".to_string());
                }
                stock_level.quantity_on_hand -= transaction.quantity.abs();
            },
            TransactionType::Purchase | TransactionType::Return | TransactionType::Adjustment => {
                stock_level.quantity_on_hand += transaction.quantity.abs();
            },
            TransactionType::Transfer => {
                // For transfers, this would be more complex with source/destination
                stock_level.quantity_on_hand -= transaction.quantity.abs();
            },
        }

        // Update timestamps
        stock_level.updated_at = Utc::now();

        // Check for low stock alerts
        if stock_level.quantity_on_hand <= stock_level.reorder_point {
            // TODO: Publish low stock event
            println!("LOW STOCK ALERT: Product {} at location {} is below reorder point", 
                transaction.product_id, transaction.location_id);
        }

        // TODO: Save transaction and updated stock level to database
        // TODO: Publish inventory.updated event

        Ok(stock_level)
    }

    pub async fn get_stock_level(&self, product_id: Uuid, location_id: Uuid) -> Result<StockLevel, String> {
        // Mock stock level - in production, fetch from database
        Ok(StockLevel {
            id: Uuid::new_v4(),
            product_id,
            location_id,
            quantity_on_hand: 100,
            quantity_reserved: 5,
            reorder_point: 20,
            max_stock_level: 500,
            unit_cost: Money::new(1250, olympus_shared::types::Currency::USD),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn reserve_stock(&self, product_id: Uuid, location_id: Uuid, quantity: i32) -> Result<(), String> {
        let mut stock_level = self.get_stock_level(product_id, location_id).await?;
        
        let available = stock_level.quantity_on_hand - stock_level.quantity_reserved;
        if available < quantity {
            return Err("Insufficient available stock for reservation".to_string());
        }

        stock_level.quantity_reserved += quantity;
        stock_level.updated_at = Utc::now();

        // TODO: Update database
        // TODO: Publish stock.reserved event

        Ok(())
    }

    pub async fn release_reservation(&self, product_id: Uuid, location_id: Uuid, quantity: i32) -> Result<(), String> {
        let mut stock_level = self.get_stock_level(product_id, location_id).await?;
        
        if stock_level.quantity_reserved < quantity {
            return Err("Cannot release more than reserved".to_string());
        }

        stock_level.quantity_reserved -= quantity;
        stock_level.updated_at = Utc::now();

        // TODO: Update database
        // TODO: Publish stock.released event

        Ok(())
    }

    pub async fn transfer_stock(
        &self, 
        product_id: Uuid, 
        from_location: Uuid, 
        to_location: Uuid, 
        quantity: i32
    ) -> Result<(StockLevel, StockLevel), String> {
        // Validate source has enough stock
        let mut from_stock = self.get_stock_level(product_id, from_location).await?;
        if from_stock.quantity_on_hand < quantity {
            return Err("Insufficient stock at source location".to_string());
        }

        // Process outbound transaction
        let outbound_transaction = StockTransaction {
            id: Uuid::new_v4(),
            product_id,
            location_id: from_location,
            transaction_type: TransactionType::Transfer,
            quantity: -quantity, // Negative for outbound
            unit_cost: from_stock.unit_cost.clone(),
            reference_id: None,
            notes: Some(format!("Transfer to location {}", to_location)),
            created_at: Utc::now(),
        };

        // Process inbound transaction
        let inbound_transaction = StockTransaction {
            id: Uuid::new_v4(),
            product_id,
            location_id: to_location,
            transaction_type: TransactionType::Transfer,
            quantity, // Positive for inbound
            unit_cost: from_stock.unit_cost.clone(),
            reference_id: None,
            notes: Some(format!("Transfer from location {}", from_location)),
            created_at: Utc::now(),
        };

        // Process both transactions
        let updated_from = self.process_stock_transaction(outbound_transaction).await?;
        let updated_to = self.process_stock_transaction(inbound_transaction).await?;

        // TODO: Create transfer record linking both transactions
        // TODO: Publish stock.transferred event

        Ok((updated_from, updated_to))
    }

    pub async fn adjust_stock(
        &self,
        product_id: Uuid,
        location_id: Uuid,
        adjustment_quantity: i32,
        reason: String,
    ) -> Result<StockLevel, String> {
        let transaction = StockTransaction {
            id: Uuid::new_v4(),
            product_id,
            location_id,
            transaction_type: TransactionType::Adjustment,
            quantity: adjustment_quantity,
            unit_cost: Money::new(0, olympus_shared::types::Currency::USD), // No cost for adjustments
            reference_id: None,
            notes: Some(reason),
            created_at: Utc::now(),
        };

        self.process_stock_transaction(transaction).await
    }
}