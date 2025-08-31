use async_trait::async_trait;
use banking_db::models::person::{
    MessagingAuditModel, MessagingIdxModel, MessagingIdxModelCache, MessagingModel,
};
use banking_db::repository::MessagingRepository;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, PgPool, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use std::sync::{Arc, RwLock};
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct MessagingRepositoryImpl {
    pool: Arc<PgPool>,
    messaging_idx_cache: Arc<RwLock<MessagingIdxModelCache>>,
}

impl MessagingRepositoryImpl {
    pub async fn new(pool: Arc<PgPool>) -> Self {
        let messaging_idx_models = Self::load_all_messaging_idx(&pool).await.unwrap();
        let messaging_idx_cache = Arc::new(RwLock::new(
            MessagingIdxModelCache::new(messaging_idx_models).unwrap(),
        ));
        Self {
            pool,
            messaging_idx_cache,
        }
    }

    async fn load_all_messaging_idx(
        pool: &PgPool,
    ) -> Result<Vec<MessagingIdxModel>, sqlx::Error> {
        sqlx::query_as::<_, MessagingIdxModel>("SELECT * FROM messaging_idx")
            .fetch_all(pool)
            .await
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
            let cache_read_guard = self.messaging_idx_cache.read().unwrap();
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

            sqlx::query(
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
            .bind(audit_model.audit_log_id)
            .execute(&*self.pool)
            .await?;

            sqlx::query(
                r#"
                UPDATE messaging SET
                    messaging_type = $2::messaging_type, value = $3, other_type = $4
                WHERE id = $1
                "#,
            )
            .bind(messaging.id)
            .bind(messaging.messaging_type)
            .bind(messaging.value.as_str())
            .bind(messaging.other_type.as_ref().map(|s| s.as_str()))
            .execute(&*self.pool)
            .await?;

            sqlx::query(
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
            .bind(new_hash)
            .execute(&*self.pool)
            .await?;

            let new_idx = MessagingIdxModel {
                messaging_id: messaging.id,
                value_hash: new_value_hash,
                version: new_version,
                hash: new_hash,
            };
            self.messaging_idx_cache.write().unwrap().update(new_idx);
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

            sqlx::query(
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
            .bind(audit_model.audit_log_id)
            .execute(&*self.pool)
            .await?;

            sqlx::query(
                r#"
                INSERT INTO messaging (id, messaging_type, value, other_type)
                VALUES ($1, $2::messaging_type, $3, $4)
                "#,
            )
            .bind(messaging.id)
            .bind(messaging.messaging_type)
            .bind(messaging.value.as_str())
            .bind(messaging.other_type.as_ref().map(|s| s.as_str()))
            .execute(&*self.pool)
            .await?;

            sqlx::query(
                r#"
                INSERT INTO messaging_idx (messaging_id, value_hash, version, hash)
                VALUES ($1, $2, $3, $4)
                "#,
            )
            .bind(messaging.id)
            .bind(new_value_hash)
            .bind(version)
            .bind(new_hash)
            .execute(&*self.pool)
            .await?;

            let new_idx = MessagingIdxModel {
                messaging_id: messaging.id,
                value_hash: new_value_hash,
                version,
                hash: new_hash,
            };
            self.messaging_idx_cache.write().unwrap().add(new_idx);
        }

        Ok(messaging)
    }

    async fn load(&self, id: Uuid) -> Result<MessagingModel, sqlx::Error> {
        let row = sqlx::query("SELECT * FROM messaging WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pool)
            .await?;
        MessagingModel::try_from_row(&row).map_err(sqlx::Error::Decode)
    }

    async fn find_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MessagingIdxModel>, Box<dyn Error + Send + Sync>> {
        Ok(self.messaging_idx_cache.read().unwrap().get_by_primary(&id))
    }
    async fn find_by_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<MessagingIdxModel>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query("SELECT * FROM messaging_idx WHERE messaging_id = ANY($1)")
            .bind(ids)
            .fetch_all(&*self.pool)
            .await?;
        let mut messagings = Vec::new();
        for row in rows {
            messagings.push(MessagingIdxModel::try_from_row(&row)?);
        }
        Ok(messagings)
    }
    async fn exists_by_id(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exists = sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM messaging WHERE id = $1)", id)
            .fetch_one(&*self.pool)
            .await?;
        Ok(exists.unwrap_or(false))
    }
    async fn find_ids_by_value(
        &self,
        value: &str,
    ) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(value.as_bytes());
        let hash = hasher.finish() as i64;

        let ids = sqlx::query_scalar("SELECT messaging_id FROM messaging_idx WHERE value_hash = $1")
            .bind(hash)
            .fetch_all(&*self.pool)
            .await?;
        Ok(ids)
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