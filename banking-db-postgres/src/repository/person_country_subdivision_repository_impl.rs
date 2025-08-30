use async_trait::async_trait;
use banking_db::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};
use banking_db::repository::CountrySubdivisionRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use uuid::Uuid;

pub struct CountrySubdivisionRepositoryImpl {
    pool: Arc<PgPool>,
}

impl CountrySubdivisionRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
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
        let row = sqlx::query(
            r#"
            SELECT * FROM country_subdivision_idx WHERE country_subdivision_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                CountrySubdivisionIdxModel::try_from_row(&row)
                    .map_err(sqlx::Error::Decode)?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_country_id(
        &self,
        country_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<CountrySubdivisionIdxModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country_subdivision_idx WHERE country_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(country_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut subdivisions = Vec::new();
        for row in rows {
            subdivisions.push(
                CountrySubdivisionIdxModel::try_from_row(&row)
                    .map_err(sqlx::Error::Decode)?,
            );
        }
        Ok(subdivisions)
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<CountrySubdivisionIdxModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT csi.*
            FROM country_subdivision_idx csi
            JOIN country_subdivision cs ON csi.country_subdivision_id = cs.id
            WHERE cs.country_id = $1 AND cs.code = $2
            "#,
        )
        .bind(country_id)
        .bind(code)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                CountrySubdivisionIdxModel::try_from_row(&row)
                    .map_err(sqlx::Error::Decode)?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<CountrySubdivisionIdxModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM country_subdivision_idx WHERE country_subdivision_id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut subdivisions = Vec::new();
        for row in rows {
            subdivisions.push(CountrySubdivisionIdxModel::try_from_row(&row)?);
        }
        Ok(subdivisions)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM country_subdivision WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_country_id(
        &self,
        country_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM country_subdivision WHERE country_id = $1
            "#,
        )
        .bind(country_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
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