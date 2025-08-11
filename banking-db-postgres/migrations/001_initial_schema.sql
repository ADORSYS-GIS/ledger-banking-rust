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

-- Address type enum
CREATE TYPE address_type AS ENUM ('residential', 'business', 'mailing', 'temporary', 'branch', 'community', 'other');

-- Messaging type enum
CREATE TYPE messaging_type AS ENUM ('email', 'phone', 'sms', 'whatsapp', 'telegram', 'skype', 'teams', 'signal', 'wechat', 'viber', 'messenger', 'linkedin', 'slack', 'discord', 'other');

-- Service type enum for branch capabilities
CREATE TYPE service_type AS ENUM ('cash_withdrawal', 'cash_deposit', 'cash_transfer', 'bill_payment', 'account_opening', 'card_services', 'check_deposit', 'foreign_exchange', 'remittance_collection', 'agent_banking');

-- Certification status enum
CREATE TYPE certification_status AS ENUM ('active', 'expired', 'suspended', 'revoked');

-- Person entity type enum
CREATE TYPE person_entity_type AS ENUM ('customer', 'employee', 'shareholder', 'director', 'beneficialowner', 'agent', 'vendor', 'partner', 'regulatorycontact', 'emergencycontact', 'systemadmin', 'other');

-- Disbursement status enum
CREATE TYPE disbursement_status AS ENUM ('Pending', 'Approved', 'Executed', 'Cancelled', 'Failed', 'PartiallyExecuted');

-- Agent network type enum
CREATE TYPE network_type AS ENUM ('internal', 'partner', 'thirdparty');

-- Agent network status enum
CREATE TYPE network_status AS ENUM ('active', 'suspended', 'terminated');

-- Branch status enum
CREATE TYPE branch_status AS ENUM ('active', 'suspended', 'closed', 'temporarilyclosed');

-- Branch type enum
CREATE TYPE branch_type AS ENUM ('main_branch', 'sub_branch', 'agent_outlet', 'standalone_kiosk', 'partner_agent', 'atm_location', 'mobile_unit');

-- Terminal type enum
CREATE TYPE terminal_type AS ENUM ('pos', 'mobile', 'atm', 'webportal');

-- Terminal status enum
CREATE TYPE terminal_status AS ENUM ('active', 'maintenance', 'suspended', 'decommissioned');

-- Branch risk rating enum
CREATE TYPE branch_risk_rating AS ENUM ('low', 'medium', 'high', 'critical');

-- Customer related enums
CREATE TYPE customer_type AS ENUM ('Individual', 'Corporate');
CREATE TYPE identity_type AS ENUM ('NationalId', 'Passport', 'CompanyRegistration', 'PermanentResidentCard', 'AsylumCard', 'TemporaryResidentPermit', 'Unknown');
CREATE TYPE risk_rating AS ENUM ('Low', 'Medium', 'High', 'Blacklisted');
CREATE TYPE customer_status AS ENUM ('Active', 'PendingVerification', 'Deceased', 'Dissolved', 'Blacklisted');
CREATE TYPE kyc_status AS ENUM ('NotStarted', 'InProgress', 'Pending', 'Complete', 'Approved', 'Rejected', 'RequiresUpdate', 'Failed');
CREATE TYPE document_status AS ENUM ('Uploaded', 'Verified', 'Rejected', 'Expired');

-- Account related enums
CREATE TYPE account_type AS ENUM ('Savings', 'Current', 'Loan');
CREATE TYPE account_status AS ENUM ('PendingApproval', 'Active', 'Dormant', 'Frozen', 'PendingClosure', 'Closed', 'PendingReactivation');
CREATE TYPE signing_condition AS ENUM ('None', 'AnyOwner', 'AllOwners');
CREATE TYPE disbursement_method AS ENUM ('Transfer', 'CashWithdrawal', 'Check', 'HoldFunds', 'OverdraftFacility', 'StagedRelease');
-- hold_type has been updated to match the domain definition
DROP TYPE IF EXISTS hold_type CASCADE;
CREATE TYPE hold_type AS ENUM ('UnclearedFunds', 'JudicialLien', 'LoanPledge', 'ComplianceHold', 'AdministrativeHold', 'FraudHold', 'PendingAuthorization', 'OverdraftReserve', 'CardAuthorization', 'Other');
-- hold_status has been updated to match the domain definition
DROP TYPE IF EXISTS hold_status CASCADE;
CREATE TYPE hold_status AS ENUM ('Active', 'Released', 'Expired', 'Cancelled', 'PartiallyReleased');
-- hold_priority has been updated to match the domain definition
DROP TYPE IF EXISTS hold_priority CASCADE;
CREATE TYPE hold_priority AS ENUM ('Critical', 'High', 'Standard', 'Medium', 'Low');
CREATE TYPE ownership_type AS ENUM ('Single', 'Joint', 'Corporate');
CREATE TYPE entity_type AS ENUM ('Branch', 'Agent', 'RiskManager', 'ComplianceOfficer', 'CustomerService');
CREATE TYPE relationship_type AS ENUM ('PrimaryHandler', 'BackupHandler', 'RiskOversight', 'ComplianceOversight', 'Accountant');
CREATE TYPE relationship_status AS ENUM ('Active', 'Inactive', 'Suspended');
CREATE TYPE permission_type AS ENUM ('ViewOnly', 'LimitedWithdrawal', 'JointApproval', 'FullAccess');
CREATE TYPE mandate_status AS ENUM ('Active', 'Suspended', 'Revoked', 'Expired');

-- Transaction related enums
CREATE TYPE transaction_type AS ENUM ('Credit', 'Debit');
CREATE TYPE transaction_status AS ENUM ('Pending', 'Posted', 'Reversed', 'Failed', 'AwaitingApproval', 'ApprovalRejected');
CREATE TYPE transaction_approval_status AS ENUM ('Pending', 'Approved', 'Rejected', 'PartiallyApproved');
CREATE TYPE transaction_workflow_status AS ENUM ('Pending', 'Approved', 'Rejected', 'TimedOut');
CREATE TYPE transaction_audit_action AS ENUM ('Created', 'StatusChanged', 'Posted', 'Reversed', 'Failed', 'Approved', 'Rejected');
CREATE TYPE channel_type AS ENUM ('MobileApp', 'AgentTerminal', 'ATM', 'InternetBanking', 'BranchTeller', 'USSD', 'ApiGateway');
CREATE TYPE channel_status AS ENUM ('Active', 'Inactive', 'Maintenance', 'Suspended');
CREATE TYPE channel_fee_type AS ENUM ('TransactionFee', 'MaintenanceFee', 'ServiceFee', 'PenaltyFee', 'ProcessingFee', 'ComplianceFee', 'InterchangeFee', 'NetworkFee');
CREATE TYPE channel_fee_calculation_method AS ENUM ('Fixed', 'Percentage', 'Tiered', 'BalanceBased', 'RuleBased', 'Hybrid');
CREATE TYPE reconciliation_status AS ENUM ('InProgress', 'Completed', 'Failed', 'RequiresManualReview');
CREATE TYPE permitted_operation AS ENUM ('Credit', 'Debit', 'InterestPosting', 'FeeApplication', 'ClosureSettlement', 'None');
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

-- Collateral related enums
CREATE TYPE collateral_type AS ENUM ('ResidentialProperty', 'CommercialProperty', 'IndustrialProperty', 'Land', 'PassengerVehicle', 'CommercialVehicle', 'Motorcycle', 'Boat', 'Aircraft', 'CashDeposit', 'GovernmentSecurities', 'CorporateBonds', 'Stocks', 'MutualFunds', 'Inventory', 'AccountsReceivable', 'Equipment', 'Machinery', 'Jewelry', 'ArtAndAntiques', 'Electronics', 'PreciousMetals', 'AgriculturalProducts', 'Other');
CREATE TYPE collateral_category AS ENUM ('Immovable', 'Movable', 'Financial', 'Intangible');
CREATE TYPE custody_location AS ENUM ('BankVault', 'ThirdPartyCustodian', 'ClientPremises', 'RegisteredWarehouse', 'GovernmentRegistry', 'Other');
CREATE TYPE perfection_status AS ENUM ('Pending', 'Perfected', 'Imperfected', 'Expired', 'UnderDispute');
CREATE TYPE collateral_risk_rating AS ENUM ('Excellent', 'Good', 'Average', 'Poor', 'Unacceptable');
CREATE TYPE collateral_status AS ENUM ('Active', 'Released', 'PartiallyReleased', 'UnderReview', 'Disputed', 'Liquidated', 'Impaired', 'Substituted');
CREATE TYPE environmental_risk_level AS ENUM ('None', 'Low', 'Medium', 'High', 'Critical');
CREATE TYPE environmental_compliance_status AS ENUM ('Compliant', 'NonCompliant', 'UnderReview', 'RequiresRemediation');
CREATE TYPE insurance_coverage_type AS ENUM ('Comprehensive', 'FireAndTheft', 'AllRisk', 'SpecifiedPerils', 'MarineTransit', 'KeyPersonLife');
CREATE TYPE valuation_method AS ENUM ('MarketComparison', 'IncomeApproach', 'CostApproach', 'ExpertAppraisal', 'AuctionValue', 'BookValue', 'MarkToMarket');
CREATE TYPE pledge_priority AS ENUM ('First', 'Second', 'Third', 'Subordinate');
CREATE TYPE pledge_status AS ENUM ('Active', 'Released', 'PartiallyReleased', 'Substituted', 'Disputed', 'UnderEnforcement');
CREATE TYPE covenant_compliance_status AS ENUM ('Compliant', 'MinorBreach', 'MaterialBreach', 'Critical');
CREATE TYPE collateral_alert_type AS ENUM ('ValuationDue', 'ValuationOverdue', 'InsuranceExpiring', 'InsuranceExpired', 'PerfectionExpiring', 'PerfectionExpired', 'LtvBreach', 'CovenantBreach', 'MaintenanceRequired', 'DocumentationMissing', 'EnvironmentalRisk', 'MarketValueDecline');
CREATE TYPE alert_severity AS ENUM ('Low', 'Medium', 'High', 'Critical');
CREATE TYPE collateral_alert_status AS ENUM ('Open', 'InProgress', 'Resolved', 'Dismissed', 'Escalated');
CREATE TYPE concentration_category AS ENUM ('CollateralType', 'GeographicLocation', 'Industry', 'Borrower', 'CustodyLocation');
CREATE TYPE enforcement_type AS ENUM ('PrivateSale', 'PublicAuction', 'CourtOrdered', 'SelfHelp', 'ForeClosureAction', 'Repossession');
CREATE TYPE enforcement_method AS ENUM ('DirectSale', 'AuctionHouse', 'OnlinePlatform', 'BrokerSale', 'CourtSale', 'AssetManagementCompany');
CREATE TYPE enforcement_status AS ENUM ('Initiated', 'InProgress', 'Completed', 'Suspended', 'Cancelled', 'UnderLegalReview');

-- Reason and workflow related enums
CREATE TYPE reason_category AS ENUM (
    'LoanPurpose', 'LoanRejection', 'AccountClosure', 'AccountSuspension', 'AccountReactivation', 'StatusChange',
    'TransactionRejection', 'TransactionReversal', 'HoldReason', 'ComplianceFlag', 'AuditFinding',
    'AmlAlert', 'AmlInvestigation', 'SuspiciousActivity', 'CtfRiskFlag', 'SanctionsHit', 'PepFlag', 
    'HighRiskCountry', 'UnusualPattern', 'KycMissingDocument', 'KycDocumentRejection', 'KycVerificationFailure',
    'KycUpdateRequired', 'IdentityVerificationIssue', 'AddressVerificationIssue', 'SourceOfFundsRequired',
    'ComplaintReason', 'ServiceRequest', 'SystemGenerated', 'MaintenanceReason', 'Other'
);
CREATE TYPE reason_context AS ENUM ('Account', 'Loan', 'Transaction', 'Customer', 'Compliance', 'AmlCtf', 'Kyc', 'System', 'General');
CREATE TYPE reason_severity AS ENUM ('Critical', 'High', 'Medium', 'Low', 'Informational');
CREATE TYPE workflow_type AS ENUM ('AccountOpening', 'AccountClosure', 'LoanApplication', 'LoanDisbursement', 'TransactionApproval', 'ComplianceCheck', 'KycUpdate', 'DocumentVerification', 'CreditDecision', 'CollateralValuation', 'InterestRateChange', 'FeeWaiver', 'LimitChange', 'StatusChange', 'ManualIntervention');
CREATE TYPE workflow_status AS ENUM ('Pending', 'InProgress', 'Approved', 'Rejected', 'Cancelled', 'OnHold', 'Expired', 'Completed');
CREATE TYPE restructuring_type AS ENUM ('Rescheduling', 'Renewal', 'Refinancing', 'Restructuring', 'WriteOff');

-- Update existing transaction_audit_action to match API enum
DROP TYPE IF EXISTS transaction_audit_action CASCADE;
CREATE TYPE transaction_audit_action AS ENUM ('Create', 'Approve', 'Reject', 'Cancel', 'Reverse', 'Modify', 'StatusChange');

-- The hold_type enum is now aligned with the domain model.

-- The hold_status enum is now aligned with the domain model.

-- Update existing sar_status to match API enum
DROP TYPE IF EXISTS sar_status CASCADE;
CREATE TYPE sar_status AS ENUM ('Draft', 'Submitted', 'Acknowledged', 'UnderReview', 'Closed');

-- Daily Collection Service related enums
CREATE TYPE agent_status AS ENUM ('active', 'suspended', 'training', 'onleave', 'terminated');
CREATE TYPE area_type AS ENUM ('urban', 'suburban', 'rural', 'commercial', 'industrial', 'mixed');
CREATE TYPE customer_density AS ENUM ('high', 'medium', 'low');
CREATE TYPE transport_mode AS ENUM ('walking', 'bicycle', 'motorcycle', 'car', 'publictransport', 'mixed');
CREATE TYPE device_type AS ENUM ('smartphone', 'tablet', 'portableterminal', 'smartwatch');
CREATE TYPE connectivity_status AS ENUM ('online', 'offline', 'limitedconnectivity', 'syncpending');
CREATE TYPE collection_program_type AS ENUM ('fixedamount', 'variableamount', 'targetbased', 'durationbased');
CREATE TYPE program_status AS ENUM ('active', 'suspended', 'closed', 'underreview');
CREATE TYPE collection_frequency AS ENUM ('daily', 'weekly', 'monthly', 'quarterly', 'yearly');
CREATE TYPE collection_status AS ENUM ('active', 'suspended', 'defaulted', 'graduated', 'terminated');
CREATE TYPE holiday_handling AS ENUM ('skip', 'nextbusinessday', 'previousbusinessday', 'collectdouble');
CREATE TYPE reliability_rating AS ENUM ('excellent', 'good', 'fair', 'poor', 'critical');
CREATE TYPE collection_method AS ENUM ('cash', 'mobilepayment', 'banktransfer', 'digitalwallet');
CREATE TYPE collection_record_status AS ENUM ('pending', 'processed', 'failed', 'reversed', 'underreview');
CREATE TYPE biometric_method AS ENUM ('fingerprint', 'facerecognition', 'voiceprint', 'combined');
CREATE TYPE batch_status AS ENUM ('pending', 'processing', 'completed', 'failed', 'partiallyprocessed', 'requiresreconciliation');
CREATE TYPE fee_frequency AS ENUM ('percollection', 'daily', 'weekly', 'monthly', 'onetime');

-- Product catalogue enums
CREATE TYPE product_type AS ENUM ('CASA', 'LOAN');

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
-- MESSAGING TABLE - Must be created before persons table due to foreign key references
-- =============================================================================

-- Messaging table (without person references initially)
CREATE TABLE messaging (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    messaging_type messaging_type NOT NULL,
    value VARCHAR(100) NOT NULL,
    other_type VARCHAR(20),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    priority SMALLINT CHECK (priority > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- =============================================================================
-- PERSONS TABLE - Central person reference system
-- =============================================================================

CREATE TABLE persons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_type person_type NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    external_identifier VARCHAR(50),
    organization UUID, -- References PersonModel.id for organizational hierarchy
    
    -- Individual messaging fields (up to 5 messaging methods)
    messaging1_id UUID REFERENCES messaging(id),
    messaging1_type messaging_type,
    messaging2_id UUID REFERENCES messaging(id),
    messaging2_type messaging_type,
    messaging3_id UUID REFERENCES messaging(id),
    messaging3_type messaging_type,
    messaging4_id UUID REFERENCES messaging(id),
    messaging4_type messaging_type,
    messaging5_id UUID REFERENCES messaging(id),
    messaging5_type messaging_type,
    
    department VARCHAR(50),
    location UUID, -- References AddressModel.address_id for person's location (FK added later)
    duplicate_of UUID REFERENCES persons(id),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for persons
CREATE INDEX idx_persons_external_id ON persons(external_identifier) WHERE external_identifier IS NOT NULL;
-- Index on entity_reference removed since fields were moved to entity_reference table
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
-- GEOGRAPHIC AND LOCATION TABLES
-- =============================================================================

-- Countries table
CREATE TABLE countries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    iso2 CHAR(2) NOT NULL UNIQUE,
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- States/Provinces table
CREATE TABLE state_provinces (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_id UUID NOT NULL REFERENCES countries(id),
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Cities table
CREATE TABLE cities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    country_id UUID NOT NULL REFERENCES countries(id),
    state_id UUID REFERENCES state_provinces(id),
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Addresses table
CREATE TABLE addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    street_line1 VARCHAR(50) NOT NULL DEFAULT '',
    street_line2 VARCHAR(50) NOT NULL DEFAULT '',
    street_line3 VARCHAR(50) NOT NULL DEFAULT '',
    street_line4 VARCHAR(50) NOT NULL DEFAULT '',
    city_id UUID REFERENCES cities(id),
    postal_code VARCHAR(20),
    latitude DECIMAL(10,8),
    longitude DECIMAL(11,8),
    accuracy_meters REAL,
    address_type address_type NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Add person reference foreign keys to messaging table now that persons table exists
ALTER TABLE messaging ADD COLUMN created_by_person_id UUID REFERENCES persons(id);
ALTER TABLE messaging ADD COLUMN updated_by_person_id UUID REFERENCES persons(id);

-- Entity Reference table for person entity relationships
CREATE TABLE entity_reference (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    person_id UUID NOT NULL REFERENCES persons(id),
    entity_role person_entity_type NOT NULL,
    reference_external_id VARCHAR(50),
    reference_details_l1 VARCHAR(50),
    reference_details_l2 VARCHAR(50),
    reference_details_l3 VARCHAR(50),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Indexes for geographic tables
CREATE INDEX idx_countries_iso2 ON countries(iso2);
CREATE INDEX idx_state_provinces_country ON state_provinces(country_id);
CREATE INDEX idx_cities_country ON cities(country_id);
CREATE INDEX idx_cities_state ON cities(state_id) WHERE state_id IS NOT NULL;
CREATE INDEX idx_addresses_city ON addresses(city_id) WHERE city_id IS NOT NULL;
CREATE INDEX idx_addresses_coordinates ON addresses(latitude, longitude) WHERE latitude IS NOT NULL AND longitude IS NOT NULL;
CREATE INDEX idx_messaging_type ON messaging(messaging_type);
CREATE INDEX idx_messaging_value ON messaging(value);

-- Indexes for entity reference
CREATE INDEX idx_entity_reference_person ON entity_reference(person_id);
CREATE INDEX idx_entity_reference_type ON entity_reference(entity_role);
CREATE INDEX idx_entity_reference_active ON entity_reference(is_active) WHERE is_active = TRUE;

-- Indexes for holiday plans and schedules moved to after table creation

-- Indexes for required documents moved to after table creation

-- Indexes for compliance certifications moved to after table creation

-- Indexes for disbursement instructions moved to after table creation

-- Triggers for updated_at
CREATE TRIGGER tr_countries_updated_at BEFORE UPDATE ON countries FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_state_provinces_updated_at BEFORE UPDATE ON state_provinces FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_cities_updated_at BEFORE UPDATE ON cities FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_addresses_updated_at BEFORE UPDATE ON addresses FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_messaging_updated_at BEFORE UPDATE ON messaging FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_entity_reference_updated_at BEFORE UPDATE ON entity_reference FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
-- Triggers for holliday_plan and temporary_closure moved to after table creation
-- Trigger for disbursement_instructions moved to after table creation

-- =============================================================================
-- CUSTOMER INFORMATION FILE (CIF) - Master Repository
-- =============================================================================

-- Main customers table - Single source of truth for customer data
CREATE TABLE customers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_type customer_type NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    id_type identity_type NOT NULL,
    id_number VARCHAR(50) NOT NULL,
    risk_rating risk_rating NOT NULL DEFAULT 'Low',
    status customer_status NOT NULL DEFAULT 'PendingVerification',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id),
    
    -- Business constraints
    CONSTRAINT uk_customer_identity UNIQUE (id_type, id_number),
    CONSTRAINT ck_customer_name_length CHECK (LENGTH(TRIM(full_name)) >= 2),
    CONSTRAINT ck_id_number_length CHECK (LENGTH(TRIM(id_number)) >= 3)
);

-- Customer documents for KYC compliance
CREATE TABLE customer_documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    document_type VARCHAR(50) NOT NULL,
    document_path VARCHAR(500),
    status document_status NOT NULL DEFAULT 'Uploaded',
    uploaded_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    uploaded_by UUID NOT NULL REFERENCES persons(id),
    verified_at TIMESTAMP WITH TIME ZONE,
    verified_by UUID REFERENCES persons(id),
    
    CONSTRAINT ck_document_verification CHECK (
        (status IN ('Verified', 'Rejected') AND verified_at IS NOT NULL AND verified_by IS NOT NULL) OR
        (status IN ('Uploaded', 'Expired') AND verified_at IS NULL)
    )
);

-- Immutable audit trail for customer changes
CREATE TABLE customer_audit_trail (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    field_name VARCHAR(50) NOT NULL,
    old_value VARCHAR(255),
    new_value VARCHAR(255),
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    changed_by UUID NOT NULL REFERENCES persons(id),
    reason VARCHAR(255)
);

-- =============================================================================
-- REASON AND PURPOSE MULTILINGUAL SYSTEM
-- =============================================================================

-- Create the main reason_and_purpose table (moved up for foreign key dependencies)
CREATE TABLE reason_and_purpose (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) UNIQUE NOT NULL,
    category reason_category NOT NULL,
    context reason_context NOT NULL,
    
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
    severity reason_severity,
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
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id VARCHAR(100) NOT NULL,
    updated_by_person_id VARCHAR(100) NOT NULL
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
-- ACCOUNT HOLDS - Must be defined before accounts table due to FK reference
-- =============================================================================

-- Account holds table - temporary restrictions on account balances
CREATE TABLE account_holds (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL, -- FK to accounts(id) - added after accounts table creation
    amount DECIMAL(15,2) NOT NULL CHECK (amount > 0),
    hold_type hold_type NOT NULL,
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for hold reason
    additional_details VARCHAR(200), -- Additional context beyond the standard reason
    placed_by_person_id UUID NOT NULL REFERENCES persons(id),
    placed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    status hold_status NOT NULL DEFAULT 'Active',
    released_at TIMESTAMP WITH TIME ZONE,
    released_by_person_id UUID REFERENCES persons(id),
    priority hold_priority NOT NULL DEFAULT 'Medium',
    source_reference VARCHAR(100), -- External reference for judicial holds, etc.
    automatic_release BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_hold_release_consistency CHECK (
        (status NOT IN ('Released', 'PartiallyReleased') AND released_at IS NULL) OR
        (status IN ('Released', 'PartiallyReleased') AND released_at IS NOT NULL AND released_by_person_id IS NOT NULL)
    ),
    CONSTRAINT ck_hold_expiry_consistency CHECK (
        (automatic_release = FALSE AND expires_at IS NULL) OR
        (automatic_release = TRUE AND expires_at IS NOT NULL)
    )
);

-- Account ownership table - ownership records for accounts
CREATE TABLE account_ownership (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL, -- FK to accounts(id) - added after accounts table creation
    customer_id UUID NOT NULL REFERENCES customers(id),
    ownership_type ownership_type NOT NULL,
    ownership_percentage DECIMAL(5,2) DEFAULT 100.00 CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT uk_account_primary_owner UNIQUE (account_id, customer_id)
);

-- Account relationships for servicing and internal bank operations
CREATE TABLE account_relationships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL, -- FK to accounts(id) - added after accounts table creation
    person_id UUID NOT NULL, -- Could be employee, department, or external entity
    entity_type entity_type NOT NULL,
    relationship_type relationship_type NOT NULL,
    status relationship_status NOT NULL DEFAULT 'Active',
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,
    
    CONSTRAINT ck_relationship_dates CHECK (end_date IS NULL OR end_date >= start_date)
);

-- Account mandates for operational permissions
CREATE TABLE account_mandates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL, -- FK to accounts(id) - added after accounts table creation
    grantee_customer_id UUID NOT NULL REFERENCES customers(id), -- References external person or customer
    permission_type permission_type NOT NULL,
    transaction_limit DECIMAL(15,2),
    approver01_person_id UUID REFERENCES persons(id),
    approver02_person_id UUID REFERENCES persons(id),
    approver03_person_id UUID REFERENCES persons(id),
    approver04_person_id UUID REFERENCES persons(id),
    approver05_person_id UUID REFERENCES persons(id),
    approver06_person_id UUID REFERENCES persons(id),
    approver07_person_id UUID REFERENCES persons(id),
    required_signers_count SMALLINT NOT NULL DEFAULT 1 CHECK (required_signers_count >= 1 AND required_signers_count <= 7),
    conditional_mandate_id UUID REFERENCES account_mandates(id),
    status mandate_status NOT NULL DEFAULT 'Active',
    start_date DATE NOT NULL DEFAULT CURRENT_DATE,
    end_date DATE,
    
    CONSTRAINT ck_mandate_dates CHECK (end_date IS NULL OR end_date >= start_date)
);

-- Ultimate Beneficial Owner (UBO) tracking for corporate accounts - regulatory requirement
CREATE TABLE ultimate_beneficial_owners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    corporate_customer_id UUID NOT NULL REFERENCES customers(id),
    beneficiary_customer_id UUID NOT NULL REFERENCES customers(id),
    ownership_percentage DECIMAL(5,2) CHECK (ownership_percentage > 0 AND ownership_percentage <= 100),
    control_type control_type NOT NULL,
    description VARCHAR(256),
    status ubo_status NOT NULL DEFAULT 'Active',
    verification_status verification_status NOT NULL DEFAULT 'Pending',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_ubo_not_self_reference CHECK (corporate_customer_id != beneficiary_customer_id)
);

-- =============================================================================
-- UNIFIED ACCOUNT MODEL (UAM) - Multi-Product Support
-- =============================================================================

-- Comprehensive accounts table supporting all banking products
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL,
    account_type account_type NOT NULL,
    account_status account_status NOT NULL DEFAULT 'Active',
    signing_condition signing_condition NOT NULL DEFAULT 'None',
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    open_date DATE NOT NULL DEFAULT CURRENT_DATE,
    domicile_agency_branch_id UUID NOT NULL,
    gl_code_suffix VARCHAR(10),
    
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
    collateral_id UUID, -- Reference to collateral
    loan_purpose_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for loan purpose
    
    -- Account lifecycle management (from enhancements)
    close_date DATE,
    last_activity_date DATE,
    dormancy_threshold_days INTEGER DEFAULT 365, -- Days of inactivity before dormancy
    reactivation_required BOOLEAN DEFAULT FALSE,
    pending_closure_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for closure
    last_disbursement_instruction_id UUID, -- References DisbursementInstructions.disbursement_id
    
    -- Enhanced audit trail
    status_changed_by_person_id UUID REFERENCES persons(id),
    status_change_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for status change
    status_change_timestamp TIMESTAMP WITH TIME ZONE,
    
    -- Direct reference fields for most significant related entities
    most_significant_account_hold_id UUID REFERENCES account_holds(id),
    account_ownership_id UUID REFERENCES account_ownership(id),
    access01_account_relationship_id UUID REFERENCES account_relationships(id),
    access02_account_relationship_id UUID REFERENCES account_relationships(id),
    access03_account_relationship_id UUID REFERENCES account_relationships(id),
    access04_account_relationship_id UUID REFERENCES account_relationships(id),
    access05_account_relationship_id UUID REFERENCES account_relationships(id),
    access06_account_relationship_id UUID REFERENCES account_relationships(id),
    access07_account_relationship_id UUID REFERENCES account_relationships(id),
    access11_account_mandate_id UUID REFERENCES account_mandates(id),
    access12_account_mandate_id UUID REFERENCES account_mandates(id),
    access13_account_mandate_id UUID REFERENCES account_mandates(id),
    access14_account_mandate_id UUID REFERENCES account_mandates(id),
    access15_account_mandate_id UUID REFERENCES account_mandates(id),
    access16_account_mandate_id UUID REFERENCES account_mandates(id),
    access17_account_mandate_id UUID REFERENCES account_mandates(id),
    interest01_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    interest02_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    interest03_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    interest04_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    interest05_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    interest06_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    interest07_ultimate_beneficiary_id UUID REFERENCES ultimate_beneficial_owners(id),
    
    -- Audit fields
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id),
    
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

-- Add foreign key constraints for circular dependencies after accounts table creation
ALTER TABLE account_holds ADD CONSTRAINT fk_account_holds_account_id 
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE;

ALTER TABLE account_ownership ADD CONSTRAINT fk_account_ownership_account_id 
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE;

ALTER TABLE account_relationships ADD CONSTRAINT fk_account_relationships_account_id 
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE;

ALTER TABLE account_mandates ADD CONSTRAINT fk_account_mandates_account_id 
    FOREIGN KEY (account_id) REFERENCES accounts(id) ON DELETE CASCADE;

-- Disbursement Instructions table
CREATE TABLE disbursement_instructions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_account_id UUID NOT NULL REFERENCES accounts(id),
    method disbursement_method NOT NULL,
    target_account_id UUID REFERENCES accounts(id),
    cash_pickup_agency_branch_id UUID, -- Foreign key added later after agent_branches is created
    authorized_recipient_person_id UUID REFERENCES persons(id),
    
    -- Disbursement tracking and staging
    disbursement_amount DECIMAL(15,2),
    disbursement_date DATE,
    stage_number INTEGER,
    stage_description VARCHAR(200),
    status disbursement_status NOT NULL DEFAULT 'Pending',
    
    -- Audit trail
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Indexes for disbursement instructions
CREATE INDEX idx_disbursement_instructions_source ON disbursement_instructions(source_account_id);
CREATE INDEX idx_disbursement_instructions_status ON disbursement_instructions(status);
CREATE INDEX idx_disbursement_instructions_date ON disbursement_instructions(disbursement_date);

-- Trigger for disbursement instructions
CREATE TRIGGER tr_disbursement_instructions_updated_at BEFORE UPDATE ON disbursement_instructions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Add foreign key constraint for accounts table after disbursement_instructions is created
ALTER TABLE accounts ADD CONSTRAINT fk_accounts_last_disbursement 
    FOREIGN KEY (last_disbursement_instruction_id) REFERENCES disbursement_instructions(id);

-- =============================================================================
-- TRANSACTION PROCESSING ENGINE
-- =============================================================================

-- Comprehensive transaction table with multi-stage processing support
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    transaction_code VARCHAR(8) NOT NULL, -- Internal transaction type code
    transaction_type transaction_type NOT NULL,
    amount DECIMAL(15,2) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    description VARCHAR(200) NOT NULL,
    
    -- Multi-channel processing support
    channel_id VARCHAR(50) NOT NULL, -- 'BranchTeller', 'ATM', 'OnlineBanking', 'MobileBanking', 'AgentBanking'
    terminal_id UUID, -- For ATM/POS transactions, references terminals table
    agent_user_id UUID, -- For agent banking transactions
    
    -- Transaction timing
    transaction_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    value_date DATE NOT NULL, -- When the transaction affects the balance
    
    -- Status and approval workflow
    status transaction_status NOT NULL DEFAULT 'Pending',
    reference_number VARCHAR(200) NOT NULL UNIQUE,
    external_reference VARCHAR(100), -- For external system correlation
    
    -- Financial posting integration
    gl_code VARCHAR(10) NOT NULL, -- General Ledger account code
    
    -- Risk and compliance
    requires_approval BOOLEAN NOT NULL DEFAULT FALSE,
    approval_status transaction_approval_status,
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    action_type transaction_audit_action NOT NULL,
    performed_by UUID NOT NULL REFERENCES persons(id),
    performed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    old_status transaction_status,
    new_status transaction_status,  
    reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for audit reason
    details BYTEA -- Blake3 hash of additional details for tamper detection
);

-- General Ledger Entries table
CREATE TABLE gl_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    account_code UUID NOT NULL, -- References GL account structure
    debit_amount DECIMAL(15,2) CHECK (debit_amount >= 0),
    credit_amount DECIMAL(15,2) CHECK (credit_amount >= 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    description VARCHAR(200) NOT NULL,
    reference_number VARCHAR(200) NOT NULL,
    value_date DATE NOT NULL,
    posting_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_gl_entry_amount CHECK (
        (debit_amount IS NOT NULL AND credit_amount IS NULL) OR
        (debit_amount IS NULL AND credit_amount IS NOT NULL)
    ),
    CONSTRAINT ck_gl_currency_format CHECK (LENGTH(currency) = 3 AND currency = UPPER(currency))
);

-- Transaction Request table for tracking requests
CREATE TABLE transaction_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    transaction_type transaction_type NOT NULL,
    amount DECIMAL(15,2) NOT NULL CHECK (amount > 0),
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    description VARCHAR(200) NOT NULL,
    channel VARCHAR(20) NOT NULL CHECK (channel IN ('MobileApp', 'AgentTerminal', 'ATM', 'InternetBanking', 'BranchTeller', 'USSD', 'ApiGateway')),
    terminal_id UUID,
    initiator_id UUID NOT NULL REFERENCES persons(id),
    external_reference VARCHAR(100),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_request_currency_format CHECK (LENGTH(currency) = 3 AND currency = UPPER(currency))
);

-- Transaction Results table
CREATE TABLE transaction_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    reference_number VARCHAR(200) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Validation Results table
CREATE TABLE validation_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID REFERENCES transactions(id),
    is_valid BOOLEAN NOT NULL DEFAULT FALSE,
    errors JSONB, -- JSON array of error messages
    warnings JSONB, -- JSON array of warning messages
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Approval table (separate from approval workflow)
CREATE TABLE approvals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    approver_id UUID NOT NULL REFERENCES persons(id),
    approved_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_gl_entries_transaction ON gl_entries(transaction_id);
CREATE INDEX idx_gl_entries_account_code ON gl_entries(account_code);
CREATE INDEX idx_gl_entries_posting_date ON gl_entries(posting_date);
CREATE INDEX idx_transaction_requests_account ON transaction_requests(account_id);
CREATE INDEX idx_transaction_requests_initiator ON transaction_requests(initiator_id);
CREATE INDEX idx_transaction_results_transaction ON transaction_results(transaction_id);
CREATE INDEX idx_validation_results_transaction ON validation_results(transaction_id);
CREATE INDEX idx_approvals_transaction ON approvals(transaction_id);
CREATE INDEX idx_approvals_approver ON approvals(approver_id);

-- =============================================================================
-- AGENT BANKING NETWORK MANAGEMENT
-- =============================================================================

-- Holiday Plan table
CREATE TABLE holliday_plan (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Holiday Schedule table
CREATE TABLE holiday_schedule (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    holiday_plan_id UUID NOT NULL REFERENCES holliday_plan(id),
    date DATE NOT NULL,
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    is_closed BOOLEAN NOT NULL DEFAULT TRUE,
    special_open_time TIME,
    special_close_time TIME,
    special_break_start TIME,
    special_break_end TIME
);

-- Temporary Closure table
CREATE TABLE temporary_closure (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    start_date DATE NOT NULL,
    end_date DATE,
    closure_reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    additional_details_l1 VARCHAR(100),
    additional_details_l2 VARCHAR(100),
    additional_details_l3 VARCHAR(100),
    alternative_agency_branch_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Required Document table moved before branch_capabilities

-- Compliance Certification table
CREATE TABLE compliance_cert (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    certification_name_l1 VARCHAR(100) NOT NULL,
    certification_name_l2 VARCHAR(100),
    certification_name_l3 VARCHAR(100),
    issuer_person_id UUID NOT NULL REFERENCES persons(id),
    issue_date DATE NOT NULL,
    expiry_date DATE,
    status certification_status NOT NULL DEFAULT 'active'
);

-- Operating Hours standalone table
CREATE TABLE operating_hours (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    monday_open TIME,
    monday_close TIME,
    monday_break_start TIME,
    monday_break_end TIME,
    tuesday_open TIME,
    tuesday_close TIME,
    tuesday_break_start TIME,
    tuesday_break_end TIME,
    wednesday_open TIME,
    wednesday_close TIME,
    wednesday_break_start TIME,
    wednesday_break_end TIME,
    thursday_open TIME,
    thursday_close TIME,
    thursday_break_start TIME,
    thursday_break_end TIME,
    friday_open TIME,
    friday_close TIME,
    friday_break_start TIME,
    friday_break_end TIME,
    saturday_open TIME,
    saturday_close TIME,
    saturday_break_start TIME,
    saturday_break_end TIME,
    sunday_open TIME,
    sunday_close TIME,
    sunday_break_start TIME,
    sunday_break_end TIME,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Required Document table (moved here to resolve dependencies)
CREATE TABLE required_document (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_type_l1 VARCHAR(50) NOT NULL,
    document_type_l2 VARCHAR(50),
    document_type_l3 VARCHAR(50),
    is_mandatory BOOLEAN NOT NULL DEFAULT FALSE,
    alternative1_document_id UUID,
    alternative2_document_id UUID,
    alternative3_document_id UUID
);

-- Branch Capabilities standalone table
CREATE TABLE branch_capabilities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    -- Supported services (up to 10 individual fields)
    supported_service1 service_type,
    supported_service2 service_type,
    supported_service3 service_type,
    supported_service4 service_type,
    supported_service5 service_type,
    supported_service6 service_type,
    supported_service7 service_type,
    supported_service8 service_type,
    supported_service9 service_type,
    supported_service10 service_type,
    -- Supported currencies (up to 3 individual fields)
    supported_currency1 VARCHAR(3),
    supported_currency2 VARCHAR(3),
    supported_currency3 VARCHAR(3),
    -- Languages spoken (up to 3 individual fields)
    language_spoken1 VARCHAR(3),
    language_spoken2 VARCHAR(3),
    language_spoken3 VARCHAR(3),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Security Access standalone table
CREATE TABLE security_access (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    -- Security features (flattened)
    has_security_guard BOOLEAN NOT NULL DEFAULT FALSE,
    has_cctv BOOLEAN NOT NULL DEFAULT FALSE,
    has_panic_button BOOLEAN NOT NULL DEFAULT FALSE,
    has_safe BOOLEAN NOT NULL DEFAULT FALSE,
    has_biometric_verification BOOLEAN NOT NULL DEFAULT FALSE,
    police_station_distance_km REAL,
    -- Accessibility features (flattened)
    wheelchair_accessible BOOLEAN NOT NULL DEFAULT FALSE,
    has_ramp BOOLEAN NOT NULL DEFAULT FALSE,
    has_braille_signage BOOLEAN NOT NULL DEFAULT FALSE,
    has_audio_assistance BOOLEAN NOT NULL DEFAULT FALSE,
    has_sign_language_support BOOLEAN NOT NULL DEFAULT FALSE,
    parking_available BOOLEAN NOT NULL DEFAULT FALSE,
    public_transport_nearby BOOLEAN NOT NULL DEFAULT FALSE,
    -- Required documents (up to 20 individual references)
    required_document1 UUID REFERENCES required_document(id),
    required_document2 UUID REFERENCES required_document(id),
    required_document3 UUID REFERENCES required_document(id),
    required_document4 UUID REFERENCES required_document(id),
    required_document5 UUID REFERENCES required_document(id),
    required_document6 UUID REFERENCES required_document(id),
    required_document7 UUID REFERENCES required_document(id),
    required_document8 UUID REFERENCES required_document(id),
    required_document9 UUID REFERENCES required_document(id),
    required_document10 UUID REFERENCES required_document(id),
    required_document11 UUID REFERENCES required_document(id),
    required_document12 UUID REFERENCES required_document(id),
    required_document13 UUID REFERENCES required_document(id),
    required_document14 UUID REFERENCES required_document(id),
    required_document15 UUID REFERENCES required_document(id),
    required_document16 UUID REFERENCES required_document(id),
    required_document17 UUID REFERENCES required_document(id),
    required_document18 UUID REFERENCES required_document(id),
    required_document19 UUID REFERENCES required_document(id),
    required_document20 UUID REFERENCES required_document(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Agent networks - hierarchical structure for agent banking
CREATE TABLE agent_networks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    network_name VARCHAR(100) NOT NULL,
    network_type network_type NOT NULL,
    status network_status NOT NULL DEFAULT 'active',
    contract_external_id VARCHAR(50),
    aggregate_daily_limit DECIMAL(15,2) NOT NULL,
    current_daily_volume DECIMAL(15,2) NOT NULL DEFAULT 0,
    settlement_gl_code VARCHAR(8) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Agent branches within networks
CREATE TABLE agent_branches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agent_network_id UUID NOT NULL REFERENCES agent_networks(id),
    parent_agency_branch_id UUID REFERENCES agent_branches(id),
    branch_name VARCHAR(100) NOT NULL,
    branch_code VARCHAR(8) NOT NULL,
    branch_level INTEGER NOT NULL DEFAULT 1,
    gl_code_prefix VARCHAR(6) NOT NULL,
    status branch_status NOT NULL DEFAULT 'active',
    daily_transaction_limit DECIMAL(15,2) NOT NULL,
    current_daily_volume DECIMAL(15,2) NOT NULL DEFAULT 0,
    max_cash_limit DECIMAL(15,2) NOT NULL,
    current_cash_balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    minimum_cash_balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Location fields
    address_id UUID NOT NULL REFERENCES addresses(id),
    landmark_description VARCHAR(200),
    
    -- Operational details
    operating_hours_id UUID NOT NULL REFERENCES operating_hours(id),
    holiday_plan_id UUID NOT NULL REFERENCES holliday_plan(id),
    temporary_closure_id UUID REFERENCES temporary_closure(id),
    
    -- Contact information - individual messaging fields (up to 5 entries)
    messaging1_id UUID REFERENCES messaging(id),
    messaging1_type messaging_type,
    messaging2_id UUID REFERENCES messaging(id),
    messaging2_type messaging_type,
    messaging3_id UUID REFERENCES messaging(id),
    messaging3_type messaging_type,
    messaging4_id UUID REFERENCES messaging(id),
    messaging4_type messaging_type,
    messaging5_id UUID REFERENCES messaging(id),
    messaging5_type messaging_type,
    branch_manager_person_id UUID,
    
    -- Services and capabilities
    branch_type branch_type NOT NULL,
    branch_capabilities_id UUID NOT NULL REFERENCES branch_capabilities(id),
    
    -- Security and access
    security_access_id UUID NOT NULL REFERENCES security_access(id),
    
    -- Customer capacity
    max_daily_customers INTEGER,
    average_wait_time_minutes SMALLINT,
    
    -- Transaction limits
    per_transaction_limit DECIMAL(15,2) NOT NULL,
    monthly_transaction_limit DECIMAL(15,2),
    
    -- Compliance and risk
    risk_rating branch_risk_rating NOT NULL DEFAULT 'low',
    last_audit_date DATE,
    last_compliance_certification_id UUID REFERENCES compliance_cert(id),
    
    -- Metadata
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id),
    
    CONSTRAINT uk_branch_code_per_network UNIQUE (agent_network_id, branch_code)
);


-- Agent terminals - individual POS/mobile terminals
CREATE TABLE agent_terminals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    agency_branch_id UUID NOT NULL REFERENCES agent_branches(id),
    agent_person_id UUID NOT NULL,
    terminal_type terminal_type NOT NULL,
    terminal_name VARCHAR(100) NOT NULL,
    daily_transaction_limit DECIMAL(15,2) NOT NULL,
    current_daily_volume DECIMAL(15,2) NOT NULL DEFAULT 0,
    max_cash_limit DECIMAL(15,2) NOT NULL,
    current_cash_balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    minimum_cash_balance DECIMAL(15,2) NOT NULL DEFAULT 0,
    status terminal_status NOT NULL DEFAULT 'active',
    last_sync_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Triggers for agent network tables
CREATE TRIGGER tr_agent_networks_updated_at BEFORE UPDATE ON agent_networks FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_agent_branches_updated_at BEFORE UPDATE ON agent_branches FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_agent_terminals_updated_at BEFORE UPDATE ON agent_terminals FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_operating_hours_updated_at BEFORE UPDATE ON operating_hours FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_branch_capabilities_updated_at BEFORE UPDATE ON branch_capabilities FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_security_access_updated_at BEFORE UPDATE ON security_access FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Add foreign key constraints that were deferred
ALTER TABLE disbursement_instructions ADD CONSTRAINT fk_disbursement_instructions_branch 
    FOREIGN KEY (cash_pickup_agency_branch_id) REFERENCES agent_branches(id);

-- =============================================================================
-- CALENDAR AND BUSINESS DAY MANAGEMENT
-- Indexes for agent network tables
CREATE INDEX idx_holliday_plan_l1 ON holliday_plan(name_l1);
CREATE INDEX idx_holiday_schedule_plan ON holiday_schedule(holiday_plan_id);
CREATE INDEX idx_holiday_schedule_date ON holiday_schedule(date);

-- Indexes for temporary closure
CREATE INDEX idx_temporary_closure_reason ON temporary_closure(closure_reason_id);
CREATE INDEX idx_temporary_closure_dates ON temporary_closure(start_date, end_date);

-- Indexes for required documents
CREATE INDEX idx_required_document_l1 ON required_document(document_type_l1);
CREATE INDEX idx_required_document_mandatory ON required_document(is_mandatory) WHERE is_mandatory = TRUE;

-- Indexes for compliance certifications
CREATE INDEX idx_compliance_cert_issuer ON compliance_cert(issuer_person_id);
CREATE INDEX idx_compliance_cert_status ON compliance_cert(status);
CREATE INDEX idx_compliance_cert_expiry ON compliance_cert(expiry_date);

-- Triggers for agent network tables
CREATE TRIGGER tr_holliday_plan_updated_at BEFORE UPDATE ON holliday_plan FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER tr_temporary_closure_updated_at BEFORE UPDATE ON temporary_closure FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================

-- Bank holidays for business day calculations
CREATE TABLE bank_holidays (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL, -- Country/region code
    holiday_date DATE NOT NULL,
    holiday_name VARCHAR(255) NOT NULL,
    holiday_type holiday_type NOT NULL,
    is_recurring BOOLEAN NOT NULL DEFAULT FALSE,
    description VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    
    CONSTRAINT uk_holiday_per_jurisdiction UNIQUE (jurisdiction, holiday_date)
);

-- Weekend configuration per jurisdiction
CREATE TABLE weekend_configuration (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL UNIQUE,
    weekend_days VARCHAR(100) NOT NULL, -- JSON array of weekday numbers (0=Sunday, 1=Monday, etc.)
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    notes VARCHAR(256),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Date calculation rules for business day calculations
CREATE TABLE date_calculation_rules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id),
    
    CONSTRAINT ck_rule_dates CHECK (expiry_date IS NULL OR expiry_date > effective_date)
);

-- Holiday import log for audit trail of holiday imports
CREATE TABLE holiday_import_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    jurisdiction VARCHAR(10) NOT NULL,
    import_year INTEGER NOT NULL,
    import_source VARCHAR(100) NOT NULL,
    holidays_imported INTEGER NOT NULL DEFAULT 0,
    holidays_updated INTEGER NOT NULL DEFAULT 0,
    holidays_skipped INTEGER NOT NULL DEFAULT 0,
    import_status import_status NOT NULL,
    error_details VARCHAR(1000),
    imported_by_person_id UUID NOT NULL REFERENCES persons(id),
    imported_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Business day cache for performance optimization
CREATE TABLE business_day_cache (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
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
-- PRODUCT CATALOGUE
-- =============================================================================

CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_type product_type NOT NULL,
    product_code VARCHAR(50) NOT NULL,
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100),
    description VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    valid_from DATE NOT NULL,
    valid_to DATE,
    rules JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_by_person_id UUID NOT NULL REFERENCES persons(id),

    CONSTRAINT ck_product_dates CHECK (valid_to IS NULL OR valid_to >= valid_from)
);

CREATE INDEX idx_products_type ON products(product_type);
CREATE INDEX idx_products_active ON products(is_active) WHERE is_active = TRUE;

CREATE TRIGGER update_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- =============================================================================
-- WORKFLOW AND APPROVAL MANAGEMENT
-- =============================================================================

-- Account workflow management
CREATE TABLE account_workflows (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    workflow_type workflow_type NOT NULL,
    current_step VARCHAR(50) NOT NULL CHECK (current_step IN (
        'InitiateRequest', 'ComplianceCheck', 'DocumentVerification',
        'ApprovalRequired', 'FinalSettlement', 'Completed'
    )),
    status VARCHAR(20) NOT NULL CHECK (status IN (
        'InProgress', 'PendingAction', 'Completed', 'Failed', 'Cancelled', 'TimedOut'
    )),
    initiated_by UUID NOT NULL REFERENCES persons(id),
    initiated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    next_action_required VARCHAR(500),
    timeout_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Workflow step records
CREATE TABLE workflow_step_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(id) ON DELETE CASCADE,
    step VARCHAR(50) NOT NULL CHECK (step IN (
        'InitiateRequest', 'ComplianceCheck', 'DocumentVerification',
        'ApprovalRequired', 'FinalSettlement', 'Completed'
    )),
    completed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    completed_by UUID NOT NULL REFERENCES persons(id),
    notes VARCHAR(500),
    supporting_documents JSONB, -- Array of document names/IDs
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Account opening requests (workflow-specific data)
CREATE TABLE account_opening_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL REFERENCES customers(id),
    product_id UUID NOT NULL REFERENCES products(id),
    initial_deposit DECIMAL(15,2),
    channel VARCHAR(50) NOT NULL,
    initiated_by UUID NOT NULL REFERENCES persons(id),
    supporting_documents JSONB, -- Array of document references
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Closure requests (workflow-specific data)
CREATE TABLE closure_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(id) ON DELETE CASCADE,
    reason VARCHAR(50) NOT NULL CHECK (reason IN (
        'CustomerRequest', 'Regulatory', 'Compliance', 'Dormancy', 'SystemMaintenance'
    )),
    requested_by UUID NOT NULL REFERENCES persons(id),
    force_closure BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Final settlements (workflow-specific data)
CREATE TABLE final_settlements (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(id) ON DELETE CASCADE,
    current_balance DECIMAL(15,2) NOT NULL,
    accrued_interest DECIMAL(15,2) NOT NULL DEFAULT 0,
    pending_fees DECIMAL(15,2) NOT NULL DEFAULT 0,
    closure_fees DECIMAL(15,2) NOT NULL DEFAULT 0,
    final_amount DECIMAL(15,2) NOT NULL,
    requires_disbursement BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Dormancy assessments (workflow-specific data)
CREATE TABLE dormancy_assessments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(id) ON DELETE CASCADE,
    is_eligible BOOLEAN NOT NULL,
    last_activity_date DATE,
    days_inactive INTEGER NOT NULL,
    threshold_days INTEGER NOT NULL,
    product_specific_rules JSONB, -- Array of rule descriptions
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Document references (for workflows)
CREATE TABLE document_references (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID REFERENCES account_workflows(id) ON DELETE CASCADE,
    document_id BYTEA NOT NULL, -- Blake3 hash
    document_type VARCHAR(50) NOT NULL,
    document_path BYTEA, -- Blake3 hash of path content
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Transaction approval workflow
CREATE TABLE transaction_approvals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    workflow_id UUID NOT NULL REFERENCES account_workflows(id),
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    approver_id UUID NOT NULL,
    approval_action transaction_approval_status NOT NULL,
    approved_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    approval_notes VARCHAR(512),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Account status history with enhanced tracking
CREATE TABLE account_status_change_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    old_status VARCHAR(30),
    new_status VARCHAR(30) NOT NULL,
    reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for status change
    additional_context VARCHAR(200), -- Additional context beyond the standard reason
    changed_by_person_id UUID NOT NULL REFERENCES persons(id),
    changed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    system_triggered BOOLEAN NOT NULL DEFAULT FALSE,
    workflow_id UUID, -- Link to associated workflow
    supporting_documents JSONB, -- JSON array of document references
    approval_required BOOLEAN NOT NULL DEFAULT FALSE,
    approved_by UUID REFERENCES persons(id),
    approved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- =============================================================================
-- COMPLIANCE AND RISK MANAGEMENT
-- =============================================================================

-- KYC (Know Your Customer) results and documentation
CREATE TABLE kyc_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    check_type BYTEA NOT NULL, -- Blake3 hash for content-addressable check type identification
    status VARCHAR(20) NOT NULL CHECK (status IN ('Passed', 'Failed', 'Pending', 'RequiresManualReview')),
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0 AND risk_score <= 100),
    expiry_date DATE,
    performed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    performed_by UUID NOT NULL REFERENCES persons(id),
    details BYTEA, -- Blake3 hash of KYC details for tamper detection
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Sanctions screening results
CREATE TABLE sanctions_screening (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    screening_result VARCHAR(20) NOT NULL CHECK (screening_result IN ('Clear', 'PotentialMatch', 'ConfirmedMatch')),
    match_details JSONB, -- Structured data about potential matches
    screened_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    screened_by UUID NOT NULL REFERENCES persons(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Compliance alerts and monitoring
CREATE TABLE compliance_alerts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customers(id),
    account_id UUID REFERENCES accounts(id),
    transaction_id UUID REFERENCES transactions(id),
    alert_type alert_type NOT NULL,
    severity severity NOT NULL,
    description VARCHAR(500) NOT NULL,
    triggered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status alert_status NOT NULL DEFAULT 'New',
    assigned_to_person_id UUID REFERENCES persons(id),
    resolved_at TIMESTAMPTZ,
    resolved_by_person_id UUID REFERENCES persons(id),
    resolution_notes VARCHAR(500),
    metadata VARCHAR(1000),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Risk scoring for customers
CREATE TABLE customer_risk_scores (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    check_type VARCHAR(50) NOT NULL CHECK (check_type IN (
        'Kyc', 'Aml', 'Cdd', 'Edd', 'SanctionsScreening', 'PepScreening', 
        'AdverseMediaScreening', 'WatchlistScreening', 'UboVerification', 
        'DocumentVerification', 'AddressVerification', 'SourceOfFundsVerification', 
        'SourceOfWealthVerification', 'RiskAssessment', 'OngoingMonitoring', 'PeriodicReview'
    )),
    status VARCHAR(50) NOT NULL CHECK (status IN ('Passed', 'Failed', 'RequiresReview', 'Pending')),
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0 AND risk_score <= 100),
    
    -- Individual findings (normalized from Vec<String> - up to 300 chars each)
    findings_01 VARCHAR(300),
    findings_02 VARCHAR(300),
    findings_03 VARCHAR(300),
    findings_04 VARCHAR(300),
    findings_05 VARCHAR(300),
    findings_06 VARCHAR(300),
    findings_07 VARCHAR(300),
    
    -- Individual recommendations (normalized from Vec<String> - up to 300 chars each)
    recommendations_01 VARCHAR(300),
    recommendations_02 VARCHAR(300),
    recommendations_03 VARCHAR(300),
    recommendations_04 VARCHAR(300),
    recommendations_05 VARCHAR(300),
    recommendations_06 VARCHAR(300),
    recommendations_07 VARCHAR(300),
    
    checked_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Suspicious Activity Reports (SAR) - regulatory requirement
CREATE TABLE suspicious_activity_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for SAR reason  
    additional_details VARCHAR(500), -- Additional context for SAR
    
    -- Individual supporting transaction IDs (normalized from Vec<Uuid> - 19 fields as specified)
    supporting_transaction_id_01 UUID NOT NULL,
    supporting_transaction_id_02 UUID,
    supporting_transaction_id_03 UUID,
    supporting_transaction_id_04 UUID,
    supporting_transaction_id_05 UUID,
    supporting_transaction_id_06 UUID,
    supporting_transaction_id_07 UUID,
    supporting_transaction_id_08 UUID,
    supporting_transaction_id_09 UUID,
    supporting_transaction_id_10 UUID,
    supporting_transaction_id_11 UUID,
    supporting_transaction_id_12 UUID,
    supporting_transaction_id_13 UUID,
    supporting_transaction_id_14 UUID,
    supporting_transaction_id_15 UUID,
    supporting_transaction_id_16 UUID,
    supporting_transaction_id_17 UUID,
    supporting_transaction_id_18 UUID,
    supporting_transaction_id_19 UUID,
    
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    generated_by UUID NOT NULL REFERENCES persons(id),
    status VARCHAR(20) NOT NULL DEFAULT 'Draft' CHECK (status IN ('Draft', 'Filed', 'Acknowledged')),
    filed_at TIMESTAMP WITH TIME ZONE,
    acknowledgment_received_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Normalized KYC results with individual check and document fields
CREATE TABLE normalized_kyc_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    
    -- Individual completed checks (normalized from Vec<String>)
    completed_check_01 VARCHAR(100),
    completed_check_02 VARCHAR(100),
    completed_check_03 VARCHAR(100),
    completed_check_04 VARCHAR(100),
    completed_check_05 VARCHAR(100),
    completed_check_06 VARCHAR(100),
    completed_check_07 VARCHAR(100),
    
    -- Individual missing document IDs (normalized from Vec<Uuid>)
    missing_required_document_id_01 UUID,
    missing_required_document_id_02 UUID,
    missing_required_document_id_03 UUID,
    missing_required_document_id_04 UUID,
    missing_required_document_id_05 UUID,
    missing_required_document_id_06 UUID,
    missing_required_document_id_07 UUID,
    
    -- Common fields
    status VARCHAR(20) NOT NULL CHECK (status IN ('Passed', 'Failed', 'Pending', 'RequiresManualReview')),
    risk_score DECIMAL(5,2) CHECK (risk_score >= 0 AND risk_score <= 100),
    performed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    performed_by UUID NOT NULL REFERENCES persons(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Normalized screening results with individual sanctions matches
CREATE TABLE normalized_screening_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    
    -- Individual sanctions matches (normalized from Vec<String>)
    found_sanctions_match_01 VARCHAR(200),
    found_sanctions_match_02 VARCHAR(200),
    found_sanctions_match_03 VARCHAR(200),
    
    -- Common fields
    screening_result VARCHAR(20) NOT NULL CHECK (screening_result IN ('Clear', 'PotentialMatch', 'ConfirmedMatch')),
    screened_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    screened_by UUID NOT NULL REFERENCES persons(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Normalized monitoring results with individual alert IDs
CREATE TABLE normalized_monitoring_results (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID REFERENCES customers(id),
    account_id UUID REFERENCES accounts(id),
    
    -- Individual triggered alert IDs (normalized from Vec<Uuid>)
    triggered_compliance_alert_id_01 UUID,
    triggered_compliance_alert_id_02 UUID,
    triggered_compliance_alert_id_03 UUID,
    
    -- Common fields
    monitoring_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL CHECK (severity IN ('Low', 'Medium', 'High', 'Critical')),
    monitored_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    monitored_by UUID NOT NULL REFERENCES persons(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Normalized SAR data with individual supporting transaction IDs
CREATE TABLE normalized_sar_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    customer_id UUID NOT NULL REFERENCES customers(id),
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    
    -- Individual supporting transaction IDs (normalized from Vec<Uuid> - 19 fields as specified)
    supporting_transaction_id_01 UUID,
    supporting_transaction_id_02 UUID,
    supporting_transaction_id_03 UUID,
    supporting_transaction_id_04 UUID,
    supporting_transaction_id_05 UUID,
    supporting_transaction_id_06 UUID,
    supporting_transaction_id_07 UUID,
    supporting_transaction_id_08 UUID,
    supporting_transaction_id_09 UUID,
    supporting_transaction_id_10 UUID,
    supporting_transaction_id_11 UUID,
    supporting_transaction_id_12 UUID,
    supporting_transaction_id_13 UUID,
    supporting_transaction_id_14 UUID,
    supporting_transaction_id_15 UUID,
    supporting_transaction_id_16 UUID,
    supporting_transaction_id_17 UUID,
    supporting_transaction_id_18 UUID,
    supporting_transaction_id_19 UUID,
    
    -- Common fields
    additional_details VARCHAR(500),
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    generated_by UUID NOT NULL REFERENCES persons(id),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    transaction_id UUID REFERENCES transactions(id),
    fee_type VARCHAR(20) NOT NULL CHECK (fee_type IN ('EventBased', 'Periodic')),
    fee_category VARCHAR(30) NOT NULL CHECK (fee_category IN ('Transaction', 'Maintenance', 'Service', 'Penalty', 'Card', 'Loan', 'Regulatory')),
    product_id UUID NOT NULL,
    fee_code VARCHAR(12) NOT NULL,
    description VARCHAR(200) NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    calculation_method VARCHAR(20) NOT NULL CHECK (calculation_method IN ('Fixed', 'Percentage', 'Tiered', 'BalanceBased', 'RuleBased')),
    calculation_base_amount DECIMAL(15,2),
    fee_rate DECIMAL(8,6),
    trigger_event VARCHAR(50) NOT NULL CHECK (trigger_event IN ('AtmWithdrawal', 'PosTraction', 'WireTransfer', 'OnlineTransfer', 'CheckDeposit', 'InsufficientFunds', 'OverdraftUsage', 'BelowMinimumBalance', 'AccountOpening', 'AccountClosure', 'AccountMaintenance', 'StatementGeneration', 'MonthlyMaintenance', 'QuarterlyMaintenance', 'AnnualMaintenance', 'CardIssuance', 'CardReplacement', 'CardActivation', 'Manual', 'Regulatory')),
    status VARCHAR(20) NOT NULL DEFAULT 'Applied' CHECK (status IN ('Applied', 'Pending', 'Waived', 'Reversed', 'Failed')),
    applied_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    value_date DATE NOT NULL DEFAULT CURRENT_DATE,
    reversal_deadline TIMESTAMP WITH TIME ZONE,
    waived BOOLEAN NOT NULL DEFAULT FALSE,
    waived_by UUID REFERENCES persons(id),
    waived_reason_id UUID REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for waiver reason
    applied_by UUID NOT NULL REFERENCES persons(id),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fee_application_id UUID NOT NULL REFERENCES fee_applications(id),
    account_id UUID NOT NULL REFERENCES accounts(id),
    waived_amount DECIMAL(15,2) NOT NULL CHECK (waived_amount > 0),
    reason_id UUID NOT NULL REFERENCES reason_and_purpose(id), -- References reason_and_purpose(id) for waiver reason
    additional_details VARCHAR(200), -- Additional context for waiver
    waived_by UUID NOT NULL REFERENCES persons(id),
    waived_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    approval_required BOOLEAN NOT NULL DEFAULT FALSE,
    approved_by UUID REFERENCES persons(id),
    approved_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_waiver_approval CHECK (
        (approval_required = FALSE) OR 
        (approval_required = TRUE AND approved_by IS NOT NULL AND approved_at IS NOT NULL)
    )
);

-- =============================================================================
-- CASA (CURRENT & SAVINGS ACCOUNT) SPECIALIZED TABLES
-- =============================================================================

-- Overdraft facilities management
CREATE TABLE overdraft_facilities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    approved_limit DECIMAL(15,2) NOT NULL,
    current_utilized DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    available_limit DECIMAL(15,2) NOT NULL,
    interest_rate DECIMAL(8,6) NOT NULL, -- Annual debit interest rate
    facility_status VARCHAR(20) NOT NULL DEFAULT 'Active' CHECK (facility_status IN ('Active', 'Suspended', 'Expired', 'UnderReview', 'Cancelled')),
    approval_date DATE NOT NULL,
    expiry_date DATE,
    approved_by_person_id UUID NOT NULL REFERENCES persons(id),
    review_frequency VARCHAR(20) NOT NULL CHECK (review_frequency IN ('Monthly', 'Quarterly', 'SemiAnnually', 'Annually')),
    next_review_date DATE NOT NULL,
    security_required BOOLEAN NOT NULL DEFAULT FALSE,
    security_details VARCHAR(500),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
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
    posted_by_person_id UUID NOT NULL REFERENCES persons(id),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    current_limit DECIMAL(15,2) NOT NULL,
    requested_limit DECIMAL(15,2) NOT NULL,
    adjustment_reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    additional_details VARCHAR(200),
    required_document01_id UUID REFERENCES required_document(id),
    required_document02_id UUID REFERENCES required_document(id),
    required_document03_id UUID REFERENCES required_document(id),
    required_document04_id UUID REFERENCES required_document(id),
    required_document05_id UUID REFERENCES required_document(id),
    required_document06_id UUID REFERENCES required_document(id),
    required_document07_id UUID REFERENCES required_document(id),
    requested_by_person_id UUID NOT NULL REFERENCES persons(id),
    requested_at TIMESTAMP WITH TIME ZONE NOT NULL,
    approval_status VARCHAR(30) NOT NULL DEFAULT 'Pending' CHECK (approval_status IN ('Pending', 'Approved', 'Rejected', 'RequiresAdditionalDocuments', 'UnderReview')),
    approved_by_person_id UUID REFERENCES persons(id),
    approved_at TIMESTAMP WITH TIME ZONE,
    approval_notes VARCHAR(512),
    effective_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_limit_adjustment_valid CHECK (current_limit >= 0 AND requested_limit >= 0),
    CONSTRAINT ck_approval_consistency CHECK (
        (approval_status IN ('Approved', 'Rejected') AND approved_by_person_id IS NOT NULL AND approved_at IS NOT NULL) OR
        (approval_status NOT IN ('Approved', 'Rejected') AND approved_by_person_id IS NULL AND approved_at IS NULL)
    )
);

-- Overdraft interest calculations
CREATE TABLE overdraft_interest_calculations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    calculation_period_start DATE NOT NULL,
    calculation_period_end DATE NOT NULL,
    average_overdrawn_balance DECIMAL(15,2) NOT NULL,
    applicable_rate DECIMAL(8,6) NOT NULL,
    days_calculated INTEGER NOT NULL,
    interest_amount DECIMAL(15,2) NOT NULL,
    compounding_frequency VARCHAR(20) NOT NULL CHECK (compounding_frequency IN ('Daily', 'Weekly', 'Monthly', 'Quarterly')),
    capitalization_due BOOLEAN NOT NULL DEFAULT FALSE,
    calculated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    calculated_by_person_id UUID NOT NULL REFERENCES persons(id),
    
    CONSTRAINT ck_calculation_period_valid CHECK (calculation_period_start <= calculation_period_end),
    CONSTRAINT ck_calculation_days_valid CHECK (days_calculated > 0),
    CONSTRAINT ck_interest_calculation_valid CHECK (interest_amount >= 0)
);

-- Overdraft utilization tracking for interest calculation
CREATE TABLE overdraft_utilization (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    utilization_date DATE NOT NULL,
    opening_balance DECIMAL(15,2) NOT NULL, -- Negative for overdrawn
    closing_balance DECIMAL(15,2) NOT NULL, -- Negative for overdrawn
    average_daily_balance DECIMAL(15,2) NOT NULL,
    days_overdrawn INTEGER NOT NULL CHECK (days_overdrawn >= 0),
    interest_accrued DECIMAL(15,2) NOT NULL CHECK (interest_accrued >= 0),
    interest_rate DECIMAL(8,6) NOT NULL CHECK (interest_rate >= 0),
    capitalized BOOLEAN NOT NULL DEFAULT FALSE,
    capitalization_date DATE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_utilization_dates_valid CHECK (capitalization_date IS NULL OR capitalization_date >= utilization_date)
);

-- Daily overdraft processing jobs for EOD
CREATE TABLE overdraft_processing_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    processing_date DATE NOT NULL UNIQUE,
    accounts_processed INTEGER NOT NULL DEFAULT 0 CHECK (accounts_processed >= 0),
    total_interest_accrued DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    accounts_capitalized INTEGER NOT NULL DEFAULT 0 CHECK (accounts_capitalized >= 0),
    total_capitalized_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    status VARCHAR(20) NOT NULL DEFAULT 'Scheduled' CHECK (status IN ('Scheduled', 'Running', 'Completed', 'Failed', 'PartiallyCompleted')),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    errors_01 VARCHAR(200) DEFAULT '',
    errors_02 VARCHAR(200) DEFAULT '',
    errors_03 VARCHAR(200) DEFAULT '',
    errors_04 VARCHAR(200) DEFAULT '',
    errors_05 VARCHAR(200) DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_job_timing CHECK (started_at IS NULL OR completed_at IS NULL OR completed_at >= started_at)
);

-- CASA transaction validation contexts
CREATE TABLE casa_transaction_validations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    transaction_amount DECIMAL(15,2) NOT NULL,
    transaction_type VARCHAR(50) NOT NULL,
    current_balance DECIMAL(15,2) NOT NULL,
    available_balance DECIMAL(15,2) NOT NULL,
    overdraft_limit DECIMAL(15,2),
    post_transaction_balance DECIMAL(15,2) NOT NULL,
    overdraft_utilization DECIMAL(15,2),
    validation_result VARCHAR(30) NOT NULL CHECK (validation_result IN ('Approved', 'Rejected', 'RequiresAuthorization', 'RequiresHoldRelease')),
    validation_messages TEXT[], -- Array of validation messages
    requires_authorization BOOLEAN NOT NULL DEFAULT FALSE,
    authorization_level VARCHAR(20) CHECK (authorization_level IN ('Teller', 'Supervisor', 'Manager', 'CreditCommittee')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_validation_auth_consistency CHECK (
        (requires_authorization = TRUE AND authorization_level IS NOT NULL) OR
        (requires_authorization = FALSE AND authorization_level IS NULL)
    )
);

-- =============================================================================
-- CHANNEL MANAGEMENT TABLES
-- =============================================================================

-- Channels for transaction processing
CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    channel_code VARCHAR(50) NOT NULL UNIQUE,
    channel_name VARCHAR(100) NOT NULL,
    channel_type VARCHAR(30) NOT NULL CHECK (channel_type IN ('BranchTeller', 'ATM', 'InternetBanking', 'MobileApp', 'AgentTerminal', 'USSD', 'ApiGateway')),
    status channel_status NOT NULL DEFAULT 'Active',
    daily_limit DECIMAL(15,2),
    per_transaction_limit DECIMAL(15,2),
    supported_currency01 VARCHAR(3),
    supported_currency02 VARCHAR(3),
    supported_currency03 VARCHAR(3),
    requires_additional_auth BOOLEAN NOT NULL DEFAULT FALSE,
    fee_schedule_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_channel_limits CHECK (daily_limit IS NULL OR per_transaction_limit IS NULL OR daily_limit >= per_transaction_limit)
);

-- Fee schedules for channel-based fee management
CREATE TABLE fee_schedules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    schedule_name VARCHAR(100) NOT NULL,
    channel_id UUID REFERENCES channels(id),
    effective_date DATE NOT NULL,
    expiry_date DATE,
    currency VARCHAR(3) NOT NULL,
    fee01_fee_item_id UUID,
    fee02_fee_item_id UUID,
    fee03_fee_item_id UUID,
    fee04_fee_item_id UUID,
    fee05_fee_item_id UUID,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_fee_schedule_dates CHECK (expiry_date IS NULL OR expiry_date > effective_date),
    CONSTRAINT ck_currency_format CHECK (LENGTH(currency) = 3 AND currency = UPPER(currency))
);

-- Fee items within fee schedules
CREATE TABLE fee_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    schedule_id UUID NOT NULL REFERENCES fee_schedules(id) ON DELETE CASCADE,
    fee_code VARCHAR(12) NOT NULL,
    fee_name VARCHAR(100) NOT NULL,
    fee_type channel_fee_type NOT NULL,
    calculation_method channel_fee_calculation_method NOT NULL,
    fee_amount DECIMAL(15,2),
    fee_percentage DECIMAL(8,6),
    minimum_fee DECIMAL(15,2),
    maximum_fee DECIMAL(15,2),
    tier01_channel_fee_tier_id UUID,
    tier02_channel_fee_tier_id UUID,
    tier03_channel_fee_tier_id UUID,
    tier04_channel_fee_tier_id UUID,
    tier05_channel_fee_tier_id UUID,
    tier06_channel_fee_tier_id UUID,
    tier07_channel_fee_tier_id UUID,
    tier08_channel_fee_tier_id UUID,
    tier09_channel_fee_tier_id UUID,
    tier10_channel_fee_tier_id UUID,
    tier11_channel_fee_tier_id UUID,
    applies_to_transaction_type_01 VARCHAR(20),
    applies_to_transaction_type_02 VARCHAR(20),
    applies_to_transaction_type_03 VARCHAR(20),
    applies_to_transaction_type_04 VARCHAR(20),
    applies_to_transaction_type_05 VARCHAR(20),
    applies_to_transaction_type_06 VARCHAR(20),
    applies_to_transaction_type_07 VARCHAR(20),
    applies_to_transaction_type_08 VARCHAR(20),
    applies_to_transaction_type_09 VARCHAR(20),
    applies_to_transaction_type_10 VARCHAR(20),
    applies_to_transaction_type_11 VARCHAR(20),
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fee_item_id UUID NOT NULL REFERENCES fee_items(id) ON DELETE CASCADE,
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
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    channel_id UUID NOT NULL REFERENCES channels(id),
    reconciliation_date DATE NOT NULL,
    total_transactions BIGINT NOT NULL DEFAULT 0,
    total_amount DECIMAL(15,2) NOT NULL DEFAULT 0.00,
    status reconciliation_status NOT NULL DEFAULT 'InProgress',
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Reconciliation discrepancies
CREATE TABLE reconciliation_discrepancies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES channel_reconciliation_reports(id) ON DELETE CASCADE,
    transaction_id UUID NOT NULL REFERENCES transactions(id),
    description VARCHAR(500) NOT NULL,
    expected_amount DECIMAL(15,2) NOT NULL,
    actual_amount DECIMAL(15,2) NOT NULL,
    difference DECIMAL(15,2) NOT NULL,
    resolved BOOLEAN NOT NULL DEFAULT FALSE,
    resolution_notes VARCHAR(1000),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_discrepancy_difference CHECK (difference = expected_amount - actual_amount)
);

-- Reconciliation report discrepancies junction table
CREATE TABLE reconciliation_report_discrepancies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    reconciliation_report_id UUID NOT NULL REFERENCES channel_reconciliation_reports(id) ON DELETE CASCADE,
    discrepancy_id UUID NOT NULL REFERENCES reconciliation_discrepancies(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    UNIQUE(reconciliation_report_id, discrepancy_id)
);

-- Channel fees table
CREATE TABLE channel_fees (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    fee_type channel_fee_type NOT NULL,
    amount DECIMAL(15,2) NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    description VARCHAR(200) NOT NULL,
    applies_to_transaction_id UUID NOT NULL REFERENCES transactions(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add foreign key constraint for channels fee_schedule_id after fee_schedules table is created
ALTER TABLE channels ADD CONSTRAINT fk_channels_fee_schedule 
    FOREIGN KEY (fee_schedule_id) REFERENCES fee_schedules(id);

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
CREATE INDEX idx_channel_fees_type ON channel_fees(fee_type);
CREATE INDEX idx_channel_fees_transaction ON channel_fees(applies_to_transaction_id);

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

-- Amortization schedules for loan installment planning
CREATE TABLE amortization_schedules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
    original_principal DECIMAL(15,2) NOT NULL,
    interest_rate DECIMAL(8,6) NOT NULL,
    term_months INTEGER NOT NULL,
    installment_amount DECIMAL(15,2) NOT NULL,
    first_payment_date DATE NOT NULL,
    maturity_date DATE NOT NULL,
    total_interest DECIMAL(15,2) NOT NULL,
    total_payments DECIMAL(15,2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Individual installments in amortization schedule
CREATE TABLE amortization_entries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    schedule_id UUID NOT NULL REFERENCES amortization_schedules(id),
    installment_number INTEGER NOT NULL,
    due_date DATE NOT NULL,
    opening_principal_balance DECIMAL(15,2) NOT NULL,
    installment_amount DECIMAL(15,2) NOT NULL,
    principal_component DECIMAL(15,2) NOT NULL,
    interest_component DECIMAL(15,2) NOT NULL,
    closing_principal_balance DECIMAL(15,2) NOT NULL,
    cumulative_principal_paid DECIMAL(15,2) NOT NULL,
    cumulative_interest_paid DECIMAL(15,2) NOT NULL,
    payment_status VARCHAR(20) NOT NULL CHECK (payment_status IN ('Scheduled', 'Due', 'PartiallyPaid', 'Paid', 'Overdue', 'WriteOff')),
    paid_date DATE,
    paid_amount DECIMAL(15,2),
    days_overdue INTEGER,
    
    CONSTRAINT ck_installment_components CHECK (principal_component + interest_component = installment_amount),
    CONSTRAINT ck_payment_consistency CHECK (
        (payment_status IN ('Paid', 'PartiallyPaid') AND paid_date IS NOT NULL) OR
        (payment_status NOT IN ('Paid', 'PartiallyPaid') AND paid_date IS NULL)
    )
);

-- Loan delinquency tracking and management
CREATE TABLE loan_delinquencies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
    delinquency_start_date DATE NOT NULL,
    current_dpd INTEGER NOT NULL DEFAULT 0, -- Days Past Due
    highest_dpd INTEGER NOT NULL DEFAULT 0,
    delinquency_stage VARCHAR(20) NOT NULL CHECK (delinquency_stage IN ('Current', 'Stage1', 'Stage2', 'Stage3', 'NonPerforming', 'WriteOff')),
    overdue_principal DECIMAL(15,2) NOT NULL DEFAULT 0,
    overdue_interest DECIMAL(15,2) NOT NULL DEFAULT 0,
    penalty_interest_accrued DECIMAL(15,2) NOT NULL DEFAULT 0,
    total_overdue_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    last_payment_date DATE,
    last_payment_amount DECIMAL(15,2),
    restructuring_eligibility BOOLEAN NOT NULL DEFAULT TRUE,
    npl_date DATE, -- Non-Performing Loan classification
    provisioning_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_dpd_consistency CHECK (highest_dpd >= current_dpd),
    CONSTRAINT ck_overdue_total CHECK (total_overdue_amount = overdue_principal + overdue_interest + penalty_interest_accrued)
);

-- Loan delinquency processing jobs for EOD
CREATE TABLE loan_delinquency_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    processing_date DATE NOT NULL,
    loans_processed INTEGER NOT NULL DEFAULT 0,
    new_delinquent_loans INTEGER NOT NULL DEFAULT 0,
    recovered_loans INTEGER NOT NULL DEFAULT 0,
    npl_classifications INTEGER NOT NULL DEFAULT 0,
    total_penalty_interest DECIMAL(15,2) NOT NULL DEFAULT 0,
    collection_actions_triggered INTEGER NOT NULL DEFAULT 0,
    notifications_sent INTEGER NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL CHECK (status IN ('Scheduled', 'Running', 'Completed', 'Failed', 'PartiallyCompleted')),
    started_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    errors_01 VARCHAR(200) DEFAULT '',
    errors_02 VARCHAR(200) DEFAULT '',
    errors_03 VARCHAR(200) DEFAULT '',
    errors_04 VARCHAR(200) DEFAULT '',
    errors_05 VARCHAR(200) DEFAULT '',
    
    CONSTRAINT ck_job_timing CHECK (
        (status IN ('Completed', 'Failed') AND started_at IS NOT NULL AND completed_at IS NOT NULL) OR
        (status = 'Running' AND started_at IS NOT NULL AND completed_at IS NULL) OR
        (status = 'Scheduled' AND started_at IS NULL AND completed_at IS NULL)
    )
);

-- Loan payments
CREATE TABLE loan_payments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
    payment_date DATE NOT NULL,
    payment_amount DECIMAL(15,2) NOT NULL CHECK (payment_amount > 0),
    payment_type VARCHAR(20) NOT NULL CHECK (payment_type IN ('Regular', 'Prepayment', 'PartialPayment', 'Restructured', 'Settlement', 'Recovery')),
    payment_method VARCHAR(30) NOT NULL CHECK (payment_method IN ('BankTransfer', 'DirectDebit', 'Check', 'Cash', 'OnlinePayment', 'MobilePayment', 'StandingInstruction')),
    payment_status VARCHAR(20) NOT NULL CHECK (payment_status IN ('Processed', 'Pending', 'Failed', 'Reversed', 'PartiallyAllocated')),
    external_reference VARCHAR(100),
    processed_by UUID NOT NULL REFERENCES persons(id),
    processed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    reversal_info_id UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Payment allocations across loan components
CREATE TABLE payment_allocations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_id UUID NOT NULL REFERENCES loan_payments(id),
    penalty_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    overdue_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    current_interest_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    principal_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    fees_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    charges_payment DECIMAL(15,2) NOT NULL DEFAULT 0,
    excess_amount DECIMAL(15,2) NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Prepayment handling options and results
CREATE TABLE prepayment_handlings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    payment_allocation_id UUID NOT NULL REFERENCES payment_allocations(id),
    handling_type VARCHAR(30) NOT NULL CHECK (handling_type IN ('TermReduction', 'InstallmentReduction', 'HoldInSuspense', 'Refund')),
    excess_amount DECIMAL(15,2) NOT NULL,
    new_outstanding_principal DECIMAL(15,2) NOT NULL,
    term_reduction_months INTEGER,
    new_installment_amount DECIMAL(15,2),
    new_maturity_date DATE,
    schedule_regenerated BOOLEAN NOT NULL DEFAULT FALSE,
    customer_choice BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Collection actions for delinquent loans
CREATE TABLE collection_actions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    delinquency_id UUID NOT NULL REFERENCES loan_delinquencies(id),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
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
    assigned_to UUID NOT NULL REFERENCES persons(id),
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT ck_response_consistency CHECK (
        (response_received = FALSE AND response_date IS NULL) OR
        (response_received = TRUE AND response_date IS NOT NULL)
    )
);

-- Loan payments and allocations


-- Payment reversals
CREATE TABLE payment_reversals (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    original_payment_id UUID NOT NULL REFERENCES loan_payments(id),
    reversal_reason_id UUID NOT NULL REFERENCES reason_and_purpose(id),
    additional_details VARCHAR(200),
    reversed_amount DECIMAL(15,2) NOT NULL CHECK (reversed_amount > 0),
    reversed_by UUID NOT NULL REFERENCES persons(id),
    reversed_at TIMESTAMP WITH TIME ZONE NOT NULL,
    schedule_adjusted BOOLEAN NOT NULL DEFAULT FALSE
);

-- Loan restructuring records
CREATE TABLE loan_restructurings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
    restructuring_type restructuring_type NOT NULL,
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
    approved_by UUID REFERENCES persons(id),
    approved_at TIMESTAMP WITH TIME ZONE,
    conditions TEXT[], -- Array of conditions
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
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
                    INSERT INTO customer_audit_trail (id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason)
                    VALUES (uuid_generate_v4(), NEW.id, 'risk_rating', OLD.risk_rating::text, NEW.risk_rating::text, NOW(), NEW.updated_by_person_id, 'Risk rating change');
                END IF;
                
                IF OLD.status != NEW.status THEN
                    INSERT INTO customer_audit_trail (id, customer_id, field_name, old_value, new_value, changed_at, changed_by, reason)
                    VALUES (uuid_generate_v4(), NEW.id, 'status', OLD.status::text, NEW.status::text, NOW(), NEW.updated_by_person_id, 'Status change');
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

-- Insert system persons (organization field is now UUID, set to NULL for system accounts)
INSERT INTO persons (id, person_type, display_name, external_identifier, organization, is_active)
VALUES 
    ('00000000-0000-0000-0000-000000000000', 'system', 'SYSTEM', 'SYSTEM', NULL, TRUE),
    ('00000000-0000-0000-0000-000000000001', 'system', 'MIGRATION', 'MIGRATION', NULL, TRUE),
    ('00000000-0000-0000-0000-000000000002', 'integration', 'API_INTEGRATION', 'API', NULL, TRUE),
    ('00000000-0000-0000-0000-000000000003', 'system', 'BATCH_PROCESSOR', 'BATCH', NULL, TRUE);

-- =============================================================================
-- SEED DATA: REASON AND PURPOSE ENTRIES
-- =============================================================================

-- Insert initial seed data for reasons and purposes
INSERT INTO reason_and_purpose (
    id, code, category, context, 
    l1_content, l2_content, l3_content,
    l1_language_code, l2_language_code, l3_language_code,
    requires_details, is_active, severity, display_order,
    created_by_person_id, updated_by_person_id
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
INSERT INTO bank_holidays (id, jurisdiction, holiday_date, holiday_name, holiday_type, is_recurring, created_by_person_id) VALUES
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
INSERT INTO weekend_configuration (id, jurisdiction, weekend_days, effective_date, created_by_person_id, updated_by_person_id) VALUES
(uuid_generate_v4(), 'US', '[6,7]', '2024-01-01', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'), -- Saturday, Sunday
(uuid_generate_v4(), 'CM', '[6,7]', '2024-01-01', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'), -- Saturday, Sunday
(uuid_generate_v4(), 'AE', '[5,6]', '2024-01-01', '00000000-0000-0000-0000-000000000000', '00000000-0000-0000-0000-000000000000'); -- Friday, Saturday (UAE style)

-- =============================================================================
-- INDEXES FOR PERFORMANCE
-- =============================================================================

-- Customer table indexes
CREATE INDEX idx_customers_risk_rating ON customers(risk_rating);
CREATE INDEX idx_customers_status ON customers(status);
CREATE INDEX idx_customers_updated_by_person_id ON customers(updated_by_person_id);

-- Account table indexes
CREATE INDEX idx_accounts_customer_product ON accounts(product_id);
CREATE INDEX idx_accounts_status ON accounts(account_status);
CREATE INDEX idx_accounts_type ON accounts(account_type);
CREATE INDEX idx_accounts_branch ON accounts(domicile_agency_branch_id);
CREATE INDEX idx_accounts_balance ON accounts(current_balance);
CREATE INDEX idx_accounts_loan_maturity ON accounts(maturity_date) WHERE account_type = 'Loan';

-- Loan table indexes
CREATE INDEX idx_amortization_schedules_loan_account ON amortization_schedules(loan_account_id);
CREATE INDEX idx_amortization_schedules_maturity ON amortization_schedules(maturity_date);

CREATE INDEX idx_amortization_entries_schedule ON amortization_entries(schedule_id);
CREATE INDEX idx_amortization_entries_due_date ON amortization_entries(due_date);
CREATE INDEX idx_amortization_entries_status ON amortization_entries(payment_status);
CREATE INDEX idx_amortization_entries_overdue ON amortization_entries(days_overdue) WHERE days_overdue > 0;

CREATE INDEX idx_loan_delinquencies_account ON loan_delinquencies(loan_account_id);
CREATE INDEX idx_loan_delinquencies_stage ON loan_delinquencies(delinquency_stage);
CREATE INDEX idx_loan_delinquencies_dpd ON loan_delinquencies(current_dpd);
CREATE INDEX idx_loan_delinquencies_npl_date ON loan_delinquencies(npl_date) WHERE npl_date IS NOT NULL;

CREATE INDEX idx_collection_actions_delinquency ON collection_actions(delinquency_id);
CREATE INDEX idx_collection_actions_loan_account ON collection_actions(loan_account_id);
CREATE INDEX idx_collection_actions_type ON collection_actions(action_type);
CREATE INDEX idx_collection_actions_status ON collection_actions(action_status);
CREATE INDEX idx_collection_actions_assigned ON collection_actions(assigned_to);

CREATE INDEX idx_loan_payments_account ON loan_payments(loan_account_id);
CREATE INDEX idx_loan_payments_date ON loan_payments(payment_date);
CREATE INDEX idx_loan_payments_status ON loan_payments(payment_status);
CREATE INDEX idx_loan_payments_type ON loan_payments(payment_type);

CREATE INDEX idx_payment_allocations_payment ON payment_allocations(payment_id);

CREATE INDEX idx_payment_reversals_original_payment ON payment_reversals(original_payment_id);
CREATE INDEX idx_payment_reversals_reason ON payment_reversals(reversal_reason_id);

CREATE INDEX idx_loan_restructurings_account ON loan_restructurings(loan_account_id);
CREATE INDEX idx_loan_restructurings_type ON loan_restructurings(restructuring_type);
CREATE INDEX idx_loan_restructurings_status ON loan_restructurings(approval_status);
CREATE INDEX idx_loan_restructurings_request_date ON loan_restructurings(request_date);

CREATE INDEX idx_loan_delinquency_jobs_date ON loan_delinquency_jobs(processing_date);
CREATE INDEX idx_loan_delinquency_jobs_status ON loan_delinquency_jobs(status);

CREATE INDEX idx_prepayment_handlings_allocation ON prepayment_handlings(payment_allocation_id);
CREATE INDEX idx_prepayment_handlings_type ON prepayment_handlings(handling_type);

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
CREATE INDEX idx_agent_branches_network ON agent_branches(agent_network_id);
CREATE INDEX idx_agent_branches_address ON agent_branches(address_id);
CREATE INDEX idx_agent_branches_operating_hours ON agent_branches(operating_hours_id);
CREATE INDEX idx_agent_branches_capabilities ON agent_branches(branch_capabilities_id);
CREATE INDEX idx_agent_branches_security ON agent_branches(security_access_id);
CREATE INDEX idx_agent_branches_holiday_plan ON agent_branches(holiday_plan_id);
CREATE INDEX idx_agent_branches_temporary_closure ON agent_branches(temporary_closure_id) WHERE temporary_closure_id IS NOT NULL;
CREATE INDEX idx_agent_terminals_branch ON agent_terminals(agency_branch_id);
CREATE INDEX idx_operating_hours_name ON operating_hours(name_l1);
CREATE INDEX idx_branch_capabilities_name ON branch_capabilities(name_l1);
CREATE INDEX idx_security_access_name ON security_access(name_l1);
-- Remove branch messaging junction table indexes as table no longer exists

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
-- COLLATERAL TABLES
-- =============================================================================

-- Main collateral table
CREATE TABLE collateral (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_type collateral_type NOT NULL,
    collateral_category collateral_category NOT NULL,
    description VARCHAR(200) NOT NULL,
    external_reference VARCHAR(100) NOT NULL,
    
    -- Valuation information
    original_value DECIMAL(15,2) NOT NULL,
    current_market_value DECIMAL(15,2) NOT NULL,
    appraised_value DECIMAL(15,2),
    currency VARCHAR(3) NOT NULL,
    valuation_date DATE NOT NULL,
    next_valuation_date DATE,
    valuation_frequency_months INTEGER,
    
    -- Collateral details
    pledged_value DECIMAL(15,2) NOT NULL DEFAULT 0,
    available_value DECIMAL(15,2) NOT NULL DEFAULT 0,
    lien_amount DECIMAL(15,2),
    margin_percentage DECIMAL(5,2) NOT NULL DEFAULT 0,
    forced_sale_value DECIMAL(15,2),
    
    -- Location and custody
    custody_location custody_location NOT NULL,
    physical_location UUID REFERENCES addresses(id),
    custodian_details JSONB,
    
    -- Legal and documentation
    legal_title_holder UUID NOT NULL REFERENCES persons(id),
    perfection_status perfection_status NOT NULL,
    perfection_date DATE,
    perfection_expiry_date DATE,
    registration_number VARCHAR(100),
    registration_authority UUID REFERENCES persons(id),
    
    -- Insurance and risk
    insurance_required BOOLEAN NOT NULL DEFAULT false,
    insurance_coverage JSONB,
    risk_rating collateral_risk_rating NOT NULL,
    environmental_risk JSONB,
    
    -- Status and lifecycle
    status collateral_status NOT NULL DEFAULT 'Active',
    pledge_date DATE NOT NULL,
    release_date DATE,
    maturity_date DATE,
    
    -- Audit and tracking
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id),
    last_valuation_by UUID REFERENCES persons(id),
    next_review_date DATE
);

-- Collateral valuations table
CREATE TABLE collateral_valuations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_id UUID NOT NULL REFERENCES collateral(id),
    valuation_date DATE NOT NULL,
    valuation_method valuation_method NOT NULL,
    market_value DECIMAL(15,2) NOT NULL,
    forced_sale_value DECIMAL(15,2),
    appraiser_name VARCHAR(255) NOT NULL,
    appraiser_license VARCHAR(100),
    valuation_report_reference VARCHAR(100) NOT NULL,
    validity_period_months INTEGER NOT NULL,
    next_valuation_due DATE NOT NULL,
    valuation_notes VARCHAR(1000),
    
    -- Audit trail
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Collateral pledges table
CREATE TABLE collateral_pledges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_id UUID NOT NULL REFERENCES collateral(id),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
    pledged_amount DECIMAL(15,2) NOT NULL,
    pledge_percentage DECIMAL(5,2) NOT NULL,
    pledge_priority pledge_priority NOT NULL,
    pledge_date DATE NOT NULL,
    release_conditions VARCHAR(500),
    status pledge_status NOT NULL DEFAULT 'Active',
    
    -- Legal documentation
    security_agreement_reference VARCHAR(100) NOT NULL,
    pledge_registration_number VARCHAR(100),
    
    -- Monitoring
    last_review_date DATE,
    next_review_date DATE,
    covenant_compliance JSONB NOT NULL,
    
    -- Audit trail
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Collateral alerts table
CREATE TABLE collateral_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_id UUID NOT NULL REFERENCES collateral(id),
    alert_type collateral_alert_type NOT NULL,
    severity alert_severity NOT NULL,
    message VARCHAR(500) NOT NULL,
    trigger_date TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    due_date TIMESTAMPTZ,
    status collateral_alert_status NOT NULL DEFAULT 'Open',
    assigned_to UUID REFERENCES persons(id),
    resolution_notes VARCHAR(1000),
    resolved_at TIMESTAMPTZ,
    resolved_by UUID REFERENCES persons(id)
);

-- Collateral enforcement table
CREATE TABLE collateral_enforcement (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    collateral_id UUID NOT NULL REFERENCES collateral(id),
    loan_account_id UUID NOT NULL REFERENCES accounts(id),
    enforcement_type enforcement_type NOT NULL,
    enforcement_date DATE NOT NULL,
    outstanding_debt DECIMAL(15,2) NOT NULL,
    estimated_recovery DECIMAL(15,2) NOT NULL,
    enforcement_method enforcement_method NOT NULL,
    status enforcement_status NOT NULL DEFAULT 'Initiated',
    legal_counsel UUID REFERENCES persons(id),
    court_case_reference VARCHAR(100),
    expected_completion_date DATE,
    actual_completion_date DATE,
    recovery_amount DECIMAL(15,2),
    enforcement_costs DECIMAL(15,2),
    net_recovery DECIMAL(15,2),
    
    -- Audit trail
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by_person_id UUID NOT NULL REFERENCES persons(id),
    updated_by_person_id UUID NOT NULL REFERENCES persons(id)
);

-- Indexes for collateral tables
CREATE INDEX idx_collateral_type ON collateral(collateral_type);
CREATE INDEX idx_collateral_status ON collateral(status);
CREATE INDEX idx_collateral_pledge_date ON collateral(pledge_date);
CREATE INDEX idx_collateral_next_valuation ON collateral(next_valuation_date);
CREATE INDEX idx_collateral_legal_title_holder ON collateral(legal_title_holder);

CREATE INDEX idx_collateral_valuations_collateral_id ON collateral_valuations(collateral_id);
CREATE INDEX idx_collateral_valuations_date ON collateral_valuations(valuation_date);
CREATE INDEX idx_collateral_valuations_next_due ON collateral_valuations(next_valuation_due);

CREATE INDEX idx_collateral_pledges_collateral_id ON collateral_pledges(collateral_id);
CREATE INDEX idx_collateral_pledges_loan_account_id ON collateral_pledges(loan_account_id);
CREATE INDEX idx_collateral_pledges_status ON collateral_pledges(status);

CREATE INDEX idx_collateral_alerts_collateral_id ON collateral_alerts(collateral_id);
CREATE INDEX idx_collateral_alerts_status ON collateral_alerts(status);
CREATE INDEX idx_collateral_alerts_trigger_date ON collateral_alerts(trigger_date);

CREATE INDEX idx_collateral_enforcement_collateral_id ON collateral_enforcement(collateral_id);
CREATE INDEX idx_collateral_enforcement_loan_account_id ON collateral_enforcement(loan_account_id);
CREATE INDEX idx_collateral_enforcement_status ON collateral_enforcement(status);

-- Triggers for updated_at columns
CREATE TRIGGER update_collateral_updated_at
    BEFORE UPDATE ON collateral
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_collateral_pledges_updated_at
    BEFORE UPDATE ON collateral_pledges
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_collateral_enforcement_updated_at
    BEFORE UPDATE ON collateral_enforcement
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();


CREATE TABLE gl_mappings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id),
    customer_account_code VARCHAR(50) NOT NULL,
    interest_expense_code VARCHAR(50) NOT NULL,
    fee_income_code VARCHAR(50) NOT NULL,
    overdraft_code VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_gl_mappings_product FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE INDEX idx_gl_mappings_product_id ON gl_mappings(product_id);

CREATE TRIGGER update_gl_mappings_updated_at
    BEFORE UPDATE ON gl_mappings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE interest_rate_tiers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL REFERENCES products(id),
    minimum_balance DECIMAL NOT NULL,
    maximum_balance DECIMAL,
    interest_rate DECIMAL NOT NULL,
    tier_name VARCHAR(100) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_interest_rate_tiers_product FOREIGN KEY (product_id) REFERENCES products(id)
);

CREATE INDEX idx_interest_rate_tiers_product_id ON interest_rate_tiers(product_id);

CREATE TRIGGER update_interest_rate_tiers_updated_at
    BEFORE UPDATE ON interest_rate_tiers
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();


-- =============================================================================
-- COMMENTS FOR DOCUMENTATION
-- =============================================================================

-- Referenced persons
COMMENT ON TABLE persons IS 'Stores all person references used throughout the system for audit and tracking';
COMMENT ON COLUMN persons.id IS 'Unique identifier for this person reference';
COMMENT ON COLUMN persons.person_type IS 'Type of person: natural (human), legal (company), system, integration, unknown';
COMMENT ON COLUMN persons.display_name IS 'Display name of the person';
COMMENT ON COLUMN persons.external_identifier IS 'External ID like employee number, badge ID, system ID';
COMMENT ON COLUMN persons.organization IS 'Organization or department for employees or company name for legal entities';
-- Comment on email removed since field is now handled through messaging table
-- Comment on phone removed since field is now handled through messaging table
COMMENT ON COLUMN persons.department IS 'Department within organization';
COMMENT ON COLUMN persons.location IS 'Reference to address table for person location';
COMMENT ON COLUMN persons.duplicate_of IS 'Reference to another person record if this is a duplicate';
-- Comment on entity_reference removed since field moved to entity_reference table
-- Comment on entity_type removed since field moved to entity_reference table
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
-- Total tables created: 43 (added: holliday_plan, holiday_schedule, temporary_closure, required_document, compliance_cert, entity_reference, disbursement_instructions)
-- Total indexes created: 50+
-- Foreign key constraints: 60+
-- Check constraints: 50+
-- Triggers: 15+ (added new triggers for new tables)
-- Functions: 3
-- Initial seed data: System persons, reasons, holidays, weekend config
-- 
-- MAJOR SCHEMA CHANGES:
-- 1. Agent Network Domain: Multi-language support, flattened JSON fields, new holiday/closure tables
-- 2. Account Domain: Added disbursement_instructions table and last_disbursement_instruction_id field
-- 3. Person Domain: Individual messaging fields, entity_reference table, removed entity fields from persons
-- 4. All new enum types: service_type, certification_status, person_entity_type, disbursement_status

-- =============================================================================
-- CONSOLIDATED FROM 002_account_model_updates.sql
-- =============================================================================

-- Create AccountBalanceCalculation Table
CREATE TABLE "AccountBalanceCalculation" (
    "id" UUID NOT NULL,
    "account_id" UUID NOT NULL,
    "current_balance" DECIMAL NOT NULL,
    "available_balance" DECIMAL NOT NULL,
    "overdraft_limit" DECIMAL,
    "total_holds" DECIMAL NOT NULL,
    "active_hold_count" INTEGER NOT NULL,
    "calculation_timestamp" TIMESTAMPTZ NOT NULL,

    CONSTRAINT "AccountBalanceCalculation_pkey" PRIMARY KEY ("id")
);

-- Create AccountHoldSummary Table
CREATE TABLE "AccountHoldSummary" (
    "id" UUID NOT NULL,
    "account_balance_calculation_id" UUID NOT NULL,
    "hold_type" hold_type NOT NULL,
    "total_amount" DECIMAL NOT NULL,
    "hold_count" INTEGER NOT NULL,
    "priority" hold_priority NOT NULL,

    CONSTRAINT "AccountHoldSummary_pkey" PRIMARY KEY ("id")
);

-- Create AccountHoldReleaseRequest Table
CREATE TABLE "AccountHoldReleaseRequest" (
    "id" UUID NOT NULL,
    "hold_id" UUID NOT NULL,
    "release_amount" DECIMAL,
    "release_reason_id" UUID NOT NULL,
    "release_additional_details" VARCHAR(200),
    "released_by_person_id" UUID NOT NULL,
    "override_authorization" BOOLEAN NOT NULL,

    CONSTRAINT "AccountHoldReleaseRequest_pkey" PRIMARY KEY ("id")
);

-- Create AccountHoldExpiryJob Table
CREATE TABLE "AccountHoldExpiryJob" (
    "id" UUID NOT NULL,
    "processing_date" DATE NOT NULL,
    "expired_holds_count" INTEGER NOT NULL,
    "total_released_amount" DECIMAL NOT NULL,
    "processed_at" TIMESTAMPTZ NOT NULL,
    "errors" VARCHAR(100)[],

    CONSTRAINT "AccountHoldExpiryJob_pkey" PRIMARY KEY ("id")
);

-- Create PlaceHoldRequest Table
CREATE TABLE "PlaceHoldRequest" (
    "id" UUID NOT NULL,
    "account_id" UUID NOT NULL,
    "hold_type" hold_type NOT NULL,
    "amount" DECIMAL NOT NULL,
    "reason_id" UUID NOT NULL,
    "additional_details" VARCHAR(200),
    "placed_by_person_id" UUID NOT NULL,
    "expires_at" TIMESTAMPTZ,
    "priority" hold_priority NOT NULL,
    "source_reference" VARCHAR(100),

    CONSTRAINT "PlaceHoldRequest_pkey" PRIMARY KEY ("id")
);

-- Add foreign key constraints
ALTER TABLE "AccountHoldSummary" ADD CONSTRAINT "AccountHoldSummary_account_balance_calculation_id_fkey" FOREIGN KEY ("account_balance_calculation_id") REFERENCES "AccountBalanceCalculation"("id") ON DELETE RESTRICT ON UPDATE CASCADE;