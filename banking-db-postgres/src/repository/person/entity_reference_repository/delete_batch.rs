use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use banking_db::models::person::EntityReferenceModel;
use banking_db::repository::person::entity_reference_repository::EntityReferenceRepositoryError;
use banking_db::repository::{BatchRepository, EntityReferenceRepository};
use std::error::Error;
use uuid::Uuid;

pub async fn delete_batch(
    repo: &EntityReferenceRepositoryImpl,
    ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(0);
    }
    let audit_log_id = Uuid::new_v4();

    let items_to_delete = repo.load_batch(ids).await?;
    let items_to_delete: Vec<EntityReferenceModel> =
        items_to_delete.into_iter().flatten().collect();

    if items_to_delete.len() != ids.len() {
        let found_ids: std::collections::HashSet<Uuid> =
            items_to_delete.iter().map(|i| i.id).collect();
        let not_found_ids: Vec<Uuid> = ids
            .iter()
            .filter(|id| !found_ids.contains(id))
            .cloned()
            .collect();
        return Err(Box::new(
            EntityReferenceRepositoryError::ManyEntityReferencesNotFound(not_found_ids),
        ));
    }

    let cache = repo.entity_reference_idx_cache.read().await;
    for id in ids {
        cache.remove(id);
    }

    let mut entity_reference_audit_values = Vec::new();
    for item in &items_to_delete {
        if let Some(idx_model) = repo.find_by_id(item.id).await? {
            entity_reference_audit_values.push((
                item.id,
                idx_model.version,
                0, // Hash is 0 for deleted record
                item.person_id,
                item.entity_role,
                item.reference_external_id.to_string(),
                item.reference_details_l1.as_ref().map(|s| s.to_string()),
                item.reference_details_l2.as_ref().map(|s| s.to_string()),
                item.reference_details_l3.as_ref().map(|s| s.to_string()),
                audit_log_id,
            ));
        }
    }
    if !entity_reference_audit_values.is_empty() {
        repo.execute_entity_reference_audit_insert(entity_reference_audit_values)
            .await?;
    }

    let query_idx = "DELETE FROM entity_reference_idx WHERE entity_reference_id = ANY($1)";
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query_idx).bind(ids).execute(&**pool).await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query_idx).bind(ids).execute(&mut **tx).await?;
        }
    };

    let query_main = "DELETE FROM entity_reference WHERE id = ANY($1)";
    let result = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query_main).bind(ids).execute(&**pool).await?
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query_main).bind(ids).execute(&mut **tx).await?
        }
    };

    Ok(result.rows_affected() as usize)
}

#[cfg(test)]
mod tests {
    use banking_db::repository::{BatchRepository, PersonRepository, PersonRepos};
    use uuid::Uuid;

    use crate::repository::person::entity_reference_repository::create_batch::tests::{
        setup_test_entity_reference, setup_test_person,
    };
    use crate::test_helper::setup_test_context;

    #[tokio::test]
    async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();
        let entity_ref_repo = ctx.person_repos().entity_references();

        let person = setup_test_person().await;
        person_repo.save(person.clone(), Uuid::new_v4()).await?;

        let mut entity_refs = Vec::new();
        for _ in 0..4 {
            let entity_ref = setup_test_entity_reference(person.id).await;
            entity_refs.push(entity_ref);
        }
        let saved = entity_ref_repo
            .create_batch(entity_refs.clone(), Uuid::new_v4())
            .await?;
        let ids: Vec<Uuid> = saved.iter().map(|e| e.id).collect();
        let deleted_count = entity_ref_repo.delete_batch(&ids).await?;
        assert_eq!(deleted_count, 4);
        let loaded = entity_ref_repo.load_batch(&ids).await?;
        assert_eq!(loaded.len(), 4);
        assert!(loaded.iter().all(|item| item.is_none()));
        Ok(())
    }
}