use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use banking_db::repository::{BatchRepository, PersonRepository, PersonRepositoryError};
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn delete_batch(
    repo: &PersonRepositoryImpl,
    ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut person_audit_values = Vec::new();
    let existings = repo.find_by_ids(ids).await?;
    let existing_ids: Vec<Uuid> = existings.iter().map(|p| p.person_id).collect();
    {
        let cache = repo.person_idx_cache.write().await;
        for id in &existing_ids {
            cache.remove(id);
        }
    }
    let mut dependent_duplicates = Vec::new();
    for id in &existing_ids {
        if !repo.find_by_duplicate_of_person_id(*id).await?.is_empty() {
            dependent_duplicates.push(*id);
        }
    }
    if !dependent_duplicates.is_empty() {
        return Err(Box::new(PersonRepositoryError::IsDuplicatePersonFor(
            dependent_duplicates,
        )));
    }
    let mut dependent_organizations = Vec::new();
    for id in &existing_ids {
        if !repo.find_by_organization_person_id(*id).await?.is_empty() {
            dependent_organizations.push(*id);
        }
    }
    if !dependent_organizations.is_empty() {
        return Err(Box::new(PersonRepositoryError::IsOrganizationPersonFor(
            dependent_organizations,
        )));
    }
    let items = repo.load_batch(&existing_ids).await?;
    for person in items.into_iter().flatten() {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(&[]);
        let new_hash = hasher.finish() as i64;
        let existing_idx = existings.iter().find(|p| p.person_id == person.id).unwrap();
        let new_version = existing_idx.version + 1;
        person_audit_values.push((
            person.id,
            new_version,
            new_hash,
            person.person_type,
            person.display_name.to_string(),
            person.external_identifier.as_ref().map(|s| s.to_string()),
            person.organization_person_id,
            person.department.as_ref().map(|s| s.to_string()),
            person.location_id,
            person.duplicate_of_person_id,
            person.entity_reference_count,
            Uuid::new_v4(),
        ));
    }
    let delete_query = "DELETE FROM person WHERE id = ANY($1)";
    let delete_idx_query = "DELETE FROM person_idx WHERE person_id = ANY($1)";
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(delete_idx_query).bind(&existing_ids).execute(&**pool).await?;
            sqlx::query(delete_query).bind(&existing_ids).execute(&**pool).await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(delete_idx_query).bind(&existing_ids).execute(&mut **tx).await?;
            sqlx::query(delete_query).bind(&existing_ids).execute(&mut **tx).await?;
        }
    }
    crate::repository::person::person_repository::batch_helper::execute_person_audit_insert(
        repo,
        person_audit_values,
    )
    .await?;
    Ok(existing_ids.len())
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
    async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();

        let mut persons = Vec::new();
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name =
                HeaplessString::try_from(format!("Delete Test {i}").as_str()).unwrap();
            persons.push(person);
        }

        let audit_log_id = Uuid::new_v4();
        let saved_persons = person_repo
            .create_batch(persons, audit_log_id)
            .await?;

        // Delete first two persons
        let ids_to_delete: Vec<Uuid> = saved_persons.iter().take(2).map(|p| p.id).collect();

        person_repo
            .delete_batch(&ids_to_delete)
            .await?;

        // Verify deletions
        let all_ids: Vec<Uuid> = saved_persons.iter().map(|p| p.id).collect();
        let exists_results = person_repo
            .exist_by_ids(&all_ids)
            .await?;

        assert!(!exists_results[0].1); // Deleted
        assert!(!exists_results[1].1); // Deleted
        assert!(exists_results[2].1); // Still exists

        Ok(())
    }
}