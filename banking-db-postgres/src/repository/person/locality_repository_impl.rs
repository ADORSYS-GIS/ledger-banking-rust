use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{LocalityIdxModel, LocalityIdxModelCache, LocalityModel};
use banking_db::repository::{
    CountrySubdivisionRepository, LocalityRepository, LocalityRepositoryError, LocalityResult,
    TransactionAware,
};
use crate::repository::executor::Executor;
use crate::repository::person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hasher;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct LocalityRepositoryImpl {
    pub(crate) executor: Executor,
    pub(crate) locality_idx_cache: Arc<TokioRwLock<TransactionAwareLocalityIdxModelCache>>,
    pub(crate) country_subdivision_repository: Arc<CountrySubdivisionRepositoryImpl>,
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
        if !self
            .country_subdivision_repository
            .exists_by_id(locality.country_subdivision_id)
            .await
            .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?
        {
            return Err(LocalityRepositoryError::CountrySubdivisionNotFound(
                locality.country_subdivision_id,
            ));
        }

        let query1 = sqlx::query(
            r#"
            INSERT INTO locality (id, country_subdivision_id, code, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(locality.id)
        .bind(locality.country_subdivision_id)
        .bind(locality.code.as_str())
        .bind(locality.name_l1.as_str())
        .bind(locality.name_l2.as_ref().map(|s| s.as_str()))
        .bind(locality.name_l3.as_ref().map(|s| s.as_str()));

        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(locality.code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let query2 = sqlx::query(
            r#"
            INSERT INTO locality_idx (locality_id, country_subdivision_id, code_hash)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(locality.id)
        .bind(locality.country_subdivision_id)
        .bind(code_hash);

        match &self.executor {
            Executor::Pool(pool) => {
                query1
                    .execute(&**pool)
                    .await
                    .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&**pool)
                    .await
                    .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
                query2
                    .execute(&mut **tx)
                    .await
                    .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?;
            }
        }

        let new_idx = LocalityIdxModel {
            locality_id: locality.id,
            country_subdivision_id: locality.country_subdivision_id,
            code_hash,
        };
        self.locality_idx_cache.read().await.add(new_idx);

        Ok(locality)
    }

    async fn load(&self, id: Uuid) -> LocalityResult<LocalityModel> {
        let query = sqlx::query(
            r#"
            SELECT * FROM locality WHERE id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) => query
                .fetch_one(&**pool)
                .await
                .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(|e| LocalityRepositoryError::RepositoryError(e.into()))?
            }
        };

        LocalityModel::try_from_row(&row)
            .map_err(LocalityRepositoryError::RepositoryError)
    }

    async fn find_by_id(&self, id: Uuid) -> LocalityResult<Option<LocalityIdxModel>> {
        Ok(self.locality_idx_cache.read().await.get_by_primary(&id))
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> LocalityResult<Vec<LocalityIdxModel>> {
        let cache = self.locality_idx_cache.read().await;
        let mut result = Vec::new();
        if let Some(ids) = cache.get_by_country_subdivision_id(&country_subdivision_id) {
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
    ) -> LocalityResult<Option<LocalityIdxModel>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let cache = self.locality_idx_cache.read().await;
        if let Some(id) = cache.get_by_code_hash(&code_hash) {
            Ok(cache.get_by_primary(&id))
        } else {
            Ok(None)
        }
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> LocalityResult<Vec<LocalityIdxModel>> {
        let cache = self.locality_idx_cache.read().await;
        let mut result = Vec::new();
        for id in ids {
            if let Some(idx) = cache.get_by_primary(id) {
                result.push(idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> LocalityResult<bool> {
        Ok(self.locality_idx_cache.read().await.contains_primary(&id))
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> LocalityResult<Vec<Uuid>> {
        let cache = self.locality_idx_cache.read().await;
        Ok(cache
            .get_by_country_subdivision_id(&country_subdivision_id)
            .unwrap_or_default())
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> LocalityResult<Vec<bool>> {
        let cache = self.locality_idx_cache.read().await;
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(cache.contains_primary(id));
        }
        Ok(result)
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
}

impl TransactionAwareLocalityIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<LocalityIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, item: LocalityIdxModel) {
        self.local_additions
            .write()
            .insert(item.locality_id, item);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocalityIdxModel> {
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }

    pub fn get_by_country_subdivision_id(&self, country_subdivision_id: &Uuid) -> Option<Vec<Uuid>> {
        let mut shared_ids = self
            .shared_cache
            .read()
            .get_by_country_subdivision_id(country_subdivision_id)
            .cloned()
            .unwrap_or_default();

        for item in self.local_additions.read().values() {
            if item.country_subdivision_id == *country_subdivision_id {
                shared_ids.push(item.locality_id);
            }
        }

        if shared_ids.is_empty() {
            None
        } else {
            shared_ids.sort();
            shared_ids.dedup();
            Some(shared_ids)
        }
    }

    pub fn get_by_code_hash(&self, code_hash: &i64) -> Option<Uuid> {
        for item in self.local_additions.read().values() {
            if item.code_hash == *code_hash {
                return Some(item.locality_id);
            }
        }
        self.shared_cache.read().get_by_code_hash(code_hash)
    }

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        self.local_additions.read().contains_key(primary_key)
            || self.shared_cache.read().contains_primary(primary_key)
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareLocalityIdxModelCache {
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