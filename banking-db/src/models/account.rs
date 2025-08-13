use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

/// Database model for Account table with enhanced fields from banking enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountModel {
    pub id: Uuid,
    pub product_id: Uuid,
    pub account_type: DbAccountType,
    pub account_status: DbAccountStatus,
    pub signing_condition: DbSigningCondition,
    pub currency: HeaplessString<3>,
    pub open_date: NaiveDate,
    pub domicile_agency_branch_id: Uuid,
    
    pub gl_code_suffix: Option<HeaplessString<10>>,
    
    // Balance fields
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub accrued_interest: Decimal,
    pub overdraft_limit: Option<Decimal>,
    
    // Loan-specific fields
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

/// Database model for Account Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountOwnershipModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: DbOwnershipType,
    pub ownership_percentage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Account Relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountRelationshipModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub person_id: Uuid,
    pub entity_type: DbEntityType,
    pub relationship_type: DbRelationshipType,
    pub status: DbRelationshipStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Database model for Account Mandates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountMandateModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    pub permission_type: DbPermissionType,
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
    pub status: DbMandateStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}


/// Database model for Account Status History (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountStatusChangeRecordModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<DbAccountStatus>,
    pub new_status: DbAccountStatus,
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

/// Database model for Final Settlements (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountFinalSettlementModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub settlement_date: NaiveDate,
    pub current_balance: Decimal,
    pub accrued_interest: Decimal,
    pub closure_fees: Decimal,
    pub final_amount: Decimal,
    pub disbursement_method: DbDisbursementMethod,
    pub disbursement_reference: Option<HeaplessString<100>>,
    /// References Person.person_id
    pub processed_by_person_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Database model for Final Settlement (alias for compatibility)
pub type FinalSettlementModel = AccountFinalSettlementModel;


/// Database model for Disbursement Instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct DisbursementInstructionsModel {
    pub id: Uuid,
    /// References the account holding the loan (source of funds)
    pub source_account_id: Uuid,
    pub method: DbDisbursementMethod,
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
    pub status: DbDisbursementStatus,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by_person_id: Uuid,
    /// References Person.person_id
    pub updated_by_person_id: Uuid,
}

// Note: DisbursementStatus is now imported from banking_api::domain

/// Database model for Ultimate Beneficial Owner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct UltimateBeneficiaryModel {
    pub id: Uuid,
    pub corporate_customer_id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    pub control_type: DbControlType,
    pub description: Option<HeaplessString<200>>,
    pub status: DbUboStatus,
    pub verification_status: DbVerificationStatus,
    pub created_at: DateTime<Utc>,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_type", rename_all = "PascalCase")]
pub enum DbAccountType {
    Savings,
    Current,
    Loan,
}

impl FromStr for DbAccountType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Savings" => Ok(DbAccountType::Savings),
            "Current" => Ok(DbAccountType::Current),
            "Loan" => Ok(DbAccountType::Loan),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "account_status", rename_all = "PascalCase")]
pub enum DbAccountStatus {
    PendingApproval,
    Active,
    Dormant,
    Frozen,
    PendingClosure,
    Closed,
    PendingReactivation,
}

impl FromStr for DbAccountStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PendingApproval" => Ok(DbAccountStatus::PendingApproval),
            "Active" => Ok(DbAccountStatus::Active),
            "Dormant" => Ok(DbAccountStatus::Dormant),
            "Frozen" => Ok(DbAccountStatus::Frozen),
            "PendingClosure" => Ok(DbAccountStatus::PendingClosure),
            "Closed" => Ok(DbAccountStatus::Closed),
            "PendingReactivation" => Ok(DbAccountStatus::PendingReactivation),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "signing_condition", rename_all = "PascalCase")]
pub enum DbSigningCondition {
    None,
    AnyOwner,
    AllOwners,
}

impl FromStr for DbSigningCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(DbSigningCondition::None),
            "AnyOwner" => Ok(DbSigningCondition::AnyOwner),
            "AllOwners" => Ok(DbSigningCondition::AllOwners),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "disbursement_method", rename_all = "PascalCase")]
pub enum DbDisbursementMethod {
    Transfer,
    CashWithdrawal,
    Check,
    HoldFunds,
    OverdraftFacility,
    StagedRelease,
}

impl FromStr for DbDisbursementMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Transfer" => Ok(DbDisbursementMethod::Transfer),
            "CashWithdrawal" => Ok(DbDisbursementMethod::CashWithdrawal),
            "Check" => Ok(DbDisbursementMethod::Check),
            "HoldFunds" => Ok(DbDisbursementMethod::HoldFunds),
            "OverdraftFacility" => Ok(DbDisbursementMethod::OverdraftFacility),
            "StagedRelease" => Ok(DbDisbursementMethod::StagedRelease),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "disbursement_status", rename_all = "PascalCase")]
pub enum DbDisbursementStatus {
    Pending,
    Approved,
    Executed,
    Cancelled,
    Failed,
    PartiallyExecuted,
}

impl FromStr for DbDisbursementStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(DbDisbursementStatus::Pending),
            "Approved" => Ok(DbDisbursementStatus::Approved),
            "Executed" => Ok(DbDisbursementStatus::Executed),
            "Cancelled" => Ok(DbDisbursementStatus::Cancelled),
            "Failed" => Ok(DbDisbursementStatus::Failed),
            "PartiallyExecuted" => Ok(DbDisbursementStatus::PartiallyExecuted),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ownership_type", rename_all = "PascalCase")]
pub enum DbOwnershipType {
    Single,
    Joint,
    Corporate,
}

impl FromStr for DbOwnershipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Single" => Ok(DbOwnershipType::Single),
            "Joint" => Ok(DbOwnershipType::Joint),
            "Corporate" => Ok(DbOwnershipType::Corporate),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "entity_type", rename_all = "PascalCase")]
pub enum DbEntityType {
    Branch,
    Agent,
    RiskManager,
    ComplianceOfficer,
    CustomerService,
}

impl FromStr for DbEntityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Branch" => Ok(DbEntityType::Branch),
            "Agent" => Ok(DbEntityType::Agent),
            "RiskManager" => Ok(DbEntityType::RiskManager),
            "ComplianceOfficer" => Ok(DbEntityType::ComplianceOfficer),
            "CustomerService" => Ok(DbEntityType::CustomerService),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "relationship_type", rename_all = "PascalCase")]
pub enum DbRelationshipType {
    PrimaryHandler,
    BackupHandler,
    RiskOversight,
    ComplianceOversight,
    Accountant,
}

impl FromStr for DbRelationshipType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PrimaryHandler" => Ok(DbRelationshipType::PrimaryHandler),
            "BackupHandler" => Ok(DbRelationshipType::BackupHandler),
            "RiskOversight" => Ok(DbRelationshipType::RiskOversight),
            "ComplianceOversight" => Ok(DbRelationshipType::ComplianceOversight),
            "Accountant" => Ok(DbRelationshipType::Accountant),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "relationship_status", rename_all = "PascalCase")]
pub enum DbRelationshipStatus {
    Active,
    Inactive,
    Suspended,
}

impl FromStr for DbRelationshipStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(DbRelationshipStatus::Active),
            "Inactive" => Ok(DbRelationshipStatus::Inactive),
            "Suspended" => Ok(DbRelationshipStatus::Suspended),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "permission_type", rename_all = "PascalCase")]
pub enum DbPermissionType {
    ViewOnly,
    LimitedWithdrawal,
    JointApproval,
    FullAccess,
}

impl FromStr for DbPermissionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ViewOnly" => Ok(DbPermissionType::ViewOnly),
            "LimitedWithdrawal" => Ok(DbPermissionType::LimitedWithdrawal),
            "JointApproval" => Ok(DbPermissionType::JointApproval),
            "FullAccess" => Ok(DbPermissionType::FullAccess),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "mandate_status", rename_all = "PascalCase")]
pub enum DbMandateStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
}

impl FromStr for DbMandateStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(DbMandateStatus::Active),
            "Suspended" => Ok(DbMandateStatus::Suspended),
            "Revoked" => Ok(DbMandateStatus::Revoked),
            "Expired" => Ok(DbMandateStatus::Expired),
            _ => Err(()),
        }
    }
}

impl FromStr for DbControlType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DirectOwnership" => Ok(DbControlType::DirectOwnership),
            "IndirectOwnership" => Ok(DbControlType::IndirectOwnership),
            "SignificantInfluence" => Ok(DbControlType::SignificantInfluence),
            "SeniorManagement" => Ok(DbControlType::SeniorManagement),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "control_type", rename_all = "PascalCase")]
pub enum DbControlType {
    DirectOwnership,
    IndirectOwnership,
    SignificantInfluence,
    SeniorManagement,
}

impl FromStr for DbVerificationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(DbVerificationStatus::Pending),
            "Verified" => Ok(DbVerificationStatus::Verified),
            "Rejected" => Ok(DbVerificationStatus::Rejected),
            "RequiresUpdate" => Ok(DbVerificationStatus::RequiresUpdate),
            _ => Err(()),
        }
    }
}

impl FromStr for DbUboStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(DbUboStatus::Active),
            "Inactive" => Ok(DbUboStatus::Inactive),
            "UnderReview" => Ok(DbUboStatus::UnderReview),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verification_status", rename_all = "PascalCase")]
pub enum DbVerificationStatus {
    Pending,
    Verified,
    Rejected,
    RequiresUpdate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ubo_status", rename_all = "PascalCase")]
pub enum DbUboStatus {
    Active,
    Inactive,
    UnderReview,
}

impl AccountModel {
    /// Set product id
    pub fn set_product_id(&mut self, product_id: Uuid) {
        self.product_id = product_id;
    }
}