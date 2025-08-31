-- Database cleanup script for test isolation
-- This script truncates all tables from the initial person schema to ensure a clean test state.

DO $cleanup$
BEGIN
    -- Disable foreign key checks temporarily to avoid constraint violations during truncation
    SET session_replication_role = replica;

    -- Truncate person-related tables
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'person_audit') THEN
        TRUNCATE TABLE person_audit CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'person_idx') THEN
        TRUNCATE TABLE person_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'person') THEN
        TRUNCATE TABLE person CASCADE;
    END IF;

    -- Truncate EntityReference-related tables
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'entity_reference_audit') THEN
        TRUNCATE TABLE entity_reference_audit CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'entity_reference_idx') THEN
        TRUNCATE TABLE entity_reference_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'entity_reference') THEN
        TRUNCATE TABLE entity_reference CASCADE;
    END IF;

    -- Truncate Messaging-related tables
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'messaging_audit') THEN
        TRUNCATE TABLE messaging_audit CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'messaging_idx') THEN
        TRUNCATE TABLE messaging_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'messaging') THEN
        TRUNCATE TABLE messaging CASCADE;
    END IF;

    -- Truncate Location-related tables
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'location_audit') THEN
        TRUNCATE TABLE location_audit CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'location_idx') THEN
        TRUNCATE TABLE location_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'location') THEN
        TRUNCATE TABLE location CASCADE;
    END IF;

    -- Truncate geographic hierarchy tables
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'locality_idx') THEN
        TRUNCATE TABLE locality_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'locality') THEN
        TRUNCATE TABLE locality CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'country_subdivision_idx') THEN
        TRUNCATE TABLE country_subdivision_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'country_subdivision') THEN
        TRUNCATE TABLE country_subdivision CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'country_idx') THEN
        TRUNCATE TABLE country_idx CASCADE;
    END IF;
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'country') THEN
        TRUNCATE TABLE country CASCADE;
    END IF;

    -- Truncate audit log table
    IF EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'audit_log') THEN
        TRUNCATE TABLE audit_log CASCADE;
    END IF;

    -- Re-enable foreign key checks
    SET session_replication_role = DEFAULT;
END
$cleanup$;

-- Note: Most tables use UUIDs, so there are no sequences to reset for this schema.
-- Note: VACUUM ANALYZE is not used here as it cannot run inside a transaction block,
-- and SQLx executes this script as a single transaction.