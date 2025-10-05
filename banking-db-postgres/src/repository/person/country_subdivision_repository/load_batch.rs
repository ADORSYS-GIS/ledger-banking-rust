use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use crate::utils::TryFromRow;
use banking_db::models::person::CountrySubdivisionModel;
use std::error::Error;
use uuid::Uuid;

pub async fn load_batch(
    repo: &CountrySubdivisionRepositoryImpl,
    ids: &[Uuid],
) -> Result<Vec<Option<CountrySubdivisionModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = r#"SELECT * FROM country_subdivision WHERE id = ANY($1)"#;
    let rows = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => {
            sqlx::query(query).bind(ids).fetch_all(&**pool).await?
        }
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
        }
    };
    let mut item_map = std::collections::HashMap::new();
    for row in rows {
        let item = CountrySubdivisionModel::try_from_row(&row)?;
        item_map.insert(item.id, item);
    }
    let mut result = Vec::with_capacity(ids.len());
    for id in ids {
        result.push(item_map.remove(id));
    }
    Ok(result)
}
#[cfg(test)]
mod tests {
    
    use crate::repository::person::test_helpers::create_test_country_model;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{BatchRepository, CountryRepository, PersonRepos};
    
    use uuid::Uuid;

    #[tokio::test]
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();

        let country = create_test_country_model("US", "United States");
        country_repo.save(country.clone()).await?;

        let mut country_subdivisions = Vec::new();
        let mut ids = Vec::new();
        for i in 0..5 {
            let subdivision =
                crate::repository::person::test_helpers::create_test_country_subdivision_model(
                    country.id,
                    &format!("CD{i:03}"),
                    &format!("Test Subdivision {i}"),
                );
            ids.push(subdivision.id);
            country_subdivisions.push(subdivision);
        }

        let audit_log_id = Uuid::new_v4();
        country_subdivision_repo
            .create_batch(country_subdivisions.clone(), audit_log_id)
            .await?;

        let loaded_subdivisions = country_subdivision_repo.load_batch(&ids).await?;
        assert_eq!(loaded_subdivisions.len(), 5);
        for (i, subdivision) in loaded_subdivisions.iter().enumerate() {
            assert!(subdivision.is_some());
            assert_eq!(subdivision.as_ref().unwrap().id, country_subdivisions[i].id);
        }

        Ok(())
    }
}