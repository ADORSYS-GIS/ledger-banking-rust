// FILE: banking-db-postgres/src/repository/person/country_subdivision_repository_batch_impl.rs

use crate::repository::person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};
use banking_db::repository::{
    BatchRepository, CountrySubdivisionRepository,
    CountrySubdivisionRepositoryError,
};
use sqlx::Postgres;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

type CountrySubdivisionTuple = (
    Uuid,
    Uuid,
    String,
    String,
    Option<String>,
    Option<String>,
);

type CountrySubdivisionIdxTuple = (
    Uuid,
    Uuid,
    i64,
);

/// Batch operations implementation for CountrySubdivisionRepository
#[async_trait]
impl BatchRepository<Postgres, CountrySubdivisionModel> for CountrySubdivisionRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<CountrySubdivisionModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let mut truly_existing_ids = Vec::new();
        for item in &items {
            if self.exists_by_id(item.id).await? {
                truly_existing_ids.push(item.id);
            }
        }

        if !truly_existing_ids.is_empty() {
            return Err(Box::new(
                CountrySubdivisionRepositoryError::ManyCountrySubdivisionsExist(
                    truly_existing_ids,
                ),
            ));
        }

        let cache = self.country_subdivision_idx_cache.read().await;
        for item in &items {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(item.code.as_bytes());
            let code_hash = hasher.finish() as i64;

            let idx_model = CountrySubdivisionIdxModel {
                country_subdivision_id: item.id,
                country_id: item.country_id,
                code_hash,
            };
            cache.add(idx_model);
        }

        let mut country_subdivision_values: Vec<CountrySubdivisionTuple> = Vec::new();
        let mut country_subdivision_idx_values: Vec<CountrySubdivisionIdxTuple> = Vec::new();

        for item in &items {
            let idx_model = cache.get_by_primary(&item.id).unwrap();

            country_subdivision_values.push((
                item.id,
                item.country_id,
                item.code.to_string(),
                item.name_l1.to_string(),
                item.name_l2.as_ref().map(|s| s.to_string()),
                item.name_l3.as_ref().map(|s| s.to_string()),
            ));

            country_subdivision_idx_values.push((
                item.id,
                item.country_id,
                idx_model.code_hash,
            ));
        }

        if !country_subdivision_values.is_empty() {
            self.execute_country_subdivision_insert(country_subdivision_values).await?;
            self.execute_country_subdivision_idx_insert(country_subdivision_idx_values).await?;
        }

        Ok(items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<CountrySubdivisionModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = r#"SELECT * FROM country_subdivision WHERE id = ANY($1)"#;
        let rows = match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query).bind(ids).fetch_all(&**pool).await?
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
            }
        };
        let mut item_map = std::collections::HashMap::new();
        for row in rows {
            let item = CountrySubdivisionModel::try_from_row(&row)?;
            item_map.insert(item.id, item);
        }
        let mut result = Vec::with_capacity(ids.len());
        for id in ids {
            result.push(item_map.remove(id));
        }
        Ok(result)
    }

    async fn update_batch(
        &self,
        _items: Vec<CountrySubdivisionModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    async fn delete_batch(&self, _ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

/// Helper functions for batch operations
impl CountrySubdivisionRepositoryImpl {
    async fn execute_country_subdivision_insert(
        &self,
        values: Vec<CountrySubdivisionTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut ids = Vec::new();
        let mut country_ids = Vec::new();
        let mut codes = Vec::new();
        let mut names_l1 = Vec::new();
        let mut names_l2 = Vec::new();
        let mut names_l3 = Vec::new();

        for v in values {
            ids.push(v.0);
            country_ids.push(v.1);
            codes.push(v.2);
            names_l1.push(v.3);
            names_l2.push(v.4);
            names_l3.push(v.5);
        }

        let query = r#"
            INSERT INTO country_subdivision (id, country_id, code, name_l1, name_l2, name_l3)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::text[], $4::text[], $5::text[], $6::text[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&country_ids)
                    .bind(&codes)
                    .bind(&names_l1)
                    .bind(&names_l2)
                    .bind(&names_l3)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&country_ids)
                    .bind(&codes)
                    .bind(&names_l1)
                    .bind(&names_l2)
                    .bind(&names_l3)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_country_subdivision_idx_insert(
        &self,
        values: Vec<CountrySubdivisionIdxTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut country_subdivision_ids = Vec::new();
        let mut country_ids = Vec::new();
        let mut code_hashes = Vec::new();

        for v in values {
            country_subdivision_ids.push(v.0);
            country_ids.push(v.1);
            code_hashes.push(v.2);
        }

        let query = r#"
            INSERT INTO country_subdivision_idx (country_subdivision_id, country_id, code_hash)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(&country_subdivision_ids)
                    .bind(&country_ids)
                    .bind(&code_hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(&country_subdivision_ids)
                    .bind(&country_ids)
                    .bind(&code_hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}