use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{
    LocationAuditModel, LocationIdxModel, LocationIdxModelCache, LocationModel,
};
use banking_db::repository::{
    LocalityRepository, LocationRepository, LocationRepositoryError, LocationResult,
    TransactionAware,
};
use crate::repository::executor::Executor;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hasher;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct LocationRepositoryImpl {
    pub(crate) executor: Executor,
    pub(crate) location_idx_cache: Arc<TokioRwLock<TransactionAwareLocationIdxModelCache>>,
    pub(crate) locality_repository: Arc<LocalityRepositoryImpl>,
}

impl LocationRepositoryImpl {
    pub fn new(
        executor: Executor,
        locality_repository: Arc<LocalityRepositoryImpl>,
        location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            location_idx_cache: Arc::new(TokioRwLock::new(
                TransactionAwareLocationIdxModelCache::new(location_idx_cache),
            )),
            locality_repository,
        }
    }

    pub async fn load_all_location_idx(
        executor: &Executor,
    ) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        let query = sqlx::query_as::<_, LocationIdxModel>("SELECT * FROM location_idx");
        match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await
            }
        }
    }

    pub(crate) async fn get_idx_by_id(
        &self,
        id: Uuid,
    ) -> LocationResult<Option<LocationIdxModel>> {
        let cache = self.location_idx_cache.read().await;
        Ok(cache.get_by_primary(&id))
    }
}

#[async_trait]
impl LocationRepository<Postgres> for LocationRepositoryImpl {
    async fn save(
        &self,
        location: LocationModel,
        audit_log_id: Uuid,
    ) -> LocationResult<LocationModel> {
        crate::repository::person::location_repository::save::save(self, location, audit_log_id)
            .await
    }

    async fn load(&self, id: Uuid) -> LocationResult<LocationModel> {
        crate::repository::person::location_repository::load::load(self, id).await
    }

    async fn find_by_id(&self, id: Uuid) -> LocationResult<Option<LocationIdxModel>> {
        crate::repository::person::location_repository::find_by_id::find_by_id(self, id).await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> LocationResult<Vec<LocationIdxModel>> {
        crate::repository::person::location_repository::find_by_ids::find_by_ids(self, ids).await
    }

    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> LocationResult<Vec<LocationIdxModel>> {
        crate::repository::person::location_repository::find_by_locality_id::find_by_locality_id(
            self,
            locality_id,
            page,
            page_size,
        )
        .await
    }

    async fn exists_by_id(&self, id: Uuid) -> LocationResult<bool> {
        crate::repository::person::location_repository::exists_by_id::exists_by_id(self, id).await
    }

    async fn find_ids_by_locality_id(&self, locality_id: Uuid) -> LocationResult<Vec<Uuid>> {
        crate::repository::person::location_repository::find_ids_by_locality_id::find_ids_by_locality_id(
            self,
            locality_id,
        )
        .await
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> LocationResult<Vec<(Uuid, bool)>> {
        crate::repository::person::location_repository::exist_by_ids::exist_by_ids(self, ids).await
    }
}

#[async_trait]
impl TransactionAware for LocationRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.location_idx_cache.read().await.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.location_idx_cache.read().await.on_rollback().await
    }
}

pub struct TransactionAwareLocationIdxModelCache {
    shared_cache: Arc<RwLock<LocationIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, LocationIdxModel>>,
    local_updates: RwLock<HashMap<Uuid, LocationIdxModel>>,
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwareLocationIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<LocationIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: LocationIdxModel) {
        let primary_key = item.location_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: LocationIdxModel) {
        let primary_key = item.location_id;
        self.local_deletions.write().remove(&primary_key);
        if let Some(local_item) = self.local_additions.write().get_mut(&primary_key) {
            *local_item = item;
            return;
        }
        self.local_updates.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        self.local_additions.write().remove(primary_key);
        self.local_updates.write().remove(primary_key);
        self.local_deletions.write().insert(*primary_key);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocationIdxModel> {
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

    pub fn get_by_locality_id(&self, locality_id: &Uuid) -> Vec<LocationIdxModel> {
        let shared_cache = self.shared_cache.read();
        let local_additions = self.local_additions.read();
        let local_updates = self.local_updates.read();
        let local_deletions = self.local_deletions.read();

        let mut results: HashMap<Uuid, LocationIdxModel> = HashMap::new();

        if let Some(ids) = shared_cache.get_by_locality_id(locality_id) {
            for id in ids {
                if let Some(item) = shared_cache.get_by_primary(id) {
                    results.insert(item.location_id, item);
                }
            }
        }

        for id in local_updates.keys() {
            results.remove(id);
        }
        for id in local_deletions.iter() {
            results.remove(id);
        }

        for item in local_additions.values() {
            if item.locality_id == *locality_id {
                results.insert(item.location_id, item.clone());
            }
        }
        for item in local_updates.values() {
            if item.locality_id == *locality_id {
                results.insert(item.location_id, item.clone());
            }
        }

        results.into_values().collect()
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareLocationIdxModelCache {
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

impl TryFromRow<PgRow> for LocationModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocationModel {
            id: row.get("id"),
            location_type: row.get("location_type"),
            street_line1: get_heapless_string(row, "street_line1")?,
            street_line2: get_optional_heapless_string(row, "street_line2")?,
            street_line3: get_optional_heapless_string(row, "street_line3")?,
            street_line4: get_optional_heapless_string(row, "street_line4")?,
            locality_id: row.get("locality_id"),
            postal_code: get_optional_heapless_string(row, "postal_code")?,
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            accuracy_meters: row.get("accuracy_meters"),
        })
    }
}

impl TryFromRow<PgRow> for LocationIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocationIdxModel {
            location_id: row.get("location_id"),
            locality_id: row.get("locality_id"),
            version: row.get("version"),
            hash: row.get("hash"),
        })
    }
}