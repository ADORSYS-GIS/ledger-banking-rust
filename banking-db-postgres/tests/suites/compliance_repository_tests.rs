use banking_db::models::compliance::{ComplianceAlertModel, ExtendedComplianceAlertModel, AlertType, Severity, AlertStatus};
use banking_db::repository::compliance_repository::ComplianceRepository;
use banking_db_postgres::ComplianceRepositoryImpl;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database")
}

fn create_test_alert() -> ComplianceAlertModel {
    use heapless::String as HeaplessString;
    let alert_id = Uuid::new_v4();
    ComplianceAlertModel {
        alert_data: ExtendedComplianceAlertModel {
            id: alert_id,
            customer_id: None,
            account_id: None,
            transaction_id: None,
            alert_type: AlertType::SuspiciousPattern,
            severity: Severity::Medium,
            description: HeaplessString::try_from("Test alert").unwrap(),
            triggered_at: Utc::now(),
            status: AlertStatus::New,
            assigned_to_person_id: None,
            resolved_at: None,
            resolved_by_person_id: None,
            resolution_notes: None,
            metadata: None,
            created_at: Utc::now(),
            last_updated_at: Utc::now(),
        },
    }
}

#[tokio::test]
async fn test_create_and_find_alert() {
    let pool = setup_test_db().await;
    let repo = ComplianceRepositoryImpl::new(pool);
    let alert = create_test_alert();

    let created_alert = repo.create_alert(alert.clone()).await.unwrap();
    assert_eq!(alert.alert_data.id, created_alert.alert_data.id);

    let found_alert = repo.find_alert_by_id(alert.alert_data.id).await.unwrap().unwrap();
    assert_eq!(alert.alert_data.id, found_alert.alert_data.id);
}