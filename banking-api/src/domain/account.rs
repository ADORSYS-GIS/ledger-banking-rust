use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_id: Uuid,
    #[serde(
        serialize_with = "serialize_product_code",
        deserialize_with = "deserialize_product_code"
    )]
    pub product_code: [u8; 12],
    pub account_type: AccountType,
    pub account_status: AccountStatus,
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
    pub collateral_id: Option<String>,
    pub loan_purpose: Option<String>,

    // Account lifecycle management (from enhancements)
    pub close_date: Option<NaiveDate>,
    pub last_activity_date: Option<NaiveDate>,
    pub dormancy_threshold_days: Option<i32>,
    pub reactivation_required: bool,
    pub pending_closure_reason: Option<String>,
    pub disbursement_instructions: Option<DisbursementInstructions>,
    
    // Enhanced audit trail
    pub status_changed_by: Option<String>,
    pub status_change_reason: Option<String>,
    pub status_change_timestamp: Option<DateTime<Utc>>,
    
    // Audit fields
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: HeaplessString<100>,
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
    pub method: DisbursementMethod,
    pub target_account: Option<Uuid>,
    pub cash_pickup_location: Option<String>,
    pub authorized_recipient: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisbursementMethod {
    Transfer,
    CashWithdrawal,
    Check,
    HoldFunds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHold {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: HoldType,
    pub reason: String,
    pub placed_by: String,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: HoldStatus,
    pub released_at: Option<DateTime<Utc>>,
    pub released_by: Option<String>,
    pub priority: HoldPriority,
    pub source_reference: Option<String>, // External reference for judicial holds, etc.
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
    pub release_reason: String,
    pub released_by: String,
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
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChangeRecord {
    pub change_id: Uuid,
    pub account_id: Uuid,
    pub old_status: Option<AccountStatus>,
    pub new_status: AccountStatus,
    pub reason: String,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
    pub system_triggered: bool,
}

/// Request parameters for placing a hold on an account
#[derive(Debug, Clone)]
pub struct PlaceHoldRequest {
    pub account_id: Uuid,
    pub hold_type: HoldType,
    pub amount: Decimal,
    pub reason: String,
    pub placed_by: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub priority: HoldPriority,
    pub source_reference: Option<String>,
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

impl Account {
    /// Convert product_code array to string for use in APIs
    pub fn product_code_as_str(&self) -> &str {
        let end = self.product_code.iter().position(|&b| b == 0).unwrap_or(12);
        std::str::from_utf8(&self.product_code[..end]).unwrap_or("")
    }
    
    /// Create Account with product_code from string
    pub fn set_product_code_from_str(&mut self, product_code: &str) -> Result<(), &'static str> {
        if product_code.len() > 12 {
            return Err("Product code too long");
        }
        if product_code.is_empty() {
            return Err("Product code cannot be empty");
        }
        
        let mut array = [0u8; 12];
        array[..product_code.len()].copy_from_slice(product_code.as_bytes());
        self.product_code = array;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_code_array_efficiency() {
        use std::mem;
        
        // Compare memory sizes between array and string
        let string_product = String::from("SAVP0001");
        let mut array_product = [0u8; 12];
        array_product[..8].copy_from_slice(b"SAVP0001");
        
        println!("String product code size: {} bytes", mem::size_of_val(&string_product));
        println!("Array product code size: {} bytes", mem::size_of_val(&array_product));
        
        // Fixed array should be much smaller and predictable
        assert!(mem::size_of_val(&array_product) < mem::size_of_val(&string_product));
        assert_eq!(mem::size_of_val(&array_product), 12); // Always exactly 12 bytes
        
        // Test conversion functions
        let mut account = Account {
            account_id: uuid::Uuid::new_v4(),
            product_code: array_product,
            account_type: AccountType::Savings,
            account_status: AccountStatus::Active,
            signing_condition: SigningCondition::None,
            currency: [b'U', b'S', b'D'],
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
            loan_purpose: None,
            close_date: None,
            last_activity_date: None,
            dormancy_threshold_days: None,
            reactivation_required: false,
            pending_closure_reason: None,
            disbursement_instructions: None,
            status_changed_by: None,
            status_change_reason: None,
            status_change_timestamp: None,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            updated_by: HeaplessString::try_from("test").unwrap(),
        };
        
        // Test string conversion
        assert_eq!(account.product_code_as_str(), "SAVP0001");
        
        // Test setting from string
        account.set_product_code_from_str("LNST0123").unwrap();
        assert_eq!(account.product_code_as_str(), "LNST0123");
    }

    #[test]
    fn test_currency_memory_efficiency() {
        use std::mem;
        
        // Compare memory sizes - our optimization goal
        let string_currency = String::from("USD");
        let array_currency: [u8; 3] = *b"USD";
        
        println!("String currency size: {} bytes", mem::size_of_val(&string_currency));
        println!("Array currency size: {} bytes", mem::size_of_val(&array_currency));
        
        // String has overhead (capacity, length, pointer), array is just 3 bytes
        assert!(mem::size_of_val(&array_currency) < mem::size_of_val(&string_currency));
        assert_eq!(mem::size_of_val(&array_currency), 3);
    }

    #[test]
    fn test_currency_validation_and_conversion() {
        // Test that our currency arrays work as expected
        let original_currency: [u8; 3] = *b"EUR";
        
        // Test currency validation
        let currency_str = std::str::from_utf8(&original_currency).unwrap();
        assert_eq!(currency_str.len(), 3);
        assert_eq!(currency_str, "EUR");
        assert!(currency_str.chars().all(|c| c.is_ascii_uppercase()));
        
        // Test various currency codes
        let currencies = [*b"USD", *b"GBP", *b"JPY", *b"CAD", *b"XAF"];
        for currency in currencies.iter() {
            let currency_str = std::str::from_utf8(currency).unwrap();
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
}