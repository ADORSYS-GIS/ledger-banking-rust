use async_trait::async_trait;
use banking_api::domain::person::Person;
use banking_api::service::{PersonService, PersonServiceError, PersonServiceResult};
use banking_db::repository::PersonDomainError;
use heapless::String as HeaplessString;
use sqlx::Database;
use uuid::Uuid;

use crate::mappers::person_mapper::{ToDomain, ToModel};
use crate::services::repositories::Repositories;

pub struct PersonServiceImpl<DB: Database> {
    repositories: Repositories<DB>,
}

impl<DB: Database> PersonServiceImpl<DB> {
    pub fn new(repositories: Repositories<DB>) -> Self {
        Self { repositories }
    }

    fn map_domain_error(err: PersonDomainError) -> PersonServiceError {
        match err {
            PersonDomainError::InvalidHierarchy(msg) => PersonServiceError::InvalidHierarchy(msg),
            PersonDomainError::DuplicateExternalId(id) => {
                PersonServiceError::DuplicateExternalId(id)
            }
            PersonDomainError::CascadeDeleteBlocked(ids) => {
                PersonServiceError::CascadeDeleteBlocked(ids.len())
            }
            PersonDomainError::OrganizationNotFound(id) => {
                PersonServiceError::OrganizationNotFound(id)
            }
            PersonDomainError::LocationNotFound(id) => PersonServiceError::LocationNotFound(id),
            PersonDomainError::DuplicatePersonNotFound(id) => {
                PersonServiceError::DuplicatePersonNotFound(id)
            }
            PersonDomainError::InvalidPersonTypeChange { from, to } => {
                PersonServiceError::InvalidPersonTypeChange { from, to }
            }
            PersonDomainError::MessagingNotFound(id) => PersonServiceError::MessagingNotFound(id),
            PersonDomainError::BatchValidationFailed { failed_ids, errors } => {
                PersonServiceError::BatchValidationFailed {
                    failed_ids_count: failed_ids.len(),
                    errors: errors.join(", "),
                }
            }
            PersonDomainError::RepositoryError(err) => {
                PersonServiceError::RepositoryError(err.to_string())
            }
        }
    }
}

#[async_trait]
impl<DB: Database + Send + Sync> PersonService for PersonServiceImpl<DB> {
    async fn create_person(
        &self,
        person: Person,
        audit_log: banking_api::domain::AuditLog,
    ) -> PersonServiceResult<Person> {
        let model = person.to_model();
        let saved_model = self
            .repositories
            .person_repository
            .save(model, audit_log.id)
            .await
            .map_err(Self::map_domain_error)?;
        Ok(saved_model.to_domain())
    }

    async fn find_person_by_id(&self, id: Uuid) -> PersonServiceResult<Option<Person>> {
        let model_idx = self
            .repositories
            .person_repository
            .find_by_id(id)
            .await
            .map_err(Self::map_domain_error)?;
        if let Some(idx) = model_idx {
            let model = self
                .repositories
                .person_repository
                .load(idx.person_id)
                .await
                .map_err(Self::map_domain_error)?;
            Ok(Some(model.to_domain()))
        } else {
            Ok(None)
        }
    }

    async fn get_persons_by_external_identifier(
        &self,
        external_identifier: HeaplessString<50>,
    ) -> PersonServiceResult<Vec<Person>> {
        let model_ixes = self
            .repositories
            .person_repository
            .get_by_external_identifier(external_identifier.as_str())
            .await
            .map_err(Self::map_domain_error)?;
        let mut persons = Vec::new();
        for idx in model_ixes {
            let person_model = self
                .repositories
                .person_repository
                .load(idx.person_id)
                .await
                .map_err(Self::map_domain_error)?;
            persons.push(person_model.to_domain());
        }
        Ok(persons)
    }
}