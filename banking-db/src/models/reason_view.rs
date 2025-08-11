use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use banking_api::domain::{
    AccountType, AccountStatus, HoldType, HoldStatus, TransactionAuditAction, 
    WorkflowType, WorkflowStatus, RestructuringType, SarStatus, ReasonSeverity
};

/// Database model for ReasonView
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct ReasonViewModel {
    pub id: Uuid,
    pub code: HeaplessString<50>,
    pub text: HeaplessString<200>,  // Resolved based on user's language preference
    pub requires_details: bool,
    pub additional_details: Option<HeaplessString<500>>,
    #[serde(serialize_with = "serialize_reason_severity_option", deserialize_with = "deserialize_reason_severity_option")]
    pub severity: Option<ReasonSeverity>,
    pub category: HeaplessString<100>,
    pub context: HeaplessString<100>,
}

/// Database model for AccountView with resolved reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountViewModel {
    pub id: Uuid,
    pub product_id: Uuid,
    #[serde(serialize_with = "serialize_account_type", deserialize_with = "deserialize_account_type")]
    pub account_type: AccountType,
    #[serde(serialize_with = "serialize_account_status", deserialize_with = "deserialize_account_status")]
    pub account_status: AccountStatus,
    pub currency: HeaplessString<3>,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    
    // Resolved reason fields
    pub loan_purpose: Option<ReasonViewModel>,
    pub pending_closure_reason: Option<ReasonViewModel>,
    pub status_change_reason: Option<ReasonViewModel>,
    
    // Other fields...
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for AccountHoldView with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountHoldViewModel {
    pub id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    #[serde(serialize_with = "serialize_hold_type", deserialize_with = "deserialize_hold_type")]
    pub hold_type: HoldType,
    #[serde(serialize_with = "serialize_hold_status", deserialize_with = "deserialize_hold_status")]
    pub status: HoldStatus,
    pub placed_by: Uuid, // References Person.person_id
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    
    // Resolved reason
    pub reason: ReasonViewModel,
    
    pub released_at: Option<DateTime<Utc>>,
    pub released_by: Option<Uuid>, // References Person.person_id
}

/// Database model for FeeWaiverView with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct FeeWaiverViewModel {
    pub id: Uuid,
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub waived_amount: Decimal,
    pub waived_by: Uuid, // References Person.person_id
    pub waived_at: DateTime<Utc>,
    
    // Resolved reason
    pub reason: ReasonViewModel,
    
    pub approval_required: bool,
    pub approved_by: Option<Uuid>, // References Person.person_id
    pub approved_at: Option<DateTime<Utc>>,
}

/// Database model for TransactionAuditView with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct TransactionAuditViewModel {
    pub id: Uuid,
    pub transaction_id: Uuid,
    #[serde(serialize_with = "serialize_transaction_audit_action", deserialize_with = "deserialize_transaction_audit_action")]
    pub action_type: TransactionAuditAction,
    pub performed_by: Uuid, // References Person.person_id
    pub performed_at: DateTime<Utc>,
    pub old_status: Option<HeaplessString<50>>,
    pub new_status: Option<HeaplessString<50>>,
    
    // Resolved reason
    pub reason: Option<ReasonViewModel>,
}

/// Database model for SAR (Suspicious Activity Report) view with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct SarDataViewModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub supporting_transactions: Vec<Uuid>,
    pub generated_at: DateTime<Utc>,
    #[serde(serialize_with = "serialize_sar_status", deserialize_with = "deserialize_sar_status")]
    pub status: SarStatus,
    
    // Resolved reason
    pub reason: ReasonViewModel,
}

/// Database model for LoanRestructuringView with resolved reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct LoanRestructuringViewModel {
    pub id: Uuid,
    pub loan_account_id: Uuid,
    #[serde(serialize_with = "serialize_restructuring_type", deserialize_with = "deserialize_restructuring_type")]
    pub restructuring_type: RestructuringType,
    pub request_date: NaiveDate,
    pub effective_date: Option<NaiveDate>,
    
    // Resolved reason
    pub restructuring_reason: ReasonViewModel,
    
    // Original loan terms
    pub original_principal: Decimal,
    pub original_interest_rate: Decimal,
    pub original_term_months: u32,
    pub original_installment: Decimal,
    
    // New loan terms
    pub new_principal: Option<Decimal>,
    pub new_interest_rate: Option<Decimal>,
    pub new_term_months: Option<u32>,
    pub new_installment: Option<Decimal>,
}

/// Database model for WorkflowApprovalView with resolved rejection reason
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct WorkflowApprovalViewModel {
    pub id: Uuid,
    pub account_id: Uuid,
    #[serde(serialize_with = "serialize_workflow_type", deserialize_with = "deserialize_workflow_type")]
    pub workflow_type: WorkflowType,
    pub current_step: HeaplessString<50>,
    #[serde(serialize_with = "serialize_workflow_status", deserialize_with = "deserialize_workflow_status")]
    pub status: WorkflowStatus,
    pub initiated_by: Uuid, // References Person.person_id
    pub initiated_at: DateTime<Utc>,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    
    // Resolved rejection reason
    pub rejection_reason: Option<ReasonViewModel>,
}

// Custom serialization functions

pub fn serialize_reason_severity_option<S>(value: &Option<ReasonSeverity>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    match value {
        Some(severity) => {
            let value_str = match severity {
                ReasonSeverity::Critical => "Critical",
                ReasonSeverity::High => "High",
                ReasonSeverity::Medium => "Medium",
                ReasonSeverity::Low => "Low",
                ReasonSeverity::Informational => "Informational",
            };
            serializer.serialize_some(value_str)
        },
        None => serializer.serialize_none()
    }
}

pub fn deserialize_reason_severity_option<'de, D>(deserializer: D) -> Result<Option<ReasonSeverity>, D::Error>
where D: Deserializer<'de> {
    let value_opt: Option<String> = Option::deserialize(deserializer)?;
    match value_opt {
        Some(value_str) => {
            match value_str.as_str() {
                "Critical" => Ok(Some(ReasonSeverity::Critical)),
                "High" => Ok(Some(ReasonSeverity::High)),
                "Medium" => Ok(Some(ReasonSeverity::Medium)),
                "Low" => Ok(Some(ReasonSeverity::Low)),
                "Informational" => Ok(Some(ReasonSeverity::Informational)),
                _ => Err(serde::de::Error::custom("Invalid ReasonSeverity value"))
            }
        },
        None => Ok(None)
    }
}

pub fn serialize_account_type<S>(value: &AccountType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        AccountType::Savings => "Savings",
        AccountType::Current => "Current",
        AccountType::Loan => "Loan",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_account_type<'de, D>(deserializer: D) -> Result<AccountType, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Savings" => Ok(AccountType::Savings),
        "Current" => Ok(AccountType::Current),
        "Loan" => Ok(AccountType::Loan),
        _ => Err(serde::de::Error::custom("Invalid AccountType value"))
    }
}

pub fn serialize_account_status<S>(value: &AccountStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        AccountStatus::PendingApproval => "PendingApproval",
        AccountStatus::Active => "Active",
        AccountStatus::Dormant => "Dormant",
        AccountStatus::Frozen => "Frozen",
        AccountStatus::PendingClosure => "PendingClosure",
        AccountStatus::Closed => "Closed",
        AccountStatus::PendingReactivation => "PendingReactivation",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_account_status<'de, D>(deserializer: D) -> Result<AccountStatus, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "PendingApproval" => Ok(AccountStatus::PendingApproval),
        "Active" => Ok(AccountStatus::Active),
        "Dormant" => Ok(AccountStatus::Dormant),
        "Frozen" => Ok(AccountStatus::Frozen),
        "PendingClosure" => Ok(AccountStatus::PendingClosure),
        "Closed" => Ok(AccountStatus::Closed),
        "PendingReactivation" => Ok(AccountStatus::PendingReactivation),
        _ => Err(serde::de::Error::custom("Invalid AccountStatus value"))
    }
}

pub fn serialize_hold_type<S>(value: &HoldType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
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
    serializer.serialize_str(value_str)
}

pub fn deserialize_hold_type<'de, D>(deserializer: D) -> Result<HoldType, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
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
        _ => Err(serde::de::Error::custom("Invalid HoldType value"))
    }
}

pub fn serialize_hold_status<S>(value: &HoldStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        HoldStatus::Active => "Active",
        HoldStatus::Released => "Released",
        HoldStatus::Expired => "Expired",
        HoldStatus::Cancelled => "Cancelled",
        HoldStatus::PartiallyReleased => "PartiallyReleased",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_hold_status<'de, D>(deserializer: D) -> Result<HoldStatus, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Active" => Ok(HoldStatus::Active),
        "Released" => Ok(HoldStatus::Released),
        "Expired" => Ok(HoldStatus::Expired),
        "Cancelled" => Ok(HoldStatus::Cancelled),
        "PartiallyReleased" => Ok(HoldStatus::PartiallyReleased),
        _ => Err(serde::de::Error::custom("Invalid HoldStatus value"))
    }
}

pub fn serialize_transaction_audit_action<S>(value: &TransactionAuditAction, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        TransactionAuditAction::Created => "Created",
        TransactionAuditAction::StatusChanged => "StatusChanged",
        TransactionAuditAction::Posted => "Posted",
        TransactionAuditAction::Reversed => "Reversed",
        TransactionAuditAction::Failed => "Failed",
        TransactionAuditAction::Approved => "Approved",
        TransactionAuditAction::Rejected => "Rejected",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_transaction_audit_action<'de, D>(deserializer: D) -> Result<TransactionAuditAction, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Created" => Ok(TransactionAuditAction::Created),
        "StatusChanged" => Ok(TransactionAuditAction::StatusChanged),
        "Posted" => Ok(TransactionAuditAction::Posted),
        "Reversed" => Ok(TransactionAuditAction::Reversed),
        "Failed" => Ok(TransactionAuditAction::Failed),
        "Approved" => Ok(TransactionAuditAction::Approved),
        "Rejected" => Ok(TransactionAuditAction::Rejected),
        _ => Err(serde::de::Error::custom("Invalid TransactionAuditAction value"))
    }
}

pub fn serialize_sar_status<S>(value: &SarStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        SarStatus::Draft => "Draft",
        SarStatus::Filed => "Filed",
        SarStatus::Acknowledged => "Acknowledged",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_sar_status<'de, D>(deserializer: D) -> Result<SarStatus, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Draft" => Ok(SarStatus::Draft),
        "Filed" => Ok(SarStatus::Filed),
        "Acknowledged" => Ok(SarStatus::Acknowledged),
        _ => Err(serde::de::Error::custom("Invalid SarStatus value"))
    }
}

pub fn serialize_restructuring_type<S>(value: &RestructuringType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        RestructuringType::PaymentHoliday => "PaymentHoliday",
        RestructuringType::TermExtension => "TermExtension",
        RestructuringType::RateReduction => "RateReduction",
        RestructuringType::PrincipalReduction => "PrincipalReduction",
        RestructuringType::InstallmentReduction => "InstallmentReduction",
        RestructuringType::InterestCapitalization => "InterestCapitalization",
        RestructuringType::FullRestructuring => "FullRestructuring",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_restructuring_type<'de, D>(deserializer: D) -> Result<RestructuringType, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "PaymentHoliday" => Ok(RestructuringType::PaymentHoliday),
        "TermExtension" => Ok(RestructuringType::TermExtension),
        "RateReduction" => Ok(RestructuringType::RateReduction),
        "PrincipalReduction" => Ok(RestructuringType::PrincipalReduction),
        "InstallmentReduction" => Ok(RestructuringType::InstallmentReduction),
        "InterestCapitalization" => Ok(RestructuringType::InterestCapitalization),
        "FullRestructuring" => Ok(RestructuringType::FullRestructuring),
        _ => Err(serde::de::Error::custom("Invalid RestructuringType value"))
    }
}

pub fn serialize_workflow_type<S>(value: &WorkflowType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        WorkflowType::AccountOpening => "AccountOpening",
        WorkflowType::AccountClosure => "AccountClosure",
        WorkflowType::AccountReactivation => "AccountReactivation",
        WorkflowType::ComplianceVerification => "ComplianceVerification",
        WorkflowType::MultiPartyApproval => "MultiPartyApproval",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_workflow_type<'de, D>(deserializer: D) -> Result<WorkflowType, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "AccountOpening" => Ok(WorkflowType::AccountOpening),
        "AccountClosure" => Ok(WorkflowType::AccountClosure),
        "AccountReactivation" => Ok(WorkflowType::AccountReactivation),
        "ComplianceVerification" => Ok(WorkflowType::ComplianceVerification),
        "MultiPartyApproval" => Ok(WorkflowType::MultiPartyApproval),
        _ => Err(serde::de::Error::custom("Invalid WorkflowType value"))
    }
}

pub fn serialize_workflow_status<S>(value: &WorkflowStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        WorkflowStatus::InProgress => "InProgress",
        WorkflowStatus::PendingAction => "PendingAction",
        WorkflowStatus::Completed => "Completed",
        WorkflowStatus::Failed => "Failed",
        WorkflowStatus::Cancelled => "Cancelled",
        WorkflowStatus::TimedOut => "TimedOut",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_workflow_status<'de, D>(deserializer: D) -> Result<WorkflowStatus, D::Error>
where D: Deserializer<'de> {
    let value_str: String = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "InProgress" => Ok(WorkflowStatus::InProgress),
        "PendingAction" => Ok(WorkflowStatus::PendingAction),
        "Completed" => Ok(WorkflowStatus::Completed),
        "Failed" => Ok(WorkflowStatus::Failed),
        "Cancelled" => Ok(WorkflowStatus::Cancelled),
        "TimedOut" => Ok(WorkflowStatus::TimedOut),
        _ => Err(serde::de::Error::custom("Invalid WorkflowStatus value"))
    }
}