use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use banking_db::models::person::{EntityReferenceIdxModel, EntityReferenceModel};
use banking_db::repository::EntityReferenceRepository;
use banking_db::repository::person::entity_reference_repository::EntityReferenceRepositoryError;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn create_batch(
    repo: &EntityReferenceRepositoryImpl,
    items: Vec<EntityReferenceModel>,
    audit_log_id: Uuid,
) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_check = repo.exist_by_ids(&ids).await?;
    let truly_existing_ids: Vec<Uuid> = existing_check
        .into_iter()
        .filter_map(|(id, exists)| if exists { Some(id) } else { None })
        .collect();

    if !truly_existing_ids.is_empty() {
        return Err(Box::new(
            EntityReferenceRepositoryError::ManyEntityReferencesExist(truly_existing_ids),
        ));
    }

    let cache = repo.entity_reference_idx_cache.read().await;
    for item in &items {
        let mut hasher = XxHash64::with_seed(0);
        let mut cbor = Vec::new();
        ciborium::ser::into_writer(item, &mut cbor).unwrap();
        hasher.write(&cbor);
        let hash = hasher.finish() as i64;

        let mut ref_hasher = XxHash64::with_seed(0);
        ref_hasher.write(item.reference_external_id.as_bytes());
        let reference_external_id_hash = ref_hasher.finish() as i64;

        let idx_model = EntityReferenceIdxModel {
            entity_reference_id: item.id,
            person_id: item.person_id,
            reference_external_id_hash,
            version: 0,
            hash,
        };
        cache.add(idx_model);
    }

    let mut entity_reference_values = Vec::new();
    let mut entity_reference_idx_values = Vec::new();
    let mut entity_reference_audit_values = Vec::new();
    let mut saved_items = Vec::new();

    for item in items {
        let idx_model = cache.get_by_primary(&item.id).unwrap();

        entity_reference_values.push((
            item.id,
            item.person_id,
            item.entity_role,
            item.reference_external_id.to_string(),
            item.reference_details_l1.as_ref().map(|s| s.to_string()),
            item.reference_details_l2.as_ref().map(|s| s.to_string()),
            item.reference_details_l3.as_ref().map(|s| s.to_string()),
        ));

        entity_reference_idx_values.push((item.id, item.person_id, 0i32, idx_model.hash));

        entity_reference_audit_values.push((
            item.id,
            0i32,
            idx_model.hash,
            item.person_id,
            item.entity_role,
            item.reference_external_id.to_string(),
            item.reference_details_l1.as_ref().map(|s| s.to_string()),
            item.reference_details_l2.as_ref().map(|s| s.to_string()),
            item.reference_details_l3.as_ref().map(|s| s.to_string()),
            audit_log_id,
        ));
        saved_items.push(item);
    }

    if !entity_reference_values.is_empty() {
        repo.execute_entity_reference_insert(entity_reference_values)
            .await?;
        repo.execute_entity_reference_idx_insert(entity_reference_idx_values)
            .await?;
        repo.execute_entity_reference_audit_insert(entity_reference_audit_values)
            .await?;
    }

    Ok(saved_items)
}

#[cfg(test)]
pub mod tests {
    use banking_db::models::person::{
        EntityReferenceModel, PersonModel, PersonType, RelationshipRole,
    };
    use banking_db::repository::{
        BatchRepository, EntityReferenceRepository, PersonRepository, PersonRepos,
    };
    use heapless::String as HeaplessString;
    use uuid::Uuid;

    use crate::test_helper::setup_test_context;

    pub async fn setup_test_person() -> PersonModel {
        PersonModel {
            id: Uuid::new_v4(),
            person_type: PersonType::Natural,
            display_name: HeaplessString::try_from("Test Person").unwrap(),
            external_identifier: Some(HeaplessString::try_from("EXT_PERSON_001").unwrap()),
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

    pub async fn setup_test_entity_reference(person_id: Uuid) -> EntityReferenceModel {
        EntityReferenceModel {
            id: Uuid::new_v4(),
            person_id,
            entity_role: RelationshipRole::Customer,
            reference_external_id: HeaplessString::try_from("EXT_REF_001").unwrap(),
            reference_details_l1: None,
            reference_details_l2: None,
            reference_details_l3: None,
        }
    }

    #[tokio::test]
    async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();
        let entity_ref_repo = ctx.person_repos().entity_references();

        let person = setup_test_person().await;
        person_repo.save(person.clone(), Uuid::new_v4()).await?;

        let mut entity_refs = Vec::new();
        for i in 0..5 {
            let mut entity_ref = setup_test_entity_reference(person.id).await;
            entity_ref.reference_external_id =
                HeaplessString::try_from(format!("EXT_REF_{i:03}").as_str()).unwrap();
            entity_refs.push(entity_ref);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_entity_refs = entity_ref_repo
            .create_batch(entity_refs.clone(), audit_log_id)
            .await?;

        assert_eq!(saved_entity_refs.len(), 5);

        for entity_ref in &saved_entity_refs {
            assert!(entity_ref_repo.exists_by_id(entity_ref.id).await?);
        }

        Ok(())
    }
}