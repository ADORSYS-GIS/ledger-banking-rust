// FILE: banking-db-postgres/src/repository/person/locality_repository_batch_impl.rs

use crate::repository::executor::Executor;
use crate::repository::person::locality_repository_impl::LocalityRepositoryImpl;
use crate::utils::TryFromRow;
use async_trait::async_trait;
use banking_db::models::person::{LocalityIdxModel, LocalityModel};
use banking_db::repository::{
    BatchRepository, CountrySubdivisionRepository, LocalityRepository, LocalityRepositoryError,
};
use heapless::String as HeaplessString;
use sqlx::Postgres;
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

type LocalityTuple = (
    Uuid,
    Uuid,
    HeaplessString<50>,
    HeaplessString<50>,
    Option<HeaplessString<50>>,
    Option<HeaplessString<50>>,
);

type LocalityIdxTuple = (Uuid, Uuid, i64);

#[async_trait]
impl BatchRepository<Postgres, LocalityModel> for LocalityRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<LocalityModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
        if items.is_empty() {
            return Ok(Vec::new());
        }

        let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
        if self.exist_by_ids(&ids).await?.into_iter().any(|x| x) {
            return Err(Box::new(LocalityRepositoryError::DuplicateLocation(
                "One or more localities already exist".to_string(),
            )));
        }

        let subdivision_ids: HashSet<Uuid> =
            items.iter().map(|l| l.country_subdivision_id).collect();
        for id in subdivision_ids {
            if !self.country_subdivision_repository.exists_by_id(id).await? {
                return Err(Box::new(
                    LocalityRepositoryError::CountrySubdivisionNotFound(id),
                ));
            }
        }

        let mut locality_values = Vec::with_capacity(items.len());
        let mut locality_idx_values = Vec::with_capacity(items.len());

        let cache = self.locality_idx_cache.read().await;
        for item in &items {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(item.code.as_bytes());
            let code_hash = hasher.finish() as i64;

            let idx_model = LocalityIdxModel {
                locality_id: item.id,
                country_subdivision_id: item.country_subdivision_id,
                code_hash,
            };
            cache.add(idx_model.clone());

            locality_values.push((
                item.id,
                item.country_subdivision_id,
                item.code.clone(),
                item.name_l1.clone(),
                item.name_l2.clone(),
                item.name_l3.clone(),
            ));
            locality_idx_values.push((item.id, item.country_subdivision_id, code_hash));
        }

        if !locality_values.is_empty() {
            self.execute_locality_insert(locality_values).await?;
            self.execute_locality_idx_insert(locality_idx_values)
                .await?;
        }

        Ok(items)
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<LocalityModel>>, Box<dyn Error + Send + Sync>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let query = r#"SELECT * FROM locality WHERE id = ANY($1)"#;
        let rows = match &self.executor {
            Executor::Pool(pool) => sqlx::query(query).bind(ids).fetch_all(pool.as_ref()).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
            }
        };
        let mut item_map = std::collections::HashMap::new();
        for row in rows {
            let item = LocalityModel::try_from_row(&row)?;
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
        _items: Vec<LocalityModel>,
        _audit_log_id: Uuid,
    ) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    async fn delete_batch(&self, _ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

impl LocalityRepositoryImpl {
    async fn execute_locality_insert(
        &self,
        values: Vec<LocalityTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (ids, subdivision_ids, codes, names_l1, names_l2, names_l3) =
            values.into_iter().fold(
                (
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
                |mut acc, val| {
                    acc.0.push(val.0);
                    acc.1.push(val.1);
                    acc.2.push(val.2.to_string());
                    acc.3.push(val.3.to_string());
                    acc.4.push(val.4.map(|s| s.to_string()));
                    acc.5.push(val.5.map(|s| s.to_string()));
                    acc
                },
            );

        let query = r#"
            INSERT INTO locality (id, country_subdivision_id, code, name_l1, name_l2, name_l3)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::varchar[], $4::varchar[], $5::varchar[], $6::varchar[])
        "#;

        match &self.executor {
            Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(subdivision_ids)
                    .bind(codes)
                    .bind(names_l1)
                    .bind(names_l2)
                    .bind(names_l3)
                    .execute(&**pool)
                    .await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(subdivision_ids)
                    .bind(codes)
                    .bind(names_l1)
                    .bind(names_l2)
                    .bind(names_l3)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    async fn execute_locality_idx_insert(
        &self,
        values: Vec<LocalityIdxTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (locality_ids, subdivision_ids, code_hashes) = values.into_iter().fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc
            },
        );

        let query = r#"
            INSERT INTO locality_idx (locality_id, country_subdivision_id, code_hash)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::bigint[])
        "#;

        match &self.executor {
            Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(locality_ids)
                    .bind(subdivision_ids)
                    .bind(code_hashes)
                    .execute(&**pool)
                    .await?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(locality_ids)
                    .bind(subdivision_ids)
                    .bind(code_hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}