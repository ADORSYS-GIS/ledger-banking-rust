use async_trait::async_trait;
use banking_db::models::person::CountryModel;
use banking_db::repository::CountryRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

pub struct CountryRepositoryImpl {
    pool: Arc<PgPool>,
}

impl CountryRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
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

        Ok(country)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CountryModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM country WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                CountryModel::try_from_row(&row).map_err(sqlx::Error::Decode)?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_iso2(
        &self,
        iso2: &str,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountryModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country WHERE iso2 = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(iso2)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut countries = Vec::new();
        for row in rows {
            countries.push(CountryModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(countries)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<CountryModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country WHERE id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut countries = Vec::new();
        for row in rows {
            countries.push(CountryModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(countries)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM country WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_iso2(
        &self,
        iso2: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM country WHERE iso2 = $1
            "#,
        )
        .bind(iso2)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
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