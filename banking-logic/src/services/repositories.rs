use banking_db::repository::{
    audit_repository::AuditLogRepository,
    person_repository::{
        CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository,
        LocalityRepository, LocationRepository, MessagingRepository, PersonRepository,
    },
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