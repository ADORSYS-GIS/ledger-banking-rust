use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    error::BankingResult,
    domain::{Product, ProductType},
};

/// Service for managing the product catalogue.
#[async_trait]
pub trait ProductService: Send + Sync {
    /// Create a new product.
    async fn create_product(&self, product: Product) -> BankingResult<Product>;

    /// Find a product by its ID.
    async fn find_product_by_id(&self, product_id: Uuid) -> BankingResult<Option<Product>>;

    /// Update an existing product.
    async fn update_product(&self, product: Product) -> BankingResult<Product>;

    /// Deactivate a product.
    async fn deactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()>;

    /// Reactivate a product.
    async fn reactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()>;

    /// Find all active products.
    async fn find_active_products(&self) -> BankingResult<Vec<Product>>;

    /// Find products by type.
    async fn find_products_by_type(&self, product_type: ProductType) -> BankingResult<Vec<Product>>;
}