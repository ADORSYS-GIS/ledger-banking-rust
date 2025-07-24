use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use banking_api::domain::{AccountType, AccountStatus, SigningCondition};

/// Database model for Account table with enhanced fields from banking enhancements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountModel {
    pub account_id: Uuid,
    #[serde(
        serialize_with = "serialize_product_code",
        deserialize_with = "deserialize_product_code"
    )]
    pub product_code: [u8; 12],
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
    #[serde(
        serialize_with = "serialize_currency",
        deserialize_with = "deserialize_currency"
    )]
    pub currency: [u8; 3],
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
    pub collateral_id: Option<String>,
    pub loan_purpose: Option<String>,

    // Account lifecycle management (from enhancements)
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>,
    pub dormancy_threshold_days: Option<i32>,
    pub reactivation_required: bool,
    pub pending_closure_reason: Option<String>,
    
    // Enhanced audit trail
    pub status_changed_by: Option<String>,
    pub status_change_reason: Option<String>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Database model for Account Ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountOwnershipModel {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: String,
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
    pub entity_type: String,
    pub relationship_type: String,
    pub status: String,
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
    pub permission_type: String,
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    pub status: String,
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
    pub reason: String,
    pub placed_by: String,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
    pub released_by: Option<String>,
    pub released_at: Option<DateTime<Utc>>,
}

/// Database model for Account Status History (from enhancements)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct AccountStatusHistoryModel {
    pub history_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
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
    pub disbursement_method: String,
    pub disbursement_reference: Option<String>,
    pub processed_by: String,
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

// Currency serialization helpers for ISO 4217 compliance
fn serialize_currency<S>(currency: &[u8; 3], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let currency_str = std::str::from_utf8(currency)
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in currency code"))?;
    serializer.serialize_str(currency_str)
}

fn deserialize_currency<'de, D>(deserializer: D) -> Result<[u8; 3], D::Error>
where
    D: Deserializer<'de>,
{
    let currency_str = String::deserialize(deserializer)?;
    if currency_str.len() != 3 {
        return Err(serde::de::Error::custom(format!(
            "Currency code must be exactly 3 characters, got {}",
            currency_str.len()
        )));
    }
    
    let currency_bytes = currency_str.as_bytes();
    if !currency_bytes.iter().all(|&b| b.is_ascii_alphabetic() && b.is_ascii_uppercase()) {
        return Err(serde::de::Error::custom(
            "Currency code must contain only uppercase ASCII letters"
        ));
    }
    
    Ok([currency_bytes[0], currency_bytes[1], currency_bytes[2]])
}

// Product code serialization helpers for banking product codes (up to 12 chars)
fn serialize_product_code<S>(product_code: &[u8; 12], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Find the actual length by looking for null terminator or end
    let end = product_code.iter().position(|&b| b == 0).unwrap_or(12);
    let product_str = std::str::from_utf8(&product_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in product code"))?;
    serializer.serialize_str(product_str)
}

fn deserialize_product_code<'de, D>(deserializer: D) -> Result<[u8; 12], D::Error>
where
    D: Deserializer<'de>,
{
    let product_str = String::deserialize(deserializer)?;
    if product_str.len() > 12 {
        return Err(serde::de::Error::custom(format!(
            "Product code cannot exceed 12 characters, got {}",
            product_str.len()
        )));
    }
    
    if product_str.is_empty() {
        return Err(serde::de::Error::custom(
            "Product code cannot be empty"
        ));
    }
    
    let product_bytes = product_str.as_bytes();
    if !product_bytes.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-') {
        return Err(serde::de::Error::custom(
            "Product code must contain only alphanumeric characters, underscores, or hyphens"
        ));
    }
    
    let mut array = [0u8; 12];
    array[..product_bytes.len()].copy_from_slice(product_bytes);
    Ok(array)
}

impl AccountModel {
    /// Convert product_code array to string for use in APIs
    pub fn product_code_as_str(&self) -> &str {
        let end = self.product_code.iter().position(|&b| b == 0).unwrap_or(12);
        std::str::from_utf8(&self.product_code[..end]).unwrap_or("")
    }
}