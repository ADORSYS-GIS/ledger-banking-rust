use banking_db::repository::LocalityRepository;
use crate::repository::person::locality_repository::batch_helper::{
    execute_locality_idx_update, execute_locality_update,
};
use crate::repository::person::locality_repository::repo_impl::LocalityRepositoryImpl;
use banking_db::models::person::LocalityModel;
use banking_db::repository::LocalityRepositoryError;
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn update_batch(
    repo: &LocalityRepositoryImpl,
    items: Vec<LocalityModel>,
    _audit_log_id: Uuid,
) -> Result<Vec<LocalityModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = items.iter().map(|i| i.id).collect();
    let mut missing_ids = Vec::new();
    for id in &ids {
        if !repo.exists_by_id(*id).await? {
            missing_ids.push(*id);
        }
    }

    if !missing_ids.is_empty() {
        return Err(Box::new(LocalityRepositoryError::ManyLocalitiesNotFound(
            missing_ids,
        )));
    }

    let mut locality_values = Vec::new();
    let mut locality_idx_values = Vec::new();
    let mut updated_items = Vec::new();

    let cache = repo.locality_idx_cache.read().await;

    for item in items {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(item.code.as_bytes());
        let new_code_hash = hasher.finish() as i64;

        if let Some(existing_idx) = cache.get_by_primary(&item.id) {
            if existing_idx.code_hash == new_code_hash {
                continue;
            }

            locality_values.push((
                item.id,
                item.country_subdivision_id,
                item.code.clone(),
                item.name_l1.clone(),
                item.name_l2.clone(),
                item.name_l3.clone(),
            ));

            locality_idx_values.push((item.id, item.country_subdivision_id, new_code_hash));

            let mut updated_idx = existing_idx.clone();
            updated_idx.code_hash = new_code_hash;
            cache.add(updated_idx);
            updated_items.push(item);
        }
    }

    if !locality_values.is_empty() {
        execute_locality_update(&repo.executor, locality_values).await?;
        execute_locality_idx_update(&repo.executor, locality_idx_values).await?;
    }

    Ok(updated_items)
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
    async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        locality_repo
            .create_batch(localities.clone(), audit_log_id)
            .await?;

        let mut updated_localities = Vec::new();
        for (i, mut locality) in localities.into_iter().enumerate() {
            if i % 2 == 0 {
                locality.code =
                    HeaplessString::try_from(format!("UPDATED{}", i).as_str()).unwrap();
                updated_localities.push(locality);
            }
        }

        if !updated_localities.is_empty() {
            let updated_result = locality_repo
                .update_batch(updated_localities.clone(), audit_log_id)
                .await?;

            assert!(!updated_result.is_empty());
            for locality in &updated_result {
                let loaded = locality_repo.load(locality.id).await?;
                assert_eq!(loaded.code, locality.code);
            }
        }

        Ok(())
    }
}