// FILE: banking-db-postgres/src/repository/person/country_subdivision_repository_batch_impl.rs

use crate::repository::person::country_subdivision_repository_impl::CountrySubdivisionRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};
use banking_db::repository::{
    BatchRepository, CountrySubdivisionRepository, CountrySubdivisionRepositoryError,
    LocalityRepository,
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
        if _items.is_empty() {
            return Ok(Vec::new());
        }

        let ids: Vec<Uuid> = _items.iter().map(|i| i.id).collect();
        let mut missing_ids = Vec::new();
        for id in &ids {
            if !self.exists_by_id(*id).await? {
                missing_ids.push(*id);
            }
        }

        if !missing_ids.is_empty() {
            return Err(Box::new(
                CountrySubdivisionRepositoryError::ManyCountrySubdivisionsNotFound(missing_ids),
            ));
        }

        let mut country_subdivision_values = Vec::new();
        let mut country_subdivision_idx_values = Vec::new();
        let mut updated_items = Vec::new();

        let cache = self.country_subdivision_idx_cache.read().await;

        for item in _items {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(item.code.as_bytes());
            let new_code_hash = hasher.finish() as i64;

            if let Some(existing_idx) = cache.get_by_primary(&item.id) {
                if existing_idx.code_hash == new_code_hash {
                    continue;
                }

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
                    new_code_hash,
                ));

                let mut updated_idx = existing_idx.clone();
                updated_idx.code_hash = new_code_hash;
                cache.add(updated_idx);
                updated_items.push(item);
            }
        }

        if !country_subdivision_values.is_empty() {
            self.execute_country_subdivision_update(country_subdivision_values).await?;
            self.execute_country_subdivision_idx_update(country_subdivision_idx_values).await?;
        }

        Ok(updated_items)
    }

    async fn delete_batch(&self, _ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        if _ids.is_empty() {
            return Ok(0);
        }

        let locality_repo = self.locality_repository.get().unwrap();
        let mut dependent_localities = Vec::new();
        for id in _ids {
            let localities = locality_repo
                .find_by_country_subdivision_id(*id, 0, 1)
                .await?;
            if !localities.is_empty() {
                dependent_localities.push(*id);
            }
        }

        if !dependent_localities.is_empty() {
            return Err(Box::new(
                CountrySubdivisionRepositoryError::HasDependentLocalities(dependent_localities),
            ));
        }

        let cache = self.country_subdivision_idx_cache.read().await;
        for id in _ids {
            cache.remove(id);
        }

        let delete_query = r#"DELETE FROM country_subdivision WHERE id = ANY($1)"#;
        let delete_idx_query = r#"DELETE FROM country_subdivision_idx WHERE country_subdivision_id = ANY($1)"#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(delete_idx_query).bind(_ids).execute(&**pool).await?;
                sqlx::query(delete_query).bind(_ids).execute(&**pool).await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(delete_idx_query).bind(_ids).execute(&mut **tx).await?;
                sqlx::query(delete_query).bind(_ids).execute(&mut **tx).await?;
            }
        }

        Ok(_ids.len())
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

    async fn execute_country_subdivision_update(
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
            UPDATE country_subdivision SET
                country_id = u.country_id,
                code = u.code,
                name_l1 = u.name_l1,
                name_l2 = u.name_l2,
                name_l3 = u.name_l3
            FROM (SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::text[], $4::text[], $5::text[], $6::text[]))
            AS u(id, country_id, code, name_l1, name_l2, name_l3)
            WHERE country_subdivision.id = u.id
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

    async fn execute_country_subdivision_idx_update(
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
            UPDATE country_subdivision_idx SET
                country_id = u.country_id,
                code_hash = u.code_hash
            FROM (SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[]))
            AS u(country_subdivision_id, country_id, code_hash)
            WHERE country_subdivision_idx.country_subdivision_id = u.country_subdivision_id
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