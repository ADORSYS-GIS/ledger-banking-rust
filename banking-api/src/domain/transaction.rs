use blake3::Hash;
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub account_id: Uuid,
    #[serde(
        serialize_with = "serialize_transaction_code",
        deserialize_with = "deserialize_transaction_code"
    )]
    pub transaction_code: [u8; 8],
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    #[serde(
        serialize_with = "serialize_currency",
        deserialize_with = "deserialize_currency"
    )]
    pub currency: [u8; 3],
    pub description: HeaplessString<500>,
    pub channel_id: HeaplessString<50>,
    pub terminal_id: Option<Uuid>,
    pub agent_user_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub status: TransactionStatus,
    pub reference_number: HeaplessString<100>,
    pub external_reference: Option<HeaplessString<100>>,
    #[serde(
        serialize_with = "serialize_gl_code",
        deserialize_with = "deserialize_gl_code"
    )]
    pub gl_code: [u8; 10],
    pub requires_approval: bool,
    pub approval_status: Option<TransactionApprovalStatus>,
    pub risk_score: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType { 
    Credit, 
    Debit 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus { 
    Pending, 
    Posted, 
    Reversed, 
    Failed,
    AwaitingApproval,
    ApprovalRejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionApprovalStatus { 
    Pending, 
    Approved, 
    Rejected, 
    PartiallyApproved 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub account_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    #[serde(
        serialize_with = "serialize_currency",
        deserialize_with = "deserialize_currency"
    )]
    pub currency: [u8; 3],
    pub description: HeaplessString<500>,
    pub channel: ChannelType,
    pub terminal_id: Option<Uuid>,
    pub initiator_id: HeaplessString<100>,
    pub external_reference: Option<HeaplessString<100>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub transaction_id: Uuid,
    pub reference_number: String,
    pub gl_entries: Vec<GlEntry>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new(is_valid: bool, errors: Vec<String>, warnings: Vec<String>) -> Self {
        Self { is_valid, errors, warnings }
    }
    
    pub fn success() -> Self {
        Self::new(true, vec![], vec![])
    }
    
    pub fn failure(errors: Vec<String>) -> Self {
        Self::new(false, errors, vec![])
    }
    
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    
    pub fn get_failure_reasons(&self) -> Vec<String> {
        self.errors.clone()
    }
    
    pub fn add_check(&mut self, field: &str, is_valid: bool, message: String) {
        if !is_valid {
            self.errors.push(format!("{field}: {message}"));
            self.is_valid = false;
        } else {
            self.warnings.push(format!("{field}: {message}"));
        }
    }
    
    pub fn merge(&mut self, other: ValidationResult) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.is_valid = self.is_valid && other.is_valid;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlEntry {
    pub entry_id: Uuid,
    pub account_code: String,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    #[serde(
        serialize_with = "serialize_currency",
        deserialize_with = "deserialize_currency"
    )]
    pub currency: [u8; 3],
    pub description: String,
    pub reference_number: String,
    pub transaction_id: Uuid,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    pub workflow_id: Uuid,
    pub transaction_id: Uuid,
    pub required_approvers: Vec<Uuid>,
    pub received_approvals: Vec<Approval>,
    pub status: TransactionWorkflowStatus,
    pub timeout_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    pub approval_id: Uuid,
    pub approver_id: Uuid,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionWorkflowStatus { 
    Pending, 
    Approved, 
    Rejected, 
    TimedOut 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelType {
    MobileApp,
    AgentTerminal,
    ATM,
    InternetBanking,
    BranchTeller,
    USSD,
    ApiGateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermittedOperation {
    Credit,
    Debit,
    InterestPosting,
    FeeApplication,
    ClosureSettlement,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAudit {
    pub audit_id: Uuid,
    pub transaction_id: Uuid,
    pub action_type: String,
    pub performed_by: String,
    pub performed_at: DateTime<Utc>,
    pub old_status: Option<String>,
    pub new_status: Option<String>,
    pub reason: Option<String>,
    pub details: Option<Hash>,
}

impl TransactionAudit {
    /// Create new transaction audit with hash-based details
    pub fn new(
        transaction_id: Uuid,
        action_type: String,
        performed_by: String,
        old_status: Option<String>,
        new_status: Option<String>,
        reason: Option<String>,
        details_content: Option<&str>,
    ) -> Self {
        Self {
            audit_id: Uuid::new_v4(),
            transaction_id,
            action_type,
            performed_by,
            performed_at: Utc::now(),
            old_status,
            new_status,
            reason,
            details: details_content.map(|content| blake3::hash(content.as_bytes())),
        }
    }
    
    /// Get details hash as hex string for display/logging
    pub fn details_hex(&self) -> Option<String> {
        self.details.map(|hash| hash.to_hex().to_string())
    }
    
    /// Create hash from details content for verification
    pub fn hash_from_details(details_content: &str) -> Hash {
        blake3::hash(details_content.as_bytes())
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

// Transaction code serialization helpers for banking transaction codes (up to 8 chars)
fn serialize_transaction_code<S>(transaction_code: &[u8; 8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let end = transaction_code.iter().position(|&b| b == 0).unwrap_or(8);
    let code_str = std::str::from_utf8(&transaction_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in transaction code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_transaction_code<'de, D>(deserializer: D) -> Result<[u8; 8], D::Error>
where
    D: Deserializer<'de>,
{
    let code_str = String::deserialize(deserializer)?;
    if code_str.len() > 8 {
        return Err(serde::de::Error::custom(format!(
            "Transaction code cannot exceed 8 characters, got {}",
            code_str.len()
        )));
    }
    
    if code_str.is_empty() {
        return Err(serde::de::Error::custom(
            "Transaction code cannot be empty"
        ));
    }
    
    let code_bytes = code_str.as_bytes();
    if !code_bytes.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_') {
        return Err(serde::de::Error::custom(
            "Transaction code must contain only alphanumeric characters or underscores"
        ));
    }
    
    let mut array = [0u8; 8];
    array[..code_bytes.len()].copy_from_slice(code_bytes);
    Ok(array)
}

// GL code serialization helpers for banking GL codes (up to 10 chars)
fn serialize_gl_code<S>(gl_code: &[u8; 10], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let end = gl_code.iter().position(|&b| b == 0).unwrap_or(10);
    let code_str = std::str::from_utf8(&gl_code[..end])
        .map_err(|_| serde::ser::Error::custom("Invalid UTF-8 in GL code"))?;
    serializer.serialize_str(code_str)
}

fn deserialize_gl_code<'de, D>(deserializer: D) -> Result<[u8; 10], D::Error>
where
    D: Deserializer<'de>,
{
    let code_str = String::deserialize(deserializer)?;
    if code_str.len() > 10 {
        return Err(serde::de::Error::custom(format!(
            "GL code cannot exceed 10 characters, got {}",
            code_str.len()
        )));
    }
    
    if code_str.is_empty() {
        return Err(serde::de::Error::custom(
            "GL code cannot be empty"
        ));
    }
    
    let code_bytes = code_str.as_bytes();
    if !code_bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
        return Err(serde::de::Error::custom(
            "GL code must contain only alphanumeric characters"
        ));
    }
    
    let mut array = [0u8; 10];
    array[..code_bytes.len()].copy_from_slice(code_bytes);
    Ok(array)
}

impl Transaction {
    /// Convert description to standard String for use in formatting
    pub fn description_as_string(&self) -> String {
        self.description.to_string()
    }
    
    /// Set description from String with length validation
    pub fn set_description(&mut self, desc: &str) -> Result<(), &'static str> {
        self.description = HeaplessString::try_from(desc).map_err(|_| "Description too long")?;
        Ok(())
    }
    
    /// Set reference number from String with length validation
    pub fn set_reference_number(&mut self, ref_num: &str) -> Result<(), &'static str> {
        self.reference_number = HeaplessString::try_from(ref_num).map_err(|_| "Reference number too long")?;
        Ok(())
    }
    
    /// Set channel ID from String with length validation
    pub fn set_channel_id(&mut self, channel: &str) -> Result<(), &'static str> {
        self.channel_id = HeaplessString::try_from(channel).map_err(|_| "Channel ID too long")?;
        Ok(())
    }

    /// Convert reference_number to standard String
    pub fn reference_number_as_string(&self) -> String {
        self.reference_number.to_string()
    }

    /// Convert channel_id to standard String  
    pub fn channel_id_as_string(&self) -> String {
        self.channel_id.to_string()
    }
    /// Convert transaction_code array to string for use in APIs
    pub fn transaction_code_as_str(&self) -> &str {
        let end = self.transaction_code.iter().position(|&b| b == 0).unwrap_or(8);
        std::str::from_utf8(&self.transaction_code[..end]).unwrap_or("")
    }
    
    /// Convert gl_code array to string for use in APIs
    pub fn gl_code_as_str(&self) -> &str {
        let end = self.gl_code.iter().position(|&b| b == 0).unwrap_or(10);
        std::str::from_utf8(&self.gl_code[..end]).unwrap_or("")
    }
    
    /// Set gl_code from string
    pub fn set_gl_code_from_str(&mut self, gl_code: &str) -> Result<(), &'static str> {
        if gl_code.len() > 10 {
            return Err("GL code too long");
        }
        if gl_code.is_empty() {
            return Err("GL code cannot be empty");
        }
        
        let mut array = [0u8; 10];
        array[..gl_code.len()].copy_from_slice(gl_code.as_bytes());
        self.gl_code = array;
        Ok(())
    }
    
    /// Set transaction_code from string
    pub fn set_transaction_code_from_str(&mut self, transaction_code: &str) -> Result<(), &'static str> {
        if transaction_code.len() > 8 {
            return Err("Transaction code too long");
        }
        if transaction_code.is_empty() {
            return Err("Transaction code cannot be empty");
        }
        
        let mut array = [0u8; 8];
        array[..transaction_code.len()].copy_from_slice(transaction_code.as_bytes());
        self.transaction_code = array;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_fixed_array_codes_efficiency() {
        use std::mem;
        
        // Compare memory sizes for transaction_code
        let string_txn_code = String::from("DEBIT01");
        let array_txn_code = [b'D', b'E', b'B', b'I', b'T', b'0', b'1', 0];
        
        println!("String transaction code size: {} bytes", mem::size_of_val(&string_txn_code));
        println!("Array transaction code size: {} bytes", mem::size_of_val(&array_txn_code));
        
        // Fixed array should be much smaller and predictable
        assert!(mem::size_of_val(&array_txn_code) < mem::size_of_val(&string_txn_code));
        assert_eq!(mem::size_of_val(&array_txn_code), 8);
        
        // Compare memory sizes for gl_code  
        let string_gl_code = String::from("GL4010001");
        let array_gl_code = [b'G', b'L', b'4', b'0', b'1', b'0', b'0', b'0', b'1', 0];
        
        println!("String GL code size: {} bytes", mem::size_of_val(&string_gl_code));
        println!("Array GL code size: {} bytes", mem::size_of_val(&array_gl_code));
        
        // Fixed array should be much smaller and predictable
        assert!(mem::size_of_val(&array_gl_code) < mem::size_of_val(&string_gl_code));
        assert_eq!(mem::size_of_val(&array_gl_code), 10);
        
        // Test memory efficiency vs String heap allocation
        let total_fixed_size = 8 + 10 + 3; // txn_code + gl_code + currency
        let total_string_size = mem::size_of_val(&string_txn_code) + 
                               mem::size_of_val(&string_gl_code) +
                               mem::size_of_val(&String::from("USD"));
        
        println!("Total fixed arrays size: {} bytes", total_fixed_size);
        println!("Total String fields size: {} bytes", total_string_size);
        
        // Should see significant memory reduction (50%+ savings)
        assert!(total_fixed_size < total_string_size);
        let savings_percent = ((total_string_size - total_fixed_size) as f64 / total_string_size as f64) * 100.0;
        println!("Memory savings: {:.1}%", savings_percent);
        assert!(savings_percent > 50.0);
    }

    #[test]
    fn test_transaction_enum_memory_efficiency() {
        // Compare memory sizes between enum and string
        let string_status = String::from("AwaitingApproval");
        let enum_status = TransactionStatus::AwaitingApproval;
        
        println!("String transaction status size: {} bytes", mem::size_of_val(&string_status));
        println!("Enum transaction status size: {} bytes", mem::size_of_val(&enum_status));
        
        // Enum should be much smaller than String
        assert!(mem::size_of_val(&enum_status) < mem::size_of_val(&string_status));
        assert!(mem::size_of_val(&enum_status) <= 8); // Typically 1-8 bytes for enums
    }

    #[test]
    fn test_transaction_enum_type_safety() {
        // Test that enums provide compile-time type safety
        let transaction_type = TransactionType::Credit;
        let transaction_status = TransactionStatus::Pending;
        let approval_status = TransactionApprovalStatus::Approved;
        
        // These would be caught at compile time, not runtime
        assert_eq!(transaction_type, TransactionType::Credit);
        assert_eq!(transaction_status, TransactionStatus::Pending);
        assert_eq!(approval_status, TransactionApprovalStatus::Approved);
    }
}