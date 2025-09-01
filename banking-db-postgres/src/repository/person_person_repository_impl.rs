use async_trait::async_trait;
use banking_db::models::person::{
    PersonAuditModel, PersonIdxModel, PersonIdxModelCache, PersonModel,
};
use banking_db::repository::{LocationRepository, PersonRepository};
use crate::repository::executor::Executor;
use crate::repository::person_location_repository_impl::LocationRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::{Arc, RwLock};
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct PersonRepositoryImpl {
    executor: Executor,
    person_idx_cache: Arc<RwLock<PersonIdxModelCache>>,
    location_repository: Arc<LocationRepositoryImpl>,
}

impl PersonRepositoryImpl {
    pub async fn new(
        executor: Executor,
        location_repository: Arc<LocationRepositoryImpl>,
    ) -> Self {
        let person_idx_models = Self::load_all_person_idx(&executor).await.unwrap();
        let person_idx_cache =
            Arc::new(RwLock::new(PersonIdxModelCache::new(person_idx_models).unwrap()));
        Self {
            executor,
            person_idx_cache,
            location_repository,
        }
    }

    async fn load_all_person_idx(
        executor: &Executor,
    ) -> Result<Vec<PersonIdxModel>, sqlx::Error> {
        let query = sqlx::query_as::<_, PersonIdxModel>("SELECT * FROM person_idx");
        match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await
            }
        }
    }
}

/// # Refactoring Instruction
/// ## add a method load(id) returning PersonModel. also add to PersonRepository,
///    It is the only methode that returns a PersonModel. All other finders return a PersonIdxModel.
#[async_trait]
impl PersonRepository<Postgres> for PersonRepositoryImpl {
    async fn save(
        &self,
        person: PersonModel,
        audit_log_id: Uuid,
    ) -> Result<PersonModel, sqlx::Error> {
        if let Some(org_id) = person.organization_person_id {
            if !self
                .exists_by_id(org_id)
                .await
                .map_err(sqlx::Error::Configuration)?
            {
                return Err(sqlx::Error::RowNotFound);
            }
        }
        if let Some(loc_id) = person.location_id {
            if !self
                .location_repository
                .exists_by_id(loc_id)
                .await
                .map_err(sqlx::Error::Configuration)?
            {
                return Err(sqlx::Error::RowNotFound);
            }
        }
        if let Some(dup_id) = person.duplicate_of_person_id {
            if !self
                .exists_by_id(dup_id)
                .await
                .map_err(sqlx::Error::Configuration)?
            {
                return Err(sqlx::Error::RowNotFound);
            }
        }

        let mut hasher = XxHash64::with_seed(0);
        let mut person_cbor = Vec::new();
        ciborium::ser::into_writer(&person, &mut person_cbor).unwrap();
        hasher.write(&person_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.person_idx_cache.read().unwrap();
            cache_read_guard.get_by_primary(&person.id)
        };

        let new_external_hash = person.external_identifier.as_ref().map(|s| {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(s.as_bytes());
            hasher.finish() as i64
        });

        if let Some(existing_idx) = maybe_existing_idx {
            // UPDATE
            if existing_idx.hash == new_hash {
                return Ok(person); // No changes
            }

            let new_version = existing_idx.version + 1;

            let audit_model = PersonAuditModel {
                person_id: person.id,
                version: new_version,
                hash: new_hash,
                person_type: person.person_type,
                display_name: person.display_name.clone(),
                external_identifier: person.external_identifier.clone(),
                entity_reference_count: person.entity_reference_count,
                organization_person_id: person.organization_person_id,
                messaging1_id: person.messaging1_id,
                messaging1_type: person.messaging1_type,
                messaging2_id: person.messaging2_id,
                messaging2_type: person.messaging2_type,
                messaging3_id: person.messaging3_id,
                messaging3_type: person.messaging3_type,
                messaging4_id: person.messaging4_id,
                messaging4_type: person.messaging4_type,
                messaging5_id: person.messaging5_id,
                messaging5_type: person.messaging5_type,
                department: person.department.clone(),
                location_id: person.location_id,
                duplicate_of_person_id: person.duplicate_of_person_id,
                audit_log_id,
            };

            let query1 = sqlx::query(
                r#"
                INSERT INTO person_audit (person_id, version, hash, person_type, display_name, external_identifier, organization_person_id, messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type, messaging4_id, messaging4_type, messaging5_id, messaging5_type, department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                "#,
            )
            .bind(audit_model.person_id)
            .bind(audit_model.version)
            .bind(audit_model.hash)
            .bind(audit_model.person_type)
            .bind(audit_model.display_name.as_str())
            .bind(
                audit_model
                    .external_identifier
                    .as_ref()
                    .map(|s| s.as_str()),
            )
            .bind(audit_model.organization_person_id)
            .bind(audit_model.messaging1_id)
            .bind(audit_model.messaging1_type)
            .bind(audit_model.messaging2_id)
            .bind(audit_model.messaging2_type)
            .bind(audit_model.messaging3_id)
            .bind(audit_model.messaging3_type)
            .bind(audit_model.messaging4_id)
            .bind(audit_model.messaging4_type)
            .bind(audit_model.messaging5_id)
            .bind(audit_model.messaging5_type)
            .bind(audit_model.department.as_ref().map(|s| s.as_str()))
            .bind(audit_model.location_id)
            .bind(audit_model.duplicate_of_person_id)
            .bind(audit_model.entity_reference_count)
            .bind(audit_model.audit_log_id);

            let query2 = sqlx::query(
                r#"
                UPDATE person SET
                    person_type = $2::person_type, display_name = $3, external_identifier = $4,
                    organization_person_id = $5, messaging1_id = $6, messaging1_type = $7::messaging_type,
                    messaging2_id = $8, messaging2_type = $9::messaging_type, messaging3_id = $10,
                    messaging3_type = $11::messaging_type, messaging4_id = $12, messaging4_type = $13::messaging_type,
                    messaging5_id = $14, messaging5_type = $15, department = $16,
                    location_id = $17, duplicate_of_person_id = $18, entity_reference_count = $19
                WHERE id = $1
                "#,
            )
            .bind(person.id)
            .bind(person.person_type)
            .bind(person.display_name.as_str())
            .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
            .bind(person.organization_person_id)
            .bind(person.messaging1_id)
            .bind(person.messaging1_type)
            .bind(person.messaging2_id)
            .bind(person.messaging2_type)
            .bind(person.messaging3_id)
            .bind(person.messaging3_type)
            .bind(person.messaging4_id)
            .bind(person.messaging4_type)
            .bind(person.messaging5_id)
            .bind(person.messaging5_type)
            .bind(person.department.as_ref().map(|s| s.as_str()))
            .bind(person.location_id)
            .bind(person.duplicate_of_person_id)
            .bind(person.entity_reference_count);

            let query3 = sqlx::query(
                r#"
                UPDATE person_idx SET
                    external_identifier_hash = $2,
                    version = $3,
                    hash = $4
                WHERE person_id = $1
                "#,
            )
            .bind(person.id)
            .bind(new_external_hash)
            .bind(new_version)
            .bind(new_hash);

            match &self.executor {
                Executor::Pool(pool) => {
                    query1.execute(&**pool).await?;
                    query2.execute(&**pool).await?;
                    query3.execute(&**pool).await?;
                }
                Executor::Tx(tx) => {
                    let mut tx = tx.lock().await;
                    query1.execute(&mut **tx).await?;
                    query2.execute(&mut **tx).await?;
                    query3.execute(&mut **tx).await?;
                }
            }

            let new_idx = PersonIdxModel {
                person_id: person.id,
                external_identifier_hash: new_external_hash,
                version: new_version,
                hash: new_hash,
            };
            self.person_idx_cache.write().unwrap().update(new_idx);
        } else {
            // INSERT
            let version = 0;
            let audit_model = PersonAuditModel {
                person_id: person.id,
                version,
                hash: new_hash,
                person_type: person.person_type,
                display_name: person.display_name.clone(),
                external_identifier: person.external_identifier.clone(),
                entity_reference_count: person.entity_reference_count,
                organization_person_id: person.organization_person_id,
                messaging1_id: person.messaging1_id,
                messaging1_type: person.messaging1_type,
                messaging2_id: person.messaging2_id,
                messaging2_type: person.messaging2_type,
                messaging3_id: person.messaging3_id,
                messaging3_type: person.messaging3_type,
                messaging4_id: person.messaging4_id,
                messaging4_type: person.messaging4_type,
                messaging5_id: person.messaging5_id,
                messaging5_type: person.messaging5_type,
                department: person.department.clone(),
                location_id: person.location_id,
                duplicate_of_person_id: person.duplicate_of_person_id,
                audit_log_id,
            };

            let query1 = sqlx::query(
                r#"
                INSERT INTO person_audit (person_id, version, hash, person_type, display_name, external_identifier, organization_person_id, messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type, messaging4_id, messaging4_type, messaging5_id, messaging5_type, department, location_id, duplicate_of_person_id, entity_reference_count, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                "#,
            )
            .bind(audit_model.person_id)
            .bind(audit_model.version)
            .bind(audit_model.hash)
            .bind(audit_model.person_type)
            .bind(audit_model.display_name.as_str())
            .bind(
                audit_model
                    .external_identifier
                    .as_ref()
                    .map(|s| s.as_str()),
            )
            .bind(audit_model.organization_person_id)
            .bind(audit_model.messaging1_id)
            .bind(audit_model.messaging1_type)
            .bind(audit_model.messaging2_id)
            .bind(audit_model.messaging2_type)
            .bind(audit_model.messaging3_id)
            .bind(audit_model.messaging3_type)
            .bind(audit_model.messaging4_id)
            .bind(audit_model.messaging4_type)
            .bind(audit_model.messaging5_id)
            .bind(audit_model.messaging5_type)
            .bind(audit_model.department.as_ref().map(|s| s.as_str()))
            .bind(audit_model.location_id)
            .bind(audit_model.duplicate_of_person_id)
            .bind(audit_model.entity_reference_count)
            .bind(audit_model.audit_log_id);

            let query2 = sqlx::query(
                r#"
                INSERT INTO person (id, person_type, display_name, external_identifier, organization_person_id, messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type, messaging4_id, messaging4_type, messaging5_id, messaging5_type, department, location_id, duplicate_of_person_id, entity_reference_count)
                VALUES ($1, $2::person_type, $3, $4, $5, $6, $7::messaging_type, $8, $9::messaging_type, $10, $11::messaging_type, $12, $13::messaging_type, $14, $15, $16, $17, $18, $19)
                "#,
            )
            .bind(person.id)
            .bind(person.person_type)
            .bind(person.display_name.as_str())
            .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
            .bind(person.organization_person_id)
            .bind(person.messaging1_id)
            .bind(person.messaging1_type)
            .bind(person.messaging2_id)
            .bind(person.messaging2_type)
            .bind(person.messaging3_id)
            .bind(person.messaging3_type)
            .bind(person.messaging4_id)
            .bind(person.messaging4_type)
            .bind(person.messaging5_id)
            .bind(person.messaging5_type)
            .bind(person.department.as_ref().map(|s| s.as_str()))
            .bind(person.location_id)
            .bind(person.duplicate_of_person_id)
            .bind(person.entity_reference_count);

            let query3 = sqlx::query(
                r#"
                INSERT INTO person_idx (person_id, external_identifier_hash, version, hash)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(person.id)
            .bind(new_external_hash)
            .bind(version)
            .bind(new_hash);

            match &self.executor {
                Executor::Pool(pool) => {
                    query1.execute(&**pool).await?;
                    query2.execute(&**pool).await?;
                    query3.execute(&**pool).await?;
                }
                Executor::Tx(tx) => {
                    let mut tx = tx.lock().await;
                    query1.execute(&mut **tx).await?;
                    query2.execute(&mut **tx).await?;
                    query3.execute(&mut **tx).await?;
                }
            }

            let new_idx = PersonIdxModel {
                person_id: person.id,
                external_identifier_hash: new_external_hash,
                version,
                hash: new_hash,
            };
            self.person_idx_cache.write().unwrap().add(new_idx);
        }

        Ok(person)
    }

    async fn load(&self, id: Uuid) -> Result<PersonModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM person WHERE id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) => query.fetch_one(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_one(&mut **tx).await?
            }
        };

        PersonModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<PersonIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.person_idx_cache.read().unwrap().get_by_primary(&id))
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<PersonIdxModel>, Box<dyn Error + Send + Sync>> {
        let cache = self.person_idx_cache.read().unwrap();
        let results = ids
            .iter()
            .filter_map(|id| cache.get_by_primary(id))
            .collect();
        Ok(results)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.person_idx_cache.read().unwrap().contains_primary(&id))
    }

    async fn get_ids_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(identifier.as_bytes());
        let hash = hasher.finish() as i64;

        let cache = self.person_idx_cache.read().unwrap();
        Ok(cache
            .get_by_external_identifier_hash(&hash)
            .cloned()
            .unwrap_or_default())
    }

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> Result<Vec<PersonIdxModel>, Box<dyn Error + Send + Sync>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(identifier.as_bytes());
        let hash = hasher.finish() as i64;

        let cache = self.person_idx_cache.read().unwrap();
        let ids = cache
            .get_by_external_identifier_hash(&hash)
            .cloned()
            .unwrap_or_default();
        let results = ids
            .iter()
            .filter_map(|id| cache.get_by_primary(id))
            .collect();
        Ok(results)
    }
}

impl TryFromRow<PgRow> for PersonModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(PersonModel {
            id: row.get("id"),
            person_type: row.get("person_type"),
            display_name: get_heapless_string(row, "display_name")?,
            external_identifier: get_optional_heapless_string(row, "external_identifier")?,
            organization_person_id: row.get("organization_person_id"),
            messaging1_id: row.get("messaging1_id"),
            messaging1_type: row.get("messaging1_type"),
            messaging2_id: row.get("messaging2_id"),
            messaging2_type: row.get("messaging2_type"),
            messaging3_id: row.get("messaging3_id"),
            messaging3_type: row.get("messaging3_type"),
            messaging4_id: row.get("messaging4_id"),
            messaging4_type: row.get("messaging4_type"),
            messaging5_id: row.get("messaging5_id"),
            messaging5_type: row.get("messaging5_type"),
            department: get_optional_heapless_string(row, "department")?,
            location_id: row.get("location_id"),
            duplicate_of_person_id: row.get("duplicate_of_person_id"),
            entity_reference_count: row.get("entity_reference_count"),
        })
    }
}