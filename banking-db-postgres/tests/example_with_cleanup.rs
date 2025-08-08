// Example test showing how to use the new database cleanup system
// This demonstrates the recommended pattern for test isolation

mod commons;

use commons::{setup_test_db, cleanup_database};
use banking_db::models::{AccountWorkflowModel, WorkflowTypeModel, WorkflowStepModel, WorkflowStatusModel};
use banking_db::repository::WorkflowRepository;
use banking_db_postgres::WorkflowRepositoryImpl;
use chrono::Utc;
use heapless::String as HeaplessString;
use uuid::Uuid;

/// Create a test workflow with provided IDs
fn create_workflow_for_test(workflow_id: Uuid, account_id: Uuid, person_id: Uuid) -> AccountWorkflowModel {
    AccountWorkflowModel {
        id: workflow_id,
        account_id,
        workflow_type: WorkflowTypeModel::AccountOpening,
        current_step: WorkflowStepModel::InitiateRequest,
        status: WorkflowStatusModel::InProgress,
        initiated_by: person_id,
        initiated_at: Utc::now(),
        completed_at: None,
        next_action_required: Some(HeaplessString::try_from("Complete KYC verification").unwrap()),
        timeout_at: Some(Utc::now() + chrono::Duration::hours(24)),
        created_at: Utc::now(),
        last_updated_at: Utc::now(),
    }
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_workflow_crud_with_cleanup() {
    // Setup database 
    let (pool, person_id, account_id) = setup_test_db().await;
    
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    let workflow_id = Uuid::new_v4();
    let workflow = create_workflow_for_test(workflow_id, account_id, person_id);
    
    // CREATE
    let created_workflow = repo.create_workflow(&workflow).await
        .expect("Failed to create workflow");
    assert_eq!(created_workflow.id, workflow.id);
    assert_eq!(created_workflow.workflow_type, workflow.workflow_type);
    
    // READ
    let found_workflow = repo.find_workflow_by_id(workflow_id).await
        .expect("Failed to find workflow")
        .expect("Workflow not found");
    assert_eq!(found_workflow.id, workflow_id);
    
    // UPDATE
    let updated_workflow = repo.complete_workflow(workflow_id, "Successfully completed").await;
    assert!(updated_workflow.is_ok());
    
    let completed_workflow = repo.find_workflow_by_id(workflow_id).await
        .expect("Failed to find updated workflow")
        .expect("Updated workflow not found");
    assert_eq!(completed_workflow.status, WorkflowStatusModel::Completed);
    
    // Manual cleanup for this test
    cleanup_database(&pool).await;
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_multiple_workflows_isolated() {
    // Clean database first, then setup test data
    let (pool, _person_id, _account_id) = setup_test_db().await;
    cleanup_database(&pool).await; // Start with clean state
    
    // Recreate test prerequisites after cleanup
    let person_id = commons::create_test_person(&pool).await;
    let account_id = commons::create_test_account(&pool, person_id).await;
    
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    
    // Create multiple workflows - no need for unique IDs to avoid conflicts
    let workflow1 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    let mut workflow2 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    workflow2.workflow_type = WorkflowTypeModel::AccountClosure;
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    
    // Test finding workflows by account - exact counts are now reliable
    let workflows = repo.find_workflows_by_account(account_id).await
        .expect("Failed to find workflows by account");
    
    assert_eq!(workflows.len(), 2, "Should have exactly 2 workflows");
    
    let workflow_types: Vec<_> = workflows.iter().map(|w| &w.workflow_type).collect();
    assert!(workflow_types.contains(&&WorkflowTypeModel::AccountOpening));
    assert!(workflow_types.contains(&&WorkflowTypeModel::AccountClosure));
    
    // Cleanup after test
    cleanup_database(&pool).await;
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_workflow_pagination_isolated() {
    let (pool, _person_id, _account_id) = setup_test_db().await;
    cleanup_database(&pool).await; // Start with clean state
    
    // Recreate test prerequisites after cleanup
    let person_id = commons::create_test_person(&pool).await;
    let account_id = commons::create_test_account(&pool, person_id).await;
    
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    
    // Create exactly 5 workflows for predictable pagination testing
    for i in 0..5 {
        let workflow = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
        repo.create_workflow(&workflow).await
            .expect(&format!("Failed to create workflow {}", i));
    }
    
    // Test pagination with exact counts (no other test data interference)
    let first_page = repo.list_workflows(0, 3).await
        .expect("Failed to list first page");
    assert_eq!(first_page.len(), 3, "First page should have exactly 3 workflows");
    
    let second_page = repo.list_workflows(3, 3).await
        .expect("Failed to list second page");
    assert_eq!(second_page.len(), 2, "Second page should have exactly 2 workflows");
    
    // Verify no overlap between pages
    let first_ids: Vec<_> = first_page.iter().map(|w| w.id).collect();
    let second_ids: Vec<_> = second_page.iter().map(|w| w.id).collect();
    
    for id in first_ids {
        assert!(!second_ids.contains(&id), "Workflow should not appear in both pages");
    }
    
    // Cleanup after test
    cleanup_database(&pool).await;
}

#[cfg(feature = "postgres_tests")]
#[tokio::test]
async fn test_workflow_count_operations_isolated() {
    let (pool, _person_id, _account_id) = setup_test_db().await;
    cleanup_database(&pool).await; // Start with clean state
    
    // Recreate test prerequisites after cleanup
    let person_id = commons::create_test_person(&pool).await;
    let account_id = commons::create_test_account(&pool, person_id).await;
    
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    
    // Start with empty database (guaranteed by cleanup)
    let initial_count = repo.count_all_workflows().await
        .expect("Failed to count initial workflows");
    assert_eq!(initial_count, 0, "Should start with 0 workflows");
    
    // Create 3 workflows of different types
    let workflow1 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    let mut workflow2 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    workflow2.workflow_type = WorkflowTypeModel::AccountClosure;
    let mut workflow3 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    workflow3.workflow_type = WorkflowTypeModel::ComplianceCheck;
    
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    repo.create_workflow(&workflow2).await.expect("Failed to create workflow2");
    repo.create_workflow(&workflow3).await.expect("Failed to create workflow3");
    
    // Test exact counts
    let total_count = repo.count_all_workflows().await
        .expect("Failed to count total workflows");
    assert_eq!(total_count, 3, "Should have exactly 3 workflows total");
    
    let opening_count = repo.count_workflows_by_type("AccountOpening").await
        .expect("Failed to count account opening workflows");
    assert_eq!(opening_count, 1, "Should have exactly 1 AccountOpening workflow");
    
    let in_progress_count = repo.count_workflows_by_status("InProgress").await
        .expect("Failed to count in-progress workflows");
    assert_eq!(in_progress_count, 3, "Should have exactly 3 InProgress workflows");
    
    // Cleanup after test
    cleanup_database(&pool).await;
}