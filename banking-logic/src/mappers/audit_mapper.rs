use banking_api::domain::audit::AuditLog;
use banking_db::models::audit::AuditLogModel;

pub fn map_to_domain(model: &AuditLogModel) -> AuditLog {
    AuditLog {
        id: model.id,
        updated_at: model.updated_at,
        updated_by_person_id: model.updated_by_person_id,
    }
}

pub fn map_to_model(domain: &AuditLog) -> AuditLogModel {
    AuditLogModel {
        id: domain.id,
        updated_at: domain.updated_at,
        updated_by_person_id: domain.updated_by_person_id,
    }
}