use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use banking_api::error::BankingResult;
use banking_db::{
    models::{ProductModel, ProductType, product::{InterestRateTierModel, GlMappingModel}},
    repository::ProductRepository,
};

pub struct ProductRepositoryImpl {
    pool: PgPool,
}

impl ProductRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn create_product(&self, _product: ProductModel) -> BankingResult<ProductModel> {
        todo!()
    }

    async fn find_product_by_id(&self, _product_id: Uuid) -> BankingResult<Option<ProductModel>> {
        todo!()
    }

    async fn update_product(&self, _product: ProductModel) -> BankingResult<ProductModel> {
        todo!()
    }

    async fn deactivate_product(&self, _product_id: Uuid, _updated_by_person_id: Uuid) -> BankingResult<()> {
        todo!()
    }

    async fn reactivate_product(&self, _product_id: Uuid, _updated_by_person_id: Uuid) -> BankingResult<()> {
        todo!()
    }

    async fn find_active_products(&self) -> BankingResult<Vec<ProductModel>> {
        todo!()
    }

    async fn find_products_by_type(&self, _product_type: ProductType) -> BankingResult<Vec<ProductModel>> {
        todo!()
    }

    async fn find_interest_rate_tiers_by_product_id(
        &self,
        product_id: Uuid,
    ) -> BankingResult<Vec<InterestRateTierModel>> {
        let rows = sqlx::query(
            r#"
            SELECT
                tier_name,
                minimum_balance,
                maximum_balance,
                interest_rate
            FROM interest_rate_tiers
            WHERE product_id = $1
            ORDER BY tier_name
            "#,
        )
        .bind(product_id)
        .fetch_all(&self.pool)
        .await?;

        let mut tiers = Vec::new();
        for row in rows {
            let tier_name: String = row.get("tier_name");
            tiers.push(InterestRateTierModel {
                tier_name: heapless::String::try_from(tier_name.as_str())
                    .map_err(|_| banking_api::error::BankingError::ValidationError {
                        field: "tier_name".to_string(),
                        message: "tier_name too long".to_string()
                    })?,
                minimum_balance: row.get("minimum_balance"),
                maximum_balance: row.get("maximum_balance"),
                interest_rate: row.get("interest_rate"),
            });
        }

        Ok(tiers)
    }

    async fn find_gl_mapping_by_product_id(
        &self,
        product_id: Uuid,
    ) -> BankingResult<Option<GlMappingModel>> {
        let row = sqlx::query(
            r#"
            SELECT
                customer_account_code,
                interest_expense_code,
                fee_income_code,
                overdraft_code
            FROM gl_mappings
            WHERE product_id = $1
            "#,
        )
        .bind(product_id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let customer_account_code: String = row.get("customer_account_code");
                let interest_expense_code: String = row.get("interest_expense_code");
                let fee_income_code: String = row.get("fee_income_code");
                let overdraft_code: Option<String> = row.get("overdraft_code");

                Ok(Some(GlMappingModel {
                    product_id,
                    customer_account_code: heapless::String::try_from(customer_account_code.as_str())
                        .map_err(|_| banking_api::error::BankingError::ValidationError {
                            field: "customer_account_code".to_string(),
                            message: "customer_account_code too long".to_string()
                        })?,
                    interest_expense_code: heapless::String::try_from(interest_expense_code.as_str())
                        .map_err(|_| banking_api::error::BankingError::ValidationError {
                            field: "interest_expense_code".to_string(),
                            message: "interest_expense_code too long".to_string()
                        })?,
                    fee_income_code: heapless::String::try_from(fee_income_code.as_str())
                        .map_err(|_| banking_api::error::BankingError::ValidationError {
                            field: "fee_income_code".to_string(),
                            message: "fee_income_code too long".to_string()
                        })?,
                    overdraft_code: match overdraft_code {
                        Some(code) => Some(heapless::String::try_from(code.as_str())
                            .map_err(|_| banking_api::error::BankingError::ValidationError {
                                field: "overdraft_code".to_string(),
                                message: "overdraft_code too long".to_string()
                            })?),
                        None => None,
                    },
                }))
            }
            None => Ok(None),
        }
    }
}