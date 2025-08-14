use banking_db::models::daily_collection::CollectionAgentModel;
use banking_db::repository::daily_collection_repository::DailyCollectionRepository;
use sqlx::PgPool;
use std::sync::Arc;

pub struct DailyCollectionRepositoryImpl {
    #[allow(dead_code)]
    pool: Arc<PgPool>,
}

impl DailyCollectionRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

impl DailyCollectionRepository for DailyCollectionRepositoryImpl {
    fn create_collection_agent(&self, _collection_agent: CollectionAgentModel) -> Result<CollectionAgentModel, String> {
        // Implementation goes here
        unimplemented!()
    }
}