use banking_db::models::person::LocalityIdxModel;
use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_ids(repo: &LocalityRepositoryImpl, ids: &[Uuid]) -> LocalityResult<Vec<LocalityIdxModel>> {
    let cache = repo.locality_idx_cache.read().await;
    let mut result = Vec::new();
    for id in ids {
        if let Some(idx) = cache.get_by_primary(id) {
            result.push(idx);
        }
    }
    Ok(result)
}