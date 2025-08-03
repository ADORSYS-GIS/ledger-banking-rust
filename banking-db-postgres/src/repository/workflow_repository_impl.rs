use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_db::models::{AccountWorkflowModel, WorkflowStepRecordModel, WorkflowTypeModel, WorkflowStepModel, WorkflowStatusModel};
use banking_db::repository::{WorkflowRepository, WorkflowMetricsReport, WorkflowPerformanceReport, WorkflowBottleneckReport};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use heapless::String as HeaplessString;
use std::str::FromStr;

pub struct WorkflowRepositoryImpl {
    pool: PgPool,
}

impl WorkflowRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkflowRepository for WorkflowRepositoryImpl {
    /// Account Workflow Operations
    async fn create_workflow(&self, workflow: &AccountWorkflowModel) -> BankingResult<AccountWorkflowModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_workflows (
                workflow_id, account_id, workflow_type, current_step, status, 
                initiated_by, initiated_at, completed_at, next_action_required, timeout_at
            ) VALUES ($1, $2, $3::workflow_type, $4, $5, $6, $7, $8, $9, $10)
            RETURNING workflow_id, account_id, workflow_type::text, current_step, status, 
                     initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                     created_at, last_updated_at
            "#
        )
        .bind(workflow.workflow_id)
        .bind(workflow.account_id)
        .bind(workflow.workflow_type.to_string())
        .bind(workflow.current_step.to_string())
        .bind(workflow.status.to_string())
        .bind(workflow.initiated_by)
        .bind(workflow.initiated_at)
        .bind(workflow.completed_at)
        .bind(workflow.next_action_required.as_ref().map(|s| s.as_str()))
        .bind(workflow.timeout_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to create workflow: {e}")))?;

        Ok(AccountWorkflowModel {
            workflow_id: result.get("workflow_id"),
            account_id: result.get("account_id"),
            workflow_type: WorkflowTypeModel::from_str(&result.get::<String, _>("workflow_type"))
                .map_err(|e| BankingError::ValidationError {
                    field: "workflow_type".to_string(),
                    message: e,
                })?,
            current_step: WorkflowStepModel::from_str(&result.get::<String, _>("current_step"))
                .map_err(|e| BankingError::ValidationError {
                    field: "current_step".to_string(),
                    message: e,
                })?,
            status: WorkflowStatusModel::from_str(&result.get::<String, _>("status"))
                .map_err(|e| BankingError::ValidationError {
                    field: "status".to_string(),
                    message: e,
                })?,
            initiated_by: result.get("initiated_by"),
            initiated_at: result.get("initiated_at"),
            completed_at: result.get("completed_at"),
            next_action_required: result.get::<Option<String>, _>("next_action_required")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "next_action_required".to_string(),
                    message: "Next action required too long".to_string(),
                })?,
            timeout_at: result.get("timeout_at"),
            created_at: result.get("created_at"),
            last_updated_at: result.get("last_updated_at"),
        })
    }

    async fn update_workflow(&self, workflow: AccountWorkflowModel) -> BankingResult<AccountWorkflowModel> {
        let result = sqlx::query(
            r#"
            UPDATE account_workflows 
            SET workflow_type = $2::workflow_type, current_step = $3, status = $4, 
                completed_at = $5, next_action_required = $6, timeout_at = $7,
                last_updated_at = NOW()
            WHERE workflow_id = $1
            RETURNING workflow_id, account_id, workflow_type::text, current_step, status, 
                     initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                     created_at, last_updated_at
            "#
        )
        .bind(workflow.workflow_id)
        .bind(workflow.workflow_type.to_string())
        .bind(workflow.current_step.to_string())
        .bind(workflow.status.to_string())
        .bind(workflow.completed_at)
        .bind(workflow.next_action_required.as_ref().map(|s| s.as_str()))
        .bind(workflow.timeout_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to update workflow: {e}"),
        ))?;

        Ok(AccountWorkflowModel {
            workflow_id: result.get("workflow_id"),
            account_id: result.get("account_id"),
            workflow_type: WorkflowTypeModel::from_str(&result.get::<String, _>("workflow_type"))
                .map_err(|e| BankingError::ValidationError {
                    field: "workflow_type".to_string(),
                    message: e,
                })?,
            current_step: WorkflowStepModel::from_str(&result.get::<String, _>("current_step"))
                .map_err(|e| BankingError::ValidationError {
                    field: "current_step".to_string(),
                    message: e,
                })?,
            status: WorkflowStatusModel::from_str(&result.get::<String, _>("status"))
                .map_err(|e| BankingError::ValidationError {
                    field: "status".to_string(),
                    message: e,
                })?,
            initiated_by: result.get("initiated_by"),
            initiated_at: result.get("initiated_at"),
            completed_at: result.get("completed_at"),
            next_action_required: result.get::<Option<String>, _>("next_action_required")
                .map(|s| HeaplessString::try_from(s.as_str()))
                .transpose()
                .map_err(|_| BankingError::ValidationError {
                    field: "next_action_required".to_string(),
                    message: "Next action required too long".to_string(),
                })?,
            timeout_at: result.get("timeout_at"),
            created_at: result.get("created_at"),
            last_updated_at: result.get("last_updated_at"),
        })
    }

    async fn find_workflow_by_id(&self, workflow_id: Uuid) -> BankingResult<Option<AccountWorkflowModel>> {
        let result = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE workflow_id = $1
            "#
        )
        .bind(workflow_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find workflow by ID: {e}"),
        ))?;

        match result {
            Some(row) => Ok(Some(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn find_workflows_by_account(&self, account_id: Uuid) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE account_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find workflows by account: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn find_active_workflow(&self, account_id: Uuid, workflow_type: &str) -> BankingResult<Option<AccountWorkflowModel>> {
        let result = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE account_id = $1 AND workflow_type = $2::workflow_type 
                  AND status IN ('InProgress', 'PendingAction')
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .bind(account_id)
        .bind(workflow_type)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find active workflow: {e}"),
        ))?;

        match result {
            Some(row) => Ok(Some(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn find_workflows_by_type(&self, workflow_type: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE workflow_type = $1::workflow_type
            ORDER BY created_at DESC
            "#
        )
        .bind(workflow_type)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find workflows by type: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn find_workflows_by_status(&self, status: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE status = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find workflows by status: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn find_workflows_by_initiator(&self, initiated_by: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        let initiated_by_uuid = Uuid::parse_str(initiated_by)
            .map_err(|_| BankingError::ValidationError {
                field: "initiated_by".to_string(),
                message: "Invalid UUID format".to_string(),
            })?;

        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE initiated_by = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(initiated_by_uuid)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find workflows by initiator: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    /// Workflow Status Management
    async fn update_workflow_status(&self, workflow_id: Uuid, status: &str, notes: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = $2, next_action_required = $3, last_updated_at = NOW()
            WHERE workflow_id = $1
            "#
        )
        .bind(workflow_id)
        .bind(status)
        .bind(if notes.is_empty() { None } else { Some(notes) })
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to update workflow status: {e}"),
        ))?;

        Ok(())
    }

    async fn update_workflow_step(&self, workflow_id: Uuid, current_step: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_workflows 
            SET current_step = $2, last_updated_at = NOW()
            WHERE workflow_id = $1
            "#
        )
        .bind(workflow_id)
        .bind(current_step)
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to update workflow step: {e}"),
        ))?;

        Ok(())
    }

    async fn advance_workflow_step(&self, workflow_id: Uuid, step: &str, notes: &str) -> BankingResult<()> {
        // First update the workflow current step
        self.update_workflow_step(workflow_id, step).await?;

        // Add a step record
        let step_record = WorkflowStepRecordModel {
            step: WorkflowStepModel::from_str(step)
                .map_err(|e| BankingError::ValidationError {
                    field: "step".to_string(),
                    message: e,
                })?,
            completed_at: Utc::now(),
            completed_by: Uuid::nil(), // Would normally be provided by the caller
            notes: if notes.is_empty() { None } else { 
                Some(HeaplessString::try_from(notes).map_err(|_| BankingError::ValidationError {
                    field: "notes".to_string(),
                    message: "Notes too long".to_string(),
                })?)
            },
            supporting_documents: Vec::new(),
        };

        self.add_step_record(step_record).await?;

        Ok(())
    }

    async fn complete_workflow(&self, workflow_id: Uuid, completion_notes: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = 'Completed', completed_at = NOW(), next_action_required = $2, last_updated_at = NOW()
            WHERE workflow_id = $1
            "#
        )
        .bind(workflow_id)
        .bind(if completion_notes.is_empty() { None } else { Some(completion_notes) })
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to complete workflow: {e}"),
        ))?;

        Ok(())
    }

    async fn fail_workflow(&self, workflow_id: Uuid, failure_reason: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = 'Failed', next_action_required = $2, last_updated_at = NOW()
            WHERE workflow_id = $1
            "#
        )
        .bind(workflow_id)
        .bind(if failure_reason.is_empty() { None } else { Some(failure_reason) })
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to fail workflow: {e}"),
        ))?;

        Ok(())
    }

    async fn cancel_workflow(&self, workflow_id: Uuid, reason: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = 'Cancelled', next_action_required = $2, last_updated_at = NOW()
            WHERE workflow_id = $1
            "#
        )
        .bind(workflow_id)
        .bind(if reason.is_empty() { None } else { Some(reason) })
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to cancel workflow: {e}"),
        ))?;

        Ok(())
    }

    /// Workflow Step Record Operations
    async fn add_step_record(&self, step_record: WorkflowStepRecordModel) -> BankingResult<WorkflowStepRecordModel> {
        // Since workflow_step_records requires a workflow_id, we need to modify the signature
        // For now, we'll use a placeholder implementation
        Ok(step_record)
    }

    async fn find_step_records_by_workflow(&self, workflow_id: Uuid) -> BankingResult<Vec<WorkflowStepRecordModel>> {
        let rows = sqlx::query(
            r#"
            SELECT step, completed_at, completed_by, notes, supporting_documents
            FROM workflow_step_records 
            WHERE workflow_id = $1
            ORDER BY completed_at ASC
            "#
        )
        .bind(workflow_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find step records by workflow: {e}"),
        ))?;

        let mut step_records = Vec::new();
        for row in rows {
            step_records.push(WorkflowStepRecordModel {
                step: WorkflowStepModel::from_str(&row.get::<String, _>("step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "step".to_string(),
                        message: e,
                    })?,
                completed_at: row.get("completed_at"),
                completed_by: row.get("completed_by"),
                notes: row.get::<Option<String>, _>("notes")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "notes".to_string(),
                        message: "Notes too long".to_string(),
                    })?,
                supporting_documents: Vec::new(), // Would need JSON parsing for full implementation
            });
        }
        Ok(step_records)
    }

    async fn find_latest_step_record(&self, workflow_id: Uuid) -> BankingResult<Option<WorkflowStepRecordModel>> {
        let result = sqlx::query(
            r#"
            SELECT step, completed_at, completed_by, notes, supporting_documents
            FROM workflow_step_records 
            WHERE workflow_id = $1
            ORDER BY completed_at DESC
            LIMIT 1
            "#
        )
        .bind(workflow_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find latest step record: {e}"),
        ))?;

        match result {
            Some(row) => Ok(Some(WorkflowStepRecordModel {
                step: WorkflowStepModel::from_str(&row.get::<String, _>("step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "step".to_string(),
                        message: e,
                    })?,
                completed_at: row.get("completed_at"),
                completed_by: row.get("completed_by"),
                notes: row.get::<Option<String>, _>("notes")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "notes".to_string(),
                        message: "Notes too long".to_string(),
                    })?,
                supporting_documents: Vec::new(),
            })),
            None => Ok(None),
        }
    }

    /// Workflow Monitoring and Management
    async fn find_pending_workflows(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        self.find_workflows_by_status("PendingAction").await
    }

    async fn find_in_progress_workflows(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        self.find_workflows_by_status("InProgress").await
    }

    async fn find_expired_workflows(&self, reference_time: DateTime<Utc>) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE timeout_at IS NOT NULL AND timeout_at < $1 AND status IN ('InProgress', 'PendingAction')
            ORDER BY timeout_at ASC
            "#
        )
        .bind(reference_time)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find expired workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    // Simplified implementation for remaining methods - would be expanded in production
    async fn find_workflows_requiring_action(&self, _action_type: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        self.find_pending_workflows().await
    }

    async fn find_account_opening_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>> {
        match status {
            Some(s) => {
                let workflows = self.find_workflows_by_type("AccountOpening").await?;
                Ok(workflows.into_iter().filter(|w| w.status.to_string() == s).collect())
            },
            None => self.find_workflows_by_type("AccountOpening").await,
        }
    }

    async fn find_pending_kyc_workflows(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE workflow_type = 'ComplianceCheck' AND current_step = 'ComplianceCheck' AND status = 'PendingAction'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find pending KYC workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn find_pending_document_verification(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE current_step = 'DocumentVerification' AND status = 'PendingAction'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find pending document verification workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    // Placeholder implementations for remaining methods
    async fn find_account_closure_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>> {
        match status {
            Some(s) => {
                let workflows = self.find_workflows_by_type("AccountClosure").await?;
                Ok(workflows.into_iter().filter(|w| w.status.to_string() == s).collect())
            },
            None => self.find_workflows_by_type("AccountClosure").await,
        }
    }

    async fn find_pending_final_settlement(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE workflow_type = 'AccountClosure' AND current_step = 'FinalSettlement' AND status = 'PendingAction'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find pending final settlement workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn find_pending_disbursement(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        // This would typically join with final_settlements table
        Ok(Vec::new())
    }

    async fn find_reactivation_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>> {
        match status {
            Some(s) => {
                let workflows = self.find_workflows_by_type("KycUpdate").await?;
                Ok(workflows.into_iter().filter(|w| w.status.to_string() == s).collect())
            },
            None => self.find_workflows_by_type("KycUpdate").await,
        }
    }

    async fn find_pending_mini_kyc(&self) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE workflow_type = 'KycUpdate' AND current_step = 'ComplianceCheck' AND status = 'PendingAction'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find pending mini KYC workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn find_compliance_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>> {
        match status {
            Some(s) => {
                let workflows = self.find_workflows_by_type("ComplianceCheck").await?;
                Ok(workflows.into_iter().filter(|w| w.status.to_string() == s).collect())
            },
            None => self.find_workflows_by_type("ComplianceCheck").await,
        }
    }

    async fn find_workflows_by_customer_risk(&self, _risk_rating: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        // This would require joining with customers table
        Ok(Vec::new())
    }

    async fn find_approval_workflows(&self, status: Option<&str>) -> BankingResult<Vec<AccountWorkflowModel>> {
        match status {
            Some(s) => {
                let workflows = self.find_workflows_by_type("TransactionApproval").await?;
                Ok(workflows.into_iter().filter(|w| w.status.to_string() == s).collect())
            },
            None => self.find_workflows_by_type("TransactionApproval").await,
        }
    }

    async fn find_workflows_by_approver(&self, approver_id: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        self.find_workflows_by_initiator(approver_id).await
    }

    async fn find_workflows_awaiting_approval(&self, _approver_id: &str) -> BankingResult<Vec<AccountWorkflowModel>> {
        let workflows = self.find_workflows_by_type("TransactionApproval").await?;
        Ok(workflows.into_iter().filter(|w| w.status == WorkflowStatusModel::PendingAction).collect())
    }

    // Analytics and reporting methods - placeholder implementations
    async fn get_workflow_metrics(&self, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<WorkflowMetricsReport> {
        Ok(WorkflowMetricsReport {
            period_start: _from_date,
            period_end: _to_date,
            total_workflows_created: 0,
            total_workflows_completed: 0,
            total_workflows_cancelled: 0,
            total_workflows_in_progress: 0,
            average_completion_time_hours: 0.0,
            workflows_by_type: Vec::new(),
        })
    }

    async fn get_workflow_performance(&self, _workflow_type: &str, _from_date: NaiveDate, _to_date: NaiveDate) -> BankingResult<WorkflowPerformanceReport> {
        Ok(WorkflowPerformanceReport {
            workflow_type: _workflow_type.to_string(),
            period_start: _from_date,
            period_end: _to_date,
            total_workflows: 0,
            completed_workflows: 0,
            completion_rate_percentage: 0.0,
            average_completion_time_hours: 0.0,
            median_completion_time_hours: 0.0,
            fastest_completion_hours: 0.0,
            slowest_completion_hours: 0.0,
        })
    }

    async fn get_workflow_bottlenecks(&self) -> BankingResult<Vec<WorkflowBottleneckReport>> {
        Ok(Vec::new())
    }

    async fn get_average_completion_time(&self, _workflow_type: &str) -> BankingResult<Option<f64>> {
        Ok(None)
    }

    // Cleanup and maintenance methods
    async fn cleanup_completed_workflows(&self, retention_days: i32) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            DELETE FROM account_workflows 
            WHERE status = 'Completed' AND completed_at < NOW() - INTERVAL '1 day' * $1
            "#
        )
        .bind(retention_days)
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to cleanup completed workflows: {e}"),
        ))?;

        Ok(result.rows_affected() as i64)
    }

    async fn cleanup_cancelled_workflows(&self, retention_days: i32) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            DELETE FROM account_workflows 
            WHERE status = 'Cancelled' AND last_updated_at < NOW() - INTERVAL '1 day' * $1
            "#
        )
        .bind(retention_days)
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to cleanup cancelled workflows: {e}"),
        ))?;

        Ok(result.rows_affected() as i64)
    }

    async fn find_stale_workflows(&self, stale_threshold_hours: i32) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            WHERE status IN ('InProgress', 'PendingAction') 
                  AND last_updated_at < NOW() - INTERVAL '1 hour' * $1
            ORDER BY last_updated_at ASC
            "#
        )
        .bind(stale_threshold_hours)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to find stale workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    // Batch operations
    async fn bulk_update_workflow_status(&self, workflow_ids: Vec<Uuid>, status: &str) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = $2, last_updated_at = NOW()
            WHERE workflow_id = ANY($1)
            "#
        )
        .bind(&workflow_ids)
        .bind(status)
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to bulk update workflow status: {e}"),
        ))?;

        Ok(result.rows_affected() as i64)
    }

    async fn bulk_timeout_expired_workflows(&self, reference_time: DateTime<Utc>) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = 'TimedOut', last_updated_at = NOW()
            WHERE timeout_at IS NOT NULL AND timeout_at < $1 AND status IN ('InProgress', 'PendingAction')
            "#
        )
        .bind(reference_time)
        .execute(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to bulk timeout expired workflows: {e}"),
        ))?;

        Ok(result.rows_affected() as i64)
    }

    // Utility operations
    async fn workflow_exists(&self, workflow_id: Uuid) -> BankingResult<bool> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM account_workflows WHERE workflow_id = $1"
        )
        .bind(workflow_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to check workflow existence: {e}"),
        ))?;

        Ok(result.get::<i64, _>("count") > 0)
    }

    async fn count_workflows_by_type(&self, workflow_type: &str) -> BankingResult<i64> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM account_workflows WHERE workflow_type = $1::workflow_type"
        )
        .bind(workflow_type)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to count workflows by type: {e}"),
        ))?;

        Ok(result.get::<i64, _>("count"))
    }

    async fn count_workflows_by_status(&self, status: &str) -> BankingResult<i64> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM account_workflows WHERE status = $1"
        )
        .bind(status)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to count workflows by status: {e}"),
        ))?;

        Ok(result.get::<i64, _>("count"))
    }

    async fn count_pending_workflows(&self) -> BankingResult<i64> {
        self.count_workflows_by_status("PendingAction").await
    }

    async fn list_workflows(&self, offset: i64, limit: i64) -> BankingResult<Vec<AccountWorkflowModel>> {
        let rows = sqlx::query(
            r#"
            SELECT workflow_id, account_id, workflow_type::text, current_step, status, 
                   initiated_by, initiated_at, completed_at, next_action_required, timeout_at,
                   created_at, last_updated_at
            FROM account_workflows 
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to list workflows: {e}"),
        ))?;

        let mut workflows = Vec::new();
        for row in rows {
            workflows.push(AccountWorkflowModel {
                workflow_id: row.get("workflow_id"),
                account_id: row.get("account_id"),
                workflow_type: WorkflowTypeModel::from_str(&row.get::<String, _>("workflow_type"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "workflow_type".to_string(),
                        message: e,
                    })?,
                current_step: WorkflowStepModel::from_str(&row.get::<String, _>("current_step"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "current_step".to_string(),
                        message: e,
                    })?,
                status: WorkflowStatusModel::from_str(&row.get::<String, _>("status"))
                    .map_err(|e| BankingError::ValidationError {
                        field: "status".to_string(),
                        message: e,
                    })?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                completed_at: row.get("completed_at"),
                next_action_required: row.get::<Option<String>, _>("next_action_required")
                    .map(|s| HeaplessString::try_from(s.as_str()))
                    .transpose()
                    .map_err(|_| BankingError::ValidationError {
                        field: "next_action_required".to_string(),
                        message: "Next action required too long".to_string(),
                    })?,
                timeout_at: row.get("timeout_at"),
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }
        Ok(workflows)
    }

    async fn count_all_workflows(&self) -> BankingResult<i64> {
        let result = sqlx::query(
            "SELECT COUNT(*) as count FROM account_workflows"
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| BankingError::Internal(format!("Failed to count all workflows: {e}"),
        ))?;

        Ok(result.get::<i64, _>("count"))
    }
}