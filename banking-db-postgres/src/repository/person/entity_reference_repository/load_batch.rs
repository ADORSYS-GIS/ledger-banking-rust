use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use banking_db::models::person::EntityReferenceModel;
use crate::utils::TryFromRow;
use std::error::Error;
use uuid::Uuid;

pub async fn load_batch(
    repo: &EntityReferenceRepositoryImpl,
    ids: &[Uuid],
) -> Result<Vec<Option<EntityReferenceModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = r#"SELECT * FROM entity_reference WHERE id = ANY($1)"#;
    let rows = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query).bind(ids).fetch_all(&**pool).await?
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
        }
    };
    let mut item_map = std::collections::HashMap::new();
    for row in rows {
        let item = EntityReferenceModel::try_from_row(&row)?;
        item_map.insert(item.id, item);
    }
    let mut result = Vec::with_capacity(ids.len());
    for id in ids {
        result.push(item_map.remove(id));
    }
    Ok(result)
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
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();
        let entity_ref_repo = ctx.person_repos().entity_references();

        let person = setup_test_person().await;
        person_repo.save(person.clone(), Uuid::new_v4()).await?;

        let mut entity_refs = Vec::new();
        for i in 0..3 {
            let mut entity_ref = setup_test_entity_reference(person.id).await;
            entity_ref.reference_external_id =
                HeaplessString::try_from(format!("LOAD_EXT_REF_{i:03}").as_str()).unwrap();
            entity_refs.push(entity_ref);
        }
        entity_ref_repo
            .create_batch(entity_refs.clone(), Uuid::new_v4())
            .await?;
        let ids: Vec<Uuid> = entity_refs.iter().map(|e| e.id).collect();
        let loaded = entity_ref_repo.load_batch(&ids).await?;
        assert_eq!(loaded.len(), 3);
        assert!(loaded.iter().all(|item| item.is_some()));
        Ok(())
    }
}