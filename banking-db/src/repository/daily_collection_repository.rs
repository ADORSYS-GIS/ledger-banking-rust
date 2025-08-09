use async_trait::async_trait;
use banking_api::BankingResult;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate, Duration, NaiveTime};

use crate::models::{
    CollectionAgentModel, CoverageAreaModel, PerformanceAlertModel,
    CollectionProgramModel, CustomerCollectionProfileModel,
    CollectionRecordModel, CollectionBatchModel, CollectionBatchRecordModel
};

// ======== Collection Agent Repository ========

#[async_trait]
pub trait CollectionAgentRepository: Send + Sync {
    /// Create a new collection agent
    async fn create(&self, agent: CollectionAgentModel) -> BankingResult<CollectionAgentModel>;
    
    /// Update existing collection agent
    async fn update(&self, agent: CollectionAgentModel) -> BankingResult<CollectionAgentModel>;
    
    /// Find collection agent by ID
    async fn find_by_id(&self, agent_id: Uuid) -> BankingResult<Option<CollectionAgentModel>>;
    
    /// Find collection agents by employee ID
    async fn find_by_person_id(&self, person_id: Uuid) -> BankingResult<Option<CollectionAgentModel>>;
    
    /// Find collection agents by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find active collection agents
    async fn find_active_agents(&self) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find collection agents by territory
    async fn find_by_territory(&self, territory_id: Uuid) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find collection agents by territory manager
    async fn find_by_territory_manager(&self, manager_person_id: Uuid) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find collection agents by license expiry range
    async fn find_by_license_expiry_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find agents with expiring licenses (within threshold days)
    async fn find_expiring_licenses(&self, within_days: i32) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find collection agents by performance criteria
    async fn find_by_performance_criteria(
        &self,
        min_collection_rate: Decimal,
        min_satisfaction_score: Decimal,
    ) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find top performing agents
    async fn find_top_performers(&self, limit: i64) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find underperforming agents
    async fn find_underperformers(&self, threshold_score: Decimal) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find agents by cash limit range
    async fn find_by_cash_limit_range(
        &self,
        min_limit: Decimal,
        max_limit: Decimal,
    ) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find agents by device type
    async fn find_by_device_type(&self, device_type: &str) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find agents with connectivity issues
    async fn find_connectivity_issues(&self) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find agents requiring security updates
    async fn find_requiring_security_updates(&self) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Update agent status with audit trail
    /// @param changed_by - References Person.person_id
    async fn update_status(
        &self,
        agent_id: Uuid,
        status: &str,
        reason_id: Option<Uuid>,
        changed_by_person_id: Uuid,
    ) -> BankingResult<()>;
    
    /// Update agent license information
    async fn update_license(
        &self,
        agent_id: Uuid,
        license_number: &str,
        license_expiry: NaiveDate,
    ) -> BankingResult<()>;
    
    /// Update agent cash limit
    async fn update_cash_limit(&self, agent_id: Uuid, cash_limit: Decimal) -> BankingResult<()>;
    
    /// Update agent performance metrics
    #[allow(clippy::too_many_arguments)]
    async fn update_performance_metrics(
        &self,
        agent_id: Uuid,
        collection_rate: Decimal,
        customer_satisfaction_score: Decimal,
        punctuality_score: Decimal,
        cash_handling_accuracy: Decimal,
        compliance_score: Decimal,
        total_collections: i64,
        total_amount_collected: Decimal,
        average_collection_time_minutes: i64,
        customer_retention_rate: Decimal,
        route_efficiency: Decimal,
    ) -> BankingResult<()>;
    
    /// Update agent monthly targets
    async fn update_monthly_targets(
        &self,
        agent_id: Uuid,
        collection_target: Decimal,
        customer_target: i32,
        satisfaction_target: Decimal,
        punctuality_target: Decimal,
        accuracy_target: Decimal,
    ) -> BankingResult<()>;
    
    /// Update agent device information
    #[allow(clippy::too_many_arguments)]
    async fn update_device_info(
        &self,
        agent_id: Uuid,
        device_id: Uuid,
        device_external_id: &str,
        device_type: &str,
        model: &str,
        os_version: &str,
        app_version: &str,
        last_sync: Option<DateTime<Utc>>,
        battery_level: Option<f32>,
        connectivity_status: &str,
    ) -> BankingResult<()>;
    
    /// Update agent security features
    #[allow(clippy::too_many_arguments)]
    async fn update_security_features(
        &self,
        agent_id: Uuid,
        biometric_enabled: bool,
        pin_protection: bool,
        encryption_enabled: bool,
        remote_wipe_enabled: bool,
        certificate_installed: bool,
        last_security_scan: Option<DateTime<Utc>>,
    ) -> BankingResult<()>;
    
    /// Update agent contact information
    #[allow(clippy::too_many_arguments)]
    async fn update_contact_info(
        &self,
        agent_id: Uuid,
        phone_primary: &str,
        phone_secondary: Option<&str>,
        email: Option<&str>,
        emergency_name: Option<&str>,
        emergency_relationship: Option<&str>,
        emergency_phone: Option<&str>,
    ) -> BankingResult<()>;
    
    /// Assign territory to agent
    async fn assign_territory(
        &self,
        agent_id: Uuid,
        territory_id: Uuid,
        territory_name: &str,
        customer_count: i32,
        route_optimization_enabled: bool,
        manager_person_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Delete agent (soft delete by status change)
    async fn delete(&self, agent_id: Uuid, deleted_by_person_id: Uuid, reason_id: Option<Uuid>) -> BankingResult<()>;
    
    /// Get agent performance statistics
    async fn get_agent_statistics(&self, agent_id: Uuid) -> BankingResult<Option<AgentStatistics>>;
    
    /// Get all agent statistics
    async fn get_all_agent_statistics(&self) -> BankingResult<Vec<AgentStatistics>>;
    
    /// Bulk update agent status
    async fn bulk_update_status(
        &self,
        agent_ids: Vec<Uuid>,
        status: &str,
        changed_by_person_id: Uuid,
    ) -> BankingResult<i64>;
    
    /// Search agents by name pattern
    async fn search_by_name(&self, name_pattern: &str) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Find agents with pagination
    async fn find_with_pagination(
        &self,
        offset: i64,
        limit: i64,
        status_filter: Option<&str>,
        territory_filter: Option<Uuid>,
    ) -> BankingResult<Vec<CollectionAgentModel>>;
    
    /// Count total agents
    async fn count_all(&self) -> BankingResult<i64>;
    
    /// Count agents by status
    async fn count_by_status(&self, status: &str) -> BankingResult<i64>;
    
    /// Count agents by territory
    async fn count_by_territory(&self, territory_id: Uuid) -> BankingResult<i64>;
    
    // ======== Coverage Area Operations ========
    
    /// Create coverage area
    async fn create_coverage_area(&self, area: CoverageAreaModel) -> BankingResult<CoverageAreaModel>;
    
    /// Update coverage area
    async fn update_coverage_area(&self, area: CoverageAreaModel) -> BankingResult<CoverageAreaModel>;
    
    /// Find coverage area by ID
    async fn find_coverage_area_by_id(&self, area_id: Uuid) -> BankingResult<Option<CoverageAreaModel>>;
    
    /// Find coverage areas by territory
    async fn find_coverage_areas_by_territory(&self, territory_id: Uuid) -> BankingResult<Vec<CoverageAreaModel>>;
    
    /// Find coverage areas by type
    async fn find_coverage_areas_by_type(&self, area_type: &str) -> BankingResult<Vec<CoverageAreaModel>>;
    
    /// Delete coverage area
    async fn delete_coverage_area(&self, area_id: Uuid) -> BankingResult<()>;
    
    // ======== Performance Alert Operations ========
    
    /// Create performance alert
    async fn create_performance_alert(&self, alert: PerformanceAlertModel) -> BankingResult<PerformanceAlertModel>;
    
    /// Find performance alerts by agent
    async fn find_alerts_by_agent(&self, agent_id: Uuid) -> BankingResult<Vec<PerformanceAlertModel>>;
    
    /// Find unacknowledged alerts
    async fn find_unacknowledged_alerts(&self) -> BankingResult<Vec<PerformanceAlertModel>>;
    
    /// Find alerts by severity
    async fn find_alerts_by_severity(&self, severity: &str) -> BankingResult<Vec<PerformanceAlertModel>>;
    
    /// Acknowledge alert
    async fn acknowledge_alert(&self, alert_id: Uuid, acknowledged_at: DateTime<Utc>) -> BankingResult<()>;
    
    /// Resolve alert
    async fn resolve_alert(&self, alert_id: Uuid, resolved_at: DateTime<Utc>) -> BankingResult<()>;
    
    /// Delete performance alert
    async fn delete_performance_alert(&self, alert_id: Uuid) -> BankingResult<()>;
}

/// Agent performance statistics
#[derive(Debug, Clone)]
pub struct AgentStatistics {
    pub agent_id: Uuid,
    pub agent_name: String,
    pub employee_id: Uuid,
    pub status: String,
    pub territory_id: Uuid,
    pub territory_name: String,
    pub total_assigned_customers: i64,
    pub active_customers: i64,
    pub total_collections_ytd: i64,
    pub total_amount_collected_ytd: Decimal,
    pub collection_rate: Decimal,
    pub customer_satisfaction_score: Decimal,
    pub punctuality_score: Decimal,
    pub cash_handling_accuracy: Decimal,
    pub compliance_score: Decimal,
    pub average_collection_time: Duration,
    pub customer_retention_rate: Decimal,
    pub route_efficiency: Decimal,
    pub target_achievement_rate: Decimal,
    pub outstanding_alerts: i64,
    pub critical_alerts: i64,
    pub license_expiry_date: NaiveDate,
    pub days_until_license_expiry: i32,
    pub last_collection_date: Option<NaiveDate>,
    pub device_last_sync: Option<DateTime<Utc>>,
    pub security_scan_status: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// ======== Collection Program Repository ========

#[async_trait]
pub trait CollectionProgramRepository: Send + Sync {
    /// Create a new collection program record
    async fn create(&self, program: CollectionProgramModel) -> BankingResult<CollectionProgramModel>;
    
    /// Update existing collection program record
    async fn update(&self, program: CollectionProgramModel) -> BankingResult<CollectionProgramModel>;
    
    /// Find collection program by ID
    async fn find_by_id(&self, program_id: Uuid) -> BankingResult<Option<CollectionProgramModel>>;
    
    /// Find collection programs by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find active collection programs
    async fn find_active_programs(&self) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs by type
    async fn find_by_program_type(&self, program_type: &str) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs created by user
    async fn find_by_created_by(&self, created_by_person_id: Uuid) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs within date range
    async fn find_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs by duration range
    async fn find_by_duration_range(
        &self,
        min_days: i32,
        max_days: i32,
    ) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs by amount range
    async fn find_by_amount_range(
        &self,
        min_amount: Decimal,
        max_amount: Decimal,
    ) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs requiring review (older than threshold)
    async fn find_requiring_review(&self, threshold_date: NaiveDate) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find expiring programs (end date approaching)
    async fn find_expiring_programs(&self, within_days: i32) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Update program status with audit trail
    /// @param changed_by - References Person.person_id
    async fn update_status(
        &self,
        program_id: Uuid,
        status: &str,
        reason_id: Option<Uuid>,
        changed_by_person_id: Uuid,
    ) -> BankingResult<()>;
    
    /// Update program end date
    async fn update_end_date(&self, program_id: Uuid, end_date: Option<NaiveDate>) -> BankingResult<()>;
    
    /// Update program interest rate
    async fn update_interest_rate(&self, program_id: Uuid, interest_rate: Option<Decimal>) -> BankingResult<()>;
    
    /// Update program amount limits
    async fn update_amount_limits(
        &self,
        program_id: Uuid,
        minimum_amount: Decimal,
        maximum_amount: Decimal,
    ) -> BankingResult<()>;
    
    /// Delete program (soft delete by status change)
    async fn delete(&self, program_id: Uuid, deleted_by_person_id: Uuid, reason_id: Option<Uuid>) -> BankingResult<()>;
    
    /// Get program statistics
    async fn get_program_statistics(&self, program_id: Uuid) -> BankingResult<ProgramStatistics>;
    
    /// Get all program statistics
    async fn get_all_program_statistics(&self) -> BankingResult<Vec<ProgramStatistics>>;
    
    /// Find programs with specific graduation criteria
    async fn find_by_graduation_criteria(
        &self,
        auto_graduation_enabled: bool,
        target_achievement_required: bool,
    ) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs by fee structure
    async fn find_by_fee_structure(&self, fee_frequency: &str) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Bulk update programs
    async fn bulk_update_status(
        &self,
        program_ids: Vec<Uuid>,
        status: &str,
        changed_by_person_id: Uuid,
    ) -> BankingResult<i64>;
    
    /// Search programs by name pattern
    async fn search_by_name(&self, name_pattern: &str) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Find programs with pagination
    async fn find_with_pagination(
        &self,
        offset: i64,
        limit: i64,
    ) -> BankingResult<Vec<CollectionProgramModel>>;
    
    /// Count total programs
    async fn count_all(&self) -> BankingResult<i64>;
    
    /// Count programs by status
    async fn count_by_status(&self, status: &str) -> BankingResult<i64>;
}

/// Statistics for a collection program
#[derive(Debug, Clone)]
pub struct ProgramStatistics {
    pub program_id: Uuid,
    pub total_enrolled_customers: i64,
    pub active_customers: i64,
    pub graduated_customers: i64,
    pub suspended_customers: i64,
    pub defaulted_customers: i64,
    pub total_collections: i64,
    pub total_amount_collected: Decimal,
    pub average_collection_amount: Decimal,
    pub collection_rate: Decimal,
    pub customer_retention_rate: Decimal,
    pub average_program_duration: f64,
    pub graduation_rate: Decimal,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// ======== Customer Collection Profile Repository ========

#[async_trait]
pub trait CustomerCollectionProfileRepository: Send + Sync {
    /// Create a new customer collection profile record
    async fn create(&self, profile: CustomerCollectionProfileModel) -> BankingResult<CustomerCollectionProfileModel>;
    
    /// Update existing customer collection profile record
    async fn update(&self, profile: CustomerCollectionProfileModel) -> BankingResult<CustomerCollectionProfileModel>;
    
    /// Find customer collection profile by customer ID
    async fn find_by_customer_id(&self, customer_id: Uuid) -> BankingResult<Option<CustomerCollectionProfileModel>>;
    
    /// Find profiles by program ID
    async fn find_by_program_id(&self, program_id: Uuid) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by account ID
    async fn find_by_account_id(&self, account_id: Uuid) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by assigned agent
    async fn find_by_assigned_agent(&self, agent_id: Uuid) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find active profiles (status = Active)
    async fn find_active_profiles(&self) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles enrolled within date range
    async fn find_by_enrollment_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by daily amount range
    async fn find_by_daily_amount_range(
        &self,
        min_amount: Decimal,
        max_amount: Decimal,
    ) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by location type
    async fn find_by_location_type(&self, location_type: &str) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by performance criteria
    async fn find_by_performance_criteria(
        &self,
        min_collection_rate: Decimal,
        min_performance_score: Decimal,
    ) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles eligible for graduation
    async fn find_graduation_eligible(&self) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles requiring review
    async fn find_requiring_review(&self, review_date: NaiveDate) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles by reliability rating
    async fn find_by_reliability_rating(&self, rating: &str) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles with missed collections above threshold
    async fn find_with_high_missed_collections(&self, threshold: i32) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles with low performance scores
    async fn find_low_performance(&self, threshold_score: Decimal) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Update profile status with audit trail
    /// @param changed_by - References Person.person_id
    async fn update_status(
        &self,
        customer_id: Uuid,
        status: &str,
        reason_id: Option<Uuid>,
        changed_by_person_id: Uuid,
    ) -> BankingResult<()>;
    
    /// Update daily collection amount
    async fn update_daily_amount(&self, customer_id: Uuid, daily_amount: Decimal) -> BankingResult<()>;
    
    /// Update assigned agent
    async fn update_assigned_agent(&self, customer_id: Uuid, agent_id: Uuid, changed_by_person_id: Uuid) -> BankingResult<()>;
    
    /// Update performance metrics
    #[allow(clippy::too_many_arguments)]
    async fn update_performance_metrics(
        &self,
        customer_id: Uuid,
        collection_rate: Decimal,
        total_collections: i64,
        total_amount_collected: Decimal,
        consecutive_collections: i32,
        missed_collections: i32,
        performance_score: Decimal,
        reliability_rating: &str,
    ) -> BankingResult<()>;
    
    /// Update graduation progress
    async fn update_graduation_progress(
        &self,
        customer_id: Uuid,
        current_balance: Decimal,
        days_in_program: i32,
        collection_consistency_rate: Decimal,
        graduation_eligible: bool,
    ) -> BankingResult<()>;
    
    /// Update location information
    #[allow(clippy::too_many_arguments)]
    async fn update_location(
        &self,
        customer_id: Uuid,
        location_type: &str,
        street_address: &str,
        city: &str,
        state_province: &str,
        postal_code: &str,
        country: &str,
        gps_latitude: Option<f64>,
        gps_longitude: Option<f64>,
        access_instructions: Option<&str>,
    ) -> BankingResult<()>;
    
    /// Update collection schedule
    async fn update_collection_schedule(
        &self,
        customer_id: Uuid,
        days_of_week: &str,
        collection_time: NaiveTime,
        timezone: &str,
        holiday_handling: &str,
    ) -> BankingResult<()>;
    
    /// Mark profile as graduated
    async fn graduate_profile(
        &self,
        customer_id: Uuid,
        graduation_date: NaiveDate,
        reason_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Delete profile (soft delete by status change)
    async fn delete(&self, customer_id: Uuid, deleted_by_person_id: Uuid, reason_id: Option<Uuid>) -> BankingResult<()>;
    
    /// Bulk update agent assignments
    async fn bulk_update_agent(
        &self,
        customer_ids: Vec<Uuid>,
        new_agent_id: Uuid,
        changed_by_person_id: Uuid,
    ) -> BankingResult<i64>;
    
    /// Bulk update status
    async fn bulk_update_status(
        &self,
        customer_ids: Vec<Uuid>,
        status: &str,
        changed_by_person_id: Uuid,
    ) -> BankingResult<i64>;
    
    /// Find profiles by GPS location radius
    async fn find_by_location_radius(
        &self,
        center_latitude: f64,
        center_longitude: f64,
        radius_meters: f64,
    ) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles scheduled for collection on specific days
    async fn find_by_collection_days(&self, days_of_week: &str) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Find profiles with overdue reviews
    async fn find_overdue_reviews(&self, current_date: NaiveDate) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Get customer collection statistics
    async fn get_customer_statistics(&self, customer_id: Uuid) -> BankingResult<Option<CustomerStatistics>>;
    
    /// Find profiles with pagination
    async fn find_with_pagination(
        &self,
        offset: i64,
        limit: i64,
        status_filter: Option<&str>,
        program_id_filter: Option<Uuid>,
    ) -> BankingResult<Vec<CustomerCollectionProfileModel>>;
    
    /// Count total profiles
    async fn count_all(&self) -> BankingResult<i64>;
    
    /// Count profiles by status
    async fn count_by_status(&self, status: &str) -> BankingResult<i64>;
    
    /// Count profiles by program
    async fn count_by_program(&self, program_id: Uuid) -> BankingResult<i64>;
    
    /// Count profiles by agent
    async fn count_by_agent(&self, agent_id: Uuid) -> BankingResult<i64>;
}

/// Statistics for a customer's collection profile
#[derive(Debug, Clone)]
pub struct CustomerStatistics {
    pub customer_id: Uuid,
    pub enrollment_date: NaiveDate,
    pub days_in_program: i32,
    pub total_collections: i64,
    pub successful_collections: i64,
    pub missed_collections: i32,
    pub total_amount_collected: Decimal,
    pub average_collection_amount: Decimal,
    pub collection_rate: Decimal,
    pub current_balance: Decimal,
    pub performance_score: Decimal,
    pub reliability_rating: String,
    pub consecutive_collections: i32,
    pub longest_streak: i32,
    pub graduation_eligible: bool,
    pub target_completion_percentage: Option<Decimal>,
    pub estimated_graduation_date: Option<NaiveDate>,
    pub last_collection_date: Option<NaiveDate>,
    pub next_scheduled_collection: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

// ======== Collection Record Repository ========

#[async_trait]
pub trait CollectionRecordRepository: Send + Sync {
    /// Create a new collection record
    async fn create(&self, record: CollectionRecordModel) -> BankingResult<CollectionRecordModel>;
    
    /// Update existing collection record
    async fn update(&self, record: CollectionRecordModel) -> BankingResult<CollectionRecordModel>;
    
    /// Find collection record by ID
    async fn find_by_id(&self, record_id: Uuid) -> BankingResult<Option<CollectionRecordModel>>;
    
    /// Find collection records by customer ID
    async fn find_by_customer_id(&self, customer_id: Uuid) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by agent ID
    async fn find_by_agent_id(&self, agent_id: Uuid) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by program ID
    async fn find_by_program_id(&self, program_id: Uuid) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by account ID
    async fn find_by_account_id(&self, account_id: Uuid) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by status
    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by date range
    async fn find_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by agent and date
    async fn find_by_agent_and_date(
        &self,
        agent_id: Uuid,
        collection_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by customer and date range
    async fn find_by_customer_and_date_range(
        &self,
        customer_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by amount range
    async fn find_by_amount_range(
        &self,
        min_amount: Decimal,
        max_amount: Decimal,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by collection method
    async fn find_by_collection_method(&self, method: &str) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by location radius
    async fn find_by_location_radius(
        &self,
        center_latitude: f64,
        center_longitude: f64,
        radius_meters: f64,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find pending collection records
    async fn find_pending_records(&self) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find failed collection records
    async fn find_failed_records(&self) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find unprocessed collection records (pending/under review)
    async fn find_unprocessed_records(&self) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records requiring review
    async fn find_requiring_review(&self) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Find collection records by receipt number
    async fn find_by_receipt_number(&self, receipt_number: &str) -> BankingResult<Option<CollectionRecordModel>>;
    
    /// Update collection record status
    async fn update_status(
        &self,
        record_id: Uuid,
        status: &str,
        processed_at: Option<DateTime<Utc>>,
        reason_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Update collection record processing timestamp
    async fn update_processed_at(&self, record_id: Uuid, processed_at: DateTime<Utc>) -> BankingResult<()>;
    
    /// Add verification data to collection record
    async fn update_verification_data(
        &self,
        record_id: Uuid,
        customer_signature: Option<&str>,
        agent_verification_code: Option<&str>,
        biometric_method: Option<&str>,
        confidence_level: Option<f64>,
        verification_timestamp: DateTime<Utc>,
    ) -> BankingResult<()>;
    
    /// Add photo evidence to collection record
    async fn update_photo_evidence(
        &self,
        record_id: Uuid,
        customer_photo_hash: Option<&str>,
        receipt_photo_hash: Option<&str>,
        location_photo_hash: Option<&str>,
        photo_timestamp: DateTime<Utc>,
    ) -> BankingResult<()>;
    
    /// Reverse a collection record
    async fn reverse_collection(
        &self,
        record_id: Uuid,
        reason_id: Uuid,
        authorized_by_person_id: Uuid,
    ) -> BankingResult<()>;
    
    /// Delete collection record (soft delete by status change)
    async fn delete(&self, record_id: Uuid, deleted_by_person_id: Uuid, reason_id: Option<Uuid>) -> BankingResult<()>;
    
    /// Get collection statistics for date range
    async fn get_collection_statistics(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        agent_id: Option<Uuid>,
        program_id: Option<Uuid>,
    ) -> BankingResult<CollectionStatistics>;
    
    /// Get daily collection summary
    async fn get_daily_summary(&self, collection_date: NaiveDate) -> BankingResult<DailyCollectionSummary>;
    
    /// Find duplicate collections (same customer, date, amount)
    async fn find_duplicate_collections(
        &self,
        customer_id: Uuid,
        collection_date: NaiveDate,
        amount: Decimal,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Bulk update collection status
    async fn bulk_update_status(
        &self,
        record_ids: Vec<Uuid>,
        status: &str,
        processed_at: Option<DateTime<Utc>>,
    ) -> BankingResult<i64>;
    
    /// Find collection records with pagination
    async fn find_with_pagination(
        &self,
        offset: i64,
        limit: i64,
        status_filter: Option<&str>,
        agent_id_filter: Option<Uuid>,
        date_filter: Option<NaiveDate>,
    ) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Count total collection records
    async fn count_all(&self) -> BankingResult<i64>;
    
    /// Count collection records by status
    async fn count_by_status(&self, status: &str) -> BankingResult<i64>;
    
    /// Count collection records by agent
    async fn count_by_agent(&self, agent_id: Uuid) -> BankingResult<i64>;
    
    /// Count collection records by date
    async fn count_by_date(&self, collection_date: NaiveDate) -> BankingResult<i64>;
    
    // ======== Collection Batch Operations ========
    
    /// Create a new collection batch
    async fn create_batch(&self, batch: CollectionBatchModel) -> BankingResult<CollectionBatchModel>;
    
    /// Update existing collection batch
    async fn update_batch(&self, batch: CollectionBatchModel) -> BankingResult<CollectionBatchModel>;
    
    /// Find collection batch by ID
    async fn find_batch_by_id(&self, batch_id: Uuid) -> BankingResult<Option<CollectionBatchModel>>;
    
    /// Find collection batches by agent and date
    async fn find_batches_by_agent_date(
        &self,
        agent_id: Uuid,
        collection_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionBatchModel>>;
    
    /// Find collection batches by status
    async fn find_batches_by_status(&self, status: &str) -> BankingResult<Vec<CollectionBatchModel>>;
    
    /// Find collection batches requiring reconciliation
    async fn find_batches_requiring_reconciliation(&self) -> BankingResult<Vec<CollectionBatchModel>>;
    
    /// Add collection records to batch
    async fn add_records_to_batch(
        &self,
        batch_id: Uuid,
        record_ids: Vec<Uuid>,
    ) -> BankingResult<Vec<CollectionBatchRecordModel>>;
    
    /// Find collection records in batch
    async fn find_records_in_batch(&self, batch_id: Uuid) -> BankingResult<Vec<CollectionRecordModel>>;
    
    /// Update batch reconciliation data
    #[allow(clippy::too_many_arguments)]
    async fn update_batch_reconciliation(
        &self,
        batch_id: Uuid,
        expected_amount: Decimal,
        actual_amount: Decimal,
        variance: Decimal,
        variance_reason: Option<&str>,
        reconciled_by_person_id: Uuid,
        adjustment_required: bool,
    ) -> BankingResult<()>;
    
    /// Update batch status
    async fn update_batch_status(
        &self,
        batch_id: Uuid,
        status: &str,
        processed_at: Option<DateTime<Utc>>,
    ) -> BankingResult<()>;
    
    /// Delete batch (soft delete by status change)
    async fn delete_batch(&self, batch_id: Uuid, deleted_by_person_id: Uuid) -> BankingResult<()>;
}

/// Collection statistics for analysis
#[derive(Debug, Clone)]
pub struct CollectionStatistics {
    pub total_collections: i64,
    pub successful_collections: i64,
    pub failed_collections: i64,
    pub pending_collections: i64,
    pub total_amount: Decimal,
    pub average_collection_amount: Decimal,
    pub collection_success_rate: Decimal,
    pub unique_customers: i64,
    pub unique_agents: i64,
    pub most_common_method: String,
    pub total_cash_collections: i64,
    pub total_digital_collections: i64,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
}

/// Daily collection summary
#[derive(Debug, Clone)]
pub struct DailyCollectionSummary {
    pub collection_date: NaiveDate,
    pub total_scheduled: i64,
    pub total_completed: i64,
    pub total_pending: i64,
    pub total_failed: i64,
    pub total_amount_collected: Decimal,
    pub completion_rate: Decimal,
    pub average_collection_amount: Decimal,
    pub active_agents: i64,
    pub cash_collections: i64,
    pub digital_collections: i64,
    pub collections_requiring_review: i64,
    pub variance_amount: Decimal,
    pub top_performing_agent: Option<Uuid>,
    pub areas_needing_attention: Vec<String>,
}