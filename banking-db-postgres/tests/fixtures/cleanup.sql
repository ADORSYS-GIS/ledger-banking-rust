-- Database cleanup script for test isolation
-- This script truncates all tables and resets sequences to ensure clean test state

DO $cleanup$
DECLARE
    r RECORD;
BEGIN
    -- Disable foreign key checks temporarily to avoid constraint violations during truncation
    SET session_replication_role = replica;

    -- Clear workflow-related tables first (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'workflow_step_records') THEN
        TRUNCATE TABLE workflow_step_records CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'account_workflows') THEN
        TRUNCATE TABLE account_workflows CASCADE;
    END IF;

    -- Clear transaction-related tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'transaction_holds') THEN
        TRUNCATE TABLE transaction_holds CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'transactions') THEN
        TRUNCATE TABLE transactions CASCADE;
    END IF;

    -- Clear account-related tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'account_balances') THEN
        TRUNCATE TABLE account_balances CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'account_interest_calculations') THEN
        TRUNCATE TABLE account_interest_calculations CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'accounts') THEN
        TRUNCATE TABLE accounts CASCADE;
    END IF;

    -- Clear customer-related tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'customer_documents') THEN
        TRUNCATE TABLE customer_documents CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'customer_audit') THEN
        TRUNCATE TABLE customer_audit CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'customer_portfolios') THEN
        TRUNCATE TABLE customer_portfolios CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'customers') THEN
        TRUNCATE TABLE customers CASCADE;
    END IF;

    -- Clear compliance-related tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'compliance_alerts') THEN
        TRUNCATE TABLE compliance_alerts CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'compliance_results') THEN
        TRUNCATE TABLE compliance_results CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'sanctions_screening') THEN
        TRUNCATE TABLE sanctions_screening CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'ultimate_beneficial_owners') THEN
        TRUNCATE TABLE ultimate_beneficial_owners CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'suspicious_activity_reports') THEN
        TRUNCATE TABLE suspicious_activity_reports CASCADE;
    END IF;

    -- Clear collateral-related tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'collateral_valuations') THEN
        TRUNCATE TABLE collateral_valuations CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'collateral') THEN
        TRUNCATE TABLE collateral CASCADE;
    END IF;

    -- Clear person-related tables (persons should be last due to foreign key dependencies)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'persons') THEN
        TRUNCATE TABLE persons CASCADE;
    END IF;

    -- Clear agent network tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'terminals') THEN
        TRUNCATE TABLE terminals CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'branches') THEN
        TRUNCATE TABLE branches CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'agent_networks') THEN
        TRUNCATE TABLE agent_networks CASCADE;
    END IF;

    -- Clear calendar and fee tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'bank_holidays') THEN
        TRUNCATE TABLE bank_holidays CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'fee_structures') THEN
        TRUNCATE TABLE fee_structures CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'fees') THEN
        TRUNCATE TABLE fees CASCADE;
    END IF;

    -- Clear channel tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'channels') THEN
        TRUNCATE TABLE channels CASCADE;
    END IF;

    -- Clear reason and purpose tables (if they exist)
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'reasons') THEN
        TRUNCATE TABLE reasons CASCADE;
    END IF;
    
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'purposes') THEN
        TRUNCATE TABLE purposes CASCADE;
    END IF;

    -- Re-enable foreign key checks
    SET session_replication_role = DEFAULT;
    
END
$cleanup$;

-- Reset all sequences to start from 1
-- Note: Most of our tables use UUIDs, but some might have serial columns
-- This ensures any auto-incrementing columns start fresh

-- Reset any sequences that might exist (add more if needed)
-- ALTER SEQUENCE IF EXISTS some_sequence_name RESTART WITH 1;

-- Note: VACUUM ANALYZE removed because it cannot run inside a transaction block
-- and SQLx executes this script as a single transaction

-- Optional: Show table counts to verify cleanup (useful for debugging)
-- Uncomment these lines if you want to verify the cleanup worked
/*
SELECT 'account_workflows' as table_name, COUNT(*) as row_count FROM account_workflows
UNION ALL
SELECT 'transactions', COUNT(*) FROM transactions
UNION ALL
SELECT 'accounts', COUNT(*) FROM accounts
UNION ALL
SELECT 'customers', COUNT(*) FROM customers
UNION ALL
SELECT 'persons', COUNT(*) FROM persons
UNION ALL
SELECT 'compliance_results', COUNT(*) FROM compliance_results
UNION ALL
SELECT 'collateral', COUNT(*) FROM collateral
ORDER BY table_name;
*/