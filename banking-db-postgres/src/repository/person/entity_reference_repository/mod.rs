use async_trait::async_trait;
use banking_api::BankingResult;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use banking_db::models::person::{
    EntityReferenceIdxModel, EntityReferenceIdxModelCache,
    EntityReferenceModel,
};
use banking_db::repository::{EntityReferenceRepository, TransactionAware};
use banking_db::repository::person::entity_reference_repository::EntityReferenceResult;
use crate::repository::executor::Executor;
use crate::repository::person::person_repository::PersonRepositoryImpl;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::RwLock as TokioRwLock;
use uuid::Uuid;

pub mod batch_impl;
pub mod batch_helper;
pub mod create_batch;
pub mod delete_batch;
pub mod load_batch;
pub mod update_batch;
pub mod exist_by_ids;
pub mod exists_by_id;
pub mod find_by_id;
pub mod find_by_ids;
pub mod find_by_person_id;
pub mod find_by_reference_external_id;
pub mod find_ids_by_person_id;
pub mod load;
pub mod save;

pub struct EntityReferenceRepositoryImpl {
    pub executor: Executor,
    pub entity_reference_idx_cache:
        Arc<TokioRwLock<TransactionAwareEntityReferenceIdxModelCache>>,
    pub person_repository: Arc<PersonRepositoryImpl>,
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
    ) -> EntityReferenceResult<EntityReferenceModel> {
        crate::repository::person::entity_reference_repository::save::save(
            self,
            entity_ref,
            audit_log_id,
        )
        .await
    }

    async fn load(&self, id: Uuid) -> EntityReferenceResult<EntityReferenceModel> {
        crate::repository::person::entity_reference_repository::load::load(self, id).await
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> EntityReferenceResult<Option<EntityReferenceIdxModel>> {
        crate::repository::person::entity_reference_repository::find_by_id::find_by_id(self, id).await
    }

    async fn find_by_person_id(
        &self,
        person_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
        crate::repository::person::entity_reference_repository::find_by_person_id::find_by_person_id(
            self, person_id, page, page_size,
        )
        .await
    }

    async fn find_by_reference_external_id(
        &self,
        reference_external_id: &str,
        page: i32,
        page_size: i32,
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
        crate::repository::person::entity_reference_repository::find_by_reference_external_id::find_by_reference_external_id(
            self,
            reference_external_id,
            page,
            page_size,
        )
        .await
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> EntityReferenceResult<Vec<EntityReferenceIdxModel>> {
        crate::repository::person::entity_reference_repository::find_by_ids::find_by_ids(self, ids).await
    }

    async fn exists_by_id(&self, id: Uuid) -> EntityReferenceResult<bool> {
        crate::repository::person::entity_reference_repository::exists_by_id::exists_by_id(self, id)
            .await
    }

    async fn find_ids_by_person_id(
        &self,
        person_id: Uuid,
    ) -> EntityReferenceResult<Vec<Uuid>> {
        crate::repository::person::entity_reference_repository::find_ids_by_person_id::find_ids_by_person_id(
            self, person_id,
        )
        .await
    }

    async fn exist_by_ids(
        &self,
        ids: &[Uuid],
    ) -> EntityReferenceResult<Vec<(Uuid, bool)>> {
        crate::repository::person::entity_reference_repository::exist_by_ids::exist_by_ids(
            self, ids,
        )
        .await
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
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwareEntityReferenceIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<EntityReferenceIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: EntityReferenceIdxModel) {
        let primary_key = item.entity_reference_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: EntityReferenceIdxModel) {
        let primary_key = item.entity_reference_id;
        self.local_deletions.write().remove(&primary_key);
        if let Some(local_item) = self.local_additions.write().get_mut(&primary_key) {
            *local_item = item;
            return;
        }
        self.local_updates.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        if self.local_additions.write().remove(primary_key).is_none() {
            self.local_deletions.write().insert(*primary_key);
        }
        self.local_updates.write().remove(primary_key);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<EntityReferenceIdxModel> {
        if self.local_deletions.read().contains(primary_key) {
            return None;
        }
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        if let Some(item) = self.local_updates.read().get(primary_key) {
            return Some(item.clone());
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }

    pub fn get_by_person_id(&self, person_id: &Uuid) -> Option<Vec<Uuid>> {
        let shared_cache = self.shared_cache.read();
        let local_additions = self.local_additions.read();
        let local_updates = self.local_updates.read();
        let local_deletions = self.local_deletions.read();

        let mut result_ids: HashSet<Uuid> = HashSet::new();

        if let Some(ids) = shared_cache.get_by_person_id(person_id) {
            for id in ids {
                if !local_deletions.contains(id) {
                    result_ids.insert(*id);
                }
            }
        }

        for updated_item in local_updates.values() {
            if let Some(original_item) =
                shared_cache.get_by_primary(&updated_item.entity_reference_id)
            {
                if original_item.person_id == *person_id && updated_item.person_id != *person_id {
                    result_ids.remove(&updated_item.entity_reference_id);
                }
            }
        }

        for item in local_additions.values() {
            if item.person_id == *person_id {
                result_ids.insert(item.entity_reference_id);
            }
        }
        for item in local_updates.values() {
            if item.person_id == *person_id {
                result_ids.insert(item.entity_reference_id);
            }
        }

        if result_ids.is_empty() {
            None
        } else {
            Some(result_ids.into_iter().collect())
        }
    }

    pub fn get_by_reference_external_id_hash(&self, hash: &i64) -> Option<Vec<Uuid>> {
        let shared_cache = self.shared_cache.read();
        let local_additions = self.local_additions.read();
        let local_updates = self.local_updates.read();
        let local_deletions = self.local_deletions.read();

        let mut result_ids: HashSet<Uuid> = HashSet::new();

        if let Some(ids) = shared_cache.get_by_reference_external_id_hash(hash) {
            for id in ids {
                if !local_deletions.contains(id) {
                    result_ids.insert(*id);
                }
            }
        }

        for updated_item in local_updates.values() {
            if let Some(original_item) =
                shared_cache.get_by_primary(&updated_item.entity_reference_id)
            {
                if original_item.reference_external_id_hash == *hash
                    && updated_item.reference_external_id_hash != *hash
                {
                    result_ids.remove(&updated_item.entity_reference_id);
                }
            }
        }

        for item in local_additions.values() {
            if item.reference_external_id_hash == *hash {
                result_ids.insert(item.entity_reference_id);
            }
        }
        for item in local_updates.values() {
            if item.reference_external_id_hash == *hash {
                result_ids.insert(item.entity_reference_id);
            }
        }

        if result_ids.is_empty() {
            None
        } else {
            Some(result_ids.into_iter().collect())
        }
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareEntityReferenceIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared_cache = self.shared_cache.write();
        let mut local_additions = self.local_additions.write();
        let mut local_updates = self.local_updates.write();
        let mut local_deletions = self.local_deletions.write();

        for item in local_additions.values() {
            shared_cache.add(item.clone());
        }
        for item in local_updates.values() {
            shared_cache.update(item.clone());
        }
        for primary_key in local_deletions.iter() {
            shared_cache.remove(primary_key);
        }

        local_additions.clear();
        local_updates.clear();
        local_deletions.clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        self.local_deletions.write().clear();
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
            reference_external_id_hash: row.get("reference_external_id_hash"),
            version: row.get("version"),
            hash: row.get("hash"),
        })
    }
}