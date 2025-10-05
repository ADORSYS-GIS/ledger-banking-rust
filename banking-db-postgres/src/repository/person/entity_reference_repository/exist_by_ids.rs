use banking_db::repository::person::entity_reference_repository::EntityReferenceResult;
use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use uuid::Uuid;

pub async fn exist_by_ids(
    repo: &EntityReferenceRepositoryImpl,
    ids: &[Uuid],
) -> EntityReferenceResult<Vec<(Uuid, bool)>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let cache = repo.entity_reference_idx_cache.read().await;
    let mut result = Vec::with_capacity(ids.len());
    for &id in ids {
        result.push((id, cache.get_by_primary(&id).is_some()));
    }
    Ok(result)
}