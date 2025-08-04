pub mod domain;
pub mod service;
pub mod error;
pub mod views;

// Re-export all domain types
pub use domain::*;

// Re-export service types but exclude the conflicting ValidationResult
pub use service::{
    AccountService, TransactionService, CustomerService, FeeService, ReasonAndPurposeService,
    CalendarService, ComplianceService, InterestService, CasaService, CollateralService, 
    HierarchyService, EodService, LoanService,
    // Export ValidationResult with a different name to avoid conflict
    ValidationResult as ServiceValidationResult,
};
pub use error::*;
pub use views::*;