use banking_api::domain::{
    self as domain, GlEntry, Transaction, TransactionAudit, TransactionRequest,
    TransactionResult, TransactionValidationResult, TransactionType as ApiTransactionType,
};
use banking_db::models::{
    self as db, GlEntryModel, TransactionAuditModel, TransactionModel, TransactionRequestModel,
    TransactionResultModel, TransactionValidationResultModel, TransactionType as DbTransactionType,
};

pub struct TransactionMapper;

impl TransactionMapper {
    /// Map from domain Transaction to database TransactionModel
    pub fn to_model(transaction: Transaction) -> TransactionModel {
        TransactionModel {
            id: transaction.id,
            account_id: transaction.account_id,
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
            approval_status: transaction.approval_status.map(Self::transaction_approval_status_to_db),
            risk_score: transaction.risk_score,
            created_at: transaction.created_at,
        }
    }

    /// Map from database TransactionModel to domain Transaction
    pub fn from_model(model: TransactionModel) -> banking_api::BankingResult<Transaction> {
        Ok(Transaction {
            id: model.id,
            account_id: model.account_id,
            transaction_code: model.transaction_code,
            transaction_type: Self::transaction_type_from_db(model.transaction_type),
            amount: model.amount,
            currency: model.currency,
            description: model.description,
            channel_id: model.channel_id,
            terminal_id: model.terminal_id,
            agent_person_id: model.agent_person_id,
            transaction_date: model.transaction_date,
            value_date: model.value_date,
            status: Self::transaction_status_from_db(model.status),
            reference_number: model.reference_number,
            external_reference: model.external_reference,
            gl_code: model.gl_code,
            requires_approval: model.requires_approval,
            approval_status: model.approval_status.map(Self::transaction_approval_status_from_db),
            risk_score: model.risk_score,
            created_at: model.created_at,
        })
    }

    // Helper methods for enum conversions
    pub fn transaction_type_to_db(t: ApiTransactionType) -> DbTransactionType {
        match t {
            ApiTransactionType::Credit => DbTransactionType::Credit,
            ApiTransactionType::Debit => DbTransactionType::Debit,
        }
    }

    pub fn transaction_type_from_db(t: DbTransactionType) -> ApiTransactionType {
        match t {
            DbTransactionType::Credit => ApiTransactionType::Credit,
            DbTransactionType::Debit => ApiTransactionType::Debit,
        }
    }

    pub fn transaction_status_to_db(t: domain::TransactionStatus) -> db::TransactionStatus {
        match t {
            domain::TransactionStatus::Pending => db::TransactionStatus::Pending,
            domain::TransactionStatus::Posted => db::TransactionStatus::Posted,
            domain::TransactionStatus::Reversed => db::TransactionStatus::Reversed,
            domain::TransactionStatus::Failed => db::TransactionStatus::Failed,
            domain::TransactionStatus::AwaitingApproval => db::TransactionStatus::AwaitingApproval,
            domain::TransactionStatus::ApprovalRejected => db::TransactionStatus::ApprovalRejected,
        }
    }

    pub fn transaction_status_from_db(t: db::TransactionStatus) -> domain::TransactionStatus {
        match t {
            db::TransactionStatus::Pending => domain::TransactionStatus::Pending,
            db::TransactionStatus::Posted => domain::TransactionStatus::Posted,
            db::TransactionStatus::Reversed => domain::TransactionStatus::Reversed,
            db::TransactionStatus::Failed => domain::TransactionStatus::Failed,
            db::TransactionStatus::AwaitingApproval => domain::TransactionStatus::AwaitingApproval,
            db::TransactionStatus::ApprovalRejected => domain::TransactionStatus::ApprovalRejected,
        }
    }

    pub fn transaction_approval_status_to_db(t: domain::TransactionApprovalStatus) -> db::TransactionApprovalStatus {
        match t {
            domain::TransactionApprovalStatus::Pending => db::TransactionApprovalStatus::Pending,
            domain::TransactionApprovalStatus::Approved => db::TransactionApprovalStatus::Approved,
            domain::TransactionApprovalStatus::Rejected => db::TransactionApprovalStatus::Rejected,
            domain::TransactionApprovalStatus::PartiallyApproved => {
                db::TransactionApprovalStatus::PartiallyApproved
            }
        }
    }

    pub fn transaction_approval_status_from_db(t: db::TransactionApprovalStatus) -> domain::TransactionApprovalStatus {
        match t {
            db::TransactionApprovalStatus::Pending => domain::TransactionApprovalStatus::Pending,
            db::TransactionApprovalStatus::Approved => domain::TransactionApprovalStatus::Approved,
            db::TransactionApprovalStatus::Rejected => domain::TransactionApprovalStatus::Rejected,
            db::TransactionApprovalStatus::PartiallyApproved => {
                domain::TransactionApprovalStatus::PartiallyApproved
            }
        }
    }
}

pub struct TransactionAuditMapper;

impl TransactionAuditMapper {
    /// Map from domain TransactionAudit to database TransactionAuditModel
    pub fn to_model(audit: TransactionAudit) -> TransactionAuditModel {
        TransactionAuditModel {
            id: audit.id,
            transaction_id: audit.transaction_id,
            action_type: Self::transaction_audit_action_to_db(audit.action_type),
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
            transaction_id: model.transaction_id,
            action_type: Self::transaction_audit_action_from_db(model.action_type),
            performed_by_person_id: model.performed_by_person_id,
            performed_at: model.performed_at,
            old_status: model.old_status.map(TransactionMapper::transaction_status_from_db),
            new_status: model.new_status.map(TransactionMapper::transaction_status_from_db),
            reason_id: model.reason_id,
            details: model.details,
        })
    }

    pub fn transaction_audit_action_to_db(t: domain::TransactionAuditAction) -> db::TransactionAuditAction {
        match t {
            domain::TransactionAuditAction::Created => db::TransactionAuditAction::Created,
            domain::TransactionAuditAction::StatusChanged => {
                db::TransactionAuditAction::StatusChanged
            }
            domain::TransactionAuditAction::Posted => db::TransactionAuditAction::Posted,
            domain::TransactionAuditAction::Reversed => db::TransactionAuditAction::Reversed,
            domain::TransactionAuditAction::Failed => db::TransactionAuditAction::Failed,
            domain::TransactionAuditAction::Approved => db::TransactionAuditAction::Approved,
            domain::TransactionAuditAction::Rejected => db::TransactionAuditAction::Rejected,
        }
    }

    pub fn transaction_audit_action_from_db(t: db::TransactionAuditAction) -> domain::TransactionAuditAction {
        match t {
            db::TransactionAuditAction::Created => domain::TransactionAuditAction::Created,
            db::TransactionAuditAction::StatusChanged => {
                domain::TransactionAuditAction::StatusChanged
            }
            db::TransactionAuditAction::Posted => domain::TransactionAuditAction::Posted,
            db::TransactionAuditAction::Reversed => domain::TransactionAuditAction::Reversed,
            db::TransactionAuditAction::Failed => domain::TransactionAuditAction::Failed,
            db::TransactionAuditAction::Approved => domain::TransactionAuditAction::Approved,
            db::TransactionAuditAction::Rejected => domain::TransactionAuditAction::Rejected,
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
            created_at: model.created_at
        })
    }
}

pub struct TransactionRequestMapper;

impl TransactionRequestMapper {
    /// Map from domain TransactionRequest to database TransactionRequestModel
    pub fn to_model(request: TransactionRequest) -> TransactionRequestModel {
        TransactionRequestModel {
            id: request.id,
            account_id: request.account_id,
            transaction_type: TransactionMapper::transaction_type_to_db(request.transaction_type),
            amount: request.amount,
            currency: request.currency,
            description: request.description,
            channel: Self::channel_type_to_db(request.channel),
            terminal_id: request.terminal_id,
            initiator_person_id: request.initiator_person_id,
            external_reference: request.external_reference,
            created_at: request.created_at,
        }
    }

    /// Map from database TransactionRequestModel to domain TransactionRequest
    pub fn from_model(model: TransactionRequestModel) -> banking_api::BankingResult<TransactionRequest> {
        Ok(TransactionRequest {
            id: model.id,
            account_id: model.account_id,
            transaction_type: TransactionMapper::transaction_type_from_db(model.transaction_type),
            amount: model.amount,
            currency: model.currency,
            description: model.description,
            channel: Self::channel_type_from_db(model.channel),
            terminal_id: model.terminal_id,
            initiator_person_id: model.initiator_person_id,
            external_reference: model.external_reference,
            created_at: model.created_at,
        })
    }

    pub fn channel_type_to_db(t: domain::ChannelType) -> db::ChannelType {
        match t {
            domain::ChannelType::MobileApp => db::ChannelType::MobileApp,
            domain::ChannelType::AgentTerminal => db::ChannelType::AgentTerminal,
            domain::ChannelType::ATM => db::ChannelType::ATM,
            domain::ChannelType::InternetBanking => db::ChannelType::InternetBanking,
            domain::ChannelType::BranchTeller => db::ChannelType::BranchTeller,
            domain::ChannelType::USSD => db::ChannelType::USSD,
            domain::ChannelType::ApiGateway => db::ChannelType::ApiGateway,
        }
    }

    pub fn channel_type_from_db(t: db::ChannelType) -> domain::ChannelType {
        match t {
            db::ChannelType::MobileApp => domain::ChannelType::MobileApp,
            db::ChannelType::AgentTerminal => domain::ChannelType::AgentTerminal,
            db::ChannelType::ATM => domain::ChannelType::ATM,
            db::ChannelType::InternetBanking => domain::ChannelType::InternetBanking,
            db::ChannelType::BranchTeller => domain::ChannelType::BranchTeller,
            db::ChannelType::USSD => domain::ChannelType::USSD,
            db::ChannelType::ApiGateway => domain::ChannelType::ApiGateway,
        }
    }
}

pub struct TransactionResultMapper;

impl TransactionResultMapper {
    /// Map from domain TransactionResult to database TransactionResultModel
    pub fn to_model(result: TransactionResult) -> TransactionResultModel {
        TransactionResultModel {
            id: result.id,
            transaction_id: result.transaction_id,
            reference_number: result.reference_number,
            timestamp: result.timestamp,
            created_at: result.created_at,
        }
    }

    /// Map from database TransactionResultModel to domain TransactionResult
    pub fn from_model(model: TransactionResultModel) -> banking_api::BankingResult<TransactionResult> {
        Ok(TransactionResult {
            id: model.id,
            transaction_id: model.transaction_id,
            reference_number: model.reference_number,
            timestamp: model.timestamp,
            created_at: model.created_at,
        })
    }
}

pub struct TransactionValidationResultMapper;

impl TransactionValidationResultMapper {
    /// Map from domain TransactionValidationResult to database TransactionValidationResultModel
    pub fn to_model(
        validation: TransactionValidationResult,
    ) -> TransactionValidationResultModel {
        TransactionValidationResultModel {
            id: validation.id,
            is_valid: validation.is_valid,
            transaction_id: validation.transaction_id,
            validation_error_01_field: validation.validation_error_01_field,
            validation_error_01_message: validation.validation_error_01_message,
            validation_error_01_error_code: validation.validation_error_01_error_code,
            validation_error_02_field: validation.validation_error_02_field,
            validation_error_02_message: validation.validation_error_02_message,
            validation_error_02_error_code: validation.validation_error_02_error_code,
            validation_error_03_field: validation.validation_error_03_field,
            validation_error_03_message: validation.validation_error_03_message,
            validation_error_03_error_code: validation.validation_error_03_error_code,
            warning_01: validation.warning_01,
            warning_02: validation.warning_02,
            warning_03: validation.warning_03,
            created_at: validation.created_at,
        }
    }

    /// Map from database ValidationResultModel to domain ValidationResult
    pub fn from_model(
        model: TransactionValidationResultModel,
    ) -> banking_api::BankingResult<TransactionValidationResult> {
        Ok(TransactionValidationResult {
            id: model.id,
            is_valid: model.is_valid,
            transaction_id: model.transaction_id,
            validation_error_01_field: model.validation_error_01_field,
            validation_error_01_message: model.validation_error_01_message,
            validation_error_01_error_code: model.validation_error_01_error_code,
            validation_error_02_field: model.validation_error_02_field,
            validation_error_02_message: model.validation_error_02_message,
            validation_error_02_error_code: model.validation_error_02_error_code,
            validation_error_03_field: model.validation_error_03_field,
            validation_error_03_message: model.validation_error_03_message,
            validation_error_03_error_code: model.validation_error_03_error_code,
            warning_01: model.warning_01,
            warning_02: model.warning_02,
            warning_03: model.warning_03,
            created_at: model.created_at,
        })
    }
}

pub struct TransactionApprovalMapper;

impl TransactionApprovalMapper {
    /// Map from domain TransactionApproval to database TransactionApprovalModel
    pub fn to_model(approval: domain::TransactionApproval) -> db::TransactionApprovalModel {
        db::TransactionApprovalModel {
            id: approval.id,
            transaction_id: approval.transaction_id,
            required: approval.required,
            approver_person_id: approval.approver_person_id,
            approval_status: TransactionMapper::transaction_approval_status_to_db(approval.approval_status),
            approved_at: approval.approved_at,
            notes: approval.notes,
            created_at: approval.created_at,
        }
    }

    /// Map from database TransactionApprovalModel to domain TransactionApproval
    pub fn from_model(
        model: db::TransactionApprovalModel,
    ) -> banking_api::BankingResult<domain::TransactionApproval> {
        Ok(domain::TransactionApproval {
            id: model.id,
            transaction_id: model.transaction_id,
            required: model.required,
            approver_person_id: model.approver_person_id,
            approval_status: TransactionMapper::transaction_approval_status_from_db(model.approval_status),
            approved_at: model.approved_at,
            notes: model.notes,
            created_at: model.created_at,
        })
    }
}