// FILE: banking-db-postgres/src/repository/person/country_repository_batch_impl.rs

use crate::repository::person::country_repository_impl::CountryRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{CountryIdxModel, CountryModel};
use banking_db::repository::{
    BatchRepository, CountryRepository,
    CountryRepositoryError,
};
use sqlx::Postgres;
use std::error::Error;
use uuid::Uuid;

type CountryTuple = (
    Uuid,
    String,
    String,
    Option<String>,
    Option<String>,
);

type CountryIdxTuple = (
    Uuid,
    String,
);

/// Batch operations implementation for CountryRepository
#[async_trait]
impl BatchRepository<Postgres, CountryModel> for CountryRepositoryImpl {
    /// Save multiple countries in a single transaction
    async fn create_batch(
        &self,
        items: Vec<CountryModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        let existing_check = self.exist_by_ids(&ids).await?;
        let truly_existing_ids: Vec<Uuid> = existing_check
            .into_iter()
            .filter_map(|(id, exists)| if exists { Some(id) } else { None })
            .collect();

        if !truly_existing_ids.is_empty() {
            return Err(Box::new(CountryRepositoryError::ManyCountriesExist(
                truly_existing_ids,
            )));
        }

        let cache = self.country_idx_cache.read().await;
        for item in &items {
            let idx_model = CountryIdxModel {
                country_id: item.id,
                iso2: item.iso2.clone(),
            };
            cache.add(idx_model);
        }

        let mut country_values = Vec::new();
        let mut country_idx_values = Vec::new();
        let mut saved_items = Vec::new();

        for item in items {
            country_values.push((
                item.id,
                item.iso2.to_string(),
                item.name_l1.to_string(),
                item.name_l2.as_ref().map(|s| s.to_string()),
                item.name_l3.as_ref().map(|s| s.to_string()),
            ));

            country_idx_values.push((
                item.id,
                item.iso2.to_string(),
            ));
            saved_items.push(item);
        }

        if !country_values.is_empty() {
            self.execute_country_insert(country_values).await?;
            self.execute_country_idx_insert(country_idx_values).await?;
        }

        Ok(saved_items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<CountryModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = r#"SELECT * FROM country WHERE id = ANY($1)"#;
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
            let item = CountryModel::try_from_row(&row)?;
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
        items: Vec<CountryModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<CountryModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        let existing_check = self.exist_by_ids(&ids).await?;
        let missing_ids: Vec<Uuid> = existing_check
            .into_iter()
            .filter_map(|(id, exists)| if !exists { Some(id) } else { None })
            .collect();

        if !missing_ids.is_empty() {
            return Err(Box::new(CountryRepositoryError::ManyCountriesNotFound(
                missing_ids,
            )));
        }

        let mut country_values = Vec::new();
        let mut updated_items = Vec::new();

        for item in items {
            country_values.push((
                item.id,
                item.iso2.to_string(),
                item.name_l1.to_string(),
                item.name_l2.as_ref().map(|s| s.to_string()),
                item.name_l3.as_ref().map(|s| s.to_string()),
            ));
            updated_items.push(item);
        }

        if !country_values.is_empty() {
            self.execute_country_update(country_values).await?;
        }

        Ok(updated_items)
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(0);
        }

        let existings = self.find_by_ids(ids).await?;
        let existing_ids: Vec<Uuid> = existings.iter().map(|p| p.country_id).collect();

        {
            let cache = self.country_idx_cache.write().await;
            for id in &existing_ids {
                cache.remove(id);
            }
        }

        let delete_query = r#"DELETE FROM country WHERE id = ANY($1)"#;
        let delete_idx_query = r#"DELETE FROM country_idx WHERE country_id = ANY($1)"#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(delete_idx_query).bind(&existing_ids).execute(&**pool).await?;
                sqlx::query(delete_query).bind(&existing_ids).execute(&**pool).await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(delete_idx_query).bind(&existing_ids).execute(&mut **tx).await?;
                sqlx::query(delete_query).bind(&existing_ids).execute(&mut **tx).await?;
            }
        }

        Ok(existing_ids.len())
    }
}

/// Helper functions for batch operations
impl CountryRepositoryImpl {
    async fn execute_country_insert(
        &self,
        values: Vec<CountryTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = r#"
            INSERT INTO country (id, iso2, name_l1, name_l2, name_l3)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[])
        "#;
        
        let (ids, iso2s, name_l1s, name_l2s, name_l3s) =
            values.into_iter().fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
                |mut acc, val| {
                    acc.0.push(val.0);
                    acc.1.push(val.1);
                    acc.2.push(val.2);
                    acc.3.push(val.3);
                    acc.4.push(val.4);
                    acc
                },
            );

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&iso2s)
                    .bind(&name_l1s)
                    .bind(&name_l2s)
                    .bind(&name_l3s)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&iso2s)
                    .bind(&name_l1s)
                    .bind(&name_l2s)
                    .bind(&name_l3s)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_country_idx_insert(
        &self,
        values: Vec<CountryIdxTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = r#"
            INSERT INTO country_idx (country_id, iso2)
            SELECT * FROM UNNEST($1::uuid[], $2::text[])
        "#;

        let (ids, iso2s) =
            values.into_iter().fold(
                (Vec::new(), Vec::new()),
                |mut acc, val| {
                    acc.0.push(val.0);
                    acc.1.push(val.1);
                    acc
                },
            );

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&iso2s)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&iso2s)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_country_update(
        &self,
        values: Vec<CountryTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let query = r#"
            UPDATE country SET
                iso2 = u.iso2,
                name_l1 = u.name_l1,
                name_l2 = u.name_l2,
                name_l3 = u.name_l3
            FROM (SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[]))
            AS u(id, iso2, name_l1, name_l2, name_l3)
            WHERE country.id = u.id
        "#;

        let (ids, iso2s, name_l1s, name_l2s, name_l3s) =
            values.into_iter().fold(
                (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()),
                |mut acc, val| {
                    acc.0.push(val.0);
                    acc.1.push(val.1);
                    acc.2.push(val.2);
                    acc.3.push(val.3);
                    acc.4.push(val.4);
                    acc
                },
            );

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&iso2s)
                    .bind(&name_l1s)
                    .bind(&name_l2s)
                    .bind(&name_l3s)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(&ids)
                    .bind(&iso2s)
                    .bind(&name_l1s)
                    .bind(&name_l2s)
                    .bind(&name_l3s)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}