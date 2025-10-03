use crate::repository::executor::Executor;
use crate::repository::person::locality_repository::repo_impl::LocalityRepositoryImpl;
use crate::utils::TryFromRow;
use banking_db::models::person::LocalityModel;
use std::error::Error;
use uuid::Uuid;

pub async fn load_batch(
    repo: &LocalityRepositoryImpl,
    ids: &[Uuid],
) -> Result<Vec<Option<LocalityModel>>, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let query = r#"SELECT * FROM locality WHERE id = ANY($1)"#;
    let rows = match &repo.executor {
        Executor::Pool(pool) => sqlx::query(query).bind(ids).fetch_all(pool.as_ref()).await?,
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(query).bind(ids).fetch_all(&mut **tx).await?
        }
    };
    let mut item_map = std::collections::HashMap::new();
    for row in rows {
        let item = LocalityModel::try_from_row(&row)?;
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
    use super::*;
    use banking_db::models::person::LocalityModel;
    use banking_db::repository::{
        BatchRepository, CountryRepository, CountrySubdivisionRepository, LocalityRepository, PersonRepos,
    };
    use heapless::String as HeaplessString;
    use uuid::Uuid;

    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model,
    };
    use crate::test_helper::setup_test_context;

    pub async fn setup_test_locality(country_subdivision_id: Uuid) -> LocalityModel {
        LocalityModel {
            id: Uuid::new_v4(),
            country_subdivision_id,
            code: HeaplessString::try_from("TEST").unwrap(),
            name_l1: HeaplessString::try_from("Test Locality").unwrap(),
            name_l2: None,
            name_l3: None,
        }
    }

    #[tokio::test]
    async fn test_load_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();
        let locality_repo = ctx.person_repos().localities();

        let country = create_test_country_model(&format!("L{}", &Uuid::new_v4().to_string()[0..1].to_uppercase()), "Test Country");
        country_repo.save(country.clone()).await?;

        let subdivision = create_test_country_subdivision_model(country.id, &format!("LS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase()), "Test Subdivision");
        country_subdivision_repo.save(subdivision.clone()).await?;

        let mut localities = Vec::new();
        let mut ids = Vec::new();
        for i in 0..5 {
            let mut locality = setup_test_locality(subdivision.id).await;
            locality.code = HeaplessString::try_from(format!("CD{i:03}").as_str()).unwrap();
            locality.name_l1 = HeaplessString::try_from(format!("Test Locality {i}").as_str()).unwrap();
            ids.push(locality.id);
            localities.push(locality);
        }

        let audit_log_id = Uuid::new_v4();
        locality_repo
            .create_batch(localities.clone(), audit_log_id)
            .await?;

        let loaded_localities = locality_repo.load_batch(&ids).await?;
        assert_eq!(loaded_localities.len(), 5);
        for (i, locality) in loaded_localities.iter().enumerate() {
            assert!(locality.is_some());
            assert_eq!(locality.as_ref().unwrap().id, localities[i].id);
        }

        Ok(())
    }
}