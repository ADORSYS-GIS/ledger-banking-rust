// FILE: banking-db-postgres/src/repository/person/country_repository/batch_helper.rs

use crate::repository::person::country_repository_impl::CountryRepositoryImpl;
use std::error::Error;
use uuid::Uuid;

pub(crate) type CountryTuple = (
    Uuid,
    String,
    String,
    Option<String>,
    Option<String>,
);

pub(crate) type CountryIdxTuple = (
    Uuid,
    String,
);

/// Helper functions for batch operations
impl CountryRepositoryImpl {
    pub(crate) async fn execute_country_insert(
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

    pub(crate) async fn execute_country_idx_insert(
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

    pub(crate) async fn execute_country_update(
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