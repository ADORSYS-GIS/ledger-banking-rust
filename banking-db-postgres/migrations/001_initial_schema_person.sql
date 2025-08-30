-- Create ENUM types
CREATE TYPE location_type AS ENUM ('Residential', 'Business', 'Mailing', 'Temporary', 'Branch', 'Community', 'Other');
CREATE TYPE messaging_type AS ENUM ('Email', 'Phone', 'Sms', 'WhatsApp', 'Telegram', 'Skype', 'Teams', 'Signal', 'WeChat', 'Viber', 'Messenger', 'LinkedIn', 'Slack', 'Discord', 'Other');
CREATE TYPE person_type AS ENUM ('Natural', 'Legal', 'System', 'Integration', 'Unknown');
CREATE TYPE person_entity_type AS ENUM ('Customer', 'Employee', 'Shareholder', 'Director', 'BeneficialOwner', 'Agent', 'Vendor', 'Partner', 'RegulatoryContact', 'EmergencyContact', 'SystemAdmin', 'Other');

-- Main table for model CountryModel
CREATE TABLE country (
    id UUID PRIMARY KEY,
    iso2 VARCHAR(2) NOT NULL,
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100)
);

-- Index table for Country
CREATE TABLE country_idx (
    country_id UUID PRIMARY KEY,
    iso2 VARCHAR(2) NOT NULL
);

-- Main table for model CountrySubdivisionModel
CREATE TABLE country_subdivision (
    id UUID PRIMARY KEY,
    country_id UUID NOT NULL,
    code VARCHAR(10) NOT NULL,
    name_l1 VARCHAR(100) NOT NULL,
    name_l2 VARCHAR(100),
    name_l3 VARCHAR(100)
);

-- Index table for CountrySubdivision
CREATE TABLE country_subdivision_idx (
    country_subdivision_id UUID PRIMARY KEY,
    country_id UUID NOT NULL,
    code_hash BIGINT NOT NULL
);

-- Main table for model LocalityModel
CREATE TABLE locality (
    id UUID PRIMARY KEY,
    country_subdivision_id UUID NOT NULL,
    code VARCHAR(50) NOT NULL,
    name_l1 VARCHAR(50) NOT NULL,
    name_l2 VARCHAR(50),
    name_l3 VARCHAR(50)
);

-- Index table for Locality
CREATE TABLE locality_idx (
    locality_id UUID PRIMARY KEY,
    country_subdivision_id UUID NOT NULL,
    code_hash BIGINT NOT NULL
);

-- Main table for model LocationModel
CREATE TABLE location (
    id UUID PRIMARY KEY,
    street_line1 VARCHAR(50) NOT NULL,
    street_line2 VARCHAR(50),
    street_line3 VARCHAR(50),
    street_line4 VARCHAR(50),
    locality_id UUID NOT NULL,
    postal_code VARCHAR(20),
    latitude DECIMAL(15,10),
    longitude DECIMAL(15,10),
    accuracy_meters REAL,
    location_type location_type NOT NULL,
    version INTEGER NOT NULL,
    audit_log_id UUID NOT NULL
);

-- Index table for Location
CREATE TABLE location_idx (
    location_id UUID PRIMARY KEY,
    location_type location_type NOT NULL,
    locality_id UUID
);

-- Main table for model MessagingModel
CREATE TABLE messaging (
    id UUID PRIMARY KEY,
    messaging_type messaging_type NOT NULL,
    value VARCHAR(100) NOT NULL,
    other_type VARCHAR(20),
    version INTEGER NOT NULL,
    audit_log_id UUID NOT NULL
);

-- Index table for Messaging
CREATE TABLE messaging_idx (
    messaging_id UUID PRIMARY KEY,
    value_hash BIGINT NOT NULL
);

-- Main table for model EntityReferenceModel
CREATE TABLE entity_reference (
    id UUID PRIMARY KEY,
    person_id UUID NOT NULL,
    entity_role person_entity_type NOT NULL,
    reference_external_id VARCHAR(50),
    reference_details_l1 VARCHAR(50),
    reference_details_l2 VARCHAR(50),
    reference_details_l3 VARCHAR(50),
    version INTEGER NOT NULL,
    audit_log_id UUID NOT NULL
);

-- Index table for EntityReference
CREATE TABLE entity_reference_idx (
    entity_reference_id UUID PRIMARY KEY,
    person_id UUID NOT NULL,
    entity_role person_entity_type NOT NULL
);

-- Main table for model PersonModel
CREATE TABLE person (
    id UUID PRIMARY KEY,
    person_type person_type NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    external_identifier VARCHAR(50),
    organization_person_id UUID,
    messaging1_id UUID,
    messaging1_type messaging_type,
    messaging2_id UUID,
    messaging2_type messaging_type,
    messaging3_id UUID,
    messaging3_type messaging_type,
    messaging4_id UUID,
    messaging4_type messaging_type,
    messaging5_id UUID,
    messaging5_type messaging_type,
    department VARCHAR(50),
    location_id UUID,
    duplicate_of_person_id UUID,
    entity_reference_count INTEGER NOT NULL,
    audit_log_id UUID NOT NULL
);

-- Index table for Person
CREATE TABLE person_idx (
    person_id UUID PRIMARY KEY,
    external_identifier_hash BIGINT,
    version INTEGER NOT NULL,
    hash BIGINT NOT NULL
);

-- Main table for model PersonAuditModel
CREATE TABLE person_audit (
    person_id UUID NOT NULL,
    version INTEGER NOT NULL,
    hash BIGINT NOT NULL,
    person_type person_type NOT NULL,
    display_name VARCHAR(100) NOT NULL,
    external_identifier VARCHAR(50),
    organization_person_id UUID,
    messaging1_id UUID,
    messaging1_type messaging_type,
    messaging2_id UUID,
    messaging2_type messaging_type,
    messaging3_id UUID,
    messaging3_type messaging_type,
    messaging4_id UUID,
    messaging4_type messaging_type,
    messaging5_id UUID,
    messaging5_type messaging_type,
    department VARCHAR(50),
    location_id UUID,
    duplicate_of_person_id UUID,
    entity_reference_count INTEGER NOT NULL,
    audit_log_id UUID NOT NULL,
    PRIMARY KEY (person_id, version)
);

-- Main table for model AuditLogModel
CREATE TABLE audit_log (
    id UUID PRIMARY KEY,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_by_person_id UUID NOT NULL
);