-- Fee Management and Financial Controls Enhancement
-- Additional tables for fee processing and enhanced hold management

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
    waived_by VARCHAR(100),
    waived_reason VARCHAR(255),
    applied_by VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_fee_amount_positive CHECK (amount >= 0),
    CONSTRAINT ck_fee_rate_valid CHECK (fee_rate IS NULL OR (fee_rate >= 0 AND fee_rate <= 1)),
    CONSTRAINT ck_waiver_consistency CHECK (
        (waived = TRUE AND waived_by IS NOT NULL AND waived_reason IS NOT NULL) OR
        (waived = FALSE AND waived_by IS NULL)
    )
);

-- Fee waiver requests and approvals
CREATE TABLE fee_waivers (
    waiver_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fee_application_id UUID NOT NULL REFERENCES fee_applications(fee_application_id),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    waived_amount DECIMAL(15,2) NOT NULL,
    reason VARCHAR(255) NOT NULL,
    waived_by VARCHAR(100) NOT NULL,
    waived_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    approval_required BOOLEAN NOT NULL DEFAULT TRUE,
    approved_by VARCHAR(100),
    approved_at TIMESTAMP WITH TIME ZONE,
    
    CONSTRAINT ck_waived_amount_positive CHECK (waived_amount > 0),
    CONSTRAINT ck_approval_consistency CHECK (
        (approval_required = FALSE) OR
        (approval_required = TRUE AND approved_by IS NOT NULL AND approved_at IS NOT NULL)
    )
);

-- Batch fee processing jobs
CREATE TABLE fee_processing_jobs (
    job_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_type VARCHAR(30) NOT NULL CHECK (job_type IN ('DailyMaintenance', 'MonthlyMaintenance', 'QuarterlyMaintenance', 'AnnualMaintenance', 'AdHocFeeApplication')),
    job_name VARCHAR(100) NOT NULL,
    schedule_expression VARCHAR(100), -- Cron expression
    target_fee_categories TEXT, -- JSON array
    target_products TEXT, -- JSON array
    processing_date DATE NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'Scheduled' CHECK (status IN ('Scheduled', 'Running', 'Completed', 'Failed', 'Cancelled')),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    accounts_processed INTEGER NOT NULL DEFAULT 0,
    fees_applied INTEGER NOT NULL DEFAULT 0,
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    errors TEXT, -- JSON array of error messages
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_job_completion CHECK (
        (status = 'Completed' AND completed_at IS NOT NULL) OR
        (status != 'Completed')
    ),
    CONSTRAINT ck_job_counts_positive CHECK (accounts_processed >= 0 AND fees_applied >= 0)
);

-- Product fee schedule cache (from external Product Catalog)
CREATE TABLE product_fee_schedules (
    schedule_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_code VARCHAR(50) NOT NULL,
    fee_schedule_data TEXT NOT NULL, -- JSON serialized fee schedule
    effective_from DATE NOT NULL,
    effective_to DATE,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    version INTEGER NOT NULL DEFAULT 1,
    
    CONSTRAINT uk_product_fee_version UNIQUE (product_code, version),
    CONSTRAINT ck_fee_schedule_dates CHECK (effective_to IS NULL OR effective_to >= effective_from)
);

-- Fee calculation cache for performance
CREATE TABLE fee_calculation_cache (
    cache_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_code VARCHAR(50) NOT NULL,
    fee_code VARCHAR(50) NOT NULL,
    calculation_key VARCHAR(255) NOT NULL, -- Hash of calculation parameters
    calculated_amount DECIMAL(15,2) NOT NULL,
    base_amount DECIMAL(15,2),
    calculation_context TEXT, -- Additional context as JSON
    cached_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    
    CONSTRAINT uk_fee_calculation_key UNIQUE (calculation_key),
    CONSTRAINT ck_cache_expiry CHECK (expires_at > cached_at)
);

-- =============================================================================
-- ENHANCED ACCOUNT HOLDS MANAGEMENT
-- =============================================================================

-- Enhanced account holds table (updating existing structure)
-- Note: This would be an ALTER TABLE in practice, but shown as CREATE for clarity
DROP TABLE IF EXISTS account_holds_enhanced CASCADE;

CREATE TABLE account_holds_enhanced (
    hold_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    amount DECIMAL(15,2) NOT NULL,
    hold_type VARCHAR(30) NOT NULL CHECK (hold_type IN (
        'UnclearedFunds', 'JudicialLien', 'LoanPledge', 'ComplianceHold', 
        'AdministrativeHold', 'FraudHold', 'PendingAuthorization', 
        'OverdraftReserve', 'CardAuthorization', 'Other'
    )),
    reason VARCHAR(255) NOT NULL,
    placed_by VARCHAR(100) NOT NULL,
    placed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    status VARCHAR(30) NOT NULL DEFAULT 'Active' CHECK (status IN ('Active', 'Released', 'Expired', 'Cancelled', 'PartiallyReleased')),
    released_at TIMESTAMP WITH TIME ZONE,
    released_by VARCHAR(100),
    priority VARCHAR(20) NOT NULL DEFAULT 'Medium' CHECK (priority IN ('Critical', 'High', 'Medium', 'Low')),
    source_reference VARCHAR(100), -- External reference (court order, etc.)
    automatic_release BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_hold_amount_positive CHECK (amount > 0),
    CONSTRAINT ck_hold_release_consistency CHECK (
        (status IN ('Released', 'PartiallyReleased') AND released_at IS NOT NULL AND released_by IS NOT NULL) OR
        (status NOT IN ('Released', 'PartiallyReleased'))
    ),
    CONSTRAINT ck_hold_expiry_future CHECK (expires_at IS NULL OR expires_at > placed_at)
);

-- Hold release records for partial releases audit trail
CREATE TABLE hold_release_records (
    release_record_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    hold_id UUID NOT NULL REFERENCES account_holds_enhanced(hold_id),
    release_amount DECIMAL(15,2) NOT NULL,
    release_reason VARCHAR(255) NOT NULL,
    released_by VARCHAR(100) NOT NULL,
    released_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_partial_release BOOLEAN NOT NULL DEFAULT FALSE,
    remaining_amount DECIMAL(15,2) NOT NULL,
    
    CONSTRAINT ck_release_amount_positive CHECK (release_amount > 0),
    CONSTRAINT ck_remaining_amount_nonnegative CHECK (remaining_amount >= 0)
);

-- Hold expiry batch jobs
CREATE TABLE hold_expiry_jobs (
    job_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    processing_date DATE NOT NULL,
    expired_holds_count INTEGER NOT NULL DEFAULT 0,
    total_released_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    processed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    errors TEXT, -- JSON array of error messages
    
    CONSTRAINT ck_expiry_counts_positive CHECK (expired_holds_count >= 0)
);

-- Balance calculation cache for performance
CREATE TABLE balance_calculations (
    calculation_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    current_balance DECIMAL(15,2) NOT NULL,
    available_balance DECIMAL(15,2) NOT NULL,
    overdraft_limit DECIMAL(15,2),
    total_holds DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    active_hold_count INTEGER NOT NULL DEFAULT 0,
    calculation_timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    hold_breakdown TEXT, -- JSON serialized hold summary
    
    CONSTRAINT ck_balance_calculation_valid CHECK (available_balance <= current_balance + COALESCE(overdraft_limit, 0))
);

-- Hold override records for authorization tracking
CREATE TABLE hold_overrides (
    override_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(account_id),
    overridden_holds TEXT NOT NULL, -- JSON array of hold IDs
    override_amount DECIMAL(15,2) NOT NULL,
    authorized_by VARCHAR(100) NOT NULL,
    override_reason VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_override_amount_positive CHECK (override_amount > 0)
);

-- =============================================================================
-- PERFORMANCE INDEXES FOR NEW TABLES
-- =============================================================================

-- Fee application indexes
CREATE INDEX idx_fee_applications_account_date ON fee_applications(account_id, applied_at DESC);
CREATE INDEX idx_fee_applications_status ON fee_applications(status) WHERE status IN ('Pending', 'Failed');
CREATE INDEX idx_fee_applications_product ON fee_applications(product_code, fee_category);
CREATE INDEX idx_fee_applications_transaction ON fee_applications(transaction_id) WHERE transaction_id IS NOT NULL;
CREATE INDEX idx_fee_applications_waived ON fee_applications(waived, waived_by) WHERE waived = TRUE;
CREATE INDEX idx_fee_applications_reversal_deadline ON fee_applications(reversal_deadline) WHERE reversal_deadline IS NOT NULL;

-- Fee waiver indexes
CREATE INDEX idx_fee_waivers_account ON fee_waivers(account_id);
CREATE INDEX idx_fee_waivers_approval_pending ON fee_waivers(approval_required, approved_at) WHERE approval_required = TRUE AND approved_at IS NULL;
CREATE INDEX idx_fee_waivers_waived_by ON fee_waivers(waived_by, waived_at DESC);

-- Fee processing job indexes
CREATE INDEX idx_fee_jobs_status ON fee_processing_jobs(status) WHERE status IN ('Scheduled', 'Running');
CREATE INDEX idx_fee_jobs_processing_date ON fee_processing_jobs(processing_date DESC);
CREATE INDEX idx_fee_jobs_type_date ON fee_processing_jobs(job_type, processing_date DESC);

-- Product fee schedule indexes
CREATE INDEX idx_product_fee_schedules_product_effective ON product_fee_schedules(product_code, effective_from, effective_to);
CREATE INDEX idx_product_fee_schedules_version ON product_fee_schedules(product_code, version DESC);

-- Fee calculation cache indexes
CREATE INDEX idx_fee_calculation_cache_key ON fee_calculation_cache(calculation_key);
CREATE INDEX idx_fee_calculation_cache_expiry ON fee_calculation_cache(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_fee_calculation_cache_product ON fee_calculation_cache(product_code, fee_code);

-- Enhanced hold indexes
CREATE INDEX idx_account_holds_enhanced_account_active ON account_holds_enhanced(account_id, status) WHERE status = 'Active';
CREATE INDEX idx_account_holds_enhanced_type ON account_holds_enhanced(hold_type, status);
CREATE INDEX idx_account_holds_enhanced_priority ON account_holds_enhanced(priority, amount DESC) WHERE status = 'Active';
CREATE INDEX idx_account_holds_enhanced_expiry ON account_holds_enhanced(expires_at) WHERE expires_at IS NOT NULL AND status = 'Active';
CREATE INDEX idx_account_holds_enhanced_source_ref ON account_holds_enhanced(source_reference) WHERE source_reference IS NOT NULL;
CREATE INDEX idx_account_holds_enhanced_auto_release ON account_holds_enhanced(automatic_release, placed_at) WHERE automatic_release = TRUE AND status = 'Active';

-- Hold release record indexes
CREATE INDEX idx_hold_release_records_hold ON hold_release_records(hold_id, released_at DESC);
CREATE INDEX idx_hold_release_records_released_by ON hold_release_records(released_by, released_at DESC);

-- Balance calculation indexes
CREATE INDEX idx_balance_calculations_account_timestamp ON balance_calculations(account_id, calculation_timestamp DESC);
CREATE INDEX idx_balance_calculations_timestamp ON balance_calculations(calculation_timestamp DESC);

-- Hold override indexes
CREATE INDEX idx_hold_overrides_account ON hold_overrides(account_id, created_at DESC);
CREATE INDEX idx_hold_overrides_authorized_by ON hold_overrides(authorized_by, created_at DESC);

-- =============================================================================
-- TRIGGERS FOR AUDIT TRAIL AND AUTOMATION
-- =============================================================================

-- Update timestamp trigger for account_holds_enhanced
CREATE OR REPLACE FUNCTION update_hold_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_account_holds_enhanced_updated_at
    BEFORE UPDATE ON account_holds_enhanced
    FOR EACH ROW
    EXECUTE FUNCTION update_hold_updated_at();

-- Automatic hold release trigger when account is closed
CREATE OR REPLACE FUNCTION auto_release_holds_on_account_closure()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.account_status = 'Closed' AND OLD.account_status != 'Closed' THEN
        UPDATE account_holds_enhanced 
        SET status = 'Released', 
            released_at = NOW(), 
            released_by = 'SYSTEM_AUTO_RELEASE'
        WHERE account_id = NEW.account_id 
          AND status = 'Active'
          AND automatic_release = TRUE;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER tr_auto_release_holds_on_closure
    AFTER UPDATE ON accounts
    FOR EACH ROW
    WHEN (OLD.account_status IS DISTINCT FROM NEW.account_status)
    EXECUTE FUNCTION auto_release_holds_on_account_closure();

-- =============================================================================
-- VIEWS FOR COMMON QUERIES
-- =============================================================================

-- View for active account holds with calculated available balance impact
CREATE VIEW active_account_holds AS
SELECT 
    ah.hold_id,
    ah.account_id,
    ah.amount,
    ah.hold_type,
    ah.priority,
    ah.reason,
    ah.placed_by,
    ah.placed_at,
    ah.expires_at,
    ah.source_reference,
    a.current_balance,
    a.overdraft_limit,
    CASE 
        WHEN ah.expires_at IS NOT NULL AND ah.expires_at <= NOW() THEN 'Expired'
        ELSE ah.status
    END AS effective_status
FROM account_holds_enhanced ah
JOIN accounts a ON ah.account_id = a.account_id
WHERE ah.status = 'Active';

-- View for fee revenue summary by period
CREATE VIEW fee_revenue_summary AS
SELECT 
    DATE_TRUNC('month', applied_at) AS revenue_month,
    product_code,
    fee_category,
    COUNT(*) AS fee_count,
    SUM(amount) AS total_revenue,
    SUM(CASE WHEN waived THEN amount ELSE 0 END) AS waived_amount,
    SUM(CASE WHEN status = 'Reversed' THEN amount ELSE 0 END) AS reversed_amount
FROM fee_applications 
WHERE status IN ('Applied', 'Waived', 'Reversed')
GROUP BY DATE_TRUNC('month', applied_at), product_code, fee_category;

-- =============================================================================
-- INITIAL DATA AND CONSTRAINTS
-- =============================================================================

-- Add constraint to ensure hold amounts don't exceed reasonable limits
ALTER TABLE account_holds_enhanced 
ADD CONSTRAINT ck_hold_amount_reasonable 
CHECK (amount <= 10000000.00); -- 10 million limit

-- Add constraint to ensure fee amounts are reasonable
ALTER TABLE fee_applications 
ADD CONSTRAINT ck_fee_amount_reasonable 
CHECK (amount <= 1000000.00); -- 1 million limit