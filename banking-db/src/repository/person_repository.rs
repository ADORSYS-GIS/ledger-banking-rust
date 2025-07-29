use async_trait::async_trait;
use uuid::Uuid;
use crate::models::person::PersonModel;

#[async_trait]
pub trait PersonRepository: Send + Sync {
    /// Create a new person
    async fn create(&self, person: PersonModel) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get a person by ID
    async fn get_by_id(&self, person_id: Uuid) -> Result<Option<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get persons by external identifier
    async fn get_by_external_identifier(&self, identifier: &str) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get persons by entity reference
    async fn get_by_entity_reference(&self, entity_id: Uuid, entity_type: &str) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find or create a person by display name and type
    async fn find_or_create(&self, display_name: &str, person_type: crate::models::person::PersonType, external_identifier: Option<&str>) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Update a person
    async fn update(&self, person: PersonModel) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Mark a person as duplicate of another
    async fn mark_as_duplicate(&self, person_id: Uuid, duplicate_of: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get all active persons
    async fn get_all_active(&self) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Search persons by display name
    async fn search_by_name(&self, query: &str) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Batch create persons
    async fn batch_create(&self, persons: Vec<PersonModel>) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>>;
}