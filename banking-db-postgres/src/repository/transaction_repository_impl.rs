use async_trait::async_trait;
use banking_api::{BankingResult, BankingError};
use banking_db::models::{TransactionModel, TransactionStatus, TransactionApprovalStatus};
use banking_db::models::workflow::{ApprovalWorkflowModel, WorkflowTransactionApprovalModel, WorkflowStatusModel};
use banking_db::repository::TransactionRepository;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc, NaiveDate};
use heapless::String as HeaplessString;

pub struct TransactionRepositoryImpl {
    pool: PgPool,
}

impl TransactionRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Helper function to parse transaction status string
fn parse_transaction_status(status_str: &str) -> BankingResult<TransactionStatus> {
    match status_str {
        "Pending" => Ok(TransactionStatus::Pending),
        "Posted" => Ok(TransactionStatus::Posted),
        "Reversed" => Ok(TransactionStatus::Reversed),
        "Failed" => Ok(TransactionStatus::Failed),
        "AwaitingApproval" => Ok(TransactionStatus::AwaitingApproval),
        "ApprovalRejected" => Ok(TransactionStatus::ApprovalRejected),
        _ => Err(BankingError::ValidationError {
            field: "status".to_string(),
            message: format!("Invalid transaction status: {status_str}"),
        }),
    }
}

/// Helper function to parse approval status string
fn parse_approval_status(status_str: &str) -> BankingResult<TransactionApprovalStatus> {
    match status_str {
        "Pending" => Ok(TransactionApprovalStatus::Pending),
        "Approved" => Ok(TransactionApprovalStatus::Approved),
        "Rejected" => Ok(TransactionApprovalStatus::Rejected),
        "PartiallyApproved" => Ok(TransactionApprovalStatus::PartiallyApproved),
        _ => Err(BankingError::ValidationError {
            field: "approval_status".to_string(),
            message: format!("Invalid approval status: {status_str}"),
        }),
    }
}

/// Helper function to parse workflow status string
fn parse_workflow_status(status_str: &str) -> BankingResult<WorkflowStatusModel> {
    match status_str {
        "InProgress" => Ok(WorkflowStatusModel::InProgress),
        "PendingAction" => Ok(WorkflowStatusModel::PendingAction),
        "Completed" => Ok(WorkflowStatusModel::Completed),
        "Failed" => Ok(WorkflowStatusModel::Failed),
        "Cancelled" => Ok(WorkflowStatusModel::Cancelled),
        "TimedOut" => Ok(WorkflowStatusModel::TimedOut),
        _ => Err(BankingError::ValidationError {
            field: "workflow_status".to_string(),
            message: format!("Invalid workflow status: {status_str}"),
        }),
    }
}

/// Helper function to extract TransactionModel from database row
fn extract_transaction_from_row(row: &sqlx::postgres::PgRow) -> BankingResult<TransactionModel> {
    Ok(TransactionModel {
        id: row.get("id"),
        account_id: row.get("account_id"),
        transaction_code: HeaplessString::try_from(
            row.get::<String, _>("transaction_code").as_str()
        ).map_err(|_| BankingError::ValidationError {
            field: "transaction_code".to_string(),
            message: "Transaction code too long".to_string(),
        })?,
        transaction_type: match row.get::<String, _>("transaction_type").as_str() {
            "Credit" => banking_db::models::TransactionType::Credit,
            "Debit" => banking_db::models::TransactionType::Debit,
            _ => return Err(BankingError::ValidationError {
                field: "transaction_type".to_string(),
                message: "Invalid transaction type".to_string(),
            }),
        },
        amount: row.get("amount"),
        currency: HeaplessString::try_from(
            row.get::<String, _>("currency").as_str()
        ).map_err(|_| BankingError::ValidationError {
            field: "currency".to_string(),
            message: "Currency code too long".to_string(),
        })?,
        description: HeaplessString::try_from(
            row.get::<String, _>("description").as_str()
        ).map_err(|_| BankingError::ValidationError {
            field: "description".to_string(),
            message: "Description too long".to_string(),
        })?,
        channel_id: HeaplessString::try_from(
            row.get::<String, _>("channel_id").as_str()
        ).map_err(|_| BankingError::ValidationError {
            field: "channel_id".to_string(),
            message: "Channel ID too long".to_string(),
        })?,
        terminal_id: row.get("terminal_id"),
        agent_person_id: row.get("agent_person_id"),
        transaction_date: row.get("transaction_date"),
        value_date: row.get("value_date"),
        status: parse_transaction_status(&row.get::<String, _>("status"))?,
        reference_number: HeaplessString::try_from(
            row.get::<String, _>("reference_number").as_str()
        ).map_err(|_| BankingError::ValidationError {
            field: "reference_number".to_string(),
            message: "Reference number too long".to_string(),
        })?,
        external_reference: match row.get::<Option<String>, _>("external_reference") {
            Some(ext_ref) => Some(HeaplessString::try_from(ext_ref.as_str()).map_err(|_| {
                BankingError::ValidationError {
                    field: "external_reference".to_string(),
                    message: "External reference too long".to_string(),
                }
            })?),
            None => None,
        },
        gl_code: HeaplessString::try_from(
            row.get::<String, _>("gl_code").as_str()
        ).map_err(|_| BankingError::ValidationError {
            field: "gl_code".to_string(),
            message: "GL code too long".to_string(),
        })?,
        requires_approval: row.get("requires_approval"),
        approval_status: match row.get::<Option<String>, _>("approval_status") {
            Some(status_str) => Some(parse_approval_status(&status_str)?),
            None => None,
        },
        risk_score: row.get("risk_score"),
        created_at: row.get("created_at"),
    })
}

#[async_trait]
impl TransactionRepository for TransactionRepositoryImpl {
    async fn create(&self, transaction: TransactionModel) -> BankingResult<TransactionModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO transactions (
                id, account_id, transaction_code, transaction_type, amount, currency,
                description, channel_id, terminal_id, agent_person_id, transaction_date, value_date,
                status, reference_number, external_reference, gl_code, requires_approval,
                approval_status, risk_score
            )
            VALUES (
                $1, $2, $3, $4::transaction_type, $5, $6, $7, $8, $9, $10, $11, $12,
                $13::transaction_status, $14, $15, $16, $17, $18::transaction_approval_status, $19
            )
            RETURNING id, account_id, transaction_code, transaction_type::text as transaction_type,
                     amount, currency, description, channel_id, terminal_id, agent_person_id,
                     transaction_date, value_date, status::text as status, reference_number,
                     external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                     risk_score, created_at
            "#
        )
        .bind(transaction.id)
        .bind(transaction.account_id)
        .bind(transaction.transaction_code.as_str())
        .bind(transaction.transaction_type.to_string())
        .bind(transaction.amount)
        .bind(transaction.currency.as_str())
        .bind(transaction.description.as_str())
        .bind(transaction.channel_id.as_str())
        .bind(transaction.terminal_id)
        .bind(transaction.agent_person_id)
        .bind(transaction.transaction_date)
        .bind(transaction.value_date)
        .bind(transaction.status.to_string())
        .bind(transaction.reference_number.as_str())
        .bind(transaction.external_reference.as_ref().map(|s| s.as_str()))
        .bind(transaction.gl_code.as_str())
        .bind(transaction.requires_approval)
        .bind(transaction.approval_status.as_ref().map(|s| s.to_string()))
        .bind(transaction.risk_score)
        .fetch_one(&self.pool)
        .await?;

        extract_transaction_from_row(&result)
    }

    async fn update(&self, transaction: TransactionModel) -> BankingResult<TransactionModel> {
        let result = sqlx::query(
            r#"
            UPDATE transactions SET
                account_id = $2, transaction_code = $3, transaction_type = $4::transaction_type,
                amount = $5, currency = $6, description = $7, channel_id = $8, terminal_id = $9,
                agent_person_id = $10, transaction_date = $11, value_date = $12,
                status = $13::transaction_status, reference_number = $14, external_reference = $15,
                gl_code = $16, requires_approval = $17, approval_status = $18::transaction_approval_status,
                risk_score = $19
            WHERE id = $1
            RETURNING id, account_id, transaction_code, transaction_type::text as transaction_type,
                     amount, currency, description, channel_id, terminal_id, agent_person_id,
                     transaction_date, value_date, status::text as status, reference_number,
                     external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                     risk_score, created_at
            "#
        )
        .bind(transaction.id)
        .bind(transaction.account_id)
        .bind(transaction.transaction_code.as_str())
        .bind(transaction.transaction_type.to_string())
        .bind(transaction.amount)
        .bind(transaction.currency.as_str())
        .bind(transaction.description.as_str())
        .bind(transaction.channel_id.as_str())
        .bind(transaction.terminal_id)
        .bind(transaction.agent_person_id)
        .bind(transaction.transaction_date)
        .bind(transaction.value_date)
        .bind(transaction.status.to_string())
        .bind(transaction.reference_number.as_str())
        .bind(transaction.external_reference.as_ref().map(|s| s.as_str()))
        .bind(transaction.gl_code.as_str())
        .bind(transaction.requires_approval)
        .bind(transaction.approval_status.as_ref().map(|s| s.to_string()))
        .bind(transaction.risk_score)
        .fetch_one(&self.pool)
        .await?;

        extract_transaction_from_row(&result)
    }

    async fn find_by_id(&self, id: Uuid) -> BankingResult<Option<TransactionModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(extract_transaction_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_account_id(&self, account_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>> {
        let mut query = String::from(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE account_id = $1
            "#
        );

        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }
        
        query.push_str(" ORDER BY transaction_date DESC");

        let mut db_query = sqlx::query(&query).bind(account_id);
        
        if let Some(from) = from_date {
            db_query = db_query.bind(from);
        }
        
        if let Some(to) = to_date {
            db_query = db_query.bind(to);
        }

        let results = db_query
            .fetch_all(&self.pool)
            .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_by_account_date_range(&self, account_id: Uuid, from_date: NaiveDate, to_date: NaiveDate) -> BankingResult<Vec<TransactionModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE account_id = $1 AND value_date >= $2 AND value_date <= $3
            ORDER BY transaction_date DESC
            "#
        )
        .bind(account_id)
        .bind(from_date)
        .bind(to_date)
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_by_reference(&self, reference_number: &str) -> BankingResult<Option<TransactionModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE reference_number = $1
            "#
        )
        .bind(reference_number)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(extract_transaction_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_external_reference(&self, external_reference: &str) -> BankingResult<Vec<TransactionModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE external_reference = $1
            ORDER BY transaction_date DESC
            "#
        )
        .bind(external_reference)
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_by_status(&self, status: &str) -> BankingResult<Vec<TransactionModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE status = $1::transaction_status
            ORDER BY transaction_date DESC
            "#
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_requiring_approval(&self) -> BankingResult<Vec<TransactionModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE requires_approval = true AND (approval_status IS NULL OR approval_status = 'Pending')
            ORDER BY transaction_date ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_by_terminal_id(&self, terminal_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>> {
        let mut query = String::from(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE terminal_id = $1
            "#
        );

        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }
        
        query.push_str(" ORDER BY transaction_date DESC");

        let mut db_query = sqlx::query(&query).bind(terminal_id);
        
        if let Some(from) = from_date {
            db_query = db_query.bind(from);
        }
        
        if let Some(to) = to_date {
            db_query = db_query.bind(to);
        }

        let results = db_query
            .fetch_all(&self.pool)
            .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_by_agent_person_id(&self, agent_person_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>> {
        let mut query = String::from(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE agent_person_id = $1
            "#
        );

        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }
        
        query.push_str(" ORDER BY transaction_date DESC");

        let mut db_query = sqlx::query(&query).bind(agent_person_id);
        
        if let Some(from) = from_date {
            db_query = db_query.bind(from);
        }
        
        if let Some(to) = to_date {
            db_query = db_query.bind(to);
        }

        let results = db_query
            .fetch_all(&self.pool)
            .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn find_by_channel(&self, channel_id: &str, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<Vec<TransactionModel>> {
        let mut query = String::from(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE channel_id = $1
            "#
        );

        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }
        
        query.push_str(" ORDER BY transaction_date DESC");

        let mut db_query = sqlx::query(&query).bind(channel_id);
        
        if let Some(from) = from_date {
            db_query = db_query.bind(from);
        }
        
        if let Some(to) = to_date {
            db_query = db_query.bind(to);
        }

        let results = db_query
            .fetch_all(&self.pool)
            .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn update_status(&self, id: Uuid, status: &str, _reason: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE transactions 
            SET status = $2::transaction_status
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_approval_status(&self, id: Uuid, approval_status: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE transactions 
            SET approval_status = $2::transaction_approval_status
            WHERE id = $1
            "#
        )
        .bind(id)
        .bind(approval_status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_last_customer_transaction(&self, account_id: Uuid) -> BankingResult<Option<TransactionModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE account_id = $1 
              AND channel_id NOT IN ('System', 'AutoInterest', 'AutoFee')
            ORDER BY transaction_date DESC
            LIMIT 1
            "#
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(extract_transaction_from_row(&row)?)),
            None => Ok(None),
        }
    }

    async fn calculate_daily_volume_by_terminal(&self, terminal_id: Uuid, date: NaiveDate) -> BankingResult<Decimal> {
        let result = sqlx::query(
            r#"
            SELECT COALESCE(SUM(amount), 0) as total_volume
            FROM transactions
            WHERE terminal_id = $1 AND value_date = $2 AND status = 'Posted'
            "#
        )
        .bind(terminal_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.get("total_volume"))
    }

    async fn calculate_daily_volume_by_branch(&self, branch_id: Uuid, date: NaiveDate) -> BankingResult<Decimal> {
        let result = sqlx::query(
            r#"
            SELECT COALESCE(SUM(t.amount), 0) as total_volume
            FROM transactions t
            JOIN agent_terminals at ON t.terminal_id = at.terminal_id
            WHERE at.branch_id = $1 AND t.value_date = $2 AND t.status = 'Posted'
            "#
        )
        .bind(branch_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.get("total_volume"))
    }

    async fn calculate_daily_volume_by_network(&self, network_id: Uuid, date: NaiveDate) -> BankingResult<Decimal> {
        let result = sqlx::query(
            r#"
            SELECT COALESCE(SUM(t.amount), 0) as total_volume
            FROM transactions t
            JOIN agent_terminals at ON t.terminal_id = at.terminal_id
            JOIN agent_branches ab ON at.branch_id = ab.branch_id
            WHERE ab.network_id = $1 AND t.value_date = $2 AND t.status = 'Posted'
            "#
        )
        .bind(network_id)
        .bind(date)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.get("total_volume"))
    }

    async fn reverse_transaction(&self, original_id: Uuid, reversal_transaction: TransactionModel) -> BankingResult<TransactionModel> {
        let mut tx = self.pool.begin().await?;

        // Update original transaction status to Reversed
        sqlx::query(
            "UPDATE transactions SET status = 'Reversed' WHERE id = $1"
        )
        .bind(original_id)
        .execute(&mut *tx)
        .await?;

        // Insert reversal transaction
        let result = sqlx::query(
            r#"
            INSERT INTO transactions (
                id, account_id, transaction_code, transaction_type, amount, currency,
                description, channel_id, terminal_id, agent_person_id, transaction_date, value_date,
                status, reference_number, external_reference, gl_code, requires_approval,
                approval_status, risk_score
            )
            VALUES (
                $1, $2, $3, $4::transaction_type, $5, $6, $7, $8, $9, $10, $11, $12,
                $13::transaction_status, $14, $15, $16, $17, $18::transaction_approval_status, $19
            )
            RETURNING id, account_id, transaction_code, transaction_type::text as transaction_type,
                     amount, currency, description, channel_id, terminal_id, agent_person_id,
                     transaction_date, value_date, status::text as status, reference_number,
                     external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                     risk_score, created_at
            "#
        )
        .bind(reversal_transaction.id)
        .bind(reversal_transaction.account_id)
        .bind(reversal_transaction.transaction_code.as_str())
        .bind(reversal_transaction.transaction_type.to_string())
        .bind(reversal_transaction.amount)
        .bind(reversal_transaction.currency.as_str())
        .bind(reversal_transaction.description.as_str())
        .bind(reversal_transaction.channel_id.as_str())
        .bind(reversal_transaction.terminal_id)
        .bind(reversal_transaction.agent_person_id)
        .bind(reversal_transaction.transaction_date)
        .bind(reversal_transaction.value_date)
        .bind(reversal_transaction.status.to_string())
        .bind(reversal_transaction.reference_number.as_str())
        .bind(reversal_transaction.external_reference.as_ref().map(|s| s.as_str()))
        .bind(reversal_transaction.gl_code.as_str())
        .bind(reversal_transaction.requires_approval)
        .bind(reversal_transaction.approval_status.as_ref().map(|s| s.to_string()))
        .bind(reversal_transaction.risk_score)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        extract_transaction_from_row(&result)
    }

    async fn find_for_reconciliation(&self, channel_id: &str, date: NaiveDate) -> BankingResult<Vec<TransactionModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            WHERE channel_id = $1 AND value_date = $2 AND status IN ('Posted', 'Pending')
            ORDER BY transaction_date ASC
            "#
        )
        .bind(channel_id)
        .bind(date)
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    // Workflow operations - using account_workflows table as approval workflow
    async fn create_workflow(&self, workflow: ApprovalWorkflowModel) -> BankingResult<ApprovalWorkflowModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO account_workflows (
                id, account_id, workflow_type, current_step, status,
                initiated_by, initiated_at, timeout_at, created_at, last_updated_at
            )
            VALUES (
                $1, $2, 'TransactionApproval', 'ApprovalRequired', $3, $4, $5, $6, NOW(), NOW()
            )
            RETURNING id, account_id, workflow_type::text as workflow_type,
                     current_step, status, initiated_by, initiated_at, completed_at,
                     timeout_at, created_at, last_updated_at
            "#
        )
        .bind(workflow.id)
        .bind(workflow.account_id)
        .bind(workflow.status.to_string())
        .bind(workflow.initiated_by)
        .bind(workflow.initiated_at)
        .bind(workflow.timeout_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(ApprovalWorkflowModel {
            id: result.get("id"),
            transaction_id: workflow.transaction_id,
            account_id: Some(result.get("account_id")),
            approval_type: workflow.approval_type,
            minimum_approvals: workflow.minimum_approvals,
            current_approvals: 0,
            status: parse_workflow_status(&result.get::<String, _>("status"))?,
            initiated_by: result.get("initiated_by"),
            initiated_at: result.get("initiated_at"),
            timeout_at: result.get("timeout_at"),
            completed_at: result.get("completed_at"),
            rejection_reason_id: None,
            created_at: result.get("created_at"),
            last_updated_at: result.get("last_updated_at"),
        })
    }

    async fn find_workflow_by_id(&self, workflow_id: Uuid) -> BankingResult<Option<ApprovalWorkflowModel>> {
        let result = sqlx::query(
            r#"
            SELECT id, account_id, workflow_type::text as workflow_type,
                   current_step, status, initiated_by, initiated_at, completed_at,
                   timeout_at, created_at, last_updated_at
            FROM account_workflows
            WHERE id = $1
            "#
        )
        .bind(workflow_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(ApprovalWorkflowModel {
                id: row.get("id"),
                transaction_id: None,
                account_id: Some(row.get("account_id")),
                approval_type: HeaplessString::try_from("TransactionApproval").unwrap(),
                minimum_approvals: 1,
                current_approvals: 0,
                status: parse_workflow_status(&row.get::<String, _>("status"))?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                timeout_at: row.get("timeout_at"),
                completed_at: row.get("completed_at"),
                rejection_reason_id: None,
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn find_workflow_by_transaction(&self, transaction_id: Uuid) -> BankingResult<Option<ApprovalWorkflowModel>> {
        let result = sqlx::query(
            r#"
            SELECT aw.id, aw.account_id, aw.workflow_type::text as workflow_type,
                   aw.current_step, aw.status, aw.initiated_by, aw.initiated_at, aw.completed_at,
                   aw.timeout_at, aw.created_at, aw.last_updated_at
            FROM account_workflows aw
            JOIN transaction_approvals ta ON aw.id = ta.workflow_id
            WHERE ta.id = $1
            LIMIT 1
            "#
        )
        .bind(transaction_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(row) => Ok(Some(ApprovalWorkflowModel {
                id: row.get("id"),
                transaction_id: Some(transaction_id),
                account_id: Some(row.get("account_id")),
                approval_type: HeaplessString::try_from("TransactionApproval").unwrap(),
                minimum_approvals: 1,
                current_approvals: 0,
                status: parse_workflow_status(&row.get::<String, _>("status"))?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                timeout_at: row.get("timeout_at"),
                completed_at: row.get("completed_at"),
                rejection_reason_id: None,
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            })),
            None => Ok(None),
        }
    }

    async fn update_workflow_status(&self, workflow_id: Uuid, status: &str) -> BankingResult<()> {
        sqlx::query(
            r#"
            UPDATE account_workflows 
            SET status = $2, last_updated_at = NOW()
            WHERE id = $1
            "#
        )
        .bind(workflow_id)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_pending_workflows(&self) -> BankingResult<Vec<ApprovalWorkflowModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, workflow_type::text as workflow_type,
                   current_step, status, initiated_by, initiated_at, completed_at,
                   timeout_at, created_at, last_updated_at
            FROM account_workflows
            WHERE status IN ('InProgress', 'PendingAction')
            ORDER BY initiated_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut workflows = Vec::new();
        for row in results {
            workflows.push(ApprovalWorkflowModel {
                id: row.get("id"),
                transaction_id: None,
                account_id: Some(row.get("account_id")),
                approval_type: HeaplessString::try_from("TransactionApproval").unwrap(),
                minimum_approvals: 1,
                current_approvals: 0,
                status: parse_workflow_status(&row.get::<String, _>("status"))?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                timeout_at: row.get("timeout_at"),
                completed_at: row.get("completed_at"),
                rejection_reason_id: None,
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }

        Ok(workflows)
    }

    async fn find_expired_workflows(&self, reference_time: DateTime<Utc>) -> BankingResult<Vec<ApprovalWorkflowModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, workflow_type::text as workflow_type,
                   current_step, status, initiated_by, initiated_at, completed_at,
                   timeout_at, created_at, last_updated_at
            FROM account_workflows
            WHERE timeout_at < $1 AND status IN ('InProgress', 'PendingAction')
            ORDER BY timeout_at ASC
            "#
        )
        .bind(reference_time)
        .fetch_all(&self.pool)
        .await?;

        let mut workflows = Vec::new();
        for row in results {
            workflows.push(ApprovalWorkflowModel {
                id: row.get("id"),
                transaction_id: None,
                account_id: Some(row.get("account_id")),
                approval_type: HeaplessString::try_from("TransactionApproval").unwrap(),
                minimum_approvals: 1,
                current_approvals: 0,
                status: parse_workflow_status(&row.get::<String, _>("status"))?,
                initiated_by: row.get("initiated_by"),
                initiated_at: row.get("initiated_at"),
                timeout_at: row.get("timeout_at"),
                completed_at: row.get("completed_at"),
                rejection_reason_id: None,
                created_at: row.get("created_at"),
                last_updated_at: row.get("last_updated_at"),
            });
        }

        Ok(workflows)
    }

    // Transaction approval operations - using transaction_approvals table
    async fn create_approval(&self, approval: WorkflowTransactionApprovalModel) -> BankingResult<WorkflowTransactionApprovalModel> {
        let result = sqlx::query(
            r#"
            INSERT INTO transaction_approvals (
                id, workflow_id, transaction_id, approver_person_id, approval_action,
                approved_at, approval_notes
            )
            VALUES ($1, $2, $3, $4, $5::transaction_approval_status, $6, $7)
            RETURNING id, workflow_id, transaction_id, approver_person_id,
                     approval_action::text as approval_action, approved_at, approval_notes, created_at
            "#
        )
        .bind(approval.id)
        .bind(approval.workflow_id)
        .bind(approval.transaction_id)
        .bind(approval.approver_person_id)
        .bind(approval.approval_action.as_str())
        .bind(approval.approved_at)
        .bind(approval.approval_notes.as_ref().map(|s| s.as_str()))
        .fetch_one(&self.pool)
        .await?;

        Ok(WorkflowTransactionApprovalModel {
            id: result.get("id"),
            workflow_id: result.get("workflow_id"),
            transaction_id: result.get("transaction_id"),
            approver_person_id: result.get("approver_person_id"),
            approval_action: HeaplessString::try_from(
                result.get::<String, _>("approval_action").as_str()
            ).unwrap(),
            approved_at: result.get("approved_at"),
            approval_notes: result.get::<Option<String>, _>("approval_notes").map(|notes| HeaplessString::try_from(notes.as_str()).unwrap()),
            approval_method: approval.approval_method,
            approval_location: approval.approval_location,
            created_at: result.get("created_at"),
        })
    }

    async fn find_approvals_by_workflow(&self, workflow_id: Uuid) -> BankingResult<Vec<WorkflowTransactionApprovalModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, workflow_id, transaction_id, approver_person_id,
                   approval_action::text as approval_action, approved_at, approval_notes, created_at
            FROM transaction_approvals
            WHERE workflow_id = $1
            ORDER BY approved_at DESC
            "#
        )
        .bind(workflow_id)
        .fetch_all(&self.pool)
        .await?;

        let mut approvals = Vec::new();
        for row in results {
            approvals.push(WorkflowTransactionApprovalModel {
                id: row.get("id"),
                workflow_id: row.get("workflow_id"),
                transaction_id: row.get("transaction_id"),
                approver_person_id: row.get("approver_person_id"),
                approval_action: HeaplessString::try_from(
                    row.get::<String, _>("approval_action").as_str()
                ).unwrap(),
                approved_at: row.get("approved_at"),
                approval_notes: row.get::<Option<String>, _>("approval_notes").map(|notes| HeaplessString::try_from(notes.as_str()).unwrap()),
                approval_method: HeaplessString::try_from("Manual").unwrap(),
                approval_location: None,
                created_at: row.get("created_at"),
            });
        }

        Ok(approvals)
    }

    async fn find_approvals_by_approver(&self, approver_person_id: Uuid) -> BankingResult<Vec<WorkflowTransactionApprovalModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, workflow_id, transaction_id, approver_person_id,
                   approval_action::text as approval_action, approved_at, approval_notes, created_at
            FROM transaction_approvals
            WHERE approver_person_id = $1
            ORDER BY approved_at DESC
            "#
        )
        .bind(approver_person_id)
        .fetch_all(&self.pool)
        .await?;

        let mut approvals = Vec::new();
        for row in results {
            approvals.push(WorkflowTransactionApprovalModel {
                id: row.get("id"),
                workflow_id: row.get("workflow_id"),
                transaction_id: row.get("transaction_id"),
                approver_person_id: row.get("approver_person_id"),
                approval_action: HeaplessString::try_from(
                    row.get::<String, _>("approval_action").as_str()
                ).unwrap(),
                approved_at: row.get("approved_at"),
                approval_notes: row.get::<Option<String>, _>("approval_notes").map(|notes| HeaplessString::try_from(notes.as_str()).unwrap()),
                approval_method: HeaplessString::try_from("Manual").unwrap(),
                approval_location: None,
                created_at: row.get("created_at"),
            });
        }

        Ok(approvals)
    }

    async fn count_approvals_for_workflow(&self, workflow_id: Uuid) -> BankingResult<i64> {
        let result = sqlx::query(
            r#"
            SELECT COUNT(*) as approval_count
            FROM transaction_approvals
            WHERE workflow_id = $1 AND approval_action = 'Approved'
            "#
        )
        .bind(workflow_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.get("approval_count"))
    }

    // Utility operations
    async fn exists(&self, transaction_id: Uuid) -> BankingResult<bool> {
        let result = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM transactions WHERE id = $1) as exists"
        )
        .bind(transaction_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.get("exists"))
    }

    async fn count_by_account(&self, account_id: Uuid, from_date: Option<NaiveDate>, to_date: Option<NaiveDate>) -> BankingResult<i64> {
        let mut query = String::from("SELECT COUNT(*) as transaction_count FROM transactions WHERE account_id = $1");
        let mut param_count = 1;
        
        if from_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date >= ${param_count}"));
        }
        
        if to_date.is_some() {
            param_count += 1;
            query.push_str(&format!(" AND value_date <= ${param_count}"));
        }

        let mut db_query = sqlx::query(&query).bind(account_id);
        
        if let Some(from) = from_date {
            db_query = db_query.bind(from);
        }
        
        if let Some(to) = to_date {
            db_query = db_query.bind(to);
        }

        let result = db_query
            .fetch_one(&self.pool)
            .await?;

        Ok(result.get("transaction_count"))
    }

    async fn list(&self, offset: i64, limit: i64) -> BankingResult<Vec<TransactionModel>> {
        let results = sqlx::query(
            r#"
            SELECT id, account_id, transaction_code, transaction_type::text as transaction_type,
                   amount, currency, description, channel_id, terminal_id, agent_person_id,
                   transaction_date, value_date, status::text as status, reference_number,
                   external_reference, gl_code, requires_approval, approval_status::text as approval_status,
                   risk_score, created_at
            FROM transactions
            ORDER BY transaction_date DESC, id ASC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let mut transactions = Vec::new();
        for row in results {
            transactions.push(extract_transaction_from_row(&row)?);
        }

        Ok(transactions)
    }

    async fn count(&self) -> BankingResult<i64> {
        let result = sqlx::query("SELECT COUNT(*) as transaction_count FROM transactions")
            .fetch_one(&self.pool)
            .await?;

        Ok(result.get("transaction_count"))
    }
}