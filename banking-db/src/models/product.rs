use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a banking product in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub product_type: ProductType,
    pub name: HeaplessString<100>,
    pub description: HeaplessString<255>,
    pub is_active: bool,
    pub valid_from: NaiveDate,
    pub valid_to: Option<NaiveDate>,
    /// A string containing the rules for the product, to be interpreted by a rule engine.
    pub rules: HeaplessString<500>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub updated_by_person_id: Uuid,
}

/// The type of banking product.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProductType {
    CASA,
    LOAN,
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