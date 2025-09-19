use crate::repository::person::entity_reference_repository_impl::EntityReferenceRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::{
    EntityReferenceIdxModel, EntityReferenceModel, RelationshipRole,
};
use banking_db::repository::{
    BatchRepository, EntityReferenceRepository,
    person::entity_reference_repository::{EntityReferenceRepositoryError},
};
use crate::utils::TryFromRow;
use sqlx::Postgres;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

type EntityReferenceTuple = (
    Uuid,
    Uuid,
    RelationshipRole,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
);

type EntityReferenceAuditTuple = (
    Uuid,
    i32,
    i64,
    Uuid,
    RelationshipRole,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Uuid,
);

#[async_trait]
impl BatchRepository<Postgres, EntityReferenceModel> for EntityReferenceRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<EntityReferenceModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
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
            return Err(Box::new(
                EntityReferenceRepositoryError::ManyEntityReferencesExist(truly_existing_ids),
            ));
        }

        let cache = self.entity_reference_idx_cache.read().await;
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
            self.execute_entity_reference_insert(entity_reference_values)
                .await?;
            self.execute_entity_reference_idx_insert(entity_reference_idx_values)
                .await?;
            self.execute_entity_reference_audit_insert(entity_reference_audit_values)
                .await?;
        }

        Ok(saved_items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<EntityReferenceModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = r#"SELECT * FROM entity_reference WHERE id = ANY($1)"#;
        let rows = match &self.executor {
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

    async fn update_batch(
        &self,
        items: Vec<EntityReferenceModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<EntityReferenceModel>, Box<dyn Error + Send + Sync>> {
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
                EntityReferenceRepositoryError::ManyEntityReferencesNotFound(non_existing_ids),
            ));
        }

        let mut to_update = Vec::new();
        let cache = self.entity_reference_idx_cache.read().await;
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
                return Err(Box::new(EntityReferenceRepositoryError::EntityReferenceNotFound(
                    item.id,
                )));
            }
        }

        if to_update.is_empty() {
            let all_items = self
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
            self.execute_entity_reference_update(entity_reference_values)
                .await?;
            self.execute_entity_reference_idx_update(entity_reference_idx_values)
                .await?;
            self.execute_entity_reference_audit_insert(entity_reference_audit_values)
                .await?;
        }

        Ok(saved_items)
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(0);
        }
        let audit_log_id = Uuid::new_v4();

        let items_to_delete = self.load_batch(ids).await?;
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

        let cache = self.entity_reference_idx_cache.read().await;
        for id in ids {
            cache.remove(id);
        }

        let mut entity_reference_audit_values = Vec::new();
        for item in &items_to_delete {
            if let Some(idx_model) = self.find_by_id(item.id).await? {
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
            self.execute_entity_reference_audit_insert(entity_reference_audit_values)
                .await?;
        }

        let query_idx = "DELETE FROM entity_reference_idx WHERE entity_reference_id = ANY($1)";
        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query_idx).bind(ids).execute(&**pool).await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query_idx).bind(ids).execute(&mut **tx).await?;
            }
        };

        let query_main = "DELETE FROM entity_reference WHERE id = ANY($1)";
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

impl EntityReferenceRepositoryImpl {
    async fn execute_entity_reference_insert(
        &self,
        values: Vec<EntityReferenceTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            ids,
            person_ids,
            entity_roles,
            reference_external_ids,
            reference_details_l1s,
            reference_details_l2s,
            reference_details_l3s,
        ) = values.into_iter().fold(
            (
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
                acc
            },
        );

        let query = r#"
            INSERT INTO entity_reference (id, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::person_entity_type[], $4::text[], $5::text[], $6::text[], $7::text[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(person_ids)
                    .bind(entity_roles)
                    .bind(reference_external_ids)
                    .bind(reference_details_l1s)
                    .bind(reference_details_l2s)
                    .bind(reference_details_l3s)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(person_ids)
                    .bind(entity_roles)
                    .bind(reference_external_ids)
                    .bind(reference_details_l1s)
                    .bind(reference_details_l2s)
                    .bind(reference_details_l3s)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_entity_reference_idx_insert(
        &self,
        values: Vec<(Uuid, Uuid, i32, i64)>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (entity_reference_ids, person_ids, versions, hashes) = values.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc
            },
        );

        let query = r#"
            INSERT INTO entity_reference_idx (entity_reference_id, person_id, version, hash)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::int[], $4::bigint[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(entity_reference_ids)
                    .bind(person_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(entity_reference_ids)
                    .bind(person_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_entity_reference_audit_insert(
        &self,
        values: Vec<EntityReferenceAuditTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            entity_reference_ids,
            versions,
            hashes,
            person_ids,
            entity_roles,
            reference_external_ids,
            reference_details_l1s,
            reference_details_l2s,
            reference_details_l3s,
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
                acc
            },
        );

        let query = r#"
            INSERT INTO entity_reference_audit (entity_reference_id, version, hash, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3, audit_log_id)
            SELECT * FROM UNNEST($1::uuid[], $2::int[], $3::bigint[], $4::uuid[], $5::person_entity_type[], $6::text[], $7::text[], $8::text[], $9::text[], $10::uuid[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(entity_reference_ids)
                    .bind(versions)
                    .bind(hashes)
                    .bind(person_ids)
                    .bind(entity_roles)
                    .bind(reference_external_ids)
                    .bind(reference_details_l1s)
                    .bind(reference_details_l2s)
                    .bind(reference_details_l3s)
                    .bind(audit_log_ids)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(entity_reference_ids)
                    .bind(versions)
                    .bind(hashes)
                    .bind(person_ids)
                    .bind(entity_roles)
                    .bind(reference_external_ids)
                    .bind(reference_details_l1s)
                    .bind(reference_details_l2s)
                    .bind(reference_details_l3s)
                    .bind(audit_log_ids)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_entity_reference_update(
        &self,
        values: Vec<EntityReferenceTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            ids,
            person_ids,
            entity_roles,
            reference_external_ids,
            reference_details_l1s,
            reference_details_l2s,
            reference_details_l3s,
        ) = values.into_iter().fold(
            (
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
                acc
            },
        );

        let query = r#"
            UPDATE entity_reference SET
                person_id = u.person_id,
                entity_role = u.entity_role,
                reference_external_id = u.reference_external_id,
                reference_details_l1 = u.reference_details_l1,
                reference_details_l2 = u.reference_details_l2,
                reference_details_l3 = u.reference_details_l3
            FROM (
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::uuid[], $3::person_entity_type[], $4::text[], $5::text[], $6::text[], $7::text[]
                )
            ) AS u(id, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3)
            WHERE entity_reference.id = u.id
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(person_ids)
                    .bind(entity_roles)
                    .bind(reference_external_ids)
                    .bind(reference_details_l1s)
                    .bind(reference_details_l2s)
                    .bind(reference_details_l3s)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(person_ids)
                    .bind(entity_roles)
                    .bind(reference_external_ids)
                    .bind(reference_details_l1s)
                    .bind(reference_details_l2s)
                    .bind(reference_details_l3s)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_entity_reference_idx_update(
        &self,
        values: Vec<(Uuid, Uuid, i32, i64)>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (entity_reference_ids, person_ids, versions, hashes) = values.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc
            },
        );

        let query = r#"
            UPDATE entity_reference_idx SET
                person_id = u.person_id,
                version = u.version,
                hash = u.hash
            FROM (
                SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::int[], $4::bigint[])
            ) AS u(entity_reference_id, person_id, version, hash)
            WHERE entity_reference_idx.entity_reference_id = u.entity_reference_id
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(entity_reference_ids)
                    .bind(person_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(entity_reference_ids)
                    .bind(person_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}