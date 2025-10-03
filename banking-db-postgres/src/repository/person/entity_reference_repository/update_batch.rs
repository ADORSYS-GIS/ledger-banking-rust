use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use banking_db::models::person::{EntityReferenceIdxModel, EntityReferenceModel};
use banking_db::repository::person::entity_reference_repository::EntityReferenceRepositoryError;
use banking_db::repository::{BatchRepository, EntityReferenceRepository};
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn update_batch(
    repo: &EntityReferenceRepositoryImpl,
    items: Vec<EntityReferenceModel>,
    audit_log_id: Uuid,
) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    let existing_check = repo.exist_by_ids(&ids).await?;
    let non_existing_ids: Vec<Uuid> = existing_check
        .into_iter()
        .filter_map(|(id, exists)| if !exists { Some(id) } else { None })
        .collect();

    if !non_existing_ids.is_empty() {
        return Err(Box::new(
            EntityReferenceRepositoryError::ManyEntityReferencesNotFound(non_existing_ids),
        ));
    }

    let mut to_update = Vec::new();
    let cache = repo.entity_reference_idx_cache.read().await;
    for item in items {
        let mut hasher = XxHash64::with_seed(0);
        let mut cbor = Vec::new();
        ciborium::ser::into_writer(&item, &mut cbor).unwrap();
        hasher.write(&cbor);
        let new_hash = hasher.finish() as i64;

        if let Some(idx) = cache.get_by_primary(&item.id) {
            if idx.hash != new_hash {
                to_update.push((item, new_hash));
            }
        } else {
            return Err(Box::new(
                EntityReferenceRepositoryError::EntityReferenceNotFound(item.id),
            ));
        }
    }

    if to_update.is_empty() {
        let all_items = repo
            .load_batch(&ids)
            .await?
            .into_iter()
            .flatten()
            .collect();
        return Ok(all_items);
    }

    let mut entity_reference_values = Vec::new();
    let mut entity_reference_idx_values = Vec::new();
    let mut entity_reference_audit_values = Vec::new();
    let mut saved_items = Vec::new();

    for (item, new_hash) in to_update {
        let old_idx = cache.get_by_primary(&item.id).unwrap();
        let new_version = old_idx.version + 1;

        let mut ref_hasher = XxHash64::with_seed(0);
        ref_hasher.write(item.reference_external_id.as_bytes());
        let reference_external_id_hash = ref_hasher.finish() as i64;

        let new_idx = EntityReferenceIdxModel {
            entity_reference_id: item.id,
            person_id: item.person_id,
            reference_external_id_hash,
            version: new_version,
            hash: new_hash,
        };
        cache.update(new_idx);

        entity_reference_values.push((
            item.id,
            item.person_id,
            item.entity_role,
            item.reference_external_id.to_string(),
            item.reference_details_l1.as_ref().map(|s| s.to_string()),
            item.reference_details_l2.as_ref().map(|s| s.to_string()),
            item.reference_details_l3.as_ref().map(|s| s.to_string()),
        ));

        entity_reference_idx_values.push((item.id, item.person_id, new_version, new_hash));

        entity_reference_audit_values.push((
            item.id,
            new_version,
            new_hash,
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
        repo.execute_entity_reference_update(entity_reference_values)
            .await?;
        repo.execute_entity_reference_idx_update(entity_reference_idx_values)
            .await?;
        repo.execute_entity_reference_audit_insert(entity_reference_audit_values)
            .await?;
    }

    Ok(saved_items)
}

#[cfg(test)]
mod tests {
    use banking_db::repository::{BatchRepository, PersonRepository, PersonRepos};
    use uuid::Uuid;

    use crate::repository::person::entity_reference_repository::create_batch::tests::{
        setup_test_entity_reference, setup_test_person,
    };
    use crate::test_helper::setup_test_context;
    use heapless::String as HeaplessString;

    #[tokio::test]
    async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();
        let entity_ref_repo = ctx.person_repos().entity_references();

        let person = setup_test_person().await;
        person_repo.save(person.clone(), Uuid::new_v4()).await?;

        let mut entity_refs = Vec::new();
        for i in 0..3 {
            let mut entity_ref = setup_test_entity_reference(person.id).await;
            entity_ref.reference_external_id =
                HeaplessString::try_from(format!("ORIGINAL_{i}").as_str()).unwrap();
            entity_refs.push(entity_ref);
        }
        let saved = entity_ref_repo
            .create_batch(entity_refs.clone(), Uuid::new_v4())
            .await?;
        let mut to_update = saved.clone();
        for (i, item) in to_update.iter_mut().enumerate() {
            item.reference_external_id =
                HeaplessString::try_from(format!("UPDATED_{i}").as_str()).unwrap();
        }
        let updated = entity_ref_repo
            .update_batch(to_update, Uuid::new_v4())
            .await?;
        assert_eq!(updated.len(), 3);
        for item in updated {
            assert!(item.reference_external_id.starts_with("UPDATED"));
        }
        Ok(())
    }
}