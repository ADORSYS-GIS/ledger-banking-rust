use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a banking product in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductModel {
    pub id: Uuid,
    pub name_l1: heapless::String<100>,
    pub name_l2: heapless::String<100>,
    pub name_l3: heapless::String<100>,
    pub description: heapless::String<255>,
    pub is_active: bool,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    pub product_type: ProductType,
    pub rules: ProductRules,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by_person_id: Uuid,
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
    pub interest_calculation_method: heapless::String<50>,
    pub interest_posting_frequency: PostingFrequency,
    pub dormancy_threshold_days: i32,
    pub minimum_opening_balance: Decimal,
    pub closure_fee: Decimal,
    pub maintenance_fee: Option<Decimal>,
    pub maintenance_fee_frequency: Option<heapless::String<50>>,
    pub default_dormancy_days: Option<i32>,
    pub default_overdraft_limit: Option<Decimal>,
    pub per_transaction_limit: Option<Decimal>,
    pub overdraft_interest_rate: Option<Decimal>,
    pub accrual_frequency: AccrualFrequency,
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

impl std::str::FromStr for ProductType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CASA" => Ok(ProductType::CASA),
            "LOAN" => Ok(ProductType::LOAN),
            _ => Err(format!("Invalid ProductType: {s}")),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlMappingModel {
    pub product_id: Uuid,
    pub customer_account_code: heapless::String<50>,
    pub interest_expense_code: heapless::String<50>,
    pub fee_income_code: heapless::String<50>,
    pub overdraft_code: Option<heapless::String<50>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestRateTierModel {
    pub minimum_balance: Decimal,
    pub maximum_balance: Option<Decimal>,
    pub interest_rate: Decimal,
    pub tier_name: heapless::String<100>,
}

