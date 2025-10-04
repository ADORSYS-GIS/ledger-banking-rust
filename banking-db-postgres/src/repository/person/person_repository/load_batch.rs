use crate::repository::person::person_repository::repo_impl::PersonRepositoryImpl;
use crate::utils::TryFromRow;
use banking_db::models::person::PersonModel;
use std::error::Error;
use uuid::Uuid;

pub async fn load_batch(
    repo: &PersonRepositoryImpl,
    ids: &[Uuid],
) -> Result<Vec<Option<PersonModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = "SELECT * FROM person WHERE id = ANY($1)";
    let rows = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query).bind(ids).fetch_all(&**pool).await?
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
        }
    };
    let mut person_map = std::collections::HashMap::new();
    for row in rows {
        let person = PersonModel::try_from_row(&row)?;
        person_map.insert(person.id, person);
    }
    Ok(ids.iter().map(|id| person_map.remove(id)).collect())
}
#[cfg(test)]
mod tests {
    use banking_db::models::person::{PersonModel, PersonType};
    use banking_db::repository::{BatchRepository, PersonRepos};
    use heapless::String as HeaplessString;
    use uuid::Uuid;
    use crate::test_helper::setup_test_context;

    async fn setup_test_person() -> PersonModel {
        PersonModel {
            id: Uuid::new_v4(),
            person_type: PersonType::Natural,
            display_name: HeaplessString::try_from("Test Person").unwrap(),
            external_identifier: Some(HeaplessString::try_from("EXT001").unwrap()),
            entity_reference_count: 0,
            organization_person_id: None,
            messaging_info1: None,
            messaging_info2: None,
            messaging_info3: None,
            messaging_info4: None,
            messaging_info5: None,
            department: None,
            location_id: None,
            duplicate_of_person_id: None,
        }
    }

    #[tokio::test]
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let person_repo = ctx.person_repos().persons();

        let mut persons = Vec::new();
        let mut test_ids = Vec::new();

        for i in 0..3 {
            let mut person = setup_test_person().await;
            person.display_name =
                HeaplessString::try_from(format!("Load Test {i}").as_str()).unwrap();
            person.external_identifier =
                Some(HeaplessString::try_from(format!("EXT_LOAD_{i}").as_str()).unwrap());
            persons.push(person.clone());
            test_ids.push(person.id);
        }

        let audit_log_id = Uuid::new_v4();
        person_repo
            .create_batch(persons, audit_log_id)
            .await?;

        let loaded_persons = person_repo
            .load_batch(&test_ids)
            .await?;

        assert_eq!(loaded_persons.len(), 3);

        let mut loaded_persons: Vec<PersonModel> = loaded_persons.into_iter().flatten().collect();
        assert_eq!(
            loaded_persons.len(),
            3,
            "Expected to load 3 persons, but found {}",
            loaded_persons.len()
        );
        loaded_persons.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        for (i, person) in loaded_persons.iter().enumerate().take(3) {
            assert_eq!(person.display_name.as_str(), format!("Load Test {i}"));
        }

        Ok(())
    }
}