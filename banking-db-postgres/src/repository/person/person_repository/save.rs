use banking_db::models::person::{PersonAuditModel, PersonModel};
use banking_db::repository::{LocationRepository, PersonRepository, PersonRepositoryError, PersonResult};
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;

pub async fn save(
    repo: &PersonRepositoryImpl,
    person: PersonModel,
    audit_log_id: Uuid,
) -> PersonResult<PersonModel> {
    if let Some(org_id) = person.organization_person_id {
        if !repo.exists_by_id(org_id).await? {
            return Err(PersonRepositoryError::OrganizationNotFound(org_id));
        }
    }
    if let Some(loc_id) = person.location_id {
        if !repo
            .location_repository
            .exists_by_id(loc_id)
            .await
            .map_err(|e| PersonRepositoryError::RepositoryError(e.into()))?
        {
            return Err(PersonRepositoryError::LocationNotFound(loc_id));
        }
    }
    if let Some(dup_id) = person.duplicate_of_person_id {
        if !repo.exists_by_id(dup_id).await? {
            return Err(PersonRepositoryError::DuplicatePersonNotFound(dup_id));
        }
    }

    let mut hasher = XxHash64::with_seed(0);
    let mut person_cbor = Vec::new();
    ciborium::ser::into_writer(&person, &mut person_cbor).unwrap();
    hasher.write(&person_cbor);
    let new_hash = hasher.finish() as i64;

    let maybe_existing_idx = {
        let cache_read_guard = repo.person_idx_cache.read().await;
        cache_read_guard.get_by_primary(&person.id)
    };

    let new_external_hash = person.external_identifier.as_ref().map(|s| {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(s.as_bytes());
        hasher.finish() as i64
    });

    let (version, is_update) = if let Some(existing_idx) = maybe_existing_idx {
        if existing_idx.hash == new_hash {
            return Ok(person);
        }
        (existing_idx.version + 1, true)
    } else {
        (0, false)
    };

    let audit_model = PersonAuditModel {
        person_id: person.id,
        version,
        hash: new_hash,
        person_type: person.person_type,
        display_name: person.display_name.clone(),
        external_identifier: person.external_identifier.clone(),
        entity_reference_count: person.entity_reference_count,
        organization_person_id: person.organization_person_id,
        messaging_info1: person.messaging_info1.clone(),
        messaging_info2: person.messaging_info2.clone(),
        messaging_info3: person.messaging_info3.clone(),
        messaging_info4: person.messaging_info4.clone(),
        messaging_info5: person.messaging_info5.clone(),
        department: person.department.clone(),
        location_id: person.location_id,
        duplicate_of_person_id: person.duplicate_of_person_id,
        audit_log_id,
    };

    let query1 = sqlx::query(
        r#"
            INSERT INTO person_audit (
                person_id, version, hash, person_type, display_name, external_identifier,
                organization_person_id, messaging_info1, messaging_info2, messaging_info3,
                messaging_info4, messaging_info5, department, location_id, duplicate_of_person_id,
                entity_reference_count, audit_log_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        "#,
    )
    .bind(audit_model.person_id)
    .bind(audit_model.version)
    .bind(audit_model.hash)
    .bind(audit_model.person_type)
    .bind(audit_model.display_name.as_str())
    .bind(audit_model.external_identifier.as_ref().map(|s| s.as_str()))
    .bind(audit_model.organization_person_id)
    .bind(audit_model.messaging_info1.as_ref().map(|s| s.as_str()))
    .bind(audit_model.messaging_info2.as_ref().map(|s| s.as_str()))
    .bind(audit_model.messaging_info3.as_ref().map(|s| s.as_str()))
    .bind(audit_model.messaging_info4.as_ref().map(|s| s.as_str()))
    .bind(audit_model.messaging_info5.as_ref().map(|s| s.as_str()))
    .bind(audit_model.department.as_ref().map(|s| s.as_str()))
    .bind(audit_model.location_id)
    .bind(audit_model.duplicate_of_person_id)
    .bind(audit_model.entity_reference_count)
    .bind(audit_model.audit_log_id);

    let (query2_sql, query3_sql) = if is_update {
        (
            r#"
            UPDATE person SET
                person_type = $2::person_type, display_name = $3, external_identifier = $4,
                organization_person_id = $5, messaging_info1 = $6, messaging_info2 = $7,
                messaging_info3 = $8, messaging_info4 = $9, messaging_info5 = $10,
                department = $11, location_id = $12, duplicate_of_person_id = $13,
                entity_reference_count = $14
            WHERE id = $1
            "#,
            r#"
            UPDATE person_idx SET
                external_identifier_hash = $2,
                organization_person_id = $3,
                duplicate_of_person_id = $4,
                version = $5,
                hash = $6
            WHERE person_id = $1
            "#,
        )
    } else {
        (
            r#"
            INSERT INTO person (
                id, person_type, display_name, external_identifier, organization_person_id,
                messaging_info1, messaging_info2, messaging_info3, messaging_info4, messaging_info5,
                department, location_id, duplicate_of_person_id, entity_reference_count
            )
            VALUES ($1, $2::person_type, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
            r#"
            INSERT INTO person_idx (
                person_id, external_identifier_hash, organization_person_id,
                duplicate_of_person_id, version, hash
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
    };

    let query2 = sqlx::query(query2_sql)
        .bind(person.id)
        .bind(person.person_type)
        .bind(person.display_name.as_str())
        .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
        .bind(person.organization_person_id)
        .bind(person.messaging_info1.as_ref().map(|s| s.as_str()))
        .bind(person.messaging_info2.as_ref().map(|s| s.as_str()))
        .bind(person.messaging_info3.as_ref().map(|s| s.as_str()))
        .bind(person.messaging_info4.as_ref().map(|s| s.as_str()))
        .bind(person.messaging_info5.as_ref().map(|s| s.as_str()))
        .bind(person.department.as_ref().map(|s| s.as_str()))
        .bind(person.location_id)
        .bind(person.duplicate_of_person_id)
        .bind(person.entity_reference_count);

    let query3 = sqlx::query(query3_sql)
        .bind(person.id)
        .bind(new_external_hash)
        .bind(person.organization_person_id)
        .bind(person.duplicate_of_person_id)
        .bind(version)
        .bind(new_hash);

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            query1.execute(&**pool).await?;
            query2.execute(&**pool).await?;
            query3.execute(&**pool).await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query1.execute(&mut **tx).await?;
            query2.execute(&mut **tx).await?;
            query3.execute(&mut **tx).await?;
        }
    }

    let new_idx = banking_db::models::person::PersonIdxModel {
        person_id: person.id,
        external_identifier_hash: new_external_hash,
        organization_person_id: person.organization_person_id,
        duplicate_of_person_id: person.duplicate_of_person_id,
        version,
        hash: new_hash,
    };

    if is_update {
        repo.person_idx_cache.read().await.update(new_idx);
    } else {
        repo.person_idx_cache.read().await.add(new_idx);
    }

    Ok(person)
}
#[cfg(test)]
mod tests {
    use banking_db::repository::{PersonRepository, PersonRepos};
    use uuid::Uuid;
    use crate::repository::person::test_helpers::create_test_person_model;
    use crate::test_helper::setup_test_context;

    #[tokio::test]
    async fn test_save_person() {
        let ctx = setup_test_context().await.unwrap();
        let repo = ctx.person_repos().persons();

        let audit_log_id = Uuid::new_v4();
        let new_person = create_test_person_model("John Doe");
        let saved_person = repo.save(new_person.clone(), audit_log_id).await.unwrap();
        
        assert_eq!(new_person.id, saved_person.id);

        // Verify it was saved by trying to find it
        let found_person_idx = repo.find_by_id(new_person.id).await.unwrap();
        assert!(found_person_idx.is_some());
        assert_eq!(new_person.id, found_person_idx.unwrap().person_id);
    }
}