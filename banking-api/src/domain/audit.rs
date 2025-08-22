use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-api/src/service/audit_service.rs/AuditService
/// # Natures
/// - Immutable: no modification once created
/// # Documentation
/// - struct to maintain an audit log
/// # Scoping
/// - One audit log per database transaction, all objects referenced in the change set reference the same audit log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// # Trait method
    /// - find_audit_log_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Nature
    /// - secondary index
    /// - unique: no
    pub updated_at: DateTime<Utc>,
    /// # Trait method
    /// - find_by_person_id_between_start_and_end
    ///     - person_id: domain/person.rs/Person.id
    ///     - start: self.updated_at
    ///     - end: self.updated_at
    /// # Nature
    /// - not unique
    /// # Validation
    /// - domain/person.rs/Person.find_by_id(updated_by_person_id)
    pub updated_by_person_id: Uuid,
}
