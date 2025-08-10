use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{Product, ProductType};
use banking_api::error::BankingResult;

#[async_trait]
pub trait ProductRepository {
    async fn create_product(&self, product: Product) -> BankingResult<Product>;
    async fn find_product_by_id(&self, product_id: Uuid) -> BankingResult<Option<Product>>;
    async fn update_product(&self, product: Product) -> BankingResult<Product>;
    async fn deactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()>;
    async fn reactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()>;
    async fn find_active_products(&self) -> BankingResult<Vec<Product>>;
    async fn find_products_by_type(&self, product_type: ProductType) -> BankingResult<Vec<Product>>;
}