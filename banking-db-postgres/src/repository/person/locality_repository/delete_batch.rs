use banking_db::repository::LocationRepository;
use crate::repository::executor::Executor;
use crate::repository::person::locality_repository::repo_impl::LocalityRepositoryImpl;
use banking_db::repository::LocalityRepositoryError;
use std::error::Error;
use uuid::Uuid;

pub async fn delete_batch(
    repo: &LocalityRepositoryImpl,
    ids: &[Uuid],
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if ids.is_empty() {
        return Ok(0);
    }

    let location_repo = repo.location_repository.get().unwrap();
    let mut dependent_locations = Vec::new();
    for id in ids {
        let locations = location_repo.find_by_locality_id(*id, 0, 1).await?;
        if !locations.is_empty() {
            dependent_locations.push(*id);
        }
    }

    if !dependent_locations.is_empty() {
        return Err(Box::new(LocalityRepositoryError::HasDependentLocations(
            dependent_locations,
        )));
    }

    let cache = repo.locality_idx_cache.read().await;
    for id in ids {
        cache.remove(id);
    }

    let delete_query = r#"DELETE FROM locality WHERE id = ANY($1)"#;
    let delete_idx_query = r#"DELETE FROM locality_idx WHERE locality_id = ANY($1)"#;

    match &repo.executor {
        Executor::Pool(pool) => {
            sqlx::query(delete_idx_query)
                .bind(ids)
                .execute(pool.as_ref())
                .await?;
            sqlx::query(delete_query).bind(ids).execute(pool.as_ref()).await?;
        }
        Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            sqlx::query(delete_idx_query)
                .bind(ids)
                .execute(&mut **tx)
                .await?;
            sqlx::query(delete_query).bind(ids).execute(&mut **tx).await?;
        }
    }

    Ok(ids.len())
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
    async fn test_delete_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

        let deleted_count = locality_repo.delete_batch(&ids).await?;
        assert_eq!(deleted_count, 5);

        for id in &ids {
            assert!(!locality_repo.exists_by_id(*id).await?);
        }

        Ok(())
    }
}