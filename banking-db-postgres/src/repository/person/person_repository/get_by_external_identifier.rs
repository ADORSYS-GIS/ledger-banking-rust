use banking_db::models::person::PersonIdxModel;
use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use std::hash::Hasher;
use twox_hash::XxHash64;

pub async fn get_by_external_identifier(
    repo: &PersonRepositoryImpl,
    identifier: &str,
) -> PersonResult<Vec<PersonIdxModel>> {
    let mut hasher = XxHash64::with_seed(0);
    hasher.write(identifier.as_bytes());
    let hash = hasher.finish() as i64;

    let cache = repo.person_idx_cache.read().await;
    let ids = cache.get_by_external_identifier_hash(&hash).unwrap_or_default();
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
    async fn test_get_by_external_identifier() {
        let ctx = setup_test_context().await.unwrap();
        let repo = ctx.person_repos().persons();
        let audit_log_id = Uuid::new_v4();

        // 1. Setup - Create a person with a known external identifier
        let person = create_test_person_model("Eva Core");
        let external_id = person.external_identifier.clone().unwrap();
        repo.save(person.clone(), audit_log_id).await.unwrap();

        // 2. Test finding the person by their external identifier
        let found_persons = repo
            .get_by_external_identifier(external_id.as_str())
            .await
            .unwrap();

        // 3. Assertions
        assert_eq!(found_persons.len(), 1);
        assert_eq!(found_persons[0].person_id, person.id);

        // 4. Test with a non-existent external identifier
        let found_persons = repo
            .get_by_external_identifier("non-existent-id")
            .await
            .unwrap();
        assert!(found_persons.is_empty());
    }
}