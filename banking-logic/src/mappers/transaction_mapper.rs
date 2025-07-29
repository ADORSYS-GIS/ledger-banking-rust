use blake3;
use banking_api::domain::{Transaction, TransactionAudit, GlEntry};
use banking_db::models::{TransactionModel, TransactionAuditModel, GlEntryModel};

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

pub struct TransactionAuditMapper;

impl TransactionAuditMapper {
    /// Map from domain TransactionAudit to database TransactionAuditModel
    pub fn to_model(audit: TransactionAudit) -> TransactionAuditModel {
        TransactionAuditModel {
            audit_id: audit.audit_id,
            transaction_id: audit.transaction_id,
            action_type: audit.action_type,
            performed_by: audit.performed_by,
            performed_at: audit.performed_at,
            old_status: audit.old_status,
            new_status: audit.new_status,
            reason_id: audit.reason_id,
            details_hash: audit.details.map(|hash| hash.as_bytes().to_vec()),
        }
    }

    /// Map from database TransactionAuditModel to domain TransactionAudit  
    pub fn from_model(model: TransactionAuditModel) -> banking_api::BankingResult<TransactionAudit> {
        let details = model.details_hash.and_then(|bytes| {
            if bytes.len() == 32 {
                let mut hash_bytes = [0u8; 32];
                hash_bytes.copy_from_slice(&bytes);
                Some(blake3::Hash::from(hash_bytes))
            } else {
                None
            }
        });

        Ok(TransactionAudit {
            audit_id: model.audit_id,
            transaction_id: model.transaction_id,
            action_type: model.action_type,
            performed_by: model.performed_by,
            performed_at: model.performed_at,
            old_status: model.old_status,
            new_status: model.new_status,
            reason_id: model.reason_id,
            details,
        })
    }
}

pub struct GlEntryMapper;

impl GlEntryMapper {
    /// Map from domain GlEntry to database GlEntryModel
    pub fn to_model(entry: GlEntry) -> GlEntryModel {
        GlEntryModel {
            entry_id: entry.entry_id,
            transaction_id: entry.transaction_id,
            account_code: entry.account_code,
            debit_amount: entry.debit_amount,
            credit_amount: entry.credit_amount,
            currency: entry.currency,
            description: entry.description,
            reference_number: entry.reference_number,
            value_date: entry.value_date,
            posting_date: entry.posting_date,
            created_at: chrono::Utc::now(),
        }
    }

    /// Map from database GlEntryModel to domain GlEntry
    pub fn from_model(model: GlEntryModel) -> banking_api::BankingResult<GlEntry> {
        Ok(GlEntry {
            entry_id: model.entry_id,
            account_code: model.account_code,
            debit_amount: model.debit_amount,
            credit_amount: model.credit_amount,
            currency: model.currency,
            description: model.description,
            reference_number: model.reference_number,
            transaction_id: model.transaction_id,
            value_date: model.value_date,
            posting_date: model.posting_date,
        })
    }
}