use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{
    LocationAuditModel, LocationIdxModel, LocationIdxModelCache, LocationModel, LocationType,
};
use banking_db::repository::{LocalityRepository, LocationRepository, TransactionAware};
use crate::repository::executor::Executor;
use crate::repository::person_locality_repository_impl::LocalityRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use parking_lot::RwLock;
use std::sync::Arc;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct LocationRepositoryImpl {
    executor: Executor,
    location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
    locality_repository: Arc<LocalityRepositoryImpl>,
}

impl LocationRepositoryImpl {
    pub fn new(
        executor: Executor,
        locality_repository: Arc<LocalityRepositoryImpl>,
        location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            location_idx_cache,
            locality_repository,
        }
    }

    pub async fn load_all_location_idx(
        executor: &Executor,
    ) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        let query = sqlx::query_as::<_, LocationIdxModel>("SELECT * FROM location_idx");
        match executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await
            }
        }
    }
}

#[async_trait]
impl LocationRepository<Postgres> for LocationRepositoryImpl {
    async fn save(
        &self,
        location: LocationModel,
        audit_log_id: Uuid,
    ) -> Result<LocationModel, sqlx::Error> {
        if !self
            .locality_repository
            .exists_by_id(location.locality_id)
            .await
            .map_err(sqlx::Error::Configuration)?
        {
            return Err(sqlx::Error::RowNotFound);
        }

        let mut hasher = XxHash64::with_seed(0);
        let mut location_cbor = Vec::new();
        ciborium::ser::into_writer(&location, &mut location_cbor).unwrap();
        hasher.write(&location_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.location_idx_cache.read();
            cache_read_guard.get_by_primary(&location.id)
        };

        if let Some(existing_idx) = maybe_existing_idx {
            // UPDATE
            if existing_idx.hash == new_hash {
                return Ok(location); // No changes
            }

            let new_version = existing_idx.version + 1;

            let audit_model = LocationAuditModel {
                location_id: location.id,
                version: new_version,
                hash: new_hash,
                street_line1: location.street_line1.clone(),
                street_line2: location.street_line2.clone(),
                street_line3: location.street_line3.clone(),
                street_line4: location.street_line4.clone(),
                locality_id: location.locality_id,
                postal_code: location.postal_code.clone(),
                latitude: location.latitude,
                longitude: location.longitude,
                accuracy_meters: location.accuracy_meters,
                location_type: location.location_type,
                audit_log_id,
            };

            let query1 = sqlx::query(
                r#"
                INSERT INTO location_audit (location_id, version, hash, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                "#,
            )
            .bind(audit_model.location_id)
            .bind(audit_model.version)
            .bind(audit_model.hash)
            .bind(audit_model.street_line1.as_str())
            .bind(audit_model.street_line2.as_ref().map(|s| s.as_str()))
            .bind(audit_model.street_line3.as_ref().map(|s| s.as_str()))
            .bind(audit_model.street_line4.as_ref().map(|s| s.as_str()))
            .bind(audit_model.locality_id)
            .bind(audit_model.postal_code.as_ref().map(|s| s.as_str()))
            .bind(audit_model.latitude)
            .bind(audit_model.longitude)
            .bind(audit_model.accuracy_meters)
            .bind(audit_model.location_type)
            .bind(audit_model.audit_log_id);

            let query2 = sqlx::query(
                r#"
                UPDATE location SET
                    street_line1 = $2, street_line2 = $3, street_line3 = $4, street_line4 = $5,
                    locality_id = $6, postal_code = $7, latitude = $8, longitude = $9,
                    accuracy_meters = $10, location_type = $11::location_type
                WHERE id = $1
                "#,
            )
            .bind(location.id)
            .bind(location.street_line1.as_str())
            .bind(location.street_line2.as_ref().map(|s| s.as_str()))
            .bind(location.street_line3.as_ref().map(|s| s.as_str()))
            .bind(location.street_line4.as_ref().map(|s| s.as_str()))
            .bind(location.locality_id)
            .bind(location.postal_code.as_ref().map(|s| s.as_str()))
            .bind(location.latitude)
            .bind(location.longitude)
            .bind(location.accuracy_meters)
            .bind(location.location_type);

            let query3 = sqlx::query(
                r#"
                UPDATE location_idx SET
                    version = $2,
                    hash = $3
                WHERE location_id = $1
                "#,
            )
            .bind(location.id)
            .bind(new_version)
            .bind(new_hash);

            match &self.executor {
                Executor::Pool(pool) => {
                    query1.execute(&**pool).await?;
                    query2.execute(&**pool).await?;
                    query3.execute(&**pool).await?;
                }
                Executor::Tx(tx) => {
                    let mut tx = tx.lock().await;
                    query1.execute(&mut **tx).await?;
                    query2.execute(&mut **tx).await?;
                    query3.execute(&mut **tx).await?;
                }
            }

            let new_idx = LocationIdxModel {
                location_id: location.id,
                locality_id: location.locality_id,
                version: new_version,
                hash: new_hash,
            };
            self.location_idx_cache.write().update(new_idx);
        } else {
            // INSERT
            let version = 0;
            let audit_model = LocationAuditModel {
                location_id: location.id,
                version,
                hash: new_hash,
                street_line1: location.street_line1.clone(),
                street_line2: location.street_line2.clone(),
                street_line3: location.street_line3.clone(),
                street_line4: location.street_line4.clone(),
                locality_id: location.locality_id,
                postal_code: location.postal_code.clone(),
                latitude: location.latitude,
                longitude: location.longitude,
                accuracy_meters: location.accuracy_meters,
                location_type: location.location_type,
                audit_log_id,
            };

            let query1 = sqlx::query(
                r#"
                INSERT INTO location_audit (location_id, version, hash, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                "#,
            )
            .bind(audit_model.location_id)
            .bind(audit_model.version)
            .bind(audit_model.hash)
            .bind(audit_model.street_line1.as_str())
            .bind(audit_model.street_line2.as_ref().map(|s| s.as_str()))
            .bind(audit_model.street_line3.as_ref().map(|s| s.as_str()))
            .bind(audit_model.street_line4.as_ref().map(|s| s.as_str()))
            .bind(audit_model.locality_id)
            .bind(audit_model.postal_code.as_ref().map(|s| s.as_str()))
            .bind(audit_model.latitude)
            .bind(audit_model.longitude)
            .bind(audit_model.accuracy_meters)
            .bind(audit_model.location_type)
            .bind(audit_model.audit_log_id);

            let query2 = sqlx::query(
                r#"
                INSERT INTO location (id, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                "#,
            )
            .bind(location.id)
            .bind(location.street_line1.as_str())
            .bind(location.street_line2.as_ref().map(|s| s.as_str()))
            .bind(location.street_line3.as_ref().map(|s| s.as_str()))
            .bind(location.street_line4.as_ref().map(|s| s.as_str()))
            .bind(location.locality_id)
            .bind(location.postal_code.as_ref().map(|s| s.as_str()))
            .bind(location.latitude)
            .bind(location.longitude)
            .bind(location.accuracy_meters)
            .bind(location.location_type);

            let query3 = sqlx::query(
                r#"
                INSERT INTO location_idx (location_id, locality_id, version, hash)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(location.id)
            .bind(location.locality_id)
            .bind(version)
            .bind(new_hash);

            match &self.executor {
                Executor::Pool(pool) => {
                    query1.execute(&**pool).await?;
                    query2.execute(&**pool).await?;
                    query3.execute(&**pool).await?;
                }
                Executor::Tx(tx) => {
                    let mut tx = tx.lock().await;
                    query1.execute(&mut **tx).await?;
                    query2.execute(&mut **tx).await?;
                    query3.execute(&mut **tx).await?;
                }
            }

            let new_idx = LocationIdxModel {
                location_id: location.id,
                locality_id: location.locality_id,
                version,
                hash: new_hash,
            };
            self.location_idx_cache.write().add(new_idx);
        }

        Ok(location)
    }

    async fn load(&self, id: Uuid) -> Result<LocationModel, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM location WHERE id = $1
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

        LocationModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocationIdxModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM location_idx WHERE location_id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) => query.fetch_optional(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_optional(&mut **tx).await?
            }
        };

        match row {
            Some(row) => Ok(Some(
                LocationIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?,
            )),
            None => Ok(None),
        }
    }

    async fn find_ids_by_street_line1(
        &self,
        street_line1: &str,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        let query = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE street_line1 = $1
            "#,
        )
        .bind(street_line1);

        let ids = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        Ok(ids)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM location_idx WHERE location_id = ANY($1)
            "#,
        )
        .bind(ids);

        let rows = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(locations)
    }

    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT * FROM location_idx WHERE locality_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(locality_id)
        .bind(page_size)
        .bind((page - 1) * page_size);

        let rows = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(locations)
    }

    async fn find_by_type_and_locality(
        &self,
        location_type: LocationType,
        locality_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        let query = sqlx::query(
            r#"
            SELECT li.*
            FROM location_idx li
            JOIN location l ON li.location_id = l.id
            WHERE l.location_type = $1 AND l.locality_id = $2
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(location_type)
        .bind(locality_id)
        .bind(page_size)
        .bind((page - 1) * page_size);

        let rows = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(locations)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let query = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM location WHERE id = $1)",
            id
        );
        let exists = match &self.executor {
            Executor::Pool(pool) => query.fetch_one(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_one(&mut **tx).await?
            }
        };
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_location_type(
        &self,
        location_type: LocationType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let query = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE location_type = $1
            "#,
        )
        .bind(location_type);

        let ids = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        Ok(ids)
    }

    async fn find_ids_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let query = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE locality_id = $1
            "#,
        )
        .bind(locality_id);

        let ids = match &self.executor {
            Executor::Pool(pool) => query.fetch_all(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_all(&mut **tx).await?
            }
        };
        Ok(ids)
    }
}

#[async_trait]
impl TransactionAware for LocationRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        Ok(())
    }
}

impl TryFromRow<PgRow> for LocationModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocationModel {
            id: row.get("id"),
            location_type: row.get("location_type"),
            street_line1: get_heapless_string(row, "street_line1")?,
            street_line2: get_optional_heapless_string(row, "street_line2")?,
            street_line3: get_optional_heapless_string(row, "street_line3")?,
            street_line4: get_optional_heapless_string(row, "street_line4")?,
            locality_id: row.get("locality_id"),
            postal_code: get_optional_heapless_string(row, "postal_code")?,
            latitude: row.get("latitude"),
            longitude: row.get("longitude"),
            accuracy_meters: row.get("accuracy_meters"),
        })
    }
}

impl TryFromRow<PgRow> for LocationIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(LocationIdxModel {
            location_id: row.get("location_id"),
            locality_id: row.get("locality_id"),
            version: row.get("version"),
            hash: row.get("hash"),
        })
    }
}