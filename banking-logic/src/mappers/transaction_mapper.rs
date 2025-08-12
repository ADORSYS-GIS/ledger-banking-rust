use banking_api::domain::{
    Transaction, TransactionAudit, GlEntry, TransactionRequest, TransactionResult, 
    ValidationResult, Approval, ChannelType,
    TransactionType, TransactionStatus, TransactionApprovalStatus, TransactionAuditAction
};
use banking_db::models::{
    TransactionModel, TransactionAuditModel, GlEntryModel, 
    TransactionRequestModel, TransactionResultModel, ValidationResultModel,
    ApprovalModel, ChannelType as DbChannelType,
    TransactionType as DbTransactionType, TransactionStatus as DbTransactionStatus,
    TransactionApprovalStatus as DbTransactionApprovalStatus, TransactionAuditAction as DbTransactionAuditAction
};
use std::collections::HashMap;

pub struct TransactionMapper;

impl TransactionMapper {
    /// Map from domain Transaction to database TransactionModel
    pub fn to_model(transaction: Transaction) -> TransactionModel {
        TransactionModel {
            id: transaction.id,
            account_id: transaction.id,
            transaction_code: transaction.transaction_code,
            transaction_type: Self::transaction_type_to_db(transaction.transaction_type),
            amount: transaction.amount,
            currency: transaction.currency,
            description: transaction.description,
            channel_id: transaction.channel_id,
            terminal_id: transaction.terminal_id,
            agent_person_id: transaction.agent_person_id,
            transaction_date: transaction.transaction_date,
            value_date: transaction.value_date,
            status: Self::transaction_status_to_db(transaction.status),
            reference_number: transaction.reference_number,
            external_reference: transaction.external_reference,
            gl_code: transaction.gl_code,
            requires_approval: transaction.requires_approval,
            approval_status: transaction.approval_status.map(Self::approval_status_to_db),
            risk_score: transaction.risk_score,
            created_at: transaction.created_at,
        }
    }

    /// Map from database TransactionModel to domain Transaction
    pub fn from_model(model: TransactionModel) -> banking_api::BankingResult<Transaction> {
        Ok(Transaction {
            id: model.id,
            account_id: model.id,
            transaction_code: model.transaction_code,
            transaction_type: Self::db_to_transaction_type(model.transaction_type),
            amount: model.amount,
            currency: model.currency,
            description: model.description,
            channel_id: model.channel_id,
            terminal_id: model.terminal_id,
            agent_person_id: model.agent_person_id,
            transaction_date: model.transaction_date,
            value_date: model.value_date,
            status: Self::db_to_transaction_status(model.status),
            reference_number: model.reference_number,
            external_reference: model.external_reference,
            gl_code: model.gl_code,
            requires_approval: model.requires_approval,
            approval_status: model.approval_status.map(Self::db_to_approval_status),
            risk_score: model.risk_score,
            created_at: model.created_at,
        })
    }

    // Helper methods for enum conversions
    pub fn transaction_type_to_db(transaction_type: TransactionType) -> DbTransactionType {
        match transaction_type {
            TransactionType::Credit => DbTransactionType::Credit,
            TransactionType::Debit => DbTransactionType::Debit,
        }
    }

    pub fn db_to_transaction_type(db_type: DbTransactionType) -> TransactionType {
        match db_type {
            DbTransactionType::Credit => TransactionType::Credit,
            DbTransactionType::Debit => TransactionType::Debit,
        }
    }

    pub fn transaction_status_to_db(status: TransactionStatus) -> DbTransactionStatus {
        match status {
            TransactionStatus::Pending => DbTransactionStatus::Pending,
            TransactionStatus::Posted => DbTransactionStatus::Posted,
            TransactionStatus::Reversed => DbTransactionStatus::Reversed,
            TransactionStatus::Failed => DbTransactionStatus::Failed,
            TransactionStatus::AwaitingApproval => DbTransactionStatus::AwaitingApproval,
            TransactionStatus::ApprovalRejected => DbTransactionStatus::ApprovalRejected,
        }
    }

    pub fn db_to_transaction_status(db_status: DbTransactionStatus) -> TransactionStatus {
        match db_status {
            DbTransactionStatus::Pending => TransactionStatus::Pending,
            DbTransactionStatus::Posted => TransactionStatus::Posted,
            DbTransactionStatus::Reversed => TransactionStatus::Reversed,
            DbTransactionStatus::Failed => TransactionStatus::Failed,
            DbTransactionStatus::AwaitingApproval => TransactionStatus::AwaitingApproval,
            DbTransactionStatus::ApprovalRejected => TransactionStatus::ApprovalRejected,
        }
    }

    fn approval_status_to_db(status: TransactionApprovalStatus) -> DbTransactionApprovalStatus {
        match status {
            TransactionApprovalStatus::Pending => DbTransactionApprovalStatus::Pending,
            TransactionApprovalStatus::Approved => DbTransactionApprovalStatus::Approved,
            TransactionApprovalStatus::Rejected => DbTransactionApprovalStatus::Rejected,
            TransactionApprovalStatus::PartiallyApproved => DbTransactionApprovalStatus::PartiallyApproved,
        }
    }

    fn db_to_approval_status(db_status: DbTransactionApprovalStatus) -> TransactionApprovalStatus {
        match db_status {
            DbTransactionApprovalStatus::Pending => TransactionApprovalStatus::Pending,
            DbTransactionApprovalStatus::Approved => TransactionApprovalStatus::Approved,
            DbTransactionApprovalStatus::Rejected => TransactionApprovalStatus::Rejected,
            DbTransactionApprovalStatus::PartiallyApproved => TransactionApprovalStatus::PartiallyApproved,
        }
    }
}

pub struct TransactionAuditMapper;

impl TransactionAuditMapper {
    /// Map from domain TransactionAudit to database TransactionAuditModel
    pub fn to_model(audit: TransactionAudit) -> TransactionAuditModel {
        TransactionAuditModel {
            id: audit.id,
            transaction_id: audit.id,
            action_type: Self::audit_action_to_db(audit.action_type),
            performed_by_person_id: audit.performed_by_person_id,
            performed_at: audit.performed_at,
            old_status: audit.old_status.map(TransactionMapper::transaction_status_to_db),
            new_status: audit.new_status.map(TransactionMapper::transaction_status_to_db),
            reason_id: audit.reason_id,
            details: audit.details,
        }
    }

    /// Map from database TransactionAuditModel to domain TransactionAudit  
    pub fn from_model(model: TransactionAuditModel) -> banking_api::BankingResult<TransactionAudit> {
        Ok(TransactionAudit {
            id: model.id,
            transaction_id: model.id,
            action_type: Self::db_to_audit_action(model.action_type),
            performed_by_person_id: model.performed_by_person_id,
            performed_at: model.performed_at,
            old_status: model.old_status.map(TransactionMapper::db_to_transaction_status),
            new_status: model.new_status.map(TransactionMapper::db_to_transaction_status),
            reason_id: model.reason_id,
            details: model.details,
        })
    }

    fn audit_action_to_db(action: TransactionAuditAction) -> DbTransactionAuditAction {
        match action {
            TransactionAuditAction::Created => DbTransactionAuditAction::Created,
            TransactionAuditAction::StatusChanged => DbTransactionAuditAction::StatusChanged,
            TransactionAuditAction::Posted => DbTransactionAuditAction::Posted,
            TransactionAuditAction::Reversed => DbTransactionAuditAction::Reversed,
            TransactionAuditAction::Failed => DbTransactionAuditAction::Failed,
            TransactionAuditAction::Approved => DbTransactionAuditAction::Approved,
            TransactionAuditAction::Rejected => DbTransactionAuditAction::Rejected,
        }
    }

    fn db_to_audit_action(db_action: DbTransactionAuditAction) -> TransactionAuditAction {
        match db_action {
            DbTransactionAuditAction::Created => TransactionAuditAction::Created,
            DbTransactionAuditAction::StatusChanged => TransactionAuditAction::StatusChanged,
            DbTransactionAuditAction::Posted => TransactionAuditAction::Posted,
            DbTransactionAuditAction::Reversed => TransactionAuditAction::Reversed,
            DbTransactionAuditAction::Failed => TransactionAuditAction::Failed,
            DbTransactionAuditAction::Approved => TransactionAuditAction::Approved,
            DbTransactionAuditAction::Rejected => TransactionAuditAction::Rejected,
        }
    }
}

pub struct GlEntryMapper;

impl GlEntryMapper {
    /// Map from domain GlEntry to database GlEntryModel
    pub fn to_model(entry: GlEntry) -> GlEntryModel {
        GlEntryModel {
            id: entry.id,
            transaction_id: entry.id,
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
            id: model.id,
            account_code: model.account_code,
            debit_amount: model.debit_amount,
            credit_amount: model.credit_amount,
            currency: model.currency,
            description: model.description,
            reference_number: model.reference_number,
            transaction_id: model.id,
            value_date: model.value_date,
            posting_date: model.posting_date,
        })
    }
}

pub struct TransactionRequestMapper;

impl TransactionRequestMapper {
    /// Map from domain TransactionRequest to database TransactionRequestModel
    pub fn to_model(request: TransactionRequest) -> TransactionRequestModel {
        let metadata_json = serde_json::to_string(&request.metadata)
            .unwrap_or_else(|_| "{}".to_string());
            
        TransactionRequestModel {
            id: uuid::Uuid::new_v4(),
            account_id: request.account_id,
            transaction_type: TransactionMapper::transaction_type_to_db(request.transaction_type),
            amount: request.amount,
            currency: request.currency,
            description: request.description,
            channel: Self::map_channel_type(request.channel),
            terminal_id: request.terminal_id,
            initiator_person_id: request.initiator_person_id,
            external_reference: request.external_reference,
            metadata: metadata_json,
            created_at: chrono::Utc::now(),
        }
    }

    /// Map from database TransactionRequestModel to domain TransactionRequest
    pub fn from_model(model: TransactionRequestModel) -> banking_api::BankingResult<TransactionRequest> {
        let metadata: HashMap<String, String> = serde_json::from_str(&model.metadata)
            .unwrap_or_else(|_| HashMap::new());
            
        Ok(TransactionRequest {
            account_id: model.id,
            transaction_type: TransactionMapper::db_to_transaction_type(model.transaction_type),
            amount: model.amount,
            currency: model.currency,
            description: model.description,
            channel: Self::map_db_channel_type(model.channel),
            terminal_id: model.terminal_id,
            initiator_person_id: model.initiator_person_id,
            external_reference: model.external_reference,
            metadata,
        })
    }
    
    fn map_channel_type(channel: ChannelType) -> DbChannelType {
        match channel {
            ChannelType::MobileApp => DbChannelType::MobileApp,
            ChannelType::AgentTerminal => DbChannelType::AgentTerminal,
            ChannelType::ATM => DbChannelType::ATM,
            ChannelType::InternetBanking => DbChannelType::InternetBanking,
            ChannelType::BranchTeller => DbChannelType::BranchTeller,
            ChannelType::USSD => DbChannelType::USSD,
            ChannelType::ApiGateway => DbChannelType::ApiGateway,
        }
    }
    
    fn map_db_channel_type(channel: DbChannelType) -> ChannelType {
        match channel {
            DbChannelType::MobileApp => ChannelType::MobileApp,
            DbChannelType::AgentTerminal => ChannelType::AgentTerminal,
            DbChannelType::ATM => ChannelType::ATM,
            DbChannelType::InternetBanking => ChannelType::InternetBanking,
            DbChannelType::BranchTeller => ChannelType::BranchTeller,
            DbChannelType::USSD => ChannelType::USSD,
            DbChannelType::ApiGateway => ChannelType::ApiGateway,
        }
    }
}

pub struct TransactionResultMapper;

impl TransactionResultMapper {
    /// Map from domain TransactionResult to database TransactionResultModel
    pub fn to_model(result: TransactionResult) -> TransactionResultModel {
        TransactionResultModel {
            id: uuid::Uuid::new_v4(),
            transaction_id: result.transaction_id,
            reference_number: result.reference_number,
            timestamp: result.timestamp,
            created_at: chrono::Utc::now(),
        }
    }

    /// Map from database TransactionResultModel to domain TransactionResult
    pub fn from_model(model: TransactionResultModel) -> banking_api::BankingResult<TransactionResult> {
        Ok(TransactionResult {
            transaction_id: model.id,
            reference_number: model.reference_number,
            gl_entries: vec![], // GL entries would be loaded separately
            timestamp: model.timestamp,
        })
    }
}

pub struct ValidationResultMapper;

impl ValidationResultMapper {
    /// Map from domain ValidationResult to database ValidationResultModel
    pub fn to_model(validation: ValidationResult, transaction_id: Option<uuid::Uuid>) -> ValidationResultModel {
        let errors_json = serde_json::to_string(&validation.errors)
            .unwrap_or_else(|_| "[]".to_string());
        let warnings_json = serde_json::to_string(&validation.warnings)
            .unwrap_or_else(|_| "[]".to_string());
            
        ValidationResultModel {
            id: uuid::Uuid::new_v4(),
            transaction_id,
            is_valid: validation.is_valid,
            errors: errors_json,
            warnings: warnings_json,
            created_at: chrono::Utc::now(),
        }
    }

    /// Map from database ValidationResultModel to domain ValidationResult
    pub fn from_model(model: ValidationResultModel) -> banking_api::BankingResult<ValidationResult> {
        let errors: Vec<String> = serde_json::from_str(&model.errors)
            .unwrap_or_else(|_| vec![]);
        let warnings: Vec<String> = serde_json::from_str(&model.warnings)
            .unwrap_or_else(|_| vec![]);
            
        Ok(ValidationResult::new(model.is_valid, errors, warnings))
    }
}

pub struct ApprovalMapper;

impl ApprovalMapper {
    /// Map from domain Approval to database ApprovalModel
    pub fn to_model(approval: Approval, transaction_id: uuid::Uuid) -> ApprovalModel {
        ApprovalModel {
            id: approval.id,
            transaction_id,
            approver_person_id: approval.approver_person_id,
            approved_at: approval.approved_at,
            notes: approval.notes,
            created_at: chrono::Utc::now(),
        }
    }

    /// Map from database ApprovalModel to domain Approval
    pub fn from_model(model: ApprovalModel) -> banking_api::BankingResult<Approval> {
        Ok(Approval {
            id: model.id,
            approver_person_id: model.approver_person_id,
            approved_at: model.approved_at,
            notes: model.notes,
        })
    }
}