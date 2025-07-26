use async_trait::async_trait;
use uuid::Uuid;
use crate::models::referenced_person::ReferencedPersonModel;

#[async_trait]
pub trait ReferencedPersonRepository: Send + Sync {
    /// Create a new referenced person
    async fn create(&self, person: ReferencedPersonModel) -> Result<ReferencedPersonModel, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get a referenced person by ID
    async fn get_by_id(&self, person_id: Uuid) -> Result<Option<ReferencedPersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get referenced persons by external identifier
    async fn get_by_external_identifier(&self, identifier: &str) -> Result<Vec<ReferencedPersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get referenced persons by entity reference
    async fn get_by_entity_reference(&self, entity_id: Uuid, entity_type: &str) -> Result<Vec<ReferencedPersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find or create a referenced person by display name and type
    async fn find_or_create(&self, display_name: &str, person_type: crate::models::referenced_person::PersonType, external_identifier: Option<&str>) -> Result<ReferencedPersonModel, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Update a referenced person
    async fn update(&self, person: ReferencedPersonModel) -> Result<ReferencedPersonModel, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Mark a person as duplicate of another
    async fn mark_as_duplicate(&self, person_id: Uuid, duplicate_of: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all active referenced persons
    async fn get_all_active(&self) -> Result<Vec<ReferencedPersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Search referenced persons by display name
    async fn search_by_name(&self, query: &str) -> Result<Vec<ReferencedPersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Batch create referenced persons
    async fn batch_create(&self, persons: Vec<ReferencedPersonModel>) -> Result<Vec<ReferencedPersonModel>, Box<dyn std::error::Error + Send + Sync>>;
}