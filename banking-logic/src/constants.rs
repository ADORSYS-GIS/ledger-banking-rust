use uuid::Uuid;

/// System person ID - used for automated system operations
pub const SYSTEM_PERSON_ID: Uuid = Uuid::nil();

/// Migration person ID - used for data migrations  
pub const MIGRATION_PERSON_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000001);

/// API Integration person ID - used for external API operations
pub const API_INTEGRATION_PERSON_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000002);

/// Batch Processor person ID - used for batch job operations
pub const BATCH_PROCESSOR_PERSON_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000003);

/// Compliance engine person ID - used for compliance automation
pub const COMPLIANCE_PERSON_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000004);

/// Lifecycle automation person ID - used for account lifecycle processes
pub const LIFECYCLE_AUTOMATION_PERSON_ID: Uuid = Uuid::from_u128(0x00000000_0000_0000_0000_000000000005);