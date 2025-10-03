use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;

pub async fn find_ids_by_country_subdivision_id(
    repo: &LocalityRepositoryImpl,
    country_subdivision_id: Uuid,
) -> LocalityResult<Vec<Uuid>> {
    let cache = repo.locality_idx_cache.read().await;
    Ok(cache
        .get_by_country_subdivision_id(&country_subdivision_id)
        .unwrap_or_default())
}