use crate::repository::person::location_repository::LocationRepositoryImpl;
use banking_db::models::person::LocationModel;
use banking_db::repository::{
    BatchRepository, LocationRepository, LocationRepositoryError,
};
use std::error::Error;
use uuid::Uuid;

pub async fn delete_batch(
    repo: &LocationRepositoryImpl,
    ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(0);
    }

    let items_to_delete = repo.load_batch(ids).await?;
    let items_to_delete: Vec<LocationModel> = items_to_delete.into_iter().flatten().collect();

    if items_to_delete.len() != ids.len() {
        let found_ids: std::collections::HashSet<Uuid> =
            items_to_delete.iter().map(|i| i.id).collect();
        let not_found_ids: Vec<Uuid> = ids
            .iter()
            .filter(|id| !found_ids.contains(id))
            .cloned()
            .collect();
        return Err(Box::new(LocationRepositoryError::ManyLocationsNotFound(
            not_found_ids,
        )));
    }

    let cache = repo.location_idx_cache.write().await;
    for id in ids {
        cache.remove(id);
    }

    let audit_log_id = Uuid::new_v4();
    let mut location_audit_values = Vec::new();
    for item in &items_to_delete {
        if let Some(idx_model) = repo.get_idx_by_id(item.id).await? {
            location_audit_values.push((
                item.id,
                idx_model.version,
                0, // Hash is 0 for deleted record
                item.street_line1.to_string(),
                item.street_line2.as_ref().map(|s| s.to_string()),
                item.street_line3.as_ref().map(|s| s.to_string()),
                item.street_line4.as_ref().map(|s| s.to_string()),
                item.locality_id,
                item.postal_code.as_ref().map(|s| s.to_string()),
                item.latitude,
                item.longitude,
                item.accuracy_meters,
                item.location_type,
                audit_log_id,
            ));
        }
    }
    if !location_audit_values.is_empty() {
        repo.execute_location_audit_insert(location_audit_values)
            .await?;
    }

    let query_idx = "DELETE FROM location_idx WHERE location_id = ANY($1)";
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query_idx).bind(ids).execute(&**pool).await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query_idx).bind(ids).execute(&mut **tx).await?;
        }
    };

    let query_main = "DELETE FROM location WHERE id = ANY($1)";
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