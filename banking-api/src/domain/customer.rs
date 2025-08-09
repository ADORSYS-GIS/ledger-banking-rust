use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: Uuid,
    pub customer_type: CustomerType,
    pub full_name: HeaplessString<100>,
    pub id_type: IdentityType,
    pub id_number: HeaplessString<50>,
    pub risk_rating: RiskRating,
    pub status: CustomerStatus,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub updated_by_person_id: Uuid,
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

// Display implementations for database compatibility
impl std::fmt::Display for CustomerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerType::Individual => write!(f, "Individual"),
            CustomerType::Corporate => write!(f, "Corporate"),
        }
    }
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

impl std::fmt::Display for RiskRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskRating::Low => write!(f, "Low"),
            RiskRating::Medium => write!(f, "Medium"),
            RiskRating::High => write!(f, "High"),
            RiskRating::Blacklisted => write!(f, "Blacklisted"),
        }
    }
}

impl std::fmt::Display for CustomerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerStatus::Active => write!(f, "Active"),
            CustomerStatus::PendingVerification => write!(f, "PendingVerification"),
            CustomerStatus::Deceased => write!(f, "Deceased"),
            CustomerStatus::Dissolved => write!(f, "Dissolved"),
            CustomerStatus::Blacklisted => write!(f, "Blacklisted"),
        }
    }
}

impl std::fmt::Display for KycStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KycStatus::NotStarted => write!(f, "NotStarted"),
            KycStatus::InProgress => write!(f, "InProgress"),
            KycStatus::Pending => write!(f, "Pending"),
            KycStatus::Complete => write!(f, "Complete"),
            KycStatus::Approved => write!(f, "Approved"),
            KycStatus::Rejected => write!(f, "Rejected"),
            KycStatus::RequiresUpdate => write!(f, "RequiresUpdate"),
            KycStatus::Failed => write!(f, "Failed"),
        }
    }
}

// FromStr implementations for database compatibility
impl std::str::FromStr for CustomerType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Individual" => Ok(CustomerType::Individual),
            "Corporate" => Ok(CustomerType::Corporate),
            _ => Err(format!("Invalid CustomerType: {s}")),
        }
    }
}

impl std::str::FromStr for IdentityType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NationalId" => Ok(IdentityType::NationalId),
            "Passport" => Ok(IdentityType::Passport),
            "CompanyRegistration" => Ok(IdentityType::CompanyRegistration),
            "PermanentResidentCard" => Ok(IdentityType::PermanentResidentCard),
            "AsylumCard" => Ok(IdentityType::AsylumCard),
            "TemporaryResidentPermit" => Ok(IdentityType::TemporaryResidentPermit),
            "Unknown" => Ok(IdentityType::Unknown),
            _ => Err(format!("Invalid IdentityType: {s}")),
        }
    }
}

impl std::str::FromStr for RiskRating {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(RiskRating::Low),
            "Medium" => Ok(RiskRating::Medium),
            "High" => Ok(RiskRating::High),
            "Blacklisted" => Ok(RiskRating::Blacklisted),
            _ => Err(format!("Invalid RiskRating: {s}")),
        }
    }
}

impl std::str::FromStr for CustomerStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(CustomerStatus::Active),
            "PendingVerification" => Ok(CustomerStatus::PendingVerification),
            "Deceased" => Ok(CustomerStatus::Deceased),
            "Dissolved" => Ok(CustomerStatus::Dissolved),
            "Blacklisted" => Ok(CustomerStatus::Blacklisted),
            _ => Err(format!("Invalid CustomerStatus: {s}")),
        }
    }
}

impl std::str::FromStr for KycStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NotStarted" => Ok(KycStatus::NotStarted),
            "InProgress" => Ok(KycStatus::InProgress),
            "Pending" => Ok(KycStatus::Pending),
            "Complete" => Ok(KycStatus::Complete),
            "Approved" => Ok(KycStatus::Approved),
            "Rejected" => Ok(KycStatus::Rejected),
            "RequiresUpdate" => Ok(KycStatus::RequiresUpdate),
            "Failed" => Ok(KycStatus::Failed),
            _ => Err(format!("Invalid KycStatus: {s}")),
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
    pub kyc_status: KycStatus,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub current_rating: RiskRating,
    pub last_assessment_date: DateTime<Utc>,
    pub flags_01: HeaplessString<200>,
    pub flags_02: HeaplessString<200>,
    pub flags_03: HeaplessString<200>,
    pub flags_04: HeaplessString<200>,
    pub flags_05: HeaplessString<200>,
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
    updated_by_person_id: Uuid,
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
            updated_by_person_id: Uuid::nil(),
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
    
    pub fn updated_by(mut self, updated_by_person_id: Uuid) -> Self {
        self.updated_by_person_id = updated_by_person_id;
        self
    }
    
    pub fn build(self) -> Result<Customer, &'static str> {
        let full_name_heap = HeaplessString::try_from(self.full_name.as_str()).map_err(|_| "Full name too long")?;
        let id_number_heap = HeaplessString::try_from(self.id_number.as_str()).map_err(|_| "ID number too long")?;
        // No need for HeaplessString conversion for UUID
        
        let now = chrono::Utc::now();
        
        Ok(Customer {
            id: self.customer_id,
            customer_type: self.customer_type,
            full_name: full_name_heap,
            id_type: self.id_type,
            id_number: id_number_heap,
            risk_rating: self.risk_rating,
            status: self.status,
            created_at: now,
            last_updated_at: now,
            updated_by_person_id: self.updated_by_person_id,
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
        
        // Validate updated_by_person_id (should not be nil UUID)
        if self.updated_by_person_id.is_nil() {
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
        updated_by_person_id: Uuid,
    ) -> Result<Self, &'static str> {
        CustomerBuilder::new(customer_id, customer_type)
            .full_name(full_name)
            .identity(id_type, id_number)
            .risk_rating(risk_rating)
            .status(status)
            .updated_by(updated_by_person_id)
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
        let heapless_name: HeaplessString<100> = HeaplessString::try_from("John Smith").unwrap();
        
        println!("String name size: {} bytes", mem::size_of_val(&string_name));
        println!("HeaplessString name size: {} bytes", mem::size_of_val(&heapless_name));
        
        // HeaplessString should be fixed size (100 + small overhead for length/alignment)
        // Allow for reasonable alignment overhead (typically 12 bytes or less)
        let heapless_size = mem::size_of_val(&heapless_name);
        assert!(heapless_size >= 101 && heapless_size <= 116, "HeaplessString size {} should be between 101-116 bytes", heapless_size);
        
        // String has heap allocation overhead
        assert!(mem::size_of_val(&string_name) < mem::size_of_val(&heapless_name));
        
        // But HeaplessString provides predictable memory usage
        let another_heapless: HeaplessString<100> = HeaplessString::try_from("A").unwrap();
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
        assert!(!customer.updated_by_person_id.is_nil());
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
        assert!(!customer.updated_by_person_id.is_nil());

        // Should pass validation
        assert!(customer.validate().is_ok());
    }

    #[test]
    fn test_customer_builder_validation() {
        // Test that builder properly validates string lengths
        let customer_id = Uuid::new_v4();
        let long_name = "a".repeat(150); // Exceeds 100 character limit
        
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
        let long_name = "a".repeat(150); // Exceeds 100 char limit
        
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