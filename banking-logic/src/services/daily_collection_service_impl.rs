use async_trait::async_trait;
use banking_api::domain::daily_collection::{
    AgentStatus, CollectionAgent, CollectionBatch, CollectionProgram, CollectionRecord,
    CollectionRecordStatus, CollectionStatus, CustomerCollectionProfile, ProgramStatus,
};
use banking_api::service::daily_collection_service::{
    AgentPerformanceReport, AgentPerformanceUpdate, AgentRanking, CollectionRoute,
    CollectionScheduleUpdate, CollectionStatistics, CollectionTrends, DailyCollectionSummary,
    DailyCollectionService, ProgramPerformanceReport, RankingCriteria, ScheduledCollection,
    TrendGranularity,
};
use banking_api::{error::BankingError, BankingResult};
use banking_db::repository::daily_collection_repository::DailyCollectionRepository;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::mappers::daily_collection_mapper::DailyCollectionMapper;

pub struct DailyCollectionServiceImpl {
    daily_collection_repository: Arc<dyn DailyCollectionRepository>,
}

impl DailyCollectionServiceImpl {
    pub fn new(daily_collection_repository: Arc<dyn DailyCollectionRepository>) -> Self {
        Self {
            daily_collection_repository,
        }
    }
}

#[async_trait]
impl DailyCollectionService for DailyCollectionServiceImpl {
    async fn create_collection_program(
        &self,
        _program: CollectionProgram,
    ) -> BankingResult<CollectionProgram> {
        unimplemented!()
    }

    async fn update_collection_program(
        &self,
        _program_id: Uuid,
        _program: CollectionProgram,
    ) -> BankingResult<CollectionProgram> {
        unimplemented!()
    }

    async fn get_collection_program(&self, _program_id: Uuid) -> BankingResult<Option<CollectionProgram>> {
        unimplemented!()
    }

    async fn find_programs_by_status(
        &self,
        _status: ProgramStatus,
    ) -> BankingResult<Vec<CollectionProgram>> {
        unimplemented!()
    }

    async fn find_active_programs(&self) -> BankingResult<Vec<CollectionProgram>> {
        unimplemented!()
    }

    async fn deactivate_program(&self, _program_id: Uuid, _reason_id: Option<Uuid>) -> BankingResult<()> {
        unimplemented!()
    }

    async fn enroll_customer(
        &self,
        _customer_id: Uuid,
        _program_id: Uuid,
        _profile: CustomerCollectionProfile,
    ) -> BankingResult<CustomerCollectionProfile> {
        unimplemented!()
    }

    async fn update_customer_program(
        &self,
        _customer_id: Uuid,
        _profile: CustomerCollectionProfile,
    ) -> BankingResult<CustomerCollectionProfile> {
        unimplemented!()
    }

    async fn get_customer_collection_profile(
        &self,
        _customer_id: Uuid,
    ) -> BankingResult<Option<CustomerCollectionProfile>> {
        unimplemented!()
    }

    async fn get_customer_collection_history(
        &self,
        _customer_id: Uuid,
        _date_range: (NaiveDate, NaiveDate),
    ) -> BankingResult<Vec<CollectionRecord>> {
        unimplemented!()
    }

    async fn find_customers_by_status(
        &self,
        _status: CollectionStatus,
    ) -> BankingResult<Vec<CustomerCollectionProfile>> {
        unimplemented!()
    }

    async fn find_customers_by_program(
        &self,
        _program_id: Uuid,
    ) -> BankingResult<Vec<CustomerCollectionProfile>> {
        unimplemented!()
    }

    async fn update_customer_status(
        &self,
        _customer_id: Uuid,
        _status: CollectionStatus,
        _reason_id: Option<Uuid>,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn calculate_graduation_eligibility(&self, _customer_id: Uuid) -> BankingResult<bool> {
        unimplemented!()
    }

    async fn graduate_customer(
        &self,
        _customer_id: Uuid,
        _graduation_account_id: Uuid,
        _reason_id: Option<Uuid>,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn record_collection(
        &self,
        _collection: CollectionRecord,
    ) -> BankingResult<CollectionRecord> {
        unimplemented!()
    }

    async fn process_collection_batch(
        &self,
        _batch: CollectionBatch,
    ) -> BankingResult<CollectionBatch> {
        unimplemented!()
    }

    async fn find_collections_by_date_range(
        &self,
        _start_date: NaiveDate,
        _end_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecord>> {
        unimplemented!()
    }

    async fn find_collections_by_agent_date(
        &self,
        _agent_id: Uuid,
        _collection_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRecord>> {
        unimplemented!()
    }

    async fn find_collections_by_customer(
        &self,
        _customer_id: Uuid,
    ) -> BankingResult<Vec<CollectionRecord>> {
        unimplemented!()
    }

    async fn find_collections_by_status(
        &self,
        _status: CollectionRecordStatus,
    ) -> BankingResult<Vec<CollectionRecord>> {
        unimplemented!()
    }

    async fn update_collection_status(
        &self,
        _collection_id: Uuid,
        _status: CollectionRecordStatus,
        _reason_id: Option<Uuid>,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn reverse_collection(
        &self,
        _collection_id: Uuid,
        _reason_id: Uuid,
        _authorized_by_person_id: Uuid,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn reconcile_collections(
        &self,
        _agent_id: Uuid,
        _collection_date: NaiveDate,
        _expected_amount: Decimal,
        _actual_amount: Decimal,
        _reconciled_by_person_id: Uuid,
    ) -> BankingResult<Vec<CollectionRecord>> {
        unimplemented!()
    }

    async fn get_collection_statistics(
        &self,
        _start_date: NaiveDate,
        _end_date: NaiveDate,
        _agent_id: Option<Uuid>,
        _program_id: Option<Uuid>,
    ) -> BankingResult<CollectionStatistics> {
        unimplemented!()
    }

    async fn create_collection_agent(
        &self,
        agent: CollectionAgent,
    ) -> BankingResult<CollectionAgent> {
        let agent_model = DailyCollectionMapper::collection_agent_to_db(agent);
        let result = self
            .daily_collection_repository
            .create_collection_agent(agent_model)
            .await
            .map_err(BankingError::Internal)?;
        Ok(DailyCollectionMapper::collection_agent_from_db(result))
    }

    async fn update_collection_agent(
        &self,
        agent_id: Uuid,
        agent: CollectionAgent,
    ) -> BankingResult<CollectionAgent> {
        let agent_model = DailyCollectionMapper::collection_agent_to_db(agent);
        let result = self
            .daily_collection_repository
            .update_collection_agent(agent_id, agent_model)
            .await
            .map_err(BankingError::Internal)?;
        Ok(DailyCollectionMapper::collection_agent_from_db(result))
    }

    async fn get_collection_agent(&self, agent_id: Uuid) -> BankingResult<Option<CollectionAgent>> {
        let result = self
            .daily_collection_repository
            .get_collection_agent(agent_id)
            .await
            .map_err(BankingError::Internal)?;

        Ok(result.map(DailyCollectionMapper::collection_agent_from_db))
    }

    async fn find_agents_by_status(
        &self,
        status: AgentStatus,
    ) -> BankingResult<Vec<CollectionAgent>> {
        let db_status = DailyCollectionMapper::agent_status_to_db(status);
        let result = self
            .daily_collection_repository
            .find_agents_by_status(db_status)
            .await
            .map_err(BankingError::Internal)?;

        Ok(result
            .into_iter()
            .map(DailyCollectionMapper::collection_agent_from_db)
            .collect())
    }

    async fn find_active_agents(&self) -> BankingResult<Vec<CollectionAgent>> {
        self.find_agents_by_status(AgentStatus::Active).await
    }

    async fn assign_agent_to_customers(
        &self,
        _agent_id: Uuid,
        _customer_ids: Vec<Uuid>,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn get_agent_portfolio(
        &self,
        _agent_id: Uuid,
    ) -> BankingResult<Vec<CustomerCollectionProfile>> {
        unimplemented!()
    }

    async fn update_agent_performance(
        &self,
        _agent_id: Uuid,
        _performance_data: AgentPerformanceUpdate,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn get_agent_performance_report(
        &self,
        _agent_id: Uuid,
        _start_date: NaiveDate,
        _end_date: NaiveDate,
    ) -> BankingResult<AgentPerformanceReport> {
        unimplemented!()
    }

    async fn update_agent_status(
        &self,
        agent_id: Uuid,
        status: AgentStatus,
        _reason_id: Option<Uuid>,
    ) -> BankingResult<()> {
        let db_status = DailyCollectionMapper::agent_status_to_db(status);
        self.daily_collection_repository
            .update_agent_status(agent_id, db_status)
            .await
            .map_err(BankingError::Internal)
    }

    async fn find_agents_by_territory(
        &self,
        _territory_id: Uuid,
    ) -> BankingResult<Vec<CollectionAgent>> {
        unimplemented!()
    }

    async fn generate_collection_routes(
        &self,
        _agent_id: Uuid,
        _collection_date: NaiveDate,
    ) -> BankingResult<Vec<CollectionRoute>> {
        unimplemented!()
    }

    async fn get_scheduled_collections(
        &self,
        _agent_id: Uuid,
        _collection_date: NaiveDate,
    ) -> BankingResult<Vec<ScheduledCollection>> {
        unimplemented!()
    }

    async fn update_collection_schedule(
        &self,
        _customer_id: Uuid,
        _new_schedule: CollectionScheduleUpdate,
    ) -> BankingResult<()> {
        unimplemented!()
    }

    async fn generate_program_performance_report(
        &self,
        _program_id: Uuid,
        _start_date: NaiveDate,
        _end_date: NaiveDate,
    ) -> BankingResult<ProgramPerformanceReport> {
        unimplemented!()
    }

    async fn generate_daily_collection_summary(
        &self,
        _collection_date: NaiveDate,
    ) -> BankingResult<DailyCollectionSummary> {
        unimplemented!()
    }

    async fn get_collection_trends(
        &self,
        _start_date: NaiveDate,
        _end_date: NaiveDate,
        _granularity: TrendGranularity,
    ) -> BankingResult<CollectionTrends> {
        unimplemented!()
    }

    async fn get_agent_performance_ranking(
        &self,
        _start_date: NaiveDate,
        _end_date: NaiveDate,
        _ranking_criteria: RankingCriteria,
    ) -> BankingResult<Vec<AgentRanking>> {
        unimplemented!()
    }
}