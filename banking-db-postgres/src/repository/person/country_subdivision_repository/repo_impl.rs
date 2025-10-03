use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{
    CountrySubdivisionIdxModel, CountrySubdivisionIdxModelCache, CountrySubdivisionModel,
};
use banking_db::repository::{
    CountrySubdivisionRepository,
    CountrySubdivisionResult, TransactionAware,
};
use crate::repository::executor::Executor;
use crate::repository::person::country_repository::repo_impl::CountryRepositoryImpl;
use crate::repository::person::locality_repository_impl::LocalityRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use once_cell::sync::OnceCell;
use parking_lot::RwLock as ParkingRwLock;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct CountrySubdivisionRepositoryImpl {
    pub executor: Executor,
    pub country_subdivision_idx_cache: Arc<RwLock<TransactionAwareCountrySubdivisionIdxModelCache>>,
    pub(crate) locality_repository: OnceCell<Arc<LocalityRepositoryImpl>>,
    pub country_repository: Arc<CountryRepositoryImpl>,
}

impl CountrySubdivisionRepositoryImpl {
    pub fn new(
        executor: Executor,
        country_repository: Arc<CountryRepositoryImpl>,
        country_subdivision_idx_cache: Arc<ParkingRwLock<CountrySubdivisionIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            country_subdivision_idx_cache: Arc::new(RwLock::new(
                TransactionAwareCountrySubdivisionIdxModelCache::new(country_subdivision_idx_cache),
            )),
            country_repository,
            locality_repository: OnceCell::new(),
        }
    }

    pub async fn load_all_country_subdivision_idx(
        executor: &Executor,
    ) -> Result<Vec<CountrySubdivisionIdxModel>, sqlx::Error> {
        let query = sqlx::query("SELECT * FROM country_subdivision_idx");
        let rows = match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        let mut idx_models = Vec::with_capacity(rows.len());
        for row in rows {
            idx_models
                .push(CountrySubdivisionIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(idx_models)
    }
}

#[async_trait]
impl CountrySubdivisionRepository<Postgres> for CountrySubdivisionRepositoryImpl {
    async fn save(
        &self,
        country_subdivision: CountrySubdivisionModel,
    ) -> CountrySubdivisionResult<CountrySubdivisionModel> {
        super::save::save(self, country_subdivision).await
    }

    async fn load(&self, id: Uuid) -> CountrySubdivisionResult<CountrySubdivisionModel> {
        super::load::load(self, id).await
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>> {
        super::find_by_id::find_by_id(self, id).await
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>> {
        super::find_by_country_id::find_by_country_id(self, country_id, page, page_size).await
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>> {
        super::find_by_code::find_by_code(self, country_id, code).await
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>> {
        super::find_by_ids::find_by_ids(self, ids).await
    }

    async fn exists_by_id(&self, id: Uuid) -> CountrySubdivisionResult<bool> {
        super::exists_by_id::exists_by_id(self, id).await
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> CountrySubdivisionResult<Vec<Uuid>> {
        super::find_ids_by_country_id::find_ids_by_country_id(self, country_id).await
    }
}

#[async_trait]
impl TransactionAware for CountrySubdivisionRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.country_subdivision_idx_cache
            .read()
            .await
            .on_commit()
            .await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.country_subdivision_idx_cache
            .read()
            .await
            .on_rollback()
            .await
    }
}

impl TryFromRow<PgRow> for CountrySubdivisionModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CountrySubdivisionModel {
            id: row.get("id"),
            country_id: row.get("country_id"),
            code: get_heapless_string(row, "code")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
        })
    }
}

impl TryFromRow<PgRow> for CountrySubdivisionIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(CountrySubdivisionIdxModel {
            country_subdivision_id: row.get("country_subdivision_id"),
            country_id: row.get("country_id"),
            code_hash: row.get("code_hash"),
        })
    }
}

pub struct TransactionAwareCountrySubdivisionIdxModelCache {
    shared_cache: Arc<ParkingRwLock<CountrySubdivisionIdxModelCache>>,
    local_additions: ParkingRwLock<HashMap<Uuid, CountrySubdivisionIdxModel>>,
    local_removals: ParkingRwLock<HashSet<Uuid>>,
}

impl TransactionAwareCountrySubdivisionIdxModelCache {
    pub fn new(shared_cache: Arc<ParkingRwLock<CountrySubdivisionIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: ParkingRwLock::new(HashMap::new()),
            local_removals: ParkingRwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: CountrySubdivisionIdxModel) {
        let primary_key = item.country_subdivision_id;
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        self.local_removals.write().insert(*primary_key);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        if self.local_removals.read().contains(primary_key) {
            return false;
        }
        if self.local_additions.read().contains_key(primary_key) {
            return true;
        }
        self.shared_cache.read().contains_primary(primary_key)
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<CountrySubdivisionIdxModel> {
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }

    pub fn get_by_country_id(&self, key: &Uuid) -> Option<Vec<Uuid>> {
        let shared_cache = self.shared_cache.read();
        let mut result_set: HashSet<Uuid> = shared_cache
            .get_by_country_id(key)
            .map(|v| v.iter().cloned().collect())
            .unwrap_or_default();

        for item in self.local_additions.read().values() {
            if item.country_id == *key {
                result_set.insert(item.country_subdivision_id);
            }
        }

        if result_set.is_empty() {
            None
        } else {
            Some(result_set.into_iter().collect())
        }
    }

    pub fn get_by_code_hash(&self, key: &i64) -> Option<Uuid> {
        for item in self.local_additions.read().values() {
            if item.code_hash == *key {
                return Some(item.country_subdivision_id);
            }
        }

        let shared_cache = self.shared_cache.read();
        if let Some(primary_key) = shared_cache.get_by_code_hash(key) {
            return Some(primary_key);
        }

        None
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareCountrySubdivisionIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared_cache = self.shared_cache.write();
        let mut local_additions = self.local_additions.write();

        for item in local_additions.values() {
            shared_cache.add(item.clone());
        }
        for key in self.local_removals.read().iter() {
            shared_cache.remove(key);
        }

        local_additions.clear();
        self.local_removals.write().clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_removals.write().clear();
        Ok(())
    }
}