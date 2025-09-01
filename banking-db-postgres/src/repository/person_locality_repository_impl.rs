use async_trait::async_trait;
use banking_db::models::person::{LocalityIdxModel, LocalityIdxModelCache, LocalityModel};
use banking_db::repository::{CountrySubdivisionRepository, LocalityRepository};
use crate::repository::executor::Executor;
use crate::repository::person_country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::{Arc, RwLock};
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct LocalityRepositoryImpl {
    executor: Executor,
    locality_idx_cache: Arc<RwLock<LocalityIdxModelCache>>,
    country_subdivision_repository: Arc<CountrySubdivisionRepositoryImpl>,
}

impl LocalityRepositoryImpl {
    pub async fn new(
        executor: Executor,
        country_subdivision_repository: Arc<CountrySubdivisionRepositoryImpl>,
    ) -> Self {
        let locality_idx_models = Self::load_all_locality_idx(&executor).await.unwrap();
        let locality_idx_cache = Arc::new(RwLock::new(
            LocalityIdxModelCache::new(locality_idx_models).unwrap(),
        ));
        Self {
            executor,
            locality_idx_cache,
            country_subdivision_repository,
        }
    }

    async fn load_all_locality_idx(
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
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error> {
        if !self
            .country_subdivision_repository
            .exists_by_id(locality.country_subdivision_id)
            .await
            .map_err(sqlx::Error::Configuration)?
        {
            return Err(sqlx::Error::RowNotFound);
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
                query1.execute(&**pool).await?;
                query2.execute(&**pool).await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1.execute(&mut **tx).await?;
                query2.execute(&mut **tx).await?;
            }
        }

        let new_idx = LocalityIdxModel {
            locality_id: locality.id,
            country_subdivision_id: locality.country_subdivision_id,
            code_hash,
        };
        self.locality_idx_cache.write().unwrap().add(new_idx);

        Ok(locality)
    }

    async fn load(&self, id: Uuid) -> Result<LocalityModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM locality WHERE id = $1
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

        LocalityModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityIdxModel>, sqlx::Error> {
        Ok(self.locality_idx_cache.read().unwrap().get_by_primary(&id))
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<LocalityIdxModel>, sqlx::Error> {
        let cache = self.locality_idx_cache.read().unwrap();
        let mut result = Vec::new();
        if let Some(ids) = cache.get_by_country_subdivision_id(&country_subdivision_id) {
            for id in ids {
                if let Some(idx) = cache.get_by_primary(id) {
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
    ) -> Result<Option<LocalityIdxModel>, sqlx::Error> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let cache = self.locality_idx_cache.read().unwrap();
        if let Some(id) = cache.get_by_code_hash(&code_hash) {
            Ok(cache.get_by_primary(&id))
        } else {
            Ok(None)
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocalityIdxModel>, Box<dyn Error + Send + Sync>> {
        let cache = self.locality_idx_cache.read().unwrap();
        let mut result = Vec::new();
        for id in ids {
            if let Some(idx) = cache.get_by_primary(id) {
                result.push(idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.locality_idx_cache.read().unwrap().contains_primary(&id))
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let cache = self.locality_idx_cache.read().unwrap();
        Ok(cache
            .get_by_country_subdivision_id(&country_subdivision_id)
            .cloned()
            .unwrap_or_default())
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