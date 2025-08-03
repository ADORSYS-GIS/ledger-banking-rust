#[cfg(feature = "postgres_tests")]
mod fee_repository_tests {
    use banking_api::domain::fee::{
        FeeType, FeeCategory, FeeCalculationMethod, FeeTriggerEvent, FeeApplicationStatus
    };
    use banking_db::models::{FeeApplicationModel, FeeWaiverModel};
    use banking_db::repository::FeeRepository;
    use banking_db_postgres::FeeRepositoryImpl;
    use chrono::{NaiveDate, Utc};
    use heapless::String as HeaplessString;
    use rust_decimal::Decimal;
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    mod commons;

    /// Test helper to create a sample fee application
    fn create_test_fee_application() -> FeeApplicationModel {
        let fee_application_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let transaction_id = Some(Uuid::new_v4());
        // Use a fixed UUID that we'll insert into persons table in setup
        let applied_by = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        
        FeeApplicationModel {
            fee_application_id,
            account_id,
            transaction_id,
            fee_type: FeeType::EventBased,
            fee_category: FeeCategory::Transaction,
            product_code: HeaplessString::try_from("SAV01").unwrap(),
            fee_code: HeaplessString::try_from("ATM_WD").unwrap(),
            description: HeaplessString::try_from("ATM Withdrawal Fee").unwrap(),
            amount: Decimal::from_str("2.50").unwrap(),
            currency: HeaplessString::try_from("USD").unwrap(),
            calculation_method: FeeCalculationMethod::Fixed,
            calculation_base_amount: Some(Decimal::from_str("100.00").unwrap()),
            fee_rate: None,
            trigger_event: FeeTriggerEvent::AtmWithdrawal,
            status: FeeApplicationStatus::Applied,
            applied_at: Utc::now(),
            value_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            reversal_deadline: None,
            waived: false,
            waived_by: None,
            waived_reason_id: None,
            applied_by,
            created_at: Utc::now(),
        }
    }

    /// Test helper to create a sample fee waiver
    fn create_test_fee_waiver(fee_application_id: Uuid, account_id: Uuid) -> FeeWaiverModel {
        let waiver_id = Uuid::new_v4();
        let reason_id = Uuid::new_v4(); // We'll need to create this in setup
        let waived_by = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        
        FeeWaiverModel {
            waiver_id,
            fee_application_id,
            account_id,
            waived_amount: Decimal::from_str("2.50").unwrap(),
            reason_id,
            additional_details: Some(HeaplessString::try_from("Customer goodwill gesture").unwrap()),
            waived_by,
            waived_at: Utc::now(),
            approval_required: false,
            approved_by: None,
            approved_at: None,
        }
    }

    /// Set up test database with prerequisite data
    async fn setup_test_db() -> (PgPool, Uuid, Uuid) {
        let pool = commons::establish_connection().await;
        
        // Clean up before starting tests
        cleanup_database(&pool).await;
        
        // Create prerequisite persons for foreign key references
        let test_person_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
        sqlx::query(
            "INSERT INTO persons (person_id, person_type, display_name, external_identifier)
             VALUES ($1, 'system', 'Test User', 'test-user')
             ON CONFLICT (person_id) DO NOTHING"
        )
        .bind(test_person_id)
        .execute(&pool)
        .await
        .expect("Failed to create test person");
        
        // Create test account for foreign key references
        let test_account_id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO accounts (
                account_id, product_code, account_type, account_status, signing_condition,
                currency, open_date, domicile_branch_id, current_balance, available_balance,
                accrued_interest, overdraft_limit, updated_by
            ) VALUES (
                $1, 'SAV01', 'Savings', 'Active', 'AnyOwner', 'USD', '2024-01-15',
                $2, 1000.00, 950.00, 12.50, NULL, $3
            ) ON CONFLICT (account_id) DO NOTHING"#
        )
        .bind(test_account_id)
        .bind(Uuid::new_v4()) // domicile_branch_id
        .bind(test_person_id)
        .execute(&pool)
        .await
        .expect("Failed to create test account");
        
        // Create test reason for fee waivers
        let test_reason_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO reason_and_purpose (id, reason_category, reason_code, reason_description)
             VALUES ($1, 'FEE_WAIVER', 'GOODWILL', 'Customer goodwill gesture')
             ON CONFLICT (id) DO NOTHING"
        )
        .bind(test_reason_id)
        .execute(&pool)
        .await
        .expect("Failed to create test reason");
        
        (pool, test_person_id, test_account_id)
    }

    /// Cleanup database for test isolation
    async fn cleanup_database(pool: &PgPool) {
        let _ = sqlx::query("DELETE FROM fee_waivers").execute(pool).await;
        let _ = sqlx::query("DELETE FROM fee_applications").execute(pool).await;
        let _ = sqlx::query("DELETE FROM accounts WHERE product_code = 'SAV01'").execute(pool).await;
        let _ = sqlx::query("DELETE FROM reason_and_purpose WHERE reason_category = 'FEE_WAIVER'").execute(pool).await;
    }

    #[tokio::test]
    async fn test_create_fee_application() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        
        let result = repo.create_fee_application(fee_app.clone()).await;
        assert!(result.is_ok(), "Should create fee application successfully");
        
        let created = result.unwrap();
        assert_eq!(created.fee_application_id, fee_app.fee_application_id);
        assert_eq!(created.account_id, account_id);
        assert_eq!(created.amount, fee_app.amount);
        assert_eq!(created.fee_category, FeeCategory::Transaction);
        assert_eq!(created.status, FeeApplicationStatus::Applied);
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_fee_application_by_id() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        
        // Create the fee application first
        let created = repo.create_fee_application(fee_app.clone()).await
            .expect("Should create fee application");
        
        // Test retrieving by ID
        let result = repo.get_fee_application_by_id(created.fee_application_id).await;
        assert!(result.is_ok(), "Should retrieve fee application successfully");
        
        let retrieved = result.unwrap();
        assert!(retrieved.is_some(), "Should find the fee application");
        
        let found = retrieved.unwrap();
        assert_eq!(found.fee_application_id, created.fee_application_id);
        assert_eq!(found.account_id, account_id);
        assert_eq!(found.amount, created.amount);
        
        // Test retrieving non-existent ID
        let non_existent_id = Uuid::new_v4();
        let result = repo.get_fee_application_by_id(non_existent_id).await;
        assert!(result.is_ok(), "Should handle non-existent ID gracefully");
        assert!(result.unwrap().is_none(), "Should return None for non-existent ID");
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_update_fee_application() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        
        // Create the fee application first
        let created = repo.create_fee_application(fee_app.clone()).await
            .expect("Should create fee application");
        
        // Update the fee application
        let mut updated_app = created.clone();
        updated_app.status = FeeApplicationStatus::Waived;
        updated_app.waived = true;
        updated_app.amount = Decimal::from_str("0.00").unwrap();
        
        let result = repo.update_fee_application(updated_app.clone()).await;
        assert!(result.is_ok(), "Should update fee application successfully");
        
        let updated = result.unwrap();
        assert_eq!(updated.fee_application_id, created.fee_application_id);
        assert_eq!(updated.status, FeeApplicationStatus::Waived);
        assert_eq!(updated.waived, true);
        assert_eq!(updated.amount, Decimal::from_str("0.00").unwrap());
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_fee_applications_for_account() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create multiple fee applications for the same account
        let mut fee_app1 = create_test_fee_application();
        fee_app1.account_id = account_id;
        fee_app1.fee_code = HeaplessString::try_from("ATM_WD").unwrap();
        fee_app1.value_date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        
        let mut fee_app2 = create_test_fee_application();
        fee_app2.account_id = account_id;
        fee_app2.fee_application_id = Uuid::new_v4();
        fee_app2.fee_code = HeaplessString::try_from("MAINT").unwrap();
        fee_app2.status = FeeApplicationStatus::Pending;
        fee_app2.value_date = NaiveDate::from_ymd_opt(2024, 1, 20).unwrap();
        
        repo.create_fee_application(fee_app1.clone()).await
            .expect("Should create first fee application");
        repo.create_fee_application(fee_app2.clone()).await
            .expect("Should create second fee application");
        
        // Test getting all applications for account
        let result = repo.get_fee_applications_for_account(account_id, None, None, None).await;
        assert!(result.is_ok(), "Should retrieve fee applications successfully");
        
        let applications = result.unwrap();
        assert_eq!(applications.len(), 2, "Should find both fee applications");
        
        // Test filtering by status
        let result = repo.get_fee_applications_for_account(
            account_id, None, None, Some("Applied".to_string())
        ).await;
        assert!(result.is_ok(), "Should retrieve filtered applications successfully");
        
        let filtered = result.unwrap();
        assert_eq!(filtered.len(), 1, "Should find only applied fee applications");
        assert_eq!(filtered[0].status, FeeApplicationStatus::Applied);
        
        // Test date filtering
        let from_date = NaiveDate::from_ymd_opt(2024, 1, 18).unwrap();
        let result = repo.get_fee_applications_for_account(
            account_id, Some(from_date), None, None
        ).await;
        assert!(result.is_ok(), "Should retrieve date-filtered applications successfully");
        
        let date_filtered = result.unwrap();
        assert_eq!(date_filtered.len(), 1, "Should find only applications after date");
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_fee_applications_by_status() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create fee applications with different statuses
        let mut fee_app1 = create_test_fee_application();
        fee_app1.account_id = account_id;
        fee_app1.status = FeeApplicationStatus::Applied;
        
        let mut fee_app2 = create_test_fee_application();
        fee_app2.account_id = account_id;
        fee_app2.fee_application_id = Uuid::new_v4();
        fee_app2.status = FeeApplicationStatus::Pending;
        
        repo.create_fee_application(fee_app1.clone()).await
            .expect("Should create first fee application");
        repo.create_fee_application(fee_app2.clone()).await
            .expect("Should create second fee application");
        
        // Test getting applications by status
        let result = repo.get_fee_applications_by_status(
            "Applied".to_string(), None, None, None
        ).await;
        assert!(result.is_ok(), "Should retrieve applications by status successfully");
        
        let applications = result.unwrap();
        assert!(applications.len() >= 1, "Should find at least one applied application");
        
        for app in &applications {
            assert_eq!(app.status, FeeApplicationStatus::Applied);
        }
        
        // Test with limit
        let result = repo.get_fee_applications_by_status(
            "Applied".to_string(), None, None, Some(1)
        ).await;
        assert!(result.is_ok(), "Should retrieve limited applications successfully");
        
        let limited = result.unwrap();
        assert!(limited.len() <= 1, "Should respect limit parameter");
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_bulk_create_fee_applications() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create multiple fee applications
        let mut apps = Vec::new();
        for i in 0..3 {
            let mut fee_app = create_test_fee_application();
            fee_app.account_id = account_id;
            fee_app.fee_application_id = Uuid::new_v4();
            fee_app.fee_code = HeaplessString::try_from(&format!("FEE{:02}", i)).unwrap();
            fee_app.amount = Decimal::from_str(&format!("{}.00", i + 1)).unwrap();
            apps.push(fee_app);
        }
        
        let result = repo.bulk_create_fee_applications(apps.clone()).await;
        assert!(result.is_ok(), "Should bulk create fee applications successfully");
        
        let created = result.unwrap();
        assert_eq!(created.len(), 3, "Should create all three applications");
        
        // Verify all were created with correct amounts
        for (i, app) in created.iter().enumerate() {
            assert_eq!(app.amount, Decimal::from_str(&format!("{}.00", i + 1)).unwrap());
        }
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_create_fee_waiver() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // First create a fee application
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        let created_app = repo.create_fee_application(fee_app).await
            .expect("Should create fee application");
        
        // Create reason for waiver
        let reason_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO reason_and_purpose (id, reason_category, reason_code, reason_description)
             VALUES ($1, 'FEE_WAIVER', 'GOODWILL', 'Customer goodwill gesture')"
        )
        .bind(reason_id)
        .execute(&repo.pool)
        .await
        .expect("Should create test reason");
        
        // Create fee waiver
        let mut waiver = create_test_fee_waiver(created_app.fee_application_id, account_id);
        waiver.reason_id = reason_id;
        
        let result = repo.create_fee_waiver(waiver.clone()).await;
        assert!(result.is_ok(), "Should create fee waiver successfully");
        
        let created = result.unwrap();
        assert_eq!(created.waiver_id, waiver.waiver_id);
        assert_eq!(created.fee_application_id, created_app.fee_application_id);
        assert_eq!(created.account_id, account_id);
        assert_eq!(created.waived_amount, waiver.waived_amount);
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_fee_waivers_for_account() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create fee application and waiver
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        let created_app = repo.create_fee_application(fee_app).await
            .expect("Should create fee application");
        
        // Create reason for waiver
        let reason_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO reason_and_purpose (id, reason_category, reason_code, reason_description)
             VALUES ($1, 'FEE_WAIVER', 'GOODWILL', 'Customer goodwill gesture')"
        )
        .bind(reason_id)
        .execute(&repo.pool)
        .await
        .expect("Should create test reason");
        
        let mut waiver = create_test_fee_waiver(created_app.fee_application_id, account_id);
        waiver.reason_id = reason_id;
        
        repo.create_fee_waiver(waiver.clone()).await
            .expect("Should create fee waiver");
        
        // Test retrieving waivers for account
        let result = repo.get_fee_waivers_for_account(account_id, None, None).await;
        assert!(result.is_ok(), "Should retrieve fee waivers successfully");
        
        let waivers = result.unwrap();
        assert_eq!(waivers.len(), 1, "Should find one fee waiver");
        
        let found_waiver = &waivers[0];
        assert_eq!(found_waiver.account_id, account_id);
        assert_eq!(found_waiver.waived_amount, waiver.waived_amount);
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_accounts_eligible_for_fees() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Test getting eligible accounts
        let result = repo.get_accounts_eligible_for_fees(
            Some(vec!["SAV01".to_string()]),
            vec!["Maintenance".to_string()],
            NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
            0,
            10,
        ).await;
        
        assert!(result.is_ok(), "Should retrieve eligible accounts successfully");
        
        let accounts = result.unwrap();
        assert!(accounts.len() >= 1, "Should find at least one eligible account");
        assert!(accounts.contains(&account_id), "Should include our test account");
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_fee_revenue_summary() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create fee applications with different statuses and amounts
        let mut fee_app1 = create_test_fee_application();
        fee_app1.account_id = account_id;
        fee_app1.amount = Decimal::from_str("10.00").unwrap();
        fee_app1.status = FeeApplicationStatus::Applied;
        fee_app1.waived = false;
        
        let mut fee_app2 = create_test_fee_application();
        fee_app2.account_id = account_id;
        fee_app2.fee_application_id = Uuid::new_v4();
        fee_app2.amount = Decimal::from_str("5.00").unwrap();
        fee_app2.status = FeeApplicationStatus::Applied;
        fee_app2.waived = true;
        
        repo.create_fee_application(fee_app1.clone()).await
            .expect("Should create first fee application");
        repo.create_fee_application(fee_app2.clone()).await
            .expect("Should create second fee application");
        
        // Test revenue summary
        let from_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let to_date = NaiveDate::from_ymd_opt(2024, 1, 20).unwrap();
        
        let result = repo.get_fee_revenue_summary(from_date, to_date, None, None).await;
        assert!(result.is_ok(), "Should get revenue summary successfully");
        
        let summary = result.unwrap();
        assert!(summary.total_revenue >= Decimal::from_str("10.00").unwrap());
        assert!(summary.waived_amount >= Decimal::from_str("5.00").unwrap());
        assert!(summary.fee_count >= 1);
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_top_fee_accounts() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create multiple fee applications for the account
        for i in 0..3 {
            let mut fee_app = create_test_fee_application();
            fee_app.account_id = account_id;
            fee_app.fee_application_id = Uuid::new_v4();
            fee_app.amount = Decimal::from_str(&format!("{}.00", (i + 1) * 5)).unwrap();
            fee_app.status = FeeApplicationStatus::Applied;
            fee_app.waived = false;
            
            repo.create_fee_application(fee_app).await
                .expect("Should create fee application");
        }
        
        // Test top fee accounts
        let from_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let to_date = NaiveDate::from_ymd_opt(2024, 1, 20).unwrap();
        
        let result = repo.get_top_fee_accounts(from_date, to_date, 5).await;
        assert!(result.is_ok(), "Should get top fee accounts successfully");
        
        let top_accounts = result.unwrap();
        assert!(top_accounts.len() >= 1, "Should find at least one top account");
        
        // Find our test account in the results
        let test_account = top_accounts.iter()
            .find(|acc| acc.account_id == account_id);
        assert!(test_account.is_some(), "Should find our test account in top accounts");
        
        let account = test_account.unwrap();
        assert!(account.total_fees >= Decimal::from_str("15.00").unwrap());
        assert!(account.fee_count >= 3);
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_get_fee_application_statistics() {
        let (pool, _person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create fee applications
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        fee_app.amount = Decimal::from_str("10.00").unwrap();
        fee_app.fee_category = FeeCategory::Transaction;
        
        repo.create_fee_application(fee_app).await
            .expect("Should create fee application");
        
        // Test statistics
        let from_date = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        let to_date = NaiveDate::from_ymd_opt(2024, 1, 20).unwrap();
        
        let result = repo.get_fee_application_statistics(from_date, to_date, "day".to_string()).await;
        assert!(result.is_ok(), "Should get fee statistics successfully");
        
        let stats = result.unwrap();
        assert!(stats.len() >= 1, "Should find at least one statistic entry");
        
        // Find Transaction category stats
        let transaction_stats = stats.iter()
            .find(|stat| stat.fee_category == "Transaction");
        assert!(transaction_stats.is_some(), "Should find Transaction category stats");
        
        let stat = transaction_stats.unwrap();
        assert!(stat.application_count >= 1);
        assert!(stat.total_amount >= Decimal::from_str("10.00").unwrap());
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_reverse_fee_application() {
        let (pool, person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create a fee application
        let mut fee_app = create_test_fee_application();
        fee_app.account_id = account_id;
        fee_app.status = FeeApplicationStatus::Applied;
        
        let created = repo.create_fee_application(fee_app).await
            .expect("Should create fee application");
        
        // Create a reversal reason
        sqlx::query(
            "INSERT INTO reason_and_purpose (id, reason_category, reason_code, reason_description)
             VALUES ($1, 'FEE_REVERSAL', 'REVERSAL', 'Fee reversal')
             ON CONFLICT (reason_code) DO NOTHING"
        )
        .bind(Uuid::new_v4())
        .execute(&repo.pool)
        .await
        .expect("Should create reversal reason");
        
        // Test reversing the fee application
        let result = repo.reverse_fee_application(
            created.fee_application_id,
            "System error".to_string(),
            person_id.to_string(),
            Utc::now(),
        ).await;
        
        assert!(result.is_ok(), "Should reverse fee application successfully");
        
        let reversed = result.unwrap();
        assert_eq!(reversed.fee_application_id, created.fee_application_id);
        assert_eq!(reversed.status, FeeApplicationStatus::Reversed);
        
        cleanup_database(&repo.pool).await;
    }

    #[tokio::test]
    async fn test_bulk_reverse_account_fees() {
        let (pool, person_id, account_id) = setup_test_db().await;
        let repo = FeeRepositoryImpl::new(pool);
        
        // Create multiple fee applications
        let mut fee_ids = Vec::new();
        for i in 0..2 {
            let mut fee_app = create_test_fee_application();
            fee_app.account_id = account_id;
            fee_app.fee_application_id = Uuid::new_v4();
            fee_app.fee_code = HeaplessString::try_from(&format!("FEE{}", i)).unwrap();
            fee_app.status = FeeApplicationStatus::Applied;
            
            let created = repo.create_fee_application(fee_app).await
                .expect("Should create fee application");
            fee_ids.push(created.fee_application_id);
        }
        
        // Create a reversal reason
        sqlx::query(
            "INSERT INTO reason_and_purpose (id, reason_category, reason_code, reason_description)
             VALUES ($1, 'FEE_REVERSAL', 'REVERSAL', 'Fee reversal')
             ON CONFLICT (reason_code) DO NOTHING"
        )
        .bind(Uuid::new_v4())
        .execute(&repo.pool)
        .await
        .expect("Should create reversal reason");
        
        // Test bulk reversal
        let result = repo.bulk_reverse_account_fees(
            account_id,
            fee_ids.clone(),
            "Bulk reversal".to_string(),
            person_id.to_string(),
        ).await;
        
        assert!(result.is_ok(), "Should bulk reverse fee applications successfully");
        
        let reversed = result.unwrap();
        assert_eq!(reversed.len(), fee_ids.len(), "Should reverse all applications");
        
        for app in &reversed {
            assert_eq!(app.status, FeeApplicationStatus::Reversed);
            assert!(fee_ids.contains(&app.fee_application_id));
        }
        
        cleanup_database(&repo.pool).await;
    }
}