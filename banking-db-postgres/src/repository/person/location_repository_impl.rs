use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{
    LocationAuditModel, LocationIdxModel, LocationIdxModelCache, LocationModel,
};
use banking_db::repository::{
    LocalityRepository, LocationRepository, LocationRepositoryError, LocationResult,
    TransactionAware,
};
use crate::repository::executor::Executor;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hasher;
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct LocationRepositoryImpl {
    pub(crate) executor: Executor,
    pub(crate) location_idx_cache: Arc<TokioRwLock<TransactionAwareLocationIdxModelCache>>,
    pub(crate) locality_repository: Arc<LocalityRepositoryImpl>,
}

impl LocationRepositoryImpl {
    pub fn new(
        executor: Executor,
        locality_repository: Arc<LocalityRepositoryImpl>,
        location_idx_cache: Arc<RwLock<LocationIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            location_idx_cache: Arc::new(TokioRwLock::new(
                TransactionAwareLocationIdxModelCache::new(location_idx_cache),
            )),
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

    pub(crate) async fn get_idx_by_id(
        &self,
        id: Uuid,
    ) -> LocationResult<Option<LocationIdxModel>> {
        let cache = self.location_idx_cache.read().await;
        Ok(cache.get_by_primary(&id))
    }
}

#[async_trait]
impl LocationRepository<Postgres> for LocationRepositoryImpl {
    async fn save(
        &self,
        location: LocationModel,
        audit_log_id: Uuid,
    ) -> LocationResult<LocationModel> {
        if !self
            .locality_repository
            .exists_by_id(location.locality_id)
            .await
            .map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?
        {
            return Err(LocationRepositoryError::LocalityNotFound(
                location.locality_id,
            ));
        }

        let mut hasher = XxHash64::with_seed(0);
        let mut location_cbor = Vec::new();
        ciborium::ser::into_writer(&location, &mut location_cbor).unwrap();
        hasher.write(&location_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.location_idx_cache.read().await;
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
                    query1.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query2.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query3.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                }
                Executor::Tx(tx) => {
                    let mut tx = tx.lock().await;
                    query1.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query2.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query3.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                }
            }

            let new_idx = LocationIdxModel {
                location_id: location.id,
                locality_id: location.locality_id,
                version: new_version,
                hash: new_hash,
            };
            self.location_idx_cache.read().await.update(new_idx);
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
                    query1.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query2.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query3.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                }
                Executor::Tx(tx) => {
                    let mut tx = tx.lock().await;
                    query1.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query2.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                    query3.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                }
            }

            let new_idx = LocationIdxModel {
                location_id: location.id,
                locality_id: location.locality_id,
                version,
                hash: new_hash,
            };
            self.location_idx_cache.read().await.add(new_idx);
        }

        Ok(location)
    }

    async fn load(&self, id: Uuid) -> LocationResult<LocationModel> {
        let query = sqlx::query(
            r#"
            SELECT * FROM location WHERE id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) => query.fetch_one(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_one(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?
            }
        };

        LocationModel::try_from_row(&row).map_err(LocationRepositoryError::RepositoryError)
    }

    async fn find_by_id(&self, id: Uuid) -> LocationResult<Option<LocationIdxModel>> {
        Ok(self.location_idx_cache.read().await.get_by_primary(&id))
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> LocationResult<Vec<LocationIdxModel>> {
        let cache = self.location_idx_cache.read().await;
        let mut locations = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(location_idx) = cache.get_by_primary(id) {
                locations.push(location_idx);
            }
        }
        Ok(locations)
    }

    async fn find_by_locality_id(
        &self,
        locality_id: Uuid,
        _page: i32,
        _page_size: i32,
    ) -> LocationResult<Vec<LocationIdxModel>> {
        let cache = self.location_idx_cache.read().await;
        Ok(cache.get_by_locality_id(&locality_id))
    }

    async fn exists_by_id(&self, id: Uuid) -> LocationResult<bool> {
        Ok(self
            .location_idx_cache
            .read()
            .await
            .get_by_primary(&id)
            .is_some())
    }

    async fn find_ids_by_locality_id(&self, locality_id: Uuid) -> LocationResult<Vec<Uuid>> {
        let cache = self.location_idx_cache.read().await;
        let locations = cache.get_by_locality_id(&locality_id);
        let ids = locations.into_iter().map(|loc| loc.location_id).collect();
        Ok(ids)
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> LocationResult<Vec<(Uuid, bool)>> {
        let mut results = Vec::with_capacity(ids.len());
        let cache = self.location_idx_cache.read().await;
        for &id in ids {
            results.push((id, cache.get_by_primary(&id).is_some()));
        }
        Ok(results)
    }
}

#[async_trait]
impl TransactionAware for LocationRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.location_idx_cache.read().await.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.location_idx_cache.read().await.on_rollback().await
    }
}

pub struct TransactionAwareLocationIdxModelCache {
    shared_cache: Arc<RwLock<LocationIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, LocationIdxModel>>,
    local_updates: RwLock<HashMap<Uuid, LocationIdxModel>>,
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwareLocationIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<LocationIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: LocationIdxModel) {
        let primary_key = item.location_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: LocationIdxModel) {
        let primary_key = item.location_id;
        self.local_deletions.write().remove(&primary_key);
        if let Some(local_item) = self.local_additions.write().get_mut(&primary_key) {
            *local_item = item;
            return;
        }
        self.local_updates.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        self.local_additions.write().remove(primary_key);
        self.local_updates.write().remove(primary_key);
        self.local_deletions.write().insert(*primary_key);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<LocationIdxModel> {
        if self.local_deletions.read().contains(primary_key) {
            return None;
        }
        if let Some(item) = self.local_additions.read().get(primary_key) {
            return Some(item.clone());
        }
        if let Some(item) = self.local_updates.read().get(primary_key) {
            return Some(item.clone());
        }
        self.shared_cache.read().get_by_primary(primary_key)
    }

    pub fn get_by_locality_id(&self, locality_id: &Uuid) -> Vec<LocationIdxModel> {
        let shared_cache = self.shared_cache.read();
        let local_additions = self.local_additions.read();
        let local_updates = self.local_updates.read();
        let local_deletions = self.local_deletions.read();

        let mut results: HashMap<Uuid, LocationIdxModel> = HashMap::new();

        if let Some(ids) = shared_cache.get_by_locality_id(locality_id) {
            for id in ids {
                if let Some(item) = shared_cache.get_by_primary(id) {
                    results.insert(item.location_id, item);
                }
            }
        }

        for id in local_updates.keys() {
            results.remove(id);
        }
        for id in local_deletions.iter() {
            results.remove(id);
        }

        for item in local_additions.values() {
            if item.locality_id == *locality_id {
                results.insert(item.location_id, item.clone());
            }
        }
        for item in local_updates.values() {
            if item.locality_id == *locality_id {
                results.insert(item.location_id, item.clone());
            }
        }

        results.into_values().collect()
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareLocationIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared_cache = self.shared_cache.write();
        let mut local_additions = self.local_additions.write();
        let mut local_updates = self.local_updates.write();
        let mut local_deletions = self.local_deletions.write();

        for item in local_additions.values() {
            shared_cache.add(item.clone());
        }
        for item in local_updates.values() {
            shared_cache.update(item.clone());
        }
        for primary_key in local_deletions.iter() {
            shared_cache.remove(primary_key);
        }

        local_additions.clear();
        local_updates.clear();
        local_deletions.clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        self.local_deletions.write().clear();
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