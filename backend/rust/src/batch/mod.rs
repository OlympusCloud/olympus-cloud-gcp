//! Batch operations for efficient bulk processing

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::{stream, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{RwLock, Semaphore};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Batch operation configuration
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of items per batch
    pub max_batch_size: usize,
    /// Maximum concurrent batches
    pub max_concurrency: usize,
    /// Timeout for individual operations
    pub operation_timeout: Duration,
    /// Timeout for entire batch
    pub batch_timeout: Duration,
    /// Enable automatic retry on failure
    pub enable_retry: bool,
    /// Maximum retry attempts
    pub max_retries: usize,
    /// Retry delay
    pub retry_delay: Duration,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            max_concurrency: 10,
            operation_timeout: Duration::from_secs(30),
            batch_timeout: Duration::from_secs(300), // 5 minutes
            enable_retry: true,
            max_retries: 3,
            retry_delay: Duration::from_millis(500),
        }
    }
}

/// Batch operation request
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest<T> {
    pub operations: Vec<BatchOperation<T>>,
    pub options: Option<BatchOptions>,
}

/// Individual batch operation
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOperation<T> {
    pub id: String,
    pub operation_type: String,
    pub data: T,
}

/// Batch operation options
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOptions {
    pub continue_on_error: Option<bool>,
    pub max_concurrency: Option<usize>,
    pub timeout_seconds: Option<u64>,
}

/// Batch operation response
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse {
    pub results: Vec<BatchResult>,
    pub summary: BatchSummary,
}

/// Individual batch result
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResult {
    pub id: String,
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub duration_ms: u64,
}

/// Batch operation summary
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSummary {
    pub total_operations: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub total_duration_ms: u64,
    pub average_duration_ms: u64,
}

/// Batch processor for handling bulk operations
pub struct BatchProcessor {
    config: BatchConfig,
    pool: PgPool,
    semaphore: Arc<Semaphore>,
    active_batches: Arc<RwLock<HashMap<String, BatchStatus>>>,
}

/// Batch status tracking
#[derive(Debug, Clone)]
pub struct BatchStatus {
    pub id: String,
    pub started_at: Instant,
    pub total_operations: usize,
    pub completed_operations: usize,
    pub failed_operations: usize,
    pub status: BatchState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BatchState {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl BatchProcessor {
    pub fn new(config: BatchConfig, pool: PgPool) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrency));

        Self {
            config,
            pool,
            semaphore,
            active_batches: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Process batch of product operations
    pub async fn process_product_batch(
        &self,
        batch_request: BatchRequest<ProductBatchData>,
    ) -> Result<BatchResponse, BatchError> {
        let batch_id = Uuid::new_v4().to_string();
        let start_time = Instant::now();

        // Initialize batch status
        let batch_status = BatchStatus {
            id: batch_id.clone(),
            started_at: start_time,
            total_operations: batch_request.operations.len(),
            completed_operations: 0,
            failed_operations: 0,
            status: BatchState::Running,
        };

        self.active_batches.write().await.insert(batch_id.clone(), batch_status);

        info!("Starting product batch processing: {} operations", batch_request.operations.len());

        let continue_on_error = batch_request.options
            .as_ref()
            .and_then(|o| o.continue_on_error)
            .unwrap_or(true);

        let max_concurrency = batch_request.options
            .as_ref()
            .and_then(|o| o.max_concurrency)
            .unwrap_or(self.config.max_concurrency);

        let results = stream::iter(batch_request.operations)
            .map(|operation| {
                let pool = self.pool.clone();
                let semaphore = Arc::clone(&self.semaphore);
                let batch_id = batch_id.clone();
                let active_batches = Arc::clone(&self.active_batches);

                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    let operation_start = Instant::now();

                    let result = self.process_product_operation(operation, &pool).await;

                    // Update batch status
                    {
                        let mut batches = active_batches.write().await;
                        if let Some(status) = batches.get_mut(&batch_id) {
                            status.completed_operations += 1;
                            if result.success {
                                // Success count handled by total - failed
                            } else {
                                status.failed_operations += 1;
                            }
                        }
                    }

                    BatchResult {
                        id: result.id,
                        success: result.success,
                        data: result.data,
                        error: result.error,
                        duration_ms: operation_start.elapsed().as_millis() as u64,
                    }
                }
            })
            .buffer_unordered(max_concurrency)
            .collect::<Vec<_>>()
            .await;

        let total_duration = start_time.elapsed();
        let successful_operations = results.iter().filter(|r| r.success).count();
        let failed_operations = results.len() - successful_operations;

        // Update final batch status
        {
            let mut batches = self.active_batches.write().await;
            if let Some(status) = batches.get_mut(&batch_id) {
                status.status = if failed_operations == 0 {
                    BatchState::Completed
                } else {
                    BatchState::Failed
                };
            }
        }

        let summary = BatchSummary {
            total_operations: results.len(),
            successful_operations,
            failed_operations,
            total_duration_ms: total_duration.as_millis() as u64,
            average_duration_ms: if results.is_empty() {
                0
            } else {
                total_duration.as_millis() as u64 / results.len() as u64
            },
        };

        info!(
            "Batch processing completed: {}/{} successful, {}ms total",
            successful_operations, results.len(), total_duration.as_millis()
        );

        Ok(BatchResponse { results, summary })
    }

    async fn process_product_operation(
        &self,
        operation: BatchOperation<ProductBatchData>,
        pool: &PgPool,
    ) -> OperationResult {
        let operation_id = operation.id.clone();

        let result = match operation.operation_type.as_str() {
            "create" => self.create_product(operation.data, pool).await,
            "update" => self.update_product(operation.data, pool).await,
            "delete" => self.delete_product(operation.data, pool).await,
            "update_inventory" => self.update_inventory(operation.data, pool).await,
            _ => Err(BatchError::UnsupportedOperation(operation.operation_type)),
        };

        match result {
            Ok(data) => OperationResult {
                id: operation_id,
                success: true,
                data: Some(data),
                error: None,
            },
            Err(e) => {
                error!("Operation {} failed: {}", operation_id, e);
                OperationResult {
                    id: operation_id,
                    success: false,
                    data: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }

    async fn create_product(
        &self,
        data: ProductBatchData,
        pool: &PgPool,
    ) -> Result<serde_json::Value, BatchError> {
        let product_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO commerce.products (
                id, tenant_id, sku, name, description, category,
                price, tax_rate, is_active, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            product_id,
            data.tenant_id,
            data.sku,
            data.name,
            data.description,
            data.category,
            data.price,
            data.tax_rate.unwrap_or(rust_decimal::Decimal::ZERO),
            data.is_active.unwrap_or(true),
            data.metadata.unwrap_or(serde_json::Value::Null)
        )
        .execute(pool)
        .await?;

        Ok(serde_json::json!({
            "product_id": product_id,
            "sku": data.sku
        }))
    }

    async fn update_product(
        &self,
        data: ProductBatchData,
        pool: &PgPool,
    ) -> Result<serde_json::Value, BatchError> {
        let product_id = data.id.ok_or(BatchError::MissingId)?;

        sqlx::query!(
            r#"
            UPDATE commerce.products
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                category = COALESCE($3, category),
                price = COALESCE($4, price),
                tax_rate = COALESCE($5, tax_rate),
                is_active = COALESCE($6, is_active),
                metadata = COALESCE($7, metadata),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $8 AND tenant_id = $9
            "#,
            data.name,
            data.description,
            data.category,
            data.price,
            data.tax_rate,
            data.is_active,
            data.metadata,
            product_id,
            data.tenant_id
        )
        .execute(pool)
        .await?;

        Ok(serde_json::json!({
            "product_id": product_id,
            "updated": true
        }))
    }

    async fn delete_product(
        &self,
        data: ProductBatchData,
        pool: &PgPool,
    ) -> Result<serde_json::Value, BatchError> {
        let product_id = data.id.ok_or(BatchError::MissingId)?;

        // Soft delete
        sqlx::query!(
            r#"
            UPDATE commerce.products
            SET is_active = false, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1 AND tenant_id = $2
            "#,
            product_id,
            data.tenant_id
        )
        .execute(pool)
        .await?;

        Ok(serde_json::json!({
            "product_id": product_id,
            "deleted": true
        }))
    }

    async fn update_inventory(
        &self,
        data: ProductBatchData,
        pool: &PgPool,
    ) -> Result<serde_json::Value, BatchError> {
        let product_id = data.id.ok_or(BatchError::MissingId)?;
        let location_id = data.location_id.ok_or(BatchError::MissingLocationId)?;
        let quantity = data.quantity.ok_or(BatchError::MissingQuantity)?;

        sqlx::query!(
            r#"
            INSERT INTO commerce.inventory (
                id, tenant_id, product_id, location_id,
                quantity_on_hand, quantity_available, reorder_point, reorder_quantity
            ) VALUES ($1, $2, $3, $4, $5, $5, 10, 50)
            ON CONFLICT (product_id, location_id)
            DO UPDATE SET
                quantity_on_hand = $5,
                quantity_available = $5,
                updated_at = CURRENT_TIMESTAMP
            "#,
            Uuid::new_v4(),
            data.tenant_id,
            product_id,
            location_id,
            quantity
        )
        .execute(pool)
        .await?;

        Ok(serde_json::json!({
            "product_id": product_id,
            "location_id": location_id,
            "quantity": quantity
        }))
    }

    /// Get batch status
    pub async fn get_batch_status(&self, batch_id: &str) -> Option<BatchStatus> {
        self.active_batches.read().await.get(batch_id).cloned()
    }

    /// Cancel batch operation
    pub async fn cancel_batch(&self, batch_id: &str) -> bool {
        let mut batches = self.active_batches.write().await;
        if let Some(status) = batches.get_mut(batch_id) {
            status.status = BatchState::Cancelled;
            true
        } else {
            false
        }
    }

    /// Clean up completed batches
    pub async fn cleanup_batches(&self, max_age: Duration) {
        let mut batches = self.active_batches.write().await;
        let now = Instant::now();

        batches.retain(|_, status| {
            now.duration_since(status.started_at) < max_age ||
            status.status == BatchState::Running
        });
    }
}

/// Product batch data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductBatchData {
    pub id: Option<Uuid>,
    pub tenant_id: Uuid,
    pub location_id: Option<Uuid>,
    pub sku: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub tax_rate: Option<rust_decimal::Decimal>,
    pub is_active: Option<bool>,
    pub quantity: Option<i32>,
    pub metadata: Option<serde_json::Value>,
}

/// Operation result for internal processing
#[derive(Debug)]
struct OperationResult {
    id: String,
    success: bool,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

/// Batch operation handlers
pub async fn process_product_batch(
    State(processor): State<Arc<BatchProcessor>>,
    Json(request): Json<BatchRequest<ProductBatchData>>,
) -> Result<Json<BatchResponse>, BatchError> {
    let response = processor.process_product_batch(request).await?;
    Ok(Json(response))
}

pub async fn get_batch_status(
    State(processor): State<Arc<BatchProcessor>>,
    Path(batch_id): Path<String>,
) -> Result<Json<BatchStatus>, BatchError> {
    match processor.get_batch_status(&batch_id).await {
        Some(status) => Ok(Json(status)),
        None => Err(BatchError::BatchNotFound(batch_id)),
    }
}

pub async fn cancel_batch(
    State(processor): State<Arc<BatchProcessor>>,
    Path(batch_id): Path<String>,
) -> Result<Json<serde_json::Value>, BatchError> {
    let cancelled = processor.cancel_batch(&batch_id).await;

    if cancelled {
        Ok(Json(serde_json::json!({
            "batch_id": batch_id,
            "cancelled": true
        })))
    } else {
        Err(BatchError::BatchNotFound(batch_id))
    }
}

/// Batch utilities for common operations
pub mod utils {
    use super::*;

    /// Split large dataset into smaller batches
    pub fn split_into_batches<T>(items: Vec<T>, batch_size: usize) -> Vec<Vec<T>> {
        items
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Calculate optimal batch size based on item size and memory constraints
    pub fn calculate_optimal_batch_size(
        item_size_bytes: usize,
        max_memory_mb: usize,
        max_batch_size: usize,
    ) -> usize {
        let max_memory_bytes = max_memory_mb * 1024 * 1024;
        let calculated_size = max_memory_bytes / item_size_bytes;

        std::cmp::min(calculated_size, max_batch_size)
    }

    /// Validate batch request
    pub fn validate_batch_request<T>(request: &BatchRequest<T>) -> Result<(), BatchError> {
        if request.operations.is_empty() {
            return Err(BatchError::EmptyBatch);
        }

        if request.operations.len() > 1000 {
            return Err(BatchError::BatchTooLarge(request.operations.len()));
        }

        // Check for duplicate operation IDs
        let mut ids = std::collections::HashSet::new();
        for operation in &request.operations {
            if !ids.insert(&operation.id) {
                return Err(BatchError::DuplicateOperationId(operation.id.clone()));
            }
        }

        Ok(())
    }

    /// Create progress tracker for batch operations
    pub fn create_progress_tracker(total_operations: usize) -> BatchProgressTracker {
        BatchProgressTracker::new(total_operations)
    }
}

/// Progress tracking for batch operations
pub struct BatchProgressTracker {
    total: usize,
    completed: std::sync::atomic::AtomicUsize,
    failed: std::sync::atomic::AtomicUsize,
    start_time: Instant,
}

impl BatchProgressTracker {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: std::sync::atomic::AtomicUsize::new(0),
            failed: std::sync::atomic::AtomicUsize::new(0),
            start_time: Instant::now(),
        }
    }

    pub fn record_success(&self) {
        self.completed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn record_failure(&self) {
        self.failed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_progress(&self) -> BatchProgress {
        let completed = self.completed.load(std::sync::atomic::Ordering::Relaxed);
        let failed = self.failed.load(std::sync::atomic::Ordering::Relaxed);
        let processed = completed + failed;

        BatchProgress {
            total: self.total,
            processed,
            completed,
            failed,
            percentage: if self.total > 0 {
                (processed as f64 / self.total as f64) * 100.0
            } else {
                0.0
            },
            elapsed_ms: self.start_time.elapsed().as_millis() as u64,
            estimated_remaining_ms: if processed > 0 {
                let avg_time_per_item = self.start_time.elapsed().as_millis() as u64 / processed as u64;
                avg_time_per_item * (self.total - processed) as u64
            } else {
                0
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchProgress {
    pub total: usize,
    pub processed: usize,
    pub completed: usize,
    pub failed: usize,
    pub percentage: f64,
    pub elapsed_ms: u64,
    pub estimated_remaining_ms: u64,
}

/// Batch error types
#[derive(Debug, thiserror::Error)]
pub enum BatchError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Batch not found: {0}")]
    BatchNotFound(String),

    #[error("Empty batch request")]
    EmptyBatch,

    #[error("Batch too large: {0} operations (max 1000)")]
    BatchTooLarge(usize),

    #[error("Duplicate operation ID: {0}")]
    DuplicateOperationId(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Missing required field: id")]
    MissingId,

    #[error("Missing required field: location_id")]
    MissingLocationId,

    #[error("Missing required field: quantity")]
    MissingQuantity,

    #[error("Operation timeout")]
    Timeout,

    #[error("Batch processing failed: {0}")]
    ProcessingFailed(String),
}

impl IntoResponse for BatchError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            BatchError::BatchNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            BatchError::EmptyBatch | BatchError::BatchTooLarge(_) |
            BatchError::DuplicateOperationId(_) | BatchError::UnsupportedOperation(_) |
            BatchError::MissingId | BatchError::MissingLocationId | BatchError::MissingQuantity => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            BatchError::Timeout => (StatusCode::REQUEST_TIMEOUT, self.to_string()),
            BatchError::Database(_) | BatchError::ProcessingFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Batch processing failed".to_string())
            }
        };

        let body = serde_json::json!({
            "error": {
                "code": "BATCH_ERROR",
                "message": message
            }
        });

        (status, axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_config_default() {
        let config = BatchConfig::default();
        assert_eq!(config.max_batch_size, 100);
        assert_eq!(config.max_concurrency, 10);
        assert!(config.enable_retry);
    }

    #[test]
    fn test_split_into_batches() {
        let items = (1..=25).collect::<Vec<i32>>();
        let batches = utils::split_into_batches(items, 10);

        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].len(), 10);
        assert_eq!(batches[1].len(), 10);
        assert_eq!(batches[2].len(), 5);
    }

    #[test]
    fn test_calculate_optimal_batch_size() {
        let batch_size = utils::calculate_optimal_batch_size(1024, 10, 1000);
        assert!(batch_size <= 1000);
        assert!(batch_size > 0);
    }

    #[test]
    fn test_progress_tracker() {
        let tracker = BatchProgressTracker::new(100);
        tracker.record_success();
        tracker.record_success();
        tracker.record_failure();

        let progress = tracker.get_progress();
        assert_eq!(progress.total, 100);
        assert_eq!(progress.processed, 3);
        assert_eq!(progress.completed, 2);
        assert_eq!(progress.failed, 1);
        assert_eq!(progress.percentage, 3.0);
    }
}