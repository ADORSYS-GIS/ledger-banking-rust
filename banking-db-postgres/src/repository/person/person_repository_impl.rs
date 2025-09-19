use async_trait::async_trait;
use banking_api::BankingResult;
use banking_db::models::person::{PersonAuditModel, PersonIdxModel, PersonIdxModelCache, PersonModel};
use banking_db::repository::{LocationRepository, PersonRepository, PersonRepositoryError, PersonResult, TransactionAware};
use crate::repository::executor::Executor;
use crate::repository::person::location_repository_impl::LocationRepositoryImpl;
use crate::utils::{get_heapless_string, get_optional_heapless_string, TryFromRow};
use sqlx::{postgres::PgRow, Postgres, Row};
use std::error::Error;
use std::hash::Hasher;
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;
use twox_hash::XxHash64;
use uuid::Uuid;

pub struct PersonRepositoryImpl {
    pub(crate) executor: Executor,
    pub(crate) person_idx_cache: Arc<TokioRwLock<TransactionAwarePersonIdxModelCache>>,
    pub(crate) location_repository: Arc<LocationRepositoryImpl>,
}

impl PersonRepositoryImpl {
    pub fn new(
        executor: Executor,
        location_repository: Arc<LocationRepositoryImpl>,
        person_idx_cache: Arc<RwLock<PersonIdxModelCache>>,
    ) -> Self {
        Self {
            executor,
            person_idx_cache: Arc::new(TokioRwLock::new(
                TransactionAwarePersonIdxModelCache::new(person_idx_cache),
            )),
            location_repository,
        }
    }

    pub async fn load_all_person_idx(
        executor: &Executor,
    ) -> Result<Vec<PersonIdxModel>, sqlx::Error> {
        let query = sqlx::query_as::<_, PersonIdxModel>("SELECT * FROM person_idx");
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
impl PersonRepository<Postgres> for PersonRepositoryImpl {
    async fn save(
        &self,
        person: PersonModel,
        audit_log_id: Uuid,
    ) -> PersonResult<PersonModel> {
        if let Some(org_id) = person.organization_person_id {
            if !self.exists_by_id(org_id).await? {
                return Err(PersonRepositoryError::OrganizationNotFound(org_id));
            }
        }
        if let Some(loc_id) = person.location_id {
            if !self
                .location_repository
                .exists_by_id(loc_id)
                .await
                .map_err(|e| PersonRepositoryError::RepositoryError(e.into()))?
            {
                return Err(PersonRepositoryError::LocationNotFound(loc_id));
            }
        }
        if let Some(dup_id) = person.duplicate_of_person_id {
            if !self.exists_by_id(dup_id).await? {
                return Err(PersonRepositoryError::DuplicatePersonNotFound(dup_id));
            }
        }

        let mut hasher = XxHash64::with_seed(0);
        let mut person_cbor = Vec::new();
        ciborium::ser::into_writer(&person, &mut person_cbor).unwrap();
        hasher.write(&person_cbor);
        let new_hash = hasher.finish() as i64;

        let maybe_existing_idx = {
            let cache_read_guard = self.person_idx_cache.read().await;
            cache_read_guard.get_by_primary(&person.id)
        };

        let new_external_hash = person.external_identifier.as_ref().map(|s| {
            let mut hasher = XxHash64::with_seed(0);
            hasher.write(s.as_bytes());
            hasher.finish() as i64
        });

        let (version, is_update) = if let Some(existing_idx) = maybe_existing_idx {
            if existing_idx.hash == new_hash {
                return Ok(person);
            }
            (existing_idx.version + 1, true)
        } else {
            (0, false)
        };

        let audit_model = PersonAuditModel {
            person_id: person.id,
            version,
            hash: new_hash,
            person_type: person.person_type,
            display_name: person.display_name.clone(),
            external_identifier: person.external_identifier.clone(),
            entity_reference_count: person.entity_reference_count,
            organization_person_id: person.organization_person_id,
            messaging_info1: person.messaging_info1.clone(),
            messaging_info2: person.messaging_info2.clone(),
            messaging_info3: person.messaging_info3.clone(),
            messaging_info4: person.messaging_info4.clone(),
            messaging_info5: person.messaging_info5.clone(),
            department: person.department.clone(),
            location_id: person.location_id,
            duplicate_of_person_id: person.duplicate_of_person_id,
            audit_log_id,
        };

        let query1 = sqlx::query(
            r#"
                INSERT INTO person_audit (
                    person_id, version, hash, person_type, display_name, external_identifier,
                    organization_person_id, messaging_info1, messaging_info2, messaging_info3,
                    messaging_info4, messaging_info5, department, location_id, duplicate_of_person_id,
                    entity_reference_count, audit_log_id
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            "#,
        )
        .bind(audit_model.person_id)
        .bind(audit_model.version)
        .bind(audit_model.hash)
        .bind(audit_model.person_type)
        .bind(audit_model.display_name.as_str())
        .bind(audit_model.external_identifier.as_ref().map(|s| s.as_str()))
        .bind(audit_model.organization_person_id)
        .bind(audit_model.messaging_info1.as_ref().map(|s| s.as_str()))
        .bind(audit_model.messaging_info2.as_ref().map(|s| s.as_str()))
        .bind(audit_model.messaging_info3.as_ref().map(|s| s.as_str()))
        .bind(audit_model.messaging_info4.as_ref().map(|s| s.as_str()))
        .bind(audit_model.messaging_info5.as_ref().map(|s| s.as_str()))
        .bind(audit_model.department.as_ref().map(|s| s.as_str()))
        .bind(audit_model.location_id)
        .bind(audit_model.duplicate_of_person_id)
        .bind(audit_model.entity_reference_count)
        .bind(audit_model.audit_log_id);

        let (query2_sql, query3_sql) = if is_update {
            (
                r#"
                UPDATE person SET
                    person_type = $2::person_type, display_name = $3, external_identifier = $4,
                    organization_person_id = $5, messaging_info1 = $6, messaging_info2 = $7,
                    messaging_info3 = $8, messaging_info4 = $9, messaging_info5 = $10,
                    department = $11, location_id = $12, duplicate_of_person_id = $13,
                    entity_reference_count = $14
                WHERE id = $1
                "#,
                r#"
                UPDATE person_idx SET
                    external_identifier_hash = $2,
                    organization_person_id = $3,
                    duplicate_of_person_id = $4,
                    version = $5,
                    hash = $6
                WHERE person_id = $1
                "#,
            )
        } else {
            (
                r#"
                INSERT INTO person (
                    id, person_type, display_name, external_identifier, organization_person_id,
                    messaging_info1, messaging_info2, messaging_info3, messaging_info4, messaging_info5,
                    department, location_id, duplicate_of_person_id, entity_reference_count
                )
                VALUES ($1, $2::person_type, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                "#,
                r#"
                INSERT INTO person_idx (
                    person_id, external_identifier_hash, organization_person_id,
                    duplicate_of_person_id, version, hash
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
        };

        let query2 = sqlx::query(query2_sql)
            .bind(person.id)
            .bind(person.person_type)
            .bind(person.display_name.as_str())
            .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
            .bind(person.organization_person_id)
            .bind(person.messaging_info1.as_ref().map(|s| s.as_str()))
            .bind(person.messaging_info2.as_ref().map(|s| s.as_str()))
            .bind(person.messaging_info3.as_ref().map(|s| s.as_str()))
            .bind(person.messaging_info4.as_ref().map(|s| s.as_str()))
            .bind(person.messaging_info5.as_ref().map(|s| s.as_str()))
            .bind(person.department.as_ref().map(|s| s.as_str()))
            .bind(person.location_id)
            .bind(person.duplicate_of_person_id)
            .bind(person.entity_reference_count);

        let query3 = sqlx::query(query3_sql)
            .bind(person.id)
            .bind(new_external_hash)
            .bind(person.organization_person_id)
            .bind(person.duplicate_of_person_id)
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

        let new_idx = PersonIdxModel {
            person_id: person.id,
            external_identifier_hash: new_external_hash,
            organization_person_id: person.organization_person_id,
            duplicate_of_person_id: person.duplicate_of_person_id,
            version,
            hash: new_hash,
        };

        if is_update {
            self.person_idx_cache.read().await.update(new_idx);
        } else {
            self.person_idx_cache.read().await.add(new_idx);
        }

        Ok(person)
    }

    async fn load(&self, id: Uuid) -> PersonResult<PersonModel> {
        let query = sqlx::query(
            r#"
            SELECT * FROM person WHERE id = $1
            "#,
        )
        .bind(id);

        let row = match &self.executor {
            Executor::Pool(pool) =>	query.fetch_one(&**pool).await?,
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query.fetch_one(&mut **tx).await?
            }
        };

        PersonModel::try_from_row(&row).map_err(PersonRepositoryError::RepositoryError)
    }

    async fn find_by_id(&self, id: Uuid) -> PersonResult<Option<PersonIdxModel>> {
        Ok(self.person_idx_cache.read().await.get_by_primary(&id))
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<PersonIdxModel>> {
        let cache = self.person_idx_cache.read().await;
        let results = ids.iter().filter_map(|id| cache.get_by_primary(id)).collect();
        Ok(results)
    }

    async fn exists_by_id(&self, id: Uuid) -> PersonResult<bool> {
        Ok(self.person_idx_cache.read().await.contains_primary(&id))
    }

    async fn exist_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<(Uuid, bool)>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let cache = self.person_idx_cache.read().await;
        Ok(ids.iter().map(|id| (*id, cache.contains_primary(id))).collect())
    }

    async fn get_ids_by_external_identifier(&self, identifier: &str) -> PersonResult<Vec<Uuid>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(identifier.as_bytes());
        let hash = hasher.finish() as i64;

        let cache = self.person_idx_cache.read().await;
        Ok(cache.get_by_external_identifier_hash(&hash).unwrap_or_default())
    }

    async fn get_by_external_identifier(
        &self,
        identifier: &str,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(identifier.as_bytes());
        let hash = hasher.finish() as i64;

        let cache = self.person_idx_cache.read().await;
        let ids = cache.get_by_external_identifier_hash(&hash).unwrap_or_default();
        let results = ids.iter().filter_map(|id| cache.get_by_primary(id)).collect();
        Ok(results)
    }

    async fn find_by_duplicate_of_person_id(
        &self,
        person_id: Uuid,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        let cache = self.person_idx_cache.read().await;
        let results = cache.iter().into_iter()
            .filter(|item| item.duplicate_of_person_id == Some(person_id))
            .collect();
        Ok(results)
    }

    async fn find_by_organization_person_id(
        &self,
        person_id: Uuid,
    ) -> PersonResult<Vec<PersonIdxModel>> {
        let cache = self.person_idx_cache.read().await;
        let results = cache.iter().into_iter()
            .filter(|item| item.organization_person_id == Some(person_id))
            .collect();
        Ok(results)
    }
}

#[async_trait]
impl TransactionAware for PersonRepositoryImpl {
    async fn on_commit(&self) -> BankingResult<()> {
        self.person_idx_cache.read().await.on_commit().await
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.person_idx_cache.read().await.on_rollback().await
    }
}

pub struct TransactionAwarePersonIdxModelCache {
    shared_cache: Arc<RwLock<PersonIdxModelCache>>,
    local_additions: RwLock<HashMap<Uuid, PersonIdxModel>>,
    local_updates: RwLock<HashMap<Uuid, PersonIdxModel>>,
    local_deletions: RwLock<HashSet<Uuid>>,
}

impl TransactionAwarePersonIdxModelCache {
    pub fn new(shared_cache: Arc<RwLock<PersonIdxModelCache>>) -> Self {
        Self {
            shared_cache,
            local_additions: RwLock::new(HashMap::new()),
            local_updates: RwLock::new(HashMap::new()),
            local_deletions: RwLock::new(HashSet::new()),
        }
    }

    pub fn add(&self, item: PersonIdxModel) {
        let primary_key = item.person_id;
        self.local_deletions.write().remove(&primary_key);
        self.local_additions.write().insert(primary_key, item);
    }

    pub fn update(&self, item: PersonIdxModel) {
        let primary_key = item.person_id;
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

    pub fn get_by_primary(&self, primary_key: &Uuid) -> Option<PersonIdxModel> {
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

    pub fn contains_primary(&self, primary_key: &Uuid) -> bool {
        if self.local_deletions.read().contains(primary_key) {
            return false;
        }
        if self.local_additions.read().contains_key(primary_key) {
            return true;
        }
        if self.local_updates.read().contains_key(primary_key) {
            return true;
        }
        self.shared_cache.read().contains_primary(primary_key)
    }

    pub fn get_by_external_identifier_hash(&self, hash: &i64) -> Option<Vec<Uuid>> {
        let shared_cache = self.shared_cache.read();
        let items: Vec<Uuid> = shared_cache
            .get_by_external_identifier_hash(hash)
            .cloned()
            .unwrap_or_default();
        let mut result_set: HashSet<Uuid> = items.into_iter().collect();

        for id in self.local_deletions.read().iter() {
            result_set.remove(id);
        }
        for (key, item) in self.local_additions.read().iter() {
            if item.external_identifier_hash == Some(*hash) {
                result_set.insert(*key);
            }
        }
        for (key, item) in self.local_updates.read().iter() {
            if item.external_identifier_hash == Some(*hash) {
                result_set.insert(*key);
            }
        }

        if result_set.is_empty() {
            None
        } else {
            Some(result_set.into_iter().collect())
        }
    }

    pub fn iter(&self) -> Vec<PersonIdxModel> {
        let mut combined = Vec::new();
        let shared = self.shared_cache.read();
        for item in shared.iter() {
            combined.push(
                self.local_updates.read().get(&item.person_id).cloned().unwrap_or_else(|| {
                    if self.local_deletions.read().contains(&item.person_id) {
                        return item.clone();
                    }
                    item.clone()
                }),
            );
        }
        combined.extend(self.local_additions.read().values().cloned());
        combined.retain(|item| !self.local_deletions.read().contains(&item.person_id));
        combined
    }
}

#[async_trait]
impl TransactionAware for TransactionAwarePersonIdxModelCache {
    async fn on_commit(&self) -> BankingResult<()> {
        let mut shared = self.shared_cache.write();
        for item in self.local_additions.read().values() {
            shared.add(item.clone());
        }
        for item in self.local_updates.read().values() {
            shared.update(item.clone());
        }
        for id in self.local_deletions.read().iter() {
            shared.remove(id);
        }
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        self.local_deletions.write().clear();
        Ok(())
    }

    async fn on_rollback(&self) -> BankingResult<()> {
        self.local_additions.write().clear();
        self.local_updates.write().clear();
        self.local_deletions.write().clear();
        Ok(())
    }
}

impl TryFromRow<PgRow> for PersonModel {
    fn try_from_row(row: &PgRow) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(PersonModel {
            id: row.get("id"),
            person_type: row.get("person_type"),
            display_name: get_heapless_string(row, "display_name")?,
            external_identifier: get_optional_heapless_string(row, "external_identifier")?,
            organization_person_id: row.get("organization_person_id"),
            messaging_info1: get_optional_heapless_string(row, "messaging_info1")?,
            messaging_info2: get_optional_heapless_string(row, "messaging_info2")?,
            messaging_info3: get_optional_heapless_string(row, "messaging_info3")?,
            messaging_info4: get_optional_heapless_string(row, "messaging_info4")?,
            messaging_info5: get_optional_heapless_string(row, "messaging_info5")?,
            department: get_optional_heapless_string(row, "department")?,
            location_id: row.get("location_id"),
            duplicate_of_person_id: row.get("duplicate_of_person_id"),
            entity_reference_count: row.get("entity_reference_count"),
        })
    }
}