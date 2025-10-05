use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use banking_db::models::person::{CountrySubdivisionIdxModel, CountrySubdivisionModel};
use banking_db::repository::{CountrySubdivisionRepository, CountrySubdivisionRepositoryError};
use std::error::Error;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

type CountrySubdivisionTuple = (
    Uuid,
    Uuid,
    String,
    String,
    Option<String>,
    Option<String>,
);

type CountrySubdivisionIdxTuple = (
    Uuid,
    Uuid,
    i64,
);

pub async fn create_batch(
    repo: &CountrySubdivisionRepositoryImpl,
    items: Vec<CountrySubdivisionModel>,
    _audit_log_id: Uuid,
) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
    if items.is_empty() {
        return Ok(Vec::new());
    }

    let mut truly_existing_ids = Vec::new();
    for item in &items {
        if repo.exists_by_id(item.id).await? {
            truly_existing_ids.push(item.id);
        }
    }

    if !truly_existing_ids.is_empty() {
        return Err(Box::new(
            CountrySubdivisionRepositoryError::ManyCountrySubdivisionsExist(
                truly_existing_ids,
            ),
        ));
    }

    let cache = repo.country_subdivision_idx_cache.read().await;
    for item in &items {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(item.code.as_bytes());
        let code_hash = hasher.finish() as i64;

        let idx_model = CountrySubdivisionIdxModel {
            country_subdivision_id: item.id,
            country_id: item.country_id,
            code_hash,
        };
        cache.add(idx_model);
    }

    let mut country_subdivision_values: Vec<CountrySubdivisionTuple> = Vec::new();
    let mut country_subdivision_idx_values: Vec<CountrySubdivisionIdxTuple> = Vec::new();

    for item in &items {
        let idx_model = cache.get_by_primary(&item.id).unwrap();

        country_subdivision_values.push((
            item.id,
            item.country_id,
            item.code.to_string(),
            item.name_l1.to_string(),
            item.name_l2.as_ref().map(|s| s.to_string()),
            item.name_l3.as_ref().map(|s| s.to_string()),
        ));

        country_subdivision_idx_values.push((
            item.id,
            item.country_id,
            idx_model.code_hash,
        ));
    }

    if !country_subdivision_values.is_empty() {
        crate::repository::person::country_subdivision_repository::batch_helper::execute_country_subdivision_insert(repo, country_subdivision_values).await?;
        crate::repository::person::country_subdivision_repository::batch_helper::execute_country_subdivision_idx_insert(repo, country_subdivision_idx_values).await?;
    }

    Ok(items)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::person::test_helpers::create_test_country_model;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{BatchRepository, CountryRepository, PersonRepos};
    

    #[tokio::test]
    async fn test_create_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let ctx = setup_test_context().await?;
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();

        let country = create_test_country_model("US", "United States");
        country_repo.save(country.clone()).await?;

        let mut country_subdivisions = Vec::new();
        for i in 0..5 {
            let subdivision =
                crate::repository::person::test_helpers::create_test_country_subdivision_model(
                    country.id,
                    &format!("CD{i:03}"),
                    &format!("Test Subdivision {i}"),
                );
            country_subdivisions.push(subdivision);
        }

        let audit_log_id = Uuid::new_v4();

        let saved_subdivisions = country_subdivision_repo
            .create_batch(country_subdivisions.clone(), audit_log_id)
            .await?;

        assert_eq!(saved_subdivisions.len(), 5);

        for subdivision in &saved_subdivisions {
            assert!(country_subdivision_repo.exists_by_id(subdivision.id).await?);
        }

        Ok(())
    }
}