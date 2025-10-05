use banking_db::models::person::CountrySubdivisionIdxModel;
use banking_db::repository::CountrySubdivisionResult;
use uuid::Uuid;

use super::repo_impl::CountrySubdivisionRepositoryImpl;

pub async fn find_by_id(
    repo: &CountrySubdivisionRepositoryImpl,
    id: Uuid,
) -> CountrySubdivisionResult<Option<CountrySubdivisionIdxModel>> {
    Ok(repo
        .country_subdivision_idx_cache
        .read()
        .await
        .get_by_primary(&id))
}

#[cfg(test)]
mod tests {
    
    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model,
    };
    use crate::test_helper::setup_test_context;
    use banking_db::repository::{CountryRepository, CountrySubdivisionRepository, PersonRepos};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_find_by_id() {
        let ctx = setup_test_context().await.unwrap();
        let country_repo = ctx.person_repos().countries();
        let subdivision_repo = ctx.person_repos().country_subdivisions();

        let unique_iso2 = format!("C{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country = create_test_country_model(&unique_iso2, "Test Country");
        country_repo.save(country.clone()).await.unwrap();

        let unique_code = format!("S{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let new_subdivision =
            create_test_country_subdivision_model(country.id, &unique_code, "Test Subdivision");
        subdivision_repo
            .save(new_subdivision.clone())
            .await
            .unwrap();

        let found_subdivision = subdivision_repo
            .find_by_id(new_subdivision.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            new_subdivision.id,
            found_subdivision.country_subdivision_id
        );
    }
}