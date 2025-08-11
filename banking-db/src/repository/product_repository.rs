use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{ProductModel, ProductType};
use banking_api::error::BankingResult;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create_product(&self, product: ProductModel) -> BankingResult<ProductModel>;
    async fn find_product_by_id(&self, product_id: Uuid) -> BankingResult<Option<ProductModel>>;
    async fn update_product(&self, product: ProductModel) -> BankingResult<ProductModel>;
    async fn deactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()>;
    async fn reactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()>;
    async fn find_active_products(&self) -> BankingResult<Vec<ProductModel>>;
    async fn find_products_by_type(&self, product_type: ProductType) -> BankingResult<Vec<ProductModel>>;
    async fn find_interest_rate_tiers_by_product_id(&self, product_id: Uuid) -> BankingResult<Vec<crate::models::product::InterestRateTierModel>>;
    async fn find_gl_mapping_by_product_id(&self, product_id: Uuid) -> BankingResult<Option<crate::models::product::GlMappingModel>>;
}