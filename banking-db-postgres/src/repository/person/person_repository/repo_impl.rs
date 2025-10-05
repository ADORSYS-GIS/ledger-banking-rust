use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{PersonIdxModel, PersonIdxModelCache, PersonModel};
use banking_db::repository::{PersonRepository, PersonResult, TransactionAware};
use crate::repository::executor::Executor;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use parking_lot::RwLock;
use tokio::sync::RwLock as TokioRwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

pub struct PersonRepositoryImpl {
    pub(crate) executor: Executor,
    pub(crate) person_idx_cache: Arc<TokioRwLock<TransactionAwarePersonIdxModelCache>>,
    pub(crate) location_repository: Arc<LocationRepositoryImpl>,
}

impl PersonRepositoryImpl {
    pub fn new(
        executor: Executor,
        location_repository: Arc<LocationRepositoryImpl>,
        person_idx_cache: Arc<RwLock<PersonIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            person_idx_cache: Arc::new(TokioRwLock::new(
                    TransactionAwarePersonIdxModelCache::new(person_idx_cache),
                )),
            location_repository,
        }
    }

    pub async fn load_all_person_idx(
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

#[async_trait]
impl PersonRepository<Postgres> for PersonRepositoryImpl {
    async fn save(
        &self,
        person: PersonModel,
        audit_log_id: Uuid,
    ) -> PersonResult<PersonModel> {
        crate::repository::person::person_repository::save::save(self, person, audit_log_id).await
    }

    async fn load(&self, id: Uuid) -> PersonResult<PersonModel> {
        crate::repository::person::person_repository::load::load(self, id).await
    }

    async fn find_by_id(&self, id: Uuid) -> PersonResult<Option<PersonIdxModel>> {
        crate::repository::person::person_repository::find_by_id::find_by_id(self, id).await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<PersonIdxModel>> {
        crate::repository::person::person_repository::find_by_ids::find_by_ids(self, ids).await
    }

    async fn exists_by_id(&self, id: Uuid) -> PersonResult<bool> {
        crate::repository::person::person_repository::exists_by_id::exists_by_id(self, id).await
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<(Uuid, bool)>> {
        crate::repository::person::person_repository::exist_by_ids::exist_by_ids(self, ids).await
    }

    async fn get_ids_by_external_identifier(&self, identifier: &str) -> PersonResult<Vec<Uuid>> {
        crate::repository::person::person_repository::get_ids_by_external_identifier::get_ids_by_external_identifier(self, identifier).await
    }

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        crate::repository::person::person_repository::get_by_external_identifier::get_by_external_identifier(self, identifier).await
    }

    async fn find_by_duplicate_of_person_id(
        &self,
        person_id: Uuid,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        crate::repository::person::person_repository::find_by_duplicate_of_person_id::find_by_duplicate_of_person_id(self, person_id).await
    }

    async fn find_by_organization_person_id(
        &self,
        person_id: Uuid,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        crate::repository::person::person_repository::find_by_organization_person_id::find_by_organization_person_id(self, person_id).await
    }
}

#[async_trait]
impl TransactionAware for PersonRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        let cache = self.person_idx_cache.read().await;
        cache.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        let cache = self.person_idx_cache.read().await;
        cache.on_rollback().await
    }
}

pub struct TransactionAwarePersonIdxModelCache {
    shared_cache: Arc<RwLock<PersonIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, PersonIdxModel>>,
    local_updates: RwLock<HashMap<Uuid, PersonIdxModel>>,
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwarePersonIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<PersonIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: PersonIdxModel) {
        let primary_key = item.person_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: PersonIdxModel) {
        let primary_key = item.person_id;
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

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<PersonIdxModel> {
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

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        if self.local_deletions.read().contains(primary_key) {
            return false;
        }
        if self.local_additions.read().contains_key(primary_key) {
            return true;
        }
        if self.local_updates.read().contains_key(primary_key) {
            return true;
        }
        self.shared_cache.read().contains_primary(primary_key)
    }

    pub fn get_by_external_identifier_hash(&self, hash: &i64) -> Option<Vec<Uuid>> {
        let shared_cache = self.shared_cache.read();
        let items: Vec<Uuid> = shared_cache
            .get_by_external_identifier_hash(hash)
            .cloned()
            .unwrap_or_default();
        let mut result_set: HashSet<Uuid> = items.into_iter().collect();

        for id in self.local_deletions.read().iter() {
            result_set.remove(id);
        }
        for (key, item) in self.local_additions.read().iter() {
            if item.external_identifier_hash == Some(*hash) {
                result_set.insert(*key);
            }
        }
        for (key, item) in self.local_updates.read().iter() {
            if item.external_identifier_hash == Some(*hash) {
                result_set.insert(*key);
            }
        }

        if result_set.is_empty() {
            None
        } else {
            Some(result_set.into_iter().collect())
        }
    }

    pub fn iter(&self) -> Vec<PersonIdxModel> {
        let mut combined = Vec::new();
        let shared = self.shared_cache.read();
        for item in shared.iter() {
            combined.push(
                self.local_updates.read().get(&item.person_id).cloned().unwrap_or_else(|| {
                    if self.local_deletions.read().contains(&item.person_id) {
                        return item.clone();
                    }
                    item.clone()
                }),
            );
        }
        combined.extend(self.local_additions.read().values().cloned());
        combined.retain(|item| !self.local_deletions.read().contains(&item.person_id));
        combined
    }
}

#[async_trait]
impl TransactionAware for TransactionAwarePersonIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared = self.shared_cache.write();
        for item in self.local_additions.read().values() {
            shared.add(item.clone());
        }
        for item in self.local_updates.read().values() {
            shared.update(item.clone());
        }
        for id in self.local_deletions.read().iter() {
            shared.remove(id);
        }
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        self.local_deletions.write().clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        self.local_deletions.write().clear();
        Ok(())
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
            messaging_info1: get_optional_heapless_string(row, "messaging_info1")?,
            messaging_info2: get_optional_heapless_string(row, "messaging_info2")?,
            messaging_info3: get_optional_heapless_string(row, "messaging_info3")?,
            messaging_info4: get_optional_heapless_string(row, "messaging_info4")?,
            messaging_info5: get_optional_heapless_string(row, "messaging_info5")?,
            department: get_optional_heapless_string(row, "department")?,
            location_id: row.get("location_id"),
            duplicate_of_person_id: row.get("duplicate_of_person_id"),
            entity_reference_count: row.get("entity_reference_count"),
        })
    }
}