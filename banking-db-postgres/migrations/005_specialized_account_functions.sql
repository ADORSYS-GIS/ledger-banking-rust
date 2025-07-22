-- Specialized Account Functions: CASA and Loan Servicing
-- Section 4.0 implementation - Current/Savings and Loan Account specialized functionality

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
    approved_by VARCHAR(100) NOT NULL,
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
    CONSTRAINT ck_overdraft_expiry_valid CHECK (expiry_date IS NULL OR expiry_date > approval_date),
    CONSTRAINT uk_overdraft_account UNIQUE (account_id)
);

-- Daily overdraft utilization tracking for interest calculation
CREATE TABLE overdraft_utilization (
    utilization_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    utilization_date DATE NOT NULL,
    opening_balance DECIMAL(15,2) NOT NULL, -- Negative for overdrawn
    closing_balance DECIMAL(15,2) NOT NULL, -- Negative for overdrawn
    average_daily_balance DECIMAL(15,2) NOT NULL,
    days_overdrawn INTEGER NOT NULL DEFAULT 0,
    interest_accrued DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    interest_rate DECIMAL(8,6) NOT NULL,
    capitalized BOOLEAN NOT NULL DEFAULT FALSE,
    capitalization_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_overdraft_utilization_days CHECK (days_overdrawn >= 0 AND days_overdrawn <= 1),
    CONSTRAINT ck_overdraft_capitalization_date CHECK (
        (capitalized = TRUE AND capitalization_date IS NOT NULL) OR
        (capitalized = FALSE AND capitalization_date IS NULL)
    ),
    CONSTRAINT uk_overdraft_utilization_date UNIQUE (account_id, utilization_date)
);

-- Overdraft interest calculation results
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
    calculated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    calculated_by VARCHAR(100) NOT NULL,
    
    CONSTRAINT ck_overdraft_calc_period_valid CHECK (calculation_period_end >= calculation_period_start),
    CONSTRAINT ck_overdraft_calc_days_valid CHECK (days_calculated > 0)
);

-- Overdraft limit adjustment requests
CREATE TABLE overdraft_limit_adjustments (
    adjustment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    current_limit DECIMAL(15,2) NOT NULL,
    requested_limit DECIMAL(15,2) NOT NULL,
    adjustment_reason VARCHAR(255) NOT NULL,
    supporting_documents TEXT, -- JSON array of document references
    requested_by VARCHAR(100) NOT NULL,
    requested_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    approval_status VARCHAR(30) NOT NULL DEFAULT 'Pending' CHECK (approval_status IN ('Pending', 'Approved', 'Rejected', 'RequiresAdditionalDocuments', 'UnderReview')),
    approved_by VARCHAR(100),
    approved_at TIMESTAMP WITH TIME ZONE,
    approval_notes VARCHAR(500),
    effective_date DATE,
    
    CONSTRAINT ck_overdraft_adjustment_limits CHECK (current_limit >= 0 AND requested_limit >= 0),
    CONSTRAINT ck_overdraft_adjustment_approval CHECK (
        (approval_status IN ('Approved', 'Rejected') AND approved_by IS NOT NULL AND approved_at IS NOT NULL) OR
        (approval_status NOT IN ('Approved', 'Rejected'))
    )
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
    posting_status VARCHAR(20) NOT NULL DEFAULT 'Posted' CHECK (posting_status IN ('Calculated', 'Posted', 'Reversed', 'Adjusted')),
    posted_by VARCHAR(100) NOT NULL,
    posted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_interest_posting_period CHECK (period_end >= period_start),
    CONSTRAINT ck_interest_posting_days CHECK (days_calculated > 0),
    CONSTRAINT ck_interest_posting_net CHECK (net_amount = interest_amount - COALESCE(tax_withheld, 0))
);

-- Daily overdraft processing jobs
CREATE TABLE overdraft_processing_jobs (
    job_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    processing_date DATE NOT NULL,
    accounts_processed INTEGER NOT NULL DEFAULT 0,
    total_interest_accrued DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    accounts_capitalized INTEGER NOT NULL DEFAULT 0,
    total_capitalized_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    status VARCHAR(30) NOT NULL DEFAULT 'Scheduled' CHECK (status IN ('Scheduled', 'Running', 'Completed', 'Failed', 'PartiallyCompleted')),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    errors TEXT, -- JSON array of error messages
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_overdraft_job_counts CHECK (accounts_processed >= 0 AND accounts_capitalized >= 0),
    CONSTRAINT ck_overdraft_job_completion CHECK (
        (status = 'Completed' AND completed_at IS NOT NULL) OR
        (status != 'Completed')
    )
);

-- =============================================================================
-- LOAN SERVICING SPECIALIZED TABLES
-- =============================================================================

-- Amortization schedules for loans
CREATE TABLE amortization_schedules (
    schedule_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    original_principal DECIMAL(15,2) NOT NULL,
    interest_rate DECIMAL(8,6) NOT NULL,
    term_months INTEGER NOT NULL,
    installment_amount DECIMAL(15,2) NOT NULL,
    first_payment_date DATE NOT NULL,
    maturity_date DATE NOT NULL,
    total_interest DECIMAL(15,2) NOT NULL,
    total_payments DECIMAL(15,2) NOT NULL,
    payment_frequency VARCHAR(20) NOT NULL DEFAULT 'Monthly' CHECK (payment_frequency IN ('Weekly', 'BiWeekly', 'Monthly', 'Quarterly', 'SemiAnnually', 'Annually')),
    calculation_method VARCHAR(20) NOT NULL DEFAULT 'EqualInstallments' CHECK (calculation_method IN ('EqualInstallments', 'EqualPrincipal', 'InterestOnly', 'BulletPayment')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_amortization_amounts CHECK (
        original_principal > 0 AND 
        installment_amount > 0 AND 
        total_interest >= 0 AND 
        total_payments >= original_principal
    ),
    CONSTRAINT ck_amortization_term CHECK (term_months > 0),
    CONSTRAINT ck_amortization_dates CHECK (maturity_date > first_payment_date),
    CONSTRAINT uk_amortization_loan UNIQUE (loan_account_id)
);

-- Individual installment entries in amortization schedule
CREATE TABLE amortization_entries (
    entry_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    schedule_id UUID NOT NULL REFERENCES amortization_schedules(schedule_id),
    installment_number INTEGER NOT NULL,
    due_date DATE NOT NULL,
    opening_principal_balance DECIMAL(15,2) NOT NULL,
    installment_amount DECIMAL(15,2) NOT NULL,
    principal_component DECIMAL(15,2) NOT NULL,
    interest_component DECIMAL(15,2) NOT NULL,
    closing_principal_balance DECIMAL(15,2) NOT NULL,
    cumulative_principal_paid DECIMAL(15,2) NOT NULL,
    cumulative_interest_paid DECIMAL(15,2) NOT NULL,
    payment_status VARCHAR(20) NOT NULL DEFAULT 'Scheduled' CHECK (payment_status IN ('Scheduled', 'Due', 'PartiallyPaid', 'Paid', 'Overdue', 'WriteOff')),
    paid_date DATE,
    paid_amount DECIMAL(15,2),
    days_overdue INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_amortization_entry_amounts CHECK (
        opening_principal_balance >= 0 AND 
        closing_principal_balance >= 0 AND
        principal_component >= 0 AND 
        interest_component >= 0 AND
        ABS(installment_amount - (principal_component + interest_component)) < 0.01
    ),
    CONSTRAINT ck_amortization_entry_installment CHECK (installment_number > 0),
    CONSTRAINT ck_amortization_entry_payment CHECK (
        (payment_status IN ('Paid', 'PartiallyPaid') AND paid_date IS NOT NULL AND paid_amount IS NOT NULL) OR
        (payment_status NOT IN ('Paid', 'PartiallyPaid'))
    ),
    CONSTRAINT uk_amortization_entry UNIQUE (schedule_id, installment_number)
);

-- Loan delinquency tracking
CREATE TABLE loan_delinquencies (
    delinquency_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    delinquency_start_date DATE NOT NULL,
    current_dpd INTEGER NOT NULL DEFAULT 0, -- Days Past Due
    highest_dpd INTEGER NOT NULL DEFAULT 0,
    delinquency_stage VARCHAR(20) NOT NULL DEFAULT 'Current' CHECK (delinquency_stage IN ('Current', 'Stage1', 'Stage2', 'Stage3', 'NonPerforming', 'WriteOff')),
    overdue_principal DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    overdue_interest DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    penalty_interest_accrued DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    total_overdue_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    last_payment_date DATE,
    last_payment_amount DECIMAL(15,2),
    restructuring_eligibility BOOLEAN NOT NULL DEFAULT TRUE,
    npl_date DATE, -- Non-Performing Loan classification date
    provisioning_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_loan_delinquency_dpd CHECK (current_dpd >= 0 AND highest_dpd >= current_dpd),
    CONSTRAINT ck_loan_delinquency_amounts CHECK (
        overdue_principal >= 0 AND 
        overdue_interest >= 0 AND 
        penalty_interest_accrued >= 0 AND
        total_overdue_amount = overdue_principal + overdue_interest + penalty_interest_accrued
    ),
    CONSTRAINT ck_loan_delinquency_npl CHECK (
        (delinquency_stage = 'NonPerforming' AND npl_date IS NOT NULL) OR
        (delinquency_stage != 'NonPerforming')
    ),
    CONSTRAINT uk_loan_delinquency_account UNIQUE (loan_account_id)
);

-- Collection actions for delinquent loans
CREATE TABLE collection_actions (
    action_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    delinquency_id UUID NOT NULL REFERENCES loan_delinquencies(delinquency_id),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    action_type VARCHAR(30) NOT NULL CHECK (action_type IN ('EmailReminder', 'SmsNotification', 'PhoneCall', 'LetterNotice', 'LegalNotice', 'FieldVisit', 'PaymentPlan', 'Restructuring', 'LegalAction', 'AssetRecovery')),
    action_date DATE NOT NULL,
    due_date DATE,
    description VARCHAR(500) NOT NULL,
    amount_demanded DECIMAL(15,2),
    response_received BOOLEAN NOT NULL DEFAULT FALSE,
    response_date DATE,
    response_details TEXT,
    follow_up_required BOOLEAN NOT NULL DEFAULT FALSE,
    follow_up_date DATE,
    action_status VARCHAR(20) NOT NULL DEFAULT 'Planned' CHECK (action_status IN ('Planned', 'InProgress', 'Completed', 'Failed', 'Cancelled')),
    assigned_to VARCHAR(100) NOT NULL,
    created_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_collection_action_dates CHECK (
        (due_date IS NULL OR due_date >= action_date) AND
        (response_date IS NULL OR response_date >= action_date) AND
        (follow_up_date IS NULL OR follow_up_date >= action_date)
    ),
    CONSTRAINT ck_collection_action_response CHECK (
        (response_received = TRUE AND response_date IS NOT NULL) OR
        (response_received = FALSE AND response_date IS NULL)
    )
);

-- Loan payments and allocation
CREATE TABLE loan_payments (
    payment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    payment_date DATE NOT NULL,
    payment_amount DECIMAL(15,2) NOT NULL,
    payment_type VARCHAR(20) NOT NULL CHECK (payment_type IN ('Regular', 'Prepayment', 'PartialPayment', 'Restructured', 'Settlement', 'Recovery')),
    payment_method VARCHAR(20) NOT NULL CHECK (payment_method IN ('BankTransfer', 'DirectDebit', 'Check', 'Cash', 'OnlinePayment', 'MobilePayment', 'StandingInstruction')),
    payment_status VARCHAR(20) NOT NULL DEFAULT 'Processed' CHECK (payment_status IN ('Processed', 'Pending', 'Failed', 'Reversed', 'PartiallyAllocated')),
    external_reference VARCHAR(100),
    processed_by VARCHAR(100) NOT NULL,
    processed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Payment allocation details
    penalty_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    overdue_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    current_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    principal_payment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    fees_payment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    charges_payment DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    excess_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    
    CONSTRAINT ck_loan_payment_amount CHECK (payment_amount > 0),
    CONSTRAINT ck_loan_payment_allocation CHECK (
        ABS(payment_amount - (penalty_interest_payment + overdue_interest_payment + 
                             current_interest_payment + principal_payment + 
                             fees_payment + charges_payment + excess_amount)) < 0.01
    ),
    CONSTRAINT ck_loan_payment_components CHECK (
        penalty_interest_payment >= 0 AND overdue_interest_payment >= 0 AND
        current_interest_payment >= 0 AND principal_payment >= 0 AND
        fees_payment >= 0 AND charges_payment >= 0 AND excess_amount >= 0
    )
);

-- Prepayment handling records
CREATE TABLE prepayment_handling (
    prepayment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_id UUID NOT NULL REFERENCES loan_payments(payment_id),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    handling_type VARCHAR(30) NOT NULL CHECK (handling_type IN ('TermReduction', 'InstallmentReduction', 'HoldInSuspense', 'Refund')),
    excess_amount DECIMAL(15,2) NOT NULL,
    new_outstanding_principal DECIMAL(15,2) NOT NULL,
    term_reduction_months INTEGER,
    new_installment_amount DECIMAL(15,2),
    new_maturity_date DATE,
    schedule_regenerated BOOLEAN NOT NULL DEFAULT FALSE,
    customer_choice BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_prepayment_excess CHECK (excess_amount > 0),
    CONSTRAINT ck_prepayment_principal CHECK (new_outstanding_principal >= 0),
    CONSTRAINT ck_prepayment_term_reduction CHECK (
        (handling_type = 'TermReduction' AND term_reduction_months IS NOT NULL AND term_reduction_months > 0) OR
        (handling_type != 'TermReduction')
    ),
    CONSTRAINT ck_prepayment_installment_reduction CHECK (
        (handling_type = 'InstallmentReduction' AND new_installment_amount IS NOT NULL AND new_installment_amount > 0) OR
        (handling_type != 'InstallmentReduction')
    )
);

-- Loan restructuring records
CREATE TABLE loan_restructurings (
    restructuring_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(account_id),
    restructuring_type VARCHAR(30) NOT NULL CHECK (restructuring_type IN ('PaymentHoliday', 'TermExtension', 'RateReduction', 'PrincipalReduction', 'InstallmentReduction', 'InterestCapitalization', 'FullRestructuring')),
    request_date DATE NOT NULL,
    effective_date DATE,
    restructuring_reason VARCHAR(255) NOT NULL,
    
    -- Original terms
    original_principal DECIMAL(15,2) NOT NULL,
    original_interest_rate DECIMAL(8,6) NOT NULL,
    original_term_months INTEGER NOT NULL,
    original_installment DECIMAL(15,2) NOT NULL,
    
    -- New terms
    new_principal DECIMAL(15,2),
    new_interest_rate DECIMAL(8,6),
    new_term_months INTEGER,
    new_installment DECIMAL(15,2),
    new_maturity_date DATE,
    
    -- Restructuring specifics
    moratorium_period INTEGER, -- Months
    capitalized_interest DECIMAL(15,2),
    waived_penalty_amount DECIMAL(15,2),
    
    approval_status VARCHAR(30) NOT NULL DEFAULT 'Pending' CHECK (approval_status IN ('Pending', 'Approved', 'Rejected', 'ConditionallyApproved', 'RequiresCommitteeApproval')),
    approved_by VARCHAR(100),
    approved_at TIMESTAMP WITH TIME ZONE,
    conditions TEXT, -- JSON array of approval conditions
    created_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_restructuring_amounts CHECK (
        original_principal > 0 AND original_installment > 0 AND
        (new_principal IS NULL OR new_principal > 0) AND
        (new_installment IS NULL OR new_installment > 0) AND
        (capitalized_interest IS NULL OR capitalized_interest >= 0) AND
        (waived_penalty_amount IS NULL OR waived_penalty_amount >= 0)
    ),
    CONSTRAINT ck_restructuring_terms CHECK (
        original_term_months > 0 AND
        (new_term_months IS NULL OR new_term_months > 0)
    ),
    CONSTRAINT ck_restructuring_approval CHECK (
        (approval_status IN ('Approved', 'Rejected') AND approved_by IS NOT NULL AND approved_at IS NOT NULL) OR
        (approval_status NOT IN ('Approved', 'Rejected'))
    )
);

-- Daily loan delinquency processing jobs
CREATE TABLE loan_delinquency_jobs (
    job_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    processing_date DATE NOT NULL,
    loans_processed INTEGER NOT NULL DEFAULT 0,
    new_delinquent_loans INTEGER NOT NULL DEFAULT 0,
    recovered_loans INTEGER NOT NULL DEFAULT 0,
    npl_classifications INTEGER NOT NULL DEFAULT 0,
    total_penalty_interest DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    collection_actions_triggered INTEGER NOT NULL DEFAULT 0,
    notifications_sent INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(30) NOT NULL DEFAULT 'Scheduled' CHECK (status IN ('Scheduled', 'Running', 'Completed', 'Failed', 'PartiallyCompleted')),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    errors TEXT, -- JSON array of error messages
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_loan_delinq_job_counts CHECK (
        loans_processed >= 0 AND new_delinquent_loans >= 0 AND 
        recovered_loans >= 0 AND npl_classifications >= 0 AND
        collection_actions_triggered >= 0 AND notifications_sent >= 0
    ),
    CONSTRAINT ck_loan_delinq_job_completion CHECK (
        (status = 'Completed' AND completed_at IS NOT NULL) OR
        (status != 'Completed')
    )
);

-- =============================================================================
-- PERFORMANCE INDEXES FOR SPECIALIZED ACCOUNT FUNCTIONS
-- =============================================================================

-- CASA (Overdraft) indexes
CREATE INDEX idx_overdraft_facilities_account ON overdraft_facilities(account_id);
CREATE INDEX idx_overdraft_facilities_status ON overdraft_facilities(facility_status) WHERE facility_status = 'Active';
CREATE INDEX idx_overdraft_facilities_review_due ON overdraft_facilities(next_review_date) WHERE facility_status = 'Active';
CREATE INDEX idx_overdraft_facilities_expiry ON overdraft_facilities(expiry_date) WHERE expiry_date IS NOT NULL;

CREATE INDEX idx_overdraft_utilization_account_date ON overdraft_utilization(account_id, utilization_date DESC);
CREATE INDEX idx_overdraft_utilization_overdrawn ON overdraft_utilization(utilization_date, closing_balance) WHERE closing_balance < 0;
CREATE INDEX idx_overdraft_utilization_capitalization ON overdraft_utilization(capitalized, capitalization_date) WHERE capitalized = TRUE;

CREATE INDEX idx_overdraft_interest_calc_account ON overdraft_interest_calculations(account_id, calculated_at DESC);
CREATE INDEX idx_overdraft_interest_calc_period ON overdraft_interest_calculations(calculation_period_start, calculation_period_end);

CREATE INDEX idx_overdraft_adjustments_account ON overdraft_limit_adjustments(account_id);
CREATE INDEX idx_overdraft_adjustments_status ON overdraft_limit_adjustments(approval_status) WHERE approval_status = 'Pending';

CREATE INDEX idx_interest_posting_account_date ON interest_posting_records(account_id, posting_date DESC);
CREATE INDEX idx_interest_posting_type_date ON interest_posting_records(interest_type, posting_date DESC);

CREATE INDEX idx_overdraft_jobs_date ON overdraft_processing_jobs(processing_date DESC);
CREATE INDEX idx_overdraft_jobs_status ON overdraft_processing_jobs(status) WHERE status IN ('Scheduled', 'Running');

-- Loan servicing indexes
CREATE INDEX idx_amortization_schedules_loan ON amortization_schedules(loan_account_id);
CREATE INDEX idx_amortization_schedules_maturity ON amortization_schedules(maturity_date);

CREATE INDEX idx_amortization_entries_schedule ON amortization_entries(schedule_id, installment_number);
CREATE INDEX idx_amortization_entries_due_date ON amortization_entries(due_date, payment_status) WHERE payment_status IN ('Scheduled', 'Due');
CREATE INDEX idx_amortization_entries_overdue ON amortization_entries(payment_status, days_overdue) WHERE payment_status = 'Overdue';

CREATE INDEX idx_loan_delinquencies_account ON loan_delinquencies(loan_account_id);
CREATE INDEX idx_loan_delinquencies_stage ON loan_delinquencies(delinquency_stage);
CREATE INDEX idx_loan_delinquencies_dpd ON loan_delinquencies(current_dpd DESC);
CREATE INDEX idx_loan_delinquencies_npl_date ON loan_delinquencies(npl_date) WHERE npl_date IS NOT NULL;

CREATE INDEX idx_collection_actions_loan ON collection_actions(loan_account_id, created_at DESC);
CREATE INDEX idx_collection_actions_type_status ON collection_actions(action_type, action_status);
CREATE INDEX idx_collection_actions_assigned ON collection_actions(assigned_to, action_status) WHERE action_status IN ('Planned', 'InProgress');
CREATE INDEX idx_collection_actions_follow_up ON collection_actions(follow_up_required, follow_up_date) WHERE follow_up_required = TRUE;

CREATE INDEX idx_loan_payments_account_date ON loan_payments(loan_account_id, payment_date DESC);
CREATE INDEX idx_loan_payments_type ON loan_payments(payment_type, payment_date DESC);
CREATE INDEX idx_loan_payments_status ON loan_payments(payment_status) WHERE payment_status != 'Processed';

CREATE INDEX idx_prepayment_handling_payment ON prepayment_handling(payment_id);
CREATE INDEX idx_prepayment_handling_loan ON prepayment_handling(loan_account_id);
CREATE INDEX idx_prepayment_handling_type ON prepayment_handling(handling_type);

CREATE INDEX idx_loan_restructurings_account ON loan_restructurings(loan_account_id, request_date DESC);
CREATE INDEX idx_loan_restructurings_status ON loan_restructurings(approval_status) WHERE approval_status IN ('Pending', 'RequiresCommitteeApproval');
CREATE INDEX idx_loan_restructurings_type ON loan_restructurings(restructuring_type);

CREATE INDEX idx_loan_delinq_jobs_date ON loan_delinquency_jobs(processing_date DESC);
CREATE INDEX idx_loan_delinq_jobs_status ON loan_delinquency_jobs(status) WHERE status IN ('Scheduled', 'Running');

-- =============================================================================
-- TRIGGERS FOR AUTOMATED PROCESSING
-- =============================================================================

-- Update overdraft available limit when utilization changes
CREATE OR REPLACE FUNCTION update_overdraft_available_limit()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE overdraft_facilities 
    SET available_limit = approved_limit - NEW.current_utilized,
        last_updated_at = NOW()
    WHERE account_id = NEW.account_id;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_update_overdraft_available_limit
    AFTER UPDATE OF current_utilized ON overdraft_facilities
    FOR EACH ROW
    EXECUTE FUNCTION update_overdraft_available_limit();

-- Auto-update loan delinquency total overdue amount
CREATE OR REPLACE FUNCTION update_loan_delinquency_total()
RETURNS TRIGGER AS $$
BEGIN
    NEW.total_overdue_amount = NEW.overdue_principal + NEW.overdue_interest + NEW.penalty_interest_accrued;
    NEW.last_updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_update_loan_delinquency_total
    BEFORE UPDATE ON loan_delinquencies
    FOR EACH ROW
    WHEN (OLD.overdue_principal IS DISTINCT FROM NEW.overdue_principal OR 
          OLD.overdue_interest IS DISTINCT FROM NEW.overdue_interest OR 
          OLD.penalty_interest_accrued IS DISTINCT FROM NEW.penalty_interest_accrued)
    EXECUTE FUNCTION update_loan_delinquency_total();

-- Update amortization schedule last_updated_at when entries change
CREATE OR REPLACE FUNCTION update_amortization_schedule_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE amortization_schedules 
    SET last_updated_at = NOW()
    WHERE schedule_id = COALESCE(NEW.schedule_id, OLD.schedule_id);
    
    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_update_amortization_schedule_timestamp
    AFTER INSERT OR UPDATE OR DELETE ON amortization_entries
    FOR EACH ROW
    EXECUTE FUNCTION update_amortization_schedule_timestamp();

-- =============================================================================
-- VIEWS FOR COMMON QUERIES
-- =============================================================================

-- Active overdraft facilities with utilization
CREATE VIEW active_overdraft_facilities AS
SELECT 
    of.facility_id,
    of.account_id,
    ao.customer_id,
    of.approved_limit,
    of.current_utilized,
    of.available_limit,
    of.interest_rate,
    ROUND((of.current_utilized / of.approved_limit) * 100, 2) AS utilization_percentage,
    of.next_review_date,
    CASE 
        WHEN of.expiry_date IS NOT NULL AND of.expiry_date <= CURRENT_DATE + INTERVAL '30 days' THEN 'Expiring Soon'
        WHEN of.next_review_date <= CURRENT_DATE THEN 'Review Due'
        WHEN of.current_utilized > (of.approved_limit * 0.8) THEN 'High Utilization'
        ELSE 'Normal'
    END AS alert_status
FROM overdraft_facilities of
JOIN accounts a ON of.account_id = a.account_id
JOIN account_ownership ao ON a.account_id = ao.account_id AND ao.ownership_type = 'Individual'
WHERE of.facility_status = 'Active';

-- Loan portfolio summary view
CREATE VIEW loan_portfolio_summary AS
SELECT 
    a.product_code,
    COUNT(*) AS total_loans,
    SUM(a.outstanding_principal) AS total_outstanding,
    AVG(a.outstanding_principal) AS avg_loan_size,
    AVG(a.loan_interest_rate) AS avg_interest_rate,
    COUNT(CASE WHEN ld.delinquency_stage = 'Current' THEN 1 END) AS current_loans,
    COUNT(CASE WHEN ld.delinquency_stage = 'NonPerforming' THEN 1 END) AS npl_loans,
    SUM(CASE WHEN ld.delinquency_stage = 'NonPerforming' THEN a.outstanding_principal ELSE 0 END) AS npl_amount,
    ROUND((SUM(CASE WHEN ld.delinquency_stage = 'NonPerforming' THEN a.outstanding_principal ELSE 0 END) / 
           NULLIF(SUM(a.outstanding_principal), 0)) * 100, 2) AS npl_ratio
FROM accounts a
LEFT JOIN loan_delinquencies ld ON a.account_id = ld.loan_account_id
WHERE a.account_type = 'Loan' 
  AND a.account_status = 'Active'
GROUP BY a.product_code;

-- Delinquent loans requiring attention
CREATE VIEW delinquent_loans_attention AS
SELECT 
    a.account_id,
    ao.customer_id,
    a.outstanding_principal,
    ld.current_dpd,
    ld.delinquency_stage,
    ld.total_overdue_amount,
    ld.last_payment_date,
    COUNT(ca.action_id) AS total_collection_actions,
    MAX(ca.action_date) AS last_collection_action_date,
    CASE 
        WHEN ld.current_dpd >= 180 THEN 'Critical'
        WHEN ld.current_dpd >= 90 THEN 'High'
        WHEN ld.current_dpd >= 60 THEN 'Medium'
        ELSE 'Low'
    END AS risk_level
FROM accounts a
JOIN account_ownership ao ON a.account_id = ao.account_id AND ao.ownership_type = 'Individual'
JOIN loan_delinquencies ld ON a.account_id = ld.loan_account_id
LEFT JOIN collection_actions ca ON ld.loan_account_id = ca.loan_account_id
WHERE ld.delinquency_stage != 'Current'
GROUP BY a.account_id, ao.customer_id, a.outstanding_principal, 
         ld.current_dpd, ld.delinquency_stage, ld.total_overdue_amount, ld.last_payment_date
ORDER BY ld.current_dpd DESC, a.outstanding_principal DESC;