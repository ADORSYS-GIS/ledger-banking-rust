use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use banking_db::models::person::RelationshipRole;
use std::error::Error;
use uuid::Uuid;

pub type EntityReferenceTuple = (
    Uuid,
    Uuid,
    RelationshipRole,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
);

pub type EntityReferenceAuditTuple = (
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

impl EntityReferenceRepositoryImpl {
    pub async fn execute_entity_reference_insert(
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

    pub async fn execute_entity_reference_idx_insert(
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

    pub async fn execute_entity_reference_audit_insert(
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

    pub async fn execute_entity_reference_update(
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

    pub async fn execute_entity_reference_idx_update(
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