use serde::{Deserialize, Serialize};
use heapless::String as HeaplessString;

/// Result of data validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationResult {
    pub is_valid: bool,
    pub validation_error_01: Option<ValidationError>,
    pub validation_error_02: Option<ValidationError>,
    pub validation_error_03: Option<ValidationError>,
    pub warning_01: HeaplessString<200>,
    pub warning_02: HeaplessString<200>,
    pub warning_03: HeaplessString<200>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: HeaplessString<50>,
    pub message: HeaplessString<200>,
    pub error_code: HeaplessString<50>,
}