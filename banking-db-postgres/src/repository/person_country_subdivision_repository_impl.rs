use async_trait::async_trait;
use banking_db::models::person::{
    CountrySubdivisionIdxModel, CountrySubdivisionIdxModelCache, CountrySubdivisionModel,
};
use banking_db::repository::{CountryRepository, CountrySubdivisionRepository};
use crate::repository::executor::Executor;
use crate::repository::person_country_repository_impl::CountryRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use parking_lot::RwLock;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct CountrySubdivisionRepositoryImpl {
    executor: Executor,
    country_subdivision_idx_cache: Arc<RwLock<CountrySubdivisionIdxModelCache>>,
    country_repository: Arc<CountryRepositoryImpl>,
}

impl CountrySubdivisionRepositoryImpl {
    pub fn new(
        executor: Executor,
        country_repository: Arc<CountryRepositoryImpl>,
        country_subdivision_idx_cache: Arc<RwLock<CountrySubdivisionIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            country_subdivision_idx_cache,
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
    ) -> Result<CountrySubdivisionModel, sqlx::Error> {
        if !self
            .country_repository
            .exists_by_id(country_subdivision.country_id)
            .await
            .map_err(sqlx::Error::Configuration)?
        {
            return Err(sqlx::Error::RowNotFound);
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
                query1.execute(&**pool).await?;
                query2.execute(&**pool).await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1.execute(&mut **tx).await?;
                query2.execute(&mut **tx).await?;
            }
        }

        let idx_model = CountrySubdivisionIdxModel {
            country_subdivision_id: country_subdivision.id,
            country_id: country_subdivision.country_id,
            code_hash,
        };
        self.country_subdivision_idx_cache.write().add(idx_model);

        Ok(country_subdivision)
    }

    async fn load(&self, id: Uuid) -> Result<CountrySubdivisionModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE id = $1
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

        CountrySubdivisionModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error> {
        Ok(self.country_subdivision_idx_cache.read().get_by_primary(&id))
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<CountrySubdivisionIdxModel>, sqlx::Error> {
        let mut result = Vec::new();
        if let Some(ids) = self
            .country_subdivision_idx_cache
            .read()
            .get_by_country_id(&country_id)
        {
            for id in ids {
                if let Some(idx) = self.country_subdivision_idx_cache.read().get_by_primary(id) {
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
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(code.as_bytes());
        let code_hash = hasher.finish() as i64;

        if let Some(id) = self
            .country_subdivision_idx_cache
            .read()
            .get_by_code_hash(&code_hash)
        {
            Ok(self.country_subdivision_idx_cache.read().get_by_primary(&id))
        } else {
            Ok(None)
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionIdxModel>, Box<dyn Error + Send + Sync>> {
        let mut result = Vec::new();
        for id in ids {
            if let Some(idx) = self.country_subdivision_idx_cache.read().get_by_primary(id) {
                result.push(idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.country_subdivision_idx_cache.read().contains_primary(&id))
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .country_subdivision_idx_cache
            .read()
            .get_by_country_id(&country_id)
            .cloned()
            .unwrap_or_default())
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