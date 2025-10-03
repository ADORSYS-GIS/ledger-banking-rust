use crate::repository::person::location_repository::LocationRepositoryImpl;
use banking_db::models::person::LocationType;
use rust_decimal::Decimal;
use std::error::Error;
use uuid::Uuid;

pub type LocationTuple = (
    Uuid,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Uuid,
    Option<String>,
    Option<Decimal>,
    Option<Decimal>,
    Option<f32>,
    LocationType,
);

pub type LocationAuditTuple = (
    Uuid,
    i32,
    i64,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Uuid,
    Option<String>,
    Option<Decimal>,
    Option<Decimal>,
    Option<f32>,
    LocationType,
    Uuid,
);

impl LocationRepositoryImpl {
    pub async fn execute_location_insert(
        &self,
        values: Vec<LocationTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            ids,
            street_line1s,
            street_line2s,
            street_line3s,
            street_line4s,
            locality_ids,
            postal_codes,
            latitudes,
            longitudes,
            accuracy_meters,
            location_types,
        ) = values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
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
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc.4.push(val.4);
                acc.5.push(val.5);
                acc.6.push(val.6);
                acc.7.push(val.7);
                acc.8.push(val.8);
                acc.9.push(val.9);
                acc.10.push(val.10);
                acc
            },
        );

        let query = r#"
            INSERT INTO location (id, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type)
            SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[], $6::uuid[], $7::text[], $8::numeric[], $9::numeric[], $10::real[], $11::location_type[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn execute_location_idx_insert(
        &self,
        values: Vec<(Uuid, Uuid, i32, i64)>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (location_ids, locality_ids, versions, hashes) = values
            .into_iter()
            .fold((Vec::new(), Vec::new(), Vec::new(), Vec::new()), |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc
            });

        let query = r#"
            INSERT INTO location_idx (location_id, locality_id, version, hash)
            SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::int[], $4::bigint[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn execute_location_audit_insert(
        &self,
        values: Vec<LocationAuditTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            location_ids,
            versions,
            hashes,
            street_line1s,
            street_line2s,
            street_line3s,
            street_line4s,
            locality_ids,
            postal_codes,
            latitudes,
            longitudes,
            accuracy_meters,
            location_types,
            audit_log_ids,
        ) = values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
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
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc.4.push(val.4);
                acc.5.push(val.5);
                acc.6.push(val.6);
                acc.7.push(val.7);
                acc.8.push(val.8);
                acc.9.push(val.9);
                acc.10.push(val.10);
                acc.11.push(val.11);
                acc.12.push(val.12);
                acc.13.push(val.13);
                acc
            },
        );

        let query = r#"
            INSERT INTO location_audit (location_id, version, hash, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
            SELECT * FROM UNNEST($1::uuid[], $2::int[], $3::bigint[], $4::text[], $5::text[], $6::text[], $7::text[], $8::uuid[], $9::text[], $10::numeric[], $11::numeric[], $12::real[], $13::location_type[], $14::uuid[])
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(versions)
                    .bind(hashes)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .bind(audit_log_ids)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(versions)
                    .bind(hashes)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .bind(audit_log_ids)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
    pub async fn execute_location_update(
        &self,
        values: Vec<LocationTuple>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (
            ids,
            street_line1s,
            street_line2s,
            street_line3s,
            street_line4s,
            locality_ids,
            postal_codes,
            latitudes,
            longitudes,
            accuracy_meters,
            location_types,
        ) = values.into_iter().fold(
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
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
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc.4.push(val.4);
                acc.5.push(val.5);
                acc.6.push(val.6);
                acc.7.push(val.7);
                acc.8.push(val.8);
                acc.9.push(val.9);
                acc.10.push(val.10);
                acc
            },
        );

        let query = r#"
            UPDATE location SET
                street_line1 = u.street_line1,
                street_line2 = u.street_line2,
                street_line3 = u.street_line3,
                street_line4 = u.street_line4,
                locality_id = u.locality_id,
                postal_code = u.postal_code,
                latitude = u.latitude,
                longitude = u.longitude,
                accuracy_meters = u.accuracy_meters,
                location_type = u.location_type
            FROM (
                SELECT * FROM UNNEST(
                    $1::uuid[], $2::text[], $3::text[], $4::text[], $5::text[], $6::uuid[],
                    $7::text[], $8::numeric[], $9::numeric[], $10::real[], $11::location_type[]
                )
            ) AS u(id, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type)
            WHERE location.id = u.id
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(ids)
                    .bind(street_line1s)
                    .bind(street_line2s)
                    .bind(street_line3s)
                    .bind(street_line4s)
                    .bind(locality_ids)
                    .bind(postal_codes)
                    .bind(latitudes)
                    .bind(longitudes)
                    .bind(accuracy_meters)
                    .bind(location_types)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }

    pub async fn execute_location_idx_update(
        &self,
        values: Vec<(Uuid, Uuid, i32, i64)>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let (location_ids, locality_ids, versions, hashes) = values
            .into_iter()
            .fold((Vec::new(), Vec::new(), Vec::new(), Vec::new()), |mut acc, val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc.2.push(val.2);
                acc.3.push(val.3);
                acc
            });

        let query = r#"
            UPDATE location_idx SET
                locality_id = u.locality_id,
                version = u.version,
                hash = u.hash
            FROM (
                SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::int[], $4::bigint[])
            ) AS u(location_id, locality_id, version, hash)
            WHERE location_idx.location_id = u.location_id
        "#;

        match &self.executor {
            crate::repository::executor::Executor::Pool(pool) => {
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&**pool)
                    .await?;
            }
            crate::repository::executor::Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                sqlx::query(query)
                    .bind(location_ids)
                    .bind(locality_ids)
                    .bind(versions)
                    .bind(hashes)
                    .execute(&mut **tx)
                    .await?;
            }
        }
        Ok(())
    }
}