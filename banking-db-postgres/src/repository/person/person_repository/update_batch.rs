use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use banking_db::models::person::PersonModel;
use banking_db::repository::{
    BatchOperationStats, LocationRepository, PersonRepository, PersonRepositoryError,
};
use std::error::Error;
use std::hash::Hasher;
use std::time::Instant;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn update_batch(
    repo: &PersonRepositoryImpl,
    items: Vec<PersonModel>,
    audit_log_id: Uuid,
) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }
    let start = Instant::now();
    let mut updated_items = Vec::new();
    let mut stats = BatchOperationStats {
        total_items: items.len(),
        ..Default::default()
    };
    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_persons_check = repo.exist_by_ids(&ids).await?;
    let missing_ids: Vec<Uuid> = existing_persons_check
        .into_iter()
        .filter_map(|(id, exists)| if !exists { Some(id) } else { None })
        .collect();
    if !missing_ids.is_empty() {
        return Err(Box::new(PersonRepositoryError::ManyPersonsNotFound(
            missing_ids,
        )));
    }
    let cache = repo.person_idx_cache.write().await;
    let mut person_values = Vec::new();
    let mut person_idx_values = Vec::new();
    let mut person_audit_values = Vec::new();
    for person in items {
        let mut hasher = XxHash64::with_seed(0);
        let mut person_cbor = Vec::new();
        ciborium::ser::into_writer(&person, &mut person_cbor).unwrap();
        hasher.write(&person_cbor);
        let new_hash = hasher.finish() as i64;
        if let Some(existing_idx) = cache.get_by_primary(&person.id) {
            if existing_idx.hash == new_hash {
                stats.skipped_items += 1;
                continue;
            }
            let new_version = existing_idx.version + 1;
            let external_hash = person.external_identifier.as_ref().map(|s| {
                let mut h = XxHash64::with_seed(0);
                h.write(s.as_bytes());
                h.finish() as i64
            });
            person_values.push((
                person.id,
                person.person_type,
                person.display_name.to_string(),
                person.external_identifier.as_ref().map(|s| s.to_string()),
                person.organization_person_id,
                person.department.as_ref().map(|s| s.to_string()),
                person.location_id,
                person.duplicate_of_person_id,
                person.entity_reference_count,
            ));
            person_idx_values.push((person.id, external_hash, new_version, new_hash));
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
                audit_log_id,
            ));
            let mut updated_idx = existing_idx.clone();
            updated_idx.version = new_version;
            updated_idx.hash = new_hash;
            updated_idx.external_identifier_hash = external_hash;
            updated_idx.organization_person_id = person.organization_person_id;
            updated_idx.duplicate_of_person_id = person.duplicate_of_person_id;
            cache.update(updated_idx);
            updated_items.push(person);
            stats.successful_items += 1;
        } else {
            stats.failed_items += 1;
        }
    }
    // location validation
    let mut invalid_location_ids = Vec::new();
    for tuple in &person_values {
        if let Some(loc_id) = tuple.6 {
            if !repo.location_repository.exists_by_id(loc_id).await? {
                invalid_location_ids.push(loc_id);
            }
        }
    }
    if !invalid_location_ids.is_empty() {
        return Err(Box::new(PersonRepositoryError::InvalidLocations(
            invalid_location_ids,
        )));
    }
    // hierarchy validation
    let mut missing_org_ids = Vec::new();
    let mut missing_dup_ids = Vec::new();
    for tuple in &person_values {
        if let Some(org_id) = tuple.4 {
            if !cache.contains_primary(&org_id) {
                missing_org_ids.push(org_id);
            }
        }
        if let Some(dup_id) = tuple.7 {
            if !cache.contains_primary(&dup_id) {
                missing_dup_ids.push(dup_id);
            }
        }
    }
    if !missing_org_ids.is_empty() {
        return Err(Box::new(PersonRepositoryError::ManyOrganizationsNotFound(
            missing_org_ids,
        )));
    }
    if !missing_dup_ids.is_empty() {
        return Err(Box::new(PersonRepositoryError::ManyPersonsNotFound(
            missing_dup_ids,
        )));
    }
    if !person_values.is_empty() {
        crate::repository::person::person_repository::batch_helper::execute_person_update(
            repo,
            person_values,
        )
        .await?;
        crate::repository::person::person_repository::batch_helper::execute_person_idx_update(
            repo,
            person_idx_values,
        )
        .await?;
        crate::repository::person::person_repository::batch_helper::execute_person_audit_insert(
            repo,
            person_audit_values,
        )
        .await?;
    }
    stats.duration_ms = start.elapsed().as_millis() as u64;
    Ok(updated_items)
}
#[cfg(test)]
mod tests {
    use banking_db::models::person::{PersonModel, PersonType};
    use banking_db::repository::{BatchRepository, PersonRepos};
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
    async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();

        let mut persons = Vec::new();
        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name = HeaplessString::try_from(format!("Original {i}").as_str()).unwrap();
            persons.push(person);
        }

        let audit_log_id = Uuid::new_v4();
        let saved_persons = person_repo
            .create_batch(persons.clone(), audit_log_id)
            .await?;

        // Update display names
        let mut updated_persons = saved_persons.clone();
        for (i, person) in updated_persons.iter_mut().enumerate() {
            person.display_name = HeaplessString::try_from(format!("Updated {i}").as_str()).unwrap();
        }

        person_repo
            .update_batch(updated_persons.clone(), audit_log_id)
            .await?;

        // Verify updates
        let test_ids: Vec<Uuid> = updated_persons.iter().map(|p| p.id).collect();
        let loaded_persons = person_repo
            .load_batch(&test_ids)
            .await?;

        let mut loaded_persons: Vec<PersonModel> = loaded_persons.into_iter().flatten().collect();
        assert_eq!(
            loaded_persons.len(),
            3,
            "Expected to load 3 persons, but found {}",
            loaded_persons.len()
        );
        loaded_persons.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        for (i, person) in loaded_persons.iter().enumerate().take(3) {
            assert_eq!(person.display_name.as_str(), format!("Updated {i}"));
        }

        Ok(())
    }
}