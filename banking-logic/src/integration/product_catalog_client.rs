use std::time::Duration;
use banking_api::BankingResult;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use moka::future::Cache;

/// Frequency for interest posting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PostingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

/// Frequency for interest accrual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccrualFrequency {
    Daily,
    BusinessDaysOnly,
    None,
}

/// Product Catalog Client - Tier-1 critical service integration
#[derive(Debug, Clone)]
pub struct ProductCatalogClient {
    http_client: Client,
    base_url: String,
    #[allow(dead_code)]
    timeout: Duration,
    cache: Arc<Cache<String, ProductRules>>,
}

impl ProductCatalogClient {
    pub fn new(base_url: String) -> BankingResult<Self> {
        let timeout = Duration::from_secs(30);
        let cache = Arc::new(
            Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(300)) // 5 minutes cache
                .build()
        );

        Ok(Self {
            http_client: Client::builder()
                .timeout(timeout)
                .build()
                .map_err(|e| banking_api::BankingError::Internal(e.to_string()))?,
            base_url,
            timeout,
            cache,
        })
    }

    pub fn new_with_timeout(base_url: String, timeout: Duration) -> BankingResult<Self> {
        let cache = Arc::new(
            Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(300)) // 5 minutes cache
                .build()
        );

        Ok(Self {
            http_client: Client::builder()
                .timeout(timeout)
                .build()
                .map_err(|e| banking_api::BankingError::Internal(e.to_string()))?,
            base_url,
            timeout,
            cache,
        })
    }

    pub async fn get_product_rules(&self, product_code: &str) -> BankingResult<ProductRules> {
        // Check cache first
        if let Some(cached_rules) = self.cache.get(product_code).await {
            return Ok(cached_rules);
        }

        // Fetch from API
        let url = format!("{}/products/{}/rules", self.base_url, product_code);
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: e.to_string(),
                retry_possible: true,
            })?;

        let rules: ProductRules = response
            .json()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: format!("Failed to parse response: {}", e),
                retry_possible: false,
            })?;

        // Cache the result
        self.cache.insert(product_code.to_string(), rules.clone()).await;

        Ok(rules)
    }

    pub async fn get_interest_rate(&self, product_code: &str, balance_tier: Decimal) -> BankingResult<Decimal> {
        let url = format!("{}/products/{}/interest-rate?balance={}", 
                         self.base_url, product_code, balance_tier);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: e.to_string(),
                retry_possible: true,
            })?;

        let rate_response: InterestRateResponse = response
            .json()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: format!("Failed to parse interest rate response: {}", e),
                retry_possible: false,
            })?;

        Ok(rate_response.rate)
    }

    pub async fn get_fee_schedule(&self, product_code: &str) -> BankingResult<FeeSchedule> {
        let url = format!("{}/products/{}/fees", self.base_url, product_code);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: e.to_string(),
                retry_possible: true,
            })?;

        let fee_schedule: FeeSchedule = response
            .json()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: format!("Failed to parse fee schedule: {}", e),
                retry_possible: false,
            })?;

        Ok(fee_schedule)
    }

    pub async fn get_gl_mapping(&self, product_code: &str) -> BankingResult<GlMapping> {
        let url = format!("{}/products/{}/gl-mapping", self.base_url, product_code);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: e.to_string(),
                retry_possible: true,
            })?;

        let gl_mapping: GlMapping = response
            .json()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: format!("Failed to parse GL mapping: {}", e),
                retry_possible: false,
            })?;

        Ok(gl_mapping)
    }
    
    pub async fn get_interest_rate_tiers(&self, product_code: &str) -> BankingResult<Vec<InterestRateTier>> {
        let url = format!("{}/products/{}/interest-rate-tiers", self.base_url, product_code);
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: e.to_string(),
                retry_possible: true,
            })?;

        let tiers: Vec<InterestRateTier> = response
            .json()
            .await
            .map_err(|e| banking_api::BankingError::NetworkError {
                error_details: format!("Failed to parse interest rate tiers: {}", e),
                retry_possible: false,
            })?;

        Ok(tiers)
    }

    /// Invalidate cache entry for a product
    pub async fn invalidate_cache(&self, product_code: &str) {
        self.cache.invalidate(product_code).await;
    }

    /// Clear entire cache
    pub async fn clear_cache(&self) {
        self.cache.invalidate_all();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRules {
    pub product_code: String,
    pub minimum_balance: Decimal,
    pub maximum_balance: Option<Decimal>,
    pub daily_transaction_limit: Option<Decimal>,
    pub monthly_transaction_limit: Option<Decimal>,
    pub overdraft_allowed: bool,
    pub overdraft_limit: Option<Decimal>,
    pub interest_calculation_method: String,
    pub interest_posting_frequency: PostingFrequency,
    pub dormancy_threshold_days: i32,
    pub minimum_opening_balance: Decimal,
    pub closure_fee: Decimal,
    pub maintenance_fee: Option<Decimal>,
    pub maintenance_fee_frequency: Option<String>,
    pub default_dormancy_days: Option<i32>,
    pub default_overdraft_limit: Option<Decimal>,
    pub per_transaction_limit: Option<Decimal>,
    pub overdraft_interest_rate: Option<Decimal>,
    pub accrual_frequency: AccrualFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateResponse {
    pub rate: Decimal,
    pub tier: String,
    pub effective_from: chrono::NaiveDate,
    pub effective_to: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeSchedule {
    pub product_code: String,
    pub fees: Vec<Fee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    pub fee_type: String,
    pub amount: Decimal,
    pub currency: String,
    pub frequency: String,
    pub applies_to: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlMapping {
    pub product_code: String,
    pub customer_account_code: String,
    pub interest_expense_code: String,
    pub fee_income_code: String,
    pub overdraft_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateTier {
    pub minimum_balance: Decimal,
    pub maximum_balance: Option<Decimal>,
    pub interest_rate: Decimal,
    pub tier_name: String,
}