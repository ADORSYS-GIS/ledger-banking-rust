use banking_db::models::person::PersonIdxModel;
use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_id(
    repo: &PersonRepositoryImpl,
    id: Uuid,
) -> PersonResult<Option<PersonIdxModel>> {
    Ok(repo.person_idx_cache.read().await.get_by_primary(&id))
}
#[cfg(test)]
mod tests {
    use banking_db::repository::{PersonRepository, PersonRepos};
    use uuid::Uuid;
    use crate::repository::person::test_helpers::create_test_person_model;
    use crate::test_helper::setup_test_context;

    #[tokio::test]
    async fn test_find_by_id() {
        let ctx = setup_test_context().await.unwrap();
        let repo = ctx.person_repos().persons();

        // 1. Test finding an existing person
        let audit_log_id = Uuid::new_v4();
        let new_person = create_test_person_model("Jane Doe");
        repo.save(new_person.clone(), audit_log_id).await.unwrap();

        let found_person_idx = repo.find_by_id(new_person.id).await.unwrap();
        assert!(found_person_idx.is_some());
        assert_eq!(new_person.id, found_person_idx.unwrap().person_id);

        // 2. Test finding a non-existent person
        let non_existent_id = Uuid::new_v4();
        let found_person_idx = repo.find_by_id(non_existent_id).await.unwrap();
        assert!(found_person_idx.is_none());
    }
}