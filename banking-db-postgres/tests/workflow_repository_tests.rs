use banking_db::models::{AccountWorkflowModel, WorkflowTypeModel, WorkflowStepModel, WorkflowStatusModel};
use std::collections::HashSet;
use chrono::Utc;
use heapless::String as HeaplessString;
use sqlx::PgPool;
use uuid::Uuid;

/// Test helper to create a sample workflow
#[allow(dead_code)]
fn create_test_workflow() -> AccountWorkflowModel {
    // Use fixed UUIDs for consistent testing
    let workflow_id = Uuid::new_v4();
    let account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap(); // Test account
    let initiated_by = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    
    AccountWorkflowModel {
        id: workflow_id,
        account_id,
        workflow_type: WorkflowTypeModel::AccountOpening,
        current_step: WorkflowStepModel::InitiateRequest,
        status: WorkflowStatusModel::InProgress,
        initiated_by,
        initiated_at: Utc::now(),
        completed_at: None,
        next_action_required: Some(HeaplessString::try_from("Complete KYC verification").unwrap()),
        timeout_at: Some(Utc::now() + chrono::Duration::hours(24)),
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
    }
}

/// Test helper to create a workflow in different states with unique data
#[allow(dead_code)]
fn create_test_workflow_with_status(status: WorkflowStatusModel, workflow_type: WorkflowTypeModel) -> AccountWorkflowModel {
    let mut workflow = create_test_workflow();
    workflow.id = Uuid::new_v4();
    // Use the same test account to avoid foreign key issues
    workflow.account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    workflow.status = status;
    workflow.workflow_type = workflow_type;
    
    if status == WorkflowStatusModel::Completed {
        workflow.completed_at = Some(Utc::now());
        workflow.current_step = WorkflowStepModel::Completed;
    }
    
    workflow
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
        VALUES ($1, 'system', 'Test User', 'test-user')
        ON CONFLICT (id) DO NOTHING
        "#
    )
    .bind(test_person_id)
    .execute(&pool)
    .await
    .expect("Failed to create test person");

    // Create a test account for workflow references
    let test_account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    let product_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO accounts (
            id, product_id, account_type, account_status,
            signing_condition, currency, open_date, domicile_agency_branch_id,
            current_balance, available_balance, accrued_interest,
            created_at, last_updated_at, updated_by_person_id
        ) VALUES (
            $1, $2, 'Savings', 'Active',
            'AnyOwner', 'USD', '2024-01-01', $3,
            0.00, 0.00, 0.00,
            NOW(), NOW(), $4
        )
        ON CONFLICT (id) DO NOTHING
        "#
    )
    .bind(test_account_id)
    .bind(product_id)
    .bind(Uuid::new_v4()) // domicile_agency_branch_id
    .bind(test_person_id)
    .execute(&pool)
    .await
    .expect("Failed to create test account");
    
    pool
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_workflow_crud_operations() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    // Test CREATE
    let created_workflow = repo.create_workflow(&workflow).await
        .expect("Failed to create workflow");
    assert_eq!(created_workflow.id, workflow.id);
    assert_eq!(created_workflow.workflow_type, workflow.workflow_type);
    assert_eq!(created_workflow.status, workflow.status);
    
    // Test READ
    let found_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    assert_eq!(found_workflow.id, workflow.id);
    assert_eq!(found_workflow.account_id, workflow.account_id);
    
    // Test UPDATE
    let mut updated_workflow = found_workflow;
    updated_workflow.status = WorkflowStatusModel::Completed;
    updated_workflow.completed_at = Some(Utc::now());
    
    let result = repo.update_workflow(updated_workflow.clone()).await
        .expect("Failed to update workflow");
    assert_eq!(result.status, WorkflowStatusModel::Completed);
    assert!(result.completed_at.is_some());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_workflows_by_account() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create multiple workflows for the same account using the consistent test account
    let account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    let mut workflow1 = create_test_workflow();
    workflow1.account_id = account_id;
    let mut workflow2 = create_test_workflow();
    workflow2.id = Uuid::new_v4();
    workflow2.account_id = account_id;
    workflow2.workflow_type = WorkflowTypeModel::AccountClosure;
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    
    // Test finding workflows by account
    let workflows = repo.find_workflows_by_account(account_id).await
        .expect("Failed to find workflows by account");
    
    // Should have at least our 2 test workflows
    assert!(workflows.len() >= 2, "Should have at least 2 workflows for account, found {}", workflows.len());
    
    // Verify our specific workflows are in the results
    let our_workflow_ids = vec![workflow1.id, workflow2.id];
    let found_workflows_ids: Vec<_> = workflows.iter().map(|w| w.id).collect();
    
    for our_id in our_workflow_ids {
        assert!(found_workflows_ids.contains(&our_id), "Our test workflow {} not found in account results", our_id);
    }
    
    // Verify we have both workflow types in the results
    let workflow_types: Vec<_> = workflows.iter().map(|w| &w.workflow_type).collect();
    assert!(workflow_types.contains(&&WorkflowTypeModel::AccountOpening));
    assert!(workflow_types.contains(&&WorkflowTypeModel::AccountClosure));
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_active_workflow() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    let account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    let mut workflow = create_test_workflow();
    workflow.id = Uuid::new_v4();
    workflow.account_id = account_id;
    workflow.status = WorkflowStatusModel::InProgress;
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test finding active workflow
    let active_workflow = repo.find_active_workflow(account_id, "AccountOpening").await
        .expect("Failed to find active workflow")
        .expect("Active workflow not found");
    
    assert_eq!(active_workflow.account_id, account_id);
    // Active workflow can be either InProgress or PendingAction
    assert!(active_workflow.status == WorkflowStatusModel::InProgress || active_workflow.status == WorkflowStatusModel::PendingAction);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_workflows_by_type() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflows of different types with unique identifiers
    let mut workflow1 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountOpening);
    workflow1.id = Uuid::new_v4();
    
    let mut workflow2 = create_test_workflow_with_status(WorkflowStatusModel::Completed, WorkflowTypeModel::AccountOpening);
    workflow2.id = Uuid::new_v4();
    
    let mut workflow3 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountClosure);
    workflow3.id = Uuid::new_v4();
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    repo.create_workflow(&workflow3).await.expect("Failed to create workflow3");
    
    // Test finding workflows by type
    let opening_workflows = repo.find_workflows_by_type("AccountOpening").await
        .expect("Failed to find workflows by type");
    
    // Count should include at least our 2 test workflows
    assert!(opening_workflows.len() >= 2, "Should have at least 2 AccountOpening workflows, found {}", opening_workflows.len());
    
    // Verify our specific workflows are in the results
    let our_workflow_ids = vec![workflow1.id, workflow2.id];
    let found_workflow_ids: Vec<_> = opening_workflows.iter().map(|w| w.id).collect();
    
    for our_id in our_workflow_ids {
        assert!(found_workflow_ids.contains(&our_id), "Our test workflow {} not found in results", our_id);
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_workflows_by_status() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflows with different statuses and unique identifiers
    let mut workflow1 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountOpening);
    workflow1.id = Uuid::new_v4();
    
    let mut workflow2 = create_test_workflow_with_status(WorkflowStatusModel::PendingAction, WorkflowTypeModel::AccountOpening);
    workflow2.id = Uuid::new_v4();
    
    let mut workflow3 = create_test_workflow_with_status(WorkflowStatusModel::Completed, WorkflowTypeModel::AccountOpening);
    workflow3.id = Uuid::new_v4();
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    repo.create_workflow(&workflow3).await.expect("Failed to create workflow3");
    
    // Test finding workflows by status
    let pending_workflows = repo.find_workflows_by_status("PendingAction").await
        .expect("Failed to find workflows by status");
    
    // Should have at least our one test workflow
    assert!(pending_workflows.len() >= 1, "Should have at least 1 PendingAction workflow, found {}", pending_workflows.len());
    
    // Verify our specific workflow is in the results
    let found_our_workflow = pending_workflows.iter().any(|w| w.id == workflow2.id);
    assert!(found_our_workflow, "Our test workflow not found in PendingAction results");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_workflow_status_management() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test updating workflow status
    repo.update_workflow_status(workflow.id, "PendingAction", "Waiting for documents").await
        .expect("Failed to update workflow status");
    
    let updated_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    
    assert_eq!(updated_workflow.status, WorkflowStatusModel::PendingAction);
    assert_eq!(updated_workflow.next_action_required.unwrap().as_str(), "Waiting for documents");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_workflow_step_management() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test updating workflow step
    repo.update_workflow_step(workflow.id, "ComplianceCheck").await
        .expect("Failed to update workflow step");
    
    let updated_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    
    assert_eq!(updated_workflow.current_step, WorkflowStepModel::ComplianceCheck);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_complete_workflow() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test completing workflow
    repo.complete_workflow(workflow.id, "Account opened successfully").await
        .expect("Failed to complete workflow");
    
    let completed_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    
    assert_eq!(completed_workflow.status, WorkflowStatusModel::Completed);
    assert!(completed_workflow.completed_at.is_some());
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_fail_workflow() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test failing workflow
    repo.fail_workflow(workflow.id, "KYC verification failed").await
        .expect("Failed to fail workflow");
    
    let failed_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    
    assert_eq!(failed_workflow.status, WorkflowStatusModel::Failed);
    assert_eq!(failed_workflow.next_action_required.unwrap().as_str(), "KYC verification failed");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_cancel_workflow() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test cancelling workflow
    repo.cancel_workflow(workflow.id, "Customer request").await
        .expect("Failed to cancel workflow");
    
    let cancelled_workflow = repo.find_workflow_by_id(workflow.id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    
    assert_eq!(cancelled_workflow.status, WorkflowStatusModel::Cancelled);
    assert_eq!(cancelled_workflow.next_action_required.unwrap().as_str(), "Customer request");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_pending_workflows() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflows with different statuses and unique identifiers
    let mut workflow1 = create_test_workflow_with_status(WorkflowStatusModel::PendingAction, WorkflowTypeModel::AccountOpening);
    workflow1.id = Uuid::new_v4();
    
    let mut workflow2 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountOpening);
    workflow2.id = Uuid::new_v4();
    
    let mut workflow3 = create_test_workflow_with_status(WorkflowStatusModel::PendingAction, WorkflowTypeModel::AccountClosure);
    workflow3.id = Uuid::new_v4();
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    repo.create_workflow(&workflow3).await.expect("Failed to create workflow3");
    
    // Test finding pending workflows
    let pending_workflows = repo.find_pending_workflows().await
        .expect("Failed to find pending workflows");
    
    // Should have at least our 2 test workflows
    assert!(pending_workflows.len() >= 2, "Should have at least 2 pending workflows, found {}", pending_workflows.len());
    
    // Verify our specific workflows are in the results
    let our_pending_ids = vec![workflow1.id, workflow3.id];
    let found_workflow_ids: Vec<_> = pending_workflows.iter().map(|w| w.id).collect();
    
    for our_id in our_pending_ids {
        assert!(found_workflow_ids.contains(&our_id), "Our test workflow {} not found in pending results", our_id);
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_in_progress_workflows() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflows with different statuses and unique identifiers
    let mut workflow1 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountOpening);
    workflow1.id = Uuid::new_v4();
    
    let mut workflow2 = create_test_workflow_with_status(WorkflowStatusModel::PendingAction, WorkflowTypeModel::AccountOpening);
    workflow2.id = Uuid::new_v4();
    
    let mut workflow3 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountClosure);
    workflow3.id = Uuid::new_v4();
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    repo.create_workflow(&workflow3).await.expect("Failed to create workflow3");
    
    // Test finding in-progress workflows
    let in_progress_workflows = repo.find_in_progress_workflows().await
        .expect("Failed to find in-progress workflows");
    
    // Should have at least our 2 test workflows
    assert!(in_progress_workflows.len() >= 2, "Should have at least 2 in-progress workflows, found {}", in_progress_workflows.len());
    
    // Verify our specific workflows are in the results
    let our_in_progress_ids = vec![workflow1.id, workflow3.id];
    let found_workflow_ids: Vec<_> = in_progress_workflows.iter().map(|w| w.id).collect();
    
    for our_id in our_in_progress_ids {
        assert!(found_workflow_ids.contains(&our_id), "Our test workflow {} not found in in-progress results", our_id);
    }
    
    // Verify all returned workflows have the correct status
    for workflow in &in_progress_workflows {
        assert_eq!(workflow.status, WorkflowStatusModel::InProgress);
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_expired_workflows() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflow with timeout in the past with unique identifier
    let mut workflow = create_test_workflow();
    workflow.id = Uuid::new_v4();
    workflow.timeout_at = Some(Utc::now() - chrono::Duration::hours(1)); // Expired 1 hour ago
    workflow.status = WorkflowStatusModel::InProgress;
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test finding expired workflows
    let expired_workflows = repo.find_expired_workflows(Utc::now()).await
        .expect("Failed to find expired workflows");
    
    // Should have at least our 1 test workflow
    assert!(expired_workflows.len() >= 1, "Should have at least 1 expired workflow, found {}", expired_workflows.len());
    
    // Verify our specific workflow is in the results
    let found_our_workflow = expired_workflows.iter().any(|w| w.id == workflow.id);
    assert!(found_our_workflow, "Our test workflow not found in expired workflows results");
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_account_opening_workflows() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflows of different types with unique identifiers
    let mut workflow1 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountOpening);
    workflow1.id = Uuid::new_v4();
    
    let mut workflow2 = create_test_workflow_with_status(WorkflowStatusModel::Completed, WorkflowTypeModel::AccountOpening);
    workflow2.id = Uuid::new_v4();
    
    let mut workflow3 = create_test_workflow_with_status(WorkflowStatusModel::InProgress, WorkflowTypeModel::AccountClosure);
    workflow3.id = Uuid::new_v4();
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    repo.create_workflow(&workflow3).await.expect("Failed to create workflow3");
    
    // Test finding all account opening workflows
    let all_opening_workflows = repo.find_account_opening_workflows(None).await
        .expect("Failed to find account opening workflows");
    
    // Should have at least our 2 test workflows
    assert!(all_opening_workflows.len() >= 2, "Should have at least 2 account opening workflows, found {}", all_opening_workflows.len());
    
    // Verify our specific workflows are in the results
    let our_opening_ids = vec![workflow1.id, workflow2.id];
    let found_opening_ids: Vec<_> = all_opening_workflows.iter().map(|w| w.id).collect();
    
    for our_id in our_opening_ids {
        assert!(found_opening_ids.contains(&our_id), "Our test workflow {} not found in account opening results", our_id);
    }
    
    // Test finding account opening workflows by status
    let in_progress_opening = repo.find_account_opening_workflows(Some("InProgress")).await
        .expect("Failed to find in-progress account opening workflows");
    
    // Should have at least our 1 test workflow
    assert!(in_progress_opening.len() >= 1, "Should have at least 1 in-progress account opening workflow, found {}", in_progress_opening.len());
    
    // Verify our specific workflow is in the results
    let found_our_workflow = in_progress_opening.iter().any(|w| w.id == workflow1.id);
    assert!(found_our_workflow, "Our test workflow not found in in-progress account opening results");
    
    // Verify all returned workflows have the correct status
    for workflow in &in_progress_opening {
        assert_eq!(workflow.status, WorkflowStatusModel::InProgress);
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_pending_kyc_workflows() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create a compliance verification workflow in KYC step with unique identifier
    let mut workflow = create_test_workflow();
    workflow.id = Uuid::new_v4();
    workflow.workflow_type = WorkflowTypeModel::ComplianceCheck;
    workflow.current_step = WorkflowStepModel::ComplianceCheck;
    workflow.status = WorkflowStatusModel::PendingAction;
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test finding pending KYC workflows
    let kyc_workflows = repo.find_pending_kyc_workflows().await
        .expect("Failed to find pending KYC workflows");
    
    // Should have at least our 1 test workflow
    assert!(kyc_workflows.len() >= 1, "Should have at least 1 pending KYC workflow, found {}", kyc_workflows.len());
    
    // Verify our specific workflow is in the results
    let found_our_workflow = kyc_workflows.iter().any(|w| w.id == workflow.id);
    assert!(found_our_workflow, "Our test workflow not found in pending KYC results");
    
    // Find our specific workflow and verify its properties
    let our_workflow = kyc_workflows.iter().find(|w| w.id == workflow.id).unwrap();
    assert_eq!(our_workflow.workflow_type, WorkflowTypeModel::ComplianceCheck);
    assert_eq!(our_workflow.current_step, WorkflowStepModel::ComplianceCheck);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_find_pending_document_verification() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create a workflow in document verification step with unique identifier
    let mut workflow = create_test_workflow();
    workflow.id = Uuid::new_v4();
    workflow.current_step = WorkflowStepModel::DocumentVerification;
    workflow.status = WorkflowStatusModel::PendingAction;
    
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test finding pending document verification workflows
    let doc_workflows = repo.find_pending_document_verification().await
        .expect("Failed to find pending document verification workflows");
    
    // Should have at least our 1 test workflow
    assert!(doc_workflows.len() >= 1, "Should have at least 1 pending document verification workflow, found {}", doc_workflows.len());
    
    // Verify our specific workflow is in the results
    let found_our_workflow = doc_workflows.iter().any(|w| w.id == workflow.id);
    assert!(found_our_workflow, "Our test workflow not found in pending document verification results");
    
    // Find our specific workflow and verify its properties
    let our_workflow = doc_workflows.iter().find(|w| w.id == workflow.id).unwrap();
    assert_eq!(our_workflow.current_step, WorkflowStepModel::DocumentVerification);
    assert_eq!(our_workflow.status, WorkflowStatusModel::PendingAction);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_utility_operations() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    let workflow = create_test_workflow();
    
    // Test workflow existence before creation
    let exists_before = repo.workflow_exists(workflow.id).await
        .expect("Failed to check workflow existence");
    assert!(!exists_before);
    
    // Create workflow
    repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    
    // Test workflow existence after creation
    let exists_after = repo.workflow_exists(workflow.id).await
        .expect("Failed to check workflow existence");
    assert!(exists_after);
    
    // Test count operations
    let type_count = repo.count_workflows_by_type("AccountOpening").await
        .expect("Failed to count workflows by type");
    assert!(type_count >= 1);
    
    let status_count = repo.count_workflows_by_status("InProgress").await
        .expect("Failed to count workflows by status");
    assert!(status_count >= 1);
    
    let total_count = repo.count_all_workflows().await
        .expect("Failed to count all workflows");
    assert!(total_count >= 1);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_list_workflows_pagination() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create multiple workflows using consistent test account with unique identifiers
    let test_account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    let mut created_workflow_ids = Vec::new();
    
    for i in 0..6 { // Create 6 workflows to test pagination properly
        let mut workflow = create_test_workflow();
        workflow.id = Uuid::new_v4();
        workflow.account_id = test_account_id;
        // Add a small delay and unique next_action to ensure different created_at times
        workflow.next_action_required = Some(HeaplessString::try_from(format!("Test action {}", i).as_str()).unwrap());
        
        repo.create_workflow(&workflow).await.expect("Failed to create workflow");
        created_workflow_ids.push(workflow.id);
        
        // Small delay to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }
    
    // Test pagination - get first 2 workflows
    let first_page = repo.list_workflows(0, 2).await
        .expect("Failed to list workflows");
    assert!(first_page.len() <= 2);
    
    let second_page = repo.list_workflows(2, 2).await
        .expect("Failed to list workflows");
    assert!(second_page.len() <= 2);
    
    let third_page = repo.list_workflows(4, 2).await
        .expect("Failed to list workflows");
    assert!(third_page.len() <= 2);
    
    // Collect all IDs from all pages
    let mut all_page_ids = Vec::new();
    all_page_ids.extend(first_page.iter().map(|w| w.id));
    all_page_ids.extend(second_page.iter().map(|w| w.id));
    all_page_ids.extend(third_page.iter().map(|w| w.id));
    
    // Check that there are no duplicates across pages
    let unique_ids: HashSet<_> = all_page_ids.iter().collect();
    assert_eq!(unique_ids.len(), all_page_ids.len(), "Found duplicate workflows across pages");
    
    // Verify that at least some of our created workflows appear in the results
    let found_our_workflows = created_workflow_ids.iter().filter(|id| all_page_ids.contains(id)).count();
    assert!(found_our_workflows >= 3, "Should find at least 3 of our created workflows in paginated results, found {}", found_our_workflows);
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_bulk_operations() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create multiple workflows using the consistent test account
    let mut workflow_ids = Vec::new();
    let test_account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    for _i in 0..3 {
        let mut workflow = create_test_workflow();
        workflow.id = Uuid::new_v4();
        workflow.account_id = test_account_id;
        workflow.status = WorkflowStatusModel::InProgress;
        
        repo.create_workflow(&workflow).await.expect("Failed to create workflow");
        workflow_ids.push(workflow.id);
    }
    
    // Test bulk status update
    let updated_count = repo.bulk_update_workflow_status(workflow_ids.clone(), "Completed").await
        .expect("Failed to bulk update workflow status");
    assert_eq!(updated_count, 3);
    
    // Verify all workflows were updated
    for workflow_id in workflow_ids {
        let workflow = repo.find_workflow_by_id(workflow_id).await
            .expect("Failed to find workflow")
            .expect("Workflow not found");
        assert_eq!(workflow.status, WorkflowStatusModel::Completed);
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_bulk_timeout_expired_workflows() {
    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    let pool = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool);
    
    // Create workflows with timeouts in the past using consistent test account
    let past_time = Utc::now() - chrono::Duration::hours(2);
    let test_account_id = Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap();
    for _i in 0..2 {
        let mut workflow = create_test_workflow();
        workflow.id = Uuid::new_v4();
        workflow.account_id = test_account_id;
        workflow.timeout_at = Some(past_time);
        workflow.status = WorkflowStatusModel::InProgress;
        
        repo.create_workflow(&workflow).await.expect("Failed to create workflow");
    }
    
    // Test bulk timeout operation
    let timeout_count = repo.bulk_timeout_expired_workflows(Utc::now()).await
        .expect("Failed to bulk timeout expired workflows");
    assert!(timeout_count >= 2, "Should have timed out at least 2 workflows, timed out {}", timeout_count);
}