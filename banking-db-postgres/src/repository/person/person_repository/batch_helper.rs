use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use banking_db::models::person::{PersonModel, PersonType};
use banking_db::repository::{
    BatchOperationStats, BatchRepository, BatchResult, LocationRepository, PersonRepository,
    PersonRepositoryError,
};
use std::error::Error;
use std::time::Instant;
use uuid::Uuid;

type PersonTuple = (
    Uuid,
    PersonType,
    String,
    Option<String>,
    Option<Uuid>,
    Option<String>,
    Option<Uuid>,
    Option<Uuid>,
    i32,
);

type PersonAuditTuple = (
    Uuid,
    i32,
    i64,
    PersonType,
    String,
    Option<String>,
    Option<Uuid>,
    Option<String>,
    Option<Uuid>,
    Option<Uuid>,
    i32,
    Uuid,
);

pub async fn execute_person_insert(
    repo: &PersonRepositoryImpl,
    person_values: Vec<PersonTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let query = r#"
        INSERT INTO person (
            id, person_type, display_name, external_identifier,
            organization_person_id, department, location_id, duplicate_of_person_id, entity_reference_count
        )
        SELECT * FROM UNNEST(
            $1::uuid[], $2::person_type[], $3::text[], $4::text[],
            $5::uuid[], $6::text[], $7::uuid[], $8::uuid[], $9::int[]
        )
    "#;
    let (ids, types, names, ext_ids, org_ids, depts, loc_ids, dup_ids, ref_counts) =
        person_values.into_iter().fold(
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
                acc
            },
        );
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query)
                .bind(&ids)
                .bind(&types)
                .bind(&names)
                .bind(&ext_ids)
                .bind(&org_ids)
                .bind(&depts)
                .bind(&loc_ids)
                .bind(&dup_ids)
                .bind(&ref_counts)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query)
                .bind(&ids)
                .bind(&types)
                .bind(&names)
                .bind(&ext_ids)
                .bind(&org_ids)
                .bind(&depts)
                .bind(&loc_ids)
                .bind(&dup_ids)
                .bind(&ref_counts)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_person_idx_insert(
    repo: &PersonRepositoryImpl,
    person_idx_values: Vec<(Uuid, Option<i64>, i32, i64)>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let idx_query = r#"
        INSERT INTO person_idx (person_id, external_identifier_hash, version, hash)
        SELECT * FROM UNNEST($1::uuid[], $2::bigint[], $3::int[], $4::bigint[])
    "#;
    let (idx_ids, ext_hashes, versions, hashes) = person_idx_values.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |mut acc, val| {
            acc.0.push(val.0);
            acc.1.push(val.1);
            acc.2.push(val.2);
            acc.3.push(val.3);
            acc
        },
    );
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(idx_query)
                .bind(&idx_ids)
                .bind(&ext_hashes)
                .bind(&versions)
                .bind(&hashes)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(idx_query)
                .bind(&idx_ids)
                .bind(&ext_hashes)
                .bind(&versions)
                .bind(&hashes)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_person_update(
    repo: &PersonRepositoryImpl,
    person_values: Vec<PersonTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let update_query = r#"
        UPDATE person SET
            person_type = u.person_type,
            display_name = u.display_name,
            external_identifier = u.external_identifier,
            organization_person_id = u.organization_person_id,
            department = u.department,
            location_id = u.location_id,
            duplicate_of_person_id = u.duplicate_of_person_id,
            entity_reference_count = u.entity_reference_count
        FROM (
            SELECT * FROM UNNEST(
                $1::uuid[], $2::person_type[], $3::text[], $4::text[],
                $5::uuid[], $6::text[], $7::uuid[], $8::uuid[], $9::int[]
            )
        ) AS u(
            id, person_type, display_name, external_identifier,
            organization_person_id, department, location_id, duplicate_of_person_id, entity_reference_count
        )
        WHERE person.id = u.id
    "#;
    let (
        ids,
        person_types,
        display_names,
        external_identifiers,
        organization_person_ids,
        departments,
        location_ids,
        duplicate_ids,
        entity_counts,
    ) = person_values.into_iter().fold(
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
            acc
        },
    );
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(update_query)
                .bind(&ids)
                .bind(&person_types)
                .bind(&display_names)
                .bind(&external_identifiers)
                .bind(&organization_person_ids)
                .bind(&departments)
                .bind(&location_ids)
                .bind(&duplicate_ids)
                .bind(&entity_counts)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(update_query)
                .bind(&ids)
                .bind(&person_types)
                .bind(&display_names)
                .bind(&external_identifiers)
                .bind(&organization_person_ids)
                .bind(&departments)
                .bind(&location_ids)
                .bind(&duplicate_ids)
                .bind(&entity_counts)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_person_idx_update(
    repo: &PersonRepositoryImpl,
    person_idx_values: Vec<(Uuid, Option<i64>, i32, i64)>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let update_idx_query = r#"
        UPDATE person_idx SET
            external_identifier_hash = u.external_identifier_hash,
            version = u.version,
            hash = u.hash
        FROM (SELECT * FROM UNNEST($1::uuid[], $2::bigint[], $3::int[], $4::bigint[]))
        AS u(person_id, external_identifier_hash, version, hash)
        WHERE person_idx.person_id = u.person_id
    "#;

    let (idx_ids, ext_hashes, versions, hashes) = person_idx_values.into_iter().fold(
        (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
        |mut acc, val| {
            acc.0.push(val.0);
            acc.1.push(val.1);
            acc.2.push(val.2);
            acc.3.push(val.3);
            acc
        },
    );

    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(update_idx_query)
                .bind(&idx_ids)
                .bind(&ext_hashes)
                .bind(&versions)
                .bind(&hashes)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(update_idx_query)
                .bind(&idx_ids)
                .bind(&ext_hashes)
                .bind(&versions)
                .bind(&hashes)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn execute_person_audit_insert(
    repo: &PersonRepositoryImpl,
    person_audit_values: Vec<PersonAuditTuple>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let audit_query = r#"
        INSERT INTO person_audit (
            person_id, version, hash, person_type, display_name, external_identifier,
            organization_person_id, department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id
        )
        SELECT * FROM UNNEST(
            $1::uuid[], $2::int[], $3::bigint[], $4::person_type[], $5::text[], $6::text[],
            $7::uuid[], $8::text[], $9::uuid[], $10::uuid[], $11::int[], $12::uuid[]
        )
    "#;
    let (
        audit_ids,
        audit_versions,
        audit_hashes,
        audit_types,
        audit_names,
        audit_ext_ids,
        audit_org_ids,
        audit_depts,
        audit_loc_ids,
        audit_dup_ids,
        audit_ref_counts,
        audit_log_ids,
    ) = person_audit_values.into_iter().fold(
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
            acc
        },
    );
    match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(audit_query)
                .bind(&audit_ids)
                .bind(&audit_versions)
                .bind(&audit_hashes)
                .bind(&audit_types)
                .bind(&audit_names)
                .bind(&audit_ext_ids)
                .bind(&audit_org_ids)
                .bind(&audit_depts)
                .bind(&audit_loc_ids)
                .bind(&audit_dup_ids)
                .bind(&audit_ref_counts)
                .bind(&audit_log_ids)
                .execute(&**pool)
                .await?;
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(audit_query)
                .bind(&audit_ids)
                .bind(&audit_versions)
                .bind(&audit_hashes)
                .bind(&audit_types)
                .bind(&audit_names)
                .bind(&audit_ext_ids)
                .bind(&audit_org_ids)
                .bind(&audit_depts)
                .bind(&audit_loc_ids)
                .bind(&audit_dup_ids)
                .bind(&audit_ref_counts)
                .bind(&audit_log_ids)
                .execute(&mut **tx)
                .await?;
        }
    }
    Ok(())
}

pub async fn create_batch_chunked(
    repo: &PersonRepositoryImpl,
    items: Vec<PersonModel>,
    audit_log_id: Uuid,
    chunk_size: usize,
) -> Result<BatchResult<PersonModel>, Box<dyn Error + Send + Sync>> {
    let start = Instant::now();
    let total_items = items.len();
    let mut all_saved = Vec::new();
    let mut errors = Vec::new();
    let mut stats = BatchOperationStats {
        total_items,
        ..Default::default()
    };
    for (i, chunk) in items.chunks(chunk_size).enumerate() {
        match repo.create_batch(chunk.to_vec(), audit_log_id).await {
            Ok(saved) => {
                stats.successful_items += saved.len();
                all_saved.extend(saved);
            }
            Err(e) => {
                stats.failed_items += chunk.len();
                errors.push((i * chunk_size, e));
            }
        }
    }
    stats.duration_ms = start.elapsed().as_millis() as u64;
    Ok(BatchResult::new(all_saved)
        .with_stats(stats)
        .with_errors(errors))
}

pub async fn validate_create_batch(
    repo: &PersonRepositoryImpl,
    items: &[PersonModel],
) -> Result<Vec<bool>, PersonRepositoryError> {
    let mut validations = Vec::with_capacity(items.len());
    for person in items {
        let mut is_valid = true;
        if let Some(org_id) = person.organization_person_id {
            is_valid &= repo.exists_by_id(org_id).await?;
        }
        if let Some(loc_id) = person.location_id {
            is_valid &= repo
                .location_repository
                .exists_by_id(loc_id)
                .await
                .map_err(|e| PersonRepositoryError::RepositoryError(e.into()))?;
        }
        if let Some(dup_id) = person.duplicate_of_person_id {
            is_valid &= repo.exists_by_id(dup_id).await?;
        }
        validations.push(is_valid);
    }
    Ok(validations)
}