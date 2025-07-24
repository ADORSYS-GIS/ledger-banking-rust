use blake3::Hash;
use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use uuid::Uuid;

/// KYC Record database model
#[derive(Debug, Clone)]
pub struct KycRecordModel {
    pub kyc_id: Uuid,
    pub customer_id: Uuid,
    pub status: String, // Pending, Approved, Rejected, RequiresReview
    pub risk_assessment: String,
    pub verification_level: String, // Basic, Enhanced, Simplified
    pub documents_verified: String, // JSON array of document types
    pub last_review_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub reviewed_by: Option<String>,
    pub verification_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Sanctions Screening database model
#[derive(Debug, Clone)]
pub struct SanctionsScreeningModel {
    pub screening_id: Uuid,
    pub customer_id: Uuid,
    pub screening_date: DateTime<Utc>,
    pub screening_result: String, // Clear, Match, PotentialMatch
    pub match_details: Option<String>, // JSON with match information
    pub risk_score: Option<Decimal>,
    pub screening_provider: String,
    pub status: String, // Pending, Cleared, UnderReview
    pub reviewed_by: Option<String>,
    pub review_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Compliance Alert database model
#[derive(Debug, Clone)]
pub struct ComplianceAlertModel {
    pub alert_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub transaction_id: Option<Uuid>,
    pub alert_type: String, // TransactionMonitoring, SanctionsMatch, KYCExpired, etc.
    pub severity: String,   // Low, Medium, High, Critical
    pub description: String,
    pub generated_at: DateTime<Utc>,
    pub status: String, // Open, UnderInvestigation, Resolved, FalsePositive
    pub assigned_to: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<String>,
    pub resolution_notes: Option<String>,
    pub metadata: Option<String>, // JSON with additional alert data
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Ultimate Beneficial Owner database model
#[derive(Debug, Clone)]
pub struct UltimateBeneficiaryModel {
    pub ubo_link_id: Uuid,
    pub corporate_customer_id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    pub control_type: String, // DirectOwnership, IndirectOwnership, SignificantInfluence, SeniorManagement
    pub description: Option<String>,
    pub status: String, // Active, Inactive, Terminated
    pub verification_status: String, // Pending, Verified, Rejected, RequiresUpdate
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<String>,
    pub verification_documents: Option<String>, // JSON array
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Compliance Risk Score database model
#[derive(Debug, Clone)]
pub struct ComplianceRiskScoreModel {
    pub risk_score_id: Uuid,
    pub customer_id: Uuid,
    pub risk_score: Decimal,
    pub risk_category: String, // Low, Medium, High, Critical
    pub calculation_method: String,
    pub factors_considered: String, // JSON array of risk factors
    pub calculated_at: DateTime<Utc>,
    pub calculated_by: String,
    pub valid_until: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Suspicious Activity Report (SAR) database model
#[derive(Debug, Clone)]
pub struct SarDataModel {
    pub sar_id: Uuid,
    pub customer_id: Uuid,
    pub related_transactions: String, // JSON array of transaction IDs
    pub suspicious_activity_type: String,
    pub description: String,
    pub amount_involved: Option<Decimal>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub status: String, // Draft, UnderReview, Approved, Filed
    pub prepared_by: String,
    pub approved_by: Option<String>,
    pub filed_date: Option<NaiveDate>,
    pub reference_number: Option<String>,
    pub regulatory_response: Option<String>,
    pub supporting_documents: Option<String>, // JSON array
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
    pub updated_by: String,
}

/// Customer Document database model (for KYC)
#[derive(Debug, Clone)]
pub struct ComplianceDocumentModel {
    pub document_id: Uuid,
    pub customer_id: Uuid,
    pub document_type: String,
    pub document_path: Hash,
    pub status: String, // Uploaded, Verified, Rejected, Expired
    pub uploaded_at: DateTime<Utc>,
    pub uploaded_by: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified_by: Option<String>,
    pub verification_notes: Option<String>,
    pub expiry_date: Option<NaiveDate>,
}

/// Customer Audit Trail database model
#[derive(Debug, Clone)]
pub struct ComplianceCustomerAuditModel {
    pub audit_id: Uuid,
    pub customer_id: Uuid,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: String,
    pub changed_at: DateTime<Utc>,
    pub changed_by: String,
    pub reason: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Customer Portfolio View database model (for 360-degree customer view)
#[derive(Debug, Clone)]
pub struct ComplianceCustomerPortfolioModel {
    pub customer_id: Uuid,
    pub total_accounts: i64,
    pub total_balance: Decimal,
    pub last_activity_date: Option<DateTime<Utc>>,
    pub risk_score: Option<Decimal>,
    pub kyc_status: String,
    pub sanctions_checked: bool,
    pub last_screening_date: Option<DateTime<Utc>>,
}