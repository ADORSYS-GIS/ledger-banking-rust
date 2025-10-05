use banking_db::models::person::PersonIdxModel;
use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_duplicate_of_person_id(
    repo: &PersonRepositoryImpl,
    person_id: Uuid,
) -> PersonResult<Vec<PersonIdxModel>> {
    let cache = repo.person_idx_cache.read().await;
    let results = cache
        .iter()
        .into_iter()
        .filter(|item| item.duplicate_of_person_id == Some(person_id))
        .collect();
    Ok(results)
}