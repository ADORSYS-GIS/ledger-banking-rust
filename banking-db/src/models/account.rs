use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use banking_api::domain::{
    AccountType, AccountStatus, SigningCondition, DisbursementMethod, HoldType, HoldStatus, 
    HoldPriority, OwnershipType, EntityType, RelationshipType, RelationshipStatus, 
    PermissionType, MandateStatus, ControlType, VerificationStatus, UboStatus
};

/// Database model for Account table with enhanced fields from banking enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountModel {
    pub account_id: Uuid,
    pub product_code: HeaplessString<12>,
    #[serde(
        serialize_with = "serialize_account_type",
        deserialize_with = "deserialize_account_type"
    )]
    pub account_type: AccountType,
    #[serde(
        serialize_with = "serialize_account_status",
        deserialize_with = "deserialize_account_status"
    )]
    pub account_status: AccountStatus,
    #[serde(
        serialize_with = "serialize_signing_condition",
        deserialize_with = "deserialize_signing_condition"
    )]
    pub signing_condition: SigningCondition,
    pub currency: HeaplessString<3>,
    pub open_date: NaiveDate,
    pub domicile_branch_id: Uuid,
    
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
    /// References to DisbursementInstructions.disbursement_id (stored as JSON array, max 10 stages)
    pub disbursement_instructions: serde_json::Value, // Will store Vec<Uuid> as JSON
    
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

/// Database model for Account Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountOwnershipModel {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    #[serde(
        serialize_with = "serialize_ownership_type",
        deserialize_with = "deserialize_ownership_type"
    )]
    pub ownership_type: OwnershipType,
    pub ownership_percentage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Account Relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountRelationshipModel {
    pub relationship_id: Uuid,
    pub account_id: Uuid,
    pub entity_id: Uuid,
    #[serde(
        serialize_with = "serialize_entity_type",
        deserialize_with = "deserialize_entity_type"
    )]
    pub entity_type: EntityType,
    #[serde(
        serialize_with = "serialize_relationship_type",
        deserialize_with = "deserialize_relationship_type"
    )]
    pub relationship_type: RelationshipType,
    #[serde(
        serialize_with = "serialize_relationship_status",
        deserialize_with = "deserialize_relationship_status"
    )]
    pub status: RelationshipStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Database model for Account Mandates
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountMandateModel {
    pub mandate_id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    #[serde(
        serialize_with = "serialize_permission_type",
        deserialize_with = "deserialize_permission_type"
    )]
    pub permission_type: PermissionType,
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_mandate_status",
        deserialize_with = "deserialize_mandate_status"
    )]
    pub status: MandateStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

/// Database model for Account Holds
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldModel {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    #[serde(
        serialize_with = "serialize_hold_type",
        deserialize_with = "deserialize_hold_type"
    )]
    pub hold_type: HoldType,
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub placed_by: Uuid,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(
        serialize_with = "serialize_hold_status",
        deserialize_with = "deserialize_hold_status"
    )]
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    /// References Person.person_id
    pub released_by: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_hold_priority",
        deserialize_with = "deserialize_hold_priority"
    )]
    pub priority: HoldPriority,
    pub source_reference: Option<HeaplessString<100>>,
    pub automatic_release: bool,
}

/// Database model for Account Status History (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountStatusHistoryModel {
    pub history_id: Uuid,
    pub account_id: Uuid,
    #[serde(
        serialize_with = "serialize_account_status_option",
        deserialize_with = "deserialize_account_status_option"
    )]
    pub old_status: Option<AccountStatus>,
    #[serde(
        serialize_with = "serialize_account_status",
        deserialize_with = "deserialize_account_status"
    )]
    pub new_status: AccountStatus,
    /// References ReasonAndPurpose.id
    pub change_reason_id: Uuid,
    /// Additional context for status change
    pub additional_context: Option<HeaplessString<200>>,
    /// References Person.person_id
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
    pub created_at: DateTime<Utc>,
}

/// Database model for Final Settlements (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountFinalSettlementModel {
    pub settlement_id: Uuid,
    pub account_id: Uuid,
    pub settlement_date: NaiveDate,
    pub current_balance: Decimal,
    pub accrued_interest: Decimal,
    pub closure_fees: Decimal,
    pub final_amount: Decimal,
    #[serde(
        serialize_with = "serialize_disbursement_method",
        deserialize_with = "deserialize_disbursement_method"
    )]
    pub disbursement_method: DisbursementMethod,
    pub disbursement_reference: Option<HeaplessString<100>>,
    /// References Person.person_id
    pub processed_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Database model for Final Settlement (alias for compatibility)
pub type FinalSettlementModel = AccountFinalSettlementModel;

/// Database model for Status Change (alias for compatibility) 
pub type StatusChangeModel = AccountStatusHistoryModel;

/// Database model for Disbursement Instructions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct DisbursementInstructionsModel {
    pub disbursement_id: Uuid,
    /// References the account holding the loan (source of funds)
    pub source_account_id: Uuid,
    #[serde(
        serialize_with = "serialize_disbursement_method",
        deserialize_with = "deserialize_disbursement_method"
    )]
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
    #[serde(
        serialize_with = "serialize_disbursement_status",
        deserialize_with = "deserialize_disbursement_status"
    )]
    pub status: DisbursementStatus,
    
    // Audit trail
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub created_by: Uuid,
    /// References Person.person_id
    pub updated_by: Uuid,
}

/// Database model for disbursement status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "disbursement_status", rename_all = "lowercase")]
pub enum DisbursementStatus {
    Pending,
    Approved,
    Executed,
    Cancelled,
    Failed,
    PartiallyExecuted,
}

/// Database model for Ultimate Beneficial Owner
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct UltimateBeneficiaryModel {
    pub ubo_link_id: Uuid,
    pub corporate_customer_id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    #[serde(
        serialize_with = "serialize_control_type",
        deserialize_with = "deserialize_control_type"
    )]
    pub control_type: ControlType,
    pub description: Option<HeaplessString<256>>,
    #[serde(
        serialize_with = "serialize_ubo_status",
        deserialize_with = "deserialize_ubo_status"
    )]
    pub status: UboStatus,
    #[serde(
        serialize_with = "serialize_verification_status",
        deserialize_with = "deserialize_verification_status"
    )]
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

// Enum serialization helpers for database compatibility
fn serialize_account_type<S>(account_type: &AccountType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match account_type {
        AccountType::Savings => "Savings",
        AccountType::Current => "Current", 
        AccountType::Loan => "Loan",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_account_type<'de, D>(deserializer: D) -> Result<AccountType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "Savings" => Ok(AccountType::Savings),
        "Current" => Ok(AccountType::Current),
        "Loan" => Ok(AccountType::Loan),
        _ => Err(serde::de::Error::custom(format!("Invalid account type: {type_str}"))),
    }
}

fn serialize_account_status<S>(account_status: &AccountStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match account_status {
        AccountStatus::PendingApproval => "PendingApproval",
        AccountStatus::Active => "Active",
        AccountStatus::Dormant => "Dormant",
        AccountStatus::Frozen => "Frozen",
        AccountStatus::PendingClosure => "PendingClosure",
        AccountStatus::Closed => "Closed",
        AccountStatus::PendingReactivation => "PendingReactivation",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_account_status<'de, D>(deserializer: D) -> Result<AccountStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "PendingApproval" => Ok(AccountStatus::PendingApproval),
        "Active" => Ok(AccountStatus::Active),
        "Dormant" => Ok(AccountStatus::Dormant),
        "Frozen" => Ok(AccountStatus::Frozen),
        "PendingClosure" => Ok(AccountStatus::PendingClosure),
        "Closed" => Ok(AccountStatus::Closed),
        "PendingReactivation" => Ok(AccountStatus::PendingReactivation),
        _ => Err(serde::de::Error::custom(format!("Invalid account status: {status_str}"))),
    }
}

fn serialize_signing_condition<S>(signing_condition: &SigningCondition, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let condition_str = match signing_condition {
        SigningCondition::None => "None",
        SigningCondition::AnyOwner => "AnyOwner",
        SigningCondition::AllOwners => "AllOwners",
    };
    serializer.serialize_str(condition_str)
}

fn deserialize_signing_condition<'de, D>(deserializer: D) -> Result<SigningCondition, D::Error>
where
    D: Deserializer<'de>,
{
    let condition_str = String::deserialize(deserializer)?;
    match condition_str.as_str() {
        "None" => Ok(SigningCondition::None),
        "AnyOwner" => Ok(SigningCondition::AnyOwner),
        "AllOwners" => Ok(SigningCondition::AllOwners),
        _ => Err(serde::de::Error::custom(format!("Invalid signing condition: {condition_str}"))),
    }
}

// Additional enum serialization functions for new enums

fn serialize_disbursement_method<S>(method: &DisbursementMethod, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let method_str = match method {
        DisbursementMethod::Transfer => "Transfer",
        DisbursementMethod::CashWithdrawal => "CashWithdrawal",
        DisbursementMethod::Check => "Check",
        DisbursementMethod::HoldFunds => "HoldFunds",
        DisbursementMethod::OverdraftFacility => "OverdraftFacility",
        DisbursementMethod::StagedRelease => "StagedRelease",
    };
    serializer.serialize_str(method_str)
}

fn deserialize_disbursement_method<'de, D>(deserializer: D) -> Result<DisbursementMethod, D::Error>
where
    D: Deserializer<'de>,
{
    let method_str = String::deserialize(deserializer)?;
    match method_str.as_str() {
        "Transfer" => Ok(DisbursementMethod::Transfer),
        "CashWithdrawal" => Ok(DisbursementMethod::CashWithdrawal),
        "Check" => Ok(DisbursementMethod::Check),
        "HoldFunds" => Ok(DisbursementMethod::HoldFunds),
        "OverdraftFacility" => Ok(DisbursementMethod::OverdraftFacility),
        "StagedRelease" => Ok(DisbursementMethod::StagedRelease),
        _ => Err(serde::de::Error::custom(format!("Invalid disbursement method: {method_str}"))),
    }
}

fn serialize_hold_type<S>(hold_type: &HoldType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match hold_type {
        HoldType::UnclearedFunds => "UnclearedFunds",
        HoldType::JudicialLien => "JudicialLien",
        HoldType::LoanPledge => "LoanPledge",
        HoldType::ComplianceHold => "ComplianceHold",
        HoldType::AdministrativeHold => "AdministrativeHold",
        HoldType::FraudHold => "FraudHold",
        HoldType::PendingAuthorization => "PendingAuthorization",
        HoldType::OverdraftReserve => "OverdraftReserve",
        HoldType::CardAuthorization => "CardAuthorization",
        HoldType::Other => "Other",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_hold_type<'de, D>(deserializer: D) -> Result<HoldType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "UnclearedFunds" => Ok(HoldType::UnclearedFunds),
        "JudicialLien" => Ok(HoldType::JudicialLien),
        "LoanPledge" => Ok(HoldType::LoanPledge),
        "ComplianceHold" => Ok(HoldType::ComplianceHold),
        "AdministrativeHold" => Ok(HoldType::AdministrativeHold),
        "FraudHold" => Ok(HoldType::FraudHold),
        "PendingAuthorization" => Ok(HoldType::PendingAuthorization),
        "OverdraftReserve" => Ok(HoldType::OverdraftReserve),
        "CardAuthorization" => Ok(HoldType::CardAuthorization),
        "Other" => Ok(HoldType::Other),
        _ => Err(serde::de::Error::custom(format!("Invalid hold type: {type_str}"))),
    }
}

fn serialize_hold_status<S>(status: &HoldStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        HoldStatus::Active => "Active",
        HoldStatus::Released => "Released",
        HoldStatus::Expired => "Expired",
        HoldStatus::Cancelled => "Cancelled",
        HoldStatus::PartiallyReleased => "PartiallyReleased",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_hold_status<'de, D>(deserializer: D) -> Result<HoldStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Active" => Ok(HoldStatus::Active),
        "Released" => Ok(HoldStatus::Released),
        "Expired" => Ok(HoldStatus::Expired),
        "Cancelled" => Ok(HoldStatus::Cancelled),
        "PartiallyReleased" => Ok(HoldStatus::PartiallyReleased),
        _ => Err(serde::de::Error::custom(format!("Invalid hold status: {status_str}"))),
    }
}

fn serialize_hold_priority<S>(priority: &HoldPriority, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let priority_str = match priority {
        HoldPriority::Critical => "Critical",
        HoldPriority::High => "High",
        HoldPriority::Medium => "Medium",
        HoldPriority::Low => "Low",
    };
    serializer.serialize_str(priority_str)
}

fn deserialize_hold_priority<'de, D>(deserializer: D) -> Result<HoldPriority, D::Error>
where
    D: Deserializer<'de>,
{
    let priority_str = String::deserialize(deserializer)?;
    match priority_str.as_str() {
        "Critical" => Ok(HoldPriority::Critical),
        "High" => Ok(HoldPriority::High),
        "Medium" => Ok(HoldPriority::Medium),
        "Low" => Ok(HoldPriority::Low),
        _ => Err(serde::de::Error::custom(format!("Invalid hold priority: {priority_str}"))),
    }
}

fn serialize_ownership_type<S>(ownership_type: &OwnershipType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match ownership_type {
        OwnershipType::Single => "Single",
        OwnershipType::Joint => "Joint",
        OwnershipType::Corporate => "Corporate",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_ownership_type<'de, D>(deserializer: D) -> Result<OwnershipType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "Single" => Ok(OwnershipType::Single),
        "Joint" => Ok(OwnershipType::Joint),
        "Corporate" => Ok(OwnershipType::Corporate),
        _ => Err(serde::de::Error::custom(format!("Invalid ownership type: {type_str}"))),
    }
}

fn serialize_entity_type<S>(entity_type: &EntityType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match entity_type {
        EntityType::Branch => "Branch",
        EntityType::Agent => "Agent",
        EntityType::RiskManager => "RiskManager",
        EntityType::ComplianceOfficer => "ComplianceOfficer",
        EntityType::CustomerService => "CustomerService",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_entity_type<'de, D>(deserializer: D) -> Result<EntityType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "Branch" => Ok(EntityType::Branch),
        "Agent" => Ok(EntityType::Agent),
        "RiskManager" => Ok(EntityType::RiskManager),
        "ComplianceOfficer" => Ok(EntityType::ComplianceOfficer),
        "CustomerService" => Ok(EntityType::CustomerService),
        _ => Err(serde::de::Error::custom(format!("Invalid entity type: {type_str}"))),
    }
}

fn serialize_relationship_type<S>(relationship_type: &RelationshipType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match relationship_type {
        RelationshipType::PrimaryHandler => "PrimaryHandler",
        RelationshipType::BackupHandler => "BackupHandler",
        RelationshipType::RiskOversight => "RiskOversight",
        RelationshipType::ComplianceOversight => "ComplianceOversight",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_relationship_type<'de, D>(deserializer: D) -> Result<RelationshipType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "PrimaryHandler" => Ok(RelationshipType::PrimaryHandler),
        "BackupHandler" => Ok(RelationshipType::BackupHandler),
        "RiskOversight" => Ok(RelationshipType::RiskOversight),
        "ComplianceOversight" => Ok(RelationshipType::ComplianceOversight),
        _ => Err(serde::de::Error::custom(format!("Invalid relationship type: {type_str}"))),
    }
}

fn serialize_relationship_status<S>(status: &RelationshipStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        RelationshipStatus::Active => "Active",
        RelationshipStatus::Inactive => "Inactive",
        RelationshipStatus::Suspended => "Suspended",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_relationship_status<'de, D>(deserializer: D) -> Result<RelationshipStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Active" => Ok(RelationshipStatus::Active),
        "Inactive" => Ok(RelationshipStatus::Inactive),
        "Suspended" => Ok(RelationshipStatus::Suspended),
        _ => Err(serde::de::Error::custom(format!("Invalid relationship status: {status_str}"))),
    }
}

fn serialize_permission_type<S>(permission_type: &PermissionType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match permission_type {
        PermissionType::ViewOnly => "ViewOnly",
        PermissionType::LimitedWithdrawal => "LimitedWithdrawal",
        PermissionType::JointApproval => "JointApproval",
        PermissionType::FullAccess => "FullAccess",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_permission_type<'de, D>(deserializer: D) -> Result<PermissionType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "ViewOnly" => Ok(PermissionType::ViewOnly),
        "LimitedWithdrawal" => Ok(PermissionType::LimitedWithdrawal),
        "JointApproval" => Ok(PermissionType::JointApproval),
        "FullAccess" => Ok(PermissionType::FullAccess),
        _ => Err(serde::de::Error::custom(format!("Invalid permission type: {type_str}"))),
    }
}

fn serialize_mandate_status<S>(status: &MandateStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        MandateStatus::Active => "Active",
        MandateStatus::Suspended => "Suspended",
        MandateStatus::Revoked => "Revoked",
        MandateStatus::Expired => "Expired",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_mandate_status<'de, D>(deserializer: D) -> Result<MandateStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Active" => Ok(MandateStatus::Active),
        "Suspended" => Ok(MandateStatus::Suspended),
        "Revoked" => Ok(MandateStatus::Revoked),
        "Expired" => Ok(MandateStatus::Expired),
        _ => Err(serde::de::Error::custom(format!("Invalid mandate status: {status_str}"))),
    }
}

fn serialize_control_type<S>(control_type: &ControlType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let type_str = match control_type {
        ControlType::DirectOwnership => "DirectOwnership",
        ControlType::IndirectOwnership => "IndirectOwnership",
        ControlType::SignificantInfluence => "SignificantInfluence",
        ControlType::SeniorManagement => "SeniorManagement",
    };
    serializer.serialize_str(type_str)
}

fn deserialize_control_type<'de, D>(deserializer: D) -> Result<ControlType, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str = String::deserialize(deserializer)?;
    match type_str.as_str() {
        "DirectOwnership" => Ok(ControlType::DirectOwnership),
        "IndirectOwnership" => Ok(ControlType::IndirectOwnership),
        "SignificantInfluence" => Ok(ControlType::SignificantInfluence),
        "SeniorManagement" => Ok(ControlType::SeniorManagement),
        _ => Err(serde::de::Error::custom(format!("Invalid control type: {type_str}"))),
    }
}

fn serialize_verification_status<S>(status: &VerificationStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        VerificationStatus::Pending => "Pending",
        VerificationStatus::Verified => "Verified",
        VerificationStatus::Rejected => "Rejected",
        VerificationStatus::RequiresUpdate => "RequiresUpdate",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_verification_status<'de, D>(deserializer: D) -> Result<VerificationStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Pending" => Ok(VerificationStatus::Pending),
        "Verified" => Ok(VerificationStatus::Verified),
        "Rejected" => Ok(VerificationStatus::Rejected),
        "RequiresUpdate" => Ok(VerificationStatus::RequiresUpdate),
        _ => Err(serde::de::Error::custom(format!("Invalid verification status: {status_str}"))),
    }
}

fn serialize_ubo_status<S>(status: &UboStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        UboStatus::Active => "Active",
        UboStatus::Inactive => "Inactive",
        UboStatus::UnderReview => "UnderReview",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_ubo_status<'de, D>(deserializer: D) -> Result<UboStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Active" => Ok(UboStatus::Active),
        "Inactive" => Ok(UboStatus::Inactive),
        "UnderReview" => Ok(UboStatus::UnderReview),
        _ => Err(serde::de::Error::custom(format!("Invalid UBO status: {status_str}"))),
    }
}

fn serialize_account_status_option<S>(status: &Option<AccountStatus>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match status {
        Some(status) => serialize_account_status(status, serializer),
        None => serializer.serialize_none(),
    }
}

fn deserialize_account_status_option<'de, D>(deserializer: D) -> Result<Option<AccountStatus>, D::Error>
where
    D: Deserializer<'de>,
{
    let option_str: Option<String> = Option::deserialize(deserializer)?;
    match option_str {
        Some(status_str) => {
            let status = match status_str.as_str() {
                "PendingApproval" => AccountStatus::PendingApproval,
                "Active" => AccountStatus::Active,
                "Dormant" => AccountStatus::Dormant,
                "Frozen" => AccountStatus::Frozen,
                "PendingClosure" => AccountStatus::PendingClosure,
                "Closed" => AccountStatus::Closed,
                "PendingReactivation" => AccountStatus::PendingReactivation,
                _ => return Err(serde::de::Error::custom(format!("Invalid account status: {status_str}"))),
            };
            Ok(Some(status))
        }
        None => Ok(None),
    }
}



fn serialize_disbursement_status<S>(status: &DisbursementStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let status_str = match status {
        DisbursementStatus::Pending => "Pending",
        DisbursementStatus::Approved => "Approved",
        DisbursementStatus::Executed => "Executed",
        DisbursementStatus::Cancelled => "Cancelled",
        DisbursementStatus::Failed => "Failed",
        DisbursementStatus::PartiallyExecuted => "PartiallyExecuted",
    };
    serializer.serialize_str(status_str)
}

fn deserialize_disbursement_status<'de, D>(deserializer: D) -> Result<DisbursementStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let status_str = String::deserialize(deserializer)?;
    match status_str.as_str() {
        "Pending" => Ok(DisbursementStatus::Pending),
        "Approved" => Ok(DisbursementStatus::Approved),
        "Executed" => Ok(DisbursementStatus::Executed),
        "Cancelled" => Ok(DisbursementStatus::Cancelled),
        "Failed" => Ok(DisbursementStatus::Failed),
        "PartiallyExecuted" => Ok(DisbursementStatus::PartiallyExecuted),
        _ => Err(serde::de::Error::custom(format!("Invalid disbursement status: {status_str}"))),
    }
}

impl AccountModel {
    /// Set product code from string with validation
    pub fn set_product_code(&mut self, product_code: &str) -> Result<(), &'static str> {
        self.product_code = HeaplessString::try_from(product_code).map_err(|_| "Product code too long")?;
        Ok(())
    }
}