use banking_db::repository::{
    audit_repository::AuditLogRepository,
    person_repository::{
        CountryRepository, CountrySubdivisionRepository, EntityReferenceRepository,
        LocalityRepository, LocationRepository, MessagingRepository, PersonRepository,
    },
};
use std::sync::Arc;

use sqlx::Postgres;

pub struct Repositories {
    pub person_repository: Arc<dyn PersonRepository<Postgres>>,
    pub audit_log_repository: Arc<dyn AuditLogRepository<Postgres>>,
    pub country_repository: Arc<dyn CountryRepository<Postgres>>,
    pub country_subdivision_repository: Arc<dyn CountrySubdivisionRepository<Postgres>>,
    pub locality_repository: Arc<dyn LocalityRepository<Postgres>>,
    pub location_repository: Arc<dyn LocationRepository<Postgres>>,
    pub messaging_repository: Arc<dyn MessagingRepository<Postgres>>,
    pub entity_reference_repository: Arc<dyn EntityReferenceRepository<Postgres>>,
}