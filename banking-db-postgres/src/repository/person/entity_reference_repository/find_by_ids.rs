use banking_db::models::person::EntityReferenceIdxModel;
use banking_db::repository::person::entity_reference_repository::EntityReferenceResult;
use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_ids(
    repo: &EntityReferenceRepositoryImpl,
    ids: &[Uuid],
) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
    let cache = repo.entity_reference_idx_cache.read().await;
    let mut refs = Vec::with_capacity(ids.len());
    for id in ids {
        if let Some(model) = cache.get_by_primary(id) {
            refs.push(model);
        }
    }
    Ok(refs)
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
    async fn test_find_by_ids() {
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
        let new_entity_ref2 = create_test_entity_reference_model(
            new_person.id,
            RelationshipRole::Employee,
            "EMP-54321",
        );
        repo.save(new_entity_ref2.clone(), audit_log_id)
            .await
            .unwrap();
        let ids = vec![new_entity_ref.id, new_entity_ref2.id];
        let found_refs = repo.find_by_ids(&ids).await.unwrap();
        assert_eq!(found_refs.len(), 2);
    }
}