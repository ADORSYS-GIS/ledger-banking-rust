-- Initial schema for ledger-banking-rust
-- Core banking system tables with comprehensive constraints and business rules

-- =============================================================================
-- CUSTOMER INFORMATION FILE (CIF) - Master Repository
-- =============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Main customers table - Single source of truth for customer data
CREATE TABLE customers (
    customer_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_type VARCHAR(20) NOT NULL CHECK (customer_type IN ('Individual', 'Corporate')),
    full_name VARCHAR(255) NOT NULL,
    id_type VARCHAR(30) NOT NULL CHECK (id_type IN ('NationalId', 'Passport', 'CompanyRegistration')),
    id_number VARCHAR(50) NOT NULL,
    risk_rating VARCHAR(20) NOT NULL DEFAULT 'Low' CHECK (risk_rating IN ('Low', 'Medium', 'High', 'Blacklisted')),
    status VARCHAR(30) NOT NULL DEFAULT 'PendingVerification' CHECK (status IN ('Active', 'PendingVerification', 'Deceased', 'Dissolved', 'Blacklisted')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(100) NOT NULL,
    
    -- Business constraints
    CONSTRAINT uk_customer_identity UNIQUE (id_type, id_number),
    CONSTRAINT ck_customer_name_length CHECK (LENGTH(TRIM(full_name)) >= 2),
    CONSTRAINT ck_id_number_length CHECK (LENGTH(TRIM(id_number)) >= 3)
);

-- Customer documents for KYC compliance
CREATE TABLE customer_documents (
    document_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    document_type VARCHAR(50) NOT NULL,
    document_path VARCHAR(500),
    status VARCHAR(20) NOT NULL DEFAULT 'Uploaded' CHECK (status IN ('Uploaded', 'Verified', 'Rejected', 'Expired')),
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    uploaded_by VARCHAR(100) NOT NULL,
    verified_at TIMESTAMP WITH TIME ZONE,
    verified_by VARCHAR(100),
    
    CONSTRAINT ck_document_verification CHECK (
        (status IN ('Verified', 'Rejected') AND verified_at IS NOT NULL AND verified_by IS NOT NULL) OR
        (status IN ('Uploaded', 'Expired') AND verified_at IS NULL)
    )
);

-- Immutable audit trail for customer changes
CREATE TABLE customer_audit_trail (
    audit_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    field_name VARCHAR(50) NOT NULL,
    old_value TEXT,
    new_value TEXT,
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    changed_by VARCHAR(100) NOT NULL,
    reason VARCHAR(255)
);

-- =============================================================================
-- UNIFIED ACCOUNT MODEL (UAM) - Single Flexible Table
-- =============================================================================

-- Single table for all account types with dynamic behavior via product_code
CREATE TABLE accounts (
    account_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_code VARCHAR(50) NOT NULL, -- Foreign key to Product Catalog
    account_type VARCHAR(20) NOT NULL CHECK (account_type IN ('Savings', 'Current', 'Loan')),
    account_status VARCHAR(30) NOT NULL DEFAULT 'PendingApproval' CHECK (
        account_status IN ('PendingApproval', 'Active', 'Dormant', 'Frozen', 'PendingClosure', 'Closed', 'PendingReactivation')
    ),
    signing_condition VARCHAR(20) NOT NULL DEFAULT 'None' CHECK (signing_condition IN ('None', 'AnyOwner', 'AllOwners')),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    open_date DATE NOT NULL,
    domicile_branch_id UUID NOT NULL,
    
    -- Balance fields
    current_balance DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    available_balance DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    accrued_interest DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    overdraft_limit DECIMAL(15,2),
    
    -- Loan-specific fields (nullable for non-loan accounts)
    original_principal DECIMAL(15,2),
    outstanding_principal DECIMAL(15,2),
    loan_interest_rate DECIMAL(5,4),
    loan_term_months INTEGER,
    disbursement_date DATE,
    maturity_date DATE,
    installment_amount DECIMAL(15,2),
    next_due_date DATE,
    penalty_rate DECIMAL(5,4),
    collateral_id VARCHAR(100),
    loan_purpose VARCHAR(255),
    
    -- Enhanced lifecycle fields from enhancements
    close_date DATE,
    last_activity_date DATE,
    dormancy_threshold_days INTEGER,
    reactivation_required BOOLEAN DEFAULT FALSE,
    pending_closure_reason VARCHAR(255),
    status_changed_by VARCHAR(100),
    status_change_reason VARCHAR(255),
    status_change_timestamp TIMESTAMP WITH TIME ZONE,
    
    -- Audit fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by VARCHAR(100) NOT NULL,
    
    -- Business constraints
    CONSTRAINT ck_balance_consistency CHECK (available_balance <= current_balance + COALESCE(overdraft_limit, 0)),
    CONSTRAINT ck_loan_fields CHECK (
        (account_type = 'Loan' AND original_principal IS NOT NULL AND outstanding_principal IS NOT NULL) OR
        (account_type != 'Loan' AND original_principal IS NULL AND outstanding_principal IS NULL)
    ),
    CONSTRAINT ck_closure_date CHECK (
        (account_status = 'Closed' AND close_date IS NOT NULL) OR
        (account_status != 'Closed' AND close_date IS NULL)
    ),
    CONSTRAINT ck_overdraft_positive CHECK (overdraft_limit IS NULL OR overdraft_limit >= 0),
    CONSTRAINT ck_loan_amounts CHECK (
        outstanding_principal IS NULL OR 
        (outstanding_principal >= 0 AND outstanding_principal <= original_principal)
    )
);

-- Account ownership - Legal title/ownership relationships
CREATE TABLE account_ownership (
    ownership_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    ownership_type VARCHAR(20) NOT NULL CHECK (ownership_type IN ('Single', 'Joint', 'Corporate')),
    ownership_percentage DECIMAL(5,2),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_account_customer UNIQUE (account_id, customer_id),
    CONSTRAINT ck_ownership_percentage CHECK (
        ownership_percentage IS NULL OR 
        (ownership_percentage > 0 AND ownership_percentage <= 100)
    )
);

-- Account relationships - Internal servicing responsibility
CREATE TABLE account_relationships (
    relationship_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    entity_id UUID NOT NULL,
    entity_type VARCHAR(30) NOT NULL CHECK (entity_type IN ('Branch', 'Agent', 'RiskManager', 'ComplianceOfficer')),
    relationship_type VARCHAR(30) NOT NULL CHECK (relationship_type IN ('PrimaryHandler', 'BackupHandler', 'RiskOversight', 'ComplianceOversight')),
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Inactive', 'Suspended')),
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,
    
    CONSTRAINT ck_relationship_dates CHECK (end_date IS NULL OR end_date >= start_date)
);

-- Account mandates - Operational rights and permissions
CREATE TABLE account_mandates (
    mandate_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    grantee_customer_id UUID NOT NULL REFERENCES customers(customer_id),
    permission_type VARCHAR(30) NOT NULL CHECK (permission_type IN ('ViewOnly', 'LimitedWithdrawal', 'JointApproval', 'FullAccess')),
    transaction_limit DECIMAL(15,2),
    approval_group_id UUID,
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Suspended', 'Revoked')),
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,
    
    CONSTRAINT ck_mandate_dates CHECK (end_date IS NULL OR end_date >= start_date),
    CONSTRAINT ck_transaction_limit_positive CHECK (transaction_limit IS NULL OR transaction_limit > 0)
);

-- Ultimate Beneficial Owners - KYC/AML compliance for corporate accounts
CREATE TABLE ultimate_beneficiaries (
    ubo_link_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    corporate_customer_id UUID NOT NULL,
    beneficiary_customer_id UUID NOT NULL,
    ownership_percentage DECIMAL(5,2),
    control_type VARCHAR(30) NOT NULL CHECK (control_type IN ('DirectOwnership', 'IndirectOwnership', 'SignificantInfluence', 'SeniorManagement')),
    description VARCHAR(500),
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Inactive', 'Under_Investigation')),
    verification_status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (verification_status IN ('Pending', 'Verified', 'Rejected', 'RequiresUpdate')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT fk_ubo_corporate FOREIGN KEY (corporate_customer_id) REFERENCES customers(customer_id),
    CONSTRAINT fk_ubo_beneficiary FOREIGN KEY (beneficiary_customer_id) REFERENCES customers(customer_id),
    CONSTRAINT ck_ubo_percentage CHECK (ownership_percentage IS NULL OR (ownership_percentage > 0 AND ownership_percentage <= 100)),
    CONSTRAINT ck_different_customers CHECK (corporate_customer_id != beneficiary_customer_id)
);

-- =============================================================================
-- AGENT BANKING & INSTITUTIONAL HIERARCHY
-- =============================================================================

-- Top-level network management
CREATE TABLE agent_networks (
    network_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    network_name VARCHAR(100) NOT NULL,
    network_type VARCHAR(20) NOT NULL CHECK (network_type IN ('Internal', 'Partner', 'ThirdParty')),
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Suspended', 'Terminated')),
    contract_id UUID,
    aggregate_daily_limit DECIMAL(15,2) NOT NULL,
    current_daily_volume DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    settlement_gl_code VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_daily_limit_positive CHECK (aggregate_daily_limit > 0),
    CONSTRAINT ck_current_volume_nonnegative CHECK (current_daily_volume >= 0),
    CONSTRAINT uk_network_name UNIQUE (network_name)
);

-- Hierarchical branch structure with cascading limits
CREATE TABLE agency_branches (
    branch_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    network_id UUID NOT NULL REFERENCES agent_networks(network_id),
    parent_branch_id UUID REFERENCES agency_branches(branch_id),
    branch_name VARCHAR(100) NOT NULL,
    branch_code VARCHAR(20) NOT NULL,
    branch_level INTEGER NOT NULL DEFAULT 0,
    gl_code_prefix VARCHAR(10) NOT NULL,
    geolocation VARCHAR(100),
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Suspended', 'Closed')),
    daily_transaction_limit DECIMAL(15,2) NOT NULL,
    current_daily_volume DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_branch_limit_positive CHECK (daily_transaction_limit > 0),
    CONSTRAINT ck_branch_volume_nonnegative CHECK (current_daily_volume >= 0),
    CONSTRAINT ck_branch_level_nonnegative CHECK (branch_level >= 0),
    CONSTRAINT uk_branch_code UNIQUE (branch_code),
    CONSTRAINT uk_gl_prefix UNIQUE (gl_code_prefix)
);

-- Individual terminal controls
CREATE TABLE agent_terminals (
    terminal_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    branch_id UUID NOT NULL REFERENCES agency_branches(branch_id),
    agent_user_id UUID NOT NULL,
    terminal_type VARCHAR(20) NOT NULL CHECK (terminal_type IN ('Pos', 'Mobile', 'Atm', 'WebPortal')),
    terminal_name VARCHAR(100) NOT NULL,
    daily_transaction_limit DECIMAL(15,2) NOT NULL,
    current_daily_volume DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Maintenance', 'Suspended', 'Decommissioned')),
    last_sync_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_terminal_limit_positive CHECK (daily_transaction_limit > 0),
    CONSTRAINT ck_terminal_volume_nonnegative CHECK (current_daily_volume >= 0)
);

-- =============================================================================
-- TRANSACTION PROCESSING WITH APPROVAL WORKFLOWS
-- =============================================================================

-- Comprehensive transaction model with multi-party support
CREATE TABLE transactions (
    transaction_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    transaction_code VARCHAR(20) NOT NULL,
    transaction_type VARCHAR(10) NOT NULL CHECK (transaction_type IN ('Credit', 'Debit')),
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    description VARCHAR(255) NOT NULL,
    channel_id VARCHAR(50) NOT NULL,
    terminal_id UUID REFERENCES agent_terminals(terminal_id),
    agent_user_id UUID,
    transaction_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    value_date DATE NOT NULL DEFAULT CURRENT_DATE,
    status VARCHAR(30) NOT NULL DEFAULT 'Pending' CHECK (
        status IN ('Pending', 'Posted', 'Reversed', 'Failed', 'AwaitingApproval', 'ApprovalRejected')
    ),
    reference_number VARCHAR(50) NOT NULL,
    external_reference VARCHAR(100),
    gl_code VARCHAR(20) NOT NULL,
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    approval_status VARCHAR(20) CHECK (approval_status IN ('Pending', 'Approved', 'Rejected', 'PartiallyApproved')),
    risk_score DECIMAL(5,2),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_amount_positive CHECK (amount > 0),
    CONSTRAINT ck_risk_score_range CHECK (risk_score IS NULL OR (risk_score >= 0 AND risk_score <= 100)),
    CONSTRAINT ck_approval_consistency CHECK (
        (requires_approval = TRUE AND approval_status IS NOT NULL) OR
        (requires_approval = FALSE AND approval_status IS NULL)
    ),
    CONSTRAINT uk_reference_number UNIQUE (reference_number)
);

-- =============================================================================
-- WORKFLOW MANAGEMENT SYSTEM (From Enhancements)
-- =============================================================================

-- Account workflow tracking for multi-step processes
CREATE TABLE account_workflows (
    workflow_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    workflow_type VARCHAR(50) NOT NULL CHECK (
        workflow_type IN ('AccountOpening', 'AccountClosure', 'AccountReactivation', 'ComplianceVerification', 'MultiPartyApproval')
    ),
    current_step VARCHAR(50) NOT NULL CHECK (
        current_step IN ('InitiateRequest', 'ComplianceCheck', 'DocumentVerification', 'ApprovalRequired', 'FinalSettlement', 'Completed')
    ),
    status VARCHAR(20) NOT NULL DEFAULT 'InProgress' CHECK (
        status IN ('InProgress', 'PendingAction', 'Completed', 'Failed', 'Cancelled', 'TimedOut')
    ),
    initiated_by VARCHAR(100) NOT NULL,
    initiated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    timeout_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_workflow_completion CHECK (
        (status = 'Completed' AND completed_at IS NOT NULL) OR
        (status != 'Completed' AND completed_at IS NULL)
    )
);

-- Workflow step history for audit trail
CREATE TABLE workflow_step_records (
    record_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(workflow_id),
    step_name VARCHAR(50) NOT NULL,
    completed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    completed_by VARCHAR(100) NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Account status history - immutable audit trail
CREATE TABLE account_status_history (
    history_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    old_status VARCHAR(30),
    new_status VARCHAR(30) NOT NULL,
    change_reason VARCHAR(255) NOT NULL,
    changed_by VARCHAR(100) NOT NULL,
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    system_triggered BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Final settlement records for closed accounts
CREATE TABLE account_final_settlements (
    settlement_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    settlement_date DATE NOT NULL,
    current_balance DECIMAL(15,2) NOT NULL,
    accrued_interest DECIMAL(15,2) NOT NULL,
    closure_fees DECIMAL(15,2) NOT NULL,
    final_amount DECIMAL(15,2) NOT NULL,
    disbursement_method VARCHAR(20) NOT NULL CHECK (disbursement_method IN ('Transfer', 'CashWithdrawal', 'Check', 'HoldFunds')),
    disbursement_reference VARCHAR(100),
    processed_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_settlement_account UNIQUE (account_id)
);

-- =============================================================================
-- BUSINESS CALENDAR MANAGEMENT
-- =============================================================================

-- Definitive non-working days registry
CREATE TABLE bank_holidays (
    holiday_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL,
    holiday_date DATE NOT NULL,
    holiday_name VARCHAR(100) NOT NULL,
    holiday_type VARCHAR(20) NOT NULL CHECK (holiday_type IN ('National', 'Religious', 'Banking', 'Custom')),
    is_recurring BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_jurisdiction_date UNIQUE (jurisdiction, holiday_date)
);

-- =============================================================================
-- COMPLIANCE & KYC/AML TABLES
-- =============================================================================

-- KYC records for customer verification
CREATE TABLE kyc_records (
    kyc_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    kyc_type VARCHAR(30) NOT NULL CHECK (kyc_type IN ('Initial', 'Enhanced', 'Periodic', 'MiniKYC')),
    status VARCHAR(30) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'InProgress', 'Completed', 'RequiresReview', 'Rejected')),
    risk_assessment VARCHAR(20) NOT NULL DEFAULT 'Medium' CHECK (risk_assessment IN ('Low', 'Medium', 'High')),
    last_review_date DATE,
    next_review_date DATE,
    reviewed_by VARCHAR(100),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Sanctions screening records
CREATE TABLE sanctions_screening (
    screening_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    screening_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    screening_result VARCHAR(20) NOT NULL CHECK (screening_result IN ('Clear', 'PotentialMatch', 'Confirmed_Match')),
    match_details TEXT,
    last_screening_date TIMESTAMP WITH TIME ZONE,
    screened_by VARCHAR(100) NOT NULL,
    false_positive BOOLEAN DEFAULT FALSE
);

-- Compliance risk scores
CREATE TABLE compliance_risk_scores (
    risk_score_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    risk_score DECIMAL(5,2) NOT NULL,
    risk_factors JSONB,
    calculated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMP WITH TIME ZONE NOT NULL,
    
    CONSTRAINT ck_risk_score_valid_range CHECK (risk_score >= 0 AND risk_score <= 100),
    CONSTRAINT ck_valid_date_future CHECK (valid_until > calculated_at)
);

-- Compliance alerts and suspicious activity
CREATE TABLE compliance_alerts (
    alert_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customers(customer_id),
    account_id UUID REFERENCES accounts(account_id),
    transaction_id UUID REFERENCES transactions(transaction_id),
    alert_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('Low', 'Medium', 'High', 'Critical')),
    description TEXT NOT NULL,
    status VARCHAR(30) NOT NULL DEFAULT 'Open' CHECK (status IN ('Open', 'InProgress', 'Resolved', 'FalsePositive')),
    assigned_to VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolution_notes TEXT
);

-- =============================================================================
-- PERFORMANCE INDEXES
-- =============================================================================

-- Customer indexes
CREATE INDEX idx_customers_risk_rating ON customers(risk_rating) WHERE status = 'Active';
CREATE INDEX idx_customers_status ON customers(status);
CREATE INDEX idx_customers_id_lookup ON customers(id_type, id_number);
CREATE INDEX idx_customer_audit_lookup ON customer_audit_trail(customer_id, changed_at DESC);

-- Account indexes
CREATE INDEX idx_accounts_customer_lookup ON account_ownership(customer_id);
CREATE INDEX idx_accounts_product_code ON accounts(product_code) WHERE account_status = 'Active';
CREATE INDEX idx_accounts_status ON accounts(account_status);
CREATE INDEX idx_accounts_last_activity ON accounts(last_activity_date, account_status) WHERE account_status = 'Active';
CREATE INDEX idx_accounts_branch ON accounts(domicile_branch_id);
CREATE INDEX idx_account_status_change ON account_status_history(account_id, changed_at DESC);

-- Transaction indexes
CREATE INDEX idx_transactions_account_date ON transactions(account_id, transaction_date DESC);
CREATE INDEX idx_transactions_value_date ON transactions(value_date) WHERE status = 'Posted';
CREATE INDEX idx_transactions_channel ON transactions(channel_id, transaction_date DESC);
CREATE INDEX idx_transactions_reference ON transactions(reference_number);
CREATE INDEX idx_transactions_approval ON transactions(approval_status) WHERE requires_approval = TRUE;

-- Agent network indexes
CREATE INDEX idx_terminals_branch ON agent_terminals(branch_id);
CREATE INDEX idx_terminals_status ON agent_terminals(status) WHERE status = 'Active';
CREATE INDEX idx_branches_network ON agency_branches(network_id);
CREATE INDEX idx_branches_parent ON agency_branches(parent_branch_id);

-- Workflow indexes
CREATE INDEX idx_workflows_account ON account_workflows(account_id);
CREATE INDEX idx_workflows_status ON account_workflows(status) WHERE status IN ('InProgress', 'PendingAction');
CREATE INDEX idx_workflow_steps ON workflow_step_records(workflow_id, completed_at);

-- Compliance indexes
CREATE INDEX idx_kyc_customer ON kyc_records(customer_id);
CREATE INDEX idx_kyc_review_due ON kyc_records(next_review_date) WHERE status = 'Completed';
CREATE INDEX idx_sanctions_customer ON sanctions_screening(customer_id);
CREATE INDEX idx_sanctions_result ON sanctions_screening(screening_result) WHERE screening_result != 'Clear';
CREATE INDEX idx_compliance_alerts_status ON compliance_alerts(status) WHERE status IN ('Open', 'InProgress');

-- Calendar indexes
CREATE INDEX idx_holidays_jurisdiction_date ON bank_holidays(jurisdiction, holiday_date);
CREATE INDEX idx_holidays_recurring ON bank_holidays(holiday_date) WHERE is_recurring = TRUE;