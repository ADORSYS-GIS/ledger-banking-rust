use banking_api::domain::Transaction;
use banking_db::models::TransactionModel;

pub struct TransactionMapper;

impl TransactionMapper {
    /// Map from domain Transaction to database TransactionModel
    pub fn to_model(transaction: Transaction) -> TransactionModel {
        TransactionModel {
            transaction_id: transaction.transaction_id,
            account_id: transaction.account_id,
            transaction_code: transaction.transaction_code,
            transaction_type: transaction.transaction_type,
            amount: transaction.amount,
            currency: transaction.currency,
            description: transaction.description,
            channel_id: transaction.channel_id,
            terminal_id: transaction.terminal_id,
            agent_user_id: transaction.agent_user_id,
            transaction_date: transaction.transaction_date,
            value_date: transaction.value_date,
            status: transaction.status,
            reference_number: transaction.reference_number,
            external_reference: transaction.external_reference,
            gl_code: transaction.gl_code,
            requires_approval: transaction.requires_approval,
            approval_status: transaction.approval_status,
            risk_score: transaction.risk_score,
            created_at: transaction.created_at,
        }
    }

    /// Map from database TransactionModel to domain Transaction
    pub fn from_model(model: TransactionModel) -> banking_api::BankingResult<Transaction> {
        Ok(Transaction {
            transaction_id: model.transaction_id,
            account_id: model.account_id,
            transaction_code: model.transaction_code,
            transaction_type: model.transaction_type,
            amount: model.amount,
            currency: model.currency,
            description: model.description,
            channel_id: model.channel_id,
            terminal_id: model.terminal_id,
            agent_user_id: model.agent_user_id,
            transaction_date: model.transaction_date,
            value_date: model.value_date,
            status: model.status,
            reference_number: model.reference_number,
            external_reference: model.external_reference,
            gl_code: model.gl_code,
            requires_approval: model.requires_approval,
            approval_status: model.approval_status,
            risk_score: model.risk_score,
            created_at: model.created_at,
        })
    }
}