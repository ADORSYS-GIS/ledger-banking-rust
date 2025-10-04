use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use uuid::Uuid;

pub async fn exists_by_id(repo: &PersonRepositoryImpl, id: Uuid) -> PersonResult<bool> {
    Ok(repo.person_idx_cache.read().await.contains_primary(&id))
}
#[cfg(test)]
mod tests {
    use banking_db::repository::{PersonRepository, PersonRepos};
    use uuid::Uuid;
    use crate::repository::person::test_helpers::create_test_person_model;
    use crate::test_helper::setup_test_context;

    #[tokio::test]
    async fn test_exists_by_id() {
        let ctx = setup_test_context().await.unwrap();
        let repo = ctx.person_repos().persons();

        // 1. Test with an existing person
        let audit_log_id = Uuid::new_v4();
        let new_person = create_test_person_model("Alex Smith");
        repo.save(new_person.clone(), audit_log_id).await.unwrap();

        assert!(repo.exists_by_id(new_person.id).await.unwrap());

        // 2. Test with a non-existent person
        assert!(!repo.exists_by_id(Uuid::new_v4()).await.unwrap());
    }
}