use chrono::{DateTime, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;
use std::str::FromStr;

/// Database model for Customer table
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerModel {
    pub id: Uuid,
    #[serde(serialize_with = "serialize_customer_type", deserialize_with = "deserialize_customer_type")]
    pub customer_type: CustomerType,
    pub full_name: HeaplessString<100>,
    #[serde(serialize_with = "serialize_identity_type", deserialize_with = "deserialize_identity_type")]
    pub id_type: IdentityType,
    pub id_number: HeaplessString<50>,
    #[serde(serialize_with = "serialize_risk_rating", deserialize_with = "deserialize_risk_rating")]
    pub risk_rating: RiskRating,
    #[serde(serialize_with = "serialize_customer_status", deserialize_with = "deserialize_customer_status")]
    pub status: CustomerStatus,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    /// References Person.person_id
    pub updated_by_person_id: Uuid,
}

/// Database model for Customer Portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerPortfolioModel {
    pub customer_id: Uuid,
    pub total_accounts: i64,
    pub total_balance: Decimal,
    pub total_loan_outstanding: Option<Decimal>,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub risk_score: Option<Decimal>,
    #[serde(serialize_with = "serialize_kyc_status", deserialize_with = "deserialize_kyc_status")]
    pub kyc_status: KycStatus,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

/// Database model for Customer documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerDocumentModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub document_type: HeaplessString<50>,
    pub document_path: Option<HeaplessString<500>>,
    #[serde(serialize_with = "serialize_document_status", deserialize_with = "deserialize_document_status")]
    pub status: DocumentStatus,
    pub uploaded_at: DateTime<Utc>,
    /// References Person.person_id
    pub uploaded_by: Uuid,
    pub verified_at: Option<DateTime<Utc>>,
    /// References Person.person_id
    pub verified_by: Option<Uuid>,
}

/// Database model for Customer audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerAuditModel {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub field_name: HeaplessString<50>,
    pub old_value: Option<HeaplessString<255>>,
    pub new_value: Option<HeaplessString<255>>,
    pub changed_at: DateTime<Utc>,
    /// References Person.person_id
    pub changed_by: Uuid,
    pub reason: Option<HeaplessString<255>>,
}

/// Database model for Customer risk summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct RiskSummaryModel {
    #[serde(serialize_with = "serialize_risk_rating", deserialize_with = "deserialize_risk_rating")]
    pub current_rating: RiskRating,
    pub last_assessment_date: DateTime<Utc>,
    pub flags_01: HeaplessString<200>,
    pub flags_02: HeaplessString<200>,
    pub flags_03: HeaplessString<200>,
    pub flags_04: HeaplessString<200>,
    pub flags_05: HeaplessString<200>,
}

/// Database model for Customer compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct CustomerComplianceStatusModel {
    #[serde(serialize_with = "serialize_kyc_status", deserialize_with = "deserialize_kyc_status")]
    pub kyc_status: KycStatus,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}

// ============================================================================
// ENUMS - Database layer specific enums
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "customer_type", rename_all = "PascalCase")]
pub enum CustomerType {
    Individual,
    Corporate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "identity_type", rename_all = "PascalCase")]
pub enum IdentityType {
    NationalId,
    Passport,
    CompanyRegistration,
    PermanentResidentCard,
    AsylumCard,
    TemporaryResidentPermit,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "risk_rating", rename_all = "PascalCase")]
pub enum RiskRating {
    Low,
    Medium,
    High,
    Blacklisted,
}

impl std::fmt::Display for RiskRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiskRating::Low => write!(f, "Low"),
            RiskRating::Medium => write!(f, "Medium"),
            RiskRating::High => write!(f, "High"),
            RiskRating::Blacklisted => write!(f, "Blacklisted"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "customer_status", rename_all = "PascalCase")]
pub enum CustomerStatus {
    Active,
    PendingVerification,
    Deceased,
    Dissolved,
    Blacklisted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "kyc_status", rename_all = "PascalCase")]
pub enum KycStatus {
    NotStarted,
    InProgress,
    Pending,
    Complete,
    Approved,
    Rejected,
    RequiresUpdate,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "document_status", rename_all = "PascalCase")]
pub enum DocumentStatus {
    Uploaded,
    Verified,
    Rejected,
    Expired,
}


impl std::fmt::Display for DocumentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentStatus::Uploaded => write!(f, "Uploaded"),
            DocumentStatus::Verified => write!(f, "Verified"),
            DocumentStatus::Rejected => write!(f, "Rejected"),
            DocumentStatus::Expired => write!(f, "Expired"),
        }
    }
}

impl FromStr for DocumentStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Uploaded" => Ok(DocumentStatus::Uploaded),
            "Verified" => Ok(DocumentStatus::Verified),
            "Rejected" => Ok(DocumentStatus::Rejected),
            "Expired" => Ok(DocumentStatus::Expired),
            _ => Err(()),
        }
    }
}

impl FromStr for CustomerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Individual" => Ok(CustomerType::Individual),
            "Corporate" => Ok(CustomerType::Corporate),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for IdentityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentityType::NationalId => write!(f, "NationalId"),
            IdentityType::Passport => write!(f, "Passport"),
            IdentityType::CompanyRegistration => write!(f, "CompanyRegistration"),
            IdentityType::PermanentResidentCard => write!(f, "PermanentResidentCard"),
            IdentityType::AsylumCard => write!(f, "AsylumCard"),
            IdentityType::TemporaryResidentPermit => write!(f, "TemporaryResidentPermit"),
            IdentityType::Unknown => write!(f, "Unknown"),
        }
    }
}

impl FromStr for IdentityType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NationalId" => Ok(IdentityType::NationalId),
            "Passport" => Ok(IdentityType::Passport),
            "CompanyRegistration" => Ok(IdentityType::CompanyRegistration),
            "PermanentResidentCard" => Ok(IdentityType::PermanentResidentCard),
            "AsylumCard" => Ok(IdentityType::AsylumCard),
            "TemporaryResidentPermit" => Ok(IdentityType::TemporaryResidentPermit),
            "Unknown" => Ok(IdentityType::Unknown),
            _ => Err(()),
        }
    }
}

impl FromStr for RiskRating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(RiskRating::Low),
            "Medium" => Ok(RiskRating::Medium),
            "High" => Ok(RiskRating::High),
            "Blacklisted" => Ok(RiskRating::Blacklisted),
            _ => Err(()),
        }
    }
}

impl FromStr for CustomerStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(CustomerStatus::Active),
            "PendingVerification" => Ok(CustomerStatus::PendingVerification),
            "Deceased" => Ok(CustomerStatus::Deceased),
            "Dissolved" => Ok(CustomerStatus::Dissolved),
            "Blacklisted" => Ok(CustomerStatus::Blacklisted),
            _ => Err(()),
        }
    }
}

impl FromStr for KycStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NotStarted" => Ok(KycStatus::NotStarted),
            "InProgress" => Ok(KycStatus::InProgress),
            "Pending" => Ok(KycStatus::Pending),
            "Complete" => Ok(KycStatus::Complete),
            "Approved" => Ok(KycStatus::Approved),
            "Rejected" => Ok(KycStatus::Rejected),
            "RequiresUpdate" => Ok(KycStatus::RequiresUpdate),
            "Failed" => Ok(KycStatus::Failed),
            _ => Err(()),
        }
    }
}


// ============================================================================
// CUSTOM SERIALIZATION FUNCTIONS
// ============================================================================

fn serialize_customer_type<S>(value: &CustomerType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        CustomerType::Individual => "Individual",
        CustomerType::Corporate => "Corporate",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_customer_type<'de, D>(deserializer: D) -> Result<CustomerType, D::Error>
where
    D: Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Individual" => Ok(CustomerType::Individual),
        "Corporate" => Ok(CustomerType::Corporate),
        _ => Err(serde::de::Error::custom(format!("Invalid CustomerType: {value_str}"))),
    }
}

fn serialize_identity_type<S>(value: &IdentityType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        IdentityType::NationalId => "NationalId",
        IdentityType::Passport => "Passport",
        IdentityType::CompanyRegistration => "CompanyRegistration",
        IdentityType::PermanentResidentCard => "PermanentResidentCard",
        IdentityType::AsylumCard => "AsylumCard",
        IdentityType::TemporaryResidentPermit => "TemporaryResidentPermit",
        IdentityType::Unknown => "Unknown",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_identity_type<'de, D>(deserializer: D) -> Result<IdentityType, D::Error>
where
    D: Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "NationalId" => Ok(IdentityType::NationalId),
        "Passport" => Ok(IdentityType::Passport),
        "CompanyRegistration" => Ok(IdentityType::CompanyRegistration),
        "PermanentResidentCard" => Ok(IdentityType::PermanentResidentCard),
        "AsylumCard" => Ok(IdentityType::AsylumCard),
        "TemporaryResidentPermit" => Ok(IdentityType::TemporaryResidentPermit),
        "Unknown" => Ok(IdentityType::Unknown),
        _ => Err(serde::de::Error::custom(format!("Invalid IdentityType: {value_str}"))),
    }
}

fn serialize_risk_rating<S>(value: &RiskRating, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        RiskRating::Low => "Low",
        RiskRating::Medium => "Medium",
        RiskRating::High => "High",
        RiskRating::Blacklisted => "Blacklisted",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_risk_rating<'de, D>(deserializer: D) -> Result<RiskRating, D::Error>
where
    D: Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Low" => Ok(RiskRating::Low),
        "Medium" => Ok(RiskRating::Medium),
        "High" => Ok(RiskRating::High),
        "Blacklisted" => Ok(RiskRating::Blacklisted),
        _ => Err(serde::de::Error::custom(format!("Invalid RiskRating: {value_str}"))),
    }
}

fn serialize_customer_status<S>(value: &CustomerStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        CustomerStatus::Active => "Active",
        CustomerStatus::PendingVerification => "PendingVerification",
        CustomerStatus::Deceased => "Deceased",
        CustomerStatus::Dissolved => "Dissolved",
        CustomerStatus::Blacklisted => "Blacklisted",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_customer_status<'de, D>(deserializer: D) -> Result<CustomerStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Active" => Ok(CustomerStatus::Active),
        "PendingVerification" => Ok(CustomerStatus::PendingVerification),
        "Deceased" => Ok(CustomerStatus::Deceased),
        "Dissolved" => Ok(CustomerStatus::Dissolved),
        "Blacklisted" => Ok(CustomerStatus::Blacklisted),
        _ => Err(serde::de::Error::custom(format!("Invalid CustomerStatus: {value_str}"))),
    }
}

fn serialize_kyc_status<S>(value: &KycStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
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
where
    D: Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "NotStarted" => Ok(KycStatus::NotStarted),
        "InProgress" => Ok(KycStatus::InProgress),
        "Pending" => Ok(KycStatus::Pending),
        "Complete" => Ok(KycStatus::Complete),
        "Approved" => Ok(KycStatus::Approved),
        "Rejected" => Ok(KycStatus::Rejected),
        "RequiresUpdate" => Ok(KycStatus::RequiresUpdate),
        "Failed" => Ok(KycStatus::Failed),
        _ => Err(serde::de::Error::custom(format!("Invalid KycStatus: {value_str}"))),
    }
}

fn serialize_document_status<S>(value: &DocumentStatus, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value_str = match value {
        DocumentStatus::Uploaded => "Uploaded",
        DocumentStatus::Verified => "Verified",
        DocumentStatus::Rejected => "Rejected",
        DocumentStatus::Expired => "Expired",
    };
    serializer.serialize_str(value_str)
}

fn deserialize_document_status<'de, D>(deserializer: D) -> Result<DocumentStatus, D::Error>
where
    D: Deserializer<'de>,
{
    let value_str = String::deserialize(deserializer)?;
    match value_str.as_str() {
        "Uploaded" => Ok(DocumentStatus::Uploaded),
        "Verified" => Ok(DocumentStatus::Verified),
        "Rejected" => Ok(DocumentStatus::Rejected),
        "Expired" => Ok(DocumentStatus::Expired),
        _ => Err(serde::de::Error::custom(format!("Invalid DocumentStatus: {value_str}"))),
    }
}