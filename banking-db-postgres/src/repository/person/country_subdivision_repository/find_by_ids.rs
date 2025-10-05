use banking_db::models::person::CountrySubdivisionIdxModel;
use banking_db::repository::CountrySubdivisionResult;
use uuid::Uuid;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn find_by_ids(
    repo: &CountrySubdivisionRepositoryImpl,
    ids: &[Uuid],
) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>> {
    let mut result = Vec::new();
    let cache = repo.country_subdivision_idx_cache.read().await;
    for id in ids {
        if let Some(idx) = cache.get_by_primary(id) {
            result.push(idx);
        }
    }
    Ok(result)
}