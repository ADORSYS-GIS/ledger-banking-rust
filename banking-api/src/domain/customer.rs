use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: Uuid,
    pub customer_type: CustomerType,
    pub full_name: HeaplessString<255>,
    pub id_type: IdentityType,
    pub id_number: HeaplessString<50>,
    pub risk_rating: RiskRating,
    pub status: CustomerStatus,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub updated_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CustomerType { 
    Individual, 
    Corporate 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IdentityType { 
    NationalId, 
    Passport, 
    CompanyRegistration,
    PermanentResidentCard,
    AsylumCard,
    TemporaryResidentPermit,
    Unknown
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityType::NationalId => write!(f, "NationalId"),
            IdentityType::Passport => write!(f, "Passport"),
            IdentityType::CompanyRegistration => write!(f, "CompanyRegistration"),
            IdentityType::PermanentResidentCard => write!(f, "PermanentResidentCard"),
            IdentityType::AsylumCard => write!(f, "AsylumCard"),
            IdentityType::TemporaryResidentPermit => write!(f, "TemporaryResidentPermit"),
            IdentityType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum RiskRating { 
    Low, 
    Medium, 
    High, 
    Blacklisted
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CustomerStatus { 
    Active, 
    PendingVerification, 
    Deceased,
    Dissolved,
    Blacklisted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortfolio {
    pub customer_id: Uuid,
    pub total_accounts: i64,
    pub total_balance: rust_decimal::Decimal,
    pub total_loan_outstanding: Option<rust_decimal::Decimal>,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub risk_score: Option<rust_decimal::Decimal>,
    pub kyc_status: String,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub current_rating: RiskRating,
    pub last_assessment_date: DateTime<Utc>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerComplianceStatus {
    pub kyc_status: KycStatus,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KycStatus {
    NotStarted,
    InProgress,
    Pending,
    Complete,
    Approved,
    Rejected,
    RequiresUpdate,
    Failed,
}

/// Builder for creating Customer instances with validation
#[derive(Debug)]
pub struct CustomerBuilder {
    customer_id: Uuid,
    customer_type: CustomerType,
    full_name: String,
    id_type: IdentityType,
    id_number: String,
    risk_rating: RiskRating,
    status: CustomerStatus,
    updated_by: Uuid,
}

impl CustomerBuilder {
    pub fn new(customer_id: Uuid, customer_type: CustomerType) -> Self {
        Self {
            customer_id,
            customer_type,
            full_name: String::new(),
            id_type: IdentityType::NationalId,
            id_number: String::new(),
            risk_rating: RiskRating::Low,
            status: CustomerStatus::Active,
            updated_by: Uuid::nil(),
        }
    }
    
    pub fn full_name(mut self, full_name: &str) -> Self {
        self.full_name = full_name.to_string();
        self
    }
    
    pub fn identity(mut self, id_type: IdentityType, id_number: &str) -> Self {
        self.id_type = id_type;
        self.id_number = id_number.to_string();
        self
    }
    
    pub fn risk_rating(mut self, risk_rating: RiskRating) -> Self {
        self.risk_rating = risk_rating;
        self
    }
    
    pub fn status(mut self, status: CustomerStatus) -> Self {
        self.status = status;
        self
    }
    
    pub fn updated_by(mut self, updated_by: Uuid) -> Self {
        self.updated_by = updated_by;
        self
    }
    
    pub fn build(self) -> Result<Customer, &'static str> {
        let full_name_heap = HeaplessString::try_from(self.full_name.as_str()).map_err(|_| "Full name too long")?;
        let id_number_heap = HeaplessString::try_from(self.id_number.as_str()).map_err(|_| "ID number too long")?;
        // No need for HeaplessString conversion for UUID
        
        let now = chrono::Utc::now();
        
        Ok(Customer {
            customer_id: self.customer_id,
            customer_type: self.customer_type,
            full_name: full_name_heap,
            id_type: self.id_type,
            id_number: id_number_heap,
            risk_rating: self.risk_rating,
            status: self.status,
            created_at: now,
            last_updated_at: now,
            updated_by: self.updated_by,
        })
    }
}

impl Customer {
    /// Validate customer data with heapless string length checks
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Validate full_name (should not be empty, max 255 chars)
        if self.full_name.is_empty() {
            errors.push("Full name cannot be empty".to_string());
        }
        
        // Validate id_number (should not be empty, max 50 chars)
        if self.id_number.is_empty() {
            errors.push("ID number cannot be empty".to_string());
        }
        
        // Validate updated_by (should not be nil UUID)
        if self.updated_by.is_nil() {
            errors.push("Updated by cannot be nil UUID".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    /// Create a new customer builder
    pub fn builder(customer_id: Uuid, customer_type: CustomerType) -> CustomerBuilder {
        CustomerBuilder::new(customer_id, customer_type)
    }
    
    /// Helper method to create customer with string validation (kept for backward compatibility)
    /// 
    /// # Deprecated
    /// Use `Customer::builder()` instead for better ergonomics and fewer arguments.
    #[deprecated(since = "0.1.0", note = "Use Customer::builder() for better API design")]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        customer_id: Uuid,
        customer_type: CustomerType,
        full_name: &str,
        id_type: IdentityType,
        id_number: &str,
        risk_rating: RiskRating,
        status: CustomerStatus,
        updated_by: Uuid,
    ) -> Result<Self, &'static str> {
        CustomerBuilder::new(customer_id, customer_type)
            .full_name(full_name)
            .identity(id_type, id_number)
            .risk_rating(risk_rating)
            .status(status)
            .updated_by(updated_by)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_heapless_string_memory_efficiency() {
        // Compare memory sizes between String and HeaplessString
        let string_name = String::from("John Smith");
        let heapless_name: HeaplessString<255> = HeaplessString::try_from("John Smith").unwrap();
        
        println!("String name size: {} bytes", mem::size_of_val(&string_name));
        println!("HeaplessString name size: {} bytes", mem::size_of_val(&heapless_name));
        
        // HeaplessString should be fixed size (255 + small overhead for length/alignment)
        // Allow for small alignment overhead (typically 8 bytes or less)
        let heapless_size = mem::size_of_val(&heapless_name);
        assert!(heapless_size >= 256 && heapless_size <= 264, "HeaplessString size {} should be between 256-264 bytes", heapless_size);
        
        // String has heap allocation overhead
        assert!(mem::size_of_val(&string_name) < mem::size_of_val(&heapless_name));
        
        // But HeaplessString provides predictable memory usage
        let another_heapless: HeaplessString<255> = HeaplessString::try_from("A").unwrap();
        assert_eq!(mem::size_of_val(&heapless_name), mem::size_of_val(&another_heapless));
    }
    
    #[test]
    fn test_customer_validation() {
        #[allow(deprecated)]
        let customer = Customer::new(
            uuid::Uuid::new_v4(),
            CustomerType::Individual,
            "John Smith", 
            IdentityType::NationalId,
            "12345",
            RiskRating::Low,
            CustomerStatus::Active,
            Uuid::new_v4()
        ).unwrap();
        
        // Validation should pass for valid customer
        assert!(customer.validate().is_ok());
        
        // Test that helper methods work
        assert_eq!(customer.full_name.as_str(), "John Smith");
        assert_eq!(customer.id_number.as_str(), "12345");
        assert!(!customer.updated_by.is_nil());
    }

    #[test]
    fn test_customer_builder_pattern() {
        // Test the new builder pattern
        let customer = Customer::builder(Uuid::new_v4(), CustomerType::Corporate)
            .full_name("ACME Corporation Ltd")
            .identity(IdentityType::CompanyRegistration, "REG987654321")
            .risk_rating(RiskRating::Medium)
            .status(CustomerStatus::Active)
            .updated_by(Uuid::new_v4())
            .build()
            .unwrap();

        assert_eq!(customer.customer_type, CustomerType::Corporate);
        assert_eq!(customer.full_name.as_str(), "ACME Corporation Ltd");
        assert_eq!(customer.id_type, IdentityType::CompanyRegistration);
        assert_eq!(customer.id_number.as_str(), "REG987654321");
        assert_eq!(customer.risk_rating, RiskRating::Medium);
        assert_eq!(customer.status, CustomerStatus::Active);
        assert!(!customer.updated_by.is_nil());

        // Should pass validation
        assert!(customer.validate().is_ok());
    }

    #[test]
    fn test_customer_builder_validation() {
        // Test that builder properly validates string lengths
        let customer_id = Uuid::new_v4();
        let long_name = "a".repeat(300); // Exceeds 255 character limit
        
        let result = Customer::builder(customer_id, CustomerType::Individual)
            .full_name(&long_name)
            .identity(IdentityType::NationalId, "ID123")
            .updated_by(Uuid::new_v4())
            .build();
            
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Full name too long");
    }
    
    #[test]
    fn test_heapless_string_overflow_protection() {
        // Test that overly long strings are rejected
        let long_name = "a".repeat(300); // Exceeds 255 char limit
        
        #[allow(deprecated)]
        let result = Customer::new(
            uuid::Uuid::new_v4(),
            CustomerType::Individual,
            &long_name,
            IdentityType::NationalId,
            "12345",
            RiskRating::Low,
            CustomerStatus::Active,
            Uuid::new_v4()
        );
        
        // Should fail due to name being too long
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Full name too long");
    }
}