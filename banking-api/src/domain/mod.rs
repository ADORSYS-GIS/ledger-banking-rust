pub mod customer;
pub mod account;
pub mod agent_network;
pub mod transaction;
pub mod calendar;
pub mod workflow;
pub mod compliance;
pub mod channel;
pub mod fee;
pub mod casa;
pub mod loan;
pub mod reason_view;
pub mod reason_and_purpose;
pub mod person;
pub mod collateral;
pub mod daily_collection;

pub use customer::*;
pub use account::*;
pub use agent_network::*;
pub use transaction::*;
pub use calendar::*;
pub use workflow::*;
pub use compliance::*;
// Channel module exports (renamed to avoid conflicts)
pub use channel::{
    Channel, ChannelStatus, ChannelFeeType, ChannelFeeCalculationMethod, 
    ChannelFeeTier, FeeSchedule, FeeItem, ReconciliationReport, 
    Discrepancy, ReconciliationStatus, ChannelFee
};

// Fee module exports (original fee types)
pub use fee::*;
pub use casa::*;
pub use loan::*;
pub use reason_view::*;
pub use reason_and_purpose::*;
pub use person::*;
pub use collateral::*;
pub use daily_collection::{
    CollectionAgent, Territory, CoverageArea, AgentPerformanceMetrics, DeviceInformation,
    CollectionProgram, CustomerCollectionProfile, CollectionRecord, CollectionBatch,
    AgentStatus as DailyCollectionAgentStatus, AreaType as DailyCollectionAreaType,
    CustomerDensity, TransportMode, DeviceType, ConnectivityStatus,
    CollectionProgramType, ProgramStatus as DailyCollectionProgramStatus,
    CollectionFrequency, CollectionStatus as DailyCollectionStatus,
    HolidayHandling, ReliabilityRating, CollectionMethod,
    CollectionRecordStatus, BiometricMethod, BatchStatus as DailyCollectionBatchStatus,
    AlertType as DailyCollectionAlertType, FeeFrequency as DailyCollectionFeeFrequency,
};