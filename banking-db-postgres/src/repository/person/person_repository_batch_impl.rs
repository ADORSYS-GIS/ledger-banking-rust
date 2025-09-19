use crate::repository::person::person_repository_impl::PersonRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{PersonIdxModel, PersonModel, PersonType};
use banking_db::repository::{
    BatchOperationStats, BatchRepository, BatchResult, LocationRepository, PersonRepository,
    PersonRepositoryError,
};
use sqlx::Postgres;
use std::error::Error;
use std::hash::Hasher;
use std::time::Instant;
use twox_hash::XxHash64;
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

#[async_trait]
impl BatchRepository<Postgres, PersonModel> for PersonRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<PersonModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let start = Instant::now();
        let mut saved_items = Vec::with_capacity(items.len());
        let mut stats = BatchOperationStats {
            total_items: items.len(),
            ..Default::default()
        };

        // filter ids into a vec
        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        let existing_persons_check = self.exist_by_ids(&ids).await?;
        let truly_existing_ids: Vec<Uuid> = existing_persons_check
            .into_iter()
            .filter_map(|(id, exists)| if exists { Some(id) } else { None })
            .collect();

        if !truly_existing_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::ManyPersonsExists(
                truly_existing_ids,
            )));
        }

        // location validation
        let mut invalid_location_ids = Vec::new();
        for person in &items {
            if let Some(loc_id) = person.location_id {
                if !self.location_repository.exists_by_id(loc_id).await? {
                    invalid_location_ids.push(loc_id);
                }
            }
        }
        if !invalid_location_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::InvalidLocations(
                invalid_location_ids,
            )));
        }

        // prepare idx cache
        let cache = self.person_idx_cache.read().await;
        for person in &items {
            let mut hasher = XxHash64::with_seed(0);
            let mut person_cbor = Vec::new();
            ciborium::ser::into_writer(person, &mut person_cbor).unwrap();
            hasher.write(&person_cbor);
            let hash = hasher.finish() as i64;

            let external_hash = person.external_identifier.as_ref().map(|s| {
                let mut h = XxHash64::with_seed(0);
                h.write(s.as_bytes());
                h.finish() as i64
            });

            cache.add(PersonIdxModel {
                person_id: person.id,
                external_identifier_hash: external_hash,
                organization_person_id: person.organization_person_id,
                duplicate_of_person_id: person.duplicate_of_person_id,
                version: 0,
                hash,
            });
        }

        // hierarchy validation
        let mut missing_org_ids = Vec::new();
        let mut missing_dup_ids = Vec::new();
        for person in &items {
            if let Some(org_id) = person.organization_person_id {
                if !cache.contains_primary(&org_id) {
                    missing_org_ids.push(org_id);
                }
            }
            if let Some(dup_id) = person.duplicate_of_person_id {
                if !cache.contains_primary(&dup_id) {
                    missing_dup_ids.push(dup_id);
                }
            }
        }
        if !missing_org_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::ManyOrganizationsNotFound(
                missing_org_ids,
            )));
        }
        if !missing_dup_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::ManyPersonsNotFound(
                missing_dup_ids,
            )));
        }

        // prepare batch data
        let mut person_values = Vec::new();
        let mut person_idx_values = Vec::new();
        let mut person_audit_values = Vec::new();

        for person in items {
            let idx_model = cache.get_by_primary(&person.id).unwrap();
            person_values.push((
                person.id,
                person.person_type,
                person.display_name.to_string(),
                person.external_identifier.as_ref().map(|s| s.to_string()),
                person.organization_person_id,
                person.department.as_ref().map(|s| s.to_string()),
                person.location_id,
                person.duplicate_of_person_id,
                person.entity_reference_count,
            ));

            person_idx_values.push((
                person.id,
                idx_model.external_identifier_hash,
                0i32,
                idx_model.hash,
            ));

            person_audit_values.push((
                person.id,
                0i32,
                idx_model.hash,
                person.person_type,
                person.display_name.to_string(),
                person.external_identifier.as_ref().map(|s| s.to_string()),
                person.organization_person_id,
                person.department.as_ref().map(|s| s.to_string()),
                person.location_id,
                person.duplicate_of_person_id,
                person.entity_reference_count,
                audit_log_id,
            ));

            saved_items.push(person);
            stats.successful_items += 1;
        }

        if !person_values.is_empty() {
            self.execute_person_insert(person_values).await?;
            self.execute_person_idx_insert(person_idx_values).await?;
            self.execute_person_audit_insert(person_audit_values)
                .await?;
        }

        stats.duration_ms = start.elapsed().as_millis() as u64;
        Ok(saved_items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<PersonModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = "SELECT * FROM person WHERE id = ANY($1)";
        let rows = match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query).bind(ids).fetch_all(&**pool).await?
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
            }
        };
        let mut person_map = std::collections::HashMap::new();
        for row in rows {
            let person = PersonModel::try_from_row(&row)?;
            person_map.insert(person.id, person);
        }
        Ok(ids.iter().map(|id| person_map.remove(id)).collect())
    }

    async fn update_batch(
        &self,
        items: Vec<PersonModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }
        let start = Instant::now();
        let mut updated_items = Vec::new();
        let mut stats = BatchOperationStats {
            total_items: items.len(),
            ..Default::default()
        };
        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        let existing_persons_check = self.exist_by_ids(&ids).await?;
        let missing_ids: Vec<Uuid> = existing_persons_check
            .into_iter()
            .filter_map(|(id, exists)| if !exists { Some(id) } else { None })
            .collect();
        if !missing_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::ManyPersonsNotFound(
                missing_ids,
            )));
        }
        let cache = self.person_idx_cache.write().await;
        let mut person_values = Vec::new();
        let mut person_idx_values = Vec::new();
        let mut person_audit_values = Vec::new();
        for person in items {
            let mut hasher = XxHash64::with_seed(0);
            let mut person_cbor = Vec::new();
            ciborium::ser::into_writer(&person, &mut person_cbor).unwrap();
            hasher.write(&person_cbor);
            let new_hash = hasher.finish() as i64;
            if let Some(existing_idx) = cache.get_by_primary(&person.id) {
                if existing_idx.hash == new_hash {
                    stats.skipped_items += 1;
                    continue;
                }
                let new_version = existing_idx.version + 1;
                let external_hash = person.external_identifier.as_ref().map(|s| {
                    let mut h = XxHash64::with_seed(0);
                    h.write(s.as_bytes());
                    h.finish() as i64
                });
                person_values.push((
                    person.id,
                    person.person_type,
                    person.display_name.to_string(),
                    person.external_identifier.as_ref().map(|s| s.to_string()),
                    person.organization_person_id,
                    person.department.as_ref().map(|s| s.to_string()),
                    person.location_id,
                    person.duplicate_of_person_id,
                    person.entity_reference_count,
                ));
                person_idx_values.push((person.id, external_hash, new_version, new_hash));
                person_audit_values.push((
                    person.id,
                    new_version,
                    new_hash,
                    person.person_type,
                    person.display_name.to_string(),
                    person.external_identifier.as_ref().map(|s| s.to_string()),
                    person.organization_person_id,
                    person.department.as_ref().map(|s| s.to_string()),
                    person.location_id,
                    person.duplicate_of_person_id,
                    person.entity_reference_count,
                    audit_log_id,
                ));
                let mut updated_idx = existing_idx.clone();
                updated_idx.version = new_version;
                updated_idx.hash = new_hash;
                updated_idx.external_identifier_hash = external_hash;
                updated_idx.organization_person_id = person.organization_person_id;
                updated_idx.duplicate_of_person_id = person.duplicate_of_person_id;
                cache.update(updated_idx);
                updated_items.push(person);
                stats.successful_items += 1;
            } else {
                stats.failed_items += 1;
            }
        }
        // location validation
        let mut invalid_location_ids = Vec::new();
        for tuple in &person_values {
            if let Some(loc_id) = tuple.6 {
                if !self.location_repository.exists_by_id(loc_id).await? {
                    invalid_location_ids.push(loc_id);
                }
            }
        }
        if !invalid_location_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::InvalidLocations(
                invalid_location_ids,
            )));
        }
        // hierarchy validation
        let mut missing_org_ids = Vec::new();
        let mut missing_dup_ids = Vec::new();
        for tuple in &person_values {
            if let Some(org_id) = tuple.4 {
                if !cache.contains_primary(&org_id) {
                    missing_org_ids.push(org_id);
                }
            }
            if let Some(dup_id) = tuple.7 {
                if !cache.contains_primary(&dup_id) {
                    missing_dup_ids.push(dup_id);
                }
            }
        }
        if !missing_org_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::ManyOrganizationsNotFound(
                missing_org_ids,
            )));
        }
        if !missing_dup_ids.is_empty() {
            return Err(Box::new(PersonRepositoryError::ManyPersonsNotFound(
                missing_dup_ids,
            )));
        }
        if !person_values.is_empty() {
            self.execute_person_update(person_values).await?;
            self.execute_person_idx_update(person_idx_values).await?;
            self.execute_person_audit_insert(person_audit_values)
                .await?;
        }
        stats.duration_ms = start.elapsed().as_millis() as u64;
        Ok(updated_items)
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut person_audit_values = Vec::new();
        let existings = self.find_by_ids(ids).await?;
        let existing_ids: Vec<Uuid> = existings.iter().map(|p| p.person_id).collect();
        {
            let cache = self.person_idx_cache.write().await;
            for id in &existing_ids {
                cache.remove(id);
            }
        }
        let mut dependent_duplicates = Vec::new();
        for id in &existing_ids {
            if !self.find_by_duplicate_of_person_id(*id).await?.is_empty() {
                dependent_duplicates.push(*id);
            }
        }
        if !dependent_duplicates.is_empty() {
            return Err(Box::new(PersonRepositoryError::IsDuplicatePersonFor(
                dependent_duplicates,
            )));
        }
        let mut dependent_organizations = Vec::new();
        for id in &existing_ids {
            if !self.find_by_organization_person_id(*id).await?.is_empty() {
                dependent_organizations.push(*id);
            }
        }
        if !dependent_organizations.is_empty() {
            return Err(Box::new(PersonRepositoryError::IsOrganizationPersonFor(
                dependent_organizations,
            )));
        }
        let items = self.load_batch(&existing_ids).await?;
        for person in items.into_iter().flatten() {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(&[]);
            let new_hash = hasher.finish() as i64;
            let existing_idx = existings.iter().find(|p| p.person_id == person.id).unwrap();
            let new_version = existing_idx.version + 1;
            person_audit_values.push((
                person.id,
                new_version,
                new_hash,
                person.person_type,
                person.display_name.to_string(),
                person.external_identifier.as_ref().map(|s| s.to_string()),
                person.organization_person_id,
                person.department.as_ref().map(|s| s.to_string()),
                person.location_id,
                person.duplicate_of_person_id,
                person.entity_reference_count,
                Uuid::new_v4(),
            ));
        }
        let delete_query = "DELETE FROM person WHERE id = ANY($1)";
        let delete_idx_query = "DELETE FROM person_idx WHERE person_id = ANY($1)";
        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(delete_idx_query).bind(&existing_ids).execute(&**pool).await?;
                sqlx::query(delete_query).bind(&existing_ids).execute(&**pool).await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(delete_idx_query).bind(&existing_ids).execute(&mut **tx).await?;
                sqlx::query(delete_query).bind(&existing_ids).execute(&mut **tx).await?;
            }
        }
        self.execute_person_audit_insert(person_audit_values)
            .await?;
        Ok(existing_ids.len())
    }
}

impl PersonRepositoryImpl {
    async fn execute_person_insert(
        &self,
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
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
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
        match &self.executor {
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

    async fn execute_person_idx_insert(
        &self,
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
        match &self.executor {
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

    async fn execute_person_update(
        &self,
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
        let (ids, person_types, display_names, external_identifiers, organization_person_ids, departments, location_ids, duplicate_ids, entity_counts) =
            person_values.into_iter().fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
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
        match &self.executor {
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

    async fn execute_person_idx_update(
        &self,
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

        match &self.executor {
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

    async fn execute_person_audit_insert(
        &self,
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
        let (audit_ids, audit_versions, audit_hashes, audit_types, audit_names, audit_ext_ids, audit_org_ids, audit_depts, audit_loc_ids, audit_dup_ids, audit_ref_counts, audit_log_ids) =
            person_audit_values.into_iter().fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
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
        match &self.executor {
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
        &self,
        items: Vec<PersonModel>,
        audit_log_id: Uuid,
        chunk_size: usize,
    ) -> Result<BatchResult<PersonModel>, Box<dyn Error + Send + Sync>> {
        let start = Instant::now();
        let total_items = items.len();
        let mut all_saved = Vec::new();
        let mut errors = Vec::new();
        let mut stats = BatchOperationStats { total_items, ..Default::default() };
        for (i, chunk) in items.chunks(chunk_size).enumerate() {
            match self.create_batch(chunk.to_vec(), audit_log_id).await {
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
        Ok(BatchResult::new(all_saved).with_stats(stats).with_errors(errors))
    }

    pub async fn validate_create_batch(
        &self,
        items: &[PersonModel],
    ) -> Result<Vec<bool>, PersonRepositoryError> {
        let mut validations = Vec::with_capacity(items.len());
        for person in items {
            let mut is_valid = true;
            if let Some(org_id) = person.organization_person_id {
                is_valid &= self.exists_by_id(org_id).await?;
            }
            if let Some(loc_id) = person.location_id {
                is_valid &= self
                    .location_repository
                    .exists_by_id(loc_id)
                    .await
                    .map_err(|e| PersonRepositoryError::RepositoryError(e.into()))?;
            }
            if let Some(dup_id) = person.duplicate_of_person_id {
                is_valid &= self.exists_by_id(dup_id).await?;
            }
            validations.push(is_valid);
        }
        Ok(validations)
    }
}
