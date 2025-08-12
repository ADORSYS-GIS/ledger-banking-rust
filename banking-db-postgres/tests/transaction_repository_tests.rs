use banking_db::{DbAccountStatus, DbAccountType, DbSigningCondition};
use banking_db::models::{TransactionModel, TransactionType, TransactionStatus, AccountModel};
use chrono::{NaiveDate, Utc};
use heapless::String as HeaplessString;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

/// Test helper to create a sample transaction
#[allow(dead_code)]
fn create_test_transaction(account_id: Uuid) -> TransactionModel {
    let transaction_id = Uuid::new_v4();
    
    TransactionModel {
        id: transaction_id,
        account_id,
        transaction_code: HeaplessString::try_from("DEPOSIT").unwrap(),
        transaction_type: TransactionType::Credit,
        amount: Decimal::from_str("100.00").unwrap(),
        currency: HeaplessString::try_from("USD").unwrap(),
        description: HeaplessString::try_from("Test deposit transaction").unwrap(),
        channel_id: HeaplessString::try_from("ATM").unwrap(),
        terminal_id: Some(Uuid::new_v4()),
        agent_person_id: None,
        transaction_date: Utc::now(),
        value_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        status: TransactionStatus::Pending,
        // Use transaction_id and timestamp for truly unique reference numbers
        reference_number: HeaplessString::try_from(
            format!("T{}{}", 
                transaction_id.to_string().replace("-", "")[0..8].to_uppercase(),
                Utc::now().timestamp_micros() % 10000
            ).as_str()
        ).unwrap(),
        external_reference: Some(HeaplessString::try_from(
            format!("E{}{}", 
                transaction_id.to_string().replace("-", "")[0..6].to_uppercase(),
                Utc::now().timestamp_micros() % 100
            ).as_str()
        ).unwrap()),
        gl_code: HeaplessString::try_from("1001").unwrap(),
        requires_approval: false,
        approval_status: None,
        risk_score: Some(Decimal::from_str("25.5").unwrap()),
        created_at: Utc::now(),
    }
}

/// Test helper to create a sample account for transaction testing
#[allow(dead_code)]
fn create_test_account() -> AccountModel {
    let account_id = Uuid::new_v4();
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

/// Integration test helper to set up database connection and prerequisites
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
    
    // Create test person for foreign key references
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

/// Create a test account in the database
#[allow(dead_code)]
async fn create_test_account_in_db(pool: &PgPool) -> Uuid {
    let account = create_test_account();
    let account_id = account.id;
    
    sqlx::query(
        r#"
        INSERT INTO accounts (
            id, product_id, account_type, account_status, signing_condition,
            currency, open_date, domicile_agency_branch_id, current_balance, available_balance,
            accrued_interest, updated_by_person_id
        )
        VALUES (
            $1, $2, $3::account_type, $4::account_status, $5::signing_condition,
            $6, $7, $8, $9, $10, $11, $12
        )
        ON CONFLICT (id) DO NOTHING
        "#
    )
    .bind(account.id)
    .bind(account.product_id)
    .bind(format!("{:?}", account.account_type))
    .bind(format!("{:?}", account.account_status))
    .bind(format!("{:?}", account.signing_condition))
    .bind(account.currency.as_str())
    .bind(account.open_date)
    .bind(account.domicile_agency_branch_id)
    .bind(account.current_balance)
    .bind(account.available_balance)
    .bind(account.accrued_interest)
    .bind(account.updated_by_person_id)
    .execute(pool)
    .await
    .expect("Failed to create test account");
    
    account_id
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_crud_operations() {
    use banking_db::TransactionRepository;
    use banking_db_postgres::TransactionRepositoryImpl;
    
    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    let mut transaction = create_test_transaction(account_id);
    
    // Test CREATE
    let created_transaction = repo.create(transaction.clone()).await
        .expect("Failed to create transaction");
    assert_eq!(created_transaction.id, transaction.id);
    assert_eq!(created_transaction.account_id, transaction.account_id);
    assert_eq!(created_transaction.amount, transaction.amount);
    assert_eq!(created_transaction.transaction_type, transaction.transaction_type);
    assert_eq!(created_transaction.status, transaction.status);
    
    // Test READ by ID
    let found_transaction = repo.find_by_id(transaction.id).await
        .expect("Failed to find transaction")
        .expect("Transaction not found");
    assert_eq!(found_transaction.id, transaction.id);
    assert_eq!(found_transaction.amount, transaction.amount);
    
    // Test UPDATE
    transaction.status = TransactionStatus::Posted;
    transaction.amount = Decimal::from_str("150.00").unwrap();
    let updated_transaction = repo.update(transaction.clone()).await
        .expect("Failed to update transaction");
    assert_eq!(updated_transaction.status, TransactionStatus::Posted);
    assert_eq!(updated_transaction.amount, Decimal::from_str("150.00").unwrap());
    
    // Test EXISTS
    let exists = repo.exists(transaction.id).await
        .expect("Failed to check if transaction exists");
    assert!(exists);
    
    // Test with non-existent transaction
    let non_existent_id = Uuid::new_v4();
    let not_exists = repo.exists(non_existent_id).await
        .expect("Failed to check if transaction exists");
    assert!(!not_exists);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_find_by_reference() {
    use banking_db::TransactionRepository;
    use banking_db_postgres::TransactionRepositoryImpl;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    let transaction = create_test_transaction(account_id);
    
    // Create transaction
    repo.create(transaction.clone()).await
        .expect("Failed to create transaction");
    
    // Test find by reference number
    let found_transaction = repo.find_by_reference(transaction.reference_number.as_str()).await
        .expect("Failed to find transaction by reference")
        .expect("Transaction not found by reference");
    assert_eq!(found_transaction.id, transaction.id);
    assert_eq!(found_transaction.reference_number, transaction.reference_number);
    
    // Test with non-existent reference
    let not_found = repo.find_by_reference("NONEXISTENT").await
        .expect("Failed to search for non-existent reference");
    assert!(not_found.is_none());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_find_by_account_id() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create multiple transactions for the same account with unique reference numbers
    let mut transaction1 = create_test_transaction(account_id);
    transaction1.reference_number = HeaplessString::try_from(
        format!("TXN1{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction1.amount = Decimal::from_str("100.00").unwrap();
    
    let mut transaction2 = create_test_transaction(account_id);
    transaction2.reference_number = HeaplessString::try_from(
        format!("TXN2{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction2.amount = Decimal::from_str("200.00").unwrap();
    transaction2.value_date = NaiveDate::from_ymd_opt(2024, 1, 16).unwrap();
    
    repo.create(transaction1.clone()).await
        .expect("Failed to create transaction1");
    repo.create(transaction2.clone()).await
        .expect("Failed to create transaction2");
    
    // Test find by account ID (no date filter)
    let transactions = repo.find_by_account_id(account_id, None, None).await
        .expect("Failed to find transactions by account ID");
    assert_eq!(transactions.len(), 2);
    
    // Test find by account ID with date range
    let from_date = NaiveDate::from_ymd_opt(2024, 1, 16).unwrap();
    let to_date = NaiveDate::from_ymd_opt(2024, 1, 16).unwrap();
    let filtered_transactions = repo.find_by_account_date_range(account_id, from_date, to_date).await
        .expect("Failed to find transactions by account date range");
    assert_eq!(filtered_transactions.len(), 1);
    assert_eq!(filtered_transactions[0].amount, Decimal::from_str("200.00").unwrap());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_find_by_external_reference() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create transactions with same external reference but unique reference numbers
    let external_ref = format!("BATCH{}", Utc::now().timestamp_micros() % 1000000);
    let mut transaction1 = create_test_transaction(account_id);
    transaction1.reference_number = HeaplessString::try_from(
        format!("TXN1{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction1.external_reference = Some(HeaplessString::try_from(external_ref.as_str()).unwrap());
    
    let mut transaction2 = create_test_transaction(account_id);
    transaction2.reference_number = HeaplessString::try_from(
        format!("TXN2{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction2.external_reference = Some(HeaplessString::try_from(external_ref.as_str()).unwrap());
    
    repo.create(transaction1.clone()).await
        .expect("Failed to create transaction1");
    repo.create(transaction2.clone()).await
        .expect("Failed to create transaction2");
    
    // Test find by external reference
    let transactions = repo.find_by_external_reference(&external_ref).await
        .expect("Failed to find transactions by external reference");
    assert_eq!(transactions.len(), 2);
    
    // Verify both transactions have the correct external reference
    for transaction in &transactions {
        assert_eq!(transaction.external_reference.as_ref().unwrap().as_str(), external_ref);
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_find_by_status() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create transactions with different statuses and unique reference numbers
    let mut transaction1 = create_test_transaction(account_id);
    transaction1.reference_number = HeaplessString::try_from(
        format!("PEND{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction1.status = TransactionStatus::Pending;
    
    let mut transaction2 = create_test_transaction(account_id);
    transaction2.reference_number = HeaplessString::try_from(
        format!("POST{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction2.status = TransactionStatus::Posted;
    
    repo.create(transaction1.clone()).await
        .expect("Failed to create pending transaction");
    repo.create(transaction2.clone()).await
        .expect("Failed to create posted transaction");
    
    // Test find by pending status
    let pending_transactions = repo.find_by_status("Pending").await
        .expect("Failed to find pending transactions");
    let pending_count = pending_transactions.iter().filter(|t| t.status == TransactionStatus::Pending).count();
    assert!(pending_count >= 1);
    
    // Test find by posted status
    let posted_transactions = repo.find_by_status("Posted").await
        .expect("Failed to find posted transactions");
    let posted_count = posted_transactions.iter().filter(|t| t.status == TransactionStatus::Posted).count();
    assert!(posted_count >= 1);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_requiring_approval() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;
    use banking_db::TransactionApprovalStatus;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create transaction requiring approval with unique reference number
    let mut transaction = create_test_transaction(account_id);
    transaction.reference_number = HeaplessString::try_from(
        format!("APPR{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction.requires_approval = true;
    transaction.approval_status = Some(TransactionApprovalStatus::Pending);
    
    repo.create(transaction.clone()).await
        .expect("Failed to create transaction requiring approval");
    
    // Test find transactions requiring approval
    let requiring_approval = repo.find_requiring_approval().await
        .expect("Failed to find transactions requiring approval");
    
    // Find our specific transaction in the results
    let our_transaction = requiring_approval.iter()
        .find(|t| t.id == transaction.id);
    assert!(our_transaction.is_some());
    
    let found_transaction = our_transaction.unwrap();
    assert!(found_transaction.requires_approval);
    assert_eq!(found_transaction.approval_status, Some(TransactionApprovalStatus::Pending));
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_status_updates() {
    use banking_db::TransactionApprovalStatus;
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    
    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    let transaction = create_test_transaction(account_id);
    
    // Create transaction
    repo.create(transaction.clone()).await
        .expect("Failed to create transaction");
    
    // Test status update
    repo.update_status(transaction.id, "Posted", "Approved by system").await
        .expect("Failed to update transaction status");
    
    // Verify status was updated
    let updated_transaction = repo.find_by_id(transaction.id).await
        .expect("Failed to find updated transaction")
        .expect("Transaction not found");
    assert_eq!(updated_transaction.status, TransactionStatus::Posted);
    
    // Test approval status update
    repo.update_approval_status(transaction.id, "Approved").await
        .expect("Failed to update approval status");
    
    // Verify approval status was updated
    let approved_transaction = repo.find_by_id(transaction.id).await
        .expect("Failed to find approved transaction")
        .expect("Transaction not found");
    assert_eq!(approved_transaction.approval_status, Some(TransactionApprovalStatus::Approved));
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_find_by_channel() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create transactions with specific channel and unique reference numbers
    let channel = "MobileApp";
    let mut transaction1 = create_test_transaction(account_id);
    transaction1.reference_number = HeaplessString::try_from(
        format!("MOB{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction1.channel_id = HeaplessString::try_from(channel).unwrap();
    
    let mut transaction2 = create_test_transaction(account_id);
    transaction2.reference_number = HeaplessString::try_from(
        format!("ATM{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction2.channel_id = HeaplessString::try_from("ATM").unwrap();
    
    repo.create(transaction1.clone()).await
        .expect("Failed to create mobile transaction");
    repo.create(transaction2.clone()).await
        .expect("Failed to create ATM transaction");
    
    // Test find by channel
    let mobile_transactions = repo.find_by_channel(channel, None, None).await
        .expect("Failed to find transactions by channel");
    
    // Find our specific transaction in the results
    let our_transaction = mobile_transactions.iter()
        .find(|t| t.id == transaction1.id);
    assert!(our_transaction.is_some());
    assert_eq!(our_transaction.unwrap().channel_id.as_str(), channel);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_find_last_customer_transaction() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create customer transaction (non-system channel) with unique reference number
    let mut customer_transaction = create_test_transaction(account_id);
    customer_transaction.reference_number = HeaplessString::try_from(
        format!("CUST{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    customer_transaction.channel_id = HeaplessString::try_from("MobileApp").unwrap();
    customer_transaction.transaction_date = Utc::now();
    
    // Create system transaction (should be ignored) with unique reference number
    let mut system_transaction = create_test_transaction(account_id);
    system_transaction.reference_number = HeaplessString::try_from(
        format!("SYS{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    system_transaction.channel_id = HeaplessString::try_from("System").unwrap();
    system_transaction.transaction_date = Utc::now() + chrono::Duration::hours(1);
    
    repo.create(customer_transaction.clone()).await
        .expect("Failed to create customer transaction");
    repo.create(system_transaction.clone()).await
        .expect("Failed to create system transaction");
    
    // Test find last customer transaction (should return customer_transaction, not system_transaction)
    let last_transaction = repo.find_last_customer_transaction(account_id).await
        .expect("Failed to find last customer transaction")
        .expect("No customer transaction found");
    
    assert_eq!(last_transaction.id, customer_transaction.id);
    assert_eq!(last_transaction.channel_id.as_str(), "MobileApp");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_reverse_transaction() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create original transaction
    let mut original_transaction = create_test_transaction(account_id);
    original_transaction.reference_number = HeaplessString::try_from(
        format!("ORIG{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    original_transaction.status = TransactionStatus::Posted;
    
    repo.create(original_transaction.clone()).await
        .expect("Failed to create original transaction");
    
    // Create reversal transaction (use positive amount, opposite transaction type)
    let mut reversal_transaction = create_test_transaction(account_id);
    reversal_transaction.id = Uuid::new_v4();
    reversal_transaction.reference_number = HeaplessString::try_from(
        format!("REV{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    reversal_transaction.amount = original_transaction.amount; // Use same positive amount
    // Reverse the transaction type for accounting purposes
    reversal_transaction.transaction_type = match original_transaction.transaction_type {
        TransactionType::Credit => TransactionType::Debit,
        TransactionType::Debit => TransactionType::Credit,
    };
    reversal_transaction.description = HeaplessString::try_from("Reversal of ORIG001").unwrap();
    
    // Test reverse transaction
    let created_reversal = repo.reverse_transaction(
        original_transaction.id, 
        reversal_transaction.clone()
    ).await
        .expect("Failed to reverse transaction");
    
    assert_eq!(created_reversal.id, reversal_transaction.id);
    assert_eq!(created_reversal.amount, reversal_transaction.amount);
    assert_eq!(created_reversal.transaction_type, reversal_transaction.transaction_type);
    
    // Verify original transaction status was updated to Reversed
    let updated_original = repo.find_by_id(original_transaction.id).await
        .expect("Failed to find original transaction")
        .expect("Original transaction not found");
    assert_eq!(updated_original.status, TransactionStatus::Reversed);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_reconciliation() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create transactions for reconciliation
    let channel = "ATM";
    let reconciliation_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
    
    let mut transaction1 = create_test_transaction(account_id);
    transaction1.reference_number = HeaplessString::try_from(
        format!("REC1{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction1.channel_id = HeaplessString::try_from(channel).unwrap();
    transaction1.value_date = reconciliation_date;
    transaction1.status = TransactionStatus::Posted;
    
    let mut transaction2 = create_test_transaction(account_id);
    transaction2.reference_number = HeaplessString::try_from(
        format!("REC2{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction2.channel_id = HeaplessString::try_from(channel).unwrap();
    transaction2.value_date = reconciliation_date;
    transaction2.status = TransactionStatus::Pending;
    
    repo.create(transaction1.clone()).await
        .expect("Failed to create reconciliation transaction 1");
    repo.create(transaction2.clone()).await
        .expect("Failed to create reconciliation transaction 2");
    
    // Test find for reconciliation
    let reconciliation_transactions = repo.find_for_reconciliation(channel, reconciliation_date).await
        .expect("Failed to find transactions for reconciliation");
    
    // Should find both transactions (Posted and Pending statuses)
    let found_txn1 = reconciliation_transactions.iter()
        .find(|t| t.id == transaction1.id);
    let found_txn2 = reconciliation_transactions.iter()
        .find(|t| t.id == transaction2.id);
    
    assert!(found_txn1.is_some());
    assert!(found_txn2.is_some());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_count_operations() {
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Get initial account count (should be 0 for new account)
    let initial_account_count = repo.count_by_account(account_id, None, None).await
        .expect("Failed to get initial account count");
    
    // Create some transactions for this specific account with unique reference numbers
    let mut transaction1 = create_test_transaction(account_id);
    transaction1.reference_number = HeaplessString::try_from(
        format!("CNT1{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    let mut transaction2 = create_test_transaction(account_id);
    transaction2.reference_number = HeaplessString::try_from(
        format!("CNT2{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    
    repo.create(transaction1.clone()).await
        .expect("Failed to create transaction 1");
    repo.create(transaction2.clone()).await
        .expect("Failed to create transaction 2");
    
    // Test count by account (should be exactly 2 more than initial)
    let new_account_count = repo.count_by_account(account_id, None, None).await
        .expect("Failed to get new account count");
    assert_eq!(new_account_count, initial_account_count + 2, 
               "Account should have exactly 2 more transactions");
    
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
    
    // Should get same transactions (same count and order due to deterministic ordering)
    assert_eq!(page1_attempt1.len(), page1_attempt2.len(), 
               "Same pagination parameters should return same number of results");
    
    // Test total count is reasonable (should be at least our 2 transactions)
    let total_count = repo.count().await.expect("Failed to get total count");
    assert!(total_count >= 2, "Total count should be at least 2 (our transactions)");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_with_approval_workflow() {
    use banking_db::TransactionApprovalStatus;
    use banking_db::ApprovalWorkflowModel;
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;
    use banking_db::WorkflowStatusModel;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    
    // Create transaction requiring approval with unique reference number
    let mut transaction = create_test_transaction(account_id);
    transaction.reference_number = HeaplessString::try_from(
        format!("WF{}", Utc::now().timestamp_micros() % 100000).as_str()
    ).unwrap();
    transaction.requires_approval = true;
    transaction.approval_status = Some(TransactionApprovalStatus::Pending);
    
    let created_transaction = repo.create(transaction.clone()).await
        .expect("Failed to create transaction");
    
    // Create approval workflow
    let workflow = ApprovalWorkflowModel {
        id: Uuid::new_v4(),
        transaction_id: Some(created_transaction.id),
        account_id: Some(account_id),
        approval_type: HeaplessString::try_from("TransactionApproval").unwrap(),
        minimum_approvals: 1,
        current_approvals: 0,
        status: WorkflowStatusModel::InProgress,
        initiated_by: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        initiated_at: Utc::now(),
        timeout_at: Utc::now() + chrono::Duration::hours(24),
        completed_at: None,
        rejection_reason_id: None,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
    };
    
    let created_workflow = repo.create_workflow(workflow.clone()).await
        .expect("Failed to create workflow");
    assert_eq!(created_workflow.id, workflow.id);
    
    // Test find workflow by ID
    let found_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    assert_eq!(found_workflow.id, workflow.id);
    
    // Test find pending workflows
    let pending_workflows = repo.find_pending_workflows().await
        .expect("Failed to find pending workflows");
    let our_workflow = pending_workflows.iter()
        .find(|w| w.id == workflow.id);
    assert!(our_workflow.is_some());
    
    // Test update workflow status
    repo.update_workflow_status(workflow.id, "Completed").await
        .expect("Failed to update workflow status");
    
    // Verify status was updated
    let updated_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find updated workflow")
        .expect("Updated workflow not found");
    assert_eq!(updated_workflow.status, WorkflowStatusModel::Completed);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_transaction_approval_operations() {
    use banking_db::{ApprovalWorkflowModel, WorkflowStatusModel, WorkflowTransactionApprovalModel};
    use banking_db_postgres::TransactionRepositoryImpl;
    use banking_db::TransactionRepository;

    let pool = setup_test_db().await;
    let repo = TransactionRepositoryImpl::new(pool.clone());
    
    // Create test account first
    let account_id = create_test_account_in_db(&pool).await;
    let transaction = create_test_transaction(account_id);
    
    // Create transaction and workflow first
    let created_transaction = repo.create(transaction.clone()).await
        .expect("Failed to create transaction");
    
    let workflow = ApprovalWorkflowModel {
        id: Uuid::new_v4(),
        transaction_id: Some(created_transaction.id),
        account_id: Some(account_id),
        approval_type: HeaplessString::try_from("TransactionApproval").unwrap(),
        minimum_approvals: 2,
        current_approvals: 0,
        status: WorkflowStatusModel::InProgress,
        initiated_by: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        initiated_at: Utc::now(),
        timeout_at: Utc::now() + chrono::Duration::hours(24),
        completed_at: None,
        rejection_reason_id: None,
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
    };
    
    let created_workflow = repo.create_workflow(workflow.clone()).await
        .expect("Failed to create workflow");
    
    // Create transaction approval
    let approval = WorkflowTransactionApprovalModel {
        id: Uuid::new_v4(),
        workflow_id: created_workflow.id,
        transaction_id: created_transaction.id,
        approver_person_id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        approval_action: HeaplessString::try_from("Approved").unwrap(),
        approved_at: Utc::now(),
        approval_notes: Some(HeaplessString::try_from("Looks good to approve").unwrap()),
        approval_method: HeaplessString::try_from("Manual").unwrap(),
        approval_location: None,
        created_at: Utc::now(),
    };
    
    let created_approval = repo.create_approval(approval.clone()).await
        .expect("Failed to create approval");
    assert_eq!(created_approval.id, approval.id);
    
    // Test find approvals by workflow
    let workflow_approvals = repo.find_approvals_by_workflow(created_workflow.id).await
        .expect("Failed to find approvals by workflow");
    assert_eq!(workflow_approvals.len(), 1);
    assert_eq!(workflow_approvals[0].id, approval.id);
    
    // Test find approvals by approver
    let approver_approvals = repo.find_approvals_by_approver(approval.approver_person_id).await
        .expect("Failed to find approvals by approver");
    let our_approval = approver_approvals.iter()
        .find(|a| a.id == approval.id);
    assert!(our_approval.is_some());
    
    // Test count approvals for workflow
    let approval_count = repo.count_approvals_for_workflow(created_workflow.id).await
        .expect("Failed to count approvals for workflow");
    assert_eq!(approval_count, 1);
}