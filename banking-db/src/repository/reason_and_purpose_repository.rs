use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use banking_api::BankingResult;

use crate::models::ReasonAndPurpose as ReasonAndPurposeModel;
use banking_api::domain::{ReasonCategory, ReasonContext, ReasonSeverity};

/// Repository trait for ReasonAndPurpose data access operations
#[async_trait]
pub trait ReasonAndPurposeRepository: Send + Sync {
    
    // ============================================================================
    // CRUD OPERATIONS
    // ============================================================================
    
    /// Create a new reason
    async fn create(&self, reason: ReasonAndPurposeModel) -> BankingResult<ReasonAndPurposeModel>;
    
    /// Find reason by ID
    async fn find_by_id(&self, reason_id: Uuid) -> BankingResult<Option<ReasonAndPurposeModel>>;
    
    /// Find reason by unique code
    async fn find_by_code(&self, code: &str) -> BankingResult<Option<ReasonAndPurposeModel>>;
    
    /// Update existing reason
    async fn update(&self, reason: ReasonAndPurposeModel) -> BankingResult<ReasonAndPurposeModel>;
    
    /// Delete reason (hard delete - use carefully)
    async fn delete(&self, reason_id: Uuid) -> BankingResult<()>;
    
    /// Soft delete (deactivate) reason
    async fn deactivate(&self, reason_id: Uuid, deactivated_by: Uuid) -> BankingResult<()>;
    
    /// Reactivate previously deactivated reason
    async fn reactivate(&self, reason_id: Uuid, reactivated_by: Uuid) -> BankingResult<()>;
    
    // ============================================================================
    // QUERY OPERATIONS
    // ============================================================================
    
    /// Get all active reasons
    async fn find_all_active(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Find reasons by category
    async fn find_by_category(&self, category: ReasonCategory) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Find reasons by context
    async fn find_by_context(&self, context: ReasonContext) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Find reasons by category and context (most common query)
    async fn find_by_category_and_context(
        &self,
        category: ReasonCategory,
        context: ReasonContext,
    ) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Find reasons by severity
    async fn find_by_severity(&self, severity: ReasonSeverity) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Search reasons by content (supports partial matching across all language fields)
    async fn search_by_content(
        &self,
        search_term: &str,
        language_codes: Option<Vec<[u8; 3]>>,
    ) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get reasons for display ordered by display_order
    async fn find_for_display(
        &self,
        category: Option<ReasonCategory>,
        context: Option<ReasonContext>,
        active_only: bool,
    ) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    // ============================================================================
    // COMPLIANCE-SPECIFIC QUERIES
    // ============================================================================
    
    /// Get all compliance reasons that require regulatory reporting
    async fn find_reportable_compliance_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get reasons that trigger Suspicious Activity Reports
    async fn find_sar_triggering_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get reasons that trigger Currency Transaction Reports
    async fn find_ctr_triggering_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get AML/CTF related reasons
    async fn find_aml_ctf_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get KYC related reasons
    async fn find_kyc_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get reasons applicable to specific jurisdiction
    async fn find_by_jurisdiction(&self, jurisdiction_code: [u8; 2]) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    /// Get reasons requiring management escalation
    async fn find_escalation_required_reasons(&self) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    // ============================================================================
    // USAGE STATISTICS AND ANALYTICS
    // ============================================================================
    
    /// Get usage count for a specific reason within date range
    async fn get_usage_count(
        &self,
        reason_id: Uuid,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<u64>;
    
    /// Get usage statistics for a reason
    async fn get_usage_statistics(
        &self,
        reason_id: Uuid,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<ReasonUsageStatistics>;
    
    /// Get most frequently used reasons by category
    async fn get_top_used_reasons_by_category(
        &self,
        category: ReasonCategory,
        limit: i32,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> BankingResult<Vec<ReasonUsageStatistics>>;
    
    /// Get least used or unused reasons (for cleanup)
    async fn find_unused_reasons(
        &self,
        since_date: NaiveDate,
    ) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    // ============================================================================
    // CHANGE TRACKING AND AUDIT
    // ============================================================================
    
    /// Record reason usage (for analytics)
    async fn record_usage(
        &self,
        reason_id: Uuid,
        context: ReasonContext,
        used_by: &str,
        additional_context: Option<&str>,
    ) -> BankingResult<()>;
    
    /// Get change history for a reason
    async fn get_change_history(&self, reason_id: Uuid) -> BankingResult<Vec<ReasonChangeRecord>>;
    
    /// Record reason change
    async fn record_change(
        &self,
        change_record: ReasonChangeRecord,
    ) -> BankingResult<ReasonChangeRecord>;
    
    // ============================================================================
    // VALIDATION OPERATIONS
    // ============================================================================
    
    /// Check if reason code already exists
    async fn code_exists(&self, code: &str, exclude_id: Option<Uuid>) -> BankingResult<bool>;
    
    /// Validate reason is active and usable
    async fn is_active(&self, reason_id: Uuid) -> BankingResult<bool>;
    
    /// Validate reason is appropriate for context
    async fn is_valid_for_context(
        &self,
        reason_id: Uuid,
        context: ReasonContext,
    ) -> BankingResult<bool>;
    
    /// Get validation rules for a reason
    async fn get_validation_rules(&self, reason_id: Uuid) -> BankingResult<Option<ReasonValidationRules>>;
    
    // ============================================================================
    // BULK OPERATIONS
    // ============================================================================
    
    /// Bulk insert reasons (for imports or migrations)
    async fn bulk_insert(&self, reasons: Vec<ReasonAndPurposeModel>) -> BankingResult<BulkOperationResult>;
    
    /// Bulk update display orders
    async fn bulk_update_display_orders(
        &self,
        category: ReasonCategory,
        order_updates: Vec<(Uuid, i32)>,
        updated_by_person_id: &str,
    ) -> BankingResult<()>;
    
    /// Bulk activate/deactivate reasons
    async fn bulk_update_status(
        &self,
        reason_ids: Vec<Uuid>,
        is_active: bool,
        updated_by_person_id: &str,
    ) -> BankingResult<BulkOperationResult>;
    
    // ============================================================================
    // LOCALIZATION OPERATIONS
    // ============================================================================
    
    /// Update localized content for a specific language
    async fn update_localized_content(
        &self,
        reason_id: Uuid,
        language_code: [u8; 3],
        content: &str,
        updated_by_person_id: &str,
    ) -> BankingResult<()>;
    
    /// Remove localized content for a language
    async fn remove_localized_content(
        &self,
        reason_id: Uuid,
        language_code: [u8; 3],
        updated_by_person_id: &str,
    ) -> BankingResult<()>;
    
    /// Get reasons with content in specific languages
    async fn find_with_languages(
        &self,
        language_codes: &[[u8; 3]],
        category: Option<ReasonCategory>,
        context: Option<ReasonContext>,
    ) -> BankingResult<Vec<LocalizedReasonModel>>;
    
    /// Find reasons missing localization for a language
    async fn find_missing_localization(
        &self,
        language_code: [u8; 3],
        category: Option<ReasonCategory>,
    ) -> BankingResult<Vec<ReasonAndPurposeModel>>;
    
    // ============================================================================
    // UTILITY OPERATIONS
    // ============================================================================
    
    /// Get total count of reasons
    async fn count_total(&self) -> BankingResult<i64>;
    
    /// Get count by category
    async fn count_by_category(&self, category: ReasonCategory) -> BankingResult<i64>;
    
    /// Get count by context
    async fn count_by_context(&self, context: ReasonContext) -> BankingResult<i64>;
    
    /// Check database health and constraints
    async fn validate_data_integrity(&self) -> BankingResult<DataIntegrityReport>;
    
    /// List all categories in use
    async fn get_categories_in_use(&self) -> BankingResult<Vec<ReasonCategory>>;
    
    /// List all contexts in use
    async fn get_contexts_in_use(&self) -> BankingResult<Vec<ReasonContext>>;
}

/// Usage statistics for a reason
#[derive(Debug, Clone)]
pub struct ReasonUsageStatistics {
    pub reason_id: Uuid,
    pub reason_code: String,
    pub usage_count: u64,
    pub first_used: Option<chrono::DateTime<chrono::Utc>>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub contexts_used: Vec<ReasonContext>,
    pub peak_usage_month: Option<String>,
    pub average_monthly_usage: f64,
}

/// Change record for audit trail
#[derive(Debug, Clone)]
pub struct ReasonChangeRecord {
    pub change_id: Uuid,
    pub reason_id: Uuid,
    pub change_type: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: String,
    pub changed_at: chrono::DateTime<chrono::Utc>,
    pub change_reason: Option<String>,
    pub additional_context: Option<String>,
}

/// Validation rules for a reason
#[derive(Debug, Clone)]
pub struct ReasonValidationRules {
    pub reason_id: Uuid,
    pub requires_additional_details: bool,
    pub min_details_length: Option<usize>,
    pub max_details_length: Option<usize>,
    pub valid_contexts: Vec<ReasonContext>,
    pub requires_authorization: bool,
    pub authorization_level: Option<String>,
    pub compliance_checks_required: bool,
}

/// Result of bulk operations
#[derive(Debug, Clone)]
pub struct BulkOperationResult {
    pub total_processed: usize,
    pub successful: usize,
    pub failed: usize,
    pub errors: Vec<BulkOperationError>,
}

#[derive(Debug, Clone)]
pub struct BulkOperationError {
    pub item_index: usize,
    pub item_id: Option<Uuid>,
    pub error_message: String,
    pub error_code: String,
}

/// Localized reason model for multi-language support
#[derive(Debug, Clone)]
pub struct LocalizedReasonModel {
    pub reason_id: Uuid,
    pub code: String,
    pub category: ReasonCategory,
    pub context: ReasonContext,
    pub content: String,
    pub language_code: [u8; 3],
    pub requires_details: bool,
    pub severity: Option<ReasonSeverity>,
    pub display_order: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Data integrity report
#[derive(Debug, Clone)]
pub struct DataIntegrityReport {
    pub total_reasons: i64,
    pub active_reasons: i64,
    pub inactive_reasons: i64,
    pub orphaned_reasons: i64,
    pub duplicate_codes: Vec<String>,
    pub invalid_language_codes: Vec<Uuid>,
    pub missing_primary_content: Vec<Uuid>,
    pub constraint_violations: Vec<ConstraintViolation>,
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub reason_id: Uuid,
    pub violation_type: String,
    pub description: String,
    pub severity: String,
}