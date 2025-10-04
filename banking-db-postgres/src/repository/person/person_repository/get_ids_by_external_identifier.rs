use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn get_ids_by_external_identifier(
    repo: &PersonRepositoryImpl,
    identifier: &str,
) -> PersonResult<Vec<Uuid>> {
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(identifier.as_bytes());
    let hash = hasher.finish() as i64;

    let cache = repo.person_idx_cache.read().await;
    Ok(cache.get_by_external_identifier_hash(&hash).unwrap_or_default())
}