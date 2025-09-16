use async_trait::async_trait;
use sqlx::Database;
use std::error::Error;
use std::fmt;
use uuid::Uuid;

use crate::models::person::{PersonIdxModel, PersonModel};

/// Domain-specific errors for Person repository operations
#[derive(Debug)]
pub enum PersonRepositoryError {
    /// Invalid organizational hierarchy (e.g., circular reference)
    InvalidHierarchy(String),
    /// Duplicate external identifier constraint violation
    DuplicateExternalId(String),
    /// Person already exists
    AlreadyExists(String),
    /// A list of persons that already exist
    ManyPersonsExists(Vec<Uuid>),
    /// Cannot delete due to dependent records
    CascadeDeleteBlocked(Vec<Uuid>),
    /// Organization not found for person
    OrganizationNotFound(Uuid),
    /// A list of organizations that were not found
    ManyOrganizationsNotFound(Vec<Uuid>),
    /// Location not found for person
    LocationNotFound(Uuid),
    /// Referenced person not found (for duplicate_of)
    DuplicatePersonNotFound(Uuid),
    InvalidLocations(Vec<Uuid>),
    /// A list of persons that were not found
    ManyPersonsNotFound(Vec<Uuid>),
    /// Person is a duplicate for other persons
    IsDuplicatePersonFor(Vec<Uuid>),
    /// Person is an organization for other persons
    IsOrganizationPersonFor(Vec<Uuid>),
    /// Invalid person type transition
    InvalidPersonTypeChange { from: String, to: String },
    /// Messaging reference not found
    MessagingNotFound(Uuid),
    /// Batch operation validation failed
    BatchValidationFailed {
        failed_ids: Vec<Uuid>,
        errors: Vec<String>,
    },
    /// Generic repository error (wraps database errors)
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for PersonRepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHierarchy(msg) => write!(f, "Invalid organizational hierarchy: {msg}"),
            Self::DuplicateExternalId(id) => write!(f, "Duplicate external identifier: {id}"),
            Self::AlreadyExists(msg) => write!(f, "{msg}"),
            Self::ManyPersonsExists(ids) => write!(
                f,
                "Persons already exist: {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::CascadeDeleteBlocked(ids) => {
                write!(f, "Cannot delete: {} dependent records exist", ids.len())
            }
            Self::OrganizationNotFound(id) => write!(f, "Organization not found: {id}"),
            Self::ManyOrganizationsNotFound(ids) => write!(
                f,
                "Organizations not found: {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::LocationNotFound(id) => write!(f, "Location not found: {id}"),
            Self::DuplicatePersonNotFound(id) => {
                write!(f, "Referenced duplicate person not found: {id}")
            }
            Self::InvalidLocations(ids) => {
                write!(
                    f,
                    "Invalid locations: {}",
                    ids.iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Self::IsDuplicatePersonFor(ids) => write!(
                f,
                "Person is a duplicate for others: {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::IsOrganizationPersonFor(ids) => write!(
                f,
                "Person is an organization for others: {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::ManyPersonsNotFound(ids) => write!(
                f,
                "Persons not found: {}",
                ids.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::InvalidPersonTypeChange { from, to } => {
                write!(f, "Invalid person type change from {from} to {to}")
            }
            Self::MessagingNotFound(id) => write!(f, "Messaging reference not found: {id}"),
            Self::BatchValidationFailed { failed_ids, errors } => write!(
                f,
                "Batch validation failed for {} records: {}",
                failed_ids.len(),
                errors.join(", ")
            ),
            Self::RepositoryError(err) => write!(f, "Repository error: {err}"),
        }
    }
}

impl Error for PersonRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for PersonRepositoryError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) => {
                let message = db_err.message();
                if message.contains("duplicate key") && message.contains("external_identifier") {
                    return Self::DuplicateExternalId(
                        extract_constraint_value(message).unwrap_or_else(|| "unknown".to_string()),
                    );
                }
                if message.contains("foreign key") {
                    if message.contains("organization_person_id") {
                        if let Some(id) = extract_uuid_from_message(message) {
                            return Self::OrganizationNotFound(id);
                        }
                    }
                    if message.contains("location_id") {
                        if let Some(id) = extract_uuid_from_message(message) {
                            return Self::LocationNotFound(id);
                        }
                    }
                    if message.contains("duplicate_of_person_id") {
                        if let Some(id) = extract_uuid_from_message(message) {
                            return Self::DuplicatePersonNotFound(id);
                        }
                    }
                }
                if message.contains("would violate") && message.contains("cascade") {
                    let dependent_ids = extract_dependent_ids(message);
                    if !dependent_ids.is_empty() {
                        return Self::CascadeDeleteBlocked(dependent_ids);
                    }
                }
            }
            sqlx::Error::RowNotFound => return Self::RepositoryError(Box::new(err)),
            _ => {}
        }
        Self::RepositoryError(Box::new(err))
    }
}

fn extract_constraint_value(message: &str) -> Option<String> {
    if let Some(start) = message.find("=(") {
        if let Some(end) = message[start + 2..].find(')') {
            return Some(message[start + 2..start + 2 + end].to_string());
        }
    }
    None
}

fn extract_uuid_from_message(message: &str) -> Option<Uuid> {
    for word in message.split_whitespace() {
        if let Ok(uuid) = word.parse::<Uuid>() {
            return Some(uuid);
        }
    }
    None
}

fn extract_dependent_ids(message: &str) -> Vec<Uuid> {
    message
        .split_whitespace()
        .filter_map(|word| word.parse::<Uuid>().ok())
        .collect()
}

/// Result type using PersonRepositoryError
pub type PersonResult<T> = Result<T, PersonRepositoryError>;

#[async_trait]
pub trait PersonRepository<DB: Database>: Send + Sync {
    async fn save(&self, person: PersonModel, audit_log_id: Uuid) -> PersonResult<PersonModel>;
    async fn load(&self, id: Uuid) -> PersonResult<PersonModel>;
    async fn find_by_id(&self, id: Uuid) -> PersonResult<Option<PersonIdxModel>>;
    async fn find_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<PersonIdxModel>>;
    async fn exists_by_id(&self, id: Uuid) -> PersonResult<bool>;
    async fn exist_by_ids(&self, ids: &[Uuid]) -> PersonResult<Vec<(Uuid, bool)>>;
    async fn get_ids_by_external_identifier(&self, identifier: &str) -> PersonResult<Vec<Uuid>>;
    async fn get_by_external_identifier(&self, identifier: &str) -> PersonResult<Vec<PersonIdxModel>>;
    async fn find_by_duplicate_of_person_id(&self, person_id: Uuid) -> PersonResult<Vec<PersonIdxModel>>;
    async fn find_by_organization_person_id(&self, person_id: Uuid) -> PersonResult<Vec<PersonIdxModel>>;
}