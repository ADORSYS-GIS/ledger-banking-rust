use banking_db::repository::LocalityRepository;
use crate::repository::person::locality_repository::batch_helper::{
    execute_locality_idx_insert, execute_locality_insert,
};
use crate::repository::person::locality_repository::repo_impl::LocalityRepositoryImpl;
use banking_db::models::person::{LocalityIdxModel, LocalityModel};
use banking_db::repository::{CountrySubdivisionRepository, LocalityRepositoryError};
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn create_batch(
    repo: &LocalityRepositoryImpl,
    items: Vec<LocalityModel>,
    _audit_log_id: Uuid,
) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|p| p.id).collect();
    if repo.exist_by_ids(&ids).await?.into_iter().any(|x| x) {
        return Err(Box::new(LocalityRepositoryError::DuplicateLocation(
            "One or more localities already exist".to_string(),
        )));
    }

    let subdivision_ids: HashSet<Uuid> =
        items.iter().map(|l| l.country_subdivision_id).collect();
    for id in subdivision_ids {
        if !repo.country_subdivision_repository.exists_by_id(id).await? {
            return Err(Box::new(
                LocalityRepositoryError::CountrySubdivisionNotFound(id),
            ));
        }
    }

    let mut locality_values = Vec::with_capacity(items.len());
    let mut locality_idx_values = Vec::with_capacity(items.len());

    let cache = repo.locality_idx_cache.read().await;
    for item in &items {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(item.code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let idx_model = LocalityIdxModel {
            locality_id: item.id,
            country_subdivision_id: item.country_subdivision_id,
            code_hash,
        };
        cache.add(idx_model.clone());

        locality_values.push((
            item.id,
            item.country_subdivision_id,
            item.code.clone(),
            item.name_l1.clone(),
            item.name_l2.clone(),
            item.name_l3.clone(),
        ));
        locality_idx_values.push((item.id, item.country_subdivision_id, code_hash));
    }

    if !locality_values.is_empty() {
        execute_locality_insert(&repo.executor, locality_values).await?;
        execute_locality_idx_insert(&repo.executor, locality_idx_values).await?;
    }

    Ok(items)
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
    async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();
        let locality_repo = ctx.person_repos().localities();

        let country = create_test_country_model(&format!("L{}", &Uuid::new_v4().to_string()[0..1].to_uppercase()), "Test Country");
        country_repo.save(country.clone()).await?;

        let subdivision = create_test_country_subdivision_model(country.id, &format!("LS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase()), "Test Subdivision");
        country_subdivision_repo.save(subdivision.clone()).await?;

        let mut localities = Vec::new();
        for i in 0..5 {
            let mut locality = setup_test_locality(subdivision.id).await;
            locality.code = HeaplessString::try_from(format!("CD{i:03}").as_str()).unwrap();
            locality.name_l1 = HeaplessString::try_from(format!("Test Locality {i}").as_str()).unwrap();
            localities.push(locality);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_localities = locality_repo
            .create_batch(localities.clone(), audit_log_id)
            .await?;

        assert_eq!(saved_localities.len(), 5);

        for locality in &saved_localities {
            assert!(locality_repo.exists_by_id(locality.id).await?);
        }

        Ok(())
    }
}