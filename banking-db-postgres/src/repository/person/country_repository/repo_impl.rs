use crate::repository::executor::Executor;
use crate::repository::person::country_repository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{CountryIdxModel, CountryIdxModelCache, CountryModel};
use banking_db::repository::person::country_repository::{CountryRepository, CountryResult};
use banking_db::repository::TransactionAware;
use heapless::String as HeaplessString;
use parking_lot::RwLock as ParkingRwLock;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct CountryRepositoryImpl {
    pub executor: Executor,
    pub country_idx_cache: Arc<RwLock<TransactionAwareCountryIdxModelCache>>,
}

impl CountryRepositoryImpl {
    pub fn new(
        executor: Executor,
        country_idx_cache: Arc<ParkingRwLock<CountryIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            country_idx_cache: Arc::new(RwLock::new(TransactionAwareCountryIdxModelCache::new(
                country_idx_cache,
            ))),
        }
    }

    pub async fn load_all_country_idx(
        executor: &Executor,
    ) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
        let query = sqlx::query("SELECT * FROM country_idx");
        let rows = match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        let mut idx_models = Vec::with_capacity(rows.len());
        for row in rows {
            idx_models.push(CountryIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(idx_models)
    }
}

#[async_trait]
impl CountryRepository<Postgres> for CountryRepositoryImpl {
    async fn save(&self, country: CountryModel) -> CountryResult<CountryModel> {
        country_repository::save::save(self, country).await
    }

    async fn load(&self, id: Uuid) -> CountryResult<CountryModel> {
        country_repository::load::load(self, id).await
    }

    async fn find_by_id(&self, id: Uuid) -> CountryResult<Option<CountryIdxModel>> {
        country_repository::find_by_id::find_by_id(self, id).await
    }

    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: i32,
        page_size: i32,
    ) -> CountryResult<Vec<CountryIdxModel>> {
        country_repository::find_by_iso2::find_by_iso2(self, iso2, page, page_size).await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> CountryResult<Vec<CountryIdxModel>> {
        country_repository::find_by_ids::find_by_ids(self, ids).await
    }

    async fn exists_by_id(&self, id: Uuid) -> CountryResult<bool> {
        country_repository::exists_by_id::exists_by_id(self, id).await
    }

    async fn find_ids_by_iso2(&self, iso2: &str) -> CountryResult<Vec<Uuid>> {
        country_repository::find_ids_by_iso2::find_ids_by_iso2(self, iso2).await
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> CountryResult<Vec<(Uuid, bool)>> {
        country_repository::exist_by_ids::exist_by_ids(self, ids).await
    }
}

#[async_trait]
impl TransactionAware for CountryRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.country_idx_cache.read().await.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.country_idx_cache.read().await.on_rollback().await
    }
}

impl TryFromRow<PgRow> for CountryModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CountryModel {
            id: row.get("id"),
            iso2: get_heapless_string(row, "iso2")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
        })
    }
}

impl TryFromRow<PgRow> for CountryIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CountryIdxModel {
            country_id: row.get("country_id"),
            iso2: get_heapless_string(row, "iso2")?,
        })
    }
}

pub struct TransactionAwareCountryIdxModelCache {
    shared_cache: Arc<ParkingRwLock<CountryIdxModelCache>>,
    local_additions: ParkingRwLock<HashMap<Uuid, CountryIdxModel>>,
    local_deletions: ParkingRwLock<HashSet<Uuid>>,
}

impl TransactionAwareCountryIdxModelCache {
    pub fn new(shared_cache: Arc<ParkingRwLock<CountryIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: ParkingRwLock::new(HashMap::new()),
            local_deletions: ParkingRwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: CountryIdxModel) {
        let primary_key = item.country_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        if self.local_additions.write().remove(primary_key).is_none() {
            self.local_deletions.write().insert(*primary_key);
        }
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        if self.local_additions.read().contains_key(primary_key) {
            return true;
        }
        if self.local_deletions.read().contains(primary_key) {
            return false;
        }
        self.shared_cache.read().contains_primary(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountryIdxModel> {
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        if self.local_deletions.read().contains(primary_key) {
            return None;
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }

    pub fn get_by_iso2(&self, key: &HeaplessString<2>) -> Option<Uuid> {
        for item in self.local_additions.read().values() {
            if item.iso2 == *key {
                return Some(item.country_id);
            }
        }

        let shared_cache = self.shared_cache.read();
        if let Some(primary_key) = shared_cache.get_by_iso2(key) {
            if self.local_deletions.read().contains(&primary_key) {
                return None;
            }
            return Some(primary_key);
        }

        None
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareCountryIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared_cache = self.shared_cache.write();
        let mut local_additions = self.local_additions.write();
        let mut local_deletions = self.local_deletions.write();

        for item in local_additions.values() {
            shared_cache.add(item.clone());
        }
        for primary_key in local_deletions.iter() {
            shared_cache.remove(primary_key);
        }

        local_additions.clear();
        local_deletions.clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_deletions.write().clear();
        Ok(())
    }
}