use banking_db::{DbAccountStatus, DbAccountType, DbSigningCondition};
use banking_db::models::AccountModel;
use chrono::{NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

/// Test helper to create a sample account
#[allow(dead_code)]
fn create_test_account() -> AccountModel {
    let account_id = Uuid::new_v4();
    // Use a fixed UUID that we'll insert into persons table in setup
    let updated_by_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let domicile_agency_branch_id = Uuid::new_v4();
    
    AccountModel {
        id: account_id,
        product_id: Uuid::new_v4(),
        gl_code_suffix: None,
        account_type: DbAccountType::Savings,
        account_status: DbAccountStatus::Active,
        signing_condition: DbSigningCondition::AnyOwner,
        currency: HeaplessString::try_from("USD").unwrap(),
        open_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        domicile_agency_branch_id: domicile_agency_branch_id,
        current_balance: Decimal::from_str("1000.00").unwrap(),
        available_balance: Decimal::from_str("950.00").unwrap(),
        accrued_interest: Decimal::from_str("12.50").unwrap(),
        overdraft_limit: None,
        original_principal: None,
        outstanding_principal: None,
        loan_interest_rate: None,
        loan_term_months: None,
        disbursement_date: None,
        maturity_date: None,
        installment_amount: None,
        next_due_date: None,
        penalty_rate: None,
        collateral_id: None,
        loan_purpose_id: None,
        close_date: None,
        last_activity_date: Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()),
        dormancy_threshold_days: Some(365),
        reactivation_required: false,
        pending_closure_reason_id: None,
        last_disbursement_instruction_id: None,
        status_changed_by_person_id: None,
        status_change_reason_id: None,
        status_change_timestamp: None,
        // Reference fields (new)
        most_significant_account_hold_id: None,
        account_ownership_id: None,
        access01_account_relationship_id: None,
        access02_account_relationship_id: None,
        access03_account_relationship_id: None,
        access04_account_relationship_id: None,
        access05_account_relationship_id: None,
        access06_account_relationship_id: None,
        access07_account_relationship_id: None,
        access11_account_mandate_id: None,
        access12_account_mandate_id: None,
        access13_account_mandate_id: None,
        access14_account_mandate_id: None,
        access15_account_mandate_id: None,
        access16_account_mandate_id: None,
        access17_account_mandate_id: None,
        interest01_ultimate_beneficiary_id: None,
        interest02_ultimate_beneficiary_id: None,
        interest03_ultimate_beneficiary_id: None,
        interest04_ultimate_beneficiary_id: None,
        interest05_ultimate_beneficiary_id: None,
        interest06_ultimate_beneficiary_id: None,
        interest07_ultimate_beneficiary_id: None,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
        updated_by_person_id: updated_by_person_id,
    }
}

/// Test helper to create a loan account
#[allow(dead_code)]
fn create_test_loan_account() -> AccountModel {
    let account_id = Uuid::new_v4();
    // Use the same fixed UUID as the test person
    let updated_by_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let domicile_agency_branch_id = Uuid::new_v4();
    
    AccountModel {
        id: account_id,
        product_id: Uuid::new_v4(),
        gl_code_suffix: None,
        account_type: DbAccountType::Loan,
        account_status: DbAccountStatus::Active,
        signing_condition: DbSigningCondition::None,
        currency: HeaplessString::try_from("USD").unwrap(),
        open_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        domicile_agency_branch_id: domicile_agency_branch_id,
        current_balance: Decimal::from_str("5000.00").unwrap(), // Positive balance representing outstanding amount
        available_balance: Decimal::from_str("0.00").unwrap(), // Available is 0 for loans (can't withdraw)
        accrued_interest: Decimal::from_str("25.00").unwrap(),
        overdraft_limit: None,
        original_principal: Some(Decimal::from_str("10000.00").unwrap()),
        outstanding_principal: Some(Decimal::from_str("5000.00").unwrap()),
        loan_interest_rate: Some(Decimal::from_str("0.12").unwrap()), // 12% annual
        loan_term_months: Some(24),
        disbursement_date: Some(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()),
        maturity_date: Some(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()),
        installment_amount: Some(Decimal::from_str("469.70").unwrap()),
        next_due_date: Some(NaiveDate::from_ymd_opt(2024, 2, 15).unwrap()),
        penalty_rate: Some(Decimal::from_str("0.05").unwrap()), // 5% penalty
        collateral_id: None,
        loan_purpose_id: None,
        close_date: None,
        last_activity_date: Some(NaiveDate::from_ymd_opt(2024, 1, 10).unwrap()),
        dormancy_threshold_days: Some(30), // Loans have shorter dormancy period
        reactivation_required: false,
        pending_closure_reason_id: None,
        last_disbursement_instruction_id: None,
        status_changed_by_person_id: None,
        status_change_reason_id: None,
        status_change_timestamp: None,
        // Reference fields (new)
        most_significant_account_hold_id: None,
        account_ownership_id: None,
        access01_account_relationship_id: None,
        access02_account_relationship_id: None,
        access03_account_relationship_id: None,
        access04_account_relationship_id: None,
        access05_account_relationship_id: None,
        access06_account_relationship_id: None,
        access07_account_relationship_id: None,
        access11_account_mandate_id: None,
        access12_account_mandate_id: None,
        access13_account_mandate_id: None,
        access14_account_mandate_id: None,
        access15_account_mandate_id: None,
        access16_account_mandate_id: None,
        access17_account_mandate_id: None,
        interest01_ultimate_beneficiary_id: None,
        interest02_ultimate_beneficiary_id: None,
        interest03_ultimate_beneficiary_id: None,
        interest04_ultimate_beneficiary_id: None,
        interest05_ultimate_beneficiary_id: None,
        interest06_ultimate_beneficiary_id: None,
        interest07_ultimate_beneficiary_id: None,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
        updated_by_person_id: updated_by_person_id,
    }
}

/// Integration test helper to set up database connection
#[allow(dead_code)]
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL database");
    
    // Run migrations to ensure schema is up to date
    sqlx::migrate!("../banking-db-postgres/migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    // Create a test person for foreign key references
    let test_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    sqlx::query(
        r#"
        INSERT INTO persons (id, person_type, display_name, external_identifier)
        VALUES ($1, 'System', 'Test User', 'test-user')
        ON CONFLICT (id) DO NOTHING
        "#
    )
    .bind(test_person_id)
    .execute(&pool)
    .await
    .expect("Failed to create test person");
    
    pool
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_account_crud_operations() {
    use banking_db::AccountRepository;
    use banking_db_postgres::AccountRepositoryImpl;

    
    let pool = setup_test_db().await;
    let repo = AccountRepositoryImpl::new(pool);
    let mut account = create_test_account();
    
    // Test CREATE
    let created_account = repo.create(account.clone()).await
        .expect("Failed to create account");
    assert_eq!(created_account.id, account.id);
    assert_eq!(created_account.product_id, account.product_id);
    assert_eq!(created_account.account_type, account.account_type);
    
    // Test READ
    let found_account = repo.find_by_id(account.id).await
        .expect("Failed to find account")
        .expect("Account not found");
    assert_eq!(found_account.id, account.id);
    assert_eq!(found_account.current_balance, account.current_balance);
    
    // Test UPDATE
    account.current_balance = Decimal::from_str("1500.00").unwrap();
    account.available_balance = Decimal::from_str("1450.00").unwrap();
    let updated_account = repo.update(account.clone()).await
        .expect("Failed to update account");
    assert_eq!(updated_account.current_balance, account.current_balance);
    
    // Test EXISTS
    let exists = repo.exists(account.id).await
        .expect("Failed to check if account exists");
    assert!(exists);
    
    // Test with non-existent account
    let non_existent_id = Uuid::new_v4();
    let not_exists = repo.exists(non_existent_id).await
        .expect("Failed to check if account exists");
    assert!(!not_exists);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_account_balance_operations() {
    use banking_db::AccountRepository;
    use banking_db_postgres::AccountRepositoryImpl;


    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let account = create_test_account();
    
    // Create account first
    repo.create(account.clone()).await
        .expect("Failed to create account");
    
    // Test balance update
    let new_current = Decimal::from_str("2000.00").unwrap();
    let new_available = Decimal::from_str("1900.00").unwrap();
    
    repo.update_balance(account.id, new_current, new_available).await
        .expect("Failed to update balance");
    
    // Verify balance was updated
    let updated_account = repo.find_by_id(account.id).await
        .expect("Failed to find account")
        .expect("Account not found");
    assert_eq!(updated_account.current_balance, new_current);
    assert_eq!(updated_account.available_balance, new_available);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_accrued_interest_operations() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;

    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let account = create_test_account();
    
    // Create account first
    repo.create(account.clone()).await
        .expect("Failed to create account");
    
    // Test accrued interest update
    let new_interest = Decimal::from_str("25.75").unwrap();
    repo.update_accrued_interest(account.id, new_interest).await
        .expect("Failed to update accrued interest");
    
    // Verify interest was updated
    let updated_account = repo.find_by_id(account.id).await
        .expect("Failed to find account")
        .expect("Account not found");
    assert_eq!(updated_account.accrued_interest, new_interest);
    
    // Test reset accrued interest
    repo.reset_accrued_interest(account.id).await
        .expect("Failed to reset accrued interest");
    
    // Verify interest was reset to zero
    let reset_account = repo.find_by_id(account.id).await
        .expect("Failed to find account")
        .expect("Account not found");
    assert_eq!(reset_account.accrued_interest, Decimal::from_str("0.00").unwrap());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_account_status_operations() {
    use banking_db::AccountRepository;
    use banking_db_postgres::AccountRepositoryImpl;


    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let account = create_test_account();
    // Use the same test person UUID for changed_by
    let changed_by = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    
    // Create account first
    repo.create(account.clone()).await
        .expect("Failed to create account");
    
    // Test status update
    let reason_code = format!("hold-{}", Uuid::new_v4());
    repo.update_status(account.id, "Frozen", &reason_code, changed_by).await
        .expect("Failed to update status");
    
    // Verify status was updated
    let updated_account = repo.find_by_id(account.id).await
        .expect("Failed to find account")
        .expect("Account not found");
    assert_eq!(updated_account.account_status, DbAccountStatus::Frozen);
    assert_eq!(updated_account.status_changed_by_person_id, Some(changed_by));
    assert!(updated_account.status_change_timestamp.is_some());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_operations() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;

    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    
    // Use UUIDs to ensure truly unique product codes
    #[allow(unused_variables)]
    let product_id_1 = Uuid::new_v4();
    #[allow(unused_variables)]
    let product_id_2 = Uuid::new_v4();
    
    let mut account1 = create_test_account();
    account1.product_id = Uuid::new_v4();
    let mut account2 = create_test_account();
    account2.id = Uuid::new_v4();
    account2.product_id = Uuid::new_v4();
    account2.account_status = DbAccountStatus::Dormant;
    
    // Create accounts
    repo.create(account1.clone()).await.expect("Failed to create account1");
    repo.create(account2.clone()).await.expect("Failed to create account2");
    
    // Test find by product code
    let accounts_by_id = repo.find_by_product_id(account1.product_id).await
        .expect("Failed to find by product code");
    assert_eq!(accounts_by_id.len(), 1);
    assert_eq!(accounts_by_id[0].id, account1.id);
    
    // Test find by status
    let active_accounts = repo.find_by_status("Active").await
        .expect("Failed to find by status");
    assert!(active_accounts.iter().any(|a| a.id == account1.id));
    
    let dormant_accounts = repo.find_by_status("Dormant").await
        .expect("Failed to find by status");
    assert!(dormant_accounts.iter().any(|a| a.id == account2.id));
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_interest_bearing_accounts() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;

    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let savings_account = create_test_account(); // Savings account
    let loan_account = create_test_loan_account(); // Loan account
    let mut current_account = create_test_account();
    current_account.id = Uuid::new_v4();
    current_account.account_type = DbAccountType::Current;
    
    // Create accounts
    repo.create(savings_account.clone()).await.expect("Failed to create savings account");
    repo.create(loan_account.clone()).await.expect("Failed to create loan account");
    repo.create(current_account.clone()).await.expect("Failed to create current account");
    
    // Test find interest-bearing accounts
    let interest_accounts = repo.find_interest_bearing_accounts().await
        .expect("Failed to find interest-bearing accounts");
    
    // Should include savings and loan accounts, but not current account
    let account_ids: Vec<Uuid> = interest_accounts.iter().map(|a| a.id).collect();
    assert!(account_ids.contains(&savings_account.id));
    assert!(account_ids.contains(&loan_account.id));
    assert!(!account_ids.contains(&current_account.id));
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_dormancy_candidates() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;

    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let mut old_account = create_test_account();
    old_account.last_activity_date = Some(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap()); // Very old activity
    
    let mut recent_account = create_test_account();
    recent_account.id = Uuid::new_v4();
    recent_account.last_activity_date = Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()); // Recent activity
    
    // Create accounts
    repo.create(old_account.clone()).await.expect("Failed to create old account");
    repo.create(recent_account.clone()).await.expect("Failed to create recent account");
    
    // Test find dormancy candidates (accounts inactive for more than 300 days)
    let reference_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    let dormancy_candidates = repo.find_dormancy_candidates(reference_date, 300).await
        .expect("Failed to find dormancy candidates");
    
    // Should include old account but not recent account
    let candidate_ids: Vec<Uuid> = dormancy_candidates.iter().map(|a| a.id).collect();
    assert!(candidate_ids.contains(&old_account.id));
    assert!(!candidate_ids.contains(&recent_account.id));
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_count_operations() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;

    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    
    // Get initial count
    let initial_count = repo.count().await.expect("Failed to get count");
    
    // Create a few test accounts
    let account1 = create_test_account();
    let mut account2 = create_test_account();
    account2.id = Uuid::new_v4();
    account2.product_id = Uuid::new_v4();
    
    repo.create(account1.clone()).await.expect("Failed to create account1");
    repo.create(account2.clone()).await.expect("Failed to create account2");
    
    // Test total count (should have increased by at least 2)
    let new_count = repo.count().await.expect("Failed to get count");
    assert!(new_count >= initial_count + 2, "Expected count to increase by at least 2, initial: {}, new: {}", initial_count, new_count);
    
    // Test count by product
    let sav01_count = repo.count_by_product(account1.product_id).await
        .expect("Failed to count by product");
    assert!(sav01_count >= 1);
    
    let sav02_count = repo.count_by_product(account2.product_id).await
        .expect("Failed to count by product");
    assert!(sav02_count >= 1);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_list_with_pagination() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;

    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    
    // Test basic pagination functionality
    let limit = 3;
    
    // Get first page
    let first_page = repo.list(0, limit).await.expect("Failed to get first page");
    assert!(first_page.len() <= limit as usize, "First page should not exceed limit");
    
    // Get second page  
    let second_page = repo.list(limit, limit).await.expect("Failed to get second page");
    assert!(second_page.len() <= limit as usize, "Second page should not exceed limit");
    
    // Test edge case: empty page when offset is very large
    let empty_page = repo.list(10000, limit).await.expect("Failed to get empty page");
    assert!(empty_page.is_empty(), "Page with very large offset should be empty");
    
    // Test pagination consistency: same results for same parameters
    let page1_attempt1 = repo.list(0, limit).await.expect("Failed to get page (attempt 1)");
    let page1_attempt2 = repo.list(0, limit).await.expect("Failed to get page (attempt 2)");
    
    // Should get same accounts (same count and order due to deterministic ordering)
    assert_eq!(page1_attempt1.len(), page1_attempt2.len(), 
               "Same pagination parameters should return same number of results");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_last_activity_date_update() {
    use banking_db::AccountRepository;
    use banking_db_postgres::AccountRepositoryImpl;


    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let account = create_test_account();
    
    // Create account first
    repo.create(account.clone()).await
        .expect("Failed to create account");
    
    // Update last activity date
    let new_activity_date = NaiveDate::from_ymd_opt(2024, 2, 1).unwrap();
    repo.update_last_activity_date(account.id, new_activity_date).await
        .expect("Failed to update last activity date");
    
    // Verify update
    let updated_account = repo.find_by_id(account.id).await
        .expect("Failed to find account")
        .expect("Account not found");
    assert_eq!(updated_account.last_activity_date, Some(new_activity_date));
}

// Unit tests for row conversion functions (no database required)
#[tokio::test]
async fn test_account_model_validation() {
    // This test is no longer relevant as we are using Uuid for product_id
    // and the HeaplessString validation is not needed.
}

#[tokio::test]
async fn test_enum_conversions() {
    // Test AccountType enum (Debug format for now)
    assert_eq!(format!("{:?}", DbAccountType::Savings), "Savings");
    assert_eq!(format!("{:?}", DbAccountType::Current), "Current");
    assert_eq!(format!("{:?}", DbAccountType::Loan), "Loan");
    
    // Test AccountStatus enum (Debug format for now)
    assert_eq!(format!("{:?}", DbAccountStatus::Active), "Active");
    assert_eq!(format!("{:?}", DbAccountStatus::Frozen), "Frozen");
    assert_eq!(format!("{:?}", DbAccountStatus::Closed), "Closed");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_account_status_history() {
    use banking_db_postgres::AccountRepositoryImpl;
    use banking_db::AccountRepository;
    
    let pool = setup_test_db().await;
    #[allow(unused_variables)]
    let repo = AccountRepositoryImpl::new(pool);
    let account = create_test_account();
    let changed_by = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();

    // Create account first
    repo.create(account.clone()).await
        .expect("Failed to create account");

    // Create a test reason to satisfy the foreign key constraint
    let reason_id = Uuid::new_v4();
    let reason_code = format!("test_reason-{}", Uuid::new_v4());
    sqlx::query(
        r#"
        INSERT INTO reason_and_purpose (id, category, context, code, l1_content, is_active, created_by_person_id, updated_by_person_id)
        VALUES ($1, 'StatusChange', 'Account', $2, 'A test reason for status change', true, '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001')
        "#
    )
    .bind(reason_id)
    .bind(reason_code)
    .execute(&repo.pool)
    .await
    .expect("Failed to create test reason");

    // Add a status change record
    let status_change = banking_db::models::AccountStatusChangeRecordModel {
        id: Uuid::new_v4(),
        account_id: account.id,
        old_status: Some(DbAccountStatus::Active),
        new_status: DbAccountStatus::Frozen,
        reason_id: reason_id,
        additional_context: Some("Test freeze".try_into().unwrap()),
        changed_by_person_id: changed_by,
        changed_at: Utc::now(),
        system_triggered: false,
        created_at: Utc::now(),
    };
    repo.add_status_change(status_change.clone()).await.expect("Failed to add status change");

    // Get status history
    let history = repo.get_status_history(account.id).await.expect("Failed to get status history");

    assert!(!history.is_empty());
    assert_eq!(history[0].new_status, DbAccountStatus::Frozen);
}