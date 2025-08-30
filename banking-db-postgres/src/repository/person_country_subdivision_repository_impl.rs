use async_trait::async_trait;
use banking_db::models::person::{
    CountrySubdivisionIdxModel, CountrySubdivisionIdxModelCache, CountrySubdivisionModel,
};
use banking_db::repository::CountrySubdivisionRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct CountrySubdivisionRepositoryImpl {
    pool: Arc<PgPool>,
    country_subdivision_idx_cache: Arc<CountrySubdivisionIdxModelCache>,
}

impl CountrySubdivisionRepositoryImpl {
    pub async fn new(pool: Arc<PgPool>) -> Self {
        let country_subdivision_idx_models =
            Self::load_all_country_subdivision_idx(&pool).await.unwrap();
        let country_subdivision_idx_cache =
            CountrySubdivisionIdxModelCache::new(country_subdivision_idx_models).unwrap();
        Self {
            pool,
            country_subdivision_idx_cache,
        }
    }

    async fn load_all_country_subdivision_idx(
        pool: &PgPool,
    ) -> Result<Vec<CountrySubdivisionIdxModel>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM country_subdivision_idx")
            .fetch_all(pool)
            .await?;
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
        sqlx::query(
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
        )
        .execute(&*self.pool)
        .await?;

        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(country_subdivision.code.as_bytes());
        let code_hash = hasher.finish() as i64;

        sqlx::query(
            r#"
            INSERT INTO country_subdivision_idx (country_subdivision_id, country_id, code_hash)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(country_subdivision.id)
        .bind(country_subdivision.country_id)
        .bind(code_hash)
        .execute(&*self.pool)
        .await?;

        Ok(country_subdivision)
    }

    async fn load(&self, id: Uuid) -> Result<CountrySubdivisionModel, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM country_subdivision WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;

        CountrySubdivisionModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error> {
        Ok(self.country_subdivision_idx_cache.get_by_primary(&id))
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
            .get_by_country_id(&country_id)
        {
            for id in ids {
                if let Some(idx) = self.country_subdivision_idx_cache.get_by_primary(id) {
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
            .get_by_code_hash(&code_hash)
        {
            Ok(self.country_subdivision_idx_cache.get_by_primary(&id))
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
            if let Some(idx) = self.country_subdivision_idx_cache.get_by_primary(id) {
                result.push(idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.country_subdivision_idx_cache.contains_primary(&id))
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .country_subdivision_idx_cache
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