use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;

pub async fn exist_by_ids(repo: &LocalityRepositoryImpl, ids: &[Uuid]) -> LocalityResult<Vec<bool>> {
    let cache = repo.locality_idx_cache.read().await;
    let mut result = Vec::with_capacity(ids.len());
    for id in ids {
        result.push(cache.contains_primary(id));
    }
    Ok(result)
}