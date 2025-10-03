use banking_db::repository::LocationResult;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use uuid::Uuid;

pub async fn exist_by_ids(
    repo: &LocationRepositoryImpl,
    ids: &[Uuid],
) -> LocationResult<Vec<(Uuid, bool)>> {
    let mut results = Vec::with_capacity(ids.len());
    let cache = repo.location_idx_cache.read().await;
    for &id in ids {
        results.push((id, cache.get_by_primary(&id).is_some()));
    }
    Ok(results)
}