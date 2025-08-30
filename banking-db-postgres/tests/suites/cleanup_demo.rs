// Demonstration of the new database cleanup system for test isolation
// This shows the working pattern for using cleanup_database()

mod commons;

use banking_db::models::{AccountWorkflowModel, WorkflowTypeModel, WorkflowStepModel, WorkflowStatusModel};
use chrono::Utc;
use heapless::String as HeaplessString;
use uuid::Uuid;

/// Create a test workflow with provided IDs
#[allow(dead_code)]
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


#[tokio::test]
async fn test_cleanup_isolation_demo() {
    // Initial setup

    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    use crate::commons::{cleanup_database, create_test_account, setup_test_db};
    let (pool, person_id, account_id) = setup_test_db().await;
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    
    // Create initial workflow
    let workflow1 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    repo.create_workflow(&workflow1).await.expect("Failed to create workflow1");
    
    // Verify workflow exists
    let initial_count = repo.count_all_workflows().await
        .expect("Failed to count workflows");
    assert!(initial_count >= 1, "Should have at least 1 workflow");
    
    // Clean database
    cleanup_database(&pool).await;
    
    // Verify database is clean
    let post_cleanup_count = repo.count_all_workflows().await
        .expect("Failed to count workflows after cleanup");
    assert_eq!(post_cleanup_count, 0, "Should have 0 workflows after cleanup");
    
    // Recreate test prerequisites
    let person_id = crate::commons::create_test_person(&pool).await;
    let account_id = create_test_account(&pool, person_id).await;
    
    // Create new workflow in clean environment
    let workflow2 = create_workflow_for_test(Uuid::new_v4(), account_id, person_id);
    repo.create_workflow(&workflow2).await.expect("Failed to create workforce2");
    
    // Verify only the new workflow exists
    let final_count = repo.count_all_workflows().await
        .expect("Failed to count workflows after recreation");
    assert_eq!(final_count, 1, "Should have exactly 1 workflow after recreation");
    
    // Final cleanup
    cleanup_database(&pool).await;
}


#[tokio::test]
async fn test_workflow_crud_basic() {
    // Setup with clean environment

    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    use crate::commons::{cleanup_database, create_test_account, create_test_person, setup_test_db};
    let (pool, _person_id, _account_id) = setup_test_db().await;
    cleanup_database(&pool).await;
    
    // Recreate prerequisites - debug what's happening
    let person_id = create_test_person(&pool).await;
    println!("Created person_id: {}", person_id);
    
    let account_id = create_test_account(&pool, person_id).await;
    println!("Created account_id: {}", account_id);
    
    // Verify both person and account exist
    let person_exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM persons WHERE id = $1"
    )
    .bind(person_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to check person existence");
    
    let account_exists: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM accounts WHERE id = $1"
    )
    .bind(account_id)
    .fetch_one(&pool)
    .await
    .expect("Failed to check account existence");
    
    println!("Person exists: {}, Account exists: {}", person_exists.0, account_exists.0);
    assert_eq!(person_exists.0, 1, "Person should exist before creating workflow");
    assert_eq!(account_exists.0, 1, "Account should exist before creating workflow");
    
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
    
    // UPDATE - Use status update instead of complete workflow
    repo.update_workflow_status(workflow_id, "Completed", "Test completion").await
        .expect("Failed to update workflow status");
    
    let updated_workflow = repo.find_workflow_by_id(workflow_id).await
        .expect("Failed to find updated workflow")
        .expect("Updated workflow not found");
    assert_eq!(updated_workflow.status, WorkflowStatusModel::Completed);
    
    // Final cleanup
    cleanup_database(&pool).await;
}


#[tokio::test]
async fn test_multiple_workflows_with_cleanup() {
    // Clean start

    use banking_db_postgres::WorkflowRepositoryImpl;
    use banking_db::WorkflowRepository;

    use crate::commons::{cleanup_database, create_test_account, create_test_person, setup_test_db};
    let (pool, _person_id, _account_id) = setup_test_db().await;
    cleanup_database(&pool).await;
    
    // Recreate prerequisites after cleanup
    let person_id = create_test_person(&pool).await;
    let account_id = create_test_account(&pool, person_id).await;
    
    let repo = WorkflowRepositoryImpl::new(pool.clone());
    
    // Create multiple workflows - guaranteed isolation
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