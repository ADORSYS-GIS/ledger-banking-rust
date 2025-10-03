use banking_db::models::person::{LocationAuditModel, LocationIdxModel, LocationModel};
use banking_db::repository::{
    LocalityRepository, LocationRepositoryError, LocationResult,
};
use crate::repository::executor::Executor;
use crate::repository::person::location_repository::LocationRepositoryImpl;
use std::hash::Hasher;
use twox_hash::XxHash64;
use uuid::Uuid;

pub async fn save(
    repo: &LocationRepositoryImpl,
    location: LocationModel,
    audit_log_id: Uuid,
) -> LocationResult<LocationModel> {
    if !repo
        .locality_repository
        .exists_by_id(location.locality_id)
        .await
        .map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?
    {
        return Err(LocationRepositoryError::LocalityNotFound(
            location.locality_id,
        ));
    }

    let mut hasher = XxHash64::with_seed(0);
    let mut location_cbor = Vec::new();
    ciborium::ser::into_writer(&location, &mut location_cbor).unwrap();
    hasher.write(&location_cbor);
    let new_hash = hasher.finish() as i64;

    let maybe_existing_idx = {
        let cache_read_guard = repo.location_idx_cache.read().await;
        cache_read_guard.get_by_primary(&location.id)
    };

    if let Some(existing_idx) = maybe_existing_idx {
        // UPDATE
        if existing_idx.hash == new_hash {
            return Ok(location); // No changes
        }

        let new_version = existing_idx.version + 1;

        let audit_model = LocationAuditModel {
            location_id: location.id,
            version: new_version,
            hash: new_hash,
            street_line1: location.street_line1.clone(),
            street_line2: location.street_line2.clone(),
            street_line3: location.street_line3.clone(),
            street_line4: location.street_line4.clone(),
            locality_id: location.locality_id,
            postal_code: location.postal_code.clone(),
            latitude: location.latitude,
            longitude: location.longitude,
            accuracy_meters: location.accuracy_meters,
            location_type: location.location_type,
            audit_log_id,
        };

        let query1 = sqlx::query(
            r#"
            INSERT INTO location_audit (location_id, version, hash, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(audit_model.location_id)
        .bind(audit_model.version)
        .bind(audit_model.hash)
        .bind(audit_model.street_line1.as_str())
        .bind(audit_model.street_line2.as_ref().map(|s| s.as_str()))
        .bind(audit_model.street_line3.as_ref().map(|s| s.as_str()))
        .bind(audit_model.street_line4.as_ref().map(|s| s.as_str()))
        .bind(audit_model.locality_id)
        .bind(audit_model.postal_code.as_ref().map(|s| s.as_str()))
        .bind(audit_model.latitude)
        .bind(audit_model.longitude)
        .bind(audit_model.accuracy_meters)
        .bind(audit_model.location_type)
        .bind(audit_model.audit_log_id);

        let query2 = sqlx::query(
            r#"
            UPDATE location SET
                street_line1 = $2, street_line2 = $3, street_line3 = $4, street_line4 = $5,
                locality_id = $6, postal_code = $7, latitude = $8, longitude = $9,
                accuracy_meters = $10, location_type = $11::location_type
            WHERE id = $1
            "#,
        )
        .bind(location.id)
        .bind(location.street_line1.as_str())
        .bind(location.street_line2.as_ref().map(|s| s.as_str()))
        .bind(location.street_line3.as_ref().map(|s| s.as_str()))
        .bind(location.street_line4.as_ref().map(|s| s.as_str()))
        .bind(location.locality_id)
        .bind(location.postal_code.as_ref().map(|s| s.as_str()))
        .bind(location.latitude)
        .bind(location.longitude)
        .bind(location.accuracy_meters)
        .bind(location.location_type);

        let query3 = sqlx::query(
            r#"
            UPDATE location_idx SET
                version = $2,
                hash = $3
            WHERE location_id = $1
            "#,
        )
        .bind(location.id)
        .bind(new_version)
        .bind(new_hash);

        match &repo.executor {
            Executor::Pool(pool) => {
                query1.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query2.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query3.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query2.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query3.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
            }
        }

        let new_idx = LocationIdxModel {
            location_id: location.id,
            locality_id: location.locality_id,
            version: new_version,
            hash: new_hash,
        };
        repo.location_idx_cache.read().await.update(new_idx);
    } else {
        // INSERT
        let version = 0;
        let audit_model = LocationAuditModel {
            location_id: location.id,
            version,
            hash: new_hash,
            street_line1: location.street_line1.clone(),
            street_line2: location.street_line2.clone(),
            street_line3: location.street_line3.clone(),
            street_line4: location.street_line4.clone(),
            locality_id: location.locality_id,
            postal_code: location.postal_code.clone(),
            latitude: location.latitude,
            longitude: location.longitude,
            accuracy_meters: location.accuracy_meters,
            location_type: location.location_type,
            audit_log_id,
        };

        let query1 = sqlx::query(
            r#"
            INSERT INTO location_audit (location_id, version, hash, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type, audit_log_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            "#,
        )
        .bind(audit_model.location_id)
        .bind(audit_model.version)
        .bind(audit_model.hash)
        .bind(audit_model.street_line1.as_str())
        .bind(audit_model.street_line2.as_ref().map(|s| s.as_str()))
        .bind(audit_model.street_line3.as_ref().map(|s| s.as_str()))
        .bind(audit_model.street_line4.as_ref().map(|s| s.as_str()))
        .bind(audit_model.locality_id)
        .bind(audit_model.postal_code.as_ref().map(|s| s.as_str()))
        .bind(audit_model.latitude)
        .bind(audit_model.longitude)
        .bind(audit_model.accuracy_meters)
        .bind(audit_model.location_type)
        .bind(audit_model.audit_log_id);

        let query2 = sqlx::query(
            r#"
            INSERT INTO location (id, street_line1, street_line2, street_line3, street_line4, locality_id, postal_code, latitude, longitude, accuracy_meters, location_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(location.id)
        .bind(location.street_line1.as_str())
        .bind(location.street_line2.as_ref().map(|s| s.as_str()))
        .bind(location.street_line3.as_ref().map(|s| s.as_str()))
        .bind(location.street_line4.as_ref().map(|s| s.as_str()))
        .bind(location.locality_id)
        .bind(location.postal_code.as_ref().map(|s| s.as_str()))
        .bind(location.latitude)
        .bind(location.longitude)
        .bind(location.accuracy_meters)
        .bind(location.location_type);

        let query3 = sqlx::query(
            r#"
            INSERT INTO location_idx (location_id, locality_id, version, hash)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(location.id)
        .bind(location.locality_id)
        .bind(version)
        .bind(new_hash);

        match &repo.executor {
            Executor::Pool(pool) => {
                query1.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query2.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query3.execute(&**pool).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
            }
            Executor::Tx(tx) => {
                let mut tx = tx.lock().await;
                query1.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query2.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
                query3.execute(&mut **tx).await.map_err(|e| LocationRepositoryError::RepositoryError(e.into()))?;
            }
        }

        let new_idx = LocationIdxModel {
            location_id: location.id,
            locality_id: location.locality_id,
            version,
            hash: new_hash,
        };
        repo.location_idx_cache.read().await.add(new_idx);
    }

    Ok(location)
}
#[cfg(test)]
mod tests {
    use banking_db::repository::{
        CountryRepository, CountrySubdivisionRepository, LocalityRepository, LocationRepository,
        PersonRepos,
    };
    use uuid::Uuid;

    use crate::test_helper::setup_test_context;
    use crate::repository::person::test_helpers::{
        create_test_country_model, create_test_country_subdivision_model,
        create_test_locality_model, create_test_location_model,
    };

    #[tokio::test]
    async fn test_save_location() {
        let ctx = setup_test_context().await.unwrap();
        let country_repo = ctx.person_repos().countries();
        let country_subdivision_repo = ctx.person_repos().country_subdivisions();
        let locality_repo = ctx.person_repos().localities();
        let repo = ctx.person_repos().locations();

        // Use unique codes for test isolation
        let unique_iso2 = format!("O{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country = create_test_country_model(&unique_iso2, "Test Country");
        country_repo.save(country.clone()).await.unwrap();

        let unique_subdivision_code =
            format!("OS{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let country_subdivision = create_test_country_subdivision_model(
            country.id,
            &unique_subdivision_code,
            "Test Subdivision",
        );
        country_subdivision_repo
            .save(country_subdivision.clone())
            .await
            .unwrap();

        let unique_locality_code =
            format!("OL{}", &Uuid::new_v4().to_string()[0..1].to_uppercase());
        let locality =
            create_test_locality_model(country_subdivision.id, &unique_locality_code, "Test Locality");
        locality_repo.save(locality.clone()).await.unwrap();

        // Test save
        let new_location = create_test_location_model(locality.id, "Test Street", "12345");
        let audit_log_id = Uuid::new_v4();
        let saved_location = repo.save(new_location.clone(), audit_log_id).await.unwrap();
        assert_eq!(new_location.id, saved_location.id);
    }
}