use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{
    CountrySubdivisionIdxModel, CountrySubdivisionIdxModelCache, CountrySubdivisionModel,
};
use banking_db::repository::{
    CountryRepository, CountrySubdivisionRepository, CountrySubdivisionRepositoryError,
    CountrySubdivisionResult, TransactionAware,
};
use crate::repository::executor::Executor;
use crate::repository::person::country_repository_impl::CountryRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use parking_lot::RwLock as ParkingRwLock;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use tokio::sync::RwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct CountrySubdivisionRepositoryImpl {
    executor: Executor,
    country_subdivision_idx_cache: Arc<RwLock<TransactionAwareCountrySubdivisionIdxModelCache>>,
    country_repository: Arc<CountryRepositoryImpl>,
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
        if !self
            .country_repository
            .exists_by_id(country_subdivision.country_id)
            .await
            .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?
        {
            return Err(CountrySubdivisionRepositoryError::CountryNotFound(
                country_subdivision.country_id,
            ));
        }

        let query1 = sqlx::query(
            r#"
            INSERT INTO country_subdivision (id, country_id, code, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(country_subdivision.id)
        .bind(country_subdivision.country_id)
        .bind(country_subdivision.code.as_str())
        .bind(country_subdivision.name_l1.as_str())
        .bind(
            country_subdivision
                .name_l2
                .as_ref()
                .map(|s| s.as_str()),
        )
        .bind(
            country_subdivision
                .name_l3
                .as_ref()
                .map(|s| s.as_str()),
        );

        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(country_subdivision.code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let query2 = sqlx::query(
            r#"
            INSERT INTO country_subdivision_idx (country_subdivision_id, country_id, code_hash)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(country_subdivision.id)
        .bind(country_subdivision.country_id)
        .bind(code_hash);

        match &self.executor {
            Executor::Pool(pool) => {
                query1
                    .execute(&**pool)
                    .await
                    .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&**pool)
                    .await
                    .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?;
            }
        }

        let idx_model = CountrySubdivisionIdxModel {
            country_subdivision_id: country_subdivision.id,
            country_id: country_subdivision.country_id,
            code_hash,
        };
        self.country_subdivision_idx_cache
            .read()
            .await
            .add(idx_model);

        Ok(country_subdivision)
    }

    async fn load(&self, id: Uuid) -> CountrySubdivisionResult<CountrySubdivisionModel> {
        let query = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) => query
                .fetch_one(&**pool)
                .await
                .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(|e| CountrySubdivisionRepositoryError::RepositoryError(e.into()))?
            }
        };

        CountrySubdivisionModel::try_from_row(&row)
            .map_err(CountrySubdivisionRepositoryError::RepositoryError)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>> {
        Ok(self
            .country_subdivision_idx_cache
            .read()
            .await
            .get_by_primary(&id))
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>> {
        let mut result = Vec::new();
        let cache = self.country_subdivision_idx_cache.read().await;
        if let Some(ids) = cache.get_by_country_id(&country_id) {
            for id in ids {
                if let Some(idx) = cache.get_by_primary(&id) {
                    result.push(idx);
                }
            }
        }
        Ok(result)
    }

    async fn find_by_code(
        &self,
        _country_id: Uuid,
        code: &str,
    ) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let cache = self.country_subdivision_idx_cache.read().await;
        if let Some(id) = cache.get_by_code_hash(&code_hash) {
            Ok(cache.get_by_primary(&id))
        } else {
            Ok(None)
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> CountrySubdivisionResult<Vec<CountrySubdivisionIdxModel>> {
        let mut result = Vec::new();
        let cache = self.country_subdivision_idx_cache.read().await;
        for id in ids {
            if let Some(idx) = cache.get_by_primary(id) {
                result.push(idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> CountrySubdivisionResult<bool> {
        Ok(self
            .country_subdivision_idx_cache
            .read()
            .await
            .contains_primary(&id))
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> CountrySubdivisionResult<Vec<Uuid>> {
        Ok(self
            .country_subdivision_idx_cache
            .read()
            .await
            .get_by_country_id(&country_id)
            .unwrap_or_default())
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
}

impl TransactionAwareCountrySubdivisionIdxModelCache {
    pub fn new(shared_cache: Arc<ParkingRwLock<CountrySubdivisionIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: ParkingRwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, item: CountrySubdivisionIdxModel) {
        let primary_key = item.country_subdivision_id;
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
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

        local_additions.clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        Ok(())
    }
}