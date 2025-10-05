use crate::repository::person::location_repository::LocationRepositoryImpl;
use banking_db::models::person::{LocationIdxModel, LocationModel};
use banking_db::repository::{
    BatchRepository, LocationRepository, LocationRepositoryError,
};
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn update_batch(
    repo: &LocationRepositoryImpl,
    items: Vec<LocationModel>,
    audit_log_id: Uuid,
) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>> {
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
            LocationRepositoryError::ManyLocationsNotFound(non_existing_ids),
        ));
    }

    let mut to_update = Vec::new();
    let cache = repo.location_idx_cache.read().await;
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
            return Err(Box::new(LocationRepositoryError::LocationNotFound(item.id)));
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

    let mut location_values = Vec::new();
    let mut location_idx_values = Vec::new();
    let mut location_audit_values = Vec::new();
    let mut saved_items = Vec::new();

    for (item, new_hash) in to_update {
        let old_idx = cache.get_by_primary(&item.id).unwrap();
        let new_version = old_idx.version + 1;

        let new_idx = LocationIdxModel {
            location_id: item.id,
            locality_id: item.locality_id,
            version: new_version,
            hash: new_hash,
        };
        cache.add(new_idx);

        location_values.push((
            item.id,
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
        ));

        location_idx_values.push((item.id, item.locality_id, new_version, new_hash));

        location_audit_values.push((
            item.id,
            new_version,
            new_hash,
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
        saved_items.push(item);
    }

    if !location_values.is_empty() {
        repo.execute_location_update(location_values).await?;
        repo.execute_location_idx_update(location_idx_values)
            .await?;
        repo.execute_location_audit_insert(location_audit_values)
            .await?;
    }

    Ok(saved_items)
}