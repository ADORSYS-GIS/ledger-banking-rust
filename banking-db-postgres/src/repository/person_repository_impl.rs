use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_db::models::person::{PersonModel, PersonType};
use banking_db::repository::PersonRepository;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use heapless::String as HeaplessString;

pub struct PersonRepositoryImpl {
    pool: PgPool,
}

impl PersonRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Helper function to parse person type string
fn parse_person_type(type_str: &str) -> BankingResult<PersonType> {
    match type_str {
        "natural" => Ok(PersonType::Natural),
        "legal" => Ok(PersonType::Legal),
        "system" => Ok(PersonType::System),
        "integration" => Ok(PersonType::Integration),
        "unknown" => Ok(PersonType::Unknown),
        _ => Err(BankingError::ValidationError {
            field: "person_type".to_string(),
            message: format!("Invalid person type: {type_str}"),
        }),
    }
}


/// Helper trait to extract PersonModel from database row
trait TryFromRow<R> {
    fn try_from_row(row: &R) -> BankingResult<Self>
    where
        Self: Sized;
}

impl TryFromRow<sqlx::postgres::PgRow> for PersonModel {
    fn try_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<Self> {
        Ok(PersonModel {
            id: row.get("id"),
            person_type: parse_person_type(&row.get::<String, _>("person_type"))?,
            display_name: HeaplessString::try_from(
                row.get::<String, _>("display_name").as_str()
            ).map_err(|_| BankingError::ValidationError {
                field: "display_name".to_string(),
                message: "Display name too long".to_string(),
            })?,
            external_identifier: match row.get::<Option<String>, _>("external_identifier") {
                Some(id) => Some(HeaplessString::try_from(id.as_str()).map_err(|_| {
                    BankingError::ValidationError {
                        field: "external_identifier".to_string(),
                        message: "External identifier too long".to_string(),
                    }
                })?),
                None => None,
            },
            organization: row.get("organization"),
            messaging1_id: row.get("messaging1_id"),
            messaging1_type: row.get("messaging1_type"),
            messaging2_id: row.get("messaging2_id"),
            messaging2_type: row.get("messaging2_type"),
            messaging3_id: row.get("messaging3_id"),
            messaging3_type: row.get("messaging3_type"),
            messaging4_id: row.get("messaging4_id"),
            messaging4_type: row.get("messaging4_type"),
            messaging5_id: row.get("messaging5_id"),
            messaging5_type: row.get("messaging5_type"),
            department: match row.get::<Option<String>, _>("department") {
                Some(dept) => Some(HeaplessString::try_from(dept.as_str()).map_err(|_| {
                    BankingError::ValidationError {
                        field: "department".to_string(),
                        message: "Department too long".to_string(),
                    }
                })?),
                None => None,
            },
            location: row.get("location"),
            duplicate_of: row.get("duplicate_of"),
            is_active: row.get("is_active"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[async_trait]
impl PersonRepository for PersonRepositoryImpl {
    async fn create(&self, person: PersonModel) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>> {
        let result = sqlx::query(
            r#"
            INSERT INTO persons (
                id, person_type, display_name, external_identifier, organization,
                messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                department, location, duplicate_of, is_active, created_at, updated_at
            )
            VALUES (
                $1, $2::person_type, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21
            )
            RETURNING id, person_type::text as person_type, display_name, external_identifier, organization,
                     messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                     messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                     department, location, duplicate_of, is_active, created_at, updated_at
            "#
        )
        .bind(person.id)
        .bind(person.person_type.to_string())
        .bind(person.display_name.as_str())
        .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
        .bind(person.organization)
        .bind(person.messaging1_id)
        .bind(person.messaging1_type)
        .bind(person.messaging2_id)
        .bind(person.messaging2_type)
        .bind(person.messaging3_id)
        .bind(person.messaging3_type)
        .bind(person.messaging4_id)
        .bind(person.messaging4_type)
        .bind(person.messaging5_id)
        .bind(person.messaging5_type)
        .bind(person.department.as_ref().map(|s| s.as_str()))
        .bind(person.location)
        .bind(person.duplicate_of)
        .bind(person.is_active)
        .bind(person.created_at)
        .bind(person.updated_at)
        .fetch_one(&self.pool)
        .await?;

        PersonModel::try_from_row(&result).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<PersonModel>, Box<dyn std::error::Error + Send + Sync>> {
        let result = sqlx::query(
            r#"
            SELECT id, person_type::text as person_type, display_name, external_identifier, organization,
                   messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                   messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                   department, location, duplicate_of, is_active, created_at, updated_at
            FROM persons 
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => {
                let person = PersonModel::try_from_row(&row).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(Some(person))
            }
            None => Ok(None),
        }
    }

    async fn get_by_external_identifier(&self, identifier: &str) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>> {
        let results = sqlx::query(
            r#"
            SELECT id, person_type::text as person_type, display_name, external_identifier, organization,
                   messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                   messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                   department, location, duplicate_of, is_active, created_at, updated_at
            FROM persons 
            WHERE external_identifier = $1 AND is_active = true
            ORDER BY created_at DESC
            "#
        )
        .bind(identifier)
        .fetch_all(&self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in results {
            let person = PersonModel::try_from_row(&row).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            persons.push(person);
        }
        Ok(persons)
    }

    async fn get_by_entity_reference(&self, entity_id: Uuid, entity_type: &str) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>> {
        let results = sqlx::query(
            r#"
            SELECT p.id, p.person_type::text as person_type, p.display_name, p.external_identifier, p.organization,
                   p.messaging1_id, p.messaging1_type, p.messaging2_id, p.messaging2_type, p.messaging3_id, p.messaging3_type,
                   p.messaging4_id, p.messaging4_type, p.messaging5_id, p.messaging5_type,
                   p.department, p.location, p.duplicate_of, p.is_active, p.created_at, p.updated_at
            FROM persons p
            INNER JOIN entity_references er ON p.id = er.person_id
            WHERE er.reference_external_id = $1 AND er.entity_role = $2 AND er.is_active = true AND p.is_active = true
            ORDER BY p.created_at DESC
            "#
        )
        .bind(entity_id.to_string())
        .bind(entity_type)
        .fetch_all(&self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in results {
            let person = PersonModel::try_from_row(&row).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            persons.push(person);
        }
        Ok(persons)
    }

    async fn find_or_create(&self, display_name: &str, person_type: PersonType, external_identifier: Option<&str>) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>> {
        // First try to find existing person
        if let Some(ext_id) = external_identifier {
            let existing = self.get_by_external_identifier(ext_id).await?;
            if let Some(person) = existing.into_iter().find(|p| p.person_type == person_type) {
                return Ok(person);
            }
        }

        // If not found, create new person
        let new_person = PersonModel {
            id: Uuid::new_v4(),
            person_type,
            display_name: HeaplessString::try_from(display_name).map_err(|_| {
                BankingError::ValidationError {
                    field: "display_name".to_string(),
                    message: "Display name too long".to_string(),
                }
            })?,
            external_identifier: external_identifier.map(|id| {
                HeaplessString::try_from(id).map_err(|_| {
                    BankingError::ValidationError {
                        field: "external_identifier".to_string(),
                        message: "External identifier too long".to_string(),
                    }
                })
            }).transpose()?,
            organization: None,
            messaging1_id: None,
            messaging1_type: None,
            messaging2_id: None,
            messaging2_type: None,
            messaging3_id: None,
            messaging3_type: None,
            messaging4_id: None,
            messaging4_type: None,
            messaging5_id: None,
            messaging5_type: None,
            department: None,
            location: None,
            duplicate_of: None,
            is_active: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.create(new_person).await
    }

    async fn update(&self, person: PersonModel) -> Result<PersonModel, Box<dyn std::error::Error + Send + Sync>> {
        let result = sqlx::query(
            r#"
            UPDATE persons SET
                person_type = $2::person_type,
                display_name = $3,
                external_identifier = $4,
                organization = $5,
                messaging1_id = $6,
                messaging1_type = $7,
                messaging2_id = $8,
                messaging2_type = $9,
                messaging3_id = $10,
                messaging3_type = $11,
                messaging4_id = $12,
                messaging4_type = $13,
                messaging5_id = $14,
                messaging5_type = $15,
                department = $16,
                location = $17,
                duplicate_of = $18,
                is_active = $19,
                updated_at = $20
            WHERE id = $1
            RETURNING id, person_type::text as person_type, display_name, external_identifier, organization,
                     messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                     messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                     department, location, duplicate_of, is_active, created_at, updated_at
            "#
        )
        .bind(person.id)
        .bind(person.person_type.to_string())
        .bind(person.display_name.as_str())
        .bind(person.external_identifier.as_ref().map(|s| s.as_str()))
        .bind(person.organization)
        .bind(person.messaging1_id)
        .bind(person.messaging1_type)
        .bind(person.messaging2_id)
        .bind(person.messaging2_type)
        .bind(person.messaging3_id)
        .bind(person.messaging3_type)
        .bind(person.messaging4_id)
        .bind(person.messaging4_type)
        .bind(person.messaging5_id)
        .bind(person.messaging5_type)
        .bind(person.department.as_ref().map(|s| s.as_str()))
        .bind(person.location)
        .bind(person.duplicate_of)
        .bind(person.is_active)
        .bind(person.updated_at)
        .fetch_one(&self.pool)
        .await?;

        PersonModel::try_from_row(&result).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    async fn mark_as_duplicate(&self, id: Uuid, duplicate_of: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlx::query(
            r#"
            UPDATE persons SET
                duplicate_of = $2,
                is_active = false,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(duplicate_of)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_all_active(&self) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>> {
        let results = sqlx::query(
            r#"
            SELECT id, person_type::text as person_type, display_name, external_identifier, organization,
                   messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                   messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                   department, location, duplicate_of, is_active, created_at, updated_at
            FROM persons 
            WHERE is_active = true AND duplicate_of IS NULL
            ORDER BY display_name
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in results {
            let person = PersonModel::try_from_row(&row).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            persons.push(person);
        }
        Ok(persons)
    }

    async fn search_by_name(&self, query: &str) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>> {
        let search_pattern = format!("%{query}%");
        let results = sqlx::query(
            r#"
            SELECT id, person_type::text as person_type, display_name, external_identifier, organization,
                   messaging1_id, messaging1_type, messaging2_id, messaging2_type, messaging3_id, messaging3_type,
                   messaging4_id, messaging4_type, messaging5_id, messaging5_type,
                   department, location, duplicate_of, is_active, created_at, updated_at
            FROM persons 
            WHERE display_name ILIKE $1 AND is_active = true AND duplicate_of IS NULL
            ORDER BY display_name
            LIMIT 50
            "#
        )
        .bind(&search_pattern)
        .fetch_all(&self.pool)
        .await?;

        let mut persons = Vec::new();
        for row in results {
            let person = PersonModel::try_from_row(&row).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            persons.push(person);
        }
        Ok(persons)
    }

    async fn batch_create(&self, persons: Vec<PersonModel>) -> Result<Vec<PersonModel>, Box<dyn std::error::Error + Send + Sync>> {
        let mut created_persons = Vec::new();
        
        for person in persons {
            let created = self.create(person).await?;
            created_persons.push(created);
        }
        
        Ok(created_persons)
    }
}

#[cfg(feature = "postgres_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://ledger_user:ledger_password@localhost:5432/ledger_banking".to_string());
        
        PgPool::connect(&database_url).await.expect("Failed to connect to test database")
    }

    fn create_test_person() -> PersonModel {
        PersonModel {
            id: Uuid::new_v4(),
            person_type: PersonType::Natural,
            display_name: HeaplessString::try_from("John Doe").unwrap(),
            external_identifier: Some(HeaplessString::try_from("EXT001").unwrap()),
            organization: None,
            messaging1_id: None,
            messaging1_type: None,
            messaging2_id: None,
            messaging2_type: None,
            messaging3_id: None,
            messaging3_type: None,
            messaging4_id: None,
            messaging4_type: None,
            messaging5_id: None,
            messaging5_type: None,
            department: Some(HeaplessString::try_from("IT").unwrap()),
            location: None,
            duplicate_of: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_person() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let test_person = create_test_person();

        let result = repo.create(test_person.clone()).await;
        assert!(result.is_ok());
        
        let created = result.unwrap();
        assert_eq!(created.id, test_person.id);
        assert_eq!(created.display_name, test_person.display_name);
        assert_eq!(created.person_type, test_person.person_type);
    }

    #[tokio::test]
    async fn test_get_by_id() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let test_person = create_test_person();

        // Create person first
        let created = repo.create(test_person.clone()).await.expect("Failed to create person");
        
        // Retrieve by ID
        let result = repo.get_by_id(created.id).await;
        assert!(result.is_ok());
        
        let retrieved = result.unwrap();
        assert!(retrieved.is_some());
        let person = retrieved.unwrap();
        assert_eq!(person.id, created.id);
        assert_eq!(person.display_name, created.display_name);
    }

    #[tokio::test]
    async fn test_get_by_external_identifier() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let unique_id = Uuid::new_v4();
        let external_id = format!("EXT{}", &unique_id.to_string()[0..6]);
        
        let mut test_person = create_test_person();
        test_person.external_identifier = Some(HeaplessString::try_from(external_id.as_str()).unwrap());

        // Create person first
        let _created = repo.create(test_person.clone()).await.expect("Failed to create person");
        
        // Retrieve by external identifier
        let result = repo.get_by_external_identifier(&external_id).await;
        assert!(result.is_ok());
        
        let persons = result.unwrap();
        assert!(!persons.is_empty());
        assert_eq!(persons[0].external_identifier.as_ref().unwrap().as_str(), external_id);
    }

    #[tokio::test]
    async fn test_find_or_create_existing() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let unique_id = Uuid::new_v4();
        let external_id = format!("EXT{}", &unique_id.to_string()[0..6]);
        
        let mut test_person = create_test_person();
        test_person.external_identifier = Some(HeaplessString::try_from(external_id.as_str()).unwrap());

        // Create person first
        let created = repo.create(test_person.clone()).await.expect("Failed to create person");
        
        // Try to find or create - should find existing
        let result = repo.find_or_create("John Doe", PersonType::Natural, Some(&external_id)).await;
        assert!(result.is_ok());
        
        let found = result.unwrap();
        assert_eq!(found.id, created.id);
    }

    #[tokio::test]
    async fn test_find_or_create_new() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let unique_id = Uuid::new_v4();
        let external_id = format!("NEW{}", &unique_id.to_string()[0..6]);
        
        // Try to find or create - should create new
        let result = repo.find_or_create("Jane Smith", PersonType::Legal, Some(&external_id)).await;
        assert!(result.is_ok());
        
        let created = result.unwrap();
        assert_eq!(created.display_name.as_str(), "Jane Smith");
        assert_eq!(created.person_type, PersonType::Legal);
        assert_eq!(created.external_identifier.as_ref().unwrap().as_str(), external_id);
    }

    #[tokio::test]
    async fn test_update_person() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let test_person = create_test_person();

        // Create person first
        let created = repo.create(test_person.clone()).await.expect("Failed to create person");
        
        // Update person
        let mut updated_person = created.clone();
        updated_person.display_name = HeaplessString::try_from("John Updated").unwrap();
        updated_person.department = Some(HeaplessString::try_from("HR").unwrap());
        updated_person.updated_at = Utc::now();

        let result = repo.update(updated_person.clone()).await;
        assert!(result.is_ok());
        
        let updated = result.unwrap();
        assert_eq!(updated.display_name.as_str(), "John Updated");
        assert_eq!(updated.department.as_ref().unwrap().as_str(), "HR");
    }

    #[tokio::test]
    async fn test_mark_as_duplicate() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        
        // Create two persons
        let person1 = create_test_person();
        let created1 = repo.create(person1).await.expect("Failed to create person1");
        
        let mut person2 = create_test_person();
        person2.id = Uuid::new_v4();
        person2.external_identifier = Some(HeaplessString::try_from("EXT002").unwrap());
        let created2 = repo.create(person2).await.expect("Failed to create person2");
        
        // Mark person2 as duplicate of person1
        let result = repo.mark_as_duplicate(created2.id, created1.id).await;
        assert!(result.is_ok());
        
        // Verify person2 is marked as duplicate
        let retrieved = repo.get_by_id(created2.id).await.expect("Failed to get person");
        assert!(retrieved.is_some());
        let person = retrieved.unwrap();
        assert_eq!(person.duplicate_of, Some(created1.id));
        assert!(!person.is_active);
    }

    #[tokio::test]
    async fn test_search_by_name() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        let unique_id = Uuid::new_v4();
        let unique_name = format!("SearchTest{}", &unique_id.to_string()[0..6]);
        
        let mut test_person = create_test_person();
        test_person.display_name = HeaplessString::try_from(unique_name.as_str()).unwrap();

        // Create person first
        let created = repo.create(test_person.clone()).await.expect("Failed to create person");
        
        // Search by partial name
        let result = repo.search_by_name("SearchTest").await;
        assert!(result.is_ok());
        
        let persons = result.unwrap();
        assert!(!persons.is_empty());
        let found = persons.iter().find(|p| p.id == created.id);
        assert!(found.is_some());
    }

    #[tokio::test]
    async fn test_get_all_active() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        
        let result = repo.get_all_active().await;
        assert!(result.is_ok());
        
        let persons = result.unwrap();
        // Should return only active persons that are not duplicates
        for person in persons {
            assert!(person.is_active);
            assert!(person.duplicate_of.is_none());
        }
    }

    #[tokio::test]
    async fn test_batch_create() {
        let pool = setup_test_db().await;
        let repo = PersonRepositoryImpl::new(pool);
        
        let person1 = create_test_person();
        let mut person2 = create_test_person();
        person2.id = Uuid::new_v4();
        person2.display_name = HeaplessString::try_from("Jane Doe").unwrap();
        person2.external_identifier = Some(HeaplessString::try_from("EXT003").unwrap());
        
        let persons = vec![person1, person2];
        let result = repo.batch_create(persons.clone()).await;
        assert!(result.is_ok());
        
        let _created = result.unwrap();
        assert_eq!(_created.len(), 2);
        assert_eq!(_created[0].id, persons[0].id);
        assert_eq!(_created[1].id, persons[1].id);
    }
}