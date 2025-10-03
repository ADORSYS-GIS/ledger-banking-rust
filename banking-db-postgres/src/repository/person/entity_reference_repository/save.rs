use banking_db::models::person::{
    EntityReferenceAuditModel, EntityReferenceIdxModel, EntityReferenceModel,
};
use banking_db::repository::person::entity_reference_repository::{
    EntityReferenceRepositoryError, EntityReferenceResult,
};
use banking_db::repository::PersonRepository;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;

pub async fn save(
    repo: &EntityReferenceRepositoryImpl,
    entity_ref: EntityReferenceModel,
    audit_log_id: Uuid,
) -> EntityReferenceResult<EntityReferenceModel> {
    if !repo
        .person_repository
        .exists_by_id(entity_ref.person_id)
        .await
        .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?
    {
        return Err(EntityReferenceRepositoryError::PersonNotFound(
            entity_ref.person_id,
        ));
    }

    let mut hasher = XxHash64::with_seed(0);
    let mut entity_ref_cbor = Vec::new();
    ciborium::ser::into_writer(&entity_ref, &mut entity_ref_cbor).unwrap();
    hasher.write(&entity_ref_cbor);
    let new_hash = hasher.finish() as i64;

    let mut ref_hasher = XxHash64::with_seed(0);
    ref_hasher.write(entity_ref.reference_external_id.as_bytes());
    let reference_external_id_hash = ref_hasher.finish() as i64;

    let maybe_existing_idx = {
        let cache_read_guard = repo.entity_reference_idx_cache.read().await;
        cache_read_guard.get_by_primary(&entity_ref.id)
    };

    if let Some(existing_idx) = maybe_existing_idx {
        // UPDATE
        if existing_idx.hash == new_hash {
            return Ok(entity_ref); // No changes
        }

        let new_version = existing_idx.version + 1;

        let audit_model = EntityReferenceAuditModel {
            entity_reference_id: entity_ref.id,
            version: new_version,
            hash: new_hash,
            person_id: entity_ref.person_id,
            entity_role: entity_ref.entity_role,
            reference_external_id: entity_ref.reference_external_id.clone(),
            reference_details_l1: entity_ref.reference_details_l1.clone(),
            reference_details_l2: entity_ref.reference_details_l2.clone(),
            reference_details_l3: entity_ref.reference_details_l3.clone(),
            audit_log_id,
        };

        let query1 = sqlx::query(
            r#"
                INSERT INTO entity_reference_audit (entity_reference_id, version, hash, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                "#,
        )
        .bind(audit_model.entity_reference_id)
        .bind(audit_model.version)
        .bind(audit_model.hash)
        .bind(audit_model.person_id)
        .bind(audit_model.entity_role)
        .bind(audit_model.reference_external_id.as_str())
        .bind(
            audit_model
                .reference_details_l1
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            audit_model
                .reference_details_l2
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            audit_model
                .reference_details_l3
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(audit_model.audit_log_id);

        let query2 = sqlx::query(
            r#"
                UPDATE entity_reference SET
                    person_id = $2, entity_role = $3::person_entity_type, reference_external_id = $4,
                    reference_details_l1 = $5, reference_details_l2 = $6, reference_details_l3 = $7
                WHERE id = $1
                "#,
        )
        .bind(entity_ref.id)
        .bind(entity_ref.person_id)
        .bind(entity_ref.entity_role)
        .bind(entity_ref.reference_external_id.as_str())
        .bind(
            entity_ref
                .reference_details_l1
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            entity_ref
                .reference_details_l2
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            entity_ref
                .reference_details_l3
                .as_ref()
                .map(|s| s.as_str()),
        );

        let query3 = sqlx::query(
            r#"
                UPDATE entity_reference_idx SET
                    version = $2,
                    hash = $3
                WHERE entity_reference_id = $1
                "#,
        )
        .bind(entity_ref.id)
        .bind(new_version)
        .bind(new_hash);

        match &repo.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                query1
                    .execute(&**pool)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&**pool)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query3
                    .execute(&**pool)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query3
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
            }
        }

        let new_idx = EntityReferenceIdxModel {
            entity_reference_id: entity_ref.id,
            person_id: entity_ref.person_id,
            reference_external_id_hash,
            version: new_version,
            hash: new_hash,
        };
        repo.entity_reference_idx_cache
            .read()
            .await
            .update(new_idx);
    } else {
        // INSERT
        let version = 0;
        let audit_model = EntityReferenceAuditModel {
            entity_reference_id: entity_ref.id,
            version,
            hash: new_hash,
            person_id: entity_ref.person_id,
            entity_role: entity_ref.entity_role,
            reference_external_id: entity_ref.reference_external_id.clone(),
            reference_details_l1: entity_ref.reference_details_l1.clone(),
            reference_details_l2: entity_ref.reference_details_l2.clone(),
            reference_details_l3: entity_ref.reference_details_l3.clone(),
            audit_log_id,
        };

        let query1 = sqlx::query(
            r#"
                INSERT INTO entity_reference_audit (entity_reference_id, version, hash, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                "#,
        )
        .bind(audit_model.entity_reference_id)
        .bind(audit_model.version)
        .bind(audit_model.hash)
        .bind(audit_model.person_id)
        .bind(audit_model.entity_role)
        .bind(audit_model.reference_external_id.as_str())
        .bind(
            audit_model
                .reference_details_l1
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            audit_model
                .reference_details_l2
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            audit_model
                .reference_details_l3
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(audit_model.audit_log_id);

        let query2 = sqlx::query(
            r#"
                INSERT INTO entity_reference (id, person_id, entity_role, reference_external_id, reference_details_l1, reference_details_l2, reference_details_l3)
                VALUES ($1, $2, $3::person_entity_type, $4, $5, $6, $7)
                "#,
        )
        .bind(entity_ref.id)
        .bind(entity_ref.person_id)
        .bind(entity_ref.entity_role)
        .bind(entity_ref.reference_external_id.as_str())
        .bind(
            entity_ref
                .reference_details_l1
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            entity_ref
                .reference_details_l2
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            entity_ref
                .reference_details_l3
                .as_ref()
                .map(|s| s.as_str()),
        );

        let query3 = sqlx::query(
            r#"
                INSERT INTO entity_reference_idx (entity_reference_id, person_id, version, hash)
                VALUES ($1, $2, $3, $4)
                "#,
        )
        .bind(entity_ref.id)
        .bind(entity_ref.person_id)
        .bind(version)
        .bind(new_hash);

        match &repo.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                query1
                    .execute(&**pool)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&**pool)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query3
                    .execute(&**pool)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
                query3
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?;
            }
        }

        let new_idx = EntityReferenceIdxModel {
            entity_reference_id: entity_ref.id,
            person_id: entity_ref.person_id,
            reference_external_id_hash,
            version,
            hash: new_hash,
        };
        repo.entity_reference_idx_cache.read().await.add(new_idx);
    }

    Ok(entity_ref)
}

#[cfg(test)]
mod tests {
    use banking_db::models::person::RelationshipRole;
    use banking_db::repository::{EntityReferenceRepository, PersonRepository, PersonRepos};
    use crate::repository::person::test_helpers::{
        create_test_entity_reference_model, create_test_person_model,
    };
    use crate::test_helper::setup_test_context;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_save_entity_reference() {
        let ctx = setup_test_context().await.unwrap();
        let person_repo = ctx.person_repos().persons();
        let repo = ctx.person_repos().entity_references();

        let new_person = create_test_person_model("John Doe");
        let audit_log_id = Uuid::new_v4();
        person_repo
            .save(new_person.clone(), audit_log_id)
            .await
            .unwrap();

        // Test save and find_by_id
        let new_entity_ref = create_test_entity_reference_model(
            new_person.id,
            RelationshipRole::Customer,
            "CUST-12345",
        );
        let saved_entity_ref = repo
            .save(new_entity_ref.clone(), audit_log_id)
            .await
            .unwrap();
        assert_eq!(new_entity_ref.id, saved_entity_ref.id);

        let found_entity_ref = repo
            .find_by_id(new_entity_ref.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(new_entity_ref.id, found_entity_ref.entity_reference_id);
    }
}