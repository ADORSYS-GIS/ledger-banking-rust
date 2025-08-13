# Reason and Purpose Multilingual Refactoring Instructions

## Overview
Create a centralized `ReasonAndPurpose` struct to handle all reasons and purposes across the system with multilingual support. Replace all string-based reason/purpose fields with UUID references to enable consistent messaging and translation management.

## Step 1: Create the ReasonAndPurpose Struct

Create a new file `src/models/reason_and_purpose.rs`:

```rust
use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub created_by_person_id: HeaplessString<100>,
    pub updated_by_person_id: HeaplessString<100>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
    AddressVerificationIssue,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
}

/// Repository for managing reasons
pub struct ReasonAndPurposeRepository {
    reasons: std::collections::HashMap<Uuid, ReasonAndPurpose>,
    by_code: std::collections::HashMap<HeaplessString<50>, Uuid>,
    by_category: std::collections::HashMap<ReasonCategory, Vec<Uuid>>,
}

impl ReasonAndPurposeRepository {
    pub fn get(&self, id: &Uuid) -> Option<&ReasonAndPurpose> {
        self.reasons.get(id)
    }
    
    pub fn get_by_code(&self, code: &str) -> Option<&ReasonAndPurpose> {
        self.by_code.get(code)
            .and_then(|id| self.reasons.get(id))
    }
    
    pub fn get_by_category(&self, category: ReasonCategory) -> Vec<&ReasonAndPurpose> {
        self.by_category.get(&category)
            .map(|ids| ids.iter()
                .filter_map(|id| self.reasons.get(id))
                .filter(|r| r.is_active)
                .collect())
            .unwrap_or_default()
    }
    
    pub fn get_for_context(&self, context: ReasonContext, category: ReasonCategory) -> Vec<&ReasonAndPurpose> {
        self.get_by_category(category)
            .into_iter()
            .filter(|r| r.context == context)
            .collect()
    }
    
    /// Get all compliance-related reasons that require reporting
    pub fn get_reportable_compliance_reasons(&self) -> Vec<&ReasonAndPurpose> {
        self.reasons.values()
            .filter(|r| {
                r.compliance_metadata.as_ref()
                    .map(|m| m.reportable)
                    .unwrap_or(false)
            })
            .collect()
    }
    
    /// Get reasons that trigger SAR (Suspicious Activity Report)
    pub fn get_sar_triggering_reasons(&self) -> Vec<&ReasonAndPurpose> {
        self.reasons.values()
            .filter(|r| {
                r.compliance_metadata.as_ref()
                    .map(|m| m.requires_sar)
                    .unwrap_or(false)
            })
            .collect()
    }
}
```

## Step 2: Create Predefined Common Reasons

Create `src/models/reason_and_purpose_seeds.rs`:

```rust
/// Common reason codes that should be seeded in the database
pub struct ReasonSeeds;

impl ReasonSeeds {
    pub fn get_initial_reasons() -> Vec<ReasonAndPurpose> {
        vec![
            // Loan Purposes
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("LOAN_PURPOSE_HOME_PURCHASE").unwrap(),
                category: ReasonCategory::LoanPurpose,
                context: ReasonContext::Loan,
                l1_content: Some(HeaplessString::try_from("Home Purchase").unwrap()),
                l2_content: Some(HeaplessString::try_from("Achat de Maison").unwrap()),
                l3_content: Some(HeaplessString::try_from("Kununua Nyumba").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: false,
                is_active: true,
                severity: None,
                display_order: 1,
                compliance_metadata: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            // Account Closure Reasons
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("CLOSURE_CUSTOMER_REQUEST").unwrap(),
                category: ReasonCategory::AccountClosure,
                context: ReasonContext::Account,
                l1_content: Some(HeaplessString::try_from("Customer Request").unwrap()),
                l2_content: Some(HeaplessString::try_from("Demande du Client").unwrap()),
                l3_content: Some(HeaplessString::try_from("Ombi la Mteja").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: true,
                is_active: true,
                severity: Some(ReasonSeverity::Low),
                display_order: 1,
                compliance_metadata: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            // AML/CTF Reasons
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("AML_SUSPICIOUS_PATTERN").unwrap(),
                category: ReasonCategory::AmlAlert,
                context: ReasonContext::AmlCtf,
                l1_content: Some(HeaplessString::try_from("Suspicious transaction pattern detected").unwrap()),
                l2_content: Some(HeaplessString::try_from("Modèle de transaction suspect détecté").unwrap()),
                l3_content: Some(HeaplessString::try_from("Mfumo wa shughuli za kutiliwa shaka umegundulika").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: true,
                is_active: true,
                severity: Some(ReasonSeverity::High),
                display_order: 1,
                compliance_metadata: Some(ComplianceMetadata {
                    regulatory_code: Some(HeaplessString::try_from("FATF-R.16").unwrap()),
                    reportable: true,
                    requires_sar: true,
                    requires_ctr: false,
                    retention_years: 7,
                    escalation_required: true,
                    risk_score_impact: Some(25),
                    no_tipping_off: true,
                    jurisdictions: {
                        let mut vec = HeaplessVec::new();
                        vec.push(*b"CM").unwrap();
                        vec
                    },
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("CTF_SANCTIONS_MATCH").unwrap(),
                category: ReasonCategory::SanctionsHit,
                context: ReasonContext::AmlCtf,
                l1_content: Some(HeaplessString::try_from("Name matches sanctions list").unwrap()),
                l2_content: Some(HeaplessString::try_from("Le nom correspond à la liste des sanctions").unwrap()),
                l3_content: Some(HeaplessString::try_from("Jina linafanana na orodha ya vikwazo").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: true,
                is_active: true,
                severity: Some(ReasonSeverity::Critical),
                display_order: 1,
                compliance_metadata: Some(ComplianceMetadata {
                    regulatory_code: Some(HeaplessString::try_from("UN-RES-1267").unwrap()),
                    reportable: true,
                    requires_sar: true,
                    requires_ctr: false,
                    retention_years: 10,
                    escalation_required: true,
                    risk_score_impact: Some(100),
                    no_tipping_off: true,
                    jurisdictions: {
                        let mut vec = HeaplessVec::new();
                        vec.push(*b"CM").unwrap();
                        vec.push(*b"US").unwrap();
                        vec.push(*b"EU").unwrap();
                        vec
                    },
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("AML_HIGH_RISK_COUNTRY").unwrap(),
                category: ReasonCategory::HighRiskCountry,
                context: ReasonContext::AmlCtf,
                l1_content: Some(HeaplessString::try_from("Transaction involves high-risk jurisdiction").unwrap()),
                l2_content: Some(HeaplessString::try_from("Transaction implique une juridiction à haut risque").unwrap()),
                l3_content: Some(HeaplessString::try_from("Muamala unahusisha eneo lenye hatari kubwa").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: true,
                is_active: true,
                severity: Some(ReasonSeverity::High),
                display_order: 2,
                compliance_metadata: Some(ComplianceMetadata {
                    regulatory_code: Some(HeaplessString::try_from("FATF-HIGH-RISK").unwrap()),
                    reportable: true,
                    requires_sar: false,
                    requires_ctr: false,
                    retention_years: 7,
                    escalation_required: false,
                    risk_score_impact: Some(40),
                    no_tipping_off: false,
                    jurisdictions: HeaplessVec::new(),
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            // KYC Reasons
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("KYC_ID_EXPIRED").unwrap(),
                category: ReasonCategory::KycMissingDocument,
                context: ReasonContext::Kyc,
                l1_content: Some(HeaplessString::try_from("National ID has expired").unwrap()),
                l2_content: Some(HeaplessString::try_from("La carte d'identité nationale a expiré").unwrap()),
                l3_content: Some(HeaplessString::try_from("Kitambulisho cha taifa kimepita muda").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: false,
                is_active: true,
                severity: Some(ReasonSeverity::Medium),
                display_order: 1,
                compliance_metadata: Some(ComplianceMetadata {
                    regulatory_code: Some(HeaplessString::try_from("KYC-DOC-01").unwrap()),
                    reportable: false,
                    requires_sar: false,
                    requires_ctr: false,
                    retention_years: 5,
                    escalation_required: false,
                    risk_score_impact: Some(10),
                    no_tipping_off: false,
                    jurisdictions: HeaplessVec::new(),
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("KYC_PROOF_OF_ADDRESS_MISSING").unwrap(),
                category: ReasonCategory::KycMissingDocument,
                context: ReasonContext::Kyc,
                l1_content: Some(HeaplessString::try_from("Proof of address document required").unwrap()),
                l2_content: Some(HeaplessString::try_from("Justificatif de domicile requis").unwrap()),
                l3_content: Some(HeaplessString::try_from("Uthibitisho wa makazi unahitajika").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: false,
                is_active: true,
                severity: Some(ReasonSeverity::Medium),
                display_order: 2,
                compliance_metadata: Some(ComplianceMetadata {
                    regulatory_code: Some(HeaplessString::try_from("KYC-DOC-02").unwrap()),
                    reportable: false,
                    requires_sar: false,
                    requires_ctr: false,
                    retention_years: 5,
                    escalation_required: false,
                    risk_score_impact: Some(10),
                    no_tipping_off: false,
                    jurisdictions: HeaplessVec::new(),
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            ReasonAndPurpose {
                id: Uuid::new_v4(),
                code: HeaplessString::try_from("KYC_SOURCE_OF_FUNDS_UNCLEAR").unwrap(),
                category: ReasonCategory::SourceOfFundsRequired,
                context: ReasonContext::Kyc,
                l1_content: Some(HeaplessString::try_from("Source of funds documentation required").unwrap()),
                l2_content: Some(HeaplessString::try_from("Documentation sur l'origine des fonds requise").unwrap()),
                l3_content: Some(HeaplessString::try_from("Hati za chanzo cha fedha zinahitajika").unwrap()),
                l1_language_code: Some(*b"eng"),
                l2_language_code: Some(*b"fra"),
                l3_language_code: Some(*b"swa"),
                requires_details: true,
                is_active: true,
                severity: Some(ReasonSeverity::High),
                display_order: 1,
                compliance_metadata: Some(ComplianceMetadata {
                    regulatory_code: Some(HeaplessString::try_from("KYC-SOF-01").unwrap()),
                    reportable: true,
                    requires_sar: false,
                    requires_ctr: false,
                    retention_years: 7,
                    escalation_required: true,
                    risk_score_impact: Some(30),
                    no_tipping_off: false,
                    jurisdictions: HeaplessVec::new(),
                }),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                created_by_person_id: HeaplessString::try_from("System").unwrap(),
                updated_by_person_id: HeaplessString::try_from("System").unwrap(),
            },
            
            // Add more predefined reasons...
        ]
    }
}
```

## Step 3: Find and Replace All Reason/Purpose Fields

### Search Patterns
Look for these patterns across the entire workspace:

```rust
// Direct string fields
reason: String
reason: HeaplessString<N>
reason: Option<String>
reason: Option<HeaplessString<N>>

purpose: String
purpose: HeaplessString<N>
purpose: Option<String>
purpose: Option<HeaplessString<N>>

// Common field name patterns
*_reason: Option<HeaplessString<N>>
*_purpose: Option<HeaplessString<N>>
closure_reason
rejection_reason
status_change_reason
loan_purpose
hold_reason
reversal_reason
complaint_reason
```

### Replace With

```rust
// For required reasons
pub [field_name]_reason_id: Uuid,

// For optional reasons
pub [field_name]_reason_id: Option<Uuid>,

// Add comment for clarity
/// References ReasonAndPurpose.id
pub [field_name]_reason_id: Option<Uuid>,
```

### Specific Examples

```rust
// === BEFORE (in Account struct) ===
pub loan_purpose: Option<HeaplessString<100>>,
pub pending_closure_reason: Option<HeaplessString<100>>,
pub status_change_reason: Option<HeaplessString<100>>,

// === AFTER ===
/// References ReasonAndPurpose.id for loan purpose
pub loan_purpose_id: Option<Uuid>,
/// References ReasonAndPurpose.id for pending closure
pub pending_closure_reason_id: Option<Uuid>,
/// References ReasonAndPurpose.id for status change
pub status_change_reason_id: Option<Uuid>,

// === BEFORE (in AccountHold struct) ===
pub reason: HeaplessString<100>,

// === AFTER ===
/// References ReasonAndPurpose.id - required field
pub reason_id: Uuid,
/// Additional context beyond the standard reason
pub additional_details: Option<HeaplessString<200>>,

// === BEFORE (in StatusChangeRecord) ===
pub reason: HeaplessString<100>,

// === AFTER ===
/// References ReasonAndPurpose.id
pub reason_id: Uuid,
pub additional_context: Option<HeaplessString<200>>,
```

## Step 4: Update Method Signatures

Find and update all methods that accept reason/purpose strings:

```rust
// === BEFORE ===
impl Account {
    pub fn close_account(&mut self, reason: &str, closed_by: &str) {
        self.pending_closure_reason = Some(HeaplessString::try_from(reason).unwrap());
        // ...
    }
}

// === AFTER ===
impl Account {
    pub fn close_account(&mut self, reason_id: Uuid, closed_by: &str, additional_details: Option<&str>) {
        self.pending_closure_reason_id = Some(reason_id);
        // Store additional_details if needed in a separate field
        // ...
    }
}

// === BEFORE ===
pub struct PlaceHoldRequest {
    pub reason: HeaplessString<100>,
    // ...
}

// === AFTER ===
pub struct PlaceHoldRequest {
    pub reason_id: Uuid,
    pub additional_details: Option<HeaplessString<200>>,
    // ...
}
```

## Step 5: Create View Models with Resolved Reasons

```rust
/// View model that includes resolved reason text
#[derive(Serialize, Deserialize)]
pub struct AccountView {
    pub account_id: Uuid,
    pub loan_purpose: Option<ReasonView>,
    pub pending_closure_reason: Option<ReasonView>,
    // ... other fields
}

#[derive(Serialize, Deserialize)]
pub struct ReasonView {
    pub id: Uuid,
    pub code: String,
    pub text: String,  // Resolved based on user's language preference
    pub requires_details: bool,
    pub additional_details: Option<String>,
}

impl ReasonView {
    pub fn from_reason(
        reason: &ReasonAndPurpose, 
        language_code: &[u8; 3],
        additional_details: Option<&str>
    ) -> Self {
        Self {
            id: reason.id,
            code: reason.code.to_string(),
            text: reason.get_content(language_code)
                .unwrap_or(&reason.code)
                .to_string(),
            requires_details: reason.requires_details,
            additional_details: additional_details.map(String::from),
        }
    }
}
```

## Step 6: Service Layer Updates

```rust
pub struct AccountService {
    account_repo: AccountRepository,
    reason_repo: ReasonAndPurposeRepository,
}

impl AccountService {
    /// Get account with resolved reasons in user's language
    pub fn get_account_view(
        &self, 
        account_id: Uuid, 
        user_language: &[u8; 3]
    ) -> Result<AccountView, Error> {
        let account = self.account_repo.get(account_id)?;
        
        let loan_purpose = account.loan_purpose_id
            .and_then(|id| self.reason_repo.get(&id))
            .map(|reason| ReasonView::from_reason(reason, user_language, None));
            
        let pending_closure_reason = account.pending_closure_reason_id
            .and_then(|id| self.reason_repo.get(&id))
            .map(|reason| ReasonView::from_reason(reason, user_language, None));
        
        Ok(AccountView {
            account_id: account.account_id,
            loan_purpose,
            pending_closure_reason,
            // ... map other fields
        })
    }
    
    /// Validate reason for specific context
    pub fn validate_reason(
        &self,
        reason_id: Uuid,
        category: ReasonCategory,
        context: ReasonContext
    ) -> Result<(), Error> {
        let reason = self.reason_repo.get(&reason_id)
            .ok_or(Error::InvalidReasonId)?;
            
        if !reason.is_active {
            return Err(Error::InactiveReason);
        }
        
        if reason.category != category {
            return Err(Error::WrongReasonCategory);
        }
        
        if reason.context != context {
            return Err(Error::WrongReasonContext);
        }
        
        Ok(())
    }
}
```

## Step 7: Migration Strategy

1. **Create reasons table**:
```sql
CREATE TABLE reason_and_purpose (
    id UUID PRIMARY KEY,
    code VARCHAR(50) UNIQUE NOT NULL,
    category VARCHAR(50) NOT NULL,
    context VARCHAR(50) NOT NULL,
    l1_content VARCHAR(100),
    l2_content VARCHAR(100),
    l3_content VARCHAR(100),
    l1_language_code CHAR(3),
    l2_language_code CHAR(3),
    l3_language_code CHAR(3),
    requires_details BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    severity VARCHAR(20),
    display_order INTEGER DEFAULT 0,
    -- Compliance metadata fields
    regulatory_code VARCHAR(20),
    reportable BOOLEAN DEFAULT FALSE,
    requires_sar BOOLEAN DEFAULT FALSE,
    requires_ctr BOOLEAN DEFAULT FALSE,
    retention_years SMALLINT,
    escalation_required BOOLEAN DEFAULT FALSE,
    risk_score_impact SMALLINT,
    no_tipping_off BOOLEAN DEFAULT FALSE,
    jurisdictions TEXT[], -- Array of country codes
    -- Audit fields
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    created_by_person_id VARCHAR(100) NOT NULL,
    updated_by_person_id VARCHAR(100) NOT NULL
);

CREATE INDEX idx_reason_code ON reason_and_purpose(code);
CREATE INDEX idx_reason_category ON reason_and_purpose(category);
CREATE INDEX idx_reason_context ON reason_and_purpose(context);
CREATE INDEX idx_reason_reportable ON reason_and_purpose(reportable) WHERE reportable = TRUE;
CREATE INDEX idx_reason_sar ON reason_and_purpose(requires_sar) WHERE requires_sar = TRUE;
```

2. **Populate with existing reasons**:
```sql
-- Extract unique reasons from existing data
INSERT INTO reason_and_purpose (id, code, category, context, l1_content, ...)
SELECT DISTINCT
    gen_random_uuid(),
    UPPER(REPLACE(loan_purpose, ' ', '_')),
    'LoanPurpose',
    'Loan',
    loan_purpose,
    ...
FROM accounts
WHERE loan_purpose IS NOT NULL;
```

3. **Update foreign keys**:
```sql
-- Add new columns
ALTER TABLE accounts ADD COLUMN loan_purpose_id UUID;
ALTER TABLE accounts ADD COLUMN pending_closure_reason_id UUID;

-- Update with mapped IDs
UPDATE accounts a
SET loan_purpose_id = r.id
FROM reason_and_purpose r
WHERE UPPER(REPLACE(a.loan_purpose, ' ', '_')) = r.code;
```

## Step 8: Benefits

1. **Centralized translation** - Update reason text in one place for all languages
2. **Consistency** - No duplicate or variant reasons
3. **Memory efficiency** - Store text once, reference everywhere
4. **Audit trail** - Track who created/updated reasons
5. **Validation** - Ensure only valid reasons for specific contexts
6. **UI improvement** - Sorted dropdown lists with proper translations
7. **Reporting** - Easy analytics on reason usage

## Testing Checklist

- [ ] ReasonAndPurpose struct compiles with all fields
- [ ] All string reason/purpose fields replaced with UUID references
- [ ] Repository supports CRUD operations and queries
- [ ] Language fallback logic works correctly
- [ ] View models resolve reasons in correct language
- [ ] Validation prevents wrong category/context usage
- [ ] Migration successfully maps existing reasons
- [ ] API responses include translated reason text
- [ ] UI dropdowns show reasons in user's language