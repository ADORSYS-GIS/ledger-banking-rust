use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use heapless::String as HeaplessString;
/// Represents a banking product in the product catalogue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name_l1: HeaplessString<100>,
    pub name_l2: HeaplessString<100>,
    pub name_l3: HeaplessString<100>,
    pub description: HeaplessString<255>,
    pub is_active: bool,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub product_type: ProductType,
    pub rules: ProductRules,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: Uuid,
}

impl Product {
    /// Creates a new product builder.
    pub fn builder(id: Uuid, name_l1: &str, name_l2: &str, name_l3: &str, product_type: ProductType, updated_by: Uuid) -> Result<ProductBuilder, &'static str> {
        ProductBuilder::new(id, name_l1, name_l2, name_l3, product_type, updated_by)
    }
}

/// The type of banking product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductType {
    CASA,
    LOAN,
}

/// Frequency for interest posting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
}

/// Frequency for interest accrual
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccrualFrequency {
    Daily,
    BusinessDaysOnly,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductRules {
    pub minimum_balance: Decimal,
    pub maximum_balance: Option<Decimal>,
    pub daily_transaction_limit: Option<Decimal>,
    pub monthly_transaction_limit: Option<Decimal>,
    pub overdraft_allowed: bool,
    pub overdraft_limit: Option<Decimal>,
    pub interest_calculation_method: HeaplessString<50>,
    pub interest_posting_frequency: PostingFrequency,
    pub dormancy_threshold_days: i32,
    pub minimum_opening_balance: Decimal,
    pub closure_fee: Decimal,
    pub maintenance_fee: Option<Decimal>,
    pub maintenance_fee_frequency: Option<HeaplessString<50>>,
    pub default_dormancy_days: Option<i32>,
    pub default_overdraft_limit: Option<Decimal>,
    pub per_transaction_limit: Option<Decimal>,
    pub overdraft_interest_rate: Option<Decimal>,
    pub accrual_frequency: AccrualFrequency,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlMapping {
    pub product_id: Uuid,
    pub customer_account_code: HeaplessString<50>,
    pub interest_expense_code: HeaplessString<50>,
    pub fee_income_code: HeaplessString<50>,
    pub overdraft_code: Option<HeaplessString<50>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateTier {
    pub minimum_balance: Decimal,
    pub maximum_balance: Option<Decimal>,
    pub interest_rate: Decimal,
    pub tier_name: HeaplessString<100>,
}

// Display implementations for database compatibility
impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductType::CASA => write!(f, "CASA"),
            ProductType::LOAN => write!(f, "LOAN"),
        }
    }
}

/// Builder for creating `Product` instances.
pub struct ProductBuilder {
    id: Uuid,
    name_l1: HeaplessString<100>,
    name_l2: HeaplessString<100>,
    name_l3: HeaplessString<100>,
    description: HeaplessString<255>,
    is_active: bool,
    valid_from: Option<NaiveDate>,
    valid_to: Option<NaiveDate>,
    product_type: ProductType,
    rules: Option<ProductRules>,
    updated_by_person_id: Uuid,
}

impl ProductBuilder {
    /// Creates a new `ProductBuilder`.
    pub fn new(id: Uuid, name_l1: &str, name_l2: &str, name_l3: &str, product_type: ProductType, updated_by: Uuid) -> Result<Self, &'static str> {
        let name_l1_heapless = HeaplessString::try_from(name_l1).map_err(|_| "Product name_l1 is too long")?;
        let name_l2_heapless = HeaplessString::try_from(name_l2).map_err(|_| "Product name_l2 is too long")?;
        let name_l3_heapless = HeaplessString::try_from(name_l3).map_err(|_| "Product name_l3 is too long")?;
        Ok(Self {
            id,
            product_type,
            name_l1: name_l1_heapless,
            name_l2: name_l2_heapless,
            name_l3: name_l3_heapless,
            description: HeaplessString::new(),
            is_active: true,
            valid_from: None,
            valid_to: None,
            rules: None,
            updated_by_person_id: updated_by,
        })
    }

    pub fn description(mut self, description: &str) -> Result<Self, &'static str> {
        self.description = HeaplessString::try_from(description).map_err(|_| "Description is too long")?;
        Ok(self)
    }

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn valid_from(mut self, valid_from: NaiveDate) -> Self {
        self.valid_from = Some(valid_from);
        self
    }

    pub fn valid_to(mut self, valid_to: NaiveDate) -> Self {
        self.valid_to = Some(valid_to);
        self
    }

    pub fn rules(mut self, rules: ProductRules) -> Self {
        self.rules = Some(rules);
        self
    }

    pub fn build(self) -> Result<Product, &'static str> {
        let now = Utc::now();
        Ok(Product {
            id: self.id,
            name_l1: self.name_l1,
            name_l2: self.name_l2,
            name_l3: self.name_l3,
            description: self.description,
            is_active: self.is_active,
            valid_from: self.valid_from.ok_or("`valid_from` date is required")?,
            valid_to: self.valid_to,
            product_type: self.product_type,
            rules: self.rules.ok_or("`rules` are required")?,
            created_at: now,
            last_updated_at: now,
            updated_by_person_id: self.updated_by_person_id,
        })
    }
}