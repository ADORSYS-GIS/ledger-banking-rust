use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{LocalityIdxModel, LocalityIdxModelCache, LocalityModel};
use banking_db::repository::{
    LocalityRepository, LocalityResult,
    TransactionAware,
};
use crate::repository::executor::Executor;
use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use crate::repository::person::location_repository_impl::LocationRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use uuid::Uuid;

pub struct LocalityRepositoryImpl {
    pub(crate) executor: Executor,
    pub(crate) locality_idx_cache: Arc<TokioRwLock<TransactionAwareLocalityIdxModelCache>>,
    pub(crate) country_subdivision_repository: Arc<CountrySubdivisionRepositoryImpl>,
    pub(crate) location_repository: OnceCell<Arc<LocationRepositoryImpl>>,
}

impl LocalityRepositoryImpl {
    pub fn new(
        executor: Executor,
        country_subdivision_repository: Arc<CountrySubdivisionRepositoryImpl>,
        locality_idx_cache: Arc<RwLock<LocalityIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            locality_idx_cache: Arc::new(TokioRwLock::new(
                TransactionAwareLocalityIdxModelCache::new(locality_idx_cache),
            )),
            country_subdivision_repository,
            location_repository: OnceCell::new(),
        }
    }

    pub async fn load_all_locality_idx(
        executor: &Executor,
    ) -> Result<Vec<LocalityIdxModel>, sqlx::Error> {
        let query = sqlx::query("SELECT * FROM locality_idx");
        let rows = match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        let mut idx_models = Vec::with_capacity(rows.len());
        for row in rows {
            idx_models.push(LocalityIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(idx_models)
    }
}

#[async_trait]
impl LocalityRepository<Postgres> for LocalityRepositoryImpl {
    async fn save(&self, locality: LocalityModel) -> LocalityResult<LocalityModel> {
        crate::repository::person::locality_repository::save::save(self, locality).await
    }

    async fn load(&self, id: Uuid) -> LocalityResult<LocalityModel> {
        crate::repository::person::locality_repository::load::load(self, id).await
    }

    async fn find_by_id(&self, id: Uuid) -> LocalityResult<Option<LocalityIdxModel>> {
        crate::repository::person::locality_repository::find_by_id::find_by_id(self, id).await
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> LocalityResult<Vec<LocalityIdxModel>> {
        crate::repository::person::locality_repository::find_by_country_subdivision_id::find_by_country_subdivision_id(self, country_subdivision_id, page, page_size).await
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> LocalityResult<Option<LocalityIdxModel>> {
        crate::repository::person::locality_repository::find_by_code::find_by_code(self, country_id, code).await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> LocalityResult<Vec<LocalityIdxModel>> {
        crate::repository::person::locality_repository::find_by_ids::find_by_ids(self, ids).await
    }

    async fn exists_by_id(&self, id: Uuid) -> LocalityResult<bool> {
        crate::repository::person::locality_repository::exists_by_id::exists_by_id(self, id).await
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> LocalityResult<Vec<Uuid>> {
        crate::repository::person::locality_repository::find_ids_by_country_subdivision_id::find_ids_by_country_subdivision_id(self, country_subdivision_id).await
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> LocalityResult<Vec<bool>> {
        crate::repository::person::locality_repository::exist_by_ids::exist_by_ids(self, ids).await
    }
}

#[async_trait]
impl TransactionAware for LocalityRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.locality_idx_cache.read().await.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.locality_idx_cache.read().await.on_rollback().await
    }
}

pub struct TransactionAwareLocalityIdxModelCache {
    shared_cache: Arc<RwLock<LocalityIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, LocalityIdxModel>>,
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwareLocalityIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<LocalityIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: LocalityIdxModel) {
        let primary_key = item.locality_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        self.local_additions.write().remove(primary_key);
        self.local_deletions.write().insert(*primary_key);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocalityIdxModel> {
        if self.local_deletions.read().contains(primary_key) {
            return None;
        }
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }

    pub fn get_by_country_subdivision_id(&self, country_subdivision_id: &Uuid) -> Option<Vec<Uuid>> {
        let shared_cache = self.shared_cache.read();
        let local_additions = self.local_additions.read();
        let local_deletions = self.local_deletions.read();

        let mut results: HashSet<Uuid> = shared_cache
            .get_by_country_subdivision_id(country_subdivision_id)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect();

        for item in local_additions.values() {
            if item.country_subdivision_id == *country_subdivision_id {
                results.insert(item.locality_id);
            }
        }

        for key in local_deletions.iter() {
            results.remove(key);
        }

        if results.is_empty() {
            None
        } else {
            Some(results.into_iter().collect())
        }
    }

    pub fn get_by_code_hash(&self, code_hash: &i64) -> Option<Uuid> {
        for item in self.local_additions.read().values() {
            if item.code_hash == *code_hash {
                return Some(item.locality_id);
            }
        }
        if let Some(shared_id) = self.shared_cache.read().get_by_code_hash(code_hash) {
            if self.local_deletions.read().contains(&shared_id) {
                return None;
            }
            return Some(shared_id);
        }
        None
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        if self.local_deletions.read().contains(primary_key) {
            return false;
        }
        self.local_additions.read().contains_key(primary_key)
            || self.shared_cache.read().contains_primary(primary_key)
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareLocalityIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared_cache = self.shared_cache.write();
        let mut local_additions = self.local_additions.write();
        let mut local_deletions = self.local_deletions.write();

        for item in local_additions.values() {
            shared_cache.add(item.clone());
        }
        for key in local_deletions.iter() {
            shared_cache.remove(key);
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

impl TryFromRow<PgRow> for LocalityModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocalityModel {
            id: row.get("id"),
            country_subdivision_id: row.get("country_subdivision_id"),
            code: get_heapless_string(row, "code")?,
            name_l1: get_heapless_string(row, "name_l1")?,
            name_l2: get_optional_heapless_string(row, "name_l2")?,
            name_l3: get_optional_heapless_string(row, "name_l3")?,
        })
    }
}

impl TryFromRow<PgRow> for LocalityIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocalityIdxModel {
            locality_id: row.get("locality_id"),
            country_subdivision_id: row.get("country_subdivision_id"),
            code_hash: row.get("code_hash"),
        })
    }
}