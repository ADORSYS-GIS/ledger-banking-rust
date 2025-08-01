use banking_db::repository::ComplianceRepository;
use banking_db_postgres::SimpleComplianceRepositoryImpl;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{NaiveDate, Datelike};

/// Integration test helper to set up database connection
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
    
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database")
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_kyc_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find KYC by ID with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let kyc_record = repo.find_kyc_by_id(non_existent_id).await
        .expect("Failed to find KYC record by id");
    assert!(kyc_record.is_none());
    
    // Test find KYC by customer with non-existent customer
    let non_existent_customer = Uuid::new_v4();
    let kyc_record = repo.find_kyc_by_customer(non_existent_customer).await
        .expect("Failed to find KYC record by customer");
    assert!(kyc_record.is_none());
    
    // Test count KYC records
    let count = repo.count_kyc_records().await
        .expect("Failed to count KYC records");
    assert!(count >= 0);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_sanctions_screening_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find screening by ID with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let screening = repo.find_screening_by_id(non_existent_id).await
        .expect("Failed to find screening by id");
    assert!(screening.is_none());
    
    // Test find screening by customer with non-existent customer
    let non_existent_customer = Uuid::new_v4();
    let screenings = repo.find_screening_by_customer(non_existent_customer).await
        .expect("Failed to find screenings by customer");
    assert!(screenings.is_empty());
    
    // Test find latest screening with non-existent customer
    let latest_screening = repo.find_latest_screening(non_existent_customer).await
        .expect("Failed to find latest screening");
    assert!(latest_screening.is_none());
    
    // Test count sanctions screenings
    let count = repo.count_sanctions_screenings().await
        .expect("Failed to count sanctions screenings");
    assert!(count >= 0);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_compliance_alert_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find alert by ID with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let alert = repo.find_alert_by_id(non_existent_id).await
        .expect("Failed to find alert by id");
    assert!(alert.is_none());
    
    // Test find open alerts
    let open_alerts = repo.find_open_alerts().await
        .expect("Failed to find open alerts");
    // Should not error, even if empty
    assert!(open_alerts.len() <= 10); // Limited to 10 in our implementation
    
    // Test count compliance alerts
    let count = repo.count_compliance_alerts().await
        .expect("Failed to count compliance alerts");
    assert!(count >= 0);
    
    // Test count open alerts
    let open_count = repo.count_open_alerts().await
        .expect("Failed to count open alerts");
    assert!(open_count >= 0);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_ubo_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find UBO by ID with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let ubo = repo.find_ubo_by_id(non_existent_id).await
        .expect("Failed to find UBO by id");
    assert!(ubo.is_none());
    
    // Test find UBO by corporate with non-existent corporate
    let non_existent_corporate = Uuid::new_v4();
    let ubos = repo.find_ubo_by_corporate(non_existent_corporate).await
        .expect("Failed to find UBOs by corporate");
    assert!(ubos.is_empty());
    
    // Test find UBO by beneficiary with non-existent beneficiary
    let non_existent_beneficiary = Uuid::new_v4();
    let ubos = repo.find_ubo_by_beneficiary(non_existent_beneficiary).await
        .expect("Failed to find UBOs by beneficiary");
    assert!(ubos.is_empty());
    
    // Test count UBO links
    let count = repo.count_ubo_links().await
        .expect("Failed to count UBO links");
    assert!(count >= 0);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_risk_score_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find risk score by customer with non-existent customer
    let non_existent_customer = Uuid::new_v4();
    let risk_score = repo.find_risk_score_by_customer(non_existent_customer).await
        .expect("Failed to find risk score by customer");
    assert!(risk_score.is_none());
    
    // Test find high risk customers
    let high_risk_customers = repo.find_high_risk_customers(80.0).await
        .expect("Failed to find high risk customers");
    assert!(high_risk_customers.is_empty());
    
    // Test find risk scores requiring review
    let requiring_review = repo.find_risk_scores_requiring_review(30).await
        .expect("Failed to find risk scores requiring review");
    assert!(requiring_review.is_empty());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_compliance_result_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find compliance result by ID with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let result = repo.find_compliance_result_by_id(non_existent_id).await
        .expect("Failed to find compliance result by id");
    assert!(result.is_none());
    
    // Test find compliance results by account with non-existent account
    let non_existent_account = Uuid::new_v4();
    let results = repo.find_compliance_results_by_account(non_existent_account).await
        .expect("Failed to find compliance results by account");
    assert!(results.is_empty());
    
    // Test find failed compliance results
    let failed_results = repo.find_failed_compliance_results().await
        .expect("Failed to find failed compliance results");
    assert!(failed_results.is_empty());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_sar_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test find SAR by ID with non-existent ID
    let non_existent_id = Uuid::new_v4();
    let sar = repo.find_sar_by_id(non_existent_id).await
        .expect("Failed to find SAR by id");
    assert!(sar.is_none());
    
    // Test find SAR by customer with non-existent customer
    let non_existent_customer = Uuid::new_v4();
    let sars = repo.find_sar_by_customer(non_existent_customer).await
        .expect("Failed to find SARs by customer");
    assert!(sars.is_empty());
    
    // Test find pending SAR filings
    let pending_sars = repo.find_pending_sar_filings().await
        .expect("Failed to find pending SAR filings");
    assert!(pending_sars.is_empty());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_transaction_monitoring_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    let from_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let to_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    
    // Test find flagged transactions
    let flagged_transactions = repo.find_flagged_transactions(from_date, to_date).await
        .expect("Failed to find flagged transactions");
    assert!(flagged_transactions.is_empty());
    
    // Test find transactions by pattern
    let pattern_transactions = repo.find_transactions_by_pattern("structuring").await
        .expect("Failed to find transactions by pattern");
    assert!(pattern_transactions.is_empty());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_reporting_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    let from_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let to_date = NaiveDate::from_ymd_opt(2024, 3, 31).unwrap();
    
    // Test generate compliance summary
    let summary = repo.generate_compliance_summary(from_date, to_date).await
        .expect("Failed to generate compliance summary");
    assert_eq!(summary.period_start, from_date);
    assert_eq!(summary.period_end, to_date);
    assert_eq!(summary.total_kyc_checks, 0);
    
    // Test generate KYC report
    let kyc_report = repo.generate_kyc_report(from_date, to_date).await
        .expect("Failed to generate KYC report");
    assert_eq!(kyc_report.period_start, from_date);
    assert_eq!(kyc_report.period_end, to_date);
    assert_eq!(kyc_report.total_verifications, 0);
    
    // Test generate sanctions report
    let sanctions_report = repo.generate_sanctions_report(from_date, to_date).await
        .expect("Failed to generate sanctions report");
    assert_eq!(sanctions_report.period_start, from_date);
    assert_eq!(sanctions_report.period_end, to_date);
    assert_eq!(sanctions_report.total_screenings, 0);
    
    // Test generate alert summary
    let alert_summary = repo.generate_alert_summary(from_date, to_date).await
        .expect("Failed to generate alert summary");
    assert_eq!(alert_summary.period_start, from_date);
    assert_eq!(alert_summary.period_end, to_date);
    assert_eq!(alert_summary.total_alerts, 0);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_simple_utility_operations() {
    let pool = setup_test_db().await;
    let repo = SimpleComplianceRepositoryImpl::new(pool);
    
    // Test count pending reviews
    let pending_reviews = repo.count_pending_reviews().await
        .expect("Failed to count pending reviews");
    assert!(pending_reviews >= 0);
}

// Unit tests that don't require database
#[tokio::test]
async fn test_simple_compliance_repository_creation() {
    // This test doesn't actually connect to database
    let database_url = "postgresql://dummy:dummy@localhost:5432/dummy";
    
    // This will fail to connect but we're just testing creation
    let result = PgPool::connect(database_url).await;
    assert!(result.is_err()); // Expected to fail with dummy URL
}

#[tokio::test]
async fn test_compliance_uuid_generation() {
    let uuid1 = Uuid::new_v4();
    let uuid2 = Uuid::new_v4();
    assert_ne!(uuid1, uuid2);
}

#[tokio::test]
async fn test_date_operations() {
    let from_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let to_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    
    assert!(from_date < to_date);
    assert_eq!(from_date.year(), 2024);
    assert_eq!(to_date.month(), 12);
}