use banking_db::repository::LocationResult;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use uuid::Uuid;

pub async fn exists_by_id(repo: &LocationRepositoryImpl, id: Uuid) -> LocationResult<bool> {
    Ok(repo
        .location_idx_cache
        .read()
        .await
        .get_by_primary(&id)
        .is_some())
}