use banking_api::domain::{
    Account, AccountBalanceCalculation, AccountMandate, AccountOwnership, AccountRelationship,
    AccountStatusChangeRecord, UltimateBeneficiary, AccountType, AccountStatus, SigningCondition,
    DisbursementMethod, DisbursementStatus, OwnershipType, EntityType, RelationshipType,
    RelationshipStatus, PermissionType, MandateStatus, ControlType, VerificationStatus, UboStatus,
};
use banking_db::{
    DbAccountStatus, DbAccountType, DbControlType, DbDisbursementMethod, DbDisbursementStatus,
    DbEntityType, DbMandateStatus, DbOwnershipType, DbPermissionType, DbRelationshipStatus,
    DbRelationshipType, DbSigningCondition, DbUboStatus, DbVerificationStatus,
};
use banking_db::models::{
    AccountBalanceCalculationModel, AccountMandateModel, AccountModel, AccountOwnershipModel,
    AccountRelationshipModel, AccountStatusChangeRecordModel, UltimateBeneficiaryModel,
};
use heapless::{String as HeaplessString};

pub struct AccountMapper;

impl AccountMapper {
    /// Map from domain Account to database AccountModel
    pub fn to_model(account: Account) -> AccountModel {
        AccountModel {
            id: account.id,
            product_id: account.product_id,
            account_type: Self::account_type_to_db(account.account_type),
            account_status: Self::account_status_to_db(account.account_status),
            signing_condition: Self::signing_condition_to_db(account.signing_condition),
            currency: account.currency,
            open_date: account.open_date,
            domicile_agency_branch_id: account.domicile_agency_branch_id,
            current_balance: account.current_balance,
            available_balance: account.available_balance,
            accrued_interest: account.accrued_interest,
            overdraft_limit: account.overdraft_limit,
            original_principal: account.original_principal,
            outstanding_principal: account.outstanding_principal,
            loan_interest_rate: account.loan_interest_rate,
            loan_term_months: account.loan_term_months,
            disbursement_date: account.disbursement_date,
            maturity_date: account.maturity_date,
            installment_amount: account.installment_amount,
            next_due_date: account.next_due_date,
            penalty_rate: account.penalty_rate,
            collateral_id: account.collateral_id,
            loan_purpose_id: account.loan_purpose_id,
            gl_code_suffix: account.gl_code_suffix,
            // Enhanced lifecycle fields
            close_date: account.close_date,
            last_activity_date: account.last_activity_date,
            dormancy_threshold_days: account.dormancy_threshold_days,
            reactivation_required: account.reactivation_required,
            pending_closure_reason_id: account.pending_closure_reason_id,
            last_disbursement_instruction_id: account.last_disbursement_instruction_id,
            status_changed_by_person_id: account.status_changed_by_person_id,
            status_change_reason_id: account.status_change_reason_id,
            status_change_timestamp: account.status_change_timestamp,
            // Direct reference fields
            most_significant_account_hold_id: account.most_significant_account_hold_id,
            account_ownership_id: account.account_ownership_id,
            access01_account_relationship_id: account.access01_account_relationship_id,
            access02_account_relationship_id: account.access02_account_relationship_id,
            access03_account_relationship_id: account.access03_account_relationship_id,
            access04_account_relationship_id: account.access04_account_relationship_id,
            access05_account_relationship_id: account.access05_account_relationship_id,
            access06_account_relationship_id: account.access06_account_relationship_id,
            access07_account_relationship_id: account.access07_account_relationship_id,
            access11_account_mandate_id: account.access11_account_mandate_id,
            access12_account_mandate_id: account.access12_account_mandate_id,
            access13_account_mandate_id: account.access13_account_mandate_id,
            access14_account_mandate_id: account.access14_account_mandate_id,
            access15_account_mandate_id: account.access15_account_mandate_id,
            access16_account_mandate_id: account.access16_account_mandate_id,
            access17_account_mandate_id: account.access17_account_mandate_id,
            interest01_ultimate_beneficiary_id: account.interest01_ultimate_beneficiary_id,
            interest02_ultimate_beneficiary_id: account.interest02_ultimate_beneficiary_id,
            interest03_ultimate_beneficiary_id: account.interest03_ultimate_beneficiary_id,
            interest04_ultimate_beneficiary_id: account.interest04_ultimate_beneficiary_id,
            interest05_ultimate_beneficiary_id: account.interest05_ultimate_beneficiary_id,
            interest06_ultimate_beneficiary_id: account.interest06_ultimate_beneficiary_id,
            interest07_ultimate_beneficiary_id: account.interest07_ultimate_beneficiary_id,
            // Audit fields
            created_at: account.created_at,
            last_updated_at: account.last_updated_at,
            updated_by_person_id: account.updated_by_person_id,
        }
    }

    /// Map from database AccountModel to domain Account
    pub fn from_model(model: AccountModel) -> banking_api::BankingResult<Account> {
        Ok(Account {
            id: model.id,
            product_id: model.product_id,
            account_type: Self::account_type_from_db(model.account_type),
            account_status: Self::account_status_from_db(model.account_status),
            signing_condition: Self::signing_condition_from_db(model.signing_condition),
            currency: model.currency,
            open_date: model.open_date,
            domicile_agency_branch_id: model.domicile_agency_branch_id,
            current_balance: model.current_balance,
            available_balance: model.available_balance,
            accrued_interest: model.accrued_interest,
            overdraft_limit: model.overdraft_limit,
            original_principal: model.original_principal,
            outstanding_principal: model.outstanding_principal,
            loan_interest_rate: model.loan_interest_rate,
            loan_term_months: model.loan_term_months,
            disbursement_date: model.disbursement_date,
            maturity_date: model.maturity_date,
            installment_amount: model.installment_amount,
            next_due_date: model.next_due_date,
            penalty_rate: model.penalty_rate,
            collateral_id: model.collateral_id,
            loan_purpose_id: model.loan_purpose_id,
            gl_code_suffix: model.gl_code_suffix,
            // Enhanced lifecycle fields
            close_date: model.close_date,
            last_activity_date: model.last_activity_date,
            dormancy_threshold_days: model.dormancy_threshold_days,
            reactivation_required: model.reactivation_required,
            pending_closure_reason_id: model.pending_closure_reason_id,
            last_disbursement_instruction_id: model.last_disbursement_instruction_id,
            status_changed_by_person_id: model.status_changed_by_person_id,
            status_change_reason_id: model.status_change_reason_id,
            status_change_timestamp: model.status_change_timestamp,
            // Direct reference fields
            most_significant_account_hold_id: model.most_significant_account_hold_id,
            account_ownership_id: model.account_ownership_id,
            access01_account_relationship_id: model.access01_account_relationship_id,
            access02_account_relationship_id: model.access02_account_relationship_id,
            access03_account_relationship_id: model.access03_account_relationship_id,
            access04_account_relationship_id: model.access04_account_relationship_id,
            access05_account_relationship_id: model.access05_account_relationship_id,
            access06_account_relationship_id: model.access06_account_relationship_id,
            access07_account_relationship_id: model.access07_account_relationship_id,
            access11_account_mandate_id: model.access11_account_mandate_id,
            access12_account_mandate_id: model.access12_account_mandate_id,
            access13_account_mandate_id: model.access13_account_mandate_id,
            access14_account_mandate_id: model.access14_account_mandate_id,
            access15_account_mandate_id: model.access15_account_mandate_id,
            access16_account_mandate_id: model.access16_account_mandate_id,
            access17_account_mandate_id: model.access17_account_mandate_id,
            interest01_ultimate_beneficiary_id: model.interest01_ultimate_beneficiary_id,
            interest02_ultimate_beneficiary_id: model.interest02_ultimate_beneficiary_id,
            interest03_ultimate_beneficiary_id: model.interest03_ultimate_beneficiary_id,
            interest04_ultimate_beneficiary_id: model.interest04_ultimate_beneficiary_id,
            interest05_ultimate_beneficiary_id: model.interest05_ultimate_beneficiary_id,
            interest06_ultimate_beneficiary_id: model.interest06_ultimate_beneficiary_id,
            interest07_ultimate_beneficiary_id: model.interest07_ultimate_beneficiary_id,
            // Audit fields
            created_at: model.created_at,
            last_updated_at: model.last_updated_at,
            updated_by_person_id: model.updated_by_person_id,
        })
    }

    // Note: Disbursement instructions are now handled via last_disbursement_instruction_id reference

    // Account Ownership mappers
    pub fn account_ownership_to_model(ownership: AccountOwnership) -> AccountOwnershipModel {
        AccountOwnershipModel {
            id: ownership.id,
            account_id: ownership.account_id,
            customer_id: ownership.customer_id,
            ownership_type: Self::ownership_type_to_db(ownership.ownership_type),
            ownership_percentage: ownership.ownership_percentage,
            created_at: ownership.created_at,
        }
    }

    pub fn account_ownership_from_model(model: AccountOwnershipModel) -> AccountOwnership {
        AccountOwnership {
            id: model.id,
            account_id: model.account_id,
            customer_id: model.customer_id,
            ownership_type: Self::ownership_type_from_db(model.ownership_type),
            ownership_percentage: model.ownership_percentage,
            created_at: model.created_at,
        }
    }

    // Account Relationship mappers
    pub fn account_relationship_to_model(
        relationship: AccountRelationship,
    ) -> AccountRelationshipModel {
        AccountRelationshipModel {
            id: relationship.id,
            account_id: relationship.account_id,
            person_id: relationship.person_id,
            entity_type: Self::entity_type_to_db(relationship.entity_type),
            relationship_type: Self::relationship_type_to_db(relationship.relationship_type),
            status: Self::relationship_status_to_db(relationship.status),
            start_date: relationship.start_date,
            end_date: relationship.end_date,
        }
    }

    pub fn account_relationship_from_model(model: AccountRelationshipModel) -> AccountRelationship {
        AccountRelationship {
            id: model.id,
            account_id: model.account_id,
            person_id: model.person_id,
            entity_type: Self::entity_type_from_db(model.entity_type),
            relationship_type: Self::relationship_type_from_db(model.relationship_type),
            status: Self::relationship_status_from_db(model.status),
            start_date: model.start_date,
            end_date: model.end_date,
        }
    }

    // Account Mandate mappers
    pub fn account_mandate_to_model(mandate: AccountMandate) -> AccountMandateModel {
        AccountMandateModel {
            id: mandate.id,
            account_id: mandate.account_id,
            grantee_customer_id: mandate.grantee_customer_id,
            permission_type: Self::permission_type_to_db(mandate.permission_type),
            transaction_limit: mandate.transaction_limit,
            approver01_person_id: mandate.approver01_person_id,
            approver02_person_id: mandate.approver02_person_id,
            approver03_person_id: mandate.approver03_person_id,
            approver04_person_id: mandate.approver04_person_id,
            approver05_person_id: mandate.approver05_person_id,
            approver06_person_id: mandate.approver06_person_id,
            approver07_person_id: mandate.approver07_person_id,
            required_signers_count: mandate.required_signers_count,
            conditional_mandate_id: mandate.conditional_mandate_id,
            status: Self::mandate_status_to_db(mandate.status),
            start_date: mandate.start_date,
            end_date: mandate.end_date,
        }
    }

    pub fn account_mandate_from_model(model: AccountMandateModel) -> AccountMandate {
        AccountMandate {
            id: model.id,
            account_id: model.account_id,
            grantee_customer_id: model.grantee_customer_id,
            permission_type: Self::permission_type_from_db(model.permission_type),
            transaction_limit: model.transaction_limit,
            approver01_person_id: model.approver01_person_id,
            approver02_person_id: model.approver02_person_id,
            approver03_person_id: model.approver03_person_id,
            approver04_person_id: model.approver04_person_id,
            approver05_person_id: model.approver05_person_id,
            approver06_person_id: model.approver06_person_id,
            approver07_person_id: model.approver07_person_id,
            required_signers_count: model.required_signers_count,
            conditional_mandate_id: model.conditional_mandate_id,
            status: Self::mandate_status_from_db(model.status),
            start_date: model.start_date,
            end_date: model.end_date,
        }
    }

    // Ultimate Beneficial Owner mappers
    pub fn ultimate_beneficiary_to_model(ubo: UltimateBeneficiary) -> UltimateBeneficiaryModel {
        UltimateBeneficiaryModel {
            id: ubo.id,
            corporate_customer_id: ubo.corporate_customer_id,
            beneficiary_customer_id: ubo.beneficiary_customer_id,
            ownership_percentage: ubo.ownership_percentage,
            control_type: Self::control_type_to_db(ubo.control_type),
            description: ubo.description.map(|desc| {
                HeaplessString::try_from(desc.as_str()).unwrap_or_else(|_| HeaplessString::new())
            }),
            status: Self::ubo_status_to_db(ubo.status),
            verification_status: Self::verification_status_to_db(ubo.verification_status),
            created_at: ubo.created_at,
        }
    }

    pub fn ultimate_beneficiary_from_model(model: UltimateBeneficiaryModel) -> UltimateBeneficiary {
        UltimateBeneficiary {
            id: model.id,
            corporate_customer_id: model.corporate_customer_id,
            beneficiary_customer_id: model.beneficiary_customer_id,
            ownership_percentage: model.ownership_percentage,
            control_type: Self::control_type_from_db(model.control_type),
            description: model.description.map(|desc| {
                HeaplessString::try_from(desc.as_str()).unwrap_or_else(|_| HeaplessString::new())
            }),
            status: Self::ubo_status_from_db(model.status),
            verification_status: Self::verification_status_from_db(model.verification_status),
            created_at: model.created_at,
        }
    }


    // Status Change Record mappers
    pub fn status_change_record_to_model(
        record: AccountStatusChangeRecord,
    ) -> AccountStatusChangeRecordModel {
        AccountStatusChangeRecordModel {
            id: record.id,
            account_id: record.account_id,
            old_status: record.old_status.map(Self::account_status_to_db),
            new_status: Self::account_status_to_db(record.new_status),
            reason_id: record.reason_id,
            additional_context: record.additional_context,
            changed_by_person_id: record.changed_by_person_id,
            changed_at: record.changed_at,
            system_triggered: record.system_triggered,
            created_at: record.created_at,
        }
    }

    pub fn status_change_record_from_model(
        model: AccountStatusChangeRecordModel,
    ) -> AccountStatusChangeRecord {
        AccountStatusChangeRecord {
            id: model.id,
            account_id: model.account_id,
            old_status: model.old_status.map(Self::account_status_from_db),
            new_status: Self::account_status_from_db(model.new_status),
            reason_id: model.reason_id,
            additional_context: model.additional_context,
            changed_by_person_id: model.changed_by_person_id,
            changed_at: model.changed_at,
            system_triggered: model.system_triggered,
            created_at: model.created_at,
        }
    }

    // AccountBalanceCalculation mappers
    pub fn balance_calculation_to_model(
        calc: AccountBalanceCalculation,
    ) -> AccountBalanceCalculationModel {
        AccountBalanceCalculationModel {
            id: calc.id,
            account_id: calc.account_id,
            current_balance: calc.current_balance,
            available_balance: calc.available_balance,
            overdraft_limit: calc.overdraft_limit,
            total_holds: calc.total_holds,
            active_hold_count: calc.active_hold_count as i32,
            calculation_timestamp: calc.calculation_timestamp,
        }
    }

    pub fn balance_calculation_from_model(
        model: AccountBalanceCalculationModel,
    ) -> AccountBalanceCalculation {
        AccountBalanceCalculation {
            id: model.id,
            account_id: model.account_id,
            current_balance: model.current_balance,
            available_balance: model.available_balance,
            overdraft_limit: model.overdraft_limit,
            total_holds: model.total_holds,
            active_hold_count: model.active_hold_count as u32,
            calculation_timestamp: model.calculation_timestamp,
        }
    }

    // Helper methods for enum conversions
    fn account_type_to_db(account_type: AccountType) -> DbAccountType {
        match account_type {
            AccountType::Savings => DbAccountType::Savings,
            AccountType::Current => DbAccountType::Current,
            AccountType::Loan => DbAccountType::Loan,
        }
    }

    fn account_type_from_db(db_type: DbAccountType) -> AccountType {
        match db_type {
            DbAccountType::Savings => AccountType::Savings,
            DbAccountType::Current => AccountType::Current,
            DbAccountType::Loan => AccountType::Loan,
        }
    }

    fn account_status_to_db(account_status: AccountStatus) -> DbAccountStatus {
        match account_status {
            AccountStatus::PendingApproval => DbAccountStatus::PendingApproval,
            AccountStatus::Active => DbAccountStatus::Active,
            AccountStatus::Dormant => DbAccountStatus::Dormant,
            AccountStatus::Frozen => DbAccountStatus::Frozen,
            AccountStatus::PendingClosure => DbAccountStatus::PendingClosure,
            AccountStatus::Closed => DbAccountStatus::Closed,
            AccountStatus::PendingReactivation => DbAccountStatus::PendingReactivation,
        }
    }

    fn account_status_from_db(db_status: DbAccountStatus) -> AccountStatus {
        match db_status {
            DbAccountStatus::PendingApproval => AccountStatus::PendingApproval,
            DbAccountStatus::Active => AccountStatus::Active,
            DbAccountStatus::Dormant => AccountStatus::Dormant,
            DbAccountStatus::Frozen => AccountStatus::Frozen,
            DbAccountStatus::PendingClosure => AccountStatus::PendingClosure,
            DbAccountStatus::Closed => AccountStatus::Closed,
            DbAccountStatus::PendingReactivation => AccountStatus::PendingReactivation,
        }
    }

    fn signing_condition_to_db(signing_condition: SigningCondition) -> DbSigningCondition {
        match signing_condition {
            SigningCondition::None => DbSigningCondition::None,
            SigningCondition::AnyOwner => DbSigningCondition::AnyOwner,
            SigningCondition::AllOwners => DbSigningCondition::AllOwners,
        }
    }

    fn signing_condition_from_db(db_condition: DbSigningCondition) -> SigningCondition {
        match db_condition {
            DbSigningCondition::None => SigningCondition::None,
            DbSigningCondition::AllOwners => SigningCondition::AllOwners,
            DbSigningCondition::AnyOwner => SigningCondition::AnyOwner,
        }
    }

    #[allow(dead_code)]
    fn disbursement_method_to_db(method: DisbursementMethod) -> DbDisbursementMethod {
        match method {
            DisbursementMethod::Transfer => DbDisbursementMethod::Transfer,
            DisbursementMethod::CashWithdrawal => DbDisbursementMethod::CashWithdrawal,
            DisbursementMethod::Check => DbDisbursementMethod::Check,
            DisbursementMethod::HoldFunds => DbDisbursementMethod::HoldFunds,
            DisbursementMethod::OverdraftFacility => DbDisbursementMethod::OverdraftFacility,
            DisbursementMethod::StagedRelease => DbDisbursementMethod::StagedRelease,
        }
    }

    #[allow(dead_code)]
    fn disbursement_method_from_db(db_method: DbDisbursementMethod) -> DisbursementMethod {
        match db_method {
            DbDisbursementMethod::Transfer => DisbursementMethod::Transfer,
            DbDisbursementMethod::CashWithdrawal => DisbursementMethod::CashWithdrawal,
            DbDisbursementMethod::Check => DisbursementMethod::Check,
            DbDisbursementMethod::HoldFunds => DisbursementMethod::HoldFunds,
            DbDisbursementMethod::OverdraftFacility => DisbursementMethod::OverdraftFacility,
            DbDisbursementMethod::StagedRelease => DisbursementMethod::StagedRelease,
        }
    }

    #[allow(dead_code)]
    fn disbursement_status_to_db(status: DisbursementStatus) -> DbDisbursementStatus {
        match status {
            DisbursementStatus::Pending => DbDisbursementStatus::Pending,
            DisbursementStatus::Approved => DbDisbursementStatus::Approved,
            DisbursementStatus::Executed => DbDisbursementStatus::Executed,
            DisbursementStatus::Cancelled => DbDisbursementStatus::Cancelled,
            DisbursementStatus::Failed => DbDisbursementStatus::Failed,
            DisbursementStatus::PartiallyExecuted => DbDisbursementStatus::PartiallyExecuted,
        }
    }

    #[allow(dead_code)]
    fn disbursement_status_from_db(db_status: DbDisbursementStatus) -> DisbursementStatus {
        match db_status {
            DbDisbursementStatus::Pending => DisbursementStatus::Pending,
            DbDisbursementStatus::Approved => DisbursementStatus::Approved,
            DbDisbursementStatus::Executed => DisbursementStatus::Executed,
            DbDisbursementStatus::Cancelled => DisbursementStatus::Cancelled,
            DbDisbursementStatus::Failed => DisbursementStatus::Failed,
            DbDisbursementStatus::PartiallyExecuted => DisbursementStatus::PartiallyExecuted,
        }
    }

    fn ownership_type_to_db(ownership_type: OwnershipType) -> DbOwnershipType {
        match ownership_type {
            OwnershipType::Single => DbOwnershipType::Single,
            OwnershipType::Joint => DbOwnershipType::Joint,
            OwnershipType::Corporate => DbOwnershipType::Corporate,
        }
    }

    fn ownership_type_from_db(db_type: DbOwnershipType) -> OwnershipType {
        match db_type {
            DbOwnershipType::Single => OwnershipType::Single,
            DbOwnershipType::Joint => OwnershipType::Joint,
            DbOwnershipType::Corporate => OwnershipType::Corporate,
        }
    }

    fn entity_type_to_db(entity_type: EntityType) -> DbEntityType {
        match entity_type {
            EntityType::Branch => DbEntityType::Branch,
            EntityType::Agent => DbEntityType::Agent,
            EntityType::RiskManager => DbEntityType::RiskManager,
            EntityType::ComplianceOfficer => DbEntityType::ComplianceOfficer,
            EntityType::CustomerService => DbEntityType::CustomerService,
        }
    }

    fn entity_type_from_db(db_type: DbEntityType) -> EntityType {
        match db_type {
            DbEntityType::Branch => EntityType::Branch,
            DbEntityType::Agent => EntityType::Agent,
            DbEntityType::RiskManager => EntityType::RiskManager,
            DbEntityType::ComplianceOfficer => EntityType::ComplianceOfficer,
            DbEntityType::CustomerService => EntityType::CustomerService,
        }
    }

    fn relationship_type_to_db(relationship_type: RelationshipType) -> DbRelationshipType {
        match relationship_type {
            RelationshipType::PrimaryHandler => DbRelationshipType::PrimaryHandler,
            RelationshipType::BackupHandler => DbRelationshipType::BackupHandler,
            RelationshipType::RiskOversight => DbRelationshipType::RiskOversight,
            RelationshipType::ComplianceOversight => DbRelationshipType::ComplianceOversight,
            RelationshipType::Accountant => DbRelationshipType::Accountant,
        }
    }

    fn relationship_type_from_db(db_type: DbRelationshipType) -> RelationshipType {
        match db_type {
            DbRelationshipType::PrimaryHandler => RelationshipType::PrimaryHandler,
            DbRelationshipType::BackupHandler => RelationshipType::BackupHandler,
            DbRelationshipType::RiskOversight => RelationshipType::RiskOversight,
            DbRelationshipType::ComplianceOversight => RelationshipType::ComplianceOversight,
            DbRelationshipType::Accountant => RelationshipType::Accountant,
        }
    }

    fn relationship_status_to_db(status: RelationshipStatus) -> DbRelationshipStatus {
        match status {
            RelationshipStatus::Active => DbRelationshipStatus::Active,
            RelationshipStatus::Inactive => DbRelationshipStatus::Inactive,
            RelationshipStatus::Suspended => DbRelationshipStatus::Suspended,
        }
    }

    fn relationship_status_from_db(db_status: DbRelationshipStatus) -> RelationshipStatus {
        match db_status {
            DbRelationshipStatus::Active => RelationshipStatus::Active,
            DbRelationshipStatus::Inactive => RelationshipStatus::Inactive,
            DbRelationshipStatus::Suspended => RelationshipStatus::Suspended,
        }
    }

    fn permission_type_to_db(permission_type: PermissionType) -> DbPermissionType {
        match permission_type {
            PermissionType::ViewOnly => DbPermissionType::ViewOnly,
            PermissionType::LimitedWithdrawal => DbPermissionType::LimitedWithdrawal,
            PermissionType::JointApproval => DbPermissionType::JointApproval,
            PermissionType::FullAccess => DbPermissionType::FullAccess,
        }
    }

    fn permission_type_from_db(db_type: DbPermissionType) -> PermissionType {
        match db_type {
            DbPermissionType::ViewOnly => PermissionType::ViewOnly,
            DbPermissionType::LimitedWithdrawal => PermissionType::LimitedWithdrawal,
            DbPermissionType::JointApproval => PermissionType::JointApproval,
            DbPermissionType::FullAccess => PermissionType::FullAccess,
        }
    }

    fn mandate_status_to_db(status: MandateStatus) -> DbMandateStatus {
        match status {
            MandateStatus::Active => DbMandateStatus::Active,
            MandateStatus::Suspended => DbMandateStatus::Suspended,
            MandateStatus::Revoked => DbMandateStatus::Revoked,
            MandateStatus::Expired => DbMandateStatus::Expired,
        }
    }

    fn mandate_status_from_db(db_status: DbMandateStatus) -> MandateStatus {
        match db_status {
            DbMandateStatus::Active => MandateStatus::Active,
            DbMandateStatus::Suspended => MandateStatus::Suspended,
            DbMandateStatus::Revoked => MandateStatus::Revoked,
            DbMandateStatus::Expired => MandateStatus::Expired,
        }
    }

    fn control_type_to_db(control_type: ControlType) -> DbControlType {
        match control_type {
            ControlType::DirectOwnership => DbControlType::DirectOwnership,
            ControlType::IndirectOwnership => DbControlType::IndirectOwnership,
            ControlType::SignificantInfluence => DbControlType::SignificantInfluence,
            ControlType::SeniorManagement => DbControlType::SeniorManagement,
        }
    }

    fn control_type_from_db(db_type: DbControlType) -> ControlType {
        match db_type {
            DbControlType::DirectOwnership => ControlType::DirectOwnership,
            DbControlType::IndirectOwnership => ControlType::IndirectOwnership,
            DbControlType::SignificantInfluence => ControlType::SignificantInfluence,
            DbControlType::SeniorManagement => ControlType::SeniorManagement,
        }
    }

    fn verification_status_to_db(status: VerificationStatus) -> DbVerificationStatus {
        match status {
            VerificationStatus::Pending => DbVerificationStatus::Pending,
            VerificationStatus::Verified => DbVerificationStatus::Verified,
            VerificationStatus::Rejected => DbVerificationStatus::Rejected,
            VerificationStatus::RequiresUpdate => DbVerificationStatus::RequiresUpdate,
        }
    }

    fn verification_status_from_db(db_status: DbVerificationStatus) -> VerificationStatus {
        match db_status {
            DbVerificationStatus::Pending => VerificationStatus::Pending,
            DbVerificationStatus::Verified => VerificationStatus::Verified,
            DbVerificationStatus::Rejected => VerificationStatus::Rejected,
            DbVerificationStatus::RequiresUpdate => VerificationStatus::RequiresUpdate,
        }
    }

    fn ubo_status_to_db(status: UboStatus) -> DbUboStatus {
        match status {
            UboStatus::Active => DbUboStatus::Active,
            UboStatus::Inactive => DbUboStatus::Inactive,
            UboStatus::UnderReview => DbUboStatus::UnderReview,
        }
    }

    fn ubo_status_from_db(db_status: DbUboStatus) -> UboStatus {
        match db_status {
            DbUboStatus::Active => UboStatus::Active,
            DbUboStatus::Inactive => UboStatus::Inactive,
            DbUboStatus::UnderReview => UboStatus::UnderReview,
        }
    }
}