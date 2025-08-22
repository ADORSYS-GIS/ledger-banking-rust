use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Repository Trait
/// - FQN: repository/audit_repository.rs/AuditLogRepository
/// # Natures
/// - Immutable: no modification once created
/// # Documentation
/// - struct to maintain an audit log
/// # Scoping
/// - One audit log per database transaction, all objects referenced in the change set reference the same audit log.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLogModel {
    /// # Trait method
    /// - find_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Nature
    /// - secondary index
    /// - unique: no
    pub updated_at: DateTime<Utc>,
    /// # Trait method
    /// - find_by_person_id_between_start_and_end
    ///     - person_id unique index in models/person.rs/Person.id
    ///     - start is self.updated_at
    ///     - end is self.updated_at
    /// # Nature
    /// - secondary index
    /// - unique: no
    /// # Validation
    /// - models/person.rs/PersonModel.find_by_id(self.updated_by_person_id)
    pub updated_by_person_id: Uuid,
}
