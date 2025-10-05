use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use banking_db::models::person::{PersonIdxModel, PersonModel};
use banking_db::repository::{
    BatchOperationStats, LocationRepository, PersonRepository, PersonRepositoryError,
};
use std::error::Error;
use std::hash::Hasher;
use std::time::Instant;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn create_batch(
    repo: &PersonRepositoryImpl,
    items: Vec<PersonModel>,
    audit_log_id: Uuid,
) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let start = Instant::now();
    let mut saved_items = Vec::with_capacity(items.len());
    let mut stats = BatchOperationStats {
        total_items: items.len(),
        ..Default::default()
    };

    // filter ids into a vec
    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_persons_check = repo.exist_by_ids(&ids).await?;
    let truly_existing_ids: Vec<Uuid> = existing_persons_check
        .into_iter()
        .filter_map(|(id, exists)| if exists { Some(id) } else { None })
        .collect();

    if !truly_existing_ids.is_empty() {
        return Err(Box::new(PersonRepositoryError::ManyPersonsExists(
            truly_existing_ids,
        )));
    }

    // location validation
    let mut invalid_location_ids = Vec::new();
    for person in &items {
        if let Some(loc_id) = person.location_id {
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

    // prepare idx cache
    let cache = repo.person_idx_cache.read().await;
    for person in &items {
        let mut hasher = XxHash64::with_seed(0);
        let mut person_cbor = Vec::new();
        ciborium::ser::into_writer(person, &mut person_cbor).unwrap();
        hasher.write(&person_cbor);
        let hash = hasher.finish() as i64;

        let external_hash = person.external_identifier.as_ref().map(|s| {
            let mut h = XxHash64::with_seed(0);
            h.write(s.as_bytes());
            h.finish() as i64
        });

        cache.add(PersonIdxModel {
            person_id: person.id,
            external_identifier_hash: external_hash,
            organization_person_id: person.organization_person_id,
            duplicate_of_person_id: person.duplicate_of_person_id,
            version: 0,
            hash,
        });
    }

    // hierarchy validation
    let mut missing_org_ids = Vec::new();
    let mut missing_dup_ids = Vec::new();
    for person in &items {
        if let Some(org_id) = person.organization_person_id {
            if !cache.contains_primary(&org_id) {
                missing_org_ids.push(org_id);
            }
        }
        if let Some(dup_id) = person.duplicate_of_person_id {
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

    // prepare batch data
    let mut person_values = Vec::new();
    let mut person_idx_values = Vec::new();
    let mut person_audit_values = Vec::new();

    for person in items {
        let idx_model = cache.get_by_primary(&person.id).unwrap();
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

        person_idx_values.push((
            person.id,
            idx_model.external_identifier_hash,
            0i32,
            idx_model.hash,
        ));

        person_audit_values.push((
            person.id,
            0i32,
            idx_model.hash,
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

        saved_items.push(person);
        stats.successful_items += 1;
    }

    if !person_values.is_empty() {
        crate::repository::person::person_repository::batch_helper::execute_person_insert(
            repo,
            person_values,
        )
        .await?;
        crate::repository::person::person_repository::batch_helper::execute_person_idx_insert(
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
    Ok(saved_items)
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
    async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();

        let mut persons = Vec::new();
        for i in 0..5 {
            let mut person = setup_test_person().await;
            person.display_name =
                HeaplessString::try_from(format!("Test Person {i}").as_str()).unwrap();
            person.external_identifier =
                Some(HeaplessString::try_from(format!("EXT{i:03}").as_str()).unwrap());
            persons.push(person);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_persons = person_repo
            .create_batch(persons.clone(), audit_log_id)
            .await?;

        assert_eq!(saved_persons.len(), 5);

        for person in &saved_persons {
            assert!(person_repo.exists_by_id(person.id).await?);
        }

        Ok(())
    }
}