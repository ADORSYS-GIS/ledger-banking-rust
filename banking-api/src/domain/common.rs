use serde::{Deserialize, Serialize};

/// Result of data validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationResult {
    pub is_valid: bool,
    pub validation_errors: Vec<ValidationError>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub error_code: String,
}