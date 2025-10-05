use banking_db::repository::PersonResult;
use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use uuid::Uuid;

pub async fn exist_by_ids(
    repo: &PersonRepositoryImpl,
    ids: &[Uuid],
) -> PersonResult<Vec<(Uuid, bool)>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let cache = repo.person_idx_cache.read().await;
    Ok(ids.iter().map(|id| (*id, cache.contains_primary(id))).collect())
}
#[cfg(test)]
mod tests {
    use banking_db::models::person::{PersonModel, PersonType};
    use banking_db::repository::{BatchRepository, PersonRepository, PersonRepos};
    use heapless::String as HeaplessString;
    use uuid::Uuid;
    use crate::test_helper::setup_test_context;

    async fn setup_test_person() -> PersonModel {
        PersonModel {
            id: Uuid::new_v4(),
            person_type: PersonType::Natural,
            display_name: HeaplessString::try_from("Test Person").unwrap(),
            external_identifier: Some(HeaplessString::try_from("EXT001").unwrap()),
            entity_reference_count: 0,
            organization_person_id: None,
            messaging_info1: None,
            messaging_info2: None,
            messaging_info3: None,
            messaging_info4: None,
            messaging_info5: None,
            department: None,
            location_id: None,
            duplicate_of_person_id: None,
        }
    }

    #[tokio::test]
    async fn test_exists_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();

        let mut persons = Vec::new();
        let mut test_ids = Vec::new();

        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name =
                HeaplessString::try_from(format!("Exists Test {i}").as_str()).unwrap();
            persons.push(person.clone());
            test_ids.push(person.id);
        }

        let audit_log_id = Uuid::new_v4();
        person_repo
            .create_batch(persons, audit_log_id)
            .await?;

        // Add a non-existent ID
        test_ids.push(Uuid::new_v4());

        // Check existence
        let exists_results = person_repo
            .exist_by_ids(&test_ids)
            .await?;

        assert_eq!(exists_results.len(), 4);
        assert!(exists_results[0].1);
        assert!(exists_results[1].1);
        assert!(exists_results[2].1);
        assert!(!exists_results[3].1); // Non-existent ID

        Ok(())
    }
}