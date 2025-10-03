use banking_db::repository::CountrySubdivisionResult;
use uuid::Uuid;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn find_ids_by_country_id(
    repo: &CountrySubdivisionRepositoryImpl,
    country_id: Uuid,
) -> CountrySubdivisionResult<Vec<Uuid>> {
    Ok(repo
        .country_subdivision_idx_cache
        .read()
        .await
        .get_by_country_id(&country_id)
        .unwrap_or_default())
}