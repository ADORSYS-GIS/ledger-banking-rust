use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use uuid::Uuid;

/// Trait for batch operations on repositories
/// Provides efficient bulk insert, update, and load operations
#[async_trait]
pub trait BatchRepository<DB: Database, T>: Send + Sync {
    /// Save multiple items in a single transaction
    /// Returns saved items with any generated fields populated
    async fn save_batch(
        &self,
        items: Vec<T>,
        audit_log_id: Uuid,
    ) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;

    /// Load multiple items by their IDs
    /// Returns items in the same order as the provided IDs
    /// Missing items are represented as None in the result
    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<T>>, Box<dyn Error + Send + Sync>>;

    /// Update multiple items in a single transaction
    /// Only updates items that have changed (based on hash comparison)
    async fn update_batch(
        &self,
        items: Vec<T>,
        audit_log_id: Uuid,
    ) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;

    /// Delete multiple items by their IDs
    /// Returns the number of items deleted
    async fn delete_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<usize, Box<dyn Error + Send + Sync>>;

}

/// Statistics for batch operations
#[derive(Debug, Clone, Default)]
pub struct BatchOperationStats {
    pub total_items: usize,
    pub successful_items: usize,
    pub failed_items: usize,
    pub skipped_items: usize,  // For updates where no changes detected
    pub duration_ms: u64,
}

/// Result of a batch operation with statistics
#[derive(Debug)]
pub struct BatchResult<T> {
    pub items: Vec<T>,
    pub stats: BatchOperationStats,
    pub errors: Vec<(usize, Box<dyn Error + Send + Sync>)>,  // (index, error)
}

impl<T> BatchResult<T> {
    pub fn new(items: Vec<T>) -> Self {
        let count = items.len();
        Self {
            items,
            stats: BatchOperationStats {
                total_items: count,
                successful_items: count,
                ..Default::default()
            },
            errors: Vec::new(),
        }
    }

    pub fn with_stats(mut self, stats: BatchOperationStats) -> Self {
        self.stats = stats;
        self
    }

    pub fn with_errors(mut self, errors: Vec<(usize, Box<dyn Error + Send + Sync>)>) -> Self {
        self.stats.failed_items = errors.len();
        self.errors = errors;
        self
    }
}

/// Options for batch operations
#[derive(Debug, Clone)]
pub struct BatchOptions {
    /// Maximum number of items to process in a single database round-trip
    pub chunk_size: usize,
    /// Whether to continue on error or stop at first failure
    pub continue_on_error: bool,
    /// Whether to use a transaction for the entire batch
    pub use_transaction: bool,
    /// Timeout for the entire batch operation (in seconds)
    pub timeout_seconds: Option<u64>,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            chunk_size: 1000,
            continue_on_error: false,
            use_transaction: true,
            timeout_seconds: Some(60),
        }
    }
}