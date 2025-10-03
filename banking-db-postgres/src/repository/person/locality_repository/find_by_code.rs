use banking_db::models::person::LocalityIdxModel;
use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;
use std::hash::Hasher;
use twox_hash::XxHash64;

pub async fn find_by_code(
    repo: &LocalityRepositoryImpl,
    _country_id: Uuid,
    code: &str,
) -> LocalityResult<Option<LocalityIdxModel>> {
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(code.as_bytes());
    let code_hash = hasher.finish() as i64;

    let cache = repo.locality_idx_cache.read().await;
    if let Some(id) = cache.get_by_code_hash(&code_hash) {
        Ok(cache.get_by_primary(&id))
    } else {
        Ok(None)
    }
}