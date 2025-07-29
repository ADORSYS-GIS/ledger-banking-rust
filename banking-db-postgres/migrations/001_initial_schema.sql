-- =============================================================================
-- CONSOLIDATED INITIAL SCHEMA FOR LEDGER-BANKING-RUST
-- Single migration for development - combines all schema definitions
-- =============================================================================

-- =============================================================================
-- EXTENSIONS AND SETUP
-- =============================================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- =============================================================================
-- CUSTOM TYPES AND ENUMS
-- =============================================================================

-- Person type enum for referenced persons
CREATE TYPE person_type AS ENUM ('natural', 'legal', 'system', 'integration', 'unknown');

-- Customer related enums
CREATE TYPE customer_type AS ENUM ('Individual', 'Corporate');
CREATE TYPE identity_type AS ENUM ('NationalId', 'Passport', 'CompanyRegistration');
CREATE TYPE risk_rating AS ENUM ('Low', 'Medium', 'High', 'Blacklisted');
CREATE TYPE customer_status AS ENUM ('Active', 'PendingVerification', 'Deceased', 'Dissolved', 'Blacklisted');
CREATE TYPE kyc_status AS ENUM ('NotStarted', 'InProgress', 'Pending', 'Complete', 'Approved', 'Rejected', 'RequiresUpdate', 'Failed');
CREATE TYPE document_status AS ENUM ('Uploaded', 'Verified', 'Rejected', 'Expired');

-- Account related enums
CREATE TYPE account_type AS ENUM ('Savings', 'Current', 'Loan');
CREATE TYPE account_status AS ENUM ('PendingApproval', 'Active', 'Dormant', 'Frozen', 'PendingClosure', 'Closed', 'PendingReactivation');
CREATE TYPE signing_condition AS ENUM ('None', 'AnyOwner', 'AllOwners');
CREATE TYPE disbursement_method AS ENUM ('Transfer', 'CashWithdrawal', 'Check', 'HoldFunds');
CREATE TYPE hold_type AS ENUM ('UnclearedFunds', 'JudicialLien', 'LoanPledge', 'ComplianceHold', 'AdministrativeHold', 'FraudHold', 'PendingAuthorization', 'OverdraftReserve', 'CardAuthorization', 'Other');
CREATE TYPE hold_status AS ENUM ('Active', 'Released', 'Expired', 'Cancelled', 'PartiallyReleased');
CREATE TYPE hold_priority AS ENUM ('Critical', 'High', 'Medium', 'Low');
CREATE TYPE ownership_type AS ENUM ('Single', 'Joint', 'Corporate');
CREATE TYPE entity_type AS ENUM ('Branch', 'Agent', 'RiskManager', 'ComplianceOfficer', 'CustomerService');
CREATE TYPE relationship_type AS ENUM ('PrimaryHandler', 'BackupHandler', 'RiskOversight', 'ComplianceOversight');
CREATE TYPE relationship_status AS ENUM ('Active', 'Inactive', 'Suspended');
CREATE TYPE permission_type AS ENUM ('ViewOnly', 'LimitedWithdrawal', 'JointApproval', 'FullAccess');
CREATE TYPE mandate_status AS ENUM ('Active', 'Suspended', 'Revoked', 'Expired');
CREATE TYPE ubo_status AS ENUM ('Active', 'Inactive', 'UnderReview');

-- Compliance related enums
CREATE TYPE check_result AS ENUM ('Pass', 'Fail', 'Warning', 'Manual');
CREATE TYPE screening_type AS ENUM ('Sanctions', 'PoliticallyExposed', 'AdverseMedia', 'Watchlist');
CREATE TYPE risk_level AS ENUM ('Low', 'Medium', 'High', 'Critical');
CREATE TYPE alert_type AS ENUM ('StructuringDetection', 'VelocityCheck', 'LargeCashTransaction', 'SuspiciousPattern', 'GeographicAnomaly', 'CrossBorderTransaction');
CREATE TYPE severity AS ENUM ('Low', 'Medium', 'High', 'Critical');
CREATE TYPE alert_status AS ENUM ('New', 'InReview', 'Investigated', 'Cleared', 'Escalated');
CREATE TYPE sar_status AS ENUM ('Draft', 'Filed', 'Acknowledged');
CREATE TYPE compliance_status AS ENUM ('Passed', 'Failed', 'RequiresReview', 'Pending');
CREATE TYPE control_type AS ENUM ('DirectOwnership', 'IndirectOwnership', 'SignificantInfluence', 'SeniorManagement');
CREATE TYPE verification_status AS ENUM ('Pending', 'Verified', 'Rejected', 'RequiresUpdate');

-- Calendar enums
CREATE TYPE holiday_type AS ENUM ('National', 'Regional', 'Religious', 'Banking');
CREATE TYPE date_shift_rule AS ENUM ('NextBusinessDay', 'PreviousBusinessDay', 'NoShift');
CREATE TYPE weekend_treatment AS ENUM ('SaturdaySunday', 'FridayOnly', 'Custom');
CREATE TYPE import_status AS ENUM ('Success', 'Partial', 'Failed');

-- =============================================================================
-- UTILITY FUNCTIONS
-- =============================================================================

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- PERSONS TABLE - Central person reference system
-- =============================================================================

CREATE TABLE persons (
    person_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_type person_type NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    external_identifier VARCHAR(50),
    organization VARCHAR(100),
    email VARCHAR(100),
    phone VARCHAR(20),
    department VARCHAR(50),
    office_location VARCHAR(200),
    duplicate_of UUID REFERENCES persons(person_id),
    entity_reference UUID,
    entity_type VARCHAR(50),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for persons
CREATE INDEX idx_persons_external_id ON persons(external_identifier) WHERE external_identifier IS NOT NULL;
CREATE INDEX idx_persons_entity_ref ON persons(entity_reference, entity_type) WHERE entity_reference IS NOT NULL;
CREATE INDEX idx_persons_duplicate ON persons(duplicate_of) WHERE duplicate_of IS NOT NULL;
CREATE INDEX idx_persons_active ON persons(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_persons_display_name ON persons(display_name);
CREATE INDEX idx_persons_type ON persons(person_type);

-- Trigger for updated_at
CREATE TRIGGER update_persons_updated_at
    BEFORE UPDATE ON persons
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- CUSTOMER INFORMATION FILE (CIF) - Master Repository
-- =============================================================================

-- Main customers table - Single source of truth for customer data
CREATE TABLE customers (
    customer_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_type customer_type NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    id_type identity_type NOT NULL,
    id_number VARCHAR(50) NOT NULL,
    risk_rating risk_rating NOT NULL DEFAULT 'Low',
    status customer_status NOT NULL DEFAULT 'PendingVerification',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES persons(person_id),
    
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
    status document_status NOT NULL DEFAULT 'Uploaded',
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    uploaded_by UUID NOT NULL REFERENCES persons(person_id),
    verified_at TIMESTAMP WITH TIME ZONE,
    verified_by UUID REFERENCES persons(person_id),
    
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
    old_value VARCHAR(255),
    new_value VARCHAR(255),
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    changed_by UUID NOT NULL REFERENCES persons(person_id),
    reason VARCHAR(255)
);

-- =============================================================================
-- REASON AND PURPOSE MULTILINGUAL SYSTEM
-- =============================================================================

-- Create the main reason_and_purpose table (moved up for foreign key dependencies)
CREATE TABLE reason_and_purpose (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    category VARCHAR(50) NOT NULL,
    context VARCHAR(50) NOT NULL,
    
    -- Multilingual content (up to 3 languages)
    l1_content VARCHAR(100),
    l2_content VARCHAR(100), 
    l3_content VARCHAR(100),
    l1_language_code CHAR(3),
    l2_language_code CHAR(3),
    l3_language_code CHAR(3),
    
    -- Reason properties
    requires_details BOOLEAN DEFAULT FALSE,
    is_active BOOLEAN DEFAULT TRUE,
    severity VARCHAR(20),
    display_order INTEGER DEFAULT 0,
    
    -- Compliance metadata fields
    regulatory_code VARCHAR(20),
    reportable BOOLEAN DEFAULT FALSE,
    requires_sar BOOLEAN DEFAULT FALSE,
    requires_ctr BOOLEAN DEFAULT FALSE,
    retention_years SMALLINT,
    escalation_required BOOLEAN DEFAULT FALSE,
    risk_score_impact SMALLINT,
    no_tipping_off BOOLEAN DEFAULT FALSE,
    jurisdictions TEXT[], -- Array of country codes
    
    -- Audit fields
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by VARCHAR(100) NOT NULL,
    updated_by VARCHAR(100) NOT NULL
);

-- Create indexes for efficient querying
CREATE INDEX idx_reason_code ON reason_and_purpose(code);
CREATE INDEX idx_reason_category ON reason_and_purpose(category);
CREATE INDEX idx_reason_context ON reason_and_purpose(context);
CREATE INDEX idx_reason_active ON reason_and_purpose(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_reason_reportable ON reason_and_purpose(reportable) WHERE reportable = TRUE;
CREATE INDEX idx_reason_sar ON reason_and_purpose(requires_sar) WHERE requires_sar = TRUE;
CREATE INDEX idx_reason_category_context ON reason_and_purpose(category, context) WHERE is_active = TRUE;

-- =============================================================================
-- UNIFIED ACCOUNT MODEL (UAM) - Multi-Product Support
-- =============================================================================

-- Comprehensive accounts table supporting all banking products
CREATE TABLE accounts (
    account_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_code VARCHAR(12) NOT NULL,
    account_type account_type NOT NULL,
    account_status account_status NOT NULL DEFAULT 'Active',
    signing_condition signing_condition NOT NULL DEFAULT 'None',
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    open_date DATE NOT NULL DEFAULT CURRENT_DATE,
    domicile_branch_id UUID NOT NULL,
    
    -- Balance fields - core to all account types
    current_balance DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    available_balance DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    accrued_interest DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    overdraft_limit DECIMAL(15,2) DEFAULT NULL, -- Only for current accounts
    
    -- Loan-specific fields (nullable for non-loan accounts)
    original_principal DECIMAL(15,2), -- Loan amount initially disbursed
    outstanding_principal DECIMAL(15,2), -- Current outstanding amount
    loan_interest_rate DECIMAL(8,6), -- Annual percentage rate
    loan_term_months INTEGER, -- Original term in months
    disbursement_date DATE, -- When loan was disbursed
    maturity_date DATE, -- Final repayment date
    installment_amount DECIMAL(15,2), -- Monthly payment amount
    next_due_date DATE, -- Next installment due date
    penalty_rate DECIMAL(8,6), -- Late payment penalty rate
    collateral_id VARCHAR(100), -- Reference to collateral
    loan_purpose_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for loan purpose
    
    -- Account lifecycle management (from enhancements)
    close_date DATE,
    last_activity_date DATE,
    dormancy_threshold_days INTEGER DEFAULT 365, -- Days of inactivity before dormancy
    reactivation_required BOOLEAN DEFAULT FALSE,
    pending_closure_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for closure
    
    -- Enhanced audit trail
    status_changed_by UUID REFERENCES persons(person_id),
    status_change_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for status change
    status_change_timestamp TIMESTAMP WITH TIME ZONE,
    
    -- Audit fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES persons(person_id),
    
    -- Business constraints ensuring data integrity
    CONSTRAINT ck_balance_consistency CHECK (current_balance >= 0 OR account_type = 'Current'),
    CONSTRAINT ck_available_balance_consistency CHECK (available_balance <= current_balance + COALESCE(overdraft_limit, 0)),
    CONSTRAINT ck_overdraft_only_current CHECK (overdraft_limit IS NULL OR account_type = 'Current'),
    CONSTRAINT ck_loan_fields_consistency CHECK (
        (account_type = 'Loan' AND original_principal IS NOT NULL AND outstanding_principal IS NOT NULL) OR
        (account_type != 'Loan' AND original_principal IS NULL AND outstanding_principal IS NULL)
    ),
    CONSTRAINT ck_loan_principal_relationship CHECK (
        account_type != 'Loan' OR (outstanding_principal <= original_principal AND outstanding_principal >= 0)
    ),
    CONSTRAINT ck_loan_dates_consistency CHECK (
        account_type != 'Loan' OR (disbursement_date IS NULL OR maturity_date IS NULL OR disbursement_date <= maturity_date)
    ),
    CONSTRAINT ck_currency_code_format CHECK (LENGTH(currency) = 3 AND currency = UPPER(currency))
);

-- =============================================================================
-- ACCOUNT ACCESS AND OWNERSHIP
-- =============================================================================

-- Account ownership tracking (supports individual and joint accounts)
CREATE TABLE account_ownership (
    ownership_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    ownership_type ownership_type NOT NULL,
    ownership_percentage DECIMAL(5,2) DEFAULT 100.00 CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_account_primary_owner UNIQUE (account_id, customer_id)
);

-- Account relationships for servicing and internal bank operations
CREATE TABLE account_relationships (
    relationship_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    entity_id UUID NOT NULL, -- Could be employee, department, or external entity
    entity_type entity_type NOT NULL,
    relationship_type relationship_type NOT NULL,
    status relationship_status NOT NULL DEFAULT 'Active',
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,
    
    CONSTRAINT ck_relationship_dates CHECK (end_date IS NULL OR end_date >= start_date)
);

-- Account mandates for operational permissions
CREATE TABLE account_mandates (
    mandate_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    grantee_customer_id UUID NOT NULL REFERENCES customers(customer_id), -- References external person or customer
    permission_type permission_type NOT NULL,
    transaction_limit DECIMAL(15,2),
    approval_group_id UUID,
    status mandate_status NOT NULL DEFAULT 'Active',
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,
    
    CONSTRAINT ck_mandate_dates CHECK (end_date IS NULL OR end_date >= start_date)
);

-- Ultimate Beneficial Owner (UBO) tracking for corporate accounts - regulatory requirement
CREATE TABLE ultimate_beneficial_owners (
    ubo_link_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    corporate_customer_id UUID NOT NULL REFERENCES customers(customer_id),
    beneficiary_customer_id UUID NOT NULL REFERENCES customers(customer_id),
    ownership_percentage DECIMAL(5,2) CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    control_type control_type NOT NULL,
    description VARCHAR(256),
    status ubo_status NOT NULL DEFAULT 'Active',
    verification_status verification_status NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_ubo_not_self_reference CHECK (corporate_customer_id != beneficiary_customer_id)
);

-- =============================================================================
-- TRANSACTION PROCESSING ENGINE
-- =============================================================================

-- Comprehensive transaction table with multi-stage processing support
CREATE TABLE transactions (
    transaction_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    transaction_code VARCHAR(20) NOT NULL, -- Internal transaction type code
    transaction_type VARCHAR(10) NOT NULL CHECK (transaction_type IN ('Credit', 'Debit')),
    amount DECIMAL(15,2) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    description VARCHAR(200) NOT NULL,
    
    -- Multi-channel processing support
    channel_id VARCHAR(50) NOT NULL, -- 'BranchTeller', 'ATM', 'OnlineBanking', 'MobileBanking', 'AgentBanking'
    terminal_id VARCHAR(50), -- For ATM/POS transactions
    agent_user_id UUID, -- For agent banking transactions
    
    -- Transaction timing
    transaction_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    value_date DATE NOT NULL, -- When the transaction affects the balance
    
    -- Status and approval workflow
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Posted', 'Failed', 'Reversed', 'OnHold')),
    reference_number VARCHAR(200) NOT NULL UNIQUE,
    external_reference VARCHAR(100), -- For external system correlation
    
    -- Financial posting integration
    gl_code VARCHAR(20) NOT NULL, -- General Ledger account code
    
    -- Risk and compliance
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    approval_status VARCHAR(20) CHECK (approval_status IN ('Pending', 'Approved', 'Rejected')),
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0 AND risk_score <= 100),
    
    -- Audit
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_transaction_approval CHECK (
        (requires_approval = FALSE) OR 
        (requires_approval = TRUE AND approval_status IS NOT NULL)
    ),
    CONSTRAINT ck_currency_format CHECK (LENGTH(currency) = 3 AND currency = UPPER(currency))
);

-- Transaction audit trail - immutable record of all transaction state changes
CREATE TABLE transaction_audit_trail (
    audit_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(transaction_id),
    action_type VARCHAR(50) NOT NULL, -- 'Created', 'StatusChanged', 'Posted', 'Reversed', 'Failed', 'Approved', 'Rejected'
    performed_by UUID NOT NULL REFERENCES persons(person_id),
    performed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    old_status VARCHAR(20), -- TransactionStatus enum values
    new_status VARCHAR(20), -- TransactionStatus enum values  
    reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for audit reason
    details_hash BYTEA -- Blake3 hash of additional details for tamper detection
);

-- =============================================================================
-- AGENT BANKING NETWORK MANAGEMENT
-- =============================================================================

-- Agent networks - hierarchical structure for agent banking
CREATE TABLE agent_networks (
    network_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    network_code VARCHAR(20) NOT NULL UNIQUE,
    network_name VARCHAR(100) NOT NULL,
    network_type VARCHAR(30) NOT NULL CHECK (network_type IN ('Rural', 'Urban', 'Corporate', 'Educational', 'Healthcare')),
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Inactive', 'Suspended', 'UnderReview')),
    max_transaction_limit DECIMAL(15,2) NOT NULL,
    max_daily_limit DECIMAL(15,2) NOT NULL,
    commission_rate DECIMAL(5,4) NOT NULL CHECK (commission_rate >= 0 AND commission_rate <= 1),
    settlement_gl_code VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_network_limits CHECK (max_daily_limit >= max_transaction_limit)
);

-- Agent branches within networks
CREATE TABLE agent_branches (
    branch_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    network_id UUID NOT NULL REFERENCES agent_networks(network_id),
    branch_code VARCHAR(20) NOT NULL,
    branch_name VARCHAR(100) NOT NULL,
    contact_person VARCHAR(100),
    phone_number VARCHAR(20),
    email VARCHAR(100),
    physical_address TEXT,
    gps_coordinates POINT, -- PostGIS point for location
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Inactive', 'Suspended', 'TemporarilyClosed')),
    max_transaction_limit DECIMAL(15,2) NOT NULL,
    max_daily_limit DECIMAL(15,2) NOT NULL,
    settlement_account_id UUID, -- Internal settlement account
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_branch_code_per_network UNIQUE (network_id, branch_code)
);

-- Agent terminals - individual POS/mobile terminals
CREATE TABLE agent_terminals (
    terminal_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    branch_id UUID NOT NULL REFERENCES agent_branches(branch_id),
    terminal_code VARCHAR(20) NOT NULL,
    terminal_type VARCHAR(20) NOT NULL CHECK (terminal_type IN ('POS', 'Mobile', 'Web', 'USSD')),
    agent_user_id UUID NOT NULL, -- Reference to agent user account
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Inactive', 'Suspended', 'Blocked')),
    max_transaction_limit DECIMAL(15,2) NOT NULL,
    max_daily_limit DECIMAL(15,2) NOT NULL,
    last_activity_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_terminal_code_per_branch UNIQUE (branch_id, terminal_code),
    CONSTRAINT ck_terminal_limits CHECK (max_daily_limit >= max_transaction_limit)
);

-- =============================================================================
-- CALENDAR AND BUSINESS DAY MANAGEMENT
-- =============================================================================

-- Bank holidays for business day calculations
CREATE TABLE bank_holidays (
    holiday_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL, -- Country/region code
    holiday_date DATE NOT NULL,
    holiday_name VARCHAR(255) NOT NULL,
    holiday_type holiday_type NOT NULL,
    is_recurring BOOLEAN NOT NULL DEFAULT FALSE,
    description VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES persons(person_id),
    
    CONSTRAINT uk_holiday_per_jurisdiction UNIQUE (jurisdiction, holiday_date)
);

-- Weekend configuration per jurisdiction
CREATE TABLE weekend_configuration (
    config_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL UNIQUE,
    weekend_days VARCHAR(100) NOT NULL, -- JSON array of weekday numbers (0=Sunday, 1=Monday, etc.)
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    notes VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES persons(person_id),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES persons(person_id)
);

-- Date calculation rules for business day calculations
CREATE TABLE date_calculation_rules (
    rule_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL,
    rule_name VARCHAR(100) NOT NULL,
    rule_type VARCHAR(30) NOT NULL CHECK (rule_type IN ('DateShift', 'MaturityCalculation', 'PaymentDue')),
    default_shift_rule date_shift_rule NOT NULL,
    weekend_treatment weekend_treatment NOT NULL,
    product_specific_overrides VARCHAR(1000), -- JSON with product-specific rules
    priority INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE,
    expiry_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by UUID NOT NULL REFERENCES persons(person_id),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by UUID NOT NULL REFERENCES persons(person_id),
    
    CONSTRAINT ck_rule_dates CHECK (expiry_date IS NULL OR expiry_date > effective_date)
);

-- Holiday import log for audit trail of holiday imports
CREATE TABLE holiday_import_log (
    import_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL,
    import_year INTEGER NOT NULL,
    import_source VARCHAR(100) NOT NULL,
    holidays_imported INTEGER NOT NULL DEFAULT 0,
    holidays_updated INTEGER NOT NULL DEFAULT 0,
    holidays_skipped INTEGER NOT NULL DEFAULT 0,
    import_status import_status NOT NULL,
    error_details VARCHAR(1000),
    imported_by UUID NOT NULL REFERENCES persons(person_id),
    imported_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Business day cache for performance optimization
CREATE TABLE business_day_cache (
    cache_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL,
    date DATE NOT NULL,
    is_business_day BOOLEAN NOT NULL,
    is_holiday BOOLEAN NOT NULL DEFAULT FALSE,
    is_weekend BOOLEAN NOT NULL DEFAULT FALSE,
    holiday_name VARCHAR(255),
    cached_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMP WITH TIME ZONE NOT NULL,
    
    CONSTRAINT uk_cache_per_jurisdiction_date UNIQUE (jurisdiction, date)
);

-- =============================================================================
-- WORKFLOW AND APPROVAL MANAGEMENT
-- =============================================================================

-- Account workflow management
CREATE TABLE account_workflows (
    workflow_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    workflow_type VARCHAR(50) NOT NULL CHECK (workflow_type IN (
        'AccountOpening', 'AccountClosure', 'AccountReactivation', 
        'ComplianceVerification', 'MultiPartyApproval'
    )),
    current_step VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Pending' CHECK (status IN ('Pending', 'Approved', 'Rejected', 'TimedOut')),
    initiated_by UUID NOT NULL REFERENCES persons(person_id),
    initiated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    timeout_at TIMESTAMP WITH TIME ZONE NOT NULL,
    completed_at TIMESTAMP WITH TIME ZONE,
    next_action_required VARCHAR(500),
    rejection_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for rejection
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Workflow step records
CREATE TABLE workflow_step_records (
    step_record_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(workflow_id) ON DELETE CASCADE,
    step_name VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL CHECK (status IN ('Pending', 'InProgress', 'Completed', 'Skipped', 'Failed')),
    assigned_to VARCHAR(100),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    notes VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Transaction approval workflow
CREATE TABLE transaction_approvals (
    approval_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(workflow_id),
    transaction_id UUID NOT NULL REFERENCES transactions(transaction_id),
    approver_id UUID NOT NULL,
    approval_action VARCHAR(20) NOT NULL CHECK (approval_action IN ('Pending', 'Approved', 'Rejected', 'PartiallyApproved')),
    approved_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    approval_notes VARCHAR(512),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Account status history with enhanced tracking
CREATE TABLE account_status_history (
    history_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    old_status VARCHAR(30),
    new_status VARCHAR(30) NOT NULL,
    change_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for status change
    additional_context VARCHAR(200), -- Additional context beyond the standard reason
    changed_by UUID NOT NULL REFERENCES persons(person_id),
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    system_triggered BOOLEAN NOT NULL DEFAULT FALSE,
    workflow_id UUID, -- Link to associated workflow
    supporting_documents JSONB, -- JSON array of document references
    approval_required BOOLEAN NOT NULL DEFAULT FALSE,
    approved_by UUID REFERENCES persons(person_id),
    approved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- COMPLIANCE AND RISK MANAGEMENT
-- =============================================================================

-- KYC (Know Your Customer) results and documentation
CREATE TABLE kyc_results (
    kyc_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    check_type BYTEA NOT NULL, -- Blake3 hash for content-addressable check type identification
    status VARCHAR(20) NOT NULL CHECK (status IN ('Passed', 'Failed', 'Pending', 'RequiresManualReview')),
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0 AND risk_score <= 100),
    expiry_date DATE,
    performed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    performed_by UUID NOT NULL REFERENCES persons(person_id),
    details BYTEA, -- Blake3 hash of KYC details for tamper detection
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Sanctions screening results
CREATE TABLE sanctions_screening (
    screening_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    screening_result VARCHAR(20) NOT NULL CHECK (screening_result IN ('Clear', 'PotentialMatch', 'ConfirmedMatch')),
    match_details JSONB, -- Structured data about potential matches
    screened_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    screened_by UUID NOT NULL REFERENCES persons(person_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Compliance alerts and monitoring
CREATE TABLE compliance_alerts (
    alert_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customers(customer_id),
    account_id UUID REFERENCES accounts(account_id),
    transaction_id UUID REFERENCES transactions(transaction_id),
    alert_type alert_type NOT NULL,
    severity severity NOT NULL,
    description VARCHAR(500) NOT NULL,
    status alert_status NOT NULL DEFAULT 'New',
    assigned_to UUID REFERENCES persons(person_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE,
    resolution_notes VARCHAR(500)
);

-- Risk scoring for customers
CREATE TABLE customer_risk_scores (
    risk_score_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    overall_score DECIMAL(5,2) NOT NULL CHECK (overall_score >= 0 AND overall_score <= 100),
    kyc_score DECIMAL(5,2) CHECK (kyc_score >= 0 AND kyc_score <= 100),
    transaction_score DECIMAL(5,2) CHECK (transaction_score >= 0 AND transaction_score <= 100),
    geographical_score DECIMAL(5,2) CHECK (geographical_score >= 0 AND geographical_score <= 100),
    pep_score DECIMAL(5,2) CHECK (pep_score >= 0 AND pep_score <= 100), -- Politically Exposed Person
    sanctions_score DECIMAL(5,2) CHECK (sanctions_score >= 0 AND sanctions_score <= 100),
    calculated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    valid_until TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Compliance results table for check type tracking
CREATE TABLE compliance_results (
    result_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    check_type VARCHAR(50) NOT NULL CHECK (check_type IN (
        'Kyc', 'Aml', 'Cdd', 'Edd', 'SanctionsScreening', 'PepScreening', 
        'AdverseMediaScreening', 'WatchlistScreening', 'UboVerification', 
        'DocumentVerification', 'AddressVerification', 'SourceOfFundsVerification', 
        'SourceOfWealthVerification', 'RiskAssessment', 'OngoingMonitoring', 'PeriodicReview'
    )),
    status VARCHAR(50) NOT NULL CHECK (status IN ('Passed', 'Failed', 'RequiresReview', 'Pending')),
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0 AND risk_score <= 100),
    findings TEXT[], -- Array of findings (up to 300 chars each)
    recommendations TEXT[], -- Array of recommendations (up to 300 chars each)  
    checked_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Suspicious Activity Reports (SAR) - regulatory requirement
CREATE TABLE suspicious_activity_reports (
    sar_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(customer_id),
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for SAR reason  
    additional_details VARCHAR(500), -- Additional context for SAR
    supporting_transactions UUID[] NOT NULL, -- Array of transaction IDs
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    generated_by UUID NOT NULL REFERENCES persons(person_id),
    status VARCHAR(20) NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'Filed', 'Acknowledged')),
    filed_at TIMESTAMP WITH TIME ZONE,
    acknowledgment_received_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- FEE MANAGEMENT TABLES
-- =============================================================================

-- Fee applications tracking all fee charges
CREATE TABLE fee_applications (
    fee_application_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    transaction_id UUID REFERENCES transactions(transaction_id),
    fee_type VARCHAR(20) NOT NULL CHECK (fee_type IN ('EventBased', 'Periodic')),
    fee_category VARCHAR(30) NOT NULL CHECK (fee_category IN ('Transaction', 'Maintenance', 'Service', 'Penalty', 'Card', 'Loan', 'Regulatory')),
    product_code VARCHAR(50) NOT NULL,
    fee_code VARCHAR(50) NOT NULL,
    description VARCHAR(255) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    calculation_method VARCHAR(20) NOT NULL CHECK (calculation_method IN ('Fixed', 'Percentage', 'Tiered', 'BalanceBased', 'RuleBased')),
    calculation_base_amount DECIMAL(15,2),
    fee_rate DECIMAL(8,6),
    trigger_event VARCHAR(50) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Applied' CHECK (status IN ('Applied', 'Pending', 'Waived', 'Reversed', 'Failed')),
    applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    value_date DATE NOT NULL DEFAULT CURRENT_DATE,
    reversal_deadline TIMESTAMP WITH TIME ZONE,
    waived BOOLEAN NOT NULL DEFAULT FALSE,
    waived_by UUID REFERENCES persons(person_id),
    waived_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for waiver reason
    applied_by UUID NOT NULL REFERENCES persons(person_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_fee_amount_positive CHECK (amount > 0),
    CONSTRAINT ck_fee_rate_valid CHECK (fee_rate IS NULL OR (fee_rate >= 0 AND fee_rate <= 1)),
    CONSTRAINT ck_fee_waiver_consistency CHECK (
        (waived = FALSE) OR 
        (waived = TRUE AND waived_by IS NOT NULL AND waived_reason_id IS NOT NULL)
    )
);

-- Fee waivers for audit trail
CREATE TABLE fee_waivers (
    waiver_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fee_application_id UUID NOT NULL REFERENCES fee_applications(fee_application_id),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    waived_amount DECIMAL(15,2) NOT NULL CHECK (waived_amount > 0),
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for waiver reason
    additional_details VARCHAR(200), -- Additional context for waiver
    waived_by UUID NOT NULL REFERENCES persons(person_id),
    waived_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    approval_required BOOLEAN NOT NULL DEFAULT FALSE,
    approved_by UUID REFERENCES persons(person_id),
    approved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_waiver_approval CHECK (
        (approval_required = FALSE) OR 
        (approval_required = TRUE AND approved_by IS NOT NULL AND approved_at IS NOT NULL)
    )
);

-- Enhanced account holds
CREATE TABLE account_holds (
    hold_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id) ON DELETE CASCADE,
    amount DECIMAL(15,2) NOT NULL CHECK (amount > 0),
    hold_type hold_type NOT NULL,
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for hold reason
    additional_details VARCHAR(200), -- Additional context beyond the standard reason
    placed_by UUID NOT NULL REFERENCES persons(person_id),
    placed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    status hold_status NOT NULL DEFAULT 'Active',
    released_at TIMESTAMP WITH TIME ZONE,
    released_by UUID REFERENCES persons(person_id),
    priority hold_priority NOT NULL DEFAULT 'Medium',
    source_reference VARCHAR(100), -- External reference for judicial holds, etc.
    automatic_release BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_hold_release_consistency CHECK (
        (status NOT IN ('Released', 'PartiallyReleased') AND released_at IS NULL) OR
        (status IN ('Released', 'PartiallyReleased') AND released_at IS NOT NULL AND released_by IS NOT NULL)
    ),
    CONSTRAINT ck_hold_expiry_consistency CHECK (
        (automatic_release = FALSE AND expires_at IS NULL) OR
        (automatic_release = TRUE AND expires_at IS NOT NULL)
    )
);

-- =============================================================================
-- CASA (CURRENT & SAVINGS ACCOUNT) SPECIALIZED TABLES
-- =============================================================================

-- Overdraft facilities management
CREATE TABLE overdraft_facilities (
    facility_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    approved_limit DECIMAL(15,2) NOT NULL,
    current_utilized DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    available_limit DECIMAL(15,2) NOT NULL,
    interest_rate DECIMAL(8,6) NOT NULL, -- Annual debit interest rate
    facility_status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (facility_status IN ('Active', 'Suspended', 'Expired', 'UnderReview', 'Cancelled')),
    approval_date DATE NOT NULL,
    expiry_date DATE,
    approved_by UUID NOT NULL REFERENCES persons(person_id),
    review_frequency VARCHAR(20) NOT NULL CHECK (review_frequency IN ('Monthly', 'Quarterly', 'SemiAnnually', 'Annually')),
    next_review_date DATE NOT NULL,
    security_required BOOLEAN NOT NULL DEFAULT FALSE,
    security_details VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_overdraft_limits_positive CHECK (approved_limit > 0 AND current_utilized >= 0 AND available_limit >= 0),
    CONSTRAINT ck_overdraft_utilization_valid CHECK (current_utilized <= approved_limit),
    CONSTRAINT ck_overdraft_available_valid CHECK (available_limit = approved_limit - current_utilized),
    CONSTRAINT ck_overdraft_interest_rate_valid CHECK (interest_rate >= 0 AND interest_rate <= 1),
    CONSTRAINT ck_overdraft_review_date_valid CHECK (next_review_date > approval_date)
);

-- Interest posting records for CASA accounts
CREATE TABLE interest_posting_records (
    posting_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    posting_date DATE NOT NULL,
    interest_type VARCHAR(20) NOT NULL CHECK (interest_type IN ('CreditInterest', 'DebitInterest', 'PenaltyInterest')),
    period_start DATE NOT NULL,
    period_end DATE NOT NULL,
    principal_amount DECIMAL(15,2) NOT NULL,
    interest_rate DECIMAL(8,6) NOT NULL,
    days_calculated INTEGER NOT NULL,
    interest_amount DECIMAL(15,2) NOT NULL,
    tax_withheld DECIMAL(15,2),
    net_amount DECIMAL(15,2) NOT NULL,
    posting_status VARCHAR(20) NOT NULL CHECK (posting_status IN ('Calculated', 'Posted', 'Reversed', 'Adjusted')),
    posted_by UUID NOT NULL REFERENCES persons(person_id),
    posted_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_interest_period_valid CHECK (period_start <= period_end),
    CONSTRAINT ck_interest_days_valid CHECK (days_calculated > 0),
    CONSTRAINT ck_interest_rate_valid CHECK (interest_rate >= 0),
    CONSTRAINT ck_interest_amount_valid CHECK (interest_amount >= 0),
    CONSTRAINT ck_net_amount_valid CHECK (net_amount = interest_amount - COALESCE(tax_withheld, 0))
);

-- Overdraft limit adjustment requests
CREATE TABLE overdraft_limit_adjustments (
    adjustment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    current_limit DECIMAL(15,2) NOT NULL,
    requested_limit DECIMAL(15,2) NOT NULL,
    adjustment_reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    additional_details VARCHAR(200),
    supporting_documents TEXT[], -- Array of document references
    requested_by UUID NOT NULL REFERENCES persons(person_id),
    requested_at TIMESTAMP WITH TIME ZONE NOT NULL,
    approval_status VARCHAR(30) NOT NULL DEFAULT 'Pending' CHECK (approval_status IN ('Pending', 'Approved', 'Rejected', 'RequiresAdditionalDocuments', 'UnderReview')),
    approved_by UUID REFERENCES persons(person_id),
    approved_at TIMESTAMP WITH TIME ZONE,
    approval_notes VARCHAR(512),
    effective_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_limit_adjustment_valid CHECK (current_limit >= 0 AND requested_limit >= 0),
    CONSTRAINT ck_approval_consistency CHECK (
        (approval_status IN ('Approved', 'Rejected') AND approved_by IS NOT NULL AND approved_at IS NOT NULL) OR
        (approval_status NOT IN ('Approved', 'Rejected') AND approved_by IS NULL AND approved_at IS NULL)
    )
);

-- Overdraft interest calculations
CREATE TABLE overdraft_interest_calculations (
    calculation_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    calculation_period_start DATE NOT NULL,
    calculation_period_end DATE NOT NULL,
    average_overdrawn_balance DECIMAL(15,2) NOT NULL,
    applicable_rate DECIMAL(8,6) NOT NULL,
    days_calculated INTEGER NOT NULL,
    interest_amount DECIMAL(15,2) NOT NULL,
    compounding_frequency VARCHAR(20) NOT NULL CHECK (compounding_frequency IN ('Daily', 'Weekly', 'Monthly', 'Quarterly')),
    capitalization_due BOOLEAN NOT NULL DEFAULT FALSE,
    calculated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    calculated_by UUID NOT NULL REFERENCES persons(person_id),
    
    CONSTRAINT ck_calculation_period_valid CHECK (calculation_period_start <= calculation_period_end),
    CONSTRAINT ck_calculation_days_valid CHECK (days_calculated > 0),
    CONSTRAINT ck_interest_calculation_valid CHECK (interest_amount >= 0)
);

-- =============================================================================
-- CHANNEL MANAGEMENT TABLES
-- =============================================================================

-- Channels for transaction processing
CREATE TABLE channels (
    channel_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    channel_code VARCHAR(50) NOT NULL UNIQUE,
    channel_name VARCHAR(255) NOT NULL,
    channel_type VARCHAR(30) NOT NULL CHECK (channel_type IN ('BranchTeller', 'ATM', 'InternetBanking', 'MobileApp', 'AgentTerminal', 'USSD', 'ApiGateway')),
    status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Inactive', 'Maintenance', 'Suspended')),
    daily_limit DECIMAL(15,2),
    per_transaction_limit DECIMAL(15,2),
    supported_currencies VARCHAR(3)[] NOT NULL DEFAULT ARRAY['USD'], -- Array of 3-character currency codes
    requires_additional_auth BOOLEAN NOT NULL DEFAULT FALSE,
    fee_schedule_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_channel_limits CHECK (daily_limit IS NULL OR per_transaction_limit IS NULL OR daily_limit >= per_transaction_limit)
);

-- Fee schedules for channel-based fee management
CREATE TABLE fee_schedules (
    schedule_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    schedule_name VARCHAR(100) NOT NULL,
    channel_id UUID REFERENCES channels(channel_id),
    effective_date DATE NOT NULL,
    expiry_date DATE,
    currency VARCHAR(3) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_fee_schedule_dates CHECK (expiry_date IS NULL OR expiry_date > effective_date),
    CONSTRAINT ck_currency_format CHECK (LENGTH(currency) = 3 AND currency = UPPER(currency))
);

-- Fee items within fee schedules
CREATE TABLE fee_items (
    fee_item_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    schedule_id UUID NOT NULL REFERENCES fee_schedules(schedule_id) ON DELETE CASCADE,
    fee_code VARCHAR(20) NOT NULL,
    fee_name VARCHAR(100) NOT NULL,
    fee_type VARCHAR(30) NOT NULL CHECK (fee_type IN ('TransactionFee', 'MaintenanceFee', 'ServiceFee', 'PenaltyFee', 'ProcessingFee', 'ComplianceFee', 'InterchangeFee', 'NetworkFee')),
    calculation_method VARCHAR(20) NOT NULL CHECK (calculation_method IN ('Fixed', 'Percentage', 'Tiered', 'BalanceBased', 'RuleBased', 'Hybrid')),
    fee_amount DECIMAL(15,2),
    fee_percentage DECIMAL(8,6),
    minimum_fee DECIMAL(15,2),
    maximum_fee DECIMAL(15,2),
    applies_to_transaction_types VARCHAR(20)[] DEFAULT ARRAY[]::VARCHAR[], -- Array of transaction type codes
    is_waivable BOOLEAN NOT NULL DEFAULT FALSE,
    requires_approval_for_waiver BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_fee_item_amounts CHECK (
        (calculation_method = 'Fixed' AND fee_amount IS NOT NULL) OR
        (calculation_method = 'Percentage' AND fee_percentage IS NOT NULL) OR
        (calculation_method NOT IN ('Fixed', 'Percentage'))
    ),
    CONSTRAINT ck_fee_percentage_valid CHECK (fee_percentage IS NULL OR (fee_percentage >= 0 AND fee_percentage <= 1)),
    CONSTRAINT ck_fee_limits CHECK (minimum_fee IS NULL OR maximum_fee IS NULL OR maximum_fee >= minimum_fee)
);

-- Fee tiers for tiered pricing structures
CREATE TABLE fee_tiers (
    tier_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fee_item_id UUID NOT NULL REFERENCES fee_items(fee_item_id) ON DELETE CASCADE,
    tier_name VARCHAR(50) NOT NULL,
    min_amount DECIMAL(15,2) NOT NULL,
    max_amount DECIMAL(15,2),
    fee_amount DECIMAL(15,2),
    fee_percentage DECIMAL(8,6),
    tier_order INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_tier_amounts CHECK (max_amount IS NULL OR max_amount > min_amount),
    CONSTRAINT ck_tier_fee_defined CHECK (fee_amount IS NOT NULL OR fee_percentage IS NOT NULL),
    CONSTRAINT ck_tier_percentage_valid CHECK (fee_percentage IS NULL OR (fee_percentage >= 0 AND fee_percentage <= 1))
);

-- Channel reconciliation reports
CREATE TABLE channel_reconciliation_reports (
    report_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    channel_id UUID NOT NULL REFERENCES channels(channel_id),
    reconciliation_date DATE NOT NULL,
    total_transactions BIGINT NOT NULL DEFAULT 0,
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    status VARCHAR(30) NOT NULL DEFAULT 'InProgress' CHECK (status IN ('InProgress', 'Completed', 'Failed', 'RequiresManualReview')),
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Reconciliation discrepancies
CREATE TABLE reconciliation_discrepancies (
    discrepancy_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES channel_reconciliation_reports(report_id) ON DELETE CASCADE,
    transaction_id UUID NOT NULL REFERENCES transactions(transaction_id),
    description VARCHAR(500) NOT NULL,
    expected_amount DECIMAL(15,2) NOT NULL,
    actual_amount DECIMAL(15,2) NOT NULL,
    difference DECIMAL(15,2) NOT NULL,
    resolved BOOLEAN NOT NULL DEFAULT FALSE,
    resolution_notes VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_discrepancy_difference CHECK (difference = expected_amount - actual_amount)
);

-- Add foreign key constraint for channels fee_schedule_id after fee_schedules table is created
ALTER TABLE channels ADD CONSTRAINT fk_channels_fee_schedule 
    FOREIGN KEY (fee_schedule_id) REFERENCES fee_schedules(schedule_id);

-- Indexes for channel tables
CREATE INDEX idx_channels_code ON channels(channel_code);
CREATE INDEX idx_channels_type ON channels(channel_type);
CREATE INDEX idx_channels_status ON channels(status);
CREATE INDEX idx_fee_schedules_channel ON fee_schedules(channel_id);
CREATE INDEX idx_fee_schedules_active ON fee_schedules(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_fee_schedules_effective ON fee_schedules(effective_date);
CREATE INDEX idx_fee_items_schedule ON fee_items(schedule_id);
CREATE INDEX idx_fee_items_code ON fee_items(fee_code);
CREATE INDEX idx_fee_items_type ON fee_items(fee_type);
CREATE INDEX idx_fee_tiers_item ON fee_tiers(fee_item_id);
CREATE INDEX idx_fee_tiers_order ON fee_tiers(tier_order);
CREATE INDEX idx_reconciliation_reports_channel ON channel_reconciliation_reports(channel_id);
CREATE INDEX idx_reconciliation_reports_date ON channel_reconciliation_reports(reconciliation_date);
CREATE INDEX idx_reconciliation_discrepancies_report ON reconciliation_discrepancies(report_id);
CREATE INDEX idx_reconciliation_discrepancies_transaction ON reconciliation_discrepancies(transaction_id);

-- Triggers for updated_at timestamps
CREATE TRIGGER update_channels_updated_at
    BEFORE UPDATE ON channels
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_fee_schedules_updated_at
    BEFORE UPDATE ON fee_schedules
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- LOAN SPECIALIZED TABLES
-- =============================================================================

-- Collection actions for delinquent loans
CREATE TABLE collection_actions (
    action_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    delinquency_id UUID NOT NULL,
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    action_type VARCHAR(30) NOT NULL CHECK (action_type IN ('EmailReminder', 'SmsNotification', 'PhoneCall', 'LetterNotice', 'LegalNotice', 'FieldVisit', 'PaymentPlan', 'Restructuring', 'LegalAction', 'AssetRecovery')),
    action_date DATE NOT NULL,
    due_date DATE,
    description VARCHAR(500) NOT NULL,
    amount_demanded DECIMAL(15,2),
    response_received BOOLEAN NOT NULL DEFAULT FALSE,
    response_date DATE,
    response_details VARCHAR(1000),
    follow_up_required BOOLEAN NOT NULL DEFAULT FALSE,
    follow_up_date DATE,
    action_status VARCHAR(20) NOT NULL CHECK (action_status IN ('Planned', 'InProgress', 'Completed', 'Failed', 'Cancelled')),
    assigned_to UUID NOT NULL REFERENCES persons(person_id),
    created_by UUID NOT NULL REFERENCES persons(person_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_response_consistency CHECK (
        (response_received = FALSE AND response_date IS NULL) OR
        (response_received = TRUE AND response_date IS NOT NULL)
    )
);

-- Loan payments and allocations
CREATE TABLE loan_payments (
    payment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    payment_date DATE NOT NULL,
    payment_amount DECIMAL(15,2) NOT NULL CHECK (payment_amount > 0),
    payment_type VARCHAR(20) NOT NULL CHECK (payment_type IN ('Regular', 'Prepayment', 'PartialPayment', 'Restructured', 'Settlement', 'Recovery')),
    payment_method VARCHAR(30) NOT NULL CHECK (payment_method IN ('BankTransfer', 'DirectDebit', 'Check', 'Cash', 'OnlinePayment', 'MobilePayment', 'StandingInstruction')),
    payment_status VARCHAR(20) NOT NULL CHECK (payment_status IN ('Processed', 'Pending', 'Failed', 'Reversed', 'PartiallyAllocated')),
    external_reference VARCHAR(100),
    processed_by UUID NOT NULL REFERENCES persons(person_id),
    processed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    reversal_info_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Payment allocations across loan components
CREATE TABLE payment_allocations (
    allocation_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_id UUID NOT NULL REFERENCES loan_payments(payment_id),
    penalty_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    overdue_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    current_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    principal_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    fees_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    charges_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    excess_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Payment reversals
CREATE TABLE payment_reversals (
    reversal_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    original_payment_id UUID NOT NULL REFERENCES loan_payments(payment_id),
    reversal_reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    additional_details VARCHAR(200),
    reversed_amount DECIMAL(15,2) NOT NULL CHECK (reversed_amount > 0),
    reversed_by UUID NOT NULL REFERENCES persons(person_id),
    reversed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    schedule_adjusted BOOLEAN NOT NULL DEFAULT FALSE
);

-- Loan restructuring records
CREATE TABLE loan_restructurings (
    restructuring_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    restructuring_type VARCHAR(30) NOT NULL CHECK (restructuring_type IN ('PaymentHoliday', 'TermExtension', 'RateReduction', 'PrincipalReduction', 'InstallmentReduction', 'InterestCapitalization', 'FullRestructuring')),
    request_date DATE NOT NULL,
    effective_date DATE,
    restructuring_reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    additional_details VARCHAR(200),
    -- Original loan terms
    original_principal DECIMAL(15,2) NOT NULL,
    original_interest_rate DECIMAL(8,6) NOT NULL,
    original_term_months INTEGER NOT NULL,
    original_installment DECIMAL(15,2) NOT NULL,
    -- New loan terms
    new_principal DECIMAL(15,2),
    new_interest_rate DECIMAL(8,6),
    new_term_months INTEGER,
    new_installment DECIMAL(15,2),
    new_maturity_date DATE,
    -- Restructuring details
    moratorium_period INTEGER, -- Months
    capitalized_interest DECIMAL(15,2),
    waived_penalty_amount DECIMAL(15,2),
    approval_status VARCHAR(30) NOT NULL CHECK (approval_status IN ('Pending', 'Approved', 'Rejected', 'ConditionallyApproved', 'RequiresCommitteeApproval')),
    approved_by UUID REFERENCES persons(person_id),
    approved_at TIMESTAMP WITH TIME ZONE,
    conditions TEXT[], -- Array of conditions
    created_by UUID NOT NULL REFERENCES persons(person_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_approval_consistency CHECK (
        (approval_status IN ('Approved', 'Rejected') AND approved_by IS NOT NULL AND approved_at IS NOT NULL) OR
        (approval_status NOT IN ('Approved', 'Rejected') AND approved_by IS NULL AND approved_at IS NULL)
    )
);

-- =============================================================================
-- TRIGGERS AND FUNCTIONS
-- =============================================================================

-- Generic audit trail function
CREATE OR REPLACE FUNCTION log_audit_trail()
RETURNS TRIGGER AS $$
BEGIN
    -- Only log if this is an actual change (not just timestamp update)
    IF TG_OP = 'UPDATE' AND OLD IS NOT DISTINCT FROM NEW THEN
        RETURN NEW;
    END IF;

    -- Log the change based on table
    CASE TG_TABLE_NAME
        WHEN 'customers' THEN
            IF TG_OP = 'UPDATE' THEN
                -- Log significant field changes
                IF OLD.risk_rating != NEW.risk_rating THEN
                    INSERT INTO customer_audit_trail (audit_id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason)
                    VALUES (uuid_generate_v4(), NEW.customer_id, 'risk_rating', OLD.risk_rating, NEW.risk_rating, NOW(), NEW.updated_by, 'Risk rating change');
                END IF;
                
                IF OLD.status != NEW.status THEN
                    INSERT INTO customer_audit_trail (audit_id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason)
                    VALUES (uuid_generate_v4(), NEW.customer_id, 'status', OLD.status, NEW.status, NOW(), NEW.updated_by, 'Status change');
                END IF;
            END IF;
        ELSE
            -- Generic handling for other tables
            NULL;
    END CASE;

    -- Update last_updated_at for tables that have it
    IF TG_OP = 'UPDATE' THEN
        IF EXISTS (SELECT 1 FROM information_schema.columns 
                  WHERE table_name = TG_TABLE_NAME AND column_name = 'last_updated_at') THEN
            NEW.last_updated_at = NOW();
        END IF;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply audit triggers to core tables
CREATE TRIGGER customers_audit_trigger
    BEFORE UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION log_audit_trail();

CREATE TRIGGER accounts_audit_trigger
    BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION log_audit_trail();

-- Function to automatically update the updated_at timestamp for reason_and_purpose
CREATE OR REPLACE FUNCTION update_reason_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reason_and_purpose_updated_at
    BEFORE UPDATE ON reason_and_purpose
    FOR EACH ROW
    EXECUTE FUNCTION update_reason_updated_at();

-- =============================================================================
-- SEED DATA: SYSTEM REFERENCED PERSONS
-- =============================================================================

-- Insert system persons
INSERT INTO persons (person_id, person_type, display_name, external_identifier, organization, is_active)
VALUES 
    ('00000000-0000-0000-0000-000000000000', 'system', 'SYSTEM', 'SYSTEM', 'Banking System', TRUE),
    ('00000000-0000-0000-0000-000000000001', 'system', 'MIGRATION', 'MIGRATION', 'Data Migration', TRUE),
    ('00000000-0000-0000-0000-000000000002', 'integration', 'API_INTEGRATION', 'API', 'External API', TRUE),
    ('00000000-0000-0000-0000-000000000003', 'system', 'BATCH_PROCESSOR', 'BATCH', 'Batch Processing', TRUE);

-- =============================================================================
-- SEED DATA: REASON AND PURPOSE ENTRIES
-- =============================================================================

-- Insert initial seed data for reasons and purposes
INSERT INTO reason_and_purpose (
    id, code, category, context, 
    l1_content, l2_content, l3_content,
    l1_language_code, l2_language_code, l3_language_code,
    requires_details, is_active, severity, display_order,
    created_by, updated_by
) VALUES 
-- Loan Purposes
(
    gen_random_uuid(),
    'LOAN_PURPOSE_HOME_PURCHASE',
    'LoanPurpose',
    'Loan',
    'Home Purchase',
    'Achat de Maison', 
    'Kununua Nyumba',
    'eng', 'fra', 'swa',
    FALSE, TRUE, NULL, 1,
    'system', 'system'
),
(
    gen_random_uuid(),
    'LOAN_PURPOSE_BUSINESS_EXPANSION',
    'LoanPurpose',
    'Loan',
    'Business Expansion',
    'Expansion d''Entreprise',
    'Upanuzi wa Biashara',
    'eng', 'fra', 'swa',
    TRUE, TRUE, NULL, 2,
    'system', 'system'
),
(
    gen_random_uuid(),
    'LOAN_PURPOSE_EDUCATION',
    'LoanPurpose',
    'Loan',
    'Education',
    'Éducation',
    'Elimu',
    'eng', 'fra', 'swa',
    FALSE, TRUE, NULL, 3,
    'system', 'system'
),

-- Account Closure Reasons
(
    gen_random_uuid(),
    'CLOSURE_CUSTOMER_REQUEST',
    'AccountClosure',
    'Account',
    'Customer Request',
    'Demande du Client',
    'Ombi la Mteja',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'Low', 1,
    'system', 'system'
),
(
    gen_random_uuid(),
    'CLOSURE_INACTIVITY',
    'AccountClosure',
    'Account',
    'Account Inactivity',
    'Inactivité du Compte',
    'Kutotumika kwa Akaunti',
    'eng', 'fra', 'swa',
    FALSE, TRUE, 'Low', 2,
    'system', 'system'
),
(
    gen_random_uuid(),
    'CLOSURE_COMPLIANCE_ISSUE',
    'AccountClosure',
    'Account',
    'Compliance Issue',
    'Problème de Conformité',
    'Suala la Utiifu',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'High', 3,
    'system', 'system'
),

-- AML/CTF Reasons  
(
    gen_random_uuid(),
    'AML_SUSPICIOUS_PATTERN',
    'AmlAlert', 
    'AmlCtf',
    'Suspicious transaction pattern detected',
    'Modèle de transaction suspect détecté',
    'Mfumo wa shughuli za kutiliwa shaka umegundulika',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'High', 1,
    'system', 'system'
),
(
    gen_random_uuid(),
    'CTF_SANCTIONS_MATCH',
    'SanctionsHit',
    'AmlCtf', 
    'Name matches sanctions list',
    'Le nom correspond à la liste des sanctions',
    'Jina linafanana na orodha ya vikwazo',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'Critical', 1,
    'system', 'system'
),
(
    gen_random_uuid(),
    'AML_HIGH_RISK_COUNTRY',
    'HighRiskCountry',
    'AmlCtf',
    'Transaction involves high-risk jurisdiction',
    'Transaction implique une juridiction à haut risque',
    'Muamala unahusisha eneo lenye hatari kubwa',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'High', 2,
    'system', 'system'
),

-- KYC Reasons
(
    gen_random_uuid(),
    'KYC_ID_EXPIRED',
    'KycMissingDocument',
    'Kyc',
    'National ID has expired',
    'La carte d''identité nationale a expiré',
    'Kitambulisho cha taifa kimepita muda',
    'eng', 'fra', 'swa',
    FALSE, TRUE, 'Medium', 1,
    'system', 'system'
),
(
    gen_random_uuid(),
    'KYC_PROOF_OF_ADDRESS_MISSING',
    'KycMissingDocument',
    'Kyc',
    'Proof of address document required',
    'Justificatif de domicile requis',
    'Uthibitisho wa makazi unahitajika',
    'eng', 'fra', 'swa',
    FALSE, TRUE, 'Medium', 2,
    'system', 'system'
),
(
    gen_random_uuid(),
    'KYC_SOURCE_OF_FUNDS_UNCLEAR',
    'SourceOfFundsRequired',
    'Kyc',
    'Source of funds documentation required',
    'Documentation sur l''origine des fonds requise',
    'Hati za chanzo cha fedha zinahitajika',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'High', 1,
    'system', 'system'
),

-- Transaction Hold Reasons
(
    gen_random_uuid(),
    'HOLD_INSUFFICIENT_FUNDS',
    'HoldReason',
    'Transaction',
    'Insufficient funds',
    'Fonds insuffisants',
    'Fedha hazitoshi',
    'eng', 'fra', 'swa',
    FALSE, TRUE, 'Medium', 1,
    'system', 'system'
),
(
    gen_random_uuid(),
    'HOLD_FRAUD_INVESTIGATION',
    'HoldReason',
    'Transaction',
    'Under fraud investigation',
    'Enquête de fraude en cours',
    'Kuna uchunguzi wa udanganyifu',
    'eng', 'fra', 'swa',
    TRUE, TRUE, 'High', 2,
    'system', 'system'
),

-- System Generated Reasons
(
    gen_random_uuid(),
    'SYSTEM_AUTO_DORMANCY',
    'SystemGenerated',
    'System',
    'Automatic dormancy due to inactivity',
    'Dormance automatique due à l''inactivité',
    'Usingizi wa otomatiki kwa kutotumika',
    'eng', 'fra', 'swa',
    FALSE, TRUE, 'Low', 1,
    'system', 'system'
);

-- Update compliance metadata for specific reasons
UPDATE reason_and_purpose 
SET 
    regulatory_code = 'FATF-R.16',
    reportable = TRUE,
    requires_sar = TRUE,
    retention_years = 7,
    escalation_required = TRUE,
    risk_score_impact = 25,
    no_tipping_off = TRUE,
    jurisdictions = ARRAY['CM']
WHERE code = 'AML_SUSPICIOUS_PATTERN';

UPDATE reason_and_purpose
SET
    regulatory_code = 'UN-RES-1267',
    reportable = TRUE,
    requires_sar = TRUE,
    retention_years = 10,
    escalation_required = TRUE,
    risk_score_impact = 100,
    no_tipping_off = TRUE,
    jurisdictions = ARRAY['CM', 'US', 'EU']
WHERE code = 'CTF_SANCTIONS_MATCH';

UPDATE reason_and_purpose
SET
    regulatory_code = 'COMP-01',
    reportable = TRUE,
    retention_years = 7,
    escalation_required = TRUE,
    risk_score_impact = 50
WHERE code = 'CLOSURE_COMPLIANCE_ISSUE';

UPDATE reason_and_purpose
SET
    regulatory_code = 'FATF-HIGH-RISK',
    reportable = TRUE,
    retention_years = 7,
    risk_score_impact = 40
WHERE code = 'AML_HIGH_RISK_COUNTRY';

UPDATE reason_and_purpose
SET
    regulatory_code = 'KYC-DOC-01',
    retention_years = 5,
    risk_score_impact = 10
WHERE code = 'KYC_ID_EXPIRED';

UPDATE reason_and_purpose
SET
    regulatory_code = 'KYC-DOC-02',
    retention_years = 5,
    risk_score_impact = 10
WHERE code = 'KYC_PROOF_OF_ADDRESS_MISSING';

UPDATE reason_and_purpose
SET
    regulatory_code = 'KYC-SOF-01',
    reportable = TRUE,
    retention_years = 7,
    escalation_required = TRUE,
    risk_score_impact = 30
WHERE code = 'KYC_SOURCE_OF_FUNDS_UNCLEAR';

UPDATE reason_and_purpose
SET
    regulatory_code = 'FRAUD-01',
    reportable = TRUE,
    retention_years = 7,
    escalation_required = TRUE,
    risk_score_impact = 75,
    no_tipping_off = TRUE
WHERE code = 'HOLD_FRAUD_INVESTIGATION';

-- =============================================================================
-- SAMPLE SEED DATA
-- =============================================================================

-- Bank holidays for multiple jurisdictions
INSERT INTO bank_holidays (holiday_id, jurisdiction, holiday_date, holiday_name, holiday_type, is_recurring, created_by) VALUES
(uuid_generate_v4(), 'US', '2024-01-01', 'New Year''s Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-01-15', 'Martin Luther King Jr. Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-02-19', 'Presidents Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-05-27', 'Memorial Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-06-19', 'Juneteenth', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-07-04', 'Independence Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-09-02', 'Labor Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-10-14', 'Columbus Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-11-11', 'Veterans Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-11-28', 'Thanksgiving Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'US', '2024-12-25', 'Christmas Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'CM', '2024-01-01', 'New Year''s Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'CM', '2024-02-11', 'Youth Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'CM', '2024-05-20', 'National Day', 'National', true, '00000000-0000-0000-0000-000000000000'),
(uuid_generate_v4(), 'CM', '2024-12-25', 'Christmas Day', 'Religious', true, '00000000-0000-0000-0000-000000000000');

-- Weekend configuration
INSERT INTO weekend_configuration (config_id, jurisdiction, weekend_days, effective_date, created_by, updated_by) VALUES
(uuid_generate_v4(), 'US', '[6,7]', '2024-01-01', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'), -- Saturday, Sunday
(uuid_generate_v4(), 'CM', '[6,7]', '2024-01-01', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'), -- Saturday, Sunday
(uuid_generate_v4(), 'AE', '[5,6]', '2024-01-01', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'); -- Friday, Saturday (UAE style)

-- =============================================================================
-- INDEXES FOR PERFORMANCE
-- =============================================================================

-- Customer table indexes
CREATE INDEX idx_customers_risk_rating ON customers(risk_rating);
CREATE INDEX idx_customers_status ON customers(status);
CREATE INDEX idx_customers_updated_by ON customers(updated_by);

-- Account table indexes
CREATE INDEX idx_accounts_customer_product ON accounts(product_code);
CREATE INDEX idx_accounts_status ON accounts(account_status);
CREATE INDEX idx_accounts_type ON accounts(account_type);
CREATE INDEX idx_accounts_branch ON accounts(domicile_branch_id);
CREATE INDEX idx_accounts_balance ON accounts(current_balance);
CREATE INDEX idx_accounts_loan_maturity ON accounts(maturity_date) WHERE account_type = 'Loan';

-- Transaction table indexes
CREATE INDEX idx_transactions_account ON transactions(account_id);
CREATE INDEX idx_transactions_date ON transactions(transaction_date);
CREATE INDEX idx_transactions_value_date ON transactions(value_date);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_channel ON transactions(channel_id);
CREATE INDEX idx_transactions_amount ON transactions(amount);
CREATE INDEX idx_transactions_reference ON transactions(reference_number);

-- Agent network indexes
CREATE INDEX idx_agent_networks_status ON agent_networks(status);
CREATE INDEX idx_agent_branches_network ON agent_branches(network_id);
CREATE INDEX idx_agent_terminals_branch ON agent_terminals(branch_id);

-- Compliance and risk indexes
CREATE INDEX idx_kyc_results_customer ON kyc_results(customer_id);
CREATE INDEX idx_kyc_results_status ON kyc_results(status);
CREATE INDEX idx_compliance_alerts_status ON compliance_alerts(status);
CREATE INDEX idx_compliance_alerts_severity ON compliance_alerts(severity);

-- Calendar table indexes
CREATE INDEX idx_bank_holidays_jurisdiction ON bank_holidays(jurisdiction);
CREATE INDEX idx_bank_holidays_date ON bank_holidays(holiday_date);
CREATE INDEX idx_bank_holidays_is_recurring ON bank_holidays(is_recurring);
CREATE INDEX idx_weekend_configuration_jurisdiction ON weekend_configuration(jurisdiction);
CREATE INDEX idx_date_calculation_rules_jurisdiction ON date_calculation_rules(jurisdiction);
CREATE INDEX idx_date_calculation_rules_active ON date_calculation_rules(is_active) WHERE is_active = TRUE;
CREATE INDEX idx_holiday_import_log_jurisdiction ON holiday_import_log(jurisdiction);
CREATE INDEX idx_holiday_import_log_year ON holiday_import_log(import_year);
CREATE INDEX idx_business_day_cache_jurisdiction_date ON business_day_cache(jurisdiction, date);
CREATE INDEX idx_business_day_cache_valid_until ON business_day_cache(valid_until);

-- =============================================================================
-- COMMENTS FOR DOCUMENTATION
-- =============================================================================

-- Referenced persons
COMMENT ON TABLE persons IS 'Stores all person references used throughout the system for audit and tracking';
COMMENT ON COLUMN persons.person_id IS 'Unique identifier for this person reference';
COMMENT ON COLUMN persons.person_type IS 'Type of person: natural (human), legal (company), system, integration, unknown';
COMMENT ON COLUMN persons.display_name IS 'Display name of the person';
COMMENT ON COLUMN persons.external_identifier IS 'External ID like employee number, badge ID, system ID';
COMMENT ON COLUMN persons.organization IS 'Organization or department for employees or company name for legal entities';
COMMENT ON COLUMN persons.email IS 'Email address for contact purposes';
COMMENT ON COLUMN persons.phone IS 'Phone number for contact purposes';
COMMENT ON COLUMN persons.department IS 'Department within organization';
COMMENT ON COLUMN persons.office_location IS 'Office location or address (up to 200 characters)';
COMMENT ON COLUMN persons.duplicate_of IS 'Reference to another person record if this is a duplicate';
COMMENT ON COLUMN persons.entity_reference IS 'Reference to related entity (customer_id, employee_id, etc.)';
COMMENT ON COLUMN persons.entity_type IS 'Type of entity referenced (customer, employee, shareholder, etc.)';
COMMENT ON COLUMN persons.is_active IS 'Whether this person reference is currently active';

-- Reason and purpose
COMMENT ON TABLE reason_and_purpose IS 'Centralized table for all reasons and purposes with multilingual support';
COMMENT ON COLUMN reason_and_purpose.code IS 'Unique programmatic identifier for the reason';
COMMENT ON COLUMN reason_and_purpose.category IS 'Category enum: LoanPurpose, AccountClosure, AmlAlert, etc.';
COMMENT ON COLUMN reason_and_purpose.context IS 'Context enum: Account, Loan, Transaction, Compliance, etc.';
COMMENT ON COLUMN reason_and_purpose.l1_content IS 'Primary language content';
COMMENT ON COLUMN reason_and_purpose.l2_content IS 'Secondary language content';
COMMENT ON COLUMN reason_and_purpose.l3_content IS 'Tertiary language content';
COMMENT ON COLUMN reason_and_purpose.requires_details IS 'Whether this reason requires additional details';

-- Core tables
COMMENT ON TABLE customers IS 'Master customer information file with full audit trail';
COMMENT ON TABLE accounts IS 'Unified account model supporting all banking products (CASA, Loans)';
COMMENT ON TABLE transactions IS 'Comprehensive transaction processing with multi-channel support';
COMMENT ON TABLE agent_networks IS 'Hierarchical agent banking network structure';

-- Calendar tables
COMMENT ON TABLE bank_holidays IS 'Bank holidays for business day calculations across jurisdictions';
COMMENT ON TABLE weekend_configuration IS 'Weekend configuration per jurisdiction with JSON array format';
COMMENT ON TABLE date_calculation_rules IS 'Date calculation rules for business day adjustments';
COMMENT ON TABLE holiday_import_log IS 'Audit trail for holiday data imports';
COMMENT ON TABLE business_day_cache IS 'Performance optimization cache for business day calculations';

-- =============================================================================
-- MIGRATION COMPLETE
-- =============================================================================

-- Migration completed successfully
-- Total tables created: 38
-- Total indexes created: 40+
-- Foreign key constraints: 50+
-- Check constraints: 50+
-- Triggers: 4
-- Functions: 3
-- Initial seed data: System persons, reasons, holidays, weekend config