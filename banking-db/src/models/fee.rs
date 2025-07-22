use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Database model for fee applications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeApplicationModel {
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub fee_type: String, // EventBased, Periodic
    pub fee_category: String,
    pub product_code: String,
    pub fee_code: String,
    pub description: String,
    pub amount: Decimal,
    pub currency: String,
    pub calculation_method: String,
    pub calculation_base_amount: Option<Decimal>,
    pub fee_rate: Option<Decimal>,
    pub trigger_event: String,
    pub status: String, // Applied, Pending, Waived, Reversed, Failed
    pub applied_at: DateTime<Utc>,
    pub value_date: NaiveDate,
    pub reversal_deadline: Option<DateTime<Utc>>,
    pub waived: bool,
    pub waived_by: Option<String>,
    pub waived_reason: Option<String>,
    pub applied_by: String,
    pub created_at: DateTime<Utc>,
}

/// Database model for fee waivers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeWaiverModel {
    pub waiver_id: Uuid,
    pub fee_application_id: Uuid,
    pub account_id: Uuid,
    pub waived_amount: Decimal,
    pub reason: String,
    pub waived_by: String,
    pub waived_at: DateTime<Utc>,
    pub approval_required: bool,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
}

/// Database model for fee processing jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeProcessingJobModel {
    pub job_id: Uuid,
    pub job_type: String, // DailyMaintenance, MonthlyMaintenance, etc.
    pub job_name: String,
    pub schedule_expression: String,
    pub target_fee_categories: String, // JSON array
    pub target_products: Option<String>, // JSON array
    pub processing_date: NaiveDate,
    pub status: String, // Scheduled, Running, Completed, Failed, Cancelled
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub accounts_processed: i32,
    pub fees_applied: i32,
    pub total_amount: Decimal,
    pub errors: Option<String>, // JSON array of error messages
    pub created_at: DateTime<Utc>,
}

/// Database model for enhanced account holds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeAccountHoldModel {
    pub hold_id: Uuid,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub hold_type: String, // UnclearedFunds, JudicialLien, LoanPledge, etc.
    pub reason: String,
    pub placed_by: String,
    pub placed_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String, // Active, Released, Expired, Cancelled, PartiallyReleased
    pub released_at: Option<DateTime<Utc>>,
    pub released_by: Option<String>,
    pub priority: String, // Critical, High, Medium, Low
    pub source_reference: Option<String>,
    pub automatic_release: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database model for hold release records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldReleaseRecordModel {
    pub release_record_id: Uuid,
    pub hold_id: Uuid,
    pub release_amount: Decimal,
    pub release_reason: String,
    pub released_by: String,
    pub released_at: DateTime<Utc>,
    pub is_partial_release: bool,
    pub remaining_amount: Decimal,
}

/// Database model for hold expiry jobs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldExpiryJobModel {
    pub job_id: Uuid,
    pub processing_date: NaiveDate,
    pub expired_holds_count: i32,
    pub total_released_amount: Decimal,
    pub processed_at: DateTime<Utc>,
    pub errors: Option<String>, // JSON array
}

/// Database model for balance calculations (for caching/audit)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceCalculationModel {
    pub calculation_id: Uuid,
    pub account_id: Uuid,
    pub current_balance: Decimal,
    pub available_balance: Decimal,
    pub overdraft_limit: Option<Decimal>,
    pub total_holds: Decimal,
    pub active_hold_count: i32,
    pub calculation_timestamp: DateTime<Utc>,
    pub hold_breakdown: String, // JSON serialized hold summary
}

/// Database model for product fee schedules (cached from Product Catalog)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFeeScheduleModel {
    pub schedule_id: Uuid,
    pub product_code: String,
    pub fee_schedule_data: String, // JSON serialized fee schedule
    pub effective_from: NaiveDate,
    pub effective_to: Option<NaiveDate>,
    pub last_updated: DateTime<Utc>,
    pub version: i32,
}

/// Database model for fee calculation cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeCalculationCacheModel {
    pub cache_id: Uuid,
    pub product_code: String,
    pub fee_code: String,
    pub calculation_key: String, // Hash of calculation parameters
    pub calculated_amount: Decimal,
    pub base_amount: Option<Decimal>,
    pub calculation_context: Option<String>,
    pub cached_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}