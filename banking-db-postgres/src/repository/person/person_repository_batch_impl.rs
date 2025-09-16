use crate::repository::person::person_repository_impl::PersonRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{PersonIdxModel, PersonModel};
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

/// Batch operations implementation for PersonRepository
#[async_trait]
impl BatchRepository<Postgres, PersonModel> for PersonRepositoryImpl {
    /// Save multiple persons in a single transaction
    /// This method performs bulk inserts with audit logging
    async fn save_batch(
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
        // call self.exists_by_ids: if result is not empty, raise an error. 
        // As this is a batch create. No update shall be in the collection.
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

        // Get cache for validations and updates
        let cache = self.person_idx_cache.read().await;

       // Fillup the cache first
       for person in &items {
           let mut hasher = XxHash64::with_seed(0);
           let mut person_cbor = Vec::new();
           ciborium::ser::into_writer(person, &mut person_cbor).unwrap();
           hasher.write(&person_cbor);
           let hash = hasher.finish() as i64;

           let external_hash = person.external_identifier.as_ref().map(|s| {
               let mut hasher = XxHash64::with_seed(0);
               hasher.write(s.as_bytes());
               hasher.finish() as i64
           });

           let idx_model = PersonIdxModel {
               person_id: person.id,
               external_identifier_hash: external_hash,
               organization_person_id: person.organization_person_id,
               duplicate_of_person_id: person.duplicate_of_person_id,
               version: 0,
               hash,
           };
           cache.add(idx_model);
       }

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

       // Prepare batch data
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
               person.messaging1_id,
               person.messaging1_type,
               person.messaging2_id,
               person.messaging2_type,
               person.messaging3_id,
               person.messaging3_type,
               person.messaging4_id,
               person.messaging4_type,
               person.messaging5_id,
               person.messaging5_type,
               person.department.as_ref().map(|s| s.to_string()),
               person.location_id,
               person.duplicate_of_person_id,
               person.entity_reference_count,
           ));

           person_idx_values.push((
               person.id,
               idx_model.external_identifier_hash,
               0i32, // version
               idx_model.hash,
           ));

           person_audit_values.push((
               person.id,
               0i32, // version
               idx_model.hash,
               person.person_type,
               person.display_name.to_string(),
               person.external_identifier.as_ref().map(|s| s.to_string()),
               person.organization_person_id,
               person.messaging1_id,
               person.messaging1_type,
               person.messaging2_id,
               person.messaging2_type,
               person.messaging3_id,
               person.messaging3_type,
               person.messaging4_id,
               person.messaging4_type,
               person.messaging5_id,
               person.messaging5_type,
               person.department.as_ref().map(|s| s.to_string()),
               person.location_id,
               person.duplicate_of_person_id,
               person.entity_reference_count,
               audit_log_id,
           ));
           saved_items.push(person);
           stats.successful_items += 1;
       }

        // Perform bulk inserts using UNNEST for maximum efficiency
        if !person_values.is_empty() {
            // Insert into person table
            let query = r#"
                INSERT INTO person (
                    id, person_type, display_name, external_identifier,
                    organization_person_id, messaging1_id, messaging1_type,
                    messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                    messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                    department, location_id, duplicate_of_person_id, entity_reference_count
                )
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::person_type[], $3::text[], $4::text[],
                    $5::uuid[], $6::uuid[], $7::messaging_type[],
                    $8::uuid[], $9::messaging_type[], $10::uuid[], $11::messaging_type[],
                    $12::uuid[], $13::messaging_type[], $14::uuid[], $15::messaging_type[],
                    $16::text[], $17::uuid[], $18::uuid[], $19::int[]
                )
            "#;

            // Extract values manually instead of using unzip with large tuples
            let mut ids = Vec::new();
            let mut types = Vec::new();
            let mut names = Vec::new();
            let mut ext_ids = Vec::new();
            let mut org_ids = Vec::new();
            let mut msg1_ids = Vec::new();
            let mut msg1_types = Vec::new();
            let mut msg2_ids = Vec::new();
            let mut msg2_types = Vec::new();
            let mut msg3_ids = Vec::new();
            let mut msg3_types = Vec::new();
            let mut msg4_ids = Vec::new();
            let mut msg4_types = Vec::new();
            let mut msg5_ids = Vec::new();
            let mut msg5_types = Vec::new();
            let mut depts = Vec::new();
            let mut loc_ids = Vec::new();
            let mut dup_ids = Vec::new();
            let mut ref_counts = Vec::new();

            for v in person_values.into_iter() {
                ids.push(v.0);
                types.push(v.1);
                names.push(v.2);
                ext_ids.push(v.3);
                org_ids.push(v.4);
                msg1_ids.push(v.5);
                msg1_types.push(v.6);
                msg2_ids.push(v.7);
                msg2_types.push(v.8);
                msg3_ids.push(v.9);
                msg3_types.push(v.10);
                msg4_ids.push(v.11);
                msg4_types.push(v.12);
                msg5_ids.push(v.13);
                msg5_types.push(v.14);
                depts.push(v.15);
                loc_ids.push(v.16);
                dup_ids.push(v.17);
                ref_counts.push(v.18);
            }

            // Use the executor (which handles both Pool and Transaction)
            match &self.executor {
                crate::repository::executor::Executor::Pool(pool) => {
                    sqlx::query(query)
                        .bind(&ids)
                        .bind(&types)
                        .bind(&names)
                        .bind(&ext_ids)
                        .bind(&org_ids)
                        .bind(&msg1_ids)
                        .bind(&msg1_types)
                        .bind(&msg2_ids)
                        .bind(&msg2_types)
                        .bind(&msg3_ids)
                        .bind(&msg3_types)
                        .bind(&msg4_ids)
                        .bind(&msg4_types)
                        .bind(&msg5_ids)
                        .bind(&msg5_types)
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
                        .bind(&msg1_ids)
                        .bind(&msg1_types)
                        .bind(&msg2_ids)
                        .bind(&msg2_types)
                        .bind(&msg3_ids)
                        .bind(&msg3_types)
                        .bind(&msg4_ids)
                        .bind(&msg4_types)
                        .bind(&msg5_ids)
                        .bind(&msg5_types)
                        .bind(&depts)
                        .bind(&loc_ids)
                        .bind(&dup_ids)
                        .bind(&ref_counts)
                        .execute(&mut **tx)
                        .await?;
                }
            }

            // Insert into person_idx table
            let idx_query = r#"
                INSERT INTO person_idx (person_id, external_identifier_hash, version, hash)
                SELECT * FROM UNNEST($1::uuid[], $2::bigint[], $3::int[], $4::bigint[])
            "#;

            // Extract values for person_idx
            let mut idx_ids = Vec::new();
            let mut ext_hashes = Vec::new();
            let mut versions = Vec::new();
            let mut hashes = Vec::new();

            for v in person_idx_values.iter() {
                idx_ids.push(v.0);
                ext_hashes.push(v.1);
                versions.push(v.2);
                hashes.push(v.3);
            }

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

            // Insert into person_audit table
            let audit_query = r#"
                INSERT INTO person_audit (
                    person_id, version, hash, person_type, display_name, external_identifier,
                    organization_person_id, messaging1_id, messaging1_type,
                    messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                    messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                    department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id
                )
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::int[], $3::bigint[], $4::person_type[], $5::text[], $6::text[],
                    $7::uuid[], $8::uuid[], $9::messaging_type[],
                    $10::uuid[], $11::messaging_type[], $12::uuid[], $13::messaging_type[],
                    $14::uuid[], $15::messaging_type[], $16::uuid[], $17::messaging_type[],
                    $18::text[], $19::uuid[], $20::uuid[], $21::int[], $22::uuid[]
                )
            "#;

            // Extract values for person_audit
            let mut audit_ids = Vec::new();
            let mut audit_versions = Vec::new();
            let mut audit_hashes = Vec::new();
            let mut audit_types = Vec::new();
            let mut audit_names = Vec::new();
            let mut audit_ext_ids = Vec::new();
            let mut audit_org_ids = Vec::new();
            let mut audit_msg1_ids = Vec::new();
            let mut audit_msg1_types = Vec::new();
            let mut audit_msg2_ids = Vec::new();
            let mut audit_msg2_types = Vec::new();
            let mut audit_msg3_ids = Vec::new();
            let mut audit_msg3_types = Vec::new();
            let mut audit_msg4_ids = Vec::new();
            let mut audit_msg4_types = Vec::new();
            let mut audit_msg5_ids = Vec::new();
            let mut audit_msg5_types = Vec::new();
            let mut audit_depts = Vec::new();
            let mut audit_loc_ids = Vec::new();
            let mut audit_dup_ids = Vec::new();
            let mut audit_ref_counts = Vec::new();
            let mut audit_log_ids = Vec::new();

            for v in person_audit_values.into_iter() {
                audit_ids.push(v.0);
                audit_versions.push(v.1);
                audit_hashes.push(v.2);
                audit_types.push(v.3);
                audit_names.push(v.4);
                audit_ext_ids.push(v.5);
                audit_org_ids.push(v.6);
                audit_msg1_ids.push(v.7);
                audit_msg1_types.push(v.8);
                audit_msg2_ids.push(v.9);
                audit_msg2_types.push(v.10);
                audit_msg3_ids.push(v.11);
                audit_msg3_types.push(v.12);
                audit_msg4_ids.push(v.13);
                audit_msg4_types.push(v.14);
                audit_msg5_ids.push(v.15);
                audit_msg5_types.push(v.16);
                audit_depts.push(v.17);
                audit_loc_ids.push(v.18);
                audit_dup_ids.push(v.19);
                audit_ref_counts.push(v.20);
                audit_log_ids.push(v.21);
            }

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
                        .bind(&audit_msg1_ids)
                        .bind(&audit_msg1_types)
                        .bind(&audit_msg2_ids)
                        .bind(&audit_msg2_types)
                        .bind(&audit_msg3_ids)
                        .bind(&audit_msg3_types)
                        .bind(&audit_msg4_ids)
                        .bind(&audit_msg4_types)
                        .bind(&audit_msg5_ids)
                        .bind(&audit_msg5_types)
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
                        .bind(&audit_msg1_ids)
                        .bind(&audit_msg1_types)
                        .bind(&audit_msg2_ids)
                        .bind(&audit_msg2_types)
                        .bind(&audit_msg3_ids)
                        .bind(&audit_msg3_types)
                        .bind(&audit_msg4_ids)
                        .bind(&audit_msg4_types)
                        .bind(&audit_msg5_ids)
                        .bind(&audit_msg5_types)
                        .bind(&audit_depts)
                        .bind(&audit_loc_ids)
                        .bind(&audit_dup_ids)
                        .bind(&audit_ref_counts)
                        .bind(&audit_log_ids)
                        .execute(&mut **tx)
                        .await?;
                }
            }
        }

        stats.duration_ms = start.elapsed().as_millis() as u64;
        Ok(saved_items)
    }

    /// Load multiple persons by their IDs
    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<PersonModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let query = r#"
            SELECT * FROM person WHERE id = ANY($1)
        "#;

        let rows = match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query).bind(ids).fetch_all(&**pool).await?
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
            }
        };

        // Create a map for fast lookup
        let mut person_map = std::collections::HashMap::new();
        for row in rows {
            let person = PersonModel::try_from_row(&row)?;
            person_map.insert(person.id, person);
        }

        // Return in same order as requested IDs
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(person_map.remove(id));
        }

        Ok(result)
    }

    /// Update multiple persons in a single transaction
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

        // filter ids into a vec
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

        let mut person_values = Vec::new();
        let mut person_idx_values = Vec::new();
        let mut person_audit_values = Vec::new();

        let cache = self.person_idx_cache.write().await;

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
                    let mut hasher = XxHash64::with_seed(0);
                    hasher.write(s.as_bytes());
                    hasher.finish() as i64
                });

                person_values.push((
                    person.id,
                    person.person_type,
                    person.display_name.to_string(),
                    person.external_identifier.as_ref().map(|s| s.to_string()),
                    person.organization_person_id,
                    person.messaging1_id,
                    person.messaging1_type,
                    person.messaging2_id,
                    person.messaging2_type,
                    person.messaging3_id,
                    person.messaging3_type,
                    person.messaging4_id,
                    person.messaging4_type,
                    person.messaging5_id,
                    person.messaging5_type,
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
                    person.messaging1_id,
                    person.messaging1_type,
                    person.messaging2_id,
                    person.messaging2_type,
                    person.messaging3_id,
                    person.messaging3_type,
                    person.messaging4_id,
                    person.messaging4_type,
                    person.messaging5_id,
                    person.messaging5_type,
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

        // review check of location for each person to be updated
        let mut invalid_location_ids = Vec::new();
        for person_tuple in &person_values {
            if let Some(loc_id) = person_tuple.16 {
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

        // review check of org and dup person
       let mut missing_org_ids = Vec::new();
       let mut missing_dup_ids = Vec::new();
       for person_tuple in &person_values {
           if let Some(org_id) = person_tuple.4 {
               if !cache.contains_primary(&org_id) {
                   missing_org_ids.push(org_id);
               }
           }
           if let Some(dup_id) = person_tuple.17 {
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
            let update_query = r#"
                UPDATE person SET
                    person_type = u.person_type, display_name = u.display_name,
                    external_identifier = u.external_identifier, organization_person_id = u.organization_person_id,
                    messaging1_id = u.messaging1_id, messaging1_type = u.messaging1_type,
                    messaging2_id = u.messaging2_id, messaging2_type = u.messaging2_type,
                    messaging3_id = u.messaging3_id, messaging3_type = u.messaging3_type,
                    messaging4_id = u.messaging4_id, messaging4_type = u.messaging4_type,
                    messaging5_id = u.messaging5_id, messaging5_type = u.messaging5_type,
                    department = u.department, location_id = u.location_id,
                    duplicate_of_person_id = u.duplicate_of_person_id,
                    entity_reference_count = u.entity_reference_count
                FROM (SELECT * FROM UNNEST(
                    $1::uuid[], $2::person_type[], $3::text[], $4::text[],
                    $5::uuid[], $6::uuid[], $7::messaging_type[],
                    $8::uuid[], $9::messaging_type[], $10::uuid[], $11::messaging_type[],
                    $12::uuid[], $13::messaging_type[], $14::uuid[], $15::messaging_type[],
                    $16::text[], $17::uuid[], $18::uuid[], $19::int[]
                )) AS u(
                    id, person_type, display_name, external_identifier,
                    organization_person_id, messaging1_id, messaging1_type,
                    messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                    messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                    department, location_id, duplicate_of_person_id, entity_reference_count
                )
                WHERE person.id = u.id
            "#;

            let (
                ids,
                person_types,
                display_names,
                external_identifiers,
                organization_person_ids,
                messaging1_ids,
                messaging1_types,
                messaging2_ids,
                messaging2_types,
                messaging3_ids,
                messaging3_types,
                messaging4_ids,
                messaging4_types,
                messaging5_ids,
                messaging5_types,
                departments,
                location_ids,
                duplicate_of_person_ids,
                entity_reference_counts,
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
                    acc.14.push(val.14);
                    acc.15.push(val.15);
                    acc.16.push(val.16);
                    acc.17.push(val.17);
                    acc.18.push(val.18);
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
                        .bind(&messaging1_ids)
                        .bind(&messaging1_types)
                        .bind(&messaging2_ids)
                        .bind(&messaging2_types)
                        .bind(&messaging3_ids)
                        .bind(&messaging3_types)
                        .bind(&messaging4_ids)
                        .bind(&messaging4_types)
                        .bind(&messaging5_ids)
                        .bind(&messaging5_types)
                        .bind(&departments)
                        .bind(&location_ids)
                        .bind(&duplicate_of_person_ids)
                        .bind(&entity_reference_counts)
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
                        .bind(&messaging1_ids)
                        .bind(&messaging1_types)
                        .bind(&messaging2_ids)
                        .bind(&messaging2_types)
                        .bind(&messaging3_ids)
                        .bind(&messaging3_types)
                        .bind(&messaging4_ids)
                        .bind(&messaging4_types)
                        .bind(&messaging5_ids)
                        .bind(&messaging5_types)
                        .bind(&departments)
                        .bind(&location_ids)
                        .bind(&duplicate_of_person_ids)
                        .bind(&entity_reference_counts)
                        .execute(&mut **tx)
                        .await?;
                }
            }

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

            let audit_query = r#"
                INSERT INTO person_audit (
                    person_id, version, hash, person_type, display_name, external_identifier,
                    organization_person_id, messaging1_id, messaging1_type,
                    messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                    messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                    department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id
                )
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::int[], $3::bigint[], $4::person_type[], $5::text[], $6::text[],
                    $7::uuid[], $8::uuid[], $9::messaging_type[],
                    $10::uuid[], $11::messaging_type[], $12::uuid[], $13::messaging_type[],
                    $14::uuid[], $15::messaging_type[], $16::uuid[], $17::messaging_type[],
                    $18::text[], $19::uuid[], $20::uuid[], $21::int[], $22::uuid[]
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
                audit_msg1_ids,
                audit_msg1_types,
                audit_msg2_ids,
                audit_msg2_types,
                audit_msg3_ids,
                audit_msg3_types,
                audit_msg4_ids,
                audit_msg4_types,
                audit_msg5_ids,
                audit_msg5_types,
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
                    acc.14.push(val.14);
                    acc.15.push(val.15);
                    acc.16.push(val.16);
                    acc.17.push(val.17);
                    acc.18.push(val.18);
                    acc.19.push(val.19);
                    acc.20.push(val.20);
                    acc.21.push(val.21);
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
                        .bind(&audit_msg1_ids)
                        .bind(&audit_msg1_types)
                        .bind(&audit_msg2_ids)
                        .bind(&audit_msg2_types)
                        .bind(&audit_msg3_ids)
                        .bind(&audit_msg3_types)
                        .bind(&audit_msg4_ids)
                        .bind(&audit_msg4_types)
                        .bind(&audit_msg5_ids)
                        .bind(&audit_msg5_types)
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
                        .bind(&audit_msg1_ids)
                        .bind(&audit_msg1_types)
                        .bind(&audit_msg2_ids)
                        .bind(&audit_msg2_types)
                        .bind(&audit_msg3_ids)
                        .bind(&audit_msg3_types)
                        .bind(&audit_msg4_ids)
                        .bind(&audit_msg4_types)
                        .bind(&audit_msg5_ids)
                        .bind(&audit_msg5_types)
                        .bind(&audit_depts)
                        .bind(&audit_loc_ids)
                        .bind(&audit_dup_ids)
                        .bind(&audit_ref_counts)
                        .bind(&audit_log_ids)
                        .execute(&mut **tx)
                        .await?;
                }
            }
        }

        stats.duration_ms = start.elapsed().as_millis() as u64;
        Ok(updated_items)
    }

    /// Delete multiple persons by their IDs
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
            return Err(Box::new(PersonRepositoryError::IsDuplicatePersonFor(dependent_duplicates)));
        }

        let mut dependent_organizations = Vec::new();
        for id in &existing_ids {
            if !self.find_by_organization_person_id(*id).await?.is_empty() {
                dependent_organizations.push(*id);
            }
        }
        if !dependent_organizations.is_empty() {
            return Err(Box::new(PersonRepositoryError::IsOrganizationPersonFor(dependent_organizations)));
        }

        let items = self.load_batch(&existing_ids).await?;

        for person in items.into_iter().flatten() {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write("".as_bytes());
            let new_hash = hasher.finish() as i64;

            // if let Some(existing_idx) = cache.get_by_primary(&person.id) {
            let existing_idx = existings.iter().find(|p| p.person_id == person.id).unwrap();
            let new_version = existing_idx.version + 1;
            let _external_hash = person.external_identifier.as_ref().map(|s| {
                let mut hasher = XxHash64::with_seed(0);
                hasher.write(s.as_bytes());
                hasher.finish() as i64
            });

            person_audit_values.push((
                person.id,
                new_version,
                new_hash,
                person.person_type,
                person.display_name.to_string(),
                person.external_identifier.as_ref().map(|s| s.to_string()),
                person.organization_person_id,
                person.messaging1_id,
                person.messaging1_type,
                person.messaging2_id,
                person.messaging2_type,
                person.messaging3_id,
                person.messaging3_type,
                person.messaging4_id,
                person.messaging4_type,
                person.messaging5_id,
                person.messaging5_type,
                person.department.as_ref().map(|s| s.to_string()),
                person.location_id,
                person.duplicate_of_person_id,
                person.entity_reference_count,
                Uuid::new_v4(), // audit_log_id, replace with actual
            ));
            // }
        }

        let delete_query = r#"DELETE FROM person WHERE id = ANY($1)"#;
        let delete_idx_query = r#"DELETE FROM person_idx WHERE person_id = ANY($1)"#;

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
        
        let audit_query = r#"
            INSERT INTO person_audit (
                person_id, version, hash, person_type, display_name, external_identifier,
                organization_person_id, messaging1_id, messaging1_type,
                messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id
            )
            SELECT * FROM UNNEST(
                $1::uuid[], $2::int[], $3::bigint[], $4::person_type[], $5::text[], $6::text[],
                $7::uuid[], $8::uuid[], $9::messaging_type[],
                $10::uuid[], $11::messaging_type[], $12::uuid[], $13::messaging_type[],
                $14::uuid[], $15::messaging_type[], $16::uuid[], $17::messaging_type[],
                $18::text[], $19::uuid[], $20::uuid[], $21::int[], $22::uuid[]
            )
        "#;

        let (
            audit_ids, audit_versions, audit_hashes, audit_types, audit_names, audit_ext_ids,
            audit_org_ids, audit_msg1_ids, audit_msg1_types, audit_msg2_ids, audit_msg2_types,
            audit_msg3_ids, audit_msg3_types, audit_msg4_ids, audit_msg4_types, audit_msg5_ids,
            audit_msg5_types, audit_depts, audit_loc_ids, audit_dup_ids, audit_ref_counts,
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
            |mut acc: (
                Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>,
                Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>, Vec<_>,
                Vec<_>, Vec<_>,
            ),
             val| {
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
                acc.14.push(val.14);
                acc.15.push(val.15);
                acc.16.push(val.16);
                acc.17.push(val.17);
                acc.18.push(val.18);
                acc.19.push(val.19);
                acc.20.push(val.20);
                acc.21.push(val.21);
                acc
            },
        );

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(audit_query)
                    .bind(&audit_ids).bind(&audit_versions).bind(&audit_hashes).bind(&audit_types).bind(&audit_names).bind(&audit_ext_ids)
                    .bind(&audit_org_ids).bind(&audit_msg1_ids).bind(&audit_msg1_types).bind(&audit_msg2_ids).bind(&audit_msg2_types)
                    .bind(&audit_msg3_ids).bind(&audit_msg3_types).bind(&audit_msg4_ids).bind(&audit_msg4_types).bind(&audit_msg5_ids)
                    .bind(&audit_msg5_types).bind(&audit_depts).bind(&audit_loc_ids).bind(&audit_dup_ids).bind(&audit_ref_counts)
                    .bind(&audit_log_ids)
                    .execute(&**pool).await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(audit_query)
                    .bind(&audit_ids).bind(&audit_versions).bind(&audit_hashes).bind(&audit_types).bind(&audit_names).bind(&audit_ext_ids)
                    .bind(&audit_org_ids).bind(&audit_msg1_ids).bind(&audit_msg1_types).bind(&audit_msg2_ids).bind(&audit_msg2_types)
                    .bind(&audit_msg3_ids).bind(&audit_msg3_types).bind(&audit_msg4_ids).bind(&audit_msg4_types).bind(&audit_msg5_ids)
                    .bind(&audit_msg5_types).bind(&audit_depts).bind(&audit_loc_ids).bind(&audit_dup_ids).bind(&audit_ref_counts)
                    .bind(&audit_log_ids)
                    .execute(&mut **tx).await?;
            }
        }

        Ok(existing_ids.len())
    }

}

/// Helper functions for batch operations
impl PersonRepositoryImpl {
    /// Process persons in chunks for very large batches
    pub async fn save_batch_chunked(
        &self,
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

        for (chunk_idx, chunk) in items.chunks(chunk_size).enumerate() {
            match self.save_batch(chunk.to_vec(), audit_log_id).await {
                Ok(saved) => {
                    stats.successful_items += saved.len();
                    all_saved.extend(saved);
                }
                Err(e) => {
                    stats.failed_items += chunk.len();
                    errors.push((chunk_idx * chunk_size, e));
                }
            }
        }

        stats.duration_ms = start.elapsed().as_millis() as u64;

        Ok(BatchResult::new(all_saved)
            .with_stats(stats)
            .with_errors(errors))
    }

    /// Validate all persons before batch save
    pub async fn validate_batch(
        &self,
        items: &[PersonModel],
    ) -> Result<Vec<bool>, PersonRepositoryError> {
        let mut validations = Vec::with_capacity(items.len());

        for person in items {
            let mut is_valid = true;

            // Check organization exists if specified
            if let Some(org_id) = person.organization_person_id {
                is_valid = is_valid && self.exists_by_id(org_id).await?;
            }

            // Check location exists if specified
            if let Some(loc_id) = person.location_id {
                is_valid = is_valid
                    && self
                        .location_repository
                        .exists_by_id(loc_id)
                        .await
                        .map_err(|e| PersonRepositoryError::RepositoryError(e.into()))?;
            }

            // Check duplicate person exists if specified
            if let Some(dup_id) = person.duplicate_of_person_id {
                is_valid = is_valid && self.exists_by_id(dup_id).await?;
            }

            validations.push(is_valid);
        }

        Ok(validations)
    }
}
