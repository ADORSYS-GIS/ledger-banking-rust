use banking_logic::services::repositories::Repositories;
use banking_logic::services::{
    CountryServiceImpl, CountrySubdivisionServiceImpl, EntityReferenceServiceImpl,
    LocalityServiceImpl, LocationServiceImpl, MessagingServiceImpl, PersonServiceImpl,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::person::mock_country_repository::MockCountryRepository;
use crate::person::mock_country_subdivision_repository::MockCountrySubdivisionRepository;
use crate::person::mock_locality_repository::MockLocalityRepository;
use crate::person::mock_location_repository::MockLocationRepository;
use crate::person::mock_messaging_repository::MockMessagingRepository;
use crate::person::mock_entity_reference_repository::MockEntityReferenceRepository;
use crate::person::mock_person_repository::MockPersonRepository;
use banking_db::models::audit::AuditLogModel;
use banking_db::repository::audit_repository::{AuditDomainError, AuditLogRepository};
use sqlx::Postgres;
use std::sync::Mutex;
use async_trait::async_trait;

pub struct TestServices {
    pub country_service: CountryServiceImpl<Postgres>,
    pub country_subdivision_service: CountrySubdivisionServiceImpl<Postgres>,
    pub locality_service: LocalityServiceImpl<Postgres>,
    pub location_service: LocationServiceImpl<Postgres>,
    pub messaging_service: MessagingServiceImpl<Postgres>,
    pub entity_reference_service: EntityReferenceServiceImpl<Postgres>,
    pub person_service: PersonServiceImpl<Postgres>,
    pub mock_country_subdivision_repository: Arc<MockCountrySubdivisionRepository>,
}

#[derive(Default)]
struct MockAuditLogRepository {
    audit_logs: Mutex<Vec<AuditLogModel>>,
}

#[async_trait]
impl AuditLogRepository<Postgres> for MockAuditLogRepository {
    async fn create(&self, audit_log: &AuditLogModel) -> Result<AuditLogModel, AuditDomainError> {
        self.audit_logs.lock().unwrap().push(audit_log.clone());
        Ok(audit_log.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuditLogModel>, AuditDomainError> {
        Ok(self
            .audit_logs
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.id == id)
            .cloned())
    }
}

pub fn create_test_services() -> TestServices {
    let mock_country_subdivision_repository =
        Arc::new(MockCountrySubdivisionRepository::default());
    let repositories = Repositories {
        person_repository: Arc::new(MockPersonRepository::default()),
        audit_log_repository: Arc::new(MockAuditLogRepository::default()),
        country_repository: Arc::new(MockCountryRepository::default()),
        country_subdivision_repository: mock_country_subdivision_repository.clone(),
        locality_repository: Arc::new(MockLocalityRepository::default()),
        location_repository: Arc::new(MockLocationRepository::default()),
        messaging_repository: Arc::new(MockMessagingRepository::default()),
        entity_reference_repository: Arc::new(MockEntityReferenceRepository::default()),
    };
    TestServices {
        country_service: CountryServiceImpl::new(repositories.clone()),
        country_subdivision_service: CountrySubdivisionServiceImpl::new(repositories.clone()),
        locality_service: LocalityServiceImpl::new(repositories.clone()),
        location_service: LocationServiceImpl::new(repositories.clone()),
        messaging_service: MessagingServiceImpl::new(repositories.clone()),
        entity_reference_service: EntityReferenceServiceImpl::new(repositories.clone()),
        person_service: PersonServiceImpl::new(repositories),
        mock_country_subdivision_repository,
    }
}

pub fn create_test_audit_log() -> banking_api::domain::AuditLog {
    banking_api::domain::AuditLog {
        id: Uuid::new_v4(),
        updated_at: chrono::Utc::now(),
        updated_by_person_id: Uuid::new_v4(),
    }
}