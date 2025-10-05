use banking_db::models::person::LocalityIdxModel;
use banking_db::repository::LocalityResult;
use crate::repository::person::locality_repository::LocalityRepositoryImpl;
use uuid::Uuid;

pub async fn find_by_country_subdivision_id(
    repo: &LocalityRepositoryImpl,
    country_subdivision_id: Uuid,
    _page: i32,
    _page_size: i32,
) -> LocalityResult<Vec<LocalityIdxModel>> {
    let cache = repo.locality_idx_cache.read().await;
    let mut result = Vec::new();
    if let Some(ids) = cache.get_by_country_subdivision_id(&country_subdivision_id) {
        for id in ids {
            if let Some(idx) = cache.get_by_primary(&id) {
                result.push(idx);
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, LocalityRepository, PersonRepos};
    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model, create_test_locality_model,
    };
    use crate::test_helper::setup_test_context;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_find_by_country_subdivision_id() {
        let ctx = setup_test_context().await.unwrap();
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();
        let repo = ctx.person_repos().localities();

        let unique_iso2 = format!("L{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country = create_test_country_model(&unique_iso2, "Test Country");
        country_repo.save(country.clone()).await.unwrap();

        let unique_subdivision_code = format!("LS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country_subdivision = create_test_country_subdivision_model(country.id, &unique_subdivision_code, "Test Subdivision");
        country_subdivision_repo
            .save(country_subdivision.clone())
            .await
            .unwrap();

        let unique_locality_code = format!("LC{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let new_locality = create_test_locality_model(country_subdivision.id, &unique_locality_code, "Test Locality");
        repo.save(new_locality.clone()).await.unwrap();

        let localities_in_country_subdivision = repo
            .find_by_country_subdivision_id(country_subdivision.id, 1, 10)
            .await
            .unwrap();
        assert_eq!(localities_in_country_subdivision.len(), 1);
    }
}