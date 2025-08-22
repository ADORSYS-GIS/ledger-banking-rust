use chrono::{DateTime, Utc};
use heapless::{String as HeaplessString, Vec as HeaplessVec};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// ReasonAndPurpose domain model for banking operations
/// This provides a standardized way to handle reasons across all banking operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonAndPurpose {
    pub id: Uuid,
    
    /// Unique identifier code for programmatic reference
    pub code: HeaplessString<50>,  // e.g., "LOAN_PURPOSE_HOME_PURCHASE"
    
    /// Category to group related reasons
    pub category: ReasonCategory,
    
    /// Context where this reason is used
    pub context: ReasonContext,
    
    /// Language content - up to 3 languages supported
    pub l1_content: Option<HeaplessString<100>>,  // Primary language
    pub l2_content: Option<HeaplessString<100>>,  // Secondary language
    pub l3_content: Option<HeaplessString<100>>,  // Tertiary language
    
    /// Language codes for each content field
    pub l1_language_code: Option<[u8; 3]>,  // e.g., "eng"
    pub l2_language_code: Option<[u8; 3]>,  // e.g., "fra"
    pub l3_language_code: Option<[u8; 3]>,  // e.g., "swa"
    
    /// Whether this reason requires additional details
    pub requires_details: bool,
    
    /// Whether this reason is currently active
    pub is_active: bool,
    
    /// Severity or importance level
    pub severity: Option<ReasonSeverity>,
    
    /// Sort order for UI display
    pub display_order: i32,
    
    /// Compliance-specific metadata (for AML/CTF/KYC reasons)
    pub compliance_metadata: Option<ComplianceMetadata>,
    
    /// Audit fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by_person_id: Uuid,
    pub updated_by_person_id: Uuid,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReasonCategory {
    // Loan related
    LoanPurpose,
    LoanRejection,
    
    // Account lifecycle
    AccountClosure,
    AccountSuspension,
    AccountReactivation,
    StatusChange,
    
    // Transaction related
    TransactionRejection,
    TransactionReversal,
    HoldReason,
    
    // Compliance
    ComplianceFlag,
    AuditFinding,
    
    // AML/CTF Categories
    AmlAlert,
    AmlInvestigation,
    SuspiciousActivity,
    CtfRiskFlag,
    SanctionsHit,
    PepFlag,  // Politically Exposed Person
    HighRiskCountry,
    UnusualPattern,
    
    // KYC Categories
    KycMissingDocument,
    KycDocumentRejection,
    KycVerificationFailure,
    KycUpdateRequired,
    IdentityVerificationIssue,
    LocationVerificationIssue,
    SourceOfFundsRequired,
    
    // Customer service
    ComplaintReason,
    ServiceRequest,
    
    // System
    SystemGenerated,
    MaintenanceReason,
    
    // Other
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReasonContext {
    Account,
    Loan,
    Transaction,
    Customer,
    Compliance,
    AmlCtf,        // Anti-Money Laundering / Counter-Terrorism Financing
    Kyc,           // Know Your Customer
    System,
    General,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReasonSeverity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

/// Compliance-specific metadata for AML/CTF/KYC reasons
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetadata {
    /// Regulatory reference code (e.g., "FATF-R.16", "BSA-3.14")
    pub regulatory_code: Option<HeaplessString<20>>,
    
    /// Whether this reason requires regulatory reporting
    pub reportable: bool,
    
    /// Whether this triggers a Suspicious Activity Report
    pub requires_sar: bool,
    
    /// Whether this triggers a Currency Transaction Report
    pub requires_ctr: bool,
    
    /// Minimum retention period in years for audit
    pub retention_years: u8,
    
    /// Whether management escalation is required
    pub escalation_required: bool,
    
    /// Risk score impact (0-100)
    pub risk_score_impact: Option<u8>,
    
    /// Whether customer notification is prohibited (tipping off)
    pub no_tipping_off: bool,
    
    /// Relevant jurisdiction codes
    pub jurisdictions: HeaplessVec<[u8; 2], 5>,  // Country codes
}

/// Additional details that can be attached to a reason reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonReference {
    pub reason_id: Uuid,
    pub additional_details: Option<HeaplessString<200>>,
    pub referenced_at: DateTime<Utc>,
    pub referenced_by: HeaplessString<100>,
}

impl ReasonAndPurpose {
    /// Get content in specified language, fallback to primary if not available
    pub fn get_content(&self, language_code: &[u8; 3]) -> Option<&str> {
        if self.l1_language_code.as_ref() == Some(language_code) {
            self.l1_content.as_deref()
        } else if self.l2_language_code.as_ref() == Some(language_code) {
            self.l2_content.as_deref()  
        } else if self.l3_language_code.as_ref() == Some(language_code) {
            self.l3_content.as_deref()
        } else {
            // Fallback to primary language
            self.l1_content.as_deref()
        }
    }
    
    /// Get content with fallback chain
    pub fn get_content_with_fallback(&self, preferred_languages: &[[u8; 3]]) -> Option<&str> {
        for lang in preferred_languages {
            if let Some(content) = self.get_content(lang) {
                return Some(content);
            }
        }
        // Final fallback to any available content
        self.l1_content.as_deref()
            .or(self.l2_content.as_deref())
            .or(self.l3_content.as_deref())
    }
    
    /// Check if reason has content in specified language
    pub fn has_language(&self, language_code: &[u8; 3]) -> bool {
        self.l1_language_code.as_ref() == Some(language_code) ||
        self.l2_language_code.as_ref() == Some(language_code) ||
        self.l3_language_code.as_ref() == Some(language_code)
    }
    
    /// Get display text for UI purposes
    pub fn get_display_text(&self, preferred_languages: &[[u8; 3]]) -> String {
        self.get_content_with_fallback(preferred_languages)
            .unwrap_or(self.code.as_str())
            .to_string()
    }
    
    /// Check if this reason is compliance-related
    pub fn is_compliance_related(&self) -> bool {
        self.compliance_metadata.is_some()
    }
    
    /// Check if this reason requires regulatory reporting
    pub fn requires_reporting(&self) -> bool {
        self.compliance_metadata
            .as_ref()
            .map(|m| m.reportable)
            .unwrap_or(false)
    }
    
    /// Check if this reason triggers SAR
    pub fn triggers_sar(&self) -> bool {
        self.compliance_metadata
            .as_ref()
            .map(|m| m.requires_sar)
            .unwrap_or(false)
    }
    
    /// Check if this reason triggers CTR
    pub fn triggers_ctr(&self) -> bool {
        self.compliance_metadata
            .as_ref()
            .map(|m| m.requires_ctr)
            .unwrap_or(false)
    }
}

impl std::fmt::Display for ReasonCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasonCategory::LoanPurpose => write!(f, "LoanPurpose"),
            ReasonCategory::LoanRejection => write!(f, "LoanRejection"),
            ReasonCategory::AccountClosure => write!(f, "AccountClosure"),
            ReasonCategory::AccountSuspension => write!(f, "AccountSuspension"),
            ReasonCategory::AccountReactivation => write!(f, "AccountReactivation"),
            ReasonCategory::StatusChange => write!(f, "StatusChange"),
            ReasonCategory::TransactionRejection => write!(f, "TransactionRejection"),
            ReasonCategory::TransactionReversal => write!(f, "TransactionReversal"),
            ReasonCategory::HoldReason => write!(f, "HoldReason"),
            ReasonCategory::ComplianceFlag => write!(f, "ComplianceFlag"),
            ReasonCategory::AuditFinding => write!(f, "AuditFinding"),
            ReasonCategory::AmlAlert => write!(f, "AmlAlert"),
            ReasonCategory::AmlInvestigation => write!(f, "AmlInvestigation"),
            ReasonCategory::SuspiciousActivity => write!(f, "SuspiciousActivity"),
            ReasonCategory::CtfRiskFlag => write!(f, "CtfRiskFlag"),
            ReasonCategory::SanctionsHit => write!(f, "SanctionsHit"),
            ReasonCategory::PepFlag => write!(f, "PepFlag"),
            ReasonCategory::HighRiskCountry => write!(f, "HighRiskCountry"),
            ReasonCategory::UnusualPattern => write!(f, "UnusualPattern"),
            ReasonCategory::KycMissingDocument => write!(f, "KycMissingDocument"),
            ReasonCategory::KycDocumentRejection => write!(f, "KycDocumentRejection"),
            ReasonCategory::KycVerificationFailure => write!(f, "KycVerificationFailure"),
            ReasonCategory::KycUpdateRequired => write!(f, "KycUpdateRequired"),
            ReasonCategory::IdentityVerificationIssue => write!(f, "IdentityVerificationIssue"),
            ReasonCategory::LocationVerificationIssue => write!(f, "LocationVerificationIssue"),
            ReasonCategory::SourceOfFundsRequired => write!(f, "SourceOfFundsRequired"),
            ReasonCategory::ComplaintReason => write!(f, "ComplaintReason"),
            ReasonCategory::ServiceRequest => write!(f, "ServiceRequest"),
            ReasonCategory::SystemGenerated => write!(f, "SystemGenerated"),
            ReasonCategory::MaintenanceReason => write!(f, "MaintenanceReason"),
            ReasonCategory::Other => write!(f, "Other"),
        }
    }
}

impl std::fmt::Display for ReasonContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasonContext::Account => write!(f, "Account"),
            ReasonContext::Loan => write!(f, "Loan"),
            ReasonContext::Transaction => write!(f, "Transaction"),
            ReasonContext::Customer => write!(f, "Customer"),
            ReasonContext::Compliance => write!(f, "Compliance"),
            ReasonContext::AmlCtf => write!(f, "AmlCtf"),
            ReasonContext::Kyc => write!(f, "Kyc"),
            ReasonContext::System => write!(f, "System"),
            ReasonContext::General => write!(f, "General"),
        }
    }
}

impl std::fmt::Display for ReasonSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasonSeverity::Critical => write!(f, "Critical"),
            ReasonSeverity::High => write!(f, "High"),
            ReasonSeverity::Medium => write!(f, "Medium"),
            ReasonSeverity::Low => write!(f, "Low"),
            ReasonSeverity::Informational => write!(f, "Informational"),
        }
    }
}

impl std::str::FromStr for ReasonCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LoanPurpose" => Ok(ReasonCategory::LoanPurpose),
            "LoanRejection" => Ok(ReasonCategory::LoanRejection),
            "AccountClosure" => Ok(ReasonCategory::AccountClosure),
            "AccountSuspension" => Ok(ReasonCategory::AccountSuspension),
            "AccountReactivation" => Ok(ReasonCategory::AccountReactivation),
            "StatusChange" => Ok(ReasonCategory::StatusChange),
            "TransactionRejection" => Ok(ReasonCategory::TransactionRejection),
            "TransactionReversal" => Ok(ReasonCategory::TransactionReversal),
            "HoldReason" => Ok(ReasonCategory::HoldReason),
            "ComplianceFlag" => Ok(ReasonCategory::ComplianceFlag),
            "AuditFinding" => Ok(ReasonCategory::AuditFinding),
            "AmlAlert" => Ok(ReasonCategory::AmlAlert),
            "AmlInvestigation" => Ok(ReasonCategory::AmlInvestigation),
            "SuspiciousActivity" => Ok(ReasonCategory::SuspiciousActivity),
            "CtfRiskFlag" => Ok(ReasonCategory::CtfRiskFlag),
            "SanctionsHit" => Ok(ReasonCategory::SanctionsHit),
            "PepFlag" => Ok(ReasonCategory::PepFlag),
            "HighRiskCountry" => Ok(ReasonCategory::HighRiskCountry),
            "UnusualPattern" => Ok(ReasonCategory::UnusualPattern),
            "KycMissingDocument" => Ok(ReasonCategory::KycMissingDocument),
            "KycDocumentRejection" => Ok(ReasonCategory::KycDocumentRejection),
            "KycVerificationFailure" => Ok(ReasonCategory::KycVerificationFailure),
            "KycUpdateRequired" => Ok(ReasonCategory::KycUpdateRequired),
            "IdentityVerificationIssue" => Ok(ReasonCategory::IdentityVerificationIssue),
            "LocationVerificationIssue" => Ok(ReasonCategory::LocationVerificationIssue),
            "SourceOfFundsRequired" => Ok(ReasonCategory::SourceOfFundsRequired),
            "ComplaintReason" => Ok(ReasonCategory::ComplaintReason),
            "ServiceRequest" => Ok(ReasonCategory::ServiceRequest),
            "SystemGenerated" => Ok(ReasonCategory::SystemGenerated),
            "MaintenanceReason" => Ok(ReasonCategory::MaintenanceReason),
            "Other" => Ok(ReasonCategory::Other),
            _ => Err(format!("Invalid reason category: {s}")),
        }
    }
}

impl std::str::FromStr for ReasonContext {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Account" => Ok(ReasonContext::Account),
            "Loan" => Ok(ReasonContext::Loan),
            "Transaction" => Ok(ReasonContext::Transaction),
            "Customer" => Ok(ReasonContext::Customer),
            "Compliance" => Ok(ReasonContext::Compliance),
            "AmlCtf" => Ok(ReasonContext::AmlCtf),
            "Kyc" => Ok(ReasonContext::Kyc),
            "System" => Ok(ReasonContext::System),
            "General" => Ok(ReasonContext::General),
            _ => Err(format!("Invalid reason context: {s}")),
        }
    }
}

impl std::str::FromStr for ReasonSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Critical" => Ok(ReasonSeverity::Critical),
            "High" => Ok(ReasonSeverity::High),
            "Medium" => Ok(ReasonSeverity::Medium),
            "Low" => Ok(ReasonSeverity::Low),
            "Informational" => Ok(ReasonSeverity::Informational),
            _ => Err(format!("Invalid reason severity: {s}")),
        }
    }
}