use banking_api::domain::{Transaction, TransactionType, TransactionStatus, TransactionApprovalStatus};
use banking_db::models::TransactionModel;

pub struct TransactionMapper;

impl TransactionMapper {
    /// Map from domain Transaction to database TransactionModel
    pub fn to_model(transaction: Transaction) -> TransactionModel {
        TransactionModel {
            transaction_id: transaction.transaction_id,
            account_id: transaction.account_id,
            transaction_code: transaction.transaction_code,
            transaction_type: Self::transaction_type_to_string(transaction.transaction_type),
            amount: transaction.amount,
            currency: transaction.currency,
            description: transaction.description,
            channel_id: transaction.channel_id,
            terminal_id: transaction.terminal_id,
            agent_user_id: transaction.agent_user_id,
            transaction_date: transaction.transaction_date,
            value_date: transaction.value_date,
            status: Self::transaction_status_to_string(transaction.status),
            reference_number: transaction.reference_number,
            external_reference: transaction.external_reference,
            gl_code: transaction.gl_code,
            requires_approval: transaction.requires_approval,
            approval_status: transaction.approval_status.map(Self::approval_status_to_string),
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
            transaction_type: Self::string_to_transaction_type(&model.transaction_type)?,
            amount: model.amount,
            currency: model.currency,
            description: model.description,
            channel_id: model.channel_id,
            terminal_id: model.terminal_id,
            agent_user_id: model.agent_user_id,
            transaction_date: model.transaction_date,
            value_date: model.value_date,
            status: Self::string_to_transaction_status(&model.status)?,
            reference_number: model.reference_number,
            external_reference: model.external_reference,
            gl_code: model.gl_code,
            requires_approval: model.requires_approval,
            approval_status: model.approval_status.map(|s| Self::string_to_approval_status(&s)).transpose()?,
            risk_score: model.risk_score,
            created_at: model.created_at,
        })
    }

    // Helper methods for enum conversions
    fn transaction_type_to_string(transaction_type: TransactionType) -> String {
        match transaction_type {
            TransactionType::Credit => "Credit".to_string(),
            TransactionType::Debit => "Debit".to_string(),
        }
    }

    fn string_to_transaction_type(s: &str) -> banking_api::BankingResult<TransactionType> {
        match s {
            "Credit" => Ok(TransactionType::Credit),
            "Debit" => Ok(TransactionType::Debit),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "transaction_type".to_string(),
                value: s.to_string(),
            }),
        }
    }

    fn transaction_status_to_string(status: TransactionStatus) -> String {
        match status {
            TransactionStatus::Pending => "Pending".to_string(),
            TransactionStatus::Posted => "Posted".to_string(),
            TransactionStatus::Reversed => "Reversed".to_string(),
            TransactionStatus::Failed => "Failed".to_string(),
            TransactionStatus::AwaitingApproval => "AwaitingApproval".to_string(),
            TransactionStatus::ApprovalRejected => "ApprovalRejected".to_string(),
        }
    }

    fn string_to_transaction_status(s: &str) -> banking_api::BankingResult<TransactionStatus> {
        match s {
            "Pending" => Ok(TransactionStatus::Pending),
            "Posted" => Ok(TransactionStatus::Posted),
            "Reversed" => Ok(TransactionStatus::Reversed),
            "Failed" => Ok(TransactionStatus::Failed),
            "AwaitingApproval" => Ok(TransactionStatus::AwaitingApproval),
            "ApprovalRejected" => Ok(TransactionStatus::ApprovalRejected),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "transaction_status".to_string(),
                value: s.to_string(),
            }),
        }
    }

    fn approval_status_to_string(status: TransactionApprovalStatus) -> String {
        match status {
            TransactionApprovalStatus::Pending => "Pending".to_string(),
            TransactionApprovalStatus::Approved => "Approved".to_string(),
            TransactionApprovalStatus::Rejected => "Rejected".to_string(),
            TransactionApprovalStatus::PartiallyApproved => "PartiallyApproved".to_string(),
        }
    }

    fn string_to_approval_status(s: &str) -> banking_api::BankingResult<TransactionApprovalStatus> {
        match s {
            "Pending" => Ok(TransactionApprovalStatus::Pending),
            "Approved" => Ok(TransactionApprovalStatus::Approved),
            "Rejected" => Ok(TransactionApprovalStatus::Rejected),
            "PartiallyApproved" => Ok(TransactionApprovalStatus::PartiallyApproved),
            _ => Err(banking_api::BankingError::InvalidEnumValue {
                field: "approval_status".to_string(),
                value: s.to_string(),
            }),
        }
    }
}