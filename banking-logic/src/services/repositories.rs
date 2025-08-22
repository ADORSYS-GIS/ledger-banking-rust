use banking_db::repository::{
    audit_repository::AuditLogRepository,
    person_repository::{
        CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository,
        LocalityRepository, LocationRepository, MessagingRepository, PersonRepository,
    },
};
use std::sync::Arc;

pub struct Repositories {
    pub person_repository: Arc<dyn PersonRepository>,
    pub audit_log_repository: Arc<dyn AuditLogRepository>,
    pub country_repository: Arc<dyn CountryRepository>,
    pub country_subdivision_repository: Arc<dyn CountrySubdivisionRepository>,
    pub locality_repository: Arc<dyn LocalityRepository>,
    pub location_repository: Arc<dyn LocationRepository>,
    pub messaging_repository: Arc<dyn MessagingRepository>,
    pub entity_reference_repository: Arc<dyn EntityReferenceRepository>,
}