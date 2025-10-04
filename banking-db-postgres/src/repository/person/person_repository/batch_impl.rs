use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use async_trait::async_trait;
use banking_db::models::person::{PersonModel};
use banking_db::repository::{
    BatchOperationStats, BatchRepository, BatchResult, PersonRepository,
};
use sqlx::Postgres;
use std::error::Error;
use std::time::Instant;
use uuid::Uuid;

#[async_trait]
impl BatchRepository<Postgres, PersonModel> for PersonRepositoryImpl {
    async fn create_batch(
        &self,
        items: Vec<PersonModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::person_repository::create_batch::create_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn load_batch(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<Option<PersonModel>>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::person_repository::load_batch::load_batch(self, ids).await
    }

    async fn update_batch(
        &self,
        items: Vec<PersonModel>,
        audit_log_id: Uuid,
    ) -> Result<Vec<PersonModel>, Box<dyn Error + Send + Sync>> {
        crate::repository::person::person_repository::update_batch::update_batch(
            self,
            items,
            audit_log_id,
        )
        .await
    }

    async fn delete_batch(&self, ids: &[Uuid]) -> Result<usize, Box<dyn Error + Send + Sync>> {
        crate::repository::person::person_repository::delete_batch::delete_batch(self, ids).await
    }
}

impl PersonRepositoryImpl {
    pub async fn create_batch_chunked(
        &self,
        items: Vec<PersonModel>,
        audit_log_id: Uuid,
        chunk_size: usize,
    ) -> Result<BatchResult<PersonModel>, Box<dyn Error + Send + Sync>> {
        let mut stats = BatchOperationStats {
            total_items: items.len(),
            ..Default::default()
        };
        let start_time = Instant::now();
        let mut saved_items = Vec::new();

        for chunk in items.chunks(chunk_size) {
            match self.create_batch(chunk.to_vec(), audit_log_id).await {
                Ok(result) => {
                    stats.successful_items += result.len();
                    saved_items.extend(result);
                }
                Err(_) => {
                    stats.failed_items += chunk.len();
                }
            }
        }

        stats.duration_ms = start_time.elapsed().as_millis() as u64;
        Ok(BatchResult {
            stats,
            items: saved_items,
            errors: Vec::new(),
        })
    }

    pub async fn validate_create_batch(
        &self,
        items: &[PersonModel],
    ) -> Result<Vec<bool>, Box<dyn Error + Send + Sync>> {
        let mut results = Vec::new();
        for person in items {
            if let Some(org_id) = person.organization_person_id {
                if !self.exists_by_id(org_id).await? {
                    results.push(false);
                    continue;
                }
            }
            results.push(true);
        }
        Ok(results)
    }
}

// #[cfg(test)]
// mod tests {
//     use banking_db::models::person::{PersonModel, PersonType};
//     use banking_db::repository::{PersonRepository, PersonRepos};
//     use heapless::String as HeaplessString;
//     use uuid::Uuid;
//     use crate::test_helper::setup_test_context;

//     async fn setup_test_person() -> PersonModel {
//         PersonModel {
//             id: Uuid::new_v4(),
//             person_type: PersonType::Natural,
//             display_name: HeaplessString::try_from("Test Person").unwrap(),
//             external_identifier: Some(HeaplessString::try_from("EXT001").unwrap()),
//             entity_reference_count: 0,
//             organization_person_id: None,
//             messaging_info1: None,
//             messaging_info2: None,
//             messaging_info3: None,
//             messaging_info4: None,
//             messaging_info5: None,
//             department: None,
//             location_id: None,
//             duplicate_of_person_id: None,
//         }
//     }

//     #[tokio::test]
//     async fn test_create_batch_chunked() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let ctx = setup_test_context().await?;
//         let person_repo = ctx.person_repos().persons();

//         let mut persons = Vec::new();
//         for i in 0..25 {
//             let mut person = setup_test_person().await;
//             person.display_name = HeaplessString::try_from(format!("Chunked {i}").as_str()).unwrap();
//             person.external_identifier =
//                 Some(HeaplessString::try_from(format!("CHK{i:03}").as_str()).unwrap());
//             persons.push(person);
//         }

//         let audit_log_id = Uuid::new_v4();

//         // Save with chunk size of 10
//         let saved_persons = person_repo
//             .create_batch_chunked(persons, audit_log_id, 10)
//             .await?;

//         assert_eq!(saved_persons.items.len(), 25);

//         // Verify all persons exist
//         for person in &saved_persons.items {
//             assert!(person_repo.exists_by_id(person.id).await?);
//         }

//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_validate_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//         let ctx = setup_test_context().await?;
//         let person_repo = ctx.person_repos().persons();

//         let mut persons = Vec::new();
//         for i in 0..3 {
//             let mut person = setup_test_person().await;
//             person.display_name = HeaplessString::try_from(format!("Valid {i}").as_str()).unwrap();
//             persons.push(person);
//         }

//         // Add invalid person (referencing non-existent organization)
//         let mut invalid_person = setup_test_person().await;
//         invalid_person.organization_person_id = Some(Uuid::new_v4());
//         persons.push(invalid_person);

//         // Validate batch
//         let validation_results = person_repo
//             .validate_create_batch(&persons)
//             .await?;

//         assert_eq!(validation_results.len(), 4);
//         assert!(validation_results[0]); // Valid
//         assert!(validation_results[1]); // Valid
//         assert!(validation_results[2]); // Valid
//         assert!(!validation_results[3]); // Invalid - org not found

//         Ok(())
//     }
// }
