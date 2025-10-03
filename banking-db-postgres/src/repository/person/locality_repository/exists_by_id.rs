use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;

pub async fn exists_by_id(repo: &LocalityRepositoryImpl, id: Uuid) -> LocalityResult<bool> {
    Ok(repo.locality_idx_cache.read().await.contains_primary(&id))
}