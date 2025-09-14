//! Enhanced PersonRepository trait with domain-specific error handling
//! 
//! This module demonstrates how to use domain-specific errors instead of
//! generic database errors for better error context and handling.

use crate::repository::errors::{PersonDomainError, PersonResult};
use banking_api::models::Person;
use uuid::Uuid;
use async_trait::async_trait;

/// Enhanced PersonRepository trait with domain-specific error handling
#[async_trait]
pub trait PersonRepositoryEnhanced: Send + Sync {
    /// Create a new person with validation
    /// 
    /// # Errors
    /// - `PersonDomainError::InvalidHierarchy` if parent/organization relationship is invalid
    /// - `PersonDomainError::DuplicateExternalId` if external_id already exists
    /// - `PersonDomainError::OrganizationNotFound` if specified organization doesn't exist
    async fn create(&self, person: &Person) -> PersonResult<Person>;
    
    /// Find person by ID
    /// 
    /// # Errors
    /// - `PersonDomainError::NotFound` if person doesn't exist
    async fn find_by_id(&self, id: Uuid) -> PersonResult<Person>;
    
    /// Find person by external ID
    /// 
    /// # Errors
    /// - `PersonDomainError::NotFound` if no person with this external_id exists
    async fn find_by_external_id(&self, external_id: &str) -> PersonResult<Person>;
    
    /// Update existing person
    /// 
    /// # Errors
    /// - `PersonDomainError::NotFound` if person doesn't exist
    /// - `PersonDomainError::InvalidHierarchy` if update would break hierarchy rules
    /// - `PersonDomainError::StaleData` if version doesn't match (optimistic locking)
    async fn update(&self, person: &Person) -> PersonResult<Person>;
    
    /// Delete person with cascade checking
    /// 
    /// # Errors
    /// - `PersonDomainError::NotFound` if person doesn't exist
    /// - `PersonDomainError::CascadeDeleteBlocked` if person has dependent records
    async fn delete(&self, id: Uuid) -> PersonResult<()>;
    
    /// Find all persons in an organization
    /// 
    /// # Errors
    /// - `PersonDomainError::OrganizationNotFound` if organization doesn't exist
    async fn find_by_organization(&self, org_id: Uuid) -> PersonResult<Vec<Person>>;
    
    /// Validate person hierarchy
    /// 
    /// # Errors
    /// - `PersonDomainError::InvalidHierarchy` with specific validation failure details
    async fn validate_hierarchy(&self, person: &Person) -> PersonResult<()>;
    
    /// Check for duplicate external IDs
    /// 
    /// # Errors
    /// - `PersonDomainError::DuplicateExternalId` if external_id is already in use
    async fn check_duplicate_external_id(&self, external_id: &str, exclude_id: Option<Uuid>) -> PersonResult<()>;
}

/// Example implementation showing error mapping
pub mod example_impl {
    use super::*;
    use sqlx::{PgPool, postgres::PgRow};
    
    pub struct PersonRepositoryImpl {
        pool: PgPool,
    }
    
    impl PersonRepositoryImpl {
        /// Map SQLx errors to domain errors
        fn map_sqlx_error(err: sqlx::Error, context: &str) -> PersonDomainError {
            match err {
                sqlx::Error::RowNotFound => PersonDomainError::NotFound(context.to_string()),
                sqlx::Error::Database(db_err) => {
                    // Check for specific database constraints
                    if let Some(constraint) = db_err.constraint() {
                        match constraint {
                            "person_external_id_unique" => {
                                PersonDomainError::DuplicateExternalId(context.to_string())
                            }
                            "person_organization_fk" => {
                                PersonDomainError::OrganizationNotFound(Uuid::nil())
                            }
                            _ => PersonDomainError::DatabaseError(db_err.to_string())
                        }
                    } else {
                        PersonDomainError::DatabaseError(db_err.to_string())
                    }
                }
                _ => PersonDomainError::DatabaseError(err.to_string())
            }
        }
    }
    
    #[async_trait]
    impl PersonRepositoryEnhanced for PersonRepositoryImpl {
        async fn create(&self, person: &Person) -> PersonResult<Person> {
            // Validate hierarchy first
            self.validate_hierarchy(person).await?;
            
            // Check for duplicate external ID
            if let Some(ref ext_id) = person.external_id {
                self.check_duplicate_external_id(ext_id, None).await?;
            }
            
            // Attempt insert
            let result = sqlx::query_as::<_, Person>(
                r#"
                INSERT INTO person (id, entity_type, external_id, organization_id, parent_id, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING *
                "#
            )
            .bind(person.id)
            .bind(&person.entity_type)
            .bind(&person.external_id)
            .bind(person.organization_id)
            .bind(person.parent_id)
            .bind(person.created_at)
            .bind(person.updated_at)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Self::map_sqlx_error(e, "Failed to create person"))?;
            
            Ok(result)
        }
        
        async fn find_by_id(&self, id: Uuid) -> PersonResult<Person> {
            sqlx::query_as::<_, Person>(
                "SELECT * FROM person WHERE id = $1"
            )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => PersonDomainError::NotFound(format!("Person with id {} not found", id)),
                _ => Self::map_sqlx_error(e, &format!("Failed to find person {}", id))
            })
        }
        
        async fn find_by_external_id(&self, external_id: &str) -> PersonResult<Person> {
            sqlx::query_as::<_, Person>(
                "SELECT * FROM person WHERE external_id = $1"
            )
            .bind(external_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => PersonDomainError::NotFound(
                    format!("Person with external_id '{}' not found", external_id)
                ),
                _ => Self::map_sqlx_error(e, &format!("Failed to find person by external_id '{}'", external_id))
            })
        }
        
        async fn update(&self, person: &Person) -> PersonResult<Person> {
            // Validate hierarchy if parent changed
            self.validate_hierarchy(person).await?;
            
            // Check version for optimistic locking
            let current = self.find_by_id(person.id).await?;
            if current.version != person.version {
                return Err(PersonDomainError::StaleData {
                    id: person.id,
                    expected_version: person.version,
                    actual_version: current.version,
                });
            }
            
            // Update with version increment
            let result = sqlx::query_as::<_, Person>(
                r#"
                UPDATE person 
                SET entity_type = $2, external_id = $3, organization_id = $4, 
                    parent_id = $5, updated_at = $6, version = version + 1
                WHERE id = $1 AND version = $7
                RETURNING *
                "#
            )
            .bind(person.id)
            .bind(&person.entity_type)
            .bind(&person.external_id)
            .bind(person.organization_id)
            .bind(person.parent_id)
            .bind(person.updated_at)
            .bind(person.version)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Self::map_sqlx_error(e, "Failed to update person"))?;
            
            Ok(result)
        }
        
        async fn delete(&self, id: Uuid) -> PersonResult<()> {
            // Check for dependent records
            let dependents: Vec<Uuid> = sqlx::query_scalar(
                "SELECT id FROM person WHERE parent_id = $1 LIMIT 10"
            )
            .bind(id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Self::map_sqlx_error(e, "Failed to check dependents"))?;
            
            if !dependents.is_empty() {
                return Err(PersonDomainError::CascadeDeleteBlocked(dependents));
            }
            
            // Perform delete
            let rows_affected = sqlx::query("DELETE FROM person WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| Self::map_sqlx_error(e, "Failed to delete person"))?
                .rows_affected();
            
            if rows_affected == 0 {
                return Err(PersonDomainError::NotFound(format!("Person {} not found", id)));
            }
            
            Ok(())
        }
        
        async fn find_by_organization(&self, org_id: Uuid) -> PersonResult<Vec<Person>> {
            // First check if organization exists
            let org_exists: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM person WHERE id = $1 AND entity_type = 'Organization')"
            )
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Self::map_sqlx_error(e, "Failed to check organization"))?;
            
            if !org_exists {
                return Err(PersonDomainError::OrganizationNotFound(org_id));
            }
            
            // Find all persons in organization
            sqlx::query_as::<_, Person>(
                "SELECT * FROM person WHERE organization_id = $1 ORDER BY created_at"
            )
            .bind(org_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Self::map_sqlx_error(e, "Failed to find persons by organization"))
        }
        
        async fn validate_hierarchy(&self, person: &Person) -> PersonResult<()> {
            // Check parent exists if specified
            if let Some(parent_id) = person.parent_id {
                let parent_exists: bool = sqlx::query_scalar(
                    "SELECT EXISTS(SELECT 1 FROM person WHERE id = $1)"
                )
                .bind(parent_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| Self::map_sqlx_error(e, "Failed to validate parent"))?;
                
                if !parent_exists {
                    return Err(PersonDomainError::InvalidHierarchy(
                        format!("Parent person {} does not exist", parent_id)
                    ));
                }
                
                // Check for circular references
                if parent_id == person.id {
                    return Err(PersonDomainError::InvalidHierarchy(
                        "Person cannot be its own parent".to_string()
                    ));
                }
            }
            
            // Check organization exists if specified
            if let Some(org_id) = person.organization_id {
                let org_exists: bool = sqlx::query_scalar(
                    "SELECT EXISTS(SELECT 1 FROM person WHERE id = $1 AND entity_type = 'Organization')"
                )
                .bind(org_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| Self::map_sqlx_error(e, "Failed to validate organization"))?;
                
                if !org_exists {
                    return Err(PersonDomainError::OrganizationNotFound(org_id));
                }
            }
            
            Ok(())
        }
        
        async fn check_duplicate_external_id(&self, external_id: &str, exclude_id: Option<Uuid>) -> PersonResult<()> {
            let query = match exclude_id {
                Some(id) => {
                    sqlx::query_scalar::<_, bool>(
                        "SELECT EXISTS(SELECT 1 FROM person WHERE external_id = $1 AND id != $2)"
                    )
                    .bind(external_id)
                    .bind(id)
                }
                None => {
                    sqlx::query_scalar::<_, bool>(
                        "SELECT EXISTS(SELECT 1 FROM person WHERE external_id = $1)"
                    )
                    .bind(external_id)
                }
            };
            
            let exists = query
                .fetch_one(&self.pool)
                .await
                .map_err(|e| Self::map_sqlx_error(e, "Failed to check duplicate external_id"))?;
            
            if exists {
                return Err(PersonDomainError::DuplicateExternalId(external_id.to_string()));
            }
            
            Ok(())
        }
    }
}