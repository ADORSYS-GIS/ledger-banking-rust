use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{
    MessagingAuditModel, MessagingIdxModel, MessagingIdxModelCache, MessagingModel,
};
use banking_db::repository::{MessagingRepository, TransactionAware};
use crate::repository::executor::Executor;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::hash::Hasher;
use std::sync::Arc;
use parking_lot::RwLock;
use tokio::sync::RwLock as TokioRwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct MessagingRepositoryImpl {
    executor: Executor,
    messaging_idx_cache: Arc<TokioRwLock<TransactionAwareMessagingIdxModelCache>>,
}

impl MessagingRepositoryImpl {
    pub fn new(executor: Executor, messaging_idx_cache: Arc<RwLock<MessagingIdxModelCache>>) -> Self {
        Self {
            executor,
            messaging_idx_cache: Arc::new(TokioRwLock::new(
                TransactionAwareMessagingIdxModelCache::new(messaging_idx_cache),
            )),
        }
    }

    pub async fn load_all_messaging_idx(
        executor: &Executor,
    ) -> Result<Vec<MessagingIdxModel>, sqlx::Error> {
        let query = sqlx::query_as::<_, MessagingIdxModel>("SELECT * FROM messaging_idx");
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
impl MessagingRepository<Postgres> for MessagingRepositoryImpl {
    async fn save(
        &self,
        messaging: MessagingModel,
        audit_log_id: Uuid,
    ) -> Result<MessagingModel, sqlx::Error> {
        let mut hasher = XxHash64::with_seed(0);
        let mut messaging_cbor = Vec::new();
        ciborium::ser::into_writer(&messaging, &mut messaging_cbor).unwrap();
        hasher.write(&messaging_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.messaging_idx_cache.read().await;
            cache_read_guard.get_by_primary(&messaging.id)
        };

        let mut value_hasher = XxHash64::with_seed(0);
        value_hasher.write(messaging.value.as_bytes());
        let new_value_hash = value_hasher.finish() as i64;

        if let Some(existing_idx) = maybe_existing_idx {
            // UPDATE
            if existing_idx.hash == new_hash {
                return Ok(messaging); // No changes
            }

            let new_version = existing_idx.version + 1;

            let audit_model = MessagingAuditModel {
                messaging_id: messaging.id,
                version: new_version,
                hash: new_hash,
                messaging_type: messaging.messaging_type,
                value: messaging.value.clone(),
                other_type: messaging.other_type.clone(),
                audit_log_id,
            };

            let query1 = sqlx::query(
                r#"
                INSERT INTO messaging_audit (messaging_id, version, hash, messaging_type, value, other_type, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(audit_model.messaging_id)
            .bind(audit_model.version)
            .bind(audit_model.hash)
            .bind(audit_model.messaging_type)
            .bind(audit_model.value.as_str())
            .bind(audit_model.other_type.as_ref().map(|s| s.as_str()))
            .bind(audit_model.audit_log_id);

            let query2 = sqlx::query(
                r#"
                UPDATE messaging SET
                    messaging_type = $2::messaging_type, value = $3, other_type = $4
                WHERE id = $1
                "#,
            )
            .bind(messaging.id)
            .bind(messaging.messaging_type)
            .bind(messaging.value.as_str())
            .bind(messaging.other_type.as_ref().map(|s| s.as_str()));

            let query3 = sqlx::query(
                r#"
                UPDATE messaging_idx SET
                    value_hash = $2,
                    version = $3,
                    hash = $4
                WHERE messaging_id = $1
                "#,
            )
            .bind(messaging.id)
            .bind(new_value_hash)
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

            let new_idx = MessagingIdxModel {
                messaging_id: messaging.id,
                value_hash: new_value_hash,
                version: new_version,
                hash: new_hash,
            };
            self.messaging_idx_cache.read().await.update(new_idx);
        } else {
            // INSERT
            let version = 0;
            let audit_model = MessagingAuditModel {
                messaging_id: messaging.id,
                version,
                hash: new_hash,
                messaging_type: messaging.messaging_type,
                value: messaging.value.clone(),
                other_type: messaging.other_type.clone(),
                audit_log_id,
            };

            let query1 = sqlx::query(
                r#"
                INSERT INTO messaging_audit (messaging_id, version, hash, messaging_type, value, other_type, audit_log_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(audit_model.messaging_id)
            .bind(audit_model.version)
            .bind(audit_model.hash)
            .bind(audit_model.messaging_type)
            .bind(audit_model.value.as_str())
            .bind(audit_model.other_type.as_ref().map(|s| s.as_str()))
            .bind(audit_model.audit_log_id);

            let query2 = sqlx::query(
                r#"
                INSERT INTO messaging (id, messaging_type, value, other_type)
                VALUES ($1, $2::messaging_type, $3, $4)
                "#,
            )
            .bind(messaging.id)
            .bind(messaging.messaging_type)
            .bind(messaging.value.as_str())
            .bind(messaging.other_type.as_ref().map(|s| s.as_str()));

            let query3 = sqlx::query(
                r#"
                INSERT INTO messaging_idx (messaging_id, value_hash, version, hash)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(messaging.id)
            .bind(new_value_hash)
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

            let new_idx = MessagingIdxModel {
                messaging_id: messaging.id,
                value_hash: new_value_hash,
                version,
                hash: new_hash,
            };
            self.messaging_idx_cache.read().await.add(new_idx);
        }

        Ok(messaging)
    }

    async fn load(&self, id: Uuid) -> Result<MessagingModel, sqlx::Error> {
        let query = sqlx::query("SELECT * FROM messaging WHERE id = $1").bind(id);
        let row = match &self.executor {
            Executor::Pool(pool) => query.fetch_one(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_one(&mut **tx).await?
            }
        };
        MessagingModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self
            .messaging_idx_cache
            .read()
            .await
            .get_by_primary(&id))
    }
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingIdxModel>, Box<dyn Error + Send + Sync>> {
        let cache_read_guard = self.messaging_idx_cache.read().await;
        let mut messagings = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(messaging) = cache_read_guard.get_by_primary(id) {
                messagings.push(messaging);
            }
        }
        Ok(messagings)
    }
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self
            .messaging_idx_cache
            .read()
            .await
            .get_by_primary(&id)
            .is_some())
    }
    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(value.as_bytes());
        let hash = hasher.finish() as i64;

        let cache_read_guard = self.messaging_idx_cache.read().await;
        if let Some(id) = cache_read_guard.get_by_value_hash(&hash) {
            Ok(vec![id])
        } else {
            Ok(Vec::new())
        }
    }
}

#[async_trait]
impl TransactionAware for MessagingRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.messaging_idx_cache.read().await.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.messaging_idx_cache.read().await.on_rollback().await
    }
}

pub struct TransactionAwareMessagingIdxModelCache {
    shared_cache: Arc<RwLock<MessagingIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, MessagingIdxModel>>,
    local_updates: RwLock<HashMap<Uuid, MessagingIdxModel>>,
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwareMessagingIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<MessagingIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: MessagingIdxModel) {
        let primary_key = item.messaging_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: MessagingIdxModel) {
        let primary_key = item.messaging_id;
        self.local_deletions.write().remove(&primary_key);
        if let Some(local_item) = self.local_additions.write().get_mut(&primary_key) {
            *local_item = item;
            return;
        }
        self.local_updates.write().insert(primary_key, item);
    }

    pub fn remove(&self, primary_key: &Uuid) {
        if self.local_additions.write().remove(primary_key).is_none() {
            self.local_deletions.write().insert(*primary_key);
        }
        self.local_updates.write().remove(primary_key);
    }

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<MessagingIdxModel> {
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

    pub fn get_by_value_hash(&self, value_hash: &i64) -> Option<Uuid> {
        // Search in additions.
        for item in self.local_additions.read().values() {
            if item.value_hash == *value_hash {
                return Some(item.messaging_id);
            }
        }

        // Search in updates.
        for item in self.local_updates.read().values() {
            if item.value_hash == *value_hash {
                return Some(item.messaging_id);
            }
        }

        // If found in shared cache, we need to ensure it wasn't updated or deleted.
        if let Some(shared_id) = self.shared_cache.read().get_by_value_hash(value_hash) {
            // If it was deleted, it's not found.
            if self.local_deletions.read().contains(&shared_id) {
                return None;
            }
            // If it was updated, the shared cache version is stale.
            if self.local_updates.read().contains_key(&shared_id) {
                return None;
            }
            return Some(shared_id);
        }

        None
    }
}

#[async_trait]
impl TransactionAware for TransactionAwareMessagingIdxModelCache {
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

impl TryFromRow<PgRow> for MessagingModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(MessagingModel {
            id: row.get("id"),
            messaging_type: row.get("messaging_type"),
            value: get_heapless_string(row, "value")?,
            other_type: get_optional_heapless_string(row, "other_type")?,
        })
    }
}

impl TryFromRow<PgRow> for MessagingIdxModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(MessagingIdxModel {
            messaging_id: row.get("messaging_id"),
            value_hash: row.get("value_hash"),
            version: row.get("version"),
            hash: row.get("hash"),
        })
    }
}