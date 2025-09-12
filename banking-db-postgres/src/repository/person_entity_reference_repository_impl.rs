use async_trait::async_trait;
use banking_api::BankingResult;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use banking_db::models::person::{
    EntityReferenceAuditModel, EntityReferenceIdxModel, EntityReferenceIdxModelCache,
    EntityReferenceModel,
};
use banking_db::repository::{EntityReferenceRepository, PersonRepository, TransactionAware};
use crate::repository::executor::Executor;
use crate::repository::person_person_repository_impl::PersonRepositoryImpl;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::RwLock as TokioRwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct EntityReferenceRepositoryImpl {
    executor: Executor,
    entity_reference_idx_cache: Arc<TokioRwLock<TransactionAwareEntityReferenceIdxModelCache>>,
    person_repository: Arc<PersonRepositoryImpl>,
}

impl EntityReferenceRepositoryImpl {
    pub fn new(
        executor: Executor,
        person_repository: Arc<PersonRepositoryImpl>,
        entity_reference_idx_cache: Arc<RwLock<EntityReferenceIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            entity_reference_idx_cache: Arc::new(TokioRwLock::new(
                TransactionAwareEntityReferenceIdxModelCache::new(entity_reference_idx_cache),
            )),
            person_repository,
        }
    }

    pub async fn load_all_entity_reference_idx(
        executor: &Executor,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error> {
        let query =
            sqlx::query_as::<_, EntityReferenceIdxModel>("SELECT * FROM entity_reference_idx");
        match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await
            }
        }
    }
}

#[async_trait]
impl EntityReferenceRepository<Postgres> for EntityReferenceRepositoryImpl {
    async fn save(
        &self,
        entity_ref: EntityReferenceModel,
        audit_log_id: Uuid,
    ) -> Result<EntityReferenceModel, sqlx::Error> {
        if !self
            .person_repository
            .exists_by_id(entity_ref.person_id)
            .await
            .map_err(sqlx::Error::Configuration)?
        {
            return Err(sqlx::Error::RowNotFound);
        }

        let mut hasher = XxHash64::with_seed(0);
        let mut entity_ref_cbor = Vec::new();
        ciborium::ser::into_writer(&entity_ref, &mut entity_ref_cbor).unwrap();
        hasher.write(&entity_ref_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.entity_reference_idx_cache.read().await;
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

            let new_idx = EntityReferenceIdxModel {
                entity_reference_id: entity_ref.id,
                person_id: entity_ref.person_id,
                version: new_version,
                hash: new_hash,
            };
            self.entity_reference_idx_cache
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

            let new_idx = EntityReferenceIdxModel {
                entity_reference_id: entity_ref.id,
                person_id: entity_ref.person_id,
                version,
                hash: new_hash,
            };
            self.entity_reference_idx_cache.read().await.add(new_idx);
        }

        Ok(entity_ref)
    }

    async fn load(&self, id: Uuid) -> Result<EntityReferenceModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM entity_reference WHERE id = $1
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

        EntityReferenceModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EntityReferenceIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .entity_reference_idx_cache
            .read()
            .await
            .get_by_primary(&id))
    }

    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM entity_reference_idx WHERE person_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(person_id)
        .bind(page_size)
        .bind((page - 1) * page_size);

        let rows = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };

        let mut refs = Vec::new();
        for row in rows {
            refs.push(
                EntityReferenceIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?,
            );
        }
        Ok(refs)
    }

    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<EntityReferenceIdxModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT ei.*
            FROM entity_reference_idx ei
            JOIN entity_reference e ON ei.entity_reference_id = e.id
            WHERE e.reference_external_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(reference_external_id)
        .bind(page_size)
        .bind((page - 1) * page_size);

        let rows = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };

        let mut refs = Vec::new();
        for row in rows {
            refs.push(
                EntityReferenceIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?,
            );
        }
        Ok(refs)
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<EntityReferenceIdxModel>, Box<dyn Error + Send + Sync>> {
        let query = sqlx::query(
            r#"
            SELECT * FROM entity_reference_idx WHERE entity_reference_id = ANY($1)
            "#,
        )
        .bind(ids);

        let rows = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };

        let mut refs = Vec::new();
        for row in rows {
            refs.push(EntityReferenceIdxModel::try_from_row(&row)?);
        }
        Ok(refs)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let query = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM entity_reference WHERE id = $1)",
            id
        );
        let exists = match &self.executor {
            Executor::Pool(pool) => query.fetch_one(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_one(&mut **tx).await?
            }
        };
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let query = sqlx::query_scalar(
            r#"
            SELECT id FROM entity_reference WHERE person_id = $1
            "#,
        )
        .bind(person_id);

        let ids = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        Ok(ids)
    }
}

#[async_trait]
impl TransactionAware for EntityReferenceRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.entity_reference_idx_cache
            .read()
            .await
            .on_commit()
            .await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.entity_reference_idx_cache
            .read()
            .await
            .on_rollback()
            .await
    }
}

pub struct TransactionAwareEntityReferenceIdxModelCache {
    shared_cache: Arc<RwLock<EntityReferenceIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, EntityReferenceIdxModel>>,
    local_updates: RwLock<HashMap<Uuid, EntityReferenceIdxModel>>,
}

impl TransactionAwareEntityReferenceIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<EntityReferenceIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, item: EntityReferenceIdxModel) {
        let primary_key = item.entity_reference_id;
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: EntityReferenceIdxModel) {
        let primary_key = item.entity_reference_id;
        if let Some(local_item) = self.local_additions.write().get_mut(&primary_key) {
            *local_item = item;
            return;
        }
        self.local_updates.write().insert(primary_key, item);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<EntityReferenceIdxModel> {
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        if let Some(item) = self.local_updates.read().get(primary_key) {
            return Some(item.clone());
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareEntityReferenceIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared_cache = self.shared_cache.write();
        let mut local_additions = self.local_additions.write();
        let mut local_updates = self.local_updates.write();

        for item in local_additions.values() {
            shared_cache.add(item.clone());
        }
        for item in local_updates.values() {
            shared_cache.update(item.clone());
        }

        local_additions.clear();
        local_updates.clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        Ok(())
    }
}

impl TryFromRow<PgRow> for EntityReferenceModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(EntityReferenceModel {
            id: row.get("id"),
            person_id: row.get("person_id"),
            entity_role: row.get("entity_role"),
            reference_external_id: get_heapless_string(row, "reference_external_id")?,
            reference_details_l1: get_optional_heapless_string(
                row,
                "reference_details_l1",
            )?,
            reference_details_l2: get_optional_heapless_string(
                row,
                "reference_details_l2",
            )?,
            reference_details_l3: get_optional_heapless_string(
                row,
                "reference_details_l3",
            )?,
        })
    }
}

impl TryFromRow<PgRow> for EntityReferenceIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(EntityReferenceIdxModel {
            entity_reference_id: row.get("entity_reference_id"),
            person_id: row.get("person_id"),
            version: row.get("version"),
            hash: row.get("hash"),
        })
    }
}