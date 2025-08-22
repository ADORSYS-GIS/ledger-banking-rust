use crate::models::daily_collection::{CollectionAgentModel, AgentStatus};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait DailyCollectionRepository: Send + Sync {
    async fn create_collection_agent(&self, collection_agent: CollectionAgentModel) -> Result<CollectionAgentModel, String>;
    async fn update_collection_agent(&self, agent_id: Uuid, collection_agent: CollectionAgentModel) -> Result<CollectionAgentModel, String>;
    async fn get_collection_agent(&self, agent_id: Uuid) -> Result<Option<CollectionAgentModel>, String>;
    async fn find_agents_by_status(&self, status: AgentStatus) -> Result<Vec<CollectionAgentModel>, String>;
    async fn update_agent_status(&self, agent_id: Uuid, status: AgentStatus) -> Result<(), String>;
}