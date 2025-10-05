use banking_db::models::person::EntityReferenceIdxModel;
use banking_db::repository::person::entity_reference_repository::EntityReferenceResult;
use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_id(
    repo: &EntityReferenceRepositoryImpl,
    id: Uuid,
) -> EntityReferenceResult<Option<EntityReferenceIdxModel>> {
    Ok(repo
        .entity_reference_idx_cache
        .read()
        .await
        .get_by_primary(&id))
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
    async fn test_find_by_id() {
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

        let found_entity_ref = repo
            .find_by_id(new_entity_ref.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(new_entity_ref.id, found_entity_ref.entity_reference_id);
    }
}