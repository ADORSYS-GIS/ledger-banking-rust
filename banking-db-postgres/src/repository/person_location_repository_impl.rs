use async_trait::async_trait;
use banking_db::models::person::{
    LocationAuditModel, LocationIdxModel, LocationIdxModelCache, LocationModel, LocationType,
};
use banking_db::repository::LocationRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::{Arc, RwLock};
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct LocationRepositoryImpl {
    pool: Arc<PgPool>,
    location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
}

impl LocationRepositoryImpl {
    pub async fn new(pool: Arc<PgPool>) -> Self {
        let location_idx_models = Self::load_all_location_idx(&pool).await.unwrap();
        let location_idx_cache =
            Arc::new(RwLock::new(LocationIdxModelCache::new(location_idx_models).unwrap()));
        Self {
            pool,
            location_idx_cache,
        }
    }

    async fn load_all_location_idx(
        pool: &PgPool,
    ) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        sqlx::query_as::<_, LocationIdxModel>("SELECT * FROM location_idx")
            .fetch_all(pool)
            .await
    }
}

#[async_trait]
impl LocationRepository<Postgres> for LocationRepositoryImpl {
    async fn save(
        &self,
        location: LocationModel,
        audit_log_id: Uuid,
    ) -> Result<LocationModel, sqlx::Error> {
        let mut hasher = XxHash64::with_seed(0);
        let location_cbor = serde_cbor::to_vec(&location).unwrap();
        hasher.write(&location_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.location_idx_cache.read().unwrap();
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

            sqlx::query(
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
            .bind(audit_model.audit_log_id)
            .execute(&*self.pool)
            .await?;

            sqlx::query(
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
            .bind(location.location_type)
            .execute(&*self.pool)
            .await?;

            sqlx::query(
                r#"
                UPDATE location_idx SET
                    version = $2,
                    hash = $3
                WHERE location_id = $1
                "#,
            )
            .bind(location.id)
            .bind(new_version)
            .bind(new_hash)
            .execute(&*self.pool)
            .await?;

            let new_idx = LocationIdxModel {
                location_id: location.id,
                locality_id: location.locality_id,
                version: new_version,
                hash: new_hash,
            };
            self.location_idx_cache.write().unwrap().update(new_idx);
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

            sqlx::query(
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
            .bind(audit_model.audit_log_id)
            .execute(&*self.pool)
            .await?;

            sqlx::query(
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
            .bind(location.location_type)
            .execute(&*self.pool)
            .await?;

            sqlx::query(
                r#"
                INSERT INTO location_idx (location_id, locality_id, version, hash)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(location.id)
            .bind(location.locality_id)
            .bind(version)
            .bind(new_hash)
            .execute(&*self.pool)
            .await?;

            let new_idx = LocationIdxModel {
                location_id: location.id,
                locality_id: location.locality_id,
                version,
                hash: new_hash,
            };
            self.location_idx_cache.write().unwrap().add(new_idx);
        }

        Ok(location)
    }

    async fn load(&self, id: Uuid) -> Result<LocationModel, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM location WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&*self.pool)
        .await?;

        LocationModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LocationIdxModel>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT * FROM location_idx WHERE location_id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await?;

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
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE street_line1 = $1
            "#,
        )
        .bind(street_line1)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<LocationIdxModel>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT * FROM location_idx WHERE location_id = ANY($1)
            "#,
        )
        .bind(ids)
        .fetch_all(&*self.pool)
        .await?;

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
        let rows = sqlx::query(
            r#"
            SELECT * FROM location_idx WHERE locality_id = $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(locality_id)
        .bind(page_size)
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

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
        let rows = sqlx::query(
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
        .bind((page - 1) * page_size)
        .fetch_all(&*self.pool)
        .await?;

        let mut locations = Vec::new();
        for row in rows {
            locations
                .push(LocationIdxModel::try_from_row(&row).map_err(sqlx::Error::Decode)?);
        }
        Ok(locations)
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM location WHERE id = $1)",
            id
        )
        .fetch_one(&*self.pool)
        .await?;
        Ok(exists.unwrap_or(false))
    }

    async fn find_ids_by_location_type(
        &self,
        location_type: LocationType,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE location_type = $1
            "#,
        )
        .bind(location_type)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
    }

    async fn find_ids_by_locality_id(
        &self,
        locality_id: Uuid,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let ids = sqlx::query_scalar(
            r#"
            SELECT id FROM location WHERE locality_id = $1
            "#,
        )
        .bind(locality_id)
        .fetch_all(&*self.pool)
        .await?;
        Ok(ids)
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