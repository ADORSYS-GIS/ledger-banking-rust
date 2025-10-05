use banking_db::models::person::PersonIdxModel;
use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_ids(
    repo: &PersonRepositoryImpl,
    ids: &[Uuid],
) -> PersonResult<Vec<PersonIdxModel>> {
    let cache = repo.person_idx_cache.read().await;
    let results = ids.iter().filter_map(|id| cache.get_by_primary(id)).collect();
    Ok(results)
}
#[cfg(test)]
mod tests {
    use banking_db::repository::{PersonRepository, PersonRepos};
    use uuid::Uuid;
    use crate::repository::person::test_helpers::create_test_person_model;
    use crate::test_helper::setup_test_context;

    #[tokio::test]
    async fn test_find_by_ids() {
        let ctx = setup_test_context().await.unwrap();
        let repo = ctx.person_repos().persons();
        let audit_log_id = Uuid::new_v4();

        // 1. Setup - Create and save multiple persons
        let person1 = create_test_person_model("Chris Green");
        let person2 = create_test_person_model("Diana Prince");
        repo.save(person1.clone(), audit_log_id).await.unwrap();
        repo.save(person2.clone(), audit_log_id).await.unwrap();

        // 2. Test with a mix of existing and non-existing IDs
        let non_existent_id = Uuid::new_v4();
        let ids_to_find = vec![person1.id, non_existent_id, person2.id];
        let found_persons = repo.find_by_ids(&ids_to_find).await.unwrap();

        // 3. Assertions
        assert_eq!(found_persons.len(), 2);
        assert!(found_persons.iter().any(|p| p.person_id == person1.id));
        assert!(found_persons.iter().any(|p| p.person_id == person2.id));
        assert!(!found_persons.iter().any(|p| p.person_id == non_existent_id));
    }
}