use async_trait::async_trait;
use banking_db::models::person::{CountryIdxModel, CountryIdxModelCache, CountryModel};
use banking_db::repository::CountryRepository;
use crate::repository::executor::Executor;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use heapless::String as HeaplessString;
use parking_lot::RwLock;
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct CountryRepositoryImpl {
    executor: Executor,
    country_idx_cache: Arc<RwLock<CountryIdxModelCache>>,
}

impl CountryRepositoryImpl {
    pub async fn new(executor: Executor) -> Self {
        let country_idx_models = Self::load_all_country_idx(&executor).await.unwrap();
        let country_idx_cache =
            Arc::new(RwLock::new(CountryIdxModelCache::new(country_idx_models).unwrap()));
        Self {
            executor,
            country_idx_cache,
        }
    }

    async fn load_all_country_idx(
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
    async fn save(&self, country: CountryModel) -> Result<CountryModel, sqlx::Error> {
        let id_to_load = {
            let cache = self.country_idx_cache.read();
            if cache.contains_primary(&country.id) {
                Some(country.id)
            } else {
                cache.get_by_iso2(&country.iso2)
            }
        };

        if let Some(id) = id_to_load {
            return self.load(id).await;
        }

        let query1 = sqlx::query(
            r#"
            INSERT INTO country (id, iso2, name_l1, name_l2, name_l3)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(country.id)
        .bind(country.iso2.as_str())
        .bind(country.name_l1.as_str())
        .bind(country.name_l2.as_ref().map(|s| s.as_str()))
        .bind(country.name_l3.as_ref().map(|s| s.as_str()));

        let query2 = sqlx::query(
            r#"
            INSERT INTO country_idx (country_id, iso2)
            VALUES ($1, $2)
            "#,
        )
        .bind(country.id)
        .bind(country.iso2.as_str());

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

        let new_idx_model = CountryIdxModel {
            country_id: country.id,
            iso2: country.iso2.clone(),
        };
        self.country_idx_cache.write().add(new_idx_model);

        Ok(country)
    }

    async fn load(&self, id: Uuid) -> Result<CountryModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM country WHERE id = $1
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

        CountryModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryIdxModel>, sqlx::Error> {
        let cache = self.country_idx_cache.read();
        Ok(cache.get_by_primary(&id))
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
        let cache = self.country_idx_cache.read();
        if let Some(country_id) = cache.get_by_iso2(&iso2_heapless) {
            if let Some(country_idx) = cache.get_by_primary(&country_id) {
                result.push(country_idx);
            }
        }
        Ok(result)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryIdxModel>, sqlx::Error> {
        let mut result = Vec::new();
        let cache = self.country_idx_cache.read();
        for id in ids {
            if let Some(country_idx) = cache.get_by_primary(id) {
                result.push(country_idx);
            }
        }
        Ok(result)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.country_idx_cache.read().contains_primary(&id))
    }

    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let iso2_heapless = HeaplessString::<2>::from_str(iso2)
            .map_err(|_| "Invalid iso2 format".to_string())?;
        let mut result = Vec::new();
        if let Some(country_id) = self.country_idx_cache.read().get_by_iso2(&iso2_heapless) {
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