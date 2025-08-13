use std::sync::Arc;
use async_trait::async_trait;
use rust_decimal::Decimal;
use uuid::Uuid;
use banking_api::{
    error::BankingResult,
    domain::{Product, ProductRules, InterestRateTier, GlMapping, ProductType},
    domain::fee::ProductFeeSchedule,
    service::ProductService,
};
use banking_db::repository::ProductRepository;
use crate::mappers::product_mapper;

pub struct ProductServiceImpl<R: ProductRepository> {
    product_repository: Arc<R>,
}

impl<R: ProductRepository> ProductServiceImpl<R> {
    pub fn new(product_repository: Arc<R>) -> Self {
        Self {
            product_repository,
        }
    }
}

#[async_trait]
impl<R: ProductRepository + Send + Sync> ProductService for ProductServiceImpl<R> {
    async fn create_product(&self, product: Product) -> BankingResult<Product> {
        let db_product = product_mapper::ProductMapper::to_db(product);
        let created_product = self.product_repository.create_product(db_product).await?;
        Ok(product_mapper::ProductMapper::from_db(created_product))
    }

    async fn find_product_by_id(&self, product_id: Uuid) -> BankingResult<Option<Product>> {
        let db_product = self.product_repository.find_product_by_id(product_id).await?;
        Ok(db_product.map(product_mapper::ProductMapper::from_db))
    }

    async fn update_product(&self, product: Product) -> BankingResult<Product> {
        let db_product = product_mapper::ProductMapper::to_db(product);
        let updated_product = self.product_repository.update_product(db_product).await?;
        Ok(product_mapper::ProductMapper::from_db(updated_product))
    }

    async fn deactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()> {
        self.product_repository.deactivate_product(product_id, updated_by_person_id).await
    }

    async fn reactivate_product(&self, product_id: Uuid, updated_by_person_id: Uuid) -> BankingResult<()> {
        self.product_repository.reactivate_product(product_id, updated_by_person_id).await
    }

    async fn find_active_products(&self) -> BankingResult<Vec<Product>> {
        let db_products = self.product_repository.find_active_products().await?;
        Ok(db_products.into_iter().map(product_mapper::ProductMapper::from_db).collect())
    }

    async fn find_products_by_type(&self, product_type: ProductType) -> BankingResult<Vec<Product>> {
        let db_product_type = match product_type {
            ProductType::CASA => banking_db::models::ProductType::CASA,
            ProductType::LOAN => banking_db::models::ProductType::LOAN,
        };
        let db_products = self.product_repository.find_products_by_type(db_product_type).await?;
        Ok(db_products.into_iter().map(product_mapper::ProductMapper::from_db).collect())
    }

    async fn get_product_rules(&self, product_id: Uuid) -> BankingResult<ProductRules> {
        let product = self.find_product_by_id(product_id).await?
            .ok_or(banking_api::BankingError::ProductNotFound(product_id))?;
        Ok(product.rules)
    }

    async fn get_interest_rate(&self, product_id: Uuid, _balance_tier: Decimal) -> BankingResult<Decimal> {
        let _product = self.find_product_by_id(product_id).await?
            .ok_or(banking_api::BankingError::ProductNotFound(product_id))?;
        // Find the correct tier and return the rate
        Ok(Decimal::new(5, 2)) // 5% placeholder
    }

    async fn get_interest_rate_tiers(&self, product_id: Uuid) -> BankingResult<Vec<InterestRateTier>> {
        let _product = self.find_product_by_id(product_id).await?
            .ok_or(banking_api::BankingError::ProductNotFound(product_id))?;
        // This would be part of the product rules in a real implementation
        Ok(vec![])
    }

    async fn get_fee_schedule(&self, product_id: Uuid) -> BankingResult<ProductFeeSchedule> {
        let _product = self.find_product_by_id(product_id).await?
            .ok_or(banking_api::BankingError::ProductNotFound(product_id))?;
        // This would be part of the product rules in a real implementation
        Ok(ProductFeeSchedule {
            product_id,
            fees: vec![],
            effective_from: chrono::Utc::now().naive_utc().date(),
            effective_to: None,
        })
    }

    async fn get_gl_mapping(&self, product_id: Uuid) -> BankingResult<GlMapping> {
        let _product = self.find_product_by_id(product_id).await?
            .ok_or(banking_api::BankingError::ProductNotFound(product_id))?;
        // This would be part of the product rules in a real implementation
        Ok(GlMapping {
            product_id,
            customer_account_code: heapless::String::try_from("1001").unwrap(),
            interest_expense_code: heapless::String::try_from("7001").unwrap(),
            fee_income_code: heapless::String::try_from("4001").unwrap(),
            overdraft_code: Some(heapless::String::try_from("1002").unwrap()),
        })
    }
}