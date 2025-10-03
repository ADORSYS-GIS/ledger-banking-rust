use crate::repository::person::country_subdivision_repository::CountrySubdivisionRepositoryImpl;
use banking_db::models::person::CountrySubdivisionModel;
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

pub async fn update_batch(
    repo: &CountrySubdivisionRepositoryImpl,
    _items: Vec<CountrySubdivisionModel>,
    _audit_log_id: Uuid,
) -> Result<Vec<CountrySubdivisionModel>, Box<dyn Error + Send + Sync>> {
    if _items.is_empty() {
        return Ok(Vec::new());
    }

    let ids: Vec<Uuid> = _items.iter().map(|i| i.id).collect();
    let mut missing_ids = Vec::new();
    for id in &ids {
        if !repo.exists_by_id(*id).await? {
            missing_ids.push(*id);
        }
    }

    if !missing_ids.is_empty() {
        return Err(Box::new(
            CountrySubdivisionRepositoryError::ManyCountrySubdivisionsNotFound(missing_ids),
        ));
    }

    let mut country_subdivision_values = Vec::new();
    let mut country_subdivision_idx_values = Vec::new();
    let mut updated_items = Vec::new();

    let cache = repo.country_subdivision_idx_cache.read().await;

    for item in _items {
        let mut hasher = XxHash64::with_seed(0);
        hasher.write(item.code.as_bytes());
        let new_code_hash = hasher.finish() as i64;

        if let Some(existing_idx) = cache.get_by_primary(&item.id) {
            if existing_idx.code_hash == new_code_hash {
                continue;
            }

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
                new_code_hash,
            ));

            let mut updated_idx = existing_idx.clone();
            updated_idx.code_hash = new_code_hash;
            cache.add(updated_idx);
            updated_items.push(item);
        }
    }

    if !country_subdivision_values.is_empty() {
        crate::repository::person::country_subdivision_repository::batch_helper::execute_country_subdivision_update(repo, country_subdivision_values).await?;
        crate::repository::person::country_subdivision_repository::batch_helper::execute_country_subdivision_idx_update(repo, country_subdivision_idx_values).await?;
    }

    Ok(updated_items)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::person::test_helpers::create_test_country_model;
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{BatchRepository, CountryRepository, CountrySubdivisionRepository, PersonRepos};
    use heapless::String as HeaplessString;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_update_batch() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

        let mut updated_subdivisions = Vec::new();
        for mut subdivision in saved_subdivisions {
            subdivision.name_l1 = HeaplessString::try_from(format!("{} Updated", subdivision.name_l1).as_str()).unwrap();
            updated_subdivisions.push(subdivision);
        }

        let result = country_subdivision_repo
            .update_batch(updated_subdivisions.clone(), audit_log_id)
            .await?;

        assert_eq!(result.len(), 5);

        for subdivision in &updated_subdivisions {
            let loaded = country_subdivision_repo.load(subdivision.id).await?;
            assert_eq!(loaded.name_l1, subdivision.name_l1);
        }

        Ok(())
    }
}