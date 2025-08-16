use sqlx::PgPool;
use uuid::Uuid;

/// Create standard test person for foreign key references
///
/// Many tests need a person record for foreign key constraints.
/// This function creates a standard test person that can be reused.
#[allow(dead_code)]
pub async fn create_test_person(pool: &PgPool) -> Uuid {
    let test_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    sqlx::query(
        r#"
        INSERT INTO person (id, person_type, display_name, external_identifier, is_active, created_at, updated_at)
        VALUES ($1, 'System', 'Test User', 'test-user', true, NOW(), NOW())
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .bind(test_person_id)
    .execute(pool)
    .await
    .expect("Failed to create test person");

    test_person_id
}