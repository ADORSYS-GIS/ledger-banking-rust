use banking_db::models::person::EntityReferenceModel;
use banking_db::repository::person::entity_reference_repository::{
    EntityReferenceRepositoryError, EntityReferenceResult,
};
use crate::repository::person::entity_reference_repository::EntityReferenceRepositoryImpl;
use crate::utils::TryFromRow;
use uuid::Uuid;

pub async fn load(
    repo: &EntityReferenceRepositoryImpl,
    id: Uuid,
) -> EntityReferenceResult<EntityReferenceModel> {
    let query = sqlx::query(
        r#"
        SELECT * FROM entity_reference WHERE id = $1
        "#,
    )
    .bind(id);

    let row = match &repo.executor {
        crate::repository::executor::Executor::Pool(pool) => query
            .fetch_one(&**pool)
            .await
            .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?,
        crate::repository::executor::Executor::Tx(tx) => {
            let mut tx = tx.lock().await;
            query
                .fetch_one(&mut **tx)
                .await
                .map_err(|e| EntityReferenceRepositoryError::RepositoryError(e.into()))?
        }
    };

    EntityReferenceModel::try_from_row(&row).map_err(EntityReferenceRepositoryError::RepositoryError)
}