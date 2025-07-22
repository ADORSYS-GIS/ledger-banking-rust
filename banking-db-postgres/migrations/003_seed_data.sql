-- Sample data and system configuration for ledger-banking-rust
-- Initial setup data for development and testing

-- =============================================================================
-- SYSTEM CONFIGURATION
-- =============================================================================

-- Bank holidays for multiple jurisdictions
INSERT INTO bank_holidays (holiday_id, jurisdiction, holiday_date, holiday_name, holiday_type, is_recurring) VALUES
(uuid_generate_v4(), 'US', '2024-01-01', 'New Year''s Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-01-15', 'Martin Luther King Jr. Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-02-19', 'Presidents Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-05-27', 'Memorial Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-06-19', 'Juneteenth', 'National', true),
(uuid_generate_v4(), 'US', '2024-07-04', 'Independence Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-09-02', 'Labor Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-10-14', 'Columbus Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-11-11', 'Veterans Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-11-28', 'Thanksgiving Day', 'National', true),
(uuid_generate_v4(), 'US', '2024-12-25', 'Christmas Day', 'National', true),
(uuid_generate_v4(), 'CM', '2024-01-01', 'New Year''s Day', 'National', true),
(uuid_generate_v4(), 'CM', '2024-02-11', 'Youth Day', 'National', true),
(uuid_generate_v4(), 'CM', '2024-05-20', 'National Day', 'National', true),
(uuid_generate_v4(), 'CM', '2024-12-25', 'Christmas Day', 'Religious', true);

-- =============================================================================
-- SAMPLE AGENT NETWORK DATA
-- =============================================================================

-- Sample agent networks
INSERT INTO agent_networks (network_id, network_name, network_type, status, aggregate_daily_limit, settlement_gl_code) VALUES
('11111111-1111-1111-1111-111111111111', 'Primary Banking Network', 'Internal', 'Active', 1000000.00, '5001'),
('22222222-2222-2222-2222-222222222222', 'Partner Agent Network', 'Partner', 'Active', 500000.00, '5002'),
('33333333-3333-3333-3333-333333333333', 'Mobile Money Network', 'ThirdParty', 'Active', 250000.00, '5003');

-- Sample agency branches
INSERT INTO agency_branches (branch_id, network_id, parent_branch_id, branch_name, branch_code, branch_level, gl_code_prefix, geolocation, daily_transaction_limit) VALUES
('a1111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', NULL, 'Downtown Branch', 'DTW001', 0, '4001', 'Downtown Financial District', 100000.00),
('b2222222-2222-2222-2222-222222222222', '11111111-1111-1111-1111-111111111111', 'a1111111-1111-1111-1111-111111111111', 'Midtown Branch', 'MDT001', 1, '4002', 'Midtown Commercial Area', 50000.00),
('c3333333-3333-3333-3333-333333333333', '22222222-2222-2222-2222-222222222222', NULL, 'Agent Hub Central', 'AHC001', 0, '4003', 'Central Business District', 75000.00),
('d4444444-4444-4444-4444-444444444444', '33333333-3333-3333-3333-333333333333', NULL, 'Mobile Hub West', 'MHW001', 0, '4004', 'West District', 30000.00);

-- Sample agent terminals
INSERT INTO agent_terminals (terminal_id, branch_id, agent_user_id, terminal_type, terminal_name, daily_transaction_limit) VALUES
('f1111111-1111-1111-1111-111111111111', 'a1111111-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Pos', 'Downtown POS Terminal 1', 10000.00),
('f2222222-2222-2222-2222-222222222222', 'b2222222-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 'Mobile', 'Midtown Mobile Terminal', 5000.00),
('f3333333-3333-3333-3333-333333333333', 'c3333333-3333-3333-3333-333333333333', '33333333-3333-3333-3333-333333333333', 'WebPortal', 'Agent Hub Portal', 15000.00),
('f4444444-4444-4444-4444-444444444444', 'd4444444-4444-4444-4444-444444444444', '44444444-4444-4444-4444-444444444444', 'Mobile', 'Mobile Money Terminal 1', 3000.00);

-- =============================================================================
-- SAMPLE CUSTOMER DATA
-- =============================================================================

-- Individual customers
INSERT INTO customers (customer_id, customer_type, full_name, id_type, id_number, risk_rating, status, updated_by) VALUES
('c1111111-1111-1111-1111-111111111111', 'Individual', 'John Michael Smith', 'NationalId', 'ID123456789', 'Low', 'Active', 'SYSTEM'),
('c2222222-2222-2222-2222-222222222222', 'Individual', 'Sarah Elizabeth Johnson', 'NationalId', 'ID987654321', 'Low', 'Active', 'SYSTEM'),
('c3333333-3333-3333-3333-333333333333', 'Individual', 'Robert James Wilson', 'Passport', 'P123ABC789', 'Medium', 'Active', 'SYSTEM'),
('c4444444-4444-4444-4444-444444444444', 'Individual', 'Maria Elena Rodriguez', 'NationalId', 'ID456789123', 'Low', 'Active', 'SYSTEM'),
('c5555555-5555-5555-5555-555555555555', 'Individual', 'David Chen Wei', 'NationalId', 'ID789123456', 'High', 'Active', 'SYSTEM');

-- Corporate customers
INSERT INTO customers (customer_id, customer_type, full_name, id_type, id_number, risk_rating, status, updated_by) VALUES
('c6666666-6666-6666-6666-666666666666', 'Corporate', 'TechCorp Solutions Ltd', 'CompanyRegistration', 'CR2024001234', 'Medium', 'Active', 'SYSTEM'),
('c7777777-7777-7777-7777-777777777777', 'Corporate', 'Global Trade Enterprises', 'CompanyRegistration', 'CR2024005678', 'High', 'Active', 'SYSTEM'),
('c8888888-8888-8888-8888-888888888888', 'Corporate', 'Local Manufacturing Co', 'CompanyRegistration', 'CR2024009012', 'Low', 'Active', 'SYSTEM');

-- =============================================================================
-- SAMPLE ACCOUNT DATA
-- =============================================================================

-- Savings accounts
INSERT INTO accounts (account_id, product_code, account_type, account_status, currency, open_date, domicile_branch_id, current_balance, available_balance, updated_by) VALUES
('a1111111-1111-1111-1111-111111111111', 'SAV001', 'Savings', 'Active', 'USD', '2024-01-15', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 5000.00, 5000.00, 'SYSTEM'),
('a2222222-2222-2222-2222-222222222222', 'SAV001', 'Savings', 'Active', 'USD', '2024-02-10', 'b2222222-2222-2222-2222-222222222222', 12500.50, 12500.50, 'SYSTEM'),
('a3333333-3333-3333-3333-333333333333', 'SAV002', 'Savings', 'Active', 'USD', '2024-01-28', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 25000.00, 25000.00, 'SYSTEM');

-- Current accounts
INSERT INTO accounts (account_id, product_code, account_type, account_status, currency, open_date, domicile_branch_id, current_balance, available_balance, overdraft_limit, updated_by) VALUES
('a4444444-4444-4444-4444-444444444444', 'CUR001', 'Current', 'Active', 'USD', '2024-02-05', 'c3333333-3333-3333-3333-333333333333', 15000.00, 20000.00, 5000.00, 'SYSTEM'),
('a5555555-5555-5555-5555-555555555555', 'CUR002', 'Current', 'Active', 'USD', '2024-03-01', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 75000.00, 85000.00, 10000.00, 'SYSTEM');

-- Loan accounts
INSERT INTO accounts (account_id, product_code, account_type, account_status, currency, open_date, domicile_branch_id, current_balance, available_balance, 
                     original_principal, outstanding_principal, loan_interest_rate, loan_term_months, disbursement_date, maturity_date, installment_amount, next_due_date, updated_by) VALUES
('a6666666-6666-6666-6666-666666666666', 'LON001', 'Loan', 'Active', 'USD', '2024-01-20', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 0.00, 0.00,
 50000.00, 45000.00, 0.0850, 60, '2024-01-20', '2029-01-20', 1024.50, '2024-04-20', 'SYSTEM'),
('a7777777-7777-7777-7777-777777777777', 'LON002', 'Loan', 'Active', 'USD', '2024-02-15', 'b2222222-2222-2222-2222-222222222222', 0.00, 0.00,
 100000.00, 98500.00, 0.0725, 120, '2024-02-15', '2034-02-15', 1189.75, '2024-04-15', 'SYSTEM');

-- =============================================================================
-- SAMPLE ACCOUNT OWNERSHIP AND RELATIONSHIPS
-- =============================================================================

-- Single ownership accounts
INSERT INTO account_ownership (ownership_id, account_id, customer_id, ownership_type, ownership_percentage) VALUES
(uuid_generate_v4(), 'a1111111-1111-1111-1111-111111111111', 'c1111111-1111-1111-1111-111111111111', 'Single', 100.00),
(uuid_generate_v4(), 'a2222222-2222-2222-2222-222222222222', 'c2222222-2222-2222-2222-222222222222', 'Single', 100.00),
(uuid_generate_v4(), 'a3333333-3333-3333-3333-333333333333', 'c3333333-3333-3333-3333-333333333333', 'Single', 100.00),
(uuid_generate_v4(), 'a6666666-6666-6666-6666-666666666666', 'c4444444-4444-4444-4444-444444444444', 'Single', 100.00),
(uuid_generate_v4(), 'a7777777-7777-7777-7777-777777777777', 'c5555555-5555-5555-5555-555555555555', 'Single', 100.00);

-- Joint ownership account
INSERT INTO account_ownership (ownership_id, account_id, customer_id, ownership_type, ownership_percentage) VALUES
(uuid_generate_v4(), 'a4444444-4444-4444-4444-444444444444', 'c1111111-1111-1111-1111-111111111111', 'Joint', 60.00),
(uuid_generate_v4(), 'a4444444-4444-4444-4444-444444444444', 'c2222222-2222-2222-2222-222222222222', 'Joint', 40.00);

-- Corporate account
INSERT INTO account_ownership (ownership_id, account_id, customer_id, ownership_type, ownership_percentage) VALUES
(uuid_generate_v4(), 'a5555555-5555-5555-5555-555555555555', 'c6666666-6666-6666-6666-666666666666', 'Corporate', 100.00);

-- Account relationships for service management
INSERT INTO account_relationships (relationship_id, account_id, entity_id, entity_type, relationship_type, status, start_date) VALUES
(uuid_generate_v4(), 'a1111111-1111-1111-1111-111111111111', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', 'Branch', 'PrimaryHandler', 'Active', '2024-01-15'),
(uuid_generate_v4(), 'a2222222-2222-2222-2222-222222222222', 'b2222222-2222-2222-2222-222222222222', 'Branch', 'PrimaryHandler', 'Active', '2024-02-10'),
(uuid_generate_v4(), 'a5555555-5555-5555-5555-555555555555', 'c5555555-5555-5555-5555-555555555555', 'RiskManager', 'RiskOversight', 'Active', '2024-03-01');

-- Ultimate Beneficial Owners for corporate accounts
INSERT INTO ultimate_beneficiaries (ubo_link_id, corporate_customer_id, beneficiary_customer_id, ownership_percentage, control_type, status, verification_status) VALUES
(uuid_generate_v4(), 'c6666666-6666-6666-6666-666666666666', 'c1111111-1111-1111-1111-111111111111', 51.00, 'DirectOwnership', 'Active', 'Verified'),
(uuid_generate_v4(), 'c6666666-6666-6666-6666-666666666666', 'c3333333-3333-3333-3333-333333333333', 25.00, 'DirectOwnership', 'Active', 'Verified'),
(uuid_generate_v4(), 'c7777777-7777-7777-7777-777777777777', 'c4444444-4444-4444-4444-444444444444', 35.00, 'DirectOwnership', 'Active', 'Pending'),
(uuid_generate_v4(), 'c7777777-7777-7777-7777-777777777777', 'c5555555-5555-5555-5555-555555555555', 40.00, 'SignificantInfluence', 'Active', 'Verified');

-- =============================================================================
-- SAMPLE TRANSACTIONS
-- =============================================================================

-- Sample deposits and withdrawals
INSERT INTO transactions (transaction_id, account_id, transaction_code, transaction_type, amount, currency, description, channel_id, 
                         terminal_id, transaction_date, value_date, status, reference_number, gl_code, created_at) VALUES
(uuid_generate_v4(), 'a1111111-1111-1111-1111-111111111111', 'DEP001', 'Credit', 1000.00, 'USD', 'Initial deposit', 'BRANCH_TELLER', 
 'f1111111-1111-1111-1111-111111111111', '2024-01-15 10:30:00+00', '2024-01-15', 'Posted', 'TXN202401150001', '4001001', '2024-01-15 10:30:00+00'),
(uuid_generate_v4(), 'a2222222-2222-2222-2222-222222222222', 'DEP001', 'Credit', 2500.50, 'USD', 'Cash deposit', 'BRANCH_TELLER', 
 'f2222222-2222-2222-2222-222222222222', '2024-02-10 14:15:00+00', '2024-02-10', 'Posted', 'TXN202402100001', '4002001', '2024-02-10 14:15:00+00'),
(uuid_generate_v4(), 'a1111111-1111-1111-1111-111111111111', 'WDR001', 'Debit', 500.00, 'USD', 'Cash withdrawal', 'ATM', 
 NULL, '2024-02-20 16:45:00+00', '2024-02-20', 'Posted', 'TXN202402200001', '4001002', '2024-02-20 16:45:00+00'),
(uuid_generate_v4(), 'a4444444-4444-4444-4444-444444444444', 'DEP002', 'Credit', 15000.00, 'USD', 'Business deposit', 'AGENT_TERMINAL', 
 'f3333333-3333-3333-3333-333333333333', '2024-02-05 11:20:00+00', '2024-02-05', 'Posted', 'TXN202402050001', '4003001', '2024-02-05 11:20:00+00');

-- Sample loan disbursement
INSERT INTO transactions (transaction_id, account_id, transaction_code, transaction_type, amount, currency, description, channel_id, 
                         transaction_date, value_date, status, reference_number, gl_code, created_at) VALUES
(uuid_generate_v4(), 'a6666666-6666-6666-6666-666666666666', 'LDI001', 'Credit', 50000.00, 'USD', 'Loan disbursement', 'SYSTEM', 
 '2024-01-20 09:00:00+00', '2024-01-20', 'Posted', 'TXN202401200001', '4001003', '2024-01-20 09:00:00+00');

-- =============================================================================
-- SAMPLE KYC AND COMPLIANCE DATA
-- =============================================================================

-- KYC records
INSERT INTO kyc_records (kyc_id, customer_id, kyc_type, status, risk_assessment, last_review_date, next_review_date, reviewed_by, notes) VALUES
(uuid_generate_v4(), 'c1111111-1111-1111-1111-111111111111', 'Initial', 'Completed', 'Low', '2024-01-10', '2025-01-10', 'KYC_OFFICER_001', 'Standard individual KYC completed'),
(uuid_generate_v4(), 'c2222222-2222-2222-2222-222222222222', 'Initial', 'Completed', 'Low', '2024-02-05', '2025-02-05', 'KYC_OFFICER_001', 'Standard individual KYC completed'),
(uuid_generate_v4(), 'c3333333-3333-3333-3333-333333333333', 'Enhanced', 'Completed', 'Medium', '2024-01-25', '2024-07-25', 'KYC_OFFICER_002', 'Enhanced due diligence completed'),
(uuid_generate_v4(), 'c5555555-5555-5555-5555-555555555555', 'Enhanced', 'RequiresReview', 'High', '2024-01-01', '2024-04-01', 'KYC_OFFICER_002', 'Requires periodic review due to high risk'),
(uuid_generate_v4(), 'c6666666-6666-6666-6666-666666666666', 'Enhanced', 'Completed', 'Medium', '2024-02-28', '2024-08-28', 'KYC_OFFICER_003', 'Corporate KYC with UBO verification');

-- Sanctions screening
INSERT INTO sanctions_screening (screening_id, customer_id, screening_date, screening_result, screened_by, last_screening_date) VALUES
(uuid_generate_v4(), 'c1111111-1111-1111-1111-111111111111', '2024-01-10 09:00:00+00', 'Clear', 'COMPLIANCE_SYSTEM', '2024-01-10 09:00:00+00'),
(uuid_generate_v4(), 'c2222222-2222-2222-2222-222222222222', '2024-02-05 09:00:00+00', 'Clear', 'COMPLIANCE_SYSTEM', '2024-02-05 09:00:00+00'),
(uuid_generate_v4(), 'c3333333-3333-3333-3333-333333333333', '2024-01-25 09:00:00+00', 'PotentialMatch', 'COMPLIANCE_SYSTEM', '2024-01-25 09:00:00+00'),
(uuid_generate_v4(), 'c5555555-5555-5555-5555-555555555555', '2024-01-01 09:00:00+00', 'Clear', 'COMPLIANCE_SYSTEM', '2024-01-01 09:00:00+00'),
(uuid_generate_v4(), 'c6666666-6666-6666-6666-666666666666', '2024-02-28 09:00:00+00', 'Clear', 'COMPLIANCE_SYSTEM', '2024-02-28 09:00:00+00');

-- Risk scores
INSERT INTO compliance_risk_scores (risk_score_id, customer_id, risk_score, calculated_at, valid_until) VALUES
(uuid_generate_v4(), 'c1111111-1111-1111-1111-111111111111', 15.5, '2024-01-10 09:00:00+00', '2024-07-10 09:00:00+00'),
(uuid_generate_v4(), 'c2222222-2222-2222-2222-222222222222', 12.3, '2024-02-05 09:00:00+00', '2024-08-05 09:00:00+00'),
(uuid_generate_v4(), 'c3333333-3333-3333-3333-333333333333', 45.7, '2024-01-25 09:00:00+00', '2024-04-25 09:00:00+00'),
(uuid_generate_v4(), 'c5555555-5555-5555-5555-555555555555', 78.9, '2024-01-01 09:00:00+00', '2024-04-01 09:00:00+00'),
(uuid_generate_v4(), 'c6666666-6666-6666-6666-666666666666', 35.2, '2024-02-28 09:00:00+00', '2024-08-28 09:00:00+00');

-- Compliance alerts
INSERT INTO compliance_alerts (alert_id, customer_id, account_id, alert_type, severity, description, status, assigned_to) VALUES
(uuid_generate_v4(), 'c3333333-3333-3333-3333-333333333333', NULL, 'SANCTIONS_POTENTIAL_MATCH', 'Medium', 'Potential sanctions match requiring investigation', 'InProgress', 'COMPLIANCE_OFFICER_001'),
(uuid_generate_v4(), 'c5555555-5555-5555-5555-555555555555', 'a7777777-7777-7777-7777-777777777777', 'HIGH_RISK_LOAN', 'High', 'High-risk customer with large loan exposure', 'Open', 'RISK_MANAGER_001');

-- =============================================================================
-- SAMPLE WORKFLOW DATA
-- =============================================================================

-- Sample account workflows
INSERT INTO account_workflows (workflow_id, account_id, workflow_type, current_step, status, initiated_by, timeout_at, completed_at) VALUES
(uuid_generate_v4(), 'a1111111-1111-1111-1111-111111111111', 'AccountOpening', 'Completed', 'Completed', 'BRANCH_OFFICER_001', NULL, NOW()),
(uuid_generate_v4(), 'a5555555-5555-5555-5555-555555555555', 'ComplianceVerification', 'ApprovalRequired', 'PendingAction', 'COMPLIANCE_OFFICER_001', NOW() + INTERVAL '7 days', NULL);

-- Update last_activity_date for accounts based on transactions
UPDATE accounts 
SET last_activity_date = (
    SELECT MAX(DATE(transaction_date))
    FROM transactions 
    WHERE transactions.account_id = accounts.account_id
    AND status = 'Posted'
)
WHERE account_id IN (
    SELECT DISTINCT account_id FROM transactions WHERE status = 'Posted'
);

-- =============================================================================
-- DATA VERIFICATION QUERIES
-- =============================================================================

-- Show summary of inserted data
DO $$
DECLARE
    customer_count INTEGER;
    account_count INTEGER;
    transaction_count INTEGER;
    branch_count INTEGER;
    terminal_count INTEGER;
BEGIN
    SELECT COUNT(*) INTO customer_count FROM customers;
    SELECT COUNT(*) INTO account_count FROM accounts;
    SELECT COUNT(*) INTO transaction_count FROM transactions;
    SELECT COUNT(*) INTO branch_count FROM agency_branches;
    SELECT COUNT(*) INTO terminal_count FROM agent_terminals;
    
    RAISE NOTICE 'Data insertion completed:';
    RAISE NOTICE '  Customers: %', customer_count;
    RAISE NOTICE '  Accounts: %', account_count;
    RAISE NOTICE '  Transactions: %', transaction_count;
    RAISE NOTICE '  Branches: %', branch_count;
    RAISE NOTICE '  Terminals: %', terminal_count;
END $$;