use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a banking product in the product catalogue.
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

impl Product {
    /// Creates a new product builder.
    pub fn builder(id: Uuid, product_type: ProductType, name: &str, updated_by: Uuid) -> Result<ProductBuilder, &'static str> {
        ProductBuilder::new(id, product_type, name, updated_by)
    }
}

/// Builder for creating `Product` instances.
pub struct ProductBuilder {
    id: Uuid,
    product_type: ProductType,
    name: HeaplessString<100>,
    description: HeaplessString<255>,
    is_active: bool,
    valid_from: Option<NaiveDate>,
    valid_to: Option<NaiveDate>,
    rules: HeaplessString<500>,
    updated_by_person_id: Uuid,
}

impl ProductBuilder {
    /// Creates a new `ProductBuilder`.
    pub fn new(id: Uuid, product_type: ProductType, name: &str, updated_by: Uuid) -> Result<Self, &'static str> {
        let name_heapless = HeaplessString::try_from(name).map_err(|_| "Product name is too long")?;
        Ok(Self {
            id,
            product_type,
            name: name_heapless,
            description: HeaplessString::new(),
            is_active: true,
            valid_from: None,
            valid_to: None,
            rules: HeaplessString::new(),
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

    pub fn rules(mut self, rules: &str) -> Result<Self, &'static str> {
        self.rules = HeaplessString::try_from(rules).map_err(|_| "Rules string is too long")?;
        Ok(self)
    }

    pub fn build(self) -> Result<Product, &'static str> {
        let now = Utc::now();
        Ok(Product {
            id: self.id,
            product_type: self.product_type,
            name: self.name,
            description: self.description,
            is_active: self.is_active,
            valid_from: self.valid_from.ok_or("`valid_from` date is required")?,
            valid_to: self.valid_to,
            rules: self.rules,
            created_at: now,
            last_updated_at: now,
            updated_by_person_id: self.updated_by_person_id,
        })
    }
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