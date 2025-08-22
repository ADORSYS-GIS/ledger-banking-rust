use async_trait::async_trait;
use banking_db::models::daily_collection::{AgentStatus, CollectionAgentModel};
use banking_db::repository::daily_collection_repository::DailyCollectionRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct DailyCollectionRepositoryImpl {
    pool: Arc<PgPool>,
}

impl DailyCollectionRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DailyCollectionRepository for DailyCollectionRepositoryImpl {
    async fn create_collection_agent(
        &self,
        collection_agent: CollectionAgentModel,
    ) -> Result<CollectionAgentModel, String> {
        let result = sqlx::query_as!(
            CollectionAgentModel,
            r#"
            INSERT INTO collection_agents (
                id, person_id, license_number, license_expiry, status, 
                assigned_territory_id, agent_performance_metrics_id, 
                cash_limit, device_information_id, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, person_id, license_number, license_expiry, status as "status: _", assigned_territory_id, agent_performance_metrics_id, cash_limit, device_information_id, created_at, updated_at
            "#,
            collection_agent.id,
            collection_agent.person_id,
            collection_agent.license_number as _,
            collection_agent.license_expiry,
            collection_agent.status as _,
            collection_agent.assigned_territory_id,
            collection_agent.agent_performance_metrics_id,
            collection_agent.cash_limit,
            collection_agent.device_information_id,
            collection_agent.created_at,
            collection_agent.updated_at
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn update_collection_agent(
        &self,
        agent_id: Uuid,
        collection_agent: CollectionAgentModel,
    ) -> Result<CollectionAgentModel, String> {
        let result = sqlx::query_as!(
            CollectionAgentModel,
            r#"
            UPDATE collection_agents
            SET 
                person_id = $2,
                license_number = $3,
                license_expiry = $4,
                status = $5,
                assigned_territory_id = $6,
                agent_performance_metrics_id = $7,
                cash_limit = $8,
                device_information_id = $9,
                updated_at = $10
            WHERE id = $1
            RETURNING id, person_id, license_number, license_expiry, status as "status: _", assigned_territory_id, agent_performance_metrics_id, cash_limit, device_information_id, created_at, updated_at
            "#,
            agent_id,
            collection_agent.person_id,
            collection_agent.license_number as _,
            collection_agent.license_expiry,
            collection_agent.status as _,
            collection_agent.assigned_territory_id,
            collection_agent.agent_performance_metrics_id,
            collection_agent.cash_limit,
            collection_agent.device_information_id,
            collection_agent.updated_at
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn get_collection_agent(&self, agent_id: Uuid) -> Result<Option<CollectionAgentModel>, String> {
        let result = sqlx::query_as!(
            CollectionAgentModel,
            r#"
            SELECT id, person_id, license_number, license_expiry, status as "status: _", assigned_territory_id, agent_performance_metrics_id, cash_limit, device_information_id, created_at, updated_at
            FROM collection_agents
            WHERE id = $1
            "#,
            agent_id
        )
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn find_agents_by_status(&self, status: AgentStatus) -> Result<Vec<CollectionAgentModel>, String> {
        let result = sqlx::query_as!(
            CollectionAgentModel,
            r#"
            SELECT id, person_id, license_number, license_expiry, status as "status: _", assigned_territory_id, agent_performance_metrics_id, cash_limit, device_information_id, created_at, updated_at
            FROM collection_agents
            WHERE status = $1
            "#,
            status as _
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(result)
    }

    async fn update_agent_status(&self, agent_id: Uuid, status: AgentStatus) -> Result<(), String> {
        sqlx::query!(
            r#"
            UPDATE collection_agents
            SET status = $2
            WHERE id = $1
            "#,
            agent_id,
            status as _
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}