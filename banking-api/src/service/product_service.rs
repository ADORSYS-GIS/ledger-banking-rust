use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use crate::{
    error::BankingResult,
    domain::{Product, ProductRules, InterestRateTier, GlMapping, ProductType},
    domain::fee::ProductFeeSchedule,
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

    /// Get the rules for a specific product.
    async fn get_product_rules(&self, product_id: Uuid) -> BankingResult<ProductRules>;

    /// Get the interest rate for a product based on a balance tier.
    async fn get_interest_rate(&self, product_id: Uuid, balance_tier: Decimal) -> BankingResult<Decimal>;

    /// Get the interest rate tiers for a product.
    async fn get_interest_rate_tiers(&self, product_id: Uuid) -> BankingResult<Vec<InterestRateTier>>;

    /// Get the fee schedule for a product.
    async fn get_fee_schedule(&self, product_id: Uuid) -> BankingResult<ProductFeeSchedule>;

    /// Get the GL mapping for a product.
    async fn get_gl_mapping(&self, product_id: Uuid) -> BankingResult<GlMapping>;
}