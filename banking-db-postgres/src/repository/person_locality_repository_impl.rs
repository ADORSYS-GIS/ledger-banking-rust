use async_trait::async_trait;
use banking_db::models::person::{LocalityIdxModel, LocalityModel};
use banking_db::repository::LocalityRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use uuid::Uuid;

pub struct LocalityRepositoryImpl {
    pool: Arc<PgPool>,
}

impl LocalityRepositoryImpl {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LocalityRepository<Postgres> for LocalityRepositoryImpl {
    async fn save(&self, locality: LocalityModel) -> Result<LocalityModel, sqlx::Error> {
        sqlx::query(
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
        .bind(locality.name_l3.as_ref().map(|s| s.as_str()))
        .execute(&*self.pool)
        .await?;

        let mut hasher = twox_hash::XxHash64::with_seed(0);
        hasher.write(locality.code.as_bytes());
        let code_hash = hasher.finish() as i64;

        sqlx::query(
            r#"
            INSERT INTO locality_idx (locality_id, country_subdivision_id, code_hash)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(locality.id)
        .bind(locality.country_subdivision_id)
        .bind(code_hash)
        .execute(&*self.pool)
        .await?;

        Ok(locality)
    }

    async fn load(&self, id: Uuid) -> Result<LocalityModel, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM locality WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;

        LocalityModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocalityIdxModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM locality_idx WHERE locality_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                LocalityIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocalityIdxModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM locality_idx WHERE country_subdivision_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(country_subdivision_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut localities = Vec::new();
        for row in rows {
            localities
                .push(LocalityIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(localities)
    }

    async fn find_by_code(
        &self,
        country_id: Uuid,
        code: &str,
    ) -> Result<Option<LocalityIdxModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT li.*
            FROM locality_idx li
            JOIN locality l ON li.locality_id = l.id
            JOIN country_subdivision cs ON l.country_subdivision_id = cs.id
            WHERE cs.country_id = $1 AND l.code = $2
            "#,
        )
        .bind(country_id)
        .bind(code)
        .fetch_optional(&*self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(
                LocalityIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?,
            )),
            None => Ok(None),
        }
    }

    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<LocalityIdxModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM locality_idx WHERE locality_id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

        let mut localities = Vec::new();
        for row in rows {
            localities.push(LocalityIdxModel::try_from_row(&row)?);
        }
        Ok(localities)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM locality WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_country_subdivision_id(
        &self,
        country_subdivision_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM locality WHERE country_subdivision_id = $1
            "#,
        )
        .bind(country_subdivision_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
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