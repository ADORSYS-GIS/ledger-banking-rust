use banking_db::repository::{
    audit_repository::AuditLogRepository, CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository, LocalityRepository, LocationRepository, MessagingRepository, PersonRepository,
};
use std::sync::Arc;

use sqlx::Database;

pub struct Repositories<DB: Database> {
    pub person_repository: Arc<dyn PersonRepository<DB>>,
    pub audit_log_repository: Arc<dyn AuditLogRepository<DB>>,
    pub country_repository: Arc<dyn CountryRepository<DB>>,
    pub country_subdivision_repository: Arc<dyn CountrySubdivisionRepository<DB>>,
    pub locality_repository: Arc<dyn LocalityRepository<DB>>,
    pub location_repository: Arc<dyn LocationRepository<DB>>,
    pub messaging_repository: Arc<dyn MessagingRepository<DB>>,
    pub entity_reference_repository: Arc<dyn EntityReferenceRepository<DB>>,
}

impl<DB: Database> Clone for Repositories<DB> {
    fn clone(&self) -> Self {
        Self {
            person_repository: self.person_repository.clone(),
            audit_log_repository: self.audit_log_repository.clone(),
            country_repository: self.country_repository.clone(),
            country_subdivision_repository: self.country_subdivision_repository.clone(),
            locality_repository: self.locality_repository.clone(),
            location_repository: self.location_repository.clone(),
            messaging_repository: self.messaging_repository.clone(),
            entity_reference_repository: self.entity_reference_repository.clone(),
        }
    }
}