use chrono::{DateTime, NaiveDate, Utc};
use heapless::{String as HeaplessString, Vec as HeaplessVec};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Uuid,
    pub product_code: HeaplessString<12>,
    pub account_type: AccountType,
    pub account_status: AccountStatus,
    pub signing_condition: SigningCondition,
    pub currency: HeaplessString<3>,
    pub open_date: NaiveDate,
    pub domicile_branch_id: Uuid,
    
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
    /// References to DisbursementInstructions.disbursement_id (max 10 stages)
    pub disbursement_instructions: HeaplessVec<Uuid, 10>,
    
    // Enhanced audit trail
    /// References Person.person_id
    pub status_changed_by: Option<Uuid>,
    /// References ReasonAndPurpose.id for status change
    pub status_change_reason_id: Option<Uuid>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub updated_by: Uuid,
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
    pub disbursement_id: Uuid,
    /// References the account holding the loan (source of funds)
    pub source_account_id: Uuid,
    pub method: DisbursementMethod,
    pub target_account: Option<Uuid>,
    /// References AgencyBranch.branch_id for cash pickup
    pub cash_pickup_branch_id: Option<Uuid>,
    /// References Person.person_id for authorized recipient
    pub authorized_recipient: Option<Uuid>,
    
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
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
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
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: HoldType,
    /// References ReasonAndPurpose.id - required field
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by: Uuid,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    /// References Person.person_id
    pub released_by: Option<Uuid>,
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>, // External reference for judicial holds, etc.
    pub automatic_release: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoldStatus {
    Active,
    Released,
    Expired,
    Cancelled,
    PartiallyReleased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoldPriority {
    /// Must be honored first (judicial, regulatory)
    Critical,
    /// Standard business hold
    High,
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
    pub released_by: Uuid,
    pub override_authorization: bool,
}

/// Batch hold processing for expired holds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExpiryJob {
    pub job_id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: u32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Vec<HeaplessString<100>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChangeRecord {
    pub change_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<AccountStatus>,
    pub new_status: AccountStatus,
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_context: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub changed_by: Uuid,
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
    pub placed_by: Uuid,
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
    
    /// Add a disbursement instruction to the account (max 10)
    pub fn add_disbursement_instruction(&mut self, instruction_id: Uuid) -> Result<(), &'static str> {
        self.disbursement_instructions
            .push(instruction_id)
            .map_err(|_| "Maximum 10 disbursement instructions allowed")?;
        Ok(())
    }
    
    /// Remove a disbursement instruction by ID
    pub fn remove_disbursement_instruction(&mut self, instruction_id: Uuid) -> bool {
        if let Some(pos) = self.disbursement_instructions.iter().position(|&id| id == instruction_id) {
            self.disbursement_instructions.swap_remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Check if account has disbursement instructions
    pub fn has_disbursement_instructions(&self) -> bool {
        !self.disbursement_instructions.is_empty()
    }
    
    /// Get count of disbursement instructions
    pub fn disbursement_instruction_count(&self) -> usize {
        self.disbursement_instructions.len()
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
            account_id: uuid::Uuid::new_v4(),
            product_code: heapless_product,
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_branch_id: uuid::Uuid::new_v4(),
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
            disbursement_instructions: HeaplessVec::new(),
            status_changed_by: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by: Uuid::new_v4(), // References Person.person_id
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
    fn test_disbursement_instructions_management() {
        let mut account = Account {
            account_id: uuid::Uuid::new_v4(),
            product_code: HeaplessString::try_from("LNST0001").unwrap(),
            account_type: AccountType::Loan,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: HeaplessString::try_from("USD").unwrap(),
            open_date: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            domicile_branch_id: uuid::Uuid::new_v4(),
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
            disbursement_instructions: HeaplessVec::new(),
            status_changed_by: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by: Uuid::new_v4(),
        };
        
        // Test initial state
        assert!(!account.has_disbursement_instructions());
        assert_eq!(account.disbursement_instruction_count(), 0);
        
        // Test adding disbursement instructions (construction loan stages)
        let stage1_id = Uuid::new_v4();
        let stage2_id = Uuid::new_v4();
        let stage3_id = Uuid::new_v4();
        
        assert!(account.add_disbursement_instruction(stage1_id).is_ok());
        assert!(account.add_disbursement_instruction(stage2_id).is_ok());
        assert!(account.add_disbursement_instruction(stage3_id).is_ok());
        
        assert!(account.has_disbursement_instructions());
        assert_eq!(account.disbursement_instruction_count(), 3);
        
        // Test removing disbursement instruction
        assert!(account.remove_disbursement_instruction(stage2_id));
        assert_eq!(account.disbursement_instruction_count(), 2);
        assert!(!account.remove_disbursement_instruction(stage2_id)); // Should return false for non-existent
        
        // Test maximum limit (10 stages)
        for _ in 0..8 {
            let stage_id = Uuid::new_v4();
            assert!(account.add_disbursement_instruction(stage_id).is_ok());
        }
        assert_eq!(account.disbursement_instruction_count(), 10);
        
        // Test exceeding maximum limit
        let overflow_id = Uuid::new_v4();
        assert!(account.add_disbursement_instruction(overflow_id).is_err());
        assert_eq!(account.disbursement_instruction_count(), 10); // Should remain 10
    }
}

// Account Relations Structs and Enums (moved from account_relations.rs)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOwnership {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: OwnershipType,
    pub ownership_percentage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRelationship {
    pub relationship_id: Uuid,
    pub account_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: EntityType,
    pub relationship_type: RelationshipType,
    pub status: RelationshipStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountMandate {
    pub mandate_id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    pub permission_type: PermissionType,
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    pub status: MandateStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UltimateBeneficiary {
    pub ubo_link_id: Uuid,
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