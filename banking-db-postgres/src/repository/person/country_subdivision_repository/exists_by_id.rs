use banking_db::repository::CountrySubdivisionResult;
use uuid::Uuid;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn exists_by_id(
    repo: &CountrySubdivisionRepositoryImpl,
    id: Uuid,
) -> CountrySubdivisionResult<bool> {
    Ok(repo
        .country_subdivision_idx_cache
        .read()
        .await
        .contains_primary(&id))
}