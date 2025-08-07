use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    BankingResult,
    domain::{
        CollectionProgram, DailyCollectionProgramStatus as ProgramStatus, CustomerCollectionProfile, DailyCollectionStatus as CollectionStatus,
        CollectionRecord, CollectionRecordStatus, CollectionAgent, DailyCollectionAgentStatus as AgentStatus,
        CollectionBatch, CollectionMethod
    },
};

/// Service for managing daily collection banking operations
#[async_trait]
pub trait DailyCollectionService: Send + Sync {
    // ======== Collection Program Management ========
    
    /// Create a new collection program
    async fn create_collection_program(&self, program: CollectionProgram) -> BankingResult<CollectionProgram>;
    
    /// Update an existing collection program
    async fn update_collection_program(&self, program_id: Uuid, program: CollectionProgram) -> BankingResult<CollectionProgram>;
    
    /// Find collection program by ID
    async fn get_collection_program(&self, program_id: Uuid) -> BankingResult<Option<CollectionProgram>>;
    
    /// Find collection programs by status
    async fn find_programs_by_status(&self, status: ProgramStatus) -> BankingResult<Vec<CollectionProgram>>;
    
    /// Find active collection programs
    async fn find_active_programs(&self) -> BankingResult<Vec<CollectionProgram>>;
    
    /// Deactivate collection program
    async fn deactivate_program(&self, program_id: Uuid, reason_id: Option<Uuid>) -> BankingResult<()>;
    
    // ======== Customer Enrollment and Management ========
    
    /// Enroll customer in a collection program
    async fn enroll_customer(
        &self,
        customer_id: Uuid,
        program_id: Uuid,
        profile: CustomerCollectionProfile,
    ) -> BankingResult<CustomerCollectionProfile>;
    
    /// Update customer collection program details
    async fn update_customer_program(
        &self,
        customer_id: Uuid,
        profile: CustomerCollectionProfile,
    ) -> BankingResult<CustomerCollectionProfile>;
    
    /// Find customer collection profile
    async fn get_customer_collection_profile(&self, customer_id: Uuid) -> BankingResult<Option<CustomerCollectionProfile>>;
    
    /// Get customer collection history for date range
    async fn get_customer_collection_history(
        &self,
        customer_id: Uuid,
        date_range: (NaiveDate, NaiveDate),
    ) -> BankingResult<Vec<CollectionRecord>>;
    
    /// Find customers by collection status
    async fn find_customers_by_status(&self, status: CollectionStatus) -> BankingResult<Vec<CustomerCollectionProfile>>;
    
    /// Find customers by program
    async fn find_customers_by_program(&self, program_id: Uuid) -> BankingResult<Vec<CustomerCollectionProfile>>;
    
    /// Update customer collection status
    async fn update_customer_status(
        &self,
        customer_id: Uuid,
        status: CollectionStatus,
        reason_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Calculate graduation eligibility
    async fn calculate_graduation_eligibility(&self, customer_id: Uuid) -> BankingResult<bool>;
    
    /// Graduate customer from collection program
    async fn graduate_customer(
        &self,
        customer_id: Uuid,
        graduation_account_id: Uuid,
        reason_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    // ======== Collection Operations ========
    
    /// Record a single collection
    async fn record_collection(&self, collection: CollectionRecord) -> BankingResult<CollectionRecord>;
    
    /// Process collection batch
    async fn process_collection_batch(&self, batch: CollectionBatch) -> BankingResult<CollectionBatch>;
    
    /// Find collection records by date range
    async fn find_collections_by_date_range(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecord>>;
    
    /// Find collections by agent for date
    async fn find_collections_by_agent_date(
        &self,
        agent_id: Uuid,
        collection_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecord>>;
    
    /// Find collections by customer
    async fn find_collections_by_customer(&self, customer_id: Uuid) -> BankingResult<Vec<CollectionRecord>>;
    
    /// Find collections by status
    async fn find_collections_by_status(&self, status: CollectionRecordStatus) -> BankingResult<Vec<CollectionRecord>>;
    
    /// Update collection status
    async fn update_collection_status(
        &self,
        collection_id: Uuid,
        status: CollectionRecordStatus,
        reason_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Reverse a collection
    async fn reverse_collection(
        &self,
        collection_id: Uuid,
        reason_id: Uuid,
        authorized_by: Uuid,
    ) -> BankingResult<()>;
    
    /// Reconcile collections for agent and date
    async fn reconcile_collections(
        &self,
        agent_id: Uuid,
        collection_date: NaiveDate,
        expected_amount: Decimal,
        actual_amount: Decimal,
        reconciled_by: Uuid,
    ) -> BankingResult<Vec<CollectionRecord>>;
    
    /// Get collection statistics for period
    async fn get_collection_statistics(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        agent_id: Option<Uuid>,
        program_id: Option<Uuid>,
    ) -> BankingResult<CollectionStatistics>;
    
    // ======== Agent Management ========
    
    /// Create collection agent
    async fn create_collection_agent(&self, agent: CollectionAgent) -> BankingResult<CollectionAgent>;
    
    /// Update collection agent
    async fn update_collection_agent(&self, agent_id: Uuid, agent: CollectionAgent) -> BankingResult<CollectionAgent>;
    
    /// Find collection agent by ID
    async fn get_collection_agent(&self, agent_id: Uuid) -> BankingResult<Option<CollectionAgent>>;
    
    /// Find agents by status
    async fn find_agents_by_status(&self, status: AgentStatus) -> BankingResult<Vec<CollectionAgent>>;
    
    /// Find active agents
    async fn find_active_agents(&self) -> BankingResult<Vec<CollectionAgent>>;
    
    /// Assign agent to customers
    async fn assign_agent_to_customers(
        &self,
        agent_id: Uuid,
        customer_ids: Vec<Uuid>,
    ) -> BankingResult<()>;
    
    /// Get agent portfolio (assigned customers)
    async fn get_agent_portfolio(&self, agent_id: Uuid) -> BankingResult<Vec<CustomerCollectionProfile>>;
    
    /// Update agent performance metrics
    async fn update_agent_performance(&self, agent_id: Uuid, performance_data: AgentPerformanceUpdate) -> BankingResult<()>;
    
    /// Get agent performance report
    async fn get_agent_performance_report(
        &self,
        agent_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<AgentPerformanceReport>;
    
    /// Update agent status
    async fn update_agent_status(
        &self,
        agent_id: Uuid,
        status: AgentStatus,
        reason_id: Option<Uuid>,
    ) -> BankingResult<()>;
    
    /// Find agents by territory
    async fn find_agents_by_territory(&self, territory_id: Uuid) -> BankingResult<Vec<CollectionAgent>>;
    
    // ======== Route Optimization and Scheduling ========
    
    /// Generate optimal collection routes for agent
    async fn generate_collection_routes(
        &self,
        agent_id: Uuid,
        collection_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRoute>>;
    
    /// Get scheduled collections for agent and date
    async fn get_scheduled_collections(
        &self,
        agent_id: Uuid,
        collection_date: NaiveDate,
    ) -> BankingResult<Vec<ScheduledCollection>>;
    
    /// Update collection schedule
    async fn update_collection_schedule(
        &self,
        customer_id: Uuid,
        new_schedule: CollectionScheduleUpdate,
    ) -> BankingResult<()>;
    
    // ======== Reporting and Analytics ========
    
    /// Generate program performance report
    async fn generate_program_performance_report(
        &self,
        program_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> BankingResult<ProgramPerformanceReport>;
    
    /// Generate daily collection summary
    async fn generate_daily_collection_summary(&self, collection_date: NaiveDate) -> BankingResult<DailyCollectionSummary>;
    
    /// Get collection trends analysis
    async fn get_collection_trends(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        granularity: TrendGranularity,
    ) -> BankingResult<CollectionTrends>;
    
    /// Generate agent performance ranking
    async fn get_agent_performance_ranking(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        ranking_criteria: RankingCriteria,
    ) -> BankingResult<Vec<AgentRanking>>;
}

// ======== Supporting Data Structures ========

/// Collection statistics for analysis
#[derive(Debug, Clone)]
pub struct CollectionStatistics {
    pub total_collections: i64,
    pub total_amount: Decimal,
    pub average_collection_amount: Decimal,
    pub collection_rate: Decimal,
    pub unique_customers: i64,
    pub active_agents: i64,
    pub missed_collections: i64,
}

/// Agent performance update data
#[derive(Debug, Clone)]
pub struct AgentPerformanceUpdate {
    pub collection_rate: Option<Decimal>,
    pub customer_satisfaction_score: Option<Decimal>,
    pub punctuality_score: Option<Decimal>,
    pub cash_handling_accuracy: Option<Decimal>,
    pub compliance_score: Option<Decimal>,
}

/// Agent performance report
#[derive(Debug, Clone)]
pub struct AgentPerformanceReport {
    pub agent_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_collections: i64,
    pub total_amount_collected: Decimal,
    pub collection_rate: Decimal,
    pub average_collection_time: chrono::Duration,
    pub customer_satisfaction_score: Decimal,
    pub punctuality_score: Decimal,
    pub compliance_score: Decimal,
    pub targets_met: Vec<String>,
    pub improvement_areas: Vec<String>,
}

/// Collection route for optimization
#[derive(Debug, Clone)]
pub struct CollectionRoute {
    pub route_id: Uuid,
    pub agent_id: Uuid,
    pub collection_date: NaiveDate,
    pub waypoints: Vec<RouteWaypoint>,
    pub estimated_duration: chrono::Duration,
    pub estimated_distance: f64,
    pub optimization_score: Decimal,
}

/// Route waypoint with collection details
#[derive(Debug, Clone)]
pub struct RouteWaypoint {
    pub customer_id: Uuid,
    pub sequence_number: i32,
    pub location_address_id: Uuid,
    pub estimated_collection_amount: Decimal,
    pub estimated_arrival_time: DateTime<Utc>,
    pub collection_method: CollectionMethod,
    pub special_instructions: Option<String>,
}

/// Scheduled collection for agent planning
#[derive(Debug, Clone)]
pub struct ScheduledCollection {
    pub customer_id: Uuid,
    pub collection_date: NaiveDate,
    pub scheduled_time: DateTime<Utc>,
    pub expected_amount: Decimal,
    pub collection_method: CollectionMethod,
    pub location_address_id: Uuid,
    pub priority: CollectionPriority,
    pub notes: Option<String>,
}

#[derive(Debug, Clone)]
pub enum CollectionPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Collection schedule update
#[derive(Debug, Clone)]
pub struct CollectionScheduleUpdate {
    pub days_of_week: Option<Vec<String>>,
    pub collection_time: Option<chrono::NaiveTime>,
    pub timezone: Option<String>,
    pub holiday_handling: Option<String>,
}

/// Program performance report
#[derive(Debug, Clone)]
pub struct ProgramPerformanceReport {
    pub program_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_enrolled_customers: i64,
    pub active_customers: i64,
    pub graduated_customers: i64,
    pub total_collections: i64,
    pub total_amount_collected: Decimal,
    pub average_customer_balance: Decimal,
    pub collection_rate: Decimal,
    pub customer_retention_rate: Decimal,
    pub program_roi: Decimal,
}

/// Daily collection summary
#[derive(Debug, Clone)]
pub struct DailyCollectionSummary {
    pub collection_date: NaiveDate,
    pub total_scheduled: i64,
    pub total_completed: i64,
    pub total_missed: i64,
    pub total_amount_collected: Decimal,
    pub completion_rate: Decimal,
    pub active_agents: i64,
    pub top_performing_agents: Vec<Uuid>,
    pub areas_needing_attention: Vec<String>,
}

/// Collection trends analysis
#[derive(Debug, Clone)]
pub struct CollectionTrends {
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub granularity: TrendGranularity,
    pub data_points: Vec<TrendDataPoint>,
    pub overall_trend: TrendDirection,
    pub growth_rate: Decimal,
}

#[derive(Debug, Clone)]
pub enum TrendGranularity {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone)]
pub enum TrendDirection {
    Increasing,
    Decreasing,
    Stable,
    Volatile,
}

#[derive(Debug, Clone)]
pub struct TrendDataPoint {
    pub period: NaiveDate,
    pub collections: i64,
    pub amount: Decimal,
    pub rate: Decimal,
}

/// Agent ranking for performance comparison
#[derive(Debug, Clone)]
pub struct AgentRanking {
    pub agent_id: Uuid,
    pub agent_name: String,
    pub rank: i32,
    pub score: Decimal,
    pub collections: i64,
    pub amount_collected: Decimal,
    pub collection_rate: Decimal,
    pub customer_satisfaction: Decimal,
}

#[derive(Debug, Clone)]
pub enum RankingCriteria {
    TotalCollections,
    AmountCollected,
    CollectionRate,
    CustomerSatisfaction,
    Overall,
}