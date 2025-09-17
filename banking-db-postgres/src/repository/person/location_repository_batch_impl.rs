use crate::repository::person::location_repository_impl::LocationRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{
    LocationIdxModel, LocationModel, LocationType,
};
use rust_decimal::Decimal;
use banking_db::repository::{
    BatchRepository, LocationRepository, LocationRepositoryError,
};
use sqlx::Postgres;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

type LocationTuple = (
    Uuid,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Uuid,
    Option<String>,
    Option<Decimal>,
    Option<Decimal>,
    Option<f32>,
    LocationType,
);

type LocationAuditTuple = (
    Uuid,
    i32,
    i64,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Uuid,
    Option<String>,
    Option<Decimal>,
    Option<Decimal>,
    Option<f32>,
    LocationType,
    Uuid,
);

#[async_trait]
impl BatchRepository<Postgres, LocationModel> for LocationRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<LocationModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        let existing_check = self.exist_by_ids(&ids).await?;
        let truly_existing_ids: Vec<Uuid> = existing_check
            .into_iter()
            .filter_map(|(id, exists)| if exists { Some(id) } else { None })
            .collect();

        if !truly_existing_ids.is_empty() {
            return Err(Box::new(LocationRepositoryError::ManyLocationsExist(
                truly_existing_ids,
            )));
        }

        let cache = self.location_idx_cache.read().await;
        for item in &items {
            let mut hasher = XxHash64::with_seed(0);
            let mut cbor = Vec::new();
            ciborium::ser::into_writer(item, &mut cbor).unwrap();
            hasher.write(&cbor);
            let hash = hasher.finish() as i64;

            let idx_model = LocationIdxModel {
                location_id: item.id,
                locality_id: item.locality_id,
                version: 0,
                hash,
            };
            cache.add(idx_model);
        }

        let mut location_values = Vec::new();
        let mut location_idx_values = Vec::new();
        let mut location_audit_values = Vec::new();
        let mut saved_items = Vec::new();

        for item in items {
            let idx_model = cache.get_by_primary(&item.id).unwrap();

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

            location_idx_values.push((item.id, item.locality_id, 0i32, idx_model.hash));

            location_audit_values.push((
                item.id,
                0i32,
                idx_model.hash,
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
            self.execute_location_insert(location_values).await?;
            self.execute_location_idx_insert(location_idx_values)
                .await?;
            self.execute_location_audit_insert(location_audit_values)
                .await?;
        }

        Ok(saved_items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<LocationModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = r#"SELECT * FROM location WHERE id = ANY($1)"#;
        let rows = match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query).bind(ids).fetch_all(&**pool).await?
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
            }
        };
        let mut item_map = HashMap::new();
        for row in rows {
            let item = LocationModel::try_from_row(&row)?;
            item_map.insert(item.id, item);
        }
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(item_map.remove(id));
        }
        Ok(result)
    }

    async fn update_batch(
        &self,
        items: Vec<LocationModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<LocationModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        let existing_check = self.exist_by_ids(&ids).await?;
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
        let cache = self.location_idx_cache.read().await;
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
            let all_items = self.load_batch(&ids).await?.into_iter().flatten().collect();
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
            self.execute_location_update(location_values).await?;
            self.execute_location_idx_update(location_idx_values).await?;
            self.execute_location_audit_insert(location_audit_values)
                .await?;
        }

        Ok(saved_items)
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(0);
        }

        let items_to_delete = self.load_batch(ids).await?;
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

        let cache = self.location_idx_cache.write().await;
        for id in ids {
            cache.remove(id);
        }

        let audit_log_id = Uuid::new_v4();
        let mut location_audit_values = Vec::new();
        for item in &items_to_delete {
            if let Some(idx_model) = self.get_idx_by_id(item.id).await? {
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
            self.execute_location_audit_insert(location_audit_values)
                .await?;
        }

        let query_idx = "DELETE FROM location_idx WHERE location_id = ANY($1)";
        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query_idx).bind(ids).execute(&**pool).await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query_idx).bind(ids).execute(&mut **tx).await?;
            }
        };

        let query_main = "DELETE FROM location WHERE id = ANY($1)";
        let result = match &self.executor {
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
}

impl LocationRepositoryImpl {
    async fn execute_location_insert(
        &self,
        values: Vec<LocationTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            ids,
            street_line1s,
            street_line2s,
            street_line3s,
            street_line4s,
            locality_ids,
            postal_codes,
            latitudes,
            longitudes,
            accuracy_meters,
            location_types,
        ) = values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc.4.push(val.4);
                acc.5.push(val.5);
                acc.6.push(val.6);
                acc.7.push(val.7);
                acc.8.push(val.8);
                acc.9.push(val.9);
                acc.10.push(val.10);
                acc
            },
        );

        let query = r#"
            INSERT INTO location (id, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[], $6::uuid[], $7::text[], $8::numeric[], $9::numeric[], $10::real[], $11::location_type[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_location_idx_insert(
        &self,
        values: Vec<(Uuid, Uuid, i32, i64)>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (location_ids, locality_ids, versions, hashes) =
            values.into_iter().fold((Vec::new(), Vec::new(), Vec::new(), Vec::new()), |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc
            });

        let query = r#"
            INSERT INTO location_idx (location_id, locality_id, version, hash)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::int[], $4::bigint[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_location_audit_insert(
        &self,
        values: Vec<LocationAuditTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            location_ids,
            versions,
            hashes,
            street_line1s,
            street_line2s,
            street_line3s,
            street_line4s,
            locality_ids,
            postal_codes,
            latitudes,
            longitudes,
            accuracy_meters,
            location_types,
            audit_log_ids,
        ) = values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc.4.push(val.4);
                acc.5.push(val.5);
                acc.6.push(val.6);
                acc.7.push(val.7);
                acc.8.push(val.8);
                acc.9.push(val.9);
                acc.10.push(val.10);
                acc.11.push(val.11);
                acc.12.push(val.12);
                acc.13.push(val.13);
                acc
            },
        );

        let query = r#"
            INSERT INTO location_audit (location_id, version, hash, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
            SELECT * FROM UNNEST($1::uuid[], $2::int[], $3::bigint[], $4::text[], $5::text[], $6::text[], $7::text[], $8::uuid[], $9::text[], $10::numeric[], $11::numeric[], $12::real[], $13::location_type[], $14::uuid[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(versions)
                    .bind(hashes)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .bind(audit_log_ids)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(versions)
                    .bind(hashes)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .bind(audit_log_ids)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
    async fn execute_location_update(
        &self,
        values: Vec<LocationTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            ids,
            street_line1s,
            street_line2s,
            street_line3s,
            street_line4s,
            locality_ids,
            postal_codes,
            latitudes,
            longitudes,
            accuracy_meters,
            location_types,
        ) = values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc.4.push(val.4);
                acc.5.push(val.5);
                acc.6.push(val.6);
                acc.7.push(val.7);
                acc.8.push(val.8);
                acc.9.push(val.9);
                acc.10.push(val.10);
                acc
            },
        );

        let query = r#"
            UPDATE location SET
                street_line1 = u.street_line1,
                street_line2 = u.street_line2,
                street_line3 = u.street_line3,
                street_line4 = u.street_line4,
                locality_id = u.locality_id,
                postal_code = u.postal_code,
                latitude = u.latitude,
                longitude = u.longitude,
                accuracy_meters = u.accuracy_meters,
                location_type = u.location_type
            FROM (
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[], $6::uuid[],
                    $7::text[], $8::numeric[], $9::numeric[], $10::real[], $11::location_type[]
                )
            ) AS u(id, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type)
            WHERE location.id = u.id
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_location_idx_update(
        &self,
        values: Vec<(Uuid, Uuid, i32, i64)>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (location_ids, locality_ids, versions, hashes) =
            values.into_iter().fold((Vec::new(), Vec::new(), Vec::new(), Vec::new()), |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc
            });

        let query = r#"
            UPDATE location_idx SET
                locality_id = u.locality_id,
                version = u.version,
                hash = u.hash
            FROM (
                SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::int[], $4::bigint[])
            ) AS u(location_id, locality_id, version, hash)
            WHERE location_idx.location_id = u.location_id
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}