use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use banking_api::domain::{AccountType, AccountStatus, SigningCondition};

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
    pub collateral_id: Option<HeaplessString<100>>,
    /// References ReasonAndPurpose.id for loan purpose
    pub loan_purpose_id: Option<Uuid>,

    // Account lifecycle management (from enhancements)
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>,
    pub dormancy_threshold_days: Option<i32>,
    pub reactivation_required: bool,
    /// References ReasonAndPurpose.id for pending closure
    pub pending_closure_reason_id: Option<Uuid>,
    
    // Enhanced audit trail
    pub status_changed_by: Option<HeaplessString<100>>,
    /// References ReasonAndPurpose.id for status change
    pub status_change_reason_id: Option<Uuid>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: HeaplessString<100>,
}

/// Database model for Account Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountOwnershipModel {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: HeaplessString<100>,
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
    pub entity_type: HeaplessString<100>,
    pub relationship_type: HeaplessString<100>,
    pub status: HeaplessString<100>,
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
    pub permission_type: HeaplessString<100>,
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    pub status: HeaplessString<100>,
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
    /// References ReasonAndPurpose.id
    pub reason_id: Uuid,
    /// Additional context beyond the standard reason
    pub additional_details: Option<HeaplessString<200>>,
    pub placed_by: HeaplessString<100>,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: HeaplessString<100>,
    pub released_by: Option<HeaplessString<100>>,
    pub released_at: Option<DateTime<Utc>>,
}

/// Database model for Account Status History (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountStatusHistoryModel {
    pub history_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<HeaplessString<100>>,
    pub new_status: HeaplessString<100>,
    /// References ReasonAndPurpose.id
    pub change_reason_id: Uuid,
    /// Additional context for status change
    pub additional_context: Option<HeaplessString<200>>,
    pub changed_by: HeaplessString<100>,
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
    pub disbursement_method: HeaplessString<100>,
    pub disbursement_reference: Option<HeaplessString<100>>,
    pub processed_by: HeaplessString<100>,
    pub created_at: DateTime<Utc>,
}

/// Database model for Final Settlement (alias for compatibility)
pub type FinalSettlementModel = AccountFinalSettlementModel;

/// Database model for Status Change (alias for compatibility) 
pub type StatusChangeModel = AccountStatusHistoryModel;

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



impl AccountModel {
    /// Set product code from string with validation
    pub fn set_product_code(&mut self, product_code: &str) -> Result<(), &'static str> {
        self.product_code = HeaplessString::try_from(product_code).map_err(|_| "Product code too long")?;
        Ok(())
    }
}