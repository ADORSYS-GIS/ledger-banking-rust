use chrono::{DateTime, NaiveDate, Utc};
use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub product_code: HeaplessString<12>,
    pub account_type: AccountType,
    pub account_status: AccountStatus,
    pub signing_condition: SigningCondition,
    pub currency: HeaplessString<3>,
    pub open_date: NaiveDate,
    pub domicile_agency_branch_id: Uuid,
    
    // Balance fields
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub accrued_interest: Decimal,
    pub overdraft_limit: Option<Decimal>,
    
    // Loan-specific fields (nullable for non-loan accounts)
    pub original_principal: Option<Decimal>,
    pub outstanding_principal: Option<Decimal>,
    pub loan_interest_rate: Option<Decimal>,
    pub loan_term_months: Option<i32>,
    pub disbursement_date: Option<NaiveDate>,
    pub maturity_date: Option<NaiveDate>,
    pub installment_amount: Option<Decimal>,
    pub next_due_date: Option<NaiveDate>,
    pub penalty_rate: Option<Decimal>,
    pub collateral_id: Option<Uuid>,
    /// References ReasonAndPurpose.id for loan purpose
    pub loan_purpose_id: Option<Uuid>,

    // Account lifecycle management (from enhancements)
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>,
    pub dormancy_threshold_days: Option<i32>,
    pub reactivation_required: bool,
    /// References ReasonAndPurpose.id for pending closure
    pub pending_closure_reason_id: Option<Uuid>,
    /// References the last DisbursementInstructions.disbursement_id
    pub last_disbursement_instruction_id: Option<Uuid>,
    
    // Enhanced audit trail
    /// References Person.person_id
    pub status_changed_by_person_id: Option<Uuid>,
    /// References ReasonAndPurpose.id for status change
    pub status_change_reason_id: Option<Uuid>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType { 
    Savings, 
    Current, 
    Loan 
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AccountStatus { 
    PendingApproval,
    Active, 
    Dormant,
    Frozen,
    PendingClosure,
    Closed,
    PendingReactivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SigningCondition { 
    None,
    AnyOwner,
    AllOwners
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisbursementInstructions {
    pub id: Uuid,
    /// References the account holding the loan (source of funds)
    pub source_account_id: Uuid,
    pub method: DisbursementMethod,
    pub target_account_id: Option<Uuid>,
    /// References AgencyBranch.branch_id for cash pickup
    pub cash_pickup_agency_branch_id: Option<Uuid>,
    /// References Person.person_id for authorized recipient
    pub authorized_recipient_person_id: Option<Uuid>,
    
    // Disbursement tracking and staging
    pub disbursement_amount: Option<Decimal>,
    pub disbursement_date: Option<NaiveDate>,
    pub stage_number: Option<i32>,
    pub stage_description: Option<HeaplessString<200>>,
    pub status: DisbursementStatus,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by_person_id: Uuid,
    /// References Person.person_id
    pub updated_by_person_id: Uuid,
}

impl Default for DisbursementInstructions {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            source_account_id: Uuid::nil(),
            method: DisbursementMethod::Transfer,
            target_account_id: None,
            cash_pickup_agency_branch_id: None,
            authorized_recipient_person_id: None,
            disbursement_amount: None,
            disbursement_date: None,
            stage_number: None,
            stage_description: None,
            status: DisbursementStatus::Pending,
            created_at: now,
            last_updated_at: now,
            created_by_person_id: Uuid::nil(),
            updated_by_person_id: Uuid::nil(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisbursementMethod {
    Transfer,
    CashWithdrawal,
    Check,
    HoldFunds,
    OverdraftFacility,
    StagedRelease,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DisbursementStatus {
    Pending,
    Approved,
    Executed,
    Cancelled,
    Failed,
    PartiallyExecuted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHold {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: HoldType,
    /// References ReasonAndPurpose.id - required field
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by_person_id: Uuid,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    /// References Person.person_id
    pub released_by_person_id: Option<Uuid>,
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>, // External reference for judicial holds, etc.
    pub automatic_release: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HoldType {
    /// Funds pending clearance
    UnclearedFunds,
    /// Court-ordered judicial lien
    JudicialLien,
    /// Loan collateral pledge
    LoanPledge,
    /// Regulatory compliance hold
    ComplianceHold,
    /// Administrative hold by bank staff
    AdministrativeHold,
    /// Fraud investigation hold
    FraudHold,
    /// Pending transaction authorization
    PendingAuthorization,
    /// Overdraft protection reserve
    OverdraftReserve,
    /// Card authorization hold
    CardAuthorization,
    /// Other types
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HoldStatus {
    Active,
    Released,
    Expired,
    Cancelled,
    PartiallyReleased,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HoldPriority {
    /// Must be honored first (judicial, regulatory)
    Critical,
    /// Standard business hold
    High,
    /// Standard priority hold
    Standard,
    /// Lower priority administrative hold
    Medium,
    /// Lowest priority, can be overridden
    Low,
}

/// Real-time balance calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCalculation {
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub total_holds: Decimal,
    pub active_hold_count: u32,
    pub calculation_timestamp: DateTime<Utc>,
    pub hold_breakdown: Vec<HoldSummary>,
}

/// Summary of hold amounts by type for balance calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldSummary {
    pub hold_type: HoldType,
    pub total_amount: Decimal,
    pub hold_count: u32,
    pub priority: HoldPriority,
}

/// Hold release request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldReleaseRequest {
    pub hold_id: Uuid,
    pub release_amount: Option<Decimal>, // For partial releases
    /// References ReasonAndPurpose.id for release
    pub release_reason_id: Uuid,
    /// Additional context for release
    pub release_additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub released_by_person_id: Uuid,
    pub override_authorization: bool,
}

/// Batch hold processing for expired holds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExpiryJob {
    pub id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: u32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Vec<HeaplessString<100>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChangeRecord {
    pub id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<AccountStatus>,
    pub new_status: AccountStatus,
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_context: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub changed_by_person_id: Uuid,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
}

/// Request parameters for placing a hold on an account
#[derive(Debug, Clone)]
pub struct PlaceHoldRequest {
    pub account_id: Uuid,
    pub hold_type: HoldType,
    pub amount: Decimal,
    /// References ReasonAndPurpose.id - required field
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by_person_id: Uuid,
    pub expires_at: Option<DateTime<Utc>>,
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>,
}



impl Account {
    /// Set product code from string with validation
    pub fn set_product_code(&mut self, product_code: &str) -> Result<(), &'static str> {
        self.product_code = HeaplessString::try_from(product_code).map_err(|_| "Product code too long")?;
        Ok(())
    }
    
    /// Set the last disbursement instruction for the account
    pub fn set_last_disbursement_instruction(&mut self, instruction_id: Option<Uuid>) {
        self.last_disbursement_instruction_id = instruction_id;
    }
    
    /// Check if account has a disbursement instruction
    pub fn has_disbursement_instruction(&self) -> bool {
        self.last_disbursement_instruction_id.is_some()
    }
    
    /// Get the last disbursement instruction ID
    pub fn get_last_disbursement_instruction(&self) -> Option<Uuid> {
        self.last_disbursement_instruction_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_code_heapless_efficiency() {
        use std::mem;
        
        // Compare memory sizes between HeaplessString and String
        let string_product = String::from("SAVP0001");
        let heapless_product: HeaplessString<12> = HeaplessString::try_from("SAVP0001").unwrap();
        
        println!("String product code size: {} bytes", mem::size_of_val(&string_product));
        println!("HeaplessString product code size: {} bytes", mem::size_of_val(&heapless_product));
        
        // HeaplessString may be similar size to String for longer strings but provides stack allocation
        // The benefit is predictable memory layout and no heap allocation
        assert_eq!(mem::size_of_val(&heapless_product), 24); // HeaplessString<12> with capacity info
        
        // Test conversion functions
        let mut account = Account {
            id: uuid::Uuid::new_v4(),
            product_code: heapless_product,
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_agency_branch_id: uuid::Uuid::new_v4(),
            current_balance: rust_decimal::Decimal::new(10000, 2),
            available_balance: rust_decimal::Decimal::new(10000, 2),
            accrued_interest: rust_decimal::Decimal::ZERO,
            overdraft_limit: None,
            original_principal: None,
            outstanding_principal: None,
            loan_interest_rate: None,
            loan_term_months: None,
            disbursement_date: None,
            maturity_date: None,
            installment_amount: None,
            next_due_date: None,
            penalty_rate: None,
            collateral_id: None,
            loan_purpose_id: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // References Person.person_id
        };
        
        // Test string access
        assert_eq!(account.product_code.as_str(), "SAVP0001");
        
        // Test setting from string
        account.set_product_code("LNST0123").unwrap();
        assert_eq!(account.product_code.as_str(), "LNST0123");
    }

    #[test]
    fn test_currency_memory_efficiency() {
        use std::mem;
        
        // Compare memory sizes - our optimization goal
        let string_currency = String::from("USD");
        let heapless_currency: HeaplessString<3> = HeaplessString::try_from("USD").unwrap();
        
        println!("String currency size: {} bytes", mem::size_of_val(&string_currency));
        println!("HeaplessString currency size: {} bytes", mem::size_of_val(&heapless_currency));
        
        // HeaplessString should be smaller than String for very short strings
        assert!(mem::size_of_val(&heapless_currency) < mem::size_of_val(&string_currency));
        // HeaplessString<3> is allocated with capacity + length info 
        assert_eq!(mem::size_of_val(&heapless_currency), 16);
    }

    #[test]
    fn test_currency_validation_and_conversion() {
        // Test that our currency HeaplessStrings work as expected
        let original_currency: HeaplessString<3> = HeaplessString::try_from("EUR").unwrap();
        
        // Test currency validation
        let currency_str = original_currency.as_str();
        assert_eq!(currency_str.len(), 3);
        assert_eq!(currency_str, "EUR");
        assert!(currency_str.chars().all(|c| c.is_ascii_uppercase()));
        
        // Test various currency codes
        let currencies = ["USD", "GBP", "JPY", "CAD", "XAF"];
        for &currency in currencies.iter() {
            let heapless_currency: HeaplessString<3> = HeaplessString::try_from(currency).unwrap();
            let currency_str = heapless_currency.as_str();
            assert_eq!(currency_str.len(), 3);
            assert!(currency_str.chars().all(|c| c.is_ascii_alphabetic()));
        }
    }

    #[test]
    fn test_enum_memory_efficiency() {
        use std::mem;
        use super::AccountStatus;
        
        // Compare memory sizes - enum optimization goal
        let string_status = String::from("Active");
        let enum_status = AccountStatus::Active;
        
        println!("String status size: {} bytes", mem::size_of_val(&string_status));
        println!("Enum status size: {} bytes", mem::size_of_val(&enum_status));
        
        // Enum should be much smaller than String
        assert!(mem::size_of_val(&enum_status) < mem::size_of_val(&string_status));
        assert!(mem::size_of_val(&enum_status) <= 8); // Typically 1-8 bytes for enums
    }

    #[test]
    fn test_disbursement_instruction_management() {
        let mut account = Account {
            id: uuid::Uuid::new_v4(),
            product_code: HeaplessString::try_from("LNST0001").unwrap(),
            account_type: AccountType::Loan,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_agency_branch_id: uuid::Uuid::new_v4(),
            current_balance: rust_decimal::Decimal::ZERO,
            available_balance: rust_decimal::Decimal::ZERO,
            accrued_interest: rust_decimal::Decimal::ZERO,
            overdraft_limit: None,
            original_principal: Some(rust_decimal::Decimal::new(50000000, 2)), // $500,000
            outstanding_principal: Some(rust_decimal::Decimal::new(50000000, 2)),
            loan_interest_rate: Some(rust_decimal::Decimal::new(750, 2)), // 7.5%
            loan_term_months: Some(24),
            disbursement_date: None,
            maturity_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
            installment_amount: Some(rust_decimal::Decimal::new(2291667, 2)),
            next_due_date: Some(chrono::NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()),
            penalty_rate: Some(rust_decimal::Decimal::new(200, 2)),
            collateral_id: Some(Uuid::new_v4()),
            loan_purpose_id: Some(Uuid::new_v4()),
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(),
        };
        
        // Test initial state
        assert!(!account.has_disbursement_instruction());
        assert_eq!(account.get_last_disbursement_instruction(), None);
        
        // Test setting disbursement instruction
        let instruction_id = Uuid::new_v4();
        account.set_last_disbursement_instruction(Some(instruction_id));
        
        assert!(account.has_disbursement_instruction());
        assert_eq!(account.get_last_disbursement_instruction(), Some(instruction_id));
        
        // Test updating to a new instruction (replaces the previous one)
        let new_instruction_id = Uuid::new_v4();
        account.set_last_disbursement_instruction(Some(new_instruction_id));
        
        assert!(account.has_disbursement_instruction());
        assert_eq!(account.get_last_disbursement_instruction(), Some(new_instruction_id));
        
        // Test clearing the instruction
        account.set_last_disbursement_instruction(None);
        
        assert!(!account.has_disbursement_instruction());
        assert_eq!(account.get_last_disbursement_instruction(), None);
    }
}

// Account Relations Structs and Enums (moved from account_relations.rs)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOwnership {
    pub id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: OwnershipType,
    pub ownership_percentage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRelationship {
    pub id: Uuid,
    pub account_id: Uuid,
    pub person_id: Uuid,
    pub entity_type: EntityType,
    pub relationship_type: RelationshipType,
    pub status: RelationshipStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountMandate {
    pub id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    pub permission_type: PermissionType,
    pub transaction_limit: Option<Decimal>,
    pub approver01_person_id: Option<Uuid>,
    pub approver02_person_id: Option<Uuid>,
    pub approver03_person_id: Option<Uuid>,
    pub approver04_person_id: Option<Uuid>,
    pub approver05_person_id: Option<Uuid>,
    pub approver06_person_id: Option<Uuid>,
    pub approver07_person_id: Option<Uuid>,
    pub required_signers_count: u8,
    pub conditional_mandate_id: Option<Uuid>,
    pub status: MandateStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UltimateBeneficiary {
    pub id: Uuid,
    pub corporate_customer_id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    pub control_type: ControlType,
    pub description: Option<HeaplessString<200>>,
    pub status: UboStatus,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OwnershipType {
    Single,
    Joint,
    Corporate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Branch,
    Agent,
    RiskManager,
    ComplianceOfficer,
    CustomerService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    PrimaryHandler,
    BackupHandler,
    RiskOversight,
    ComplianceOversight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionType {
    ViewOnly,
    LimitedWithdrawal,
    JointApproval,
    FullAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MandateStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType { 
    DirectOwnership, 
    IndirectOwnership, 
    SignificantInfluence, 
    SeniorManagement 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus { 
    Pending, 
    Verified, 
    Rejected, 
    RequiresUpdate 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UboStatus {
    Active,
    Inactive,
    UnderReview,
}

// Display implementations for database compatibility
impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountType::Savings => write!(f, "Savings"),
            AccountType::Current => write!(f, "Current"),
            AccountType::Loan => write!(f, "Loan"),
        }
    }
}

impl std::fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountStatus::PendingApproval => write!(f, "PendingApproval"),
            AccountStatus::Active => write!(f, "Active"),
            AccountStatus::Dormant => write!(f, "Dormant"),
            AccountStatus::Frozen => write!(f, "Frozen"),
            AccountStatus::PendingClosure => write!(f, "PendingClosure"),
            AccountStatus::Closed => write!(f, "Closed"),
            AccountStatus::PendingReactivation => write!(f, "PendingReactivation"),
        }
    }
}

impl std::fmt::Display for SigningCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigningCondition::None => write!(f, "None"),
            SigningCondition::AnyOwner => write!(f, "AnyOwner"),
            SigningCondition::AllOwners => write!(f, "AllOwners"),
        }
    }
}

impl std::fmt::Display for DisbursementMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisbursementMethod::Transfer => write!(f, "Transfer"),
            DisbursementMethod::CashWithdrawal => write!(f, "CashWithdrawal"),
            DisbursementMethod::Check => write!(f, "Check"),
            DisbursementMethod::HoldFunds => write!(f, "HoldFunds"),
            DisbursementMethod::OverdraftFacility => write!(f, "OverdraftFacility"),
            DisbursementMethod::StagedRelease => write!(f, "StagedRelease"),
        }
    }
}

impl std::fmt::Display for HoldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldType::UnclearedFunds => write!(f, "UnclearedFunds"),
            HoldType::JudicialLien => write!(f, "JudicialLien"),
            HoldType::LoanPledge => write!(f, "LoanPledge"),
            HoldType::ComplianceHold => write!(f, "ComplianceHold"),
            HoldType::AdministrativeHold => write!(f, "AdministrativeHold"),
            HoldType::FraudHold => write!(f, "FraudHold"),
            HoldType::PendingAuthorization => write!(f, "PendingAuthorization"),
            HoldType::OverdraftReserve => write!(f, "OverdraftReserve"),
            HoldType::CardAuthorization => write!(f, "CardAuthorization"),
            HoldType::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for HoldStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldStatus::Active => write!(f, "Active"),
            HoldStatus::Released => write!(f, "Released"),
            HoldStatus::Expired => write!(f, "Expired"),
            HoldStatus::Cancelled => write!(f, "Cancelled"),
            HoldStatus::PartiallyReleased => write!(f, "PartiallyReleased"),
        }
    }
}

impl std::fmt::Display for HoldPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HoldPriority::Critical => write!(f, "Critical"),
            HoldPriority::High => write!(f, "High"),
            HoldPriority::Standard => write!(f, "Standard"),
            HoldPriority::Medium => write!(f, "Medium"),
            HoldPriority::Low => write!(f, "Low"),
        }
    }
}

impl std::fmt::Display for OwnershipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnershipType::Single => write!(f, "Single"),
            OwnershipType::Joint => write!(f, "Joint"),
            OwnershipType::Corporate => write!(f, "Corporate"),
        }
    }
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityType::Branch => write!(f, "Branch"),
            EntityType::Agent => write!(f, "Agent"),
            EntityType::RiskManager => write!(f, "RiskManager"),
            EntityType::ComplianceOfficer => write!(f, "ComplianceOfficer"),
            EntityType::CustomerService => write!(f, "CustomerService"),
        }
    }
}

impl std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipType::PrimaryHandler => write!(f, "PrimaryHandler"),
            RelationshipType::BackupHandler => write!(f, "BackupHandler"),
            RelationshipType::RiskOversight => write!(f, "RiskOversight"),
            RelationshipType::ComplianceOversight => write!(f, "ComplianceOversight"),
        }
    }
}

impl std::fmt::Display for RelationshipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelationshipStatus::Active => write!(f, "Active"),
            RelationshipStatus::Inactive => write!(f, "Inactive"),
            RelationshipStatus::Suspended => write!(f, "Suspended"),
        }
    }
}

impl std::fmt::Display for PermissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionType::ViewOnly => write!(f, "ViewOnly"),
            PermissionType::LimitedWithdrawal => write!(f, "LimitedWithdrawal"),
            PermissionType::JointApproval => write!(f, "JointApproval"),
            PermissionType::FullAccess => write!(f, "FullAccess"),
        }
    }
}

impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MandateStatus::Active => write!(f, "Active"),
            MandateStatus::Suspended => write!(f, "Suspended"),
            MandateStatus::Revoked => write!(f, "Revoked"),
            MandateStatus::Expired => write!(f, "Expired"),
        }
    }
}