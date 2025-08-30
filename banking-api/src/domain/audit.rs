use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// # Service Trait
/// - FQN: banking-api/src/service/audit_service.rs/AuditService
/// # Documentation
/// - struct to maintain an audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    /// # Trait method
    /// - find_by_id
    pub id: Uuid,
    pub updated_at: DateTime<Utc>,
    pub updated_by_person_id: Uuid,
}
