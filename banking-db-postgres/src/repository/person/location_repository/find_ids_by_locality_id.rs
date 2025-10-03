use banking_db::repository::LocationResult;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use uuid::Uuid;

pub async fn find_ids_by_locality_id(
    repo: &LocationRepositoryImpl,
    locality_id: Uuid,
) -> LocationResult<Vec<Uuid>> {
    let cache = repo.location_idx_cache.read().await;
    let locations = cache.get_by_locality_id(&locality_id);
    let ids = locations.into_iter().map(|loc| loc.location_id).collect();
    Ok(ids)
}