use banking_db::models::person::LocationIdxModel;
use banking_db::repository::LocationResult;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_ids(
    repo: &LocationRepositoryImpl,
    ids: &[Uuid],
) -> LocationResult<Vec<LocationIdxModel>> {
    let cache = repo.location_idx_cache.read().await;
    let mut locations = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(location_idx) = cache.get_by_primary(id) {
            locations.push(location_idx);
        }
    }
    Ok(locations)
}