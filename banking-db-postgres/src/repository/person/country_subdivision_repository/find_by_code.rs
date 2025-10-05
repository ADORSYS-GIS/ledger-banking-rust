use banking_db::models::person::CountrySubdivisionIdxModel;
use banking_db::repository::CountrySubdivisionResult;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn find_by_code(
    repo: &CountrySubdivisionRepositoryImpl,
    _country_id: Uuid,
    code: &str,
) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>> {
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(code.as_bytes());
    let code_hash = hasher.finish() as i64;

    let cache = repo.country_subdivision_idx_cache.read().await;
    if let Some(id) = cache.get_by_code_hash(&code_hash) {
        Ok(cache.get_by_primary(&id))
    } else {
        Ok(None)
    }
}