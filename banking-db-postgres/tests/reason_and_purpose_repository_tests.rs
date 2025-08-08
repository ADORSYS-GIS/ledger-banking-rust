#[cfg(feature = "postgres_tests")]
mod reason_and_purpose_repository_tests {
    use banking_api::domain::{ReasonCategory, ReasonContext, ReasonSeverity};
    use banking_db::models::ReasonAndPurpose as ReasonAndPurposeModel;
    use banking_db::repository::ReasonAndPurposeRepository;
    use banking_db_postgres::repository::ReasonAndPurposeRepositoryImpl;
    use chrono::Utc;
    use heapless::String as HeaplessString;
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/mydb".to_string());
        
        let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");
        
        // Clean up any existing test data
        cleanup_database(&pool).await;
        
        pool
    }

    async fn cleanup_database(pool: &PgPool) {
        // Clean up all test data including seed data for isolated tests
        let _ = sqlx::query("DELETE FROM reason_and_purpose")
            .execute(pool)
            .await;
    }

    fn create_test_reason() -> ReasonAndPurposeModel {
        let unique_id = Uuid::new_v4();
        let code = format!("TEST_{}", &unique_id.to_string()[0..8]);
        
        ReasonAndPurposeModel {
            id: Uuid::new_v4(),
            code: HeaplessString::try_from(code.as_str()).unwrap(),
            category: ReasonCategory::LoanPurpose,
            context: ReasonContext::Loan,
            l1_content: Some(HeaplessString::try_from("Test Loan Purpose").unwrap()),
            l2_content: Some(HeaplessString::try_from("But du Prêt Test").unwrap()),
            l3_content: None,
            l1_language_code: Some([b'e', b'n', b'g']),
            l2_language_code: Some([b'f', b'r', b'a']),
            l3_language_code: None,
            requires_details: true,
            is_active: true,
            severity: Some(ReasonSeverity::Medium),
            display_order: 100,
            compliance_metadata: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by_person_id: HeaplessString::try_from("test_user").unwrap(),
            updated_by_person_id: HeaplessString::try_from("test_user").unwrap(),
        }
    }

    #[tokio::test]
    async fn test_create_reason() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let original_id = test_reason.id;
        let original_code = test_reason.code.clone();
        
        let created = repo.create(test_reason).await.expect("Failed to create reason");
        
        assert_eq!(created.id, original_id);
        assert_eq!(created.code, original_code);
        assert_eq!(created.category, ReasonCategory::LoanPurpose);
        assert_eq!(created.context, ReasonContext::Loan);
        assert!(created.is_active);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let reason_id = test_reason.id;
        
        let created = repo.create(test_reason).await.expect("Failed to create reason");
        
        let found = repo.find_by_id(reason_id).await.expect("Failed to find reason");
        
        assert!(found.is_some());
        let found_reason = found.unwrap();
        assert_eq!(found_reason.id, created.id);
        assert_eq!(found_reason.code, created.code);
        
        // Test non-existent ID
        let non_existent = repo.find_by_id(Uuid::new_v4()).await.expect("Failed to query non-existent reason");
        assert!(non_existent.is_none());
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_by_code() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let reason_code = test_reason.code.clone();
        
        let created = repo.create(test_reason).await.expect("Failed to create reason");
        
        let found = repo.find_by_code(reason_code.as_str()).await.expect("Failed to find reason by code");
        
        assert!(found.is_some());
        let found_reason = found.unwrap();
        assert_eq!(found_reason.id, created.id);
        assert_eq!(found_reason.code, created.code);
        
        // Test non-existent code
        let non_existent = repo.find_by_code("NON_EXISTENT_CODE").await.expect("Failed to query non-existent code");
        assert!(non_existent.is_none());
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_update_reason() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let created = repo.create(test_reason).await.expect("Failed to create reason");
        
        let mut updated_reason = created.clone();
        updated_reason.l1_content = Some(HeaplessString::try_from("Updated Test Reason").unwrap());
        updated_reason.severity = Some(ReasonSeverity::High);
        updated_reason.requires_details = false;
        updated_reason.updated_at = Utc::now();
        
        let updated = repo.update(updated_reason).await.expect("Failed to update reason");
        
        assert_eq!(updated.id, created.id);
        assert_eq!(updated.l1_content.as_ref().unwrap().as_str(), "Updated Test Reason");
        assert_eq!(updated.severity, Some(ReasonSeverity::High));
        assert!(!updated.requires_details);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_delete_reason() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let reason_id = test_reason.id;
        
        let _created = repo.create(test_reason).await.expect("Failed to create reason");
        
        // Verify it exists
        let found = repo.find_by_id(reason_id).await.expect("Failed to find reason");
        assert!(found.is_some());
        
        // Delete it
        repo.delete(reason_id).await.expect("Failed to delete reason");
        
        // Verify it's gone
        let not_found = repo.find_by_id(reason_id).await.expect("Failed to query deleted reason");
        assert!(not_found.is_none());
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_deactivate_and_reactivate() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let reason_id = test_reason.id;
        
        let created = repo.create(test_reason).await.expect("Failed to create reason");
        assert!(created.is_active);
        
        // Deactivate
        repo.deactivate(reason_id, "test_admin").await.expect("Failed to deactivate reason");
        
        let deactivated = repo.find_by_id(reason_id).await.expect("Failed to find reason").unwrap();
        assert!(!deactivated.is_active);
        
        // Reactivate
        repo.reactivate(reason_id, "test_admin").await.expect("Failed to reactivate reason");
        
        let reactivated = repo.find_by_id(reason_id).await.expect("Failed to find reason").unwrap();
        assert!(reactivated.is_active);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_all_active() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create multiple test reasons
        let mut test_reasons = Vec::new();
        for i in 0..3 {
            let mut reason = create_test_reason();
            reason.display_order = i * 10;
            test_reasons.push(reason);
        }
        
        // Create one inactive reason
        let mut inactive_reason = create_test_reason();
        inactive_reason.is_active = false;
        test_reasons.push(inactive_reason);
        
        // Insert all reasons
        for reason in test_reasons {
            repo.create(reason).await.expect("Failed to create reason");
        }
        
        let active_reasons = repo.find_all_active().await.expect("Failed to find active reasons");
        
        // Should only return active reasons
        assert_eq!(active_reasons.len(), 3);
        for reason in &active_reasons {
            assert!(reason.is_active);
        }
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_by_category() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create reasons with different categories
        let mut loan_reason = create_test_reason();
        loan_reason.category = ReasonCategory::LoanPurpose;
        
        let mut account_reason = create_test_reason();
        account_reason.category = ReasonCategory::AccountClosure;
        
        let mut compliance_reason = create_test_reason();
        compliance_reason.category = ReasonCategory::ComplianceFlag;
        
        repo.create(loan_reason).await.expect("Failed to create loan reason");
        repo.create(account_reason).await.expect("Failed to create account reason");
        repo.create(compliance_reason).await.expect("Failed to create compliance reason");
        
        let loan_reasons = repo.find_by_category(ReasonCategory::LoanPurpose).await.expect("Failed to find loan reasons");
        assert_eq!(loan_reasons.len(), 1);
        assert_eq!(loan_reasons[0].category, ReasonCategory::LoanPurpose);
        
        let account_reasons = repo.find_by_category(ReasonCategory::AccountClosure).await.expect("Failed to find account reasons");
        assert_eq!(account_reasons.len(), 1);
        assert_eq!(account_reasons[0].category, ReasonCategory::AccountClosure);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_by_context() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create reasons with different contexts
        let mut loan_reason = create_test_reason();
        loan_reason.context = ReasonContext::Loan;
        
        let mut account_reason = create_test_reason();
        account_reason.context = ReasonContext::Account;
        
        let mut compliance_reason = create_test_reason();
        compliance_reason.context = ReasonContext::Compliance;
        
        repo.create(loan_reason).await.expect("Failed to create loan reason");
        repo.create(account_reason).await.expect("Failed to create account reason");
        repo.create(compliance_reason).await.expect("Failed to create compliance reason");
        
        let loan_reasons = repo.find_by_context(ReasonContext::Loan).await.expect("Failed to find loan reasons");
        assert_eq!(loan_reasons.len(), 1);
        assert_eq!(loan_reasons[0].context, ReasonContext::Loan);
        
        let account_reasons = repo.find_by_context(ReasonContext::Account).await.expect("Failed to find account reasons");
        assert_eq!(account_reasons.len(), 1);
        assert_eq!(account_reasons[0].context, ReasonContext::Account);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_by_category_and_context() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create reasons with specific category and context combinations
        let mut loan_purpose = create_test_reason();
        loan_purpose.category = ReasonCategory::LoanPurpose;
        loan_purpose.context = ReasonContext::Loan;
        
        let mut loan_rejection = create_test_reason();
        loan_rejection.category = ReasonCategory::LoanRejection;
        loan_rejection.context = ReasonContext::Loan;
        
        let mut account_closure = create_test_reason();
        account_closure.category = ReasonCategory::AccountClosure;
        account_closure.context = ReasonContext::Account;
        
        repo.create(loan_purpose).await.expect("Failed to create loan purpose reason");
        repo.create(loan_rejection).await.expect("Failed to create loan rejection reason");
        repo.create(account_closure).await.expect("Failed to create account closure reason");
        
        let loan_purposes = repo.find_by_category_and_context(ReasonCategory::LoanPurpose, ReasonContext::Loan)
            .await.expect("Failed to find loan purpose reasons");
        assert_eq!(loan_purposes.len(), 1);
        assert_eq!(loan_purposes[0].category, ReasonCategory::LoanPurpose);
        assert_eq!(loan_purposes[0].context, ReasonContext::Loan);
        
        let loan_rejections = repo.find_by_category_and_context(ReasonCategory::LoanRejection, ReasonContext::Loan)
            .await.expect("Failed to find loan rejection reasons");
        assert_eq!(loan_rejections.len(), 1);
        assert_eq!(loan_rejections[0].category, ReasonCategory::LoanRejection);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_find_by_severity() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create reasons with different severities
        let mut critical_reason = create_test_reason();
        critical_reason.severity = Some(ReasonSeverity::Critical);
        
        let mut high_reason = create_test_reason();
        high_reason.severity = Some(ReasonSeverity::High);
        
        let mut low_reason = create_test_reason();
        low_reason.severity = Some(ReasonSeverity::Low);
        
        repo.create(critical_reason).await.expect("Failed to create critical reason");
        repo.create(high_reason).await.expect("Failed to create high reason");
        repo.create(low_reason).await.expect("Failed to create low reason");
        
        let critical_reasons = repo.find_by_severity(ReasonSeverity::Critical).await.expect("Failed to find critical reasons");
        assert_eq!(critical_reasons.len(), 1);
        assert_eq!(critical_reasons[0].severity, Some(ReasonSeverity::Critical));
        
        let high_reasons = repo.find_by_severity(ReasonSeverity::High).await.expect("Failed to find high reasons");
        assert_eq!(high_reasons.len(), 1);
        assert_eq!(high_reasons[0].severity, Some(ReasonSeverity::High));
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_search_by_content() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let mut reason1 = create_test_reason();
        reason1.l1_content = Some(HeaplessString::try_from("Home Purchase Loan").unwrap());
        
        let mut reason2 = create_test_reason();
        reason2.l1_content = Some(HeaplessString::try_from("Business Expansion").unwrap());
        
        let mut reason3 = create_test_reason();
        reason3.l2_content = Some(HeaplessString::try_from("Achat de Maison").unwrap());
        
        repo.create(reason1).await.expect("Failed to create reason1");
        repo.create(reason2).await.expect("Failed to create reason2");
        repo.create(reason3).await.expect("Failed to create reason3");
        
        let home_results = repo.search_by_content("Home", None).await.expect("Failed to search for Home");
        assert_eq!(home_results.len(), 1);
        
        let purchase_results = repo.search_by_content("Purchase", None).await.expect("Failed to search for Purchase");
        assert_eq!(purchase_results.len(), 1); // Should match both "Home Purchase" and "Achat de Maison" but we only created one with "Purchase"
        
        let business_results = repo.search_by_content("Business", None).await.expect("Failed to search for Business");
        assert_eq!(business_results.len(), 1);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_code_exists() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let test_reason = create_test_reason();
        let code = test_reason.code.clone();
        let reason_id = test_reason.id;
        
        // Should not exist initially
        let exists_before = repo.code_exists(code.as_str(), None).await.expect("Failed to check code existence");
        assert!(!exists_before);
        
        // Create the reason
        repo.create(test_reason).await.expect("Failed to create reason");
        
        // Should exist now
        let exists_after = repo.code_exists(code.as_str(), None).await.expect("Failed to check code existence");
        assert!(exists_after);
        
        // Should not exist when excluding the same ID
        let exists_excluded = repo.code_exists(code.as_str(), Some(reason_id)).await.expect("Failed to check code existence with exclusion");
        assert!(!exists_excluded);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_is_active() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let mut test_reason = create_test_reason();
        test_reason.is_active = true;
        let reason_id = test_reason.id;
        
        repo.create(test_reason).await.expect("Failed to create reason");
        
        let is_active = repo.is_active(reason_id).await.expect("Failed to check if reason is active");
        assert!(is_active);
        
        // Deactivate and check again
        repo.deactivate(reason_id, "test_admin").await.expect("Failed to deactivate reason");
        
        let is_active_after = repo.is_active(reason_id).await.expect("Failed to check if reason is active after deactivation");
        assert!(!is_active_after);
        
        // Check non-existent reason
        let non_existent_active = repo.is_active(Uuid::new_v4()).await.expect("Failed to check non-existent reason");
        assert!(!non_existent_active);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_is_valid_for_context() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        let mut test_reason = create_test_reason();
        test_reason.context = ReasonContext::Loan;
        test_reason.is_active = true;
        let reason_id = test_reason.id;
        
        repo.create(test_reason).await.expect("Failed to create reason");
        
        let valid_for_loan = repo.is_valid_for_context(reason_id, ReasonContext::Loan).await.expect("Failed to check context validity");
        assert!(valid_for_loan);
        
        let valid_for_account = repo.is_valid_for_context(reason_id, ReasonContext::Account).await.expect("Failed to check context validity");
        assert!(!valid_for_account);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_count_operations() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create reasons with different categories and contexts
        let mut loan_reason = create_test_reason();
        loan_reason.category = ReasonCategory::LoanPurpose;
        loan_reason.context = ReasonContext::Loan;
        
        let mut account_reason = create_test_reason();
        account_reason.category = ReasonCategory::AccountClosure;
        account_reason.context = ReasonContext::Account;
        
        let mut compliance_reason = create_test_reason();
        compliance_reason.category = ReasonCategory::ComplianceFlag;
        compliance_reason.context = ReasonContext::Compliance;
        
        repo.create(loan_reason).await.expect("Failed to create loan reason");
        repo.create(account_reason).await.expect("Failed to create account reason");
        repo.create(compliance_reason).await.expect("Failed to create compliance reason");
        
        let total_count = repo.count_total().await.expect("Failed to get total count");
        assert!(total_count >= 3); // At least our 3 test reasons
        
        let loan_count = repo.count_by_category(ReasonCategory::LoanPurpose).await.expect("Failed to count loan reasons");
        assert_eq!(loan_count, 1);
        
        let account_count = repo.count_by_context(ReasonContext::Account).await.expect("Failed to count account context reasons");
        assert_eq!(account_count, 1);
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_get_categories_and_contexts_in_use() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create reasons with specific categories and contexts
        let mut loan_reason = create_test_reason();
        loan_reason.category = ReasonCategory::LoanPurpose;
        loan_reason.context = ReasonContext::Loan;
        loan_reason.is_active = true;
        
        let mut account_reason = create_test_reason();
        account_reason.category = ReasonCategory::AccountClosure;
        account_reason.context = ReasonContext::Account;
        account_reason.is_active = true;
        
        repo.create(loan_reason).await.expect("Failed to create loan reason");
        repo.create(account_reason).await.expect("Failed to create account reason");
        
        let categories = repo.get_categories_in_use().await.expect("Failed to get categories in use");
        assert!(categories.contains(&ReasonCategory::LoanPurpose));
        assert!(categories.contains(&ReasonCategory::AccountClosure));
        
        let contexts = repo.get_contexts_in_use().await.expect("Failed to get contexts in use");
        assert!(contexts.contains(&ReasonContext::Loan));
        assert!(contexts.contains(&ReasonContext::Account));
        
        cleanup_database(&pool).await;
    }

    #[tokio::test]
    async fn test_validate_data_integrity() {
        let pool = setup_test_db().await;
        let repo = ReasonAndPurposeRepositoryImpl::new(pool.clone());
        
        // Create some test data
        let mut active_reason = create_test_reason();
        active_reason.is_active = true;
        
        let mut inactive_reason = create_test_reason();
        inactive_reason.is_active = false;
        
        repo.create(active_reason).await.expect("Failed to create active reason");
        repo.create(inactive_reason).await.expect("Failed to create inactive reason");
        
        let integrity_report = repo.validate_data_integrity().await.expect("Failed to validate data integrity");
        
        assert!(integrity_report.total_reasons >= 2);
        assert!(integrity_report.active_reasons >= 1);
        assert!(integrity_report.inactive_reasons >= 1);
        assert_eq!(integrity_report.total_reasons, integrity_report.active_reasons + integrity_report.inactive_reasons);
        
        cleanup_database(&pool).await;
    }
}