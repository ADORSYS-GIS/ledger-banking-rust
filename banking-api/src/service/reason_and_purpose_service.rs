use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    BankingResult,
    domain::{
        ReasonAndPurpose, ReasonCategory, ReasonContext, ReasonSeverity
    },
};

/// Service for managing reasons and purposes across banking operations
/// Provides CRUD operations and specialized queries for reason management
#[async_trait]
pub trait ReasonAndPurposeService: Send + Sync {
    
    // ============================================================================
    // CRUD OPERATIONS
    // ============================================================================
    
    /// Create a new reason
    async fn create_reason(&self, reason: ReasonAndPurpose) -> BankingResult<ReasonAndPurpose>;
    
    /// Find reason by ID
    async fn find_reason_by_id(&self, reason_id: Uuid) -> BankingResult<Option<ReasonAndPurpose>>;
    
    /// Find reason by code (for programmatic access)
    async fn find_reason_by_code(&self, code: &str) -> BankingResult<Option<ReasonAndPurpose>>;
    
    /// Update existing reason
    async fn update_reason(&self, reason: ReasonAndPurpose) -> BankingResult<ReasonAndPurpose>;
    
    /// Deactivate reason (soft delete)
    async fn deactivate_reason(&self, reason_id: Uuid, deactivated_by: &str) -> BankingResult<()>;
    
    /// Reactivate previously deactivated reason
    async fn reactivate_reason(&self, reason_id: Uuid, reactivated_by: &str) -> BankingResult<()>;
    
    // ============================================================================
    // QUERY OPERATIONS
    // ============================================================================
    
    /// Get all active reasons for a specific category
    async fn find_reasons_by_category(&self, category: ReasonCategory) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get all active reasons for a specific context
    async fn find_reasons_by_context(&self, context: ReasonContext) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get reasons by category and context (most common query)
    async fn find_reasons_by_category_and_context(
        &self,
        category: ReasonCategory,
        context: ReasonContext,
    ) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get reasons by severity level
    async fn find_reasons_by_severity(&self, severity: ReasonSeverity) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Search reasons by text content (supports multiple languages)
    async fn search_reasons_by_content(
        &self,
        search_term: &str,
        language_codes: Option<Vec<[u8; 3]>>,
    ) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get all reasons sorted by display order
    async fn get_reasons_for_display(
        &self,
        category: Option<ReasonCategory>,
        context: Option<ReasonContext>,
    ) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    // ============================================================================
    // COMPLIANCE-SPECIFIC OPERATIONS
    // ============================================================================
    
    /// Get all compliance-related reasons that require regulatory reporting
    async fn get_reportable_compliance_reasons(&self) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get reasons that trigger Suspicious Activity Reports (SAR)
    async fn get_sar_triggering_reasons(&self) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get reasons that trigger Currency Transaction Reports (CTR)
    async fn get_ctr_triggering_reasons(&self) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get AML/CTF related reasons
    async fn get_aml_ctf_reasons(&self) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get KYC related reasons
    async fn get_kyc_reasons(&self) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    /// Get reasons by jurisdiction (for compliance filtering)
    async fn get_reasons_by_jurisdiction(&self, jurisdiction_code: [u8; 2]) -> BankingResult<Vec<ReasonAndPurpose>>;
    
    // ============================================================================
    // AUDIT AND HISTORY
    // ============================================================================
    
    /// Get reason usage statistics
    async fn get_reason_usage_stats(
        &self,
        reason_id: Uuid,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<ReasonUsageStats>;
    
    /// Get most frequently used reasons by category
    async fn get_top_used_reasons_by_category(
        &self,
        category: ReasonCategory,
        limit: i32,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<Vec<ReasonUsageStats>>;
    
    /// Get reason change history
    async fn get_reason_change_history(&self, reason_id: Uuid) -> BankingResult<Vec<ReasonChangeLog>>;
    
    // ============================================================================
    // VALIDATION OPERATIONS
    // ============================================================================
    
    /// Validate that a reason is appropriate for a specific context
    async fn validate_reason_for_context(
        &self,
        reason_id: Uuid,
        context: ReasonContext,
    ) -> BankingResult<bool>;
    
    /// Validate reason requirements (e.g., additional details required)
    async fn validate_reason_requirements(
        &self,
        reason_id: Uuid,
        additional_details: Option<&str>,
    ) -> BankingResult<ValidationResult>;
    
    /// Check if reason is still active and usable
    async fn is_reason_active(&self, reason_id: Uuid) -> BankingResult<bool>;
    
    // ============================================================================
    // BULK OPERATIONS
    // ============================================================================
    
    /// Import reasons from external system or file
    async fn bulk_import_reasons(
        &self,
        reasons: Vec<ReasonAndPurpose>,
        imported_by: &str,
    ) -> BankingResult<BulkImportResult>;
    
    /// Update display orders for a category
    async fn update_display_orders(
        &self,
        category: ReasonCategory,
        reason_order_map: Vec<(Uuid, i32)>,
        updated_by: &str,
    ) -> BankingResult<()>;
    
    // ============================================================================
    // LOCALIZATION OPERATIONS
    // ============================================================================
    
    /// Add or update localized content for a reason
    async fn update_localized_content(
        &self,
        reason_id: Uuid,
        language_code: [u8; 3],
        content: &str,
        updated_by: &str,
    ) -> BankingResult<()>;
    
    /// Get reasons with localized content for specific language
    async fn get_localized_reasons(
        &self,
        language_codes: &[[u8; 3]],
        category: Option<ReasonCategory>,
        context: Option<ReasonContext>,
    ) -> BankingResult<Vec<LocalizedReason>>;
    
    /// Remove localized content for a language
    async fn remove_localized_content(
        &self,
        reason_id: Uuid,
        language_code: [u8; 3],
        updated_by: &str,
    ) -> BankingResult<()>;
}

/// Statistics about reason usage
#[derive(Debug, Clone)]
pub struct ReasonUsageStats {
    pub reason_id: Uuid,
    pub reason_code: String,
    pub reason_content: String,
    pub usage_count: u64,
    pub first_used: chrono::DateTime<chrono::Utc>,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub contexts_used: Vec<ReasonContext>,
}

/// Change log entry for reason modifications
#[derive(Debug, Clone)]
pub struct ReasonChangeLog {
    pub change_id: Uuid,
    pub reason_id: Uuid,
    pub change_type: ReasonChangeType,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: String,
    pub changed_at: chrono::DateTime<chrono::Utc>,
    pub change_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ReasonChangeType {
    Created,
    ContentUpdated,
    CategoryChanged,
    ContextChanged,
    SeverityChanged,
    ComplianceMetadataUpdated,
    LocalizationAdded,
    LocalizationUpdated,
    LocalizationRemoved,
    Activated,
    Deactivated,
    DisplayOrderUpdated,
}

/// Result of reason validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub error_code: String,
}

/// Result of bulk import operation
#[derive(Debug, Clone)]
pub struct BulkImportResult {
    pub total_processed: usize,
    pub successfully_imported: usize,
    pub updated_existing: usize,
    pub failed_imports: usize,
    pub errors: Vec<ImportError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ImportError {
    pub row_number: usize,
    pub reason_code: Option<String>,
    pub error_message: String,
    pub error_type: ImportErrorType,
}

#[derive(Debug, Clone)]
pub enum ImportErrorType {
    ValidationError,
    DuplicateCode,
    InvalidFormat,
    MissingRequiredField,
    DatabaseError,
}

/// Localized reason with resolved content
#[derive(Debug, Clone)]
pub struct LocalizedReason {
    pub reason_id: Uuid,
    pub code: String,
    pub category: ReasonCategory,
    pub context: ReasonContext,
    pub content: String,  // Resolved based on language preference
    pub language_code: [u8; 3],
    pub requires_details: bool,
    pub severity: Option<ReasonSeverity>,
    pub display_order: i32,
    pub is_active: bool,
}

impl std::fmt::Display for ReasonChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasonChangeType::Created => write!(f, "Created"),
            ReasonChangeType::ContentUpdated => write!(f, "ContentUpdated"),
            ReasonChangeType::CategoryChanged => write!(f, "CategoryChanged"),
            ReasonChangeType::ContextChanged => write!(f, "ContextChanged"),
            ReasonChangeType::SeverityChanged => write!(f, "SeverityChanged"),
            ReasonChangeType::ComplianceMetadataUpdated => write!(f, "ComplianceMetadataUpdated"),
            ReasonChangeType::LocalizationAdded => write!(f, "LocalizationAdded"),
            ReasonChangeType::LocalizationUpdated => write!(f, "LocalizationUpdated"),
            ReasonChangeType::LocalizationRemoved => write!(f, "LocalizationRemoved"),
            ReasonChangeType::Activated => write!(f, "Activated"),
            ReasonChangeType::Deactivated => write!(f, "Deactivated"),
            ReasonChangeType::DisplayOrderUpdated => write!(f, "DisplayOrderUpdated"),
        }
    }
}

impl std::fmt::Display for ImportErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportErrorType::ValidationError => write!(f, "ValidationError"),
            ImportErrorType::DuplicateCode => write!(f, "DuplicateCode"),
            ImportErrorType::InvalidFormat => write!(f, "InvalidFormat"),
            ImportErrorType::MissingRequiredField => write!(f, "MissingRequiredField"),
            ImportErrorType::DatabaseError => write!(f, "DatabaseError"),
        }
    }
}