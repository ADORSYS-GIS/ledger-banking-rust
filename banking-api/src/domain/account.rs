use chrono::{DateTime, NaiveDate, Utc};
use heapless::{String as HeaplessString};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub product_id: Uuid,
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
    pub gl_code_suffix: Option<HeaplessString<10>>,
 
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
    
    // Direct reference fields for most significant related entities
    /// References AccountHold.id - most significant hold on this account
    pub most_significant_account_hold_id: Option<Uuid>,
    /// References AccountOwnership.id
    pub account_ownership_id: Option<Uuid>,
    /// References AccountRelationship.id for access permissions
    pub access01_account_relationship_id: Option<Uuid>,
    pub access02_account_relationship_id: Option<Uuid>,
    pub access03_account_relationship_id: Option<Uuid>,
    pub access04_account_relationship_id: Option<Uuid>,
    pub access05_account_relationship_id: Option<Uuid>,
    pub access06_account_relationship_id: Option<Uuid>,
    pub access07_account_relationship_id: Option<Uuid>,
    /// References AccountMandate.id for access permissions
    pub access11_account_mandate_id: Option<Uuid>,
    pub access12_account_mandate_id: Option<Uuid>,
    pub access13_account_mandate_id: Option<Uuid>,
    pub access14_account_mandate_id: Option<Uuid>,
    pub access15_account_mandate_id: Option<Uuid>,
    pub access16_account_mandate_id: Option<Uuid>,
    pub access17_account_mandate_id: Option<Uuid>,
    /// References UltimateBeneficiary.id for beneficial interest
    pub interest01_ultimate_beneficiary_id: Option<Uuid>,
    pub interest02_ultimate_beneficiary_id: Option<Uuid>,
    pub interest03_ultimate_beneficiary_id: Option<Uuid>,
    pub interest04_ultimate_beneficiary_id: Option<Uuid>,
    pub interest05_ultimate_beneficiary_id: Option<Uuid>,
    pub interest06_ultimate_beneficiary_id: Option<Uuid>,
    pub interest07_ultimate_beneficiary_id: Option<Uuid>,
    
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

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountStatusChangeRecord {
    pub id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<AccountStatus>,
    pub new_status: AccountStatus,
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context for status change
    pub additional_context: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub changed_by_person_id: Uuid,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
    pub created_at: DateTime<Utc>,
}

impl Default for AccountStatusChangeRecord {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            account_id: Uuid::nil(),
            old_status: None,
            new_status: AccountStatus::PendingApproval,
            reason_id: Uuid::nil(),
            additional_context: None,
            changed_by_person_id: Uuid::nil(),
            changed_at: now,
            system_triggered: false,
            created_at: now,
        }
    }
}

impl Account {
    /// Set product id
    pub fn set_product_id(&mut self, product_id: Uuid) {
        self.product_id = product_id;
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
    fn test_product_id_efficiency() {
        use std::mem;
        
        // Compare memory sizes between Uuid and String
        let string_product = String::from("SAVP0001");
        let product_id = Uuid::new_v4();
        
        println!("String product code size: {} bytes", mem::size_of_val(&string_product));
        println!("Uuid product id size: {} bytes", mem::size_of_val(&product_id));
        
        // Uuid should be smaller than a typical string representation on the heap
        assert!(mem::size_of_val(&product_id) < mem::size_of_val(&string_product));
        assert_eq!(mem::size_of_val(&product_id), 16); // Uuid is 16 bytes
        
        // Test account creation with product_id
        let _account = Account {
            id: uuid::Uuid::new_v4(),
            product_id,
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
            gl_code_suffix: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            most_significant_account_hold_id: None,
            account_ownership_id: None,
            access01_account_relationship_id: None,
            access02_account_relationship_id: None,
            access03_account_relationship_id: None,
            access04_account_relationship_id: None,
            access05_account_relationship_id: None,
            access06_account_relationship_id: None,
            access07_account_relationship_id: None,
            access11_account_mandate_id: None,
            access12_account_mandate_id: None,
            access13_account_mandate_id: None,
            access14_account_mandate_id: None,
            access15_account_mandate_id: None,
            access16_account_mandate_id: None,
            access17_account_mandate_id: None,
            interest01_ultimate_beneficiary_id: None,
            interest02_ultimate_beneficiary_id: None,
            interest03_ultimate_beneficiary_id: None,
            interest04_ultimate_beneficiary_id: None,
            interest05_ultimate_beneficiary_id: None,
            interest06_ultimate_beneficiary_id: None,
            interest07_ultimate_beneficiary_id: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by_person_id: Uuid::new_v4(), // References Person.person_id
        };
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
            product_id: Uuid::new_v4(),
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
            gl_code_suffix: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason_id: None,
            last_disbursement_instruction_id: None,
            status_changed_by_person_id: None,
            status_change_reason_id: None,
            status_change_timestamp: None,
            most_significant_account_hold_id: None,
            account_ownership_id: None,
            access01_account_relationship_id: None,
            access02_account_relationship_id: None,
            access03_account_relationship_id: None,
            access04_account_relationship_id: None,
            access05_account_relationship_id: None,
            access06_account_relationship_id: None,
            access07_account_relationship_id: None,
            access11_account_mandate_id: None,
            access12_account_mandate_id: None,
            access13_account_mandate_id: None,
            access14_account_mandate_id: None,
            access15_account_mandate_id: None,
            access16_account_mandate_id: None,
            access17_account_mandate_id: None,
            interest01_ultimate_beneficiary_id: None,
            interest02_ultimate_beneficiary_id: None,
            interest03_ultimate_beneficiary_id: None,
            interest04_ultimate_beneficiary_id: None,
            interest05_ultimate_beneficiary_id: None,
            interest06_ultimate_beneficiary_id: None,
            interest07_ultimate_beneficiary_id: None,
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
    Accountant
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

impl std::fmt::Display for DisbursementStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisbursementStatus::Pending => write!(f, "Pending"),
            DisbursementStatus::Approved => write!(f, "Approved"),
            DisbursementStatus::Executed => write!(f, "Executed"),
            DisbursementStatus::Cancelled => write!(f, "Cancelled"),
            DisbursementStatus::Failed => write!(f, "Failed"),
            DisbursementStatus::PartiallyExecuted => write!(f, "PartiallyExecuted"),
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
            RelationshipType::Accountant => write!(f, "Accountant"),
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

impl FromStr for AccountType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Savings" => Ok(AccountType::Savings),
            "Current" => Ok(AccountType::Current),
            "Loan" => Ok(AccountType::Loan),
            _ => Err(()),
        }
    }
}

impl FromStr for AccountStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PendingApproval" => Ok(AccountStatus::PendingApproval),
            "Active" => Ok(AccountStatus::Active),
            "Dormant" => Ok(AccountStatus::Dormant),
            "Frozen" => Ok(AccountStatus::Frozen),
            "PendingClosure" => Ok(AccountStatus::PendingClosure),
            "Closed" => Ok(AccountStatus::Closed),
            "PendingReactivation" => Ok(AccountStatus::PendingReactivation),
            _ => Err(()),
        }
    }
}

impl FromStr for SigningCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(SigningCondition::None),
            "AnyOwner" => Ok(SigningCondition::AnyOwner),
            "AllOwners" => Ok(SigningCondition::AllOwners),
            _ => Err(()),
        }
    }
}
impl FromStr for DisbursementMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Transfer" => Ok(DisbursementMethod::Transfer),
            "CashWithdrawal" => Ok(DisbursementMethod::CashWithdrawal),
            "Check" => Ok(DisbursementMethod::Check),
            "HoldFunds" => Ok(DisbursementMethod::HoldFunds),
            "OverdraftFacility" => Ok(DisbursementMethod::OverdraftFacility),
            "StagedRelease" => Ok(DisbursementMethod::StagedRelease),
            _ => Err(()),
        }
    }
}

impl FromStr for OwnershipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Single" => Ok(OwnershipType::Single),
            "Joint" => Ok(OwnershipType::Joint),
            "Corporate" => Ok(OwnershipType::Corporate),
            _ => Err(()),
        }
    }
}

impl FromStr for EntityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Branch" => Ok(EntityType::Branch),
            "Agent" => Ok(EntityType::Agent),
            "RiskManager" => Ok(EntityType::RiskManager),
            "ComplianceOfficer" => Ok(EntityType::ComplianceOfficer),
            "CustomerService" => Ok(EntityType::CustomerService),
            _ => Err(()),
        }
    }
}

impl FromStr for RelationshipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PrimaryHandler" => Ok(RelationshipType::PrimaryHandler),
            "BackupHandler" => Ok(RelationshipType::BackupHandler),
            "RiskOversight" => Ok(RelationshipType::RiskOversight),
            "ComplianceOversight" => Ok(RelationshipType::ComplianceOversight),
            "Accountant" => Ok(RelationshipType::Accountant),
            _ => Err(()),
        }
    }
}

impl FromStr for RelationshipStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(RelationshipStatus::Active),
            "Inactive" => Ok(RelationshipStatus::Inactive),
            "Suspended" => Ok(RelationshipStatus::Suspended),
            _ => Err(()),
        }
    }
}

impl FromStr for PermissionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ViewOnly" => Ok(PermissionType::ViewOnly),
            "LimitedWithdrawal" => Ok(PermissionType::LimitedWithdrawal),
            "JointApproval" => Ok(PermissionType::JointApproval),
            "FullAccess" => Ok(PermissionType::FullAccess),
            _ => Err(()),
        }
    }
}

impl FromStr for MandateStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(MandateStatus::Active),
            "Suspended" => Ok(MandateStatus::Suspended),
            "Revoked" => Ok(MandateStatus::Revoked),
            "Expired" => Ok(MandateStatus::Expired),
            _ => Err(()),
        }
    }
}