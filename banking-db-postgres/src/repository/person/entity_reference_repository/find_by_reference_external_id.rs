use banking_db::models::person::EntityReferenceIdxModel;
use banking_db::repository::person::entity_reference_repository::EntityReferenceResult;
use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use std::hash::Hasher;
use twox_hash::XxHash64;

pub async fn find_by_reference_external_id(
    repo: &EntityReferenceRepositoryImpl,
    reference_external_id: &str,
    page: i32,
    page_size: i32,
) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(reference_external_id.as_bytes());
    let hash = hasher.finish() as i64;

    let cache = repo.entity_reference_idx_cache.read().await;
    if let Some(ids) = cache.get_by_reference_external_id_hash(&hash) {
        let start = ((page - 1) * page_size) as usize;
        let end = (start + page_size as usize).min(ids.len());
        if start >= ids.len() {
            return Ok(Vec::new());
        }
        let mut refs = Vec::with_capacity(end - start);
        for id in &ids[start..end] {
            if let Some(model) = cache.get_by_primary(id) {
                refs.push(model);
            }
        }
        Ok(refs)
    } else {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use banking_db::models::person::RelationshipRole;
    use banking_db::repository::{EntityReferenceRepository, PersonRepository, PersonRepos};
    use crate::repository::person::test_helpers::{
        create_test_entity_reference_model, create_test_person_model,
    };
    use crate::test_helper::setup_test_context;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_find_by_reference_external_id() {
        let ctx = setup_test_context().await.unwrap();
        let person_repo = ctx.person_repos().persons();
        let repo = ctx.person_repos().entity_references();

        let new_person = create_test_person_model("John Doe");
        let audit_log_id = Uuid::new_v4();
        person_repo
            .save(new_person.clone(), audit_log_id)
            .await
            .unwrap();

        let new_entity_ref = create_test_entity_reference_model(
            new_person.id,
            RelationshipRole::Customer,
            "CUST-12345",
        );
        repo.save(new_entity_ref.clone(), audit_log_id)
            .await
            .unwrap();

        let refs_by_ext_id = repo
            .find_by_reference_external_id("CUST-12345", 1, 10)
            .await
            .unwrap();
        assert_eq!(refs_by_ext_id.len(), 1);
    }
}