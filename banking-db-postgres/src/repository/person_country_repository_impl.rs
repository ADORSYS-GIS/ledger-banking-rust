use async_trait::async_trait;
use banking_db::models::person::{CountryIdxModel, CountryIdxModelCache, CountryModel};
use banking_db::repository::CountryRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use heapless::String as HeaplessString;
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct CountryRepositoryImpl {
    pool: Arc<PgPool>,
    country_idx_cache: Arc<CountryIdxModelCache>,
}

impl CountryRepositoryImpl {
    pub async fn new(pool: Arc<PgPool>) -> Self {
        let country_idx_models = Self::load_all_country_idx(&pool).await.unwrap();
        let country_idx_cache = CountryIdxModelCache::new(country_idx_models).unwrap();
        Self {
            pool,
            country_idx_cache,
        }
    }

    async fn load_all_country_idx(pool: &PgPool) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM country_idx").fetch_all(pool).await?;
        let mut idx_models = Vec::with_capacity(rows.len());
        for row in rows {
            idx_models.push(CountryIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(idx_models)
    }
}

#[async_trait]
impl CountryRepository<Postgres> for CountryRepositoryImpl {
    async fn save(&self, country: CountryModel) -> Result<CountryModel, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO country (id, iso2, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(country.id)
        .bind(country.iso2.as_str())
        .bind(country.name_l1.as_str())
        .bind(country.name_l2.as_ref().map(|s| s.as_str()))
        .bind(country.name_l3.as_ref().map(|s| s.as_str()))
        .execute(&*self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO country_idx (country_id, iso2)
            VALUES ($1, $2)
            "#,
        )
        .bind(country.id)
        .bind(country.iso2.as_str())
        .execute(&*self.pool)
        .await?;

        Ok(country)
    }

    async fn load(&self, id: Uuid) -> Result<CountryModel, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM country WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;

        CountryModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryIdxModel>, sqlx::Error> {
        Ok(self.country_idx_cache.get_by_primary(&id))
    }

    async fn find_by_iso2(
        &self,
        iso2: &str,
        _page: i32,
        _page_size: i32,
    ) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
        let mut result = Vec::new();
        let iso2_heapless = HeaplessString::<2>::from_str(iso2)
            .map_err(|_| sqlx::Error::Configuration("Invalid iso2 format".into()))?;
        if let Some(country_id) = self.country_idx_cache.get_by_iso2(&iso2_heapless) {
            if let Some(country_idx) = self.country_idx_cache.get_by_primary(&country_id) {
                result.push(country_idx);
            }
        }
        Ok(result)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
        let mut result = Vec::new();
        for id in ids {
            if let Some(country_idx) = self.country_idx_cache.get_by_primary(id) {
                result.push(country_idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.country_idx_cache.contains_primary(&id))
    }

    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let iso2_heapless = HeaplessString::<2>::from_str(iso2)
            .map_err(|_| "Invalid iso2 format".to_string())?;
        let mut result = Vec::new();
        if let Some(country_id) = self.country_idx_cache.get_by_iso2(&iso2_heapless) {
            result.push(country_id);
        }
        Ok(result)
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