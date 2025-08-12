use chrono::{DateTime, NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;
use uuid::Uuid;

use banking_api::domain::{KycStatus, RiskRating, TransactionType};

/// CASA (Current & Savings Account) specialized functionality
/// Building upon the Unified Account Model
/// Overdraft facility configuration and management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftFacility {
    pub id: Uuid,
    pub account_id: Uuid,
    pub approved_limit: Decimal,
    pub current_utilized: Decimal,
    pub available_limit: Decimal,
    pub interest_rate: Decimal, // Debit interest rate for overdrawn amounts
    #[serde(serialize_with = "serialize_overdraft_status", deserialize_with = "deserialize_overdraft_status")]
    pub facility_status: OverdraftStatus,
    pub approval_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub approved_by_person_id: Uuid, // References Person.person_id
    pub review_frequency: ReviewFrequency,
    pub next_review_date: NaiveDate,
    pub security_required: bool,
    pub security_details: Option<HeaplessString<200>>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "overdraft_status", rename_all = "PascalCase")]
pub enum OverdraftStatus {
    Active,
    Suspended,
    Expired,
    UnderReview,
    Cancelled,
}

impl FromStr for OverdraftStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(OverdraftStatus::Active),
            "Suspended" => Ok(OverdraftStatus::Suspended),
            "Expired" => Ok(OverdraftStatus::Expired),
            "UnderReview" => Ok(OverdraftStatus::UnderReview),
            "Cancelled" => Ok(OverdraftStatus::Cancelled),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "review_frequency", rename_all = "PascalCase")]
pub enum ReviewFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}

impl FromStr for ReviewFrequency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monthly" => Ok(ReviewFrequency::Monthly),
            "Quarterly" => Ok(ReviewFrequency::Quarterly),
            "SemiAnnually" => Ok(ReviewFrequency::SemiAnnually),
            "Annually" => Ok(ReviewFrequency::Annually),
            _ => Err(()),
        }
    }
}

/// Overdraft utilization tracking for interest calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftUtilization {
    pub id: Uuid,
    pub account_id: Uuid,
    pub utilization_date: NaiveDate,
    pub opening_balance: Decimal, // Negative for overdrawn
    pub closing_balance: Decimal, // Negative for overdrawn
    pub average_daily_balance: Decimal,
    pub days_overdrawn: u32,
    pub interest_accrued: Decimal,
    pub interest_rate: Decimal,
    pub capitalized: bool,
    pub capitalization_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

/// Overdraft interest calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftInterestCalculation {
    pub id: Uuid,
    pub account_id: Uuid,
    pub calculation_period_start: NaiveDate,
    pub calculation_period_end: NaiveDate,
    pub average_overdrawn_balance: Decimal,
    pub applicable_rate: Decimal,
    pub days_calculated: u32,
    pub interest_amount: Decimal,
    pub compounding_frequency: CompoundingFrequency,
    pub capitalization_due: bool,
    pub calculated_at: DateTime<Utc>,
    pub calculated_by_person_id: Uuid, // References Person.person_id
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "compounding_frequency", rename_all = "PascalCase")]
pub enum CompoundingFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

impl FromStr for CompoundingFrequency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Daily" => Ok(CompoundingFrequency::Daily),
            "Weekly" => Ok(CompoundingFrequency::Weekly),
            "Monthly" => Ok(CompoundingFrequency::Monthly),
            "Quarterly" => Ok(CompoundingFrequency::Quarterly),
            _ => Err(()),
        }
    }
}

/// CASA account summary for comprehensive reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasaAccountSummary {
    pub account_id: Uuid,
    #[serde(serialize_with = "crate::models::serialize_account_type", deserialize_with = "crate::models::deserialize_account_type")]
    pub account_type: banking_api::domain::AccountType,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    
    // Overdraft information (for current accounts)
    pub overdraft_facility: Option<OverdraftFacility>,
    pub is_overdrawn: bool,
    pub overdrawn_amount: Option<Decimal>,
    pub days_overdrawn: Option<u32>,
    
    // Interest information
    pub credit_interest_accrued: Decimal, // Positive balances
    pub debit_interest_accrued: Decimal,  // Overdrawn balances
    pub last_interest_posting_date: Option<NaiveDate>,
    pub next_interest_posting_date: Option<NaiveDate>,
    
    // Activity summary
    pub last_transaction_date: Option<NaiveDate>,
    pub mtd_transaction_count: u32,
    pub mtd_debit_amount: Decimal,
    pub mtd_credit_amount: Decimal,
    
    // Account status
    pub dormancy_risk: DormancyRisk,
    pub compliance_status: CasaComplianceStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "dormancy_risk", rename_all = "PascalCase")]
pub enum DormancyRisk {
    Active,
    AtRisk,
    Dormant,
    Inactive,
}

impl FromStr for DormancyRisk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(DormancyRisk::Active),
            "AtRisk" => Ok(DormancyRisk::AtRisk),
            "Dormant" => Ok(DormancyRisk::Dormant),
            "Inactive" => Ok(DormancyRisk::Inactive),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasaComplianceStatus {
    #[serde(serialize_with = "serialize_kyc_status", deserialize_with = "deserialize_kyc_status")]
    pub kyc_status: KycStatus,
    pub last_kyc_update: Option<NaiveDate>,
    #[serde(serialize_with = "serialize_risk_rating", deserialize_with = "deserialize_risk_rating")]
    pub aml_risk_rating: RiskRating,
    pub regulatory_alerts_01: HeaplessString<200>,
    pub regulatory_alerts_02: HeaplessString<200>,
    pub regulatory_alerts_03: HeaplessString<200>,
    pub regulatory_alerts_04: HeaplessString<200>,
    pub regulatory_alerts_05: HeaplessString<200>,
}

/// Daily overdraft processing job for EOD
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftProcessingJob {
    pub id: Uuid,
    pub processing_date: NaiveDate,
    pub accounts_processed: u32,
    pub total_interest_accrued: Decimal,
    pub accounts_capitalized: u32,
    pub total_capitalized_amount: Decimal,
    pub status: ProcessingJobStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub errors_01: HeaplessString<200>,
    pub errors_02: HeaplessString<200>,
    pub errors_03: HeaplessString<200>,
    pub errors_04: HeaplessString<200>,
    pub errors_05: HeaplessString<200>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "processing_job_status", rename_all = "PascalCase")]
pub enum ProcessingJobStatus {
    Scheduled,
    Running,
    Completed,
    Failed,
    PartiallyCompleted,
}

impl FromStr for ProcessingJobStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Scheduled" => Ok(ProcessingJobStatus::Scheduled),
            "Running" => Ok(ProcessingJobStatus::Running),
            "Completed" => Ok(ProcessingJobStatus::Completed),
            "Failed" => Ok(ProcessingJobStatus::Failed),
            "PartiallyCompleted" => Ok(ProcessingJobStatus::PartiallyCompleted),
            _ => Err(()),
        }
    }
}

/// Overdraft limit adjustment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdraftLimitAdjustment {
    pub id: Uuid,
    pub account_id: Uuid,
    pub current_limit: Decimal,
    pub requested_limit: Decimal,
    /// References ReasonAndPurpose.id for adjustment reason
    pub adjustment_reason_id: Uuid,
    /// Additional context for adjustment
    pub additional_details: Option<HeaplessString<200>>,
    pub required_document01_id: Option<Uuid>,
    pub required_document02_id: Option<Uuid>,
    pub required_document03_id: Option<Uuid>,
    pub required_document04_id: Option<Uuid>,
    pub required_document05_id: Option<Uuid>,
    pub required_document06_id: Option<Uuid>,
    pub required_document07_id: Option<Uuid>,
    pub requested_by_person_id: Uuid, // References Person.person_id
    pub requested_at: DateTime<Utc>,
    pub approval_status: CasaApprovalStatus,
    pub approved_by_person_id: Option<Uuid>, // References Person.person_id
    pub approved_at: Option<DateTime<Utc>>,
    pub approval_notes: Option<HeaplessString<500>>,
    pub effective_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "casa_approval_status", rename_all = "PascalCase")]
pub enum CasaApprovalStatus {
    Pending,
    Approved,
    Rejected,
    RequiresAdditionalDocuments,
    UnderReview,
}

impl FromStr for CasaApprovalStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Pending" => Ok(CasaApprovalStatus::Pending),
            "Approved" => Ok(CasaApprovalStatus::Approved),
            "Rejected" => Ok(CasaApprovalStatus::Rejected),
            "RequiresAdditionalDocuments" => Ok(CasaApprovalStatus::RequiresAdditionalDocuments),
            "UnderReview" => Ok(CasaApprovalStatus::UnderReview),
            _ => Err(()),
        }
    }
}

/// Transaction validation context for CASA accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CasaTransactionValidation {
    pub account_id: Uuid,
    pub transaction_amount: Decimal,
    #[serde(serialize_with = "serialize_transaction_type", deserialize_with = "deserialize_transaction_type")]
    pub transaction_type: TransactionType,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub post_transaction_balance: Decimal,
    pub overdraft_utilization: Option<Decimal>,
    pub validation_result: CasaValidationResult,
    pub validation_message_01: HeaplessString<200>,
    pub validation_message_02: HeaplessString<200>,
    pub validation_message_03: HeaplessString<200>,
    pub requires_authorization: bool,
    pub authorization_level: Option<AuthorizationLevel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "casa_validation_result", rename_all = "PascalCase")]
pub enum CasaValidationResult {
    Approved,
    Rejected,
    RequiresAuthorization,
    RequiresHoldRelease,
}

impl FromStr for CasaValidationResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Approved" => Ok(CasaValidationResult::Approved),
            "Rejected" => Ok(CasaValidationResult::Rejected),
            "RequiresAuthorization" => Ok(CasaValidationResult::RequiresAuthorization),
            "RequiresHoldRelease" => Ok(CasaValidationResult::RequiresHoldRelease),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "authorization_level", rename_all = "PascalCase")]
pub enum AuthorizationLevel {
    Teller,
    Supervisor,
    Manager,
    CreditCommittee,
}

impl FromStr for AuthorizationLevel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Teller" => Ok(AuthorizationLevel::Teller),
            "Supervisor" => Ok(AuthorizationLevel::Supervisor),
            "Manager" => Ok(AuthorizationLevel::Manager),
            "CreditCommittee" => Ok(AuthorizationLevel::CreditCommittee),
            _ => Err(()),
        }
    }
}

/// Interest posting record for CASA accounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterestPostingRecord {
    pub id: Uuid,
    pub account_id: Uuid,
    pub posting_date: NaiveDate,
    pub interest_type: InterestType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub principal_amount: Decimal,
    pub interest_rate: Decimal,
    pub days_calculated: u32,
    pub interest_amount: Decimal,
    pub tax_withheld: Option<Decimal>,
    pub net_amount: Decimal,
    pub posting_status: PostingStatus,
    pub posted_by_person_id: Uuid, // References Person.person_id
    pub posted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "interest_type", rename_all = "PascalCase")]
pub enum InterestType {
    CreditInterest,  // Positive balance interest
    DebitInterest,   // Overdraft interest
    PenaltyInterest, // Additional penalty
}

impl FromStr for InterestType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CreditInterest" => Ok(InterestType::CreditInterest),
            "DebitInterest" => Ok(InterestType::DebitInterest),
            "PenaltyInterest" => Ok(InterestType::PenaltyInterest),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "posting_status", rename_all = "PascalCase")]
pub enum PostingStatus {
    Calculated,
    Posted,
    Reversed,
    Adjusted,
}

impl FromStr for PostingStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Calculated" => Ok(PostingStatus::Calculated),
            "Posted" => Ok(PostingStatus::Posted),
            "Reversed" => Ok(PostingStatus::Reversed),
            "Adjusted" => Ok(PostingStatus::Adjusted),
            _ => Err(()),
        }
    }
}

// Custom serialization functions for database compatibility

fn serialize_overdraft_status<S>(value: &OverdraftStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        OverdraftStatus::Active => "Active",
        OverdraftStatus::Suspended => "Suspended",
        OverdraftStatus::Expired => "Expired",
        OverdraftStatus::UnderReview => "UnderReview",
        OverdraftStatus::Cancelled => "Cancelled",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_overdraft_status<'de, D>(deserializer: D) -> Result<OverdraftStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Active" => Ok(OverdraftStatus::Active),
        "Suspended" => Ok(OverdraftStatus::Suspended),
        "Expired" => Ok(OverdraftStatus::Expired),
        "UnderReview" => Ok(OverdraftStatus::UnderReview),
        "Cancelled" => Ok(OverdraftStatus::Cancelled),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Active", "Suspended", "Expired", "UnderReview", "Cancelled"])),
    }
}

#[allow(dead_code)]
fn serialize_review_frequency<S>(value: &ReviewFrequency, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        ReviewFrequency::Monthly => "Monthly",
        ReviewFrequency::Quarterly => "Quarterly",
        ReviewFrequency::SemiAnnually => "SemiAnnually",
        ReviewFrequency::Annually => "Annually",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_review_frequency<'de, D>(deserializer: D) -> Result<ReviewFrequency, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Monthly" => Ok(ReviewFrequency::Monthly),
        "Quarterly" => Ok(ReviewFrequency::Quarterly),
        "SemiAnnually" => Ok(ReviewFrequency::SemiAnnually),
        "Annually" => Ok(ReviewFrequency::Annually),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Monthly", "Quarterly", "SemiAnnually", "Annually"])),
    }
}

#[allow(dead_code)]
fn serialize_compounding_frequency<S>(value: &CompoundingFrequency, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        CompoundingFrequency::Daily => "Daily",
        CompoundingFrequency::Weekly => "Weekly",
        CompoundingFrequency::Monthly => "Monthly",
        CompoundingFrequency::Quarterly => "Quarterly",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_compounding_frequency<'de, D>(deserializer: D) -> Result<CompoundingFrequency, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Daily" => Ok(CompoundingFrequency::Daily),
        "Weekly" => Ok(CompoundingFrequency::Weekly),
        "Monthly" => Ok(CompoundingFrequency::Monthly),
        "Quarterly" => Ok(CompoundingFrequency::Quarterly),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Daily", "Weekly", "Monthly", "Quarterly"])),
    }
}

#[allow(dead_code)]
fn serialize_dormancy_risk<S>(value: &DormancyRisk, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        DormancyRisk::Active => "Active",
        DormancyRisk::AtRisk => "AtRisk",
        DormancyRisk::Dormant => "Dormant",
        DormancyRisk::Inactive => "Inactive",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_dormancy_risk<'de, D>(deserializer: D) -> Result<DormancyRisk, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Active" => Ok(DormancyRisk::Active),
        "AtRisk" => Ok(DormancyRisk::AtRisk),
        "Dormant" => Ok(DormancyRisk::Dormant),
        "Inactive" => Ok(DormancyRisk::Inactive),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Active", "AtRisk", "Dormant", "Inactive"])),
    }
}

pub fn serialize_processing_job_status<S>(value: &ProcessingJobStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        ProcessingJobStatus::Scheduled => "Scheduled",
        ProcessingJobStatus::Running => "Running",
        ProcessingJobStatus::Completed => "Completed",
        ProcessingJobStatus::Failed => "Failed",
        ProcessingJobStatus::PartiallyCompleted => "PartiallyCompleted",
    };
    serializer.serialize_str(value_str)
}

pub fn deserialize_processing_job_status<'de, D>(deserializer: D) -> Result<ProcessingJobStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Scheduled" => Ok(ProcessingJobStatus::Scheduled),
        "Running" => Ok(ProcessingJobStatus::Running),
        "Completed" => Ok(ProcessingJobStatus::Completed),
        "Failed" => Ok(ProcessingJobStatus::Failed),
        "PartiallyCompleted" => Ok(ProcessingJobStatus::PartiallyCompleted),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Scheduled", "Running", "Completed", "Failed", "PartiallyCompleted"])),
    }
}

#[allow(dead_code)]
fn serialize_casa_approval_status<S>(value: &CasaApprovalStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        CasaApprovalStatus::Pending => "Pending",
        CasaApprovalStatus::Approved => "Approved",
        CasaApprovalStatus::Rejected => "Rejected",
        CasaApprovalStatus::RequiresAdditionalDocuments => "RequiresAdditionalDocuments",
        CasaApprovalStatus::UnderReview => "UnderReview",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_casa_approval_status<'de, D>(deserializer: D) -> Result<CasaApprovalStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Pending" => Ok(CasaApprovalStatus::Pending),
        "Approved" => Ok(CasaApprovalStatus::Approved),
        "Rejected" => Ok(CasaApprovalStatus::Rejected),
        "RequiresAdditionalDocuments" => Ok(CasaApprovalStatus::RequiresAdditionalDocuments),
        "UnderReview" => Ok(CasaApprovalStatus::UnderReview),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Pending", "Approved", "Rejected", "RequiresAdditionalDocuments", "UnderReview"])),
    }
}

#[allow(dead_code)]
fn serialize_casa_validation_result<S>(value: &CasaValidationResult, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        CasaValidationResult::Approved => "Approved",
        CasaValidationResult::Rejected => "Rejected",
        CasaValidationResult::RequiresAuthorization => "RequiresAuthorization",
        CasaValidationResult::RequiresHoldRelease => "RequiresHoldRelease",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_casa_validation_result<'de, D>(deserializer: D) -> Result<CasaValidationResult, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Approved" => Ok(CasaValidationResult::Approved),
        "Rejected" => Ok(CasaValidationResult::Rejected),
        "RequiresAuthorization" => Ok(CasaValidationResult::RequiresAuthorization),
        "RequiresHoldRelease" => Ok(CasaValidationResult::RequiresHoldRelease),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Approved", "Rejected", "RequiresAuthorization", "RequiresHoldRelease"])),
    }
}

#[allow(dead_code)]
fn serialize_authorization_level_opt<S>(value: &Option<AuthorizationLevel>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    match value {
        Some(level) => {
            let value_str = match level {
                AuthorizationLevel::Teller => "Teller",
                AuthorizationLevel::Supervisor => "Supervisor",
                AuthorizationLevel::Manager => "Manager",
                AuthorizationLevel::CreditCommittee => "CreditCommittee",
            };
            serializer.serialize_some(value_str)
        }
        None => serializer.serialize_none(),
    }
}

#[allow(dead_code)]
fn deserialize_authorization_level_opt<'de, D>(deserializer: D) -> Result<Option<AuthorizationLevel>, D::Error>
where D: Deserializer<'de> {
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => match s.as_str() {
            "Teller" => Ok(Some(AuthorizationLevel::Teller)),
            "Supervisor" => Ok(Some(AuthorizationLevel::Supervisor)),
            "Manager" => Ok(Some(AuthorizationLevel::Manager)),
            "CreditCommittee" => Ok(Some(AuthorizationLevel::CreditCommittee)),
            _ => Err(serde::de::Error::unknown_variant(&s, &["Teller", "Supervisor", "Manager", "CreditCommittee"])),
        },
        None => Ok(None),
    }
}

#[allow(dead_code)]
fn serialize_interest_type<S>(value: &InterestType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        InterestType::CreditInterest => "CreditInterest",
        InterestType::DebitInterest => "DebitInterest",
        InterestType::PenaltyInterest => "PenaltyInterest",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_interest_type<'de, D>(deserializer: D) -> Result<InterestType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "CreditInterest" => Ok(InterestType::CreditInterest),
        "DebitInterest" => Ok(InterestType::DebitInterest),
        "PenaltyInterest" => Ok(InterestType::PenaltyInterest),
        _ => Err(serde::de::Error::unknown_variant(&s, &["CreditInterest", "DebitInterest", "PenaltyInterest"])),
    }
}

#[allow(dead_code)]
fn serialize_posting_status<S>(value: &PostingStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        PostingStatus::Calculated => "Calculated",
        PostingStatus::Posted => "Posted",
        PostingStatus::Reversed => "Reversed",
        PostingStatus::Adjusted => "Adjusted",
    };
    serializer.serialize_str(value_str)
}

#[allow(dead_code)]
fn deserialize_posting_status<'de, D>(deserializer: D) -> Result<PostingStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Calculated" => Ok(PostingStatus::Calculated),
        "Posted" => Ok(PostingStatus::Posted),
        "Reversed" => Ok(PostingStatus::Reversed),
        "Adjusted" => Ok(PostingStatus::Adjusted),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Calculated", "Posted", "Reversed", "Adjusted"])),
    }
}

// Local serialization functions for imported enums
fn serialize_kyc_status<S>(value: &KycStatus, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        KycStatus::NotStarted => "NotStarted",
        KycStatus::InProgress => "InProgress",
        KycStatus::Pending => "Pending",
        KycStatus::Complete => "Complete",
        KycStatus::Approved => "Approved",
        KycStatus::Rejected => "Rejected",
        KycStatus::RequiresUpdate => "RequiresUpdate",
        KycStatus::Failed => "Failed",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_kyc_status<'de, D>(deserializer: D) -> Result<KycStatus, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "NotStarted" => Ok(KycStatus::NotStarted),
        "InProgress" => Ok(KycStatus::InProgress),
        "Pending" => Ok(KycStatus::Pending),
        "Complete" => Ok(KycStatus::Complete),
        "Approved" => Ok(KycStatus::Approved),
        "Rejected" => Ok(KycStatus::Rejected),
        "RequiresUpdate" => Ok(KycStatus::RequiresUpdate),
        "Failed" => Ok(KycStatus::Failed),
        _ => Err(serde::de::Error::unknown_variant(&s, &["NotStarted", "InProgress", "Pending", "Complete", "Approved", "Rejected", "RequiresUpdate", "Failed"])),
    }
}

fn serialize_risk_rating<S>(value: &RiskRating, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        RiskRating::Low => "Low",
        RiskRating::Medium => "Medium",
        RiskRating::High => "High",
        RiskRating::Blacklisted => "Blacklisted",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_risk_rating<'de, D>(deserializer: D) -> Result<RiskRating, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Low" => Ok(RiskRating::Low),
        "Medium" => Ok(RiskRating::Medium),
        "High" => Ok(RiskRating::High),
        "Blacklisted" => Ok(RiskRating::Blacklisted),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Low", "Medium", "High", "Blacklisted"])),
    }
}

fn serialize_transaction_type<S>(value: &TransactionType, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    let value_str = match value {
        TransactionType::Credit => "Credit",
        TransactionType::Debit => "Debit",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_transaction_type<'de, D>(deserializer: D) -> Result<TransactionType, D::Error>
where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    match s.as_str() {
        "Credit" => Ok(TransactionType::Credit),
        "Debit" => Ok(TransactionType::Debit),
        _ => Err(serde::de::Error::unknown_variant(&s, &["Credit", "Debit"])),
    }
}