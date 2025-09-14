//! Domain-specific error types for repository operations
//! 
//! This module provides rich error types that give context about failures
//! rather than exposing raw database errors to the domain layer.

use std::error::Error;
use std::fmt;
use uuid::Uuid;

/// Domain-specific errors for Person repository operations
#[derive(Debug)]
pub enum PersonDomainError {
    /// Invalid organizational hierarchy (e.g., circular reference)
    InvalidHierarchy(String),
    
    /// Duplicate external identifier constraint violation
    DuplicateExternalId(String),
    
    /// Cannot delete due to dependent records
    CascadeDeleteBlocked(Vec<Uuid>),
    
    /// Organization not found for person
    OrganizationNotFound(Uuid),
    
    /// Location not found for person
    LocationNotFound(Uuid),
    
    /// Referenced person not found (for duplicate_of)
    DuplicatePersonNotFound(Uuid),
    
    /// Invalid person type transition
    InvalidPersonTypeChange { 
        from: String, 
        to: String 
    },
    
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

impl fmt::Display for PersonDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHierarchy(msg) => {
                write!(f, "Invalid organizational hierarchy: {}", msg)
            }
            Self::DuplicateExternalId(id) => {
                write!(f, "Duplicate external identifier: {}", id)
            }
            Self::CascadeDeleteBlocked(ids) => {
                write!(f, "Cannot delete: {} dependent records exist", ids.len())
            }
            Self::OrganizationNotFound(id) => {
                write!(f, "Organization not found: {}", id)
            }
            Self::LocationNotFound(id) => {
                write!(f, "Location not found: {}", id)
            }
            Self::DuplicatePersonNotFound(id) => {
                write!(f, "Referenced duplicate person not found: {}", id)
            }
            Self::InvalidPersonTypeChange { from, to } => {
                write!(f, "Invalid person type change from {} to {}", from, to)
            }
            Self::MessagingNotFound(id) => {
                write!(f, "Messaging reference not found: {}", id)
            }
            Self::BatchValidationFailed { failed_ids, errors } => {
                write!(
                    f, 
                    "Batch validation failed for {} records: {}", 
                    failed_ids.len(),
                    errors.join(", ")
                )
            }
            Self::RepositoryError(err) => {
                write!(f, "Repository error: {}", err)
            }
        }
    }
}

impl Error for PersonDomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for PersonDomainError {
    fn from(err: sqlx::Error) -> Self {
        // Map specific database errors to domain errors
        match &err {
            sqlx::Error::Database(db_err) => {
                let message = db_err.message();
                
                // PostgreSQL constraint violations
                if message.contains("duplicate key") {
                    if message.contains("external_identifier") {
                        // Extract the duplicate value if possible
                        return Self::DuplicateExternalId(
                            extract_constraint_value(message)
                                .unwrap_or_else(|| "unknown".to_string())
                        );
                    }
                }
                
                if message.contains("foreign key") {
                    if message.contains("organization_person_id") {
                        // Extract the ID if possible
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
                    // Extract dependent IDs if possible
                    let dependent_ids = extract_dependent_ids(message);
                    if !dependent_ids.is_empty() {
                        return Self::CascadeDeleteBlocked(dependent_ids);
                    }
                }
            }
            sqlx::Error::RowNotFound => {
                // This is typically handled at a higher level
                // but we can provide a generic message
                return Self::RepositoryError(Box::new(err));
            }
            _ => {}
        }
        
        // Default to wrapping the error
        Self::RepositoryError(Box::new(err))
    }
}

/// Domain-specific errors for Location repository operations
#[derive(Debug)]
pub enum LocationDomainError {
    /// Locality not found
    LocalityNotFound(Uuid),
    
    /// Invalid location type
    InvalidLocationType(String),
    
    /// Invalid coordinates
    InvalidCoordinates { 
        latitude: f64, 
        longitude: f64 
    },
    
    /// Duplicate location for same address
    DuplicateLocation {
        street: String,
        locality_id: Uuid,
    },
    
    /// Generic repository error
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for LocationDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LocalityNotFound(id) => {
                write!(f, "Locality not found: {}", id)
            }
            Self::InvalidLocationType(loc_type) => {
                write!(f, "Invalid location type: {}", loc_type)
            }
            Self::InvalidCoordinates { latitude, longitude } => {
                write!(f, "Invalid coordinates: ({}, {})", latitude, longitude)
            }
            Self::DuplicateLocation { street, locality_id } => {
                write!(f, "Duplicate location: {} in locality {}", street, locality_id)
            }
            Self::RepositoryError(err) => {
                write!(f, "Repository error: {}", err)
            }
        }
    }
}

impl Error for LocationDomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for LocationDomainError {
    fn from(err: sqlx::Error) -> Self {
        // Map specific database errors to domain errors
        match &err {
            sqlx::Error::Database(db_err) => {
                let message = db_err.message();
                
                if message.contains("foreign key") && message.contains("locality_id") {
                    if let Some(id) = extract_uuid_from_message(message) {
                        return Self::LocalityNotFound(id);
                    }
                }
                
                if message.contains("duplicate") && message.contains("location") {
                    // Try to extract details from the error message
                    return Self::DuplicateLocation {
                        street: extract_constraint_value(message)
                            .unwrap_or_else(|| "unknown".to_string()),
                        locality_id: extract_uuid_from_message(message)
                            .unwrap_or_default(),
                    };
                }
            }
            _ => {}
        }
        
        Self::RepositoryError(Box::new(err))
    }
}

/// Domain-specific errors for Audit repository operations
#[derive(Debug)]
pub enum AuditDomainError {
    /// Invalid audit type
    InvalidAuditType(String),
    
    /// Audit record already exists
    DuplicateAuditRecord {
        entity_id: Uuid,
        version: i32,
    },
    
    /// Invalid version sequence
    InvalidVersionSequence {
        expected: i32,
        actual: i32,
    },
    
    /// Hash mismatch (potential tampering)
    HashMismatch {
        entity_id: Uuid,
        version: i32,
    },
    
    /// Generic repository error
    RepositoryError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for AuditDomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAuditType(audit_type) => {
                write!(f, "Invalid audit type: {}", audit_type)
            }
            Self::DuplicateAuditRecord { entity_id, version } => {
                write!(f, "Duplicate audit record for entity {} version {}", entity_id, version)
            }
            Self::InvalidVersionSequence { expected, actual } => {
                write!(f, "Invalid version sequence: expected {}, got {}", expected, actual)
            }
            Self::HashMismatch { entity_id, version } => {
                write!(f, "Hash mismatch for entity {} version {} - potential tampering detected", entity_id, version)
            }
            Self::RepositoryError(err) => {
                write!(f, "Repository error: {}", err)
            }
        }
    }
}

impl Error for AuditDomainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RepositoryError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}

// Helper functions for extracting information from error messages

fn extract_constraint_value(message: &str) -> Option<String> {
    // Try to extract value from messages like "Key (external_identifier)=(VALUE) already exists"
    if let Some(start) = message.find("=(") {
        if let Some(end) = message[start+2..].find(')') {
            return Some(message[start+2..start+2+end].to_string());
        }
    }
    None
}

fn extract_uuid_from_message(message: &str) -> Option<Uuid> {
    // Simple UUID extraction - looks for UUID pattern in message
    // This is a simplified implementation
    let uuid_pattern = r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}";
    
    // For simplicity, we'll just try to parse any UUID-like string in the message
    // In production, you'd want a proper regex
    for word in message.split_whitespace() {
        if let Ok(uuid) = word.parse::<Uuid>() {
            return Some(uuid);
        }
    }
    None
}

fn extract_dependent_ids(message: &str) -> Vec<Uuid> {
    // Extract UUIDs from cascade delete error messages
    let mut ids = Vec::new();
    
    for word in message.split_whitespace() {
        if let Ok(uuid) = word.parse::<Uuid>() {
            ids.push(uuid);
        }
    }
    
    ids
}

/// Result type using PersonDomainError
pub type PersonResult<T> = Result<T, PersonDomainError>;

/// Result type using LocationDomainError  
pub type LocationResult<T> = Result<T, LocationDomainError>;

/// Result type using AuditDomainError
pub type AuditResult<T> = Result<T, AuditDomainError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_domain_error_display() {
        let err = PersonDomainError::DuplicateExternalId("EMP001".to_string());
        assert_eq!(err.to_string(), "Duplicate external identifier: EMP001");
        
        let err = PersonDomainError::OrganizationNotFound(Uuid::nil());
        assert_eq!(err.to_string(), "Organization not found: 00000000-0000-0000-0000-000000000000");
        
        let err = PersonDomainError::CascadeDeleteBlocked(vec![Uuid::nil(), Uuid::nil()]);
        assert_eq!(err.to_string(), "Cannot delete: 2 dependent records exist");
    }
    
    #[test]
    fn test_location_domain_error_display() {
        let err = LocationDomainError::InvalidCoordinates { 
            latitude: 91.0, 
            longitude: 181.0 
        };
        assert_eq!(err.to_string(), "Invalid coordinates: (91, 181)");
    }
    
    #[test]
    fn test_audit_domain_error_display() {
        let err = AuditDomainError::InvalidVersionSequence { 
            expected: 5, 
            actual: 7 
        };
        assert_eq!(err.to_string(), "Invalid version sequence: expected 5, got 7");
        
        let err = AuditDomainError::HashMismatch { 
            entity_id: Uuid::nil(), 
            version: 3 
        };
        assert!(err.to_string().contains("potential tampering detected"));
    }
}