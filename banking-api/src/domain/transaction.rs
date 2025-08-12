use blake3::Hash;
use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_code: HeaplessString<8>,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub channel_id: HeaplessString<50>,
    pub terminal_id: Option<Uuid>,
    /// References Person.person_id
    pub agent_person_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub status: TransactionStatus,
    pub reference_number: HeaplessString<100>,
    pub external_reference: Option<HeaplessString<100>>,
    pub gl_code: HeaplessString<10>,
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

// Display implementations for database compatibility
impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Credit => write!(f, "Credit"),
            TransactionType::Debit => write!(f, "Debit"),
        }
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionStatus::Pending => write!(f, "Pending"),
            TransactionStatus::Posted => write!(f, "Posted"),
            TransactionStatus::Reversed => write!(f, "Reversed"),
            TransactionStatus::Failed => write!(f, "Failed"),
            TransactionStatus::AwaitingApproval => write!(f, "AwaitingApproval"),
            TransactionStatus::ApprovalRejected => write!(f, "ApprovalRejected"),
        }
    }
}

impl std::fmt::Display for TransactionApprovalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionApprovalStatus::Pending => write!(f, "Pending"),
            TransactionApprovalStatus::Approved => write!(f, "Approved"),
            TransactionApprovalStatus::Rejected => write!(f, "Rejected"),
            TransactionApprovalStatus::PartiallyApproved => write!(f, "PartiallyApproved"),
        }
    }
}

impl std::fmt::Display for TransactionWorkflowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionWorkflowStatus::Pending => write!(f, "Pending"),
            TransactionWorkflowStatus::Approved => write!(f, "Approved"),
            TransactionWorkflowStatus::Rejected => write!(f, "Rejected"),
            TransactionWorkflowStatus::TimedOut => write!(f, "TimedOut"),
        }
    }
}

impl std::fmt::Display for TransactionAuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionAuditAction::Created => write!(f, "Created"),
            TransactionAuditAction::StatusChanged => write!(f, "StatusChanged"),
            TransactionAuditAction::Posted => write!(f, "Posted"),
            TransactionAuditAction::Reversed => write!(f, "Reversed"),
            TransactionAuditAction::Failed => write!(f, "Failed"),
            TransactionAuditAction::Approved => write!(f, "Approved"),
            TransactionAuditAction::Rejected => write!(f, "Rejected"),
        }
    }
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelType::MobileApp => write!(f, "MobileApp"),
            ChannelType::AgentTerminal => write!(f, "AgentTerminal"),
            ChannelType::ATM => write!(f, "ATM"),
            ChannelType::InternetBanking => write!(f, "InternetBanking"),
            ChannelType::BranchTeller => write!(f, "BranchTeller"),
            ChannelType::USSD => write!(f, "USSD"),
            ChannelType::ApiGateway => write!(f, "ApiGateway"),
        }
    }
}

impl std::fmt::Display for PermittedOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermittedOperation::Credit => write!(f, "Credit"),
            PermittedOperation::Debit => write!(f, "Debit"),
            PermittedOperation::InterestPosting => write!(f, "InterestPosting"),
            PermittedOperation::FeeApplication => write!(f, "FeeApplication"),
            PermittedOperation::ClosureSettlement => write!(f, "ClosureSettlement"),
            PermittedOperation::None => write!(f, "None"),
        }
    }
}

// FromStr implementations for database compatibility
impl std::str::FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Credit" => Ok(TransactionType::Credit),
            "Debit" => Ok(TransactionType::Debit),
            _ => Err(format!("Invalid TransactionType: {s}")),
        }
    }
}

impl std::str::FromStr for TransactionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(TransactionStatus::Pending),
            "Posted" => Ok(TransactionStatus::Posted),
            "Reversed" => Ok(TransactionStatus::Reversed),
            "Failed" => Ok(TransactionStatus::Failed),
            "AwaitingApproval" => Ok(TransactionStatus::AwaitingApproval),
            "ApprovalRejected" => Ok(TransactionStatus::ApprovalRejected),
            _ => Err(format!("Invalid TransactionStatus: {s}")),
        }
    }
}

impl std::str::FromStr for TransactionApprovalStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(TransactionApprovalStatus::Pending),
            "Approved" => Ok(TransactionApprovalStatus::Approved),
            "Rejected" => Ok(TransactionApprovalStatus::Rejected),
            "PartiallyApproved" => Ok(TransactionApprovalStatus::PartiallyApproved),
            _ => Err(format!("Invalid TransactionApprovalStatus: {s}")),
        }
    }
}

impl std::str::FromStr for TransactionWorkflowStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(TransactionWorkflowStatus::Pending),
            "Approved" => Ok(TransactionWorkflowStatus::Approved),
            "Rejected" => Ok(TransactionWorkflowStatus::Rejected),
            "TimedOut" => Ok(TransactionWorkflowStatus::TimedOut),
            _ => Err(format!("Invalid TransactionWorkflowStatus: {s}")),
        }
    }
}

impl std::str::FromStr for TransactionAuditAction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Created" => Ok(TransactionAuditAction::Created),
            "StatusChanged" => Ok(TransactionAuditAction::StatusChanged),
            "Posted" => Ok(TransactionAuditAction::Posted),
            "Reversed" => Ok(TransactionAuditAction::Reversed),
            "Failed" => Ok(TransactionAuditAction::Failed),
            "Approved" => Ok(TransactionAuditAction::Approved),
            "Rejected" => Ok(TransactionAuditAction::Rejected),
            _ => Err(format!("Invalid TransactionAuditAction: {s}")),
        }
    }
}

impl std::str::FromStr for ChannelType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MobileApp" => Ok(ChannelType::MobileApp),
            "AgentTerminal" => Ok(ChannelType::AgentTerminal),
            "ATM" => Ok(ChannelType::ATM),
            "InternetBanking" => Ok(ChannelType::InternetBanking),
            "BranchTeller" => Ok(ChannelType::BranchTeller),
            "USSD" => Ok(ChannelType::USSD),
            "ApiGateway" => Ok(ChannelType::ApiGateway),
            _ => Err(format!("Invalid ChannelType: {s}")),
        }
    }
}

impl std::str::FromStr for PermittedOperation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Credit" => Ok(PermittedOperation::Credit),
            "Debit" => Ok(PermittedOperation::Debit),
            "InterestPosting" => Ok(PermittedOperation::InterestPosting),
            "FeeApplication" => Ok(PermittedOperation::FeeApplication),
            "ClosureSettlement" => Ok(PermittedOperation::ClosureSettlement),
            "None" => Ok(PermittedOperation::None),
            _ => Err(format!("Invalid PermittedOperation: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequest {
    pub id: Uuid,
    pub account_id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: Decimal,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub channel: ChannelType,
    pub terminal_id: Option<Uuid>,
    pub initiator_person_id: Uuid,
    pub external_reference: Option<HeaplessString<100>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRequestMetadata {
    pub id: Uuid,
    pub transaction_request_id: Uuid,
    pub key: HeaplessString<50>,
    pub value: HeaplessString<500>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub reference_number: HeaplessString<50>,
    pub timestamp: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionValidationResult {
    pub id: Uuid,
    pub is_valid: bool,
    pub transaction_id: Option<Uuid>,
    pub validation_error_01_field: Option<HeaplessString<50>>,
    pub validation_error_01_message: Option<HeaplessString<200>>,
    pub validation_error_01_error_code: Option<HeaplessString<50>>,
    pub validation_error_02_field: Option<HeaplessString<50>>,
    pub validation_error_02_message: Option<HeaplessString<200>>,
    pub validation_error_02_error_code: Option<HeaplessString<50>>,
    pub validation_error_03_field: Option<HeaplessString<50>>,
    pub validation_error_03_message: Option<HeaplessString<200>>,
    pub validation_error_03_error_code: Option<HeaplessString<50>>,
    pub warning_01: Option<HeaplessString<200>>,
    pub warning_02: Option<HeaplessString<200>>,
    pub warning_03: Option<HeaplessString<200>>,
    pub created_at: DateTime<Utc>,
}

impl TransactionValidationResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        is_valid: bool,
        transaction_id: Option<Uuid>,
        validation_error_01_field: Option<HeaplessString<50>>,
        validation_error_01_message: Option<HeaplessString<200>>,
        validation_error_01_error_code: Option<HeaplessString<50>>,
        validation_error_02_field: Option<HeaplessString<50>>,
        validation_error_02_message: Option<HeaplessString<200>>,
        validation_error_02_error_code: Option<HeaplessString<50>>,
        validation_error_03_field: Option<HeaplessString<50>>,
        validation_error_03_message: Option<HeaplessString<200>>,
        validation_error_03_error_code: Option<HeaplessString<50>>,
        warning_01: Option<HeaplessString<200>>,
        warning_02: Option<HeaplessString<200>>,
        warning_03: Option<HeaplessString<200>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            is_valid,
            transaction_id,
            validation_error_01_field,
            validation_error_01_message,
            validation_error_01_error_code,
            validation_error_02_field,
            validation_error_02_message,
            validation_error_02_error_code,
            validation_error_03_field,
            validation_error_03_message,
            validation_error_03_error_code,
            warning_01,
            warning_02,
            warning_03,
            created_at: Utc::now(),
        }
    }

    pub fn success(transaction_id: Option<Uuid>) -> Self {
        Self::new(
            true,
            transaction_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
    }

    pub fn failure(
        transaction_id: Option<Uuid>,
        errors: Vec<(
            Option<HeaplessString<50>>,
            Option<HeaplessString<200>>,
            Option<HeaplessString<50>>,
        )>,
    ) -> Self {
        let mut result = Self::new(
            false,
            transaction_id,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        if let Some(error) = errors.first() {
            result.validation_error_01_field = error.0.clone();
            result.validation_error_01_message = error.1.clone();
            result.validation_error_01_error_code = error.2.clone();
        }
        if let Some(error) = errors.get(1) {
            result.validation_error_02_field = error.0.clone();
            result.validation_error_02_message = error.1.clone();
            result.validation_error_02_error_code = error.2.clone();
        }
        if let Some(error) = errors.get(2) {
            result.validation_error_03_field = error.0.clone();
            result.validation_error_03_message = error.1.clone();
            result.validation_error_03_error_code = error.2.clone();
        }

        result
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn get_failure_reasons(&self) -> Vec<(String, String, String)> {
        let mut errors = Vec::new();
        if let (Some(field), Some(message), Some(code)) = (
            &self.validation_error_01_field,
            &self.validation_error_01_message,
            &self.validation_error_01_error_code,
        ) {
            errors.push((field.to_string(), message.to_string(), code.to_string()));
        }
        if let (Some(field), Some(message), Some(code)) = (
            &self.validation_error_02_field,
            &self.validation_error_02_message,
            &self.validation_error_02_error_code,
        ) {
            errors.push((field.to_string(), message.to_string(), code.to_string()));
        }
        if let (Some(field), Some(message), Some(code)) = (
            &self.validation_error_03_field,
            &self.validation_error_03_message,
            &self.validation_error_03_error_code,
        ) {
            errors.push((field.to_string(), message.to_string(), code.to_string()));
        }
        errors
    }

    pub fn add_check(
        &mut self,
        field: &str,
        is_valid: bool,
        message: String,
        error_code: Option<String>,
    ) {
        if !is_valid {
            self.is_valid = false;
            let field_hs = HeaplessString::try_from(field).ok();
            let message_hs = HeaplessString::try_from(message.as_str()).ok();
            let error_code_hs = error_code.and_then(|c| HeaplessString::try_from(c.as_str()).ok());

            if self.validation_error_01_field.is_none() {
                self.validation_error_01_field = field_hs;
                self.validation_error_01_message = message_hs;
                self.validation_error_01_error_code = error_code_hs;
            } else if self.validation_error_02_field.is_none() {
                self.validation_error_02_field = field_hs;
                self.validation_error_02_message = message_hs;
                self.validation_error_02_error_code = error_code_hs;
            } else if self.validation_error_03_field.is_none() {
                self.validation_error_03_field = field_hs;
                self.validation_error_03_message = message_hs;
                self.validation_error_03_error_code = error_code_hs;
            }
        } else {
            let warning_hs = HeaplessString::try_from(format!("{field}: {message}").as_str()).ok();
            if self.warning_01.is_none() {
                self.warning_01 = warning_hs;
            } else if self.warning_02.is_none() {
                self.warning_02 = warning_hs;
            } else if self.warning_03.is_none() {
                self.warning_03 = warning_hs;
            }
        }
    }

    pub fn merge(&mut self, other: &TransactionValidationResult) {
        self.is_valid = self.is_valid && other.is_valid;

        if other.validation_error_01_field.is_some() {
            self.add_check(
                other.validation_error_01_field.as_ref().unwrap(),
                false,
                other
                    .validation_error_01_message
                    .as_ref()
                    .unwrap()
                    .to_string(),
                other
                    .validation_error_01_error_code
                    .as_ref()
                    .map(|s| s.to_string()),
            );
        }
        if other.validation_error_02_field.is_some() {
            self.add_check(
                other.validation_error_02_field.as_ref().unwrap(),
                false,
                other
                    .validation_error_02_message
                    .as_ref()
                    .unwrap()
                    .to_string(),
                other
                    .validation_error_02_error_code
                    .as_ref()
                    .map(|s| s.to_string()),
            );
        }
        if other.validation_error_03_field.is_some() {
            self.add_check(
                other.validation_error_03_field.as_ref().unwrap(),
                false,
                other
                    .validation_error_03_message
                    .as_ref()
                    .unwrap()
                    .to_string(),
                other
                    .validation_error_03_error_code
                    .as_ref()
                    .map(|s| s.to_string()),
            );
        }

        if other.warning_01.is_some() {
            if self.warning_01.is_none() {
                self.warning_01 = other.warning_01.clone();
            } else if self.warning_02.is_none() {
                self.warning_02 = other.warning_01.clone();
            } else if self.warning_03.is_none() {
                self.warning_03 = other.warning_01.clone();
            }
        }
        if other.warning_02.is_some() {
            if self.warning_01.is_none() {
                self.warning_01 = other.warning_02.clone();
            } else if self.warning_02.is_none() {
                self.warning_02 = other.warning_02.clone();
            } else if self.warning_03.is_none() {
                self.warning_03 = other.warning_02.clone();
            }
        }
        if other.warning_03.is_some() {
            if self.warning_01.is_none() {
                self.warning_01 = other.warning_03.clone();
            } else if self.warning_02.is_none() {
                self.warning_02 = other.warning_03.clone();
            } else if self.warning_03.is_none() {
                self.warning_03 = other.warning_03.clone();
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlEntry {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub account_code: Uuid,
    pub debit_amount: Option<Decimal>,
    pub credit_amount: Option<Decimal>,
    pub currency: HeaplessString<3>,
    pub description: HeaplessString<200>,
    pub reference_number: HeaplessString<50>,
    pub value_date: NaiveDate,
    pub posting_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionApprovalWorkflow {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub status: TransactionWorkflowStatus,
    pub timeout_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionApproval {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub required: bool,
    pub approver_person_id: Uuid,
    pub approval_status: TransactionApprovalStatus,
    pub approved_at: DateTime<Utc>,
    pub notes: Option<HeaplessString<500>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionWorkflowStatus { 
    Pending, 
    Approved, 
    Rejected, 
    TimedOut 
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionAuditAction {
    Created,
    StatusChanged,
    Posted,
    Reversed,
    Failed,
    Approved,
    Rejected,
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
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub action_type: TransactionAuditAction,
    /// References Person.person_id
    pub performed_by_person_id: Uuid,
    pub performed_at: DateTime<Utc>,
    pub old_status: Option<TransactionStatus>,
    pub new_status: Option<TransactionStatus>,
    /// References ReasonAndPurpose.id for audit reason
    pub reason_id: Option<Uuid>,
    pub details: Option<Hash>,
}

impl TransactionAudit {
    /// Create new transaction audit with hash-based details
    pub fn new(
        transaction_id: Uuid,
        action_type: TransactionAuditAction,
        performed_by_person_id: Uuid,
        old_status: Option<TransactionStatus>,
        new_status: Option<TransactionStatus>,
        reason_id: Option<Uuid>,
        details_content: Option<&str>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            transaction_id,
            action_type,
            performed_by_person_id,
            performed_at: Utc::now(),
            old_status,
            new_status,
            reason_id,
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
    /// Set gl_code from string with validation
    pub fn set_gl_code(&mut self, gl_code: &str) -> Result<(), &'static str> {
        self.gl_code = HeaplessString::try_from(gl_code).map_err(|_| "GL code too long")?;
        Ok(())
    }
    
    /// Set transaction_code from string with validation
    pub fn set_transaction_code(&mut self, transaction_code: &str) -> Result<(), &'static str> {
        self.transaction_code = HeaplessString::try_from(transaction_code).map_err(|_| "Transaction code too long")?;
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
        let heapless_currency: HeaplessString<3> = HeaplessString::try_from("USD").unwrap();
        let total_fixed_size = 8 + 10 + mem::size_of_val(&heapless_currency); // txn_code + gl_code + currency
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