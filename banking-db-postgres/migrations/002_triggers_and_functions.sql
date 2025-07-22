-- Triggers and Functions for Business Rule Enforcement and Audit Trails
-- ledger-banking-rust PostgreSQL implementation

-- =============================================================================
-- AUDIT TRAIL FUNCTIONS
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
            
        WHEN 'accounts' THEN
            IF TG_OP = 'UPDATE' AND OLD.account_status != NEW.account_status THEN
                INSERT INTO account_status_history (history_id, account_id, old_status, new_status, change_reason, changed_by, changed_at, system_triggered)
                VALUES (uuid_generate_v4(), NEW.account_id, OLD.account_status, NEW.account_status, 
                       COALESCE(NEW.status_change_reason, 'System update'), 
                       COALESCE(NEW.status_changed_by, NEW.updated_by), 
                       NOW(), FALSE);
            END IF;
    END CASE;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- BUSINESS RULE ENFORCEMENT FUNCTIONS
-- =============================================================================

-- Function to validate account transactions based on status
CREATE OR REPLACE FUNCTION validate_account_transaction()
RETURNS TRIGGER AS $$
DECLARE
    account_record RECORD;
    customer_record RECORD;
BEGIN
    -- Get account information
    SELECT a.account_status, a.account_id, a.signing_condition, c.status as customer_status, c.customer_id
    INTO account_record
    FROM accounts a
    JOIN account_ownership ao ON a.account_id = ao.account_id
    JOIN customers c ON ao.customer_id = c.customer_id
    WHERE a.account_id = NEW.account_id
    LIMIT 1;
    
    -- Check if account exists
    IF NOT FOUND THEN
        RAISE EXCEPTION 'Account % not found', NEW.account_id;
    END IF;
    
    -- Prevent transactions on non-transactional accounts unless it's a system operation
    IF account_record.account_status IN ('PendingApproval', 'Frozen', 'Closed', 'PendingClosure') 
       AND NEW.transaction_code NOT IN ('CLOSURE_SETTLEMENT', 'FROZEN_REVERSAL', 'SYSTEM_ADJUSTMENT') THEN
        RAISE EXCEPTION 'Account % is in % status and cannot process transactions', 
                       NEW.account_id, account_record.account_status;
    END IF;
    
    -- Prevent transactions for blacklisted or deceased customers
    IF account_record.customer_status IN ('Blacklisted', 'Deceased', 'Dissolved') 
       AND NEW.transaction_code NOT IN ('CLOSURE_SETTLEMENT', 'ESTATE_SETTLEMENT') THEN
        RAISE EXCEPTION 'Customer is % and account cannot process transactions', account_record.customer_status;
    END IF;
    
    -- Set requires_approval based on account signing condition  
    IF account_record.signing_condition = 'AllOwners' AND NEW.amount > 1000 THEN
        NEW.requires_approval = TRUE;
        NEW.approval_status = 'Pending';
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to validate balance consistency
CREATE OR REPLACE FUNCTION validate_balance_update()
RETURNS TRIGGER AS $$
BEGIN
    -- Ensure available balance doesn't exceed current balance + overdraft
    IF NEW.available_balance > NEW.current_balance + COALESCE(NEW.overdraft_limit, 0) THEN
        RAISE EXCEPTION 'Available balance cannot exceed current balance plus overdraft limit';
    END IF;
    
    -- For loan accounts, ensure outstanding principal is within bounds
    IF NEW.account_type = 'Loan' THEN
        IF NEW.outstanding_principal IS NULL OR NEW.original_principal IS NULL THEN
            RAISE EXCEPTION 'Loan accounts must have principal amounts set';
        END IF;
        
        IF NEW.outstanding_principal > NEW.original_principal THEN
            RAISE EXCEPTION 'Outstanding principal cannot exceed original principal';
        END IF;
        
        IF NEW.outstanding_principal < 0 THEN
            RAISE EXCEPTION 'Outstanding principal cannot be negative';
        END IF;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to validate joint account ownership percentages
CREATE OR REPLACE FUNCTION validate_ownership_percentages()
RETURNS TRIGGER AS $$
DECLARE
    total_percentage DECIMAL(5,2);
    ownership_count INTEGER;
BEGIN
    -- Calculate total ownership percentage for the account
    SELECT COALESCE(SUM(ownership_percentage), 0), COUNT(*)
    INTO total_percentage, ownership_count
    FROM account_ownership
    WHERE account_id = NEW.account_id AND ownership_id != COALESCE(NEW.ownership_id, uuid_generate_v4());
    
    -- Add the new/updated percentage
    total_percentage := total_percentage + COALESCE(NEW.ownership_percentage, 0);
    
    -- For joint accounts, total should be 100%
    IF ownership_count > 0 AND NEW.ownership_type = 'Joint' THEN
        IF total_percentage > 100.01 THEN -- Allow small rounding errors
            RAISE EXCEPTION 'Total ownership percentage cannot exceed 100%%. Current total: %', total_percentage;
        END IF;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update last_activity_date on account transactions
CREATE OR REPLACE FUNCTION update_account_activity()
RETURNS TRIGGER AS $$
BEGIN
    -- Only update for customer-initiated transactions, not system transactions
    IF NEW.channel_id NOT IN ('SYSTEM', 'EOD_BATCH', 'COMPLIANCE') THEN
        UPDATE accounts 
        SET last_activity_date = CURRENT_DATE,
            last_updated_at = NOW()
        WHERE account_id = NEW.account_id;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to enforce hierarchical limits in agent network
CREATE OR REPLACE FUNCTION validate_agent_limits()
RETURNS TRIGGER AS $$
DECLARE
    terminal_record RECORD;
    branch_record RECORD;
    network_record RECORD;
BEGIN
    -- Get terminal, branch, and network information
    SELECT t.daily_transaction_limit as terminal_limit, 
           t.current_daily_volume as terminal_volume,
           b.daily_transaction_limit as branch_limit,
           b.current_daily_volume as branch_volume,
           n.aggregate_daily_limit as network_limit,
           n.current_daily_volume as network_volume
    INTO terminal_record
    FROM agent_terminals t
    JOIN agency_branches b ON t.branch_id = b.branch_id
    JOIN agent_networks n ON b.network_id = n.network_id
    WHERE t.terminal_id = NEW.terminal_id;
    
    -- Check terminal limit
    IF terminal_record.terminal_volume + NEW.amount > terminal_record.terminal_limit THEN
        RAISE EXCEPTION 'Transaction exceeds terminal daily limit. Limit: %, Current: %, Attempted: %',
                       terminal_record.terminal_limit, terminal_record.terminal_volume, NEW.amount;
    END IF;
    
    -- Check branch limit
    IF terminal_record.branch_volume + NEW.amount > terminal_record.branch_limit THEN
        RAISE EXCEPTION 'Transaction exceeds branch daily limit. Limit: %, Current: %, Attempted: %',
                       terminal_record.branch_limit, terminal_record.branch_volume, NEW.amount;
    END IF;
    
    -- Check network limit
    IF terminal_record.network_volume + NEW.amount > terminal_record.network_limit THEN
        RAISE EXCEPTION 'Transaction exceeds network daily limit. Limit: %, Current: %, Attempted: %',
                       terminal_record.network_limit, terminal_record.network_volume, NEW.amount;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update daily volumes on successful transactions
CREATE OR REPLACE FUNCTION update_daily_volumes()
RETURNS TRIGGER AS $$
BEGIN
    -- Only update volumes for posted transactions
    IF NEW.status = 'Posted' AND NEW.terminal_id IS NOT NULL THEN
        -- Update terminal volume
        UPDATE agent_terminals 
        SET current_daily_volume = current_daily_volume + NEW.amount
        WHERE terminal_id = NEW.terminal_id;
        
        -- Update branch volume
        UPDATE agency_branches 
        SET current_daily_volume = current_daily_volume + NEW.amount
        WHERE branch_id = (SELECT branch_id FROM agent_terminals WHERE terminal_id = NEW.terminal_id);
        
        -- Update network volume
        UPDATE agent_networks 
        SET current_daily_volume = current_daily_volume + NEW.amount
        WHERE network_id = (
            SELECT n.network_id 
            FROM agent_terminals t
            JOIN agency_branches b ON t.branch_id = b.branch_id
            JOIN agent_networks n ON b.network_id = n.network_id
            WHERE t.terminal_id = NEW.terminal_id
        );
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to enforce UBO verification requirements
CREATE OR REPLACE FUNCTION validate_ubo_requirements()
RETURNS TRIGGER AS $$
DECLARE
    corporate_type VARCHAR(20);
    total_percentage DECIMAL(5,2);
BEGIN
    -- Check that corporate customer is actually corporate
    SELECT customer_type INTO corporate_type
    FROM customers WHERE customer_id = NEW.corporate_customer_id;
    
    IF corporate_type != 'Corporate' THEN
        RAISE EXCEPTION 'UBO corporate customer must be of type Corporate';
    END IF;
    
    -- Check that beneficiary is individual
    SELECT customer_type INTO corporate_type
    FROM customers WHERE customer_id = NEW.beneficiary_customer_id;
    
    IF corporate_type != 'Individual' THEN
        RAISE EXCEPTION 'UBO beneficiary must be of type Individual';
    END IF;
    
    -- Check total ownership percentage doesn't exceed 100%
    SELECT COALESCE(SUM(ownership_percentage), 0) INTO total_percentage
    FROM ultimate_beneficiaries
    WHERE corporate_customer_id = NEW.corporate_customer_id 
    AND ubo_link_id != COALESCE(NEW.ubo_link_id, uuid_generate_v4())
    AND status = 'Active';
    
    total_percentage := total_percentage + COALESCE(NEW.ownership_percentage, 0);
    
    IF total_percentage > 100.01 THEN
        RAISE EXCEPTION 'Total UBO ownership percentage cannot exceed 100%%. Current total: %', total_percentage;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- =============================================================================
-- TRIGGER CREATION
-- =============================================================================

-- Audit trail triggers
CREATE TRIGGER trigger_customer_audit
    AFTER UPDATE ON customers
    FOR EACH ROW EXECUTE FUNCTION log_audit_trail();

CREATE TRIGGER trigger_account_audit
    AFTER UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION log_audit_trail();

-- Business rule enforcement triggers
CREATE TRIGGER trigger_validate_account_transaction
    BEFORE INSERT ON transactions
    FOR EACH ROW EXECUTE FUNCTION validate_account_transaction();

CREATE TRIGGER trigger_validate_balance_update
    BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION validate_balance_update();

CREATE TRIGGER trigger_validate_ownership_percentages
    BEFORE INSERT OR UPDATE ON account_ownership
    FOR EACH ROW EXECUTE FUNCTION validate_ownership_percentages();

CREATE TRIGGER trigger_update_account_activity
    AFTER INSERT ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_account_activity();

CREATE TRIGGER trigger_validate_agent_limits
    BEFORE INSERT ON transactions
    FOR EACH ROW 
    WHEN (NEW.terminal_id IS NOT NULL)
    EXECUTE FUNCTION validate_agent_limits();

CREATE TRIGGER trigger_update_daily_volumes
    AFTER UPDATE ON transactions
    FOR EACH ROW 
    WHEN (OLD.status IS DISTINCT FROM NEW.status AND NEW.status = 'Posted')
    EXECUTE FUNCTION update_daily_volumes();

CREATE TRIGGER trigger_validate_ubo_requirements
    BEFORE INSERT OR UPDATE ON ultimate_beneficiaries
    FOR EACH ROW EXECUTE FUNCTION validate_ubo_requirements();

-- =============================================================================
-- UTILITY FUNCTIONS FOR APPLICATION USE
-- =============================================================================

-- Function to reset daily volumes (called by EOD job)
CREATE OR REPLACE FUNCTION reset_daily_volumes()
RETURNS void AS $$
BEGIN
    UPDATE agent_terminals SET current_daily_volume = 0;
    UPDATE agency_branches SET current_daily_volume = 0;
    UPDATE agent_networks SET current_daily_volume = 0;
    
    RAISE NOTICE 'Daily volumes reset at %', NOW();
END;
$$ LANGUAGE plpgsql;

-- Function to get next business day
CREATE OR REPLACE FUNCTION get_next_business_day(input_date DATE, jurisdiction VARCHAR(10) DEFAULT 'US')
RETURNS DATE AS $$
DECLARE
    next_day DATE := input_date + INTERVAL '1 day';
    day_of_week INTEGER;
BEGIN
    -- Skip weekends and holidays
    LOOP
        day_of_week := EXTRACT(DOW FROM next_day);
        
        -- Check if it's weekend (Saturday = 6, Sunday = 0)
        IF day_of_week NOT IN (0, 6) THEN
            -- Check if it's not a holiday
            IF NOT EXISTS (
                SELECT 1 FROM bank_holidays 
                WHERE holiday_date = next_day 
                AND (bank_holidays.jurisdiction = get_next_business_day.jurisdiction OR bank_holidays.jurisdiction = 'ALL')
            ) THEN
                RETURN next_day;
            END IF;
        END IF;
        
        next_day := next_day + INTERVAL '1 day';
    END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Function to check if date is business day
CREATE OR REPLACE FUNCTION is_business_day(check_date DATE, jurisdiction VARCHAR(10) DEFAULT 'US')
RETURNS BOOLEAN AS $$
DECLARE
    day_of_week INTEGER;
BEGIN
    day_of_week := EXTRACT(DOW FROM check_date);
    
    -- Check if it's weekend
    IF day_of_week IN (0, 6) THEN
        RETURN FALSE;
    END IF;
    
    -- Check if it's a holiday
    IF EXISTS (
        SELECT 1 FROM bank_holidays 
        WHERE holiday_date = check_date 
        AND (bank_holidays.jurisdiction = is_business_day.jurisdiction OR bank_holidays.jurisdiction = 'ALL')
    ) THEN
        RETURN FALSE;
    END IF;
    
    RETURN TRUE;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate compound interest
CREATE OR REPLACE FUNCTION calculate_compound_interest(
    principal DECIMAL(15,2),
    rate DECIMAL(5,4),
    days INTEGER
) RETURNS DECIMAL(15,2) AS $$
BEGIN
    -- Daily compound interest: A = P(1 + r/365)^days
    RETURN principal * POWER(1 + rate/365.0, days) - principal;
END;
$$ LANGUAGE plpgsql;

-- Function to calculate simple interest
CREATE OR REPLACE FUNCTION calculate_simple_interest(
    principal DECIMAL(15,2),
    rate DECIMAL(5,4),
    days INTEGER
) RETURNS DECIMAL(15,2) AS $$
BEGIN
    -- Simple interest: I = P * r * t / 365
    RETURN principal * rate * days / 365.0;
END;
$$ LANGUAGE plpgsql;

-- Function to generate account number
CREATE OR REPLACE FUNCTION generate_account_number(product_code VARCHAR(50), branch_code VARCHAR(20))
RETURNS VARCHAR(20) AS $$
DECLARE
    sequence_number INTEGER;
    account_number VARCHAR(20);
BEGIN
    -- Get next sequence number for this product and branch
    sequence_number := nextval('account_number_seq');
    
    -- Format: ProductCode(3) + BranchCode(4) + SequentialNumber(10)
    account_number := LEFT(product_code, 3) || LEFT(branch_code, 4) || LPAD(sequence_number::TEXT, 10, '0');
    
    RETURN account_number;
END;
$$ LANGUAGE plpgsql;

-- Create sequence for account numbers
CREATE SEQUENCE IF NOT EXISTS account_number_seq START 1000000001;

-- =============================================================================
-- MAINTENANCE FUNCTIONS
-- =============================================================================

-- Function to archive old audit records
CREATE OR REPLACE FUNCTION archive_audit_records(months_to_keep INTEGER DEFAULT 36)
RETURNS INTEGER AS $$
DECLARE
    cutoff_date TIMESTAMP WITH TIME ZONE;
    deleted_count INTEGER;
BEGIN
    cutoff_date := NOW() - (months_to_keep || ' months')::INTERVAL;
    
    -- Archive customer audit trail
    DELETE FROM customer_audit_trail WHERE changed_at < cutoff_date;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    RAISE NOTICE 'Archived % customer audit records older than %', deleted_count, cutoff_date;
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Function to update account status to dormant based on inactivity
CREATE OR REPLACE FUNCTION mark_dormant_accounts()
RETURNS INTEGER AS $$
DECLARE
    dormant_count INTEGER := 0;
    account_record RECORD;
    default_threshold INTEGER := 365; -- Default 1 year
    threshold_days INTEGER;
BEGIN
    -- Find accounts eligible for dormancy
    FOR account_record IN
        SELECT account_id, product_code, last_activity_date, dormancy_threshold_days
        FROM accounts
        WHERE account_status = 'Active'
        AND last_activity_date IS NOT NULL
    LOOP
        threshold_days := COALESCE(account_record.dormancy_threshold_days, default_threshold);
        
        IF account_record.last_activity_date < CURRENT_DATE - (threshold_days || ' days')::INTERVAL THEN
            UPDATE accounts
            SET account_status = 'Dormant',
                last_updated_at = NOW(),
                updated_by = 'SYSTEM_DORMANCY_JOB',
                status_change_reason = 'Automatic dormancy due to inactivity',
                status_changed_by = 'SYSTEM',
                status_change_timestamp = NOW()
            WHERE account_id = account_record.account_id;
            
            dormant_count := dormant_count + 1;
        END IF;
    END LOOP;
    
    RAISE NOTICE 'Marked % accounts as dormant', dormant_count;
    RETURN dormant_count;
END;
$$ LANGUAGE plpgsql;