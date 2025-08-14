use crate::models::daily_collection::CollectionAgentModel;

pub trait DailyCollectionRepository: Send + Sync {
    fn create_collection_agent(&self, collection_agent: CollectionAgentModel) -> Result<CollectionAgentModel, String>;
}