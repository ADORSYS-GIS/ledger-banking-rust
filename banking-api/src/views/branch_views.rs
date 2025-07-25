use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::domain::agent_network::{AgencyBranch, BranchType, ServiceType};

/// View model for API responses that include branch details with transaction
#[derive(Serialize, Deserialize)]
pub struct TransactionWithBranchView {
    pub transaction_id: Uuid,
    pub amount: Decimal,
    pub cash_pickup_branch: Option<BranchSummary>,
}

/// Summary view of a branch for API responses
#[derive(Serialize, Deserialize)]
pub struct BranchSummary {
    pub branch_id: Uuid,
    pub branch_name: String,
    pub branch_code: String,
    pub branch_type: BranchType,
    pub address: String,
    pub is_open_now: bool,
    pub services_available: Vec<ServiceType>,
    pub wait_time_minutes: Option<u16>,
    pub gps_coordinates: Option<GpsCoordinatesSummary>,
    pub contact_info: ContactInfo,
}

/// Simplified GPS coordinates for API responses
#[derive(Serialize, Deserialize)]
pub struct GpsCoordinatesSummary {
    pub latitude: f64,
    pub longitude: f64,
}

/// Contact information summary
#[derive(Serialize, Deserialize)]
pub struct ContactInfo {
    pub primary_phone: String,
    pub secondary_phone: Option<String>,
    pub email: Option<String>,
}

impl BranchSummary {
    /// Create a BranchSummary from an AgencyBranch domain object
    pub fn from_branch(branch: &AgencyBranch, current_time: DateTime<Utc>) -> Self {
        Self {
            branch_id: branch.branch_id,
            branch_name: branch.branch_name.to_string(),
            branch_code: branch.branch_code.to_string(),
            branch_type: branch.branch_type,
            address: branch.get_formatted_address(),
            is_open_now: branch.is_open_now(current_time),
            services_available: branch.supported_services.to_vec(),
            wait_time_minutes: branch.average_wait_time_minutes,
            gps_coordinates: branch.gps_coordinates.map(|coords| GpsCoordinatesSummary {
                latitude: coords.latitude,
                longitude: coords.longitude,
            }),
            contact_info: ContactInfo {
                primary_phone: branch.primary_phone.to_string(),
                secondary_phone: branch.secondary_phone.as_ref().map(|s| s.to_string()),
                email: branch.email.as_ref().map(|s| s.to_string()),
            },
        }
    }
}

/// Extended branch details for management interfaces
#[derive(Serialize, Deserialize)]
pub struct BranchDetailView {
    pub branch_id: Uuid,
    pub network_id: Uuid,
    pub parent_branch_id: Option<Uuid>,
    pub branch_name: String,
    pub branch_code: String,
    pub branch_type: BranchType,
    pub status: String,
    pub address: AddressView,
    pub operating_hours: OperatingHoursView,
    pub services_available: Vec<ServiceType>,
    pub supported_currencies: Vec<String>,
    pub transaction_limits: TransactionLimitsView,
    pub cash_management: CashManagementView,
    pub contact_info: ContactInfo,
    pub security_features: SecurityFeaturesView,
    pub accessibility_features: AccessibilityFeaturesView,
    pub risk_rating: String,
    pub is_cash_pickup_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub last_updated_at: DateTime<Utc>,
}

/// Address view for API responses
#[derive(Serialize, Deserialize)]
pub struct AddressView {
    pub street_line1: String,
    pub street_line2: Option<String>,
    pub city: String,
    pub state_province: String,
    pub postal_code: String,
    pub country_code: String,
    pub gps_coordinates: Option<GpsCoordinatesSummary>,
    pub landmark_description: Option<String>,
}

/// Operating hours view
#[derive(Serialize, Deserialize)]
pub struct OperatingHoursView {
    pub timezone: String,
    pub schedule: Vec<DaySchedule>,
    pub is_open_now: bool,
}

/// Day schedule for operating hours
#[derive(Serialize, Deserialize)]
pub struct DaySchedule {
    pub day: String,
    pub open_time: Option<String>,
    pub close_time: Option<String>,
    pub break_start: Option<String>,
    pub break_end: Option<String>,
}

/// Transaction limits summary
#[derive(Serialize, Deserialize)]
pub struct TransactionLimitsView {
    pub daily_transaction_limit: Decimal,
    pub per_transaction_limit: Decimal,
    pub monthly_transaction_limit: Option<Decimal>,
    pub current_daily_volume: Decimal,
}

/// Cash management summary
#[derive(Serialize, Deserialize)]
pub struct CashManagementView {
    pub max_cash_limit: Decimal,
    pub current_cash_balance: Decimal,
    pub minimum_cash_balance: Decimal,
    pub cash_utilization_percentage: f64,
}

/// Security features summary
#[derive(Serialize, Deserialize)]
pub struct SecurityFeaturesView {
    pub has_security_guard: bool,
    pub has_cctv: bool,
    pub has_panic_button: bool,
    pub has_safe: bool,
    pub has_biometric_verification: bool,
    pub police_station_distance_km: Option<f32>,
}

/// Accessibility features summary
#[derive(Serialize, Deserialize)]
pub struct AccessibilityFeaturesView {
    pub wheelchair_accessible: bool,
    pub has_ramp: bool,
    pub has_braille_signage: bool,
    pub has_audio_assistance: bool,
    pub has_sign_language_support: bool,
    pub parking_available: bool,
    pub public_transport_nearby: bool,
}

impl BranchDetailView {
    /// Create a detailed branch view from an AgencyBranch domain object
    pub fn from_branch(branch: &AgencyBranch, current_time: DateTime<Utc>) -> Self {
        let cash_utilization = if branch.max_cash_limit > Decimal::ZERO {
            (branch.current_cash_balance / branch.max_cash_limit * Decimal::from(100))
                .to_f64()
                .unwrap_or(0.0)
        } else {
            0.0
        };

        Self {
            branch_id: branch.branch_id,
            network_id: branch.network_id,
            parent_branch_id: branch.parent_branch_id,
            branch_name: branch.branch_name.to_string(),
            branch_code: branch.branch_code.to_string(),
            branch_type: branch.branch_type,
            status: format!("{:?}", branch.status),
            address: AddressView {
                street_line1: branch.address.street_line1.to_string(),
                street_line2: branch.address.street_line2.as_ref().map(|s| s.to_string()),
                city: branch.address.city.to_string(),
                state_province: branch.address.state_province.to_string(),
                postal_code: branch.address.postal_code.to_string(),
                country_code: std::str::from_utf8(&branch.address.country_code)
                    .unwrap_or("??")
                    .to_string(),
                gps_coordinates: branch.gps_coordinates.map(|coords| GpsCoordinatesSummary {
                    latitude: coords.latitude,
                    longitude: coords.longitude,
                }),
                landmark_description: branch.landmark_description.as_ref().map(|s| s.to_string()),
            },
            operating_hours: OperatingHoursView {
                timezone: branch.operating_hours.timezone.to_string(),
                schedule: vec![], // TODO: Implement day schedule conversion
                is_open_now: branch.is_open_now(current_time),
            },
            services_available: branch.supported_services.to_vec(),
            supported_currencies: branch
                .supported_currencies
                .iter()
                .map(|c| std::str::from_utf8(c).unwrap_or("???").to_string())
                .collect(),
            transaction_limits: TransactionLimitsView {
                daily_transaction_limit: branch.daily_transaction_limit,
                per_transaction_limit: branch.per_transaction_limit,
                monthly_transaction_limit: branch.monthly_transaction_limit,
                current_daily_volume: branch.current_daily_volume,
            },
            cash_management: CashManagementView {
                max_cash_limit: branch.max_cash_limit,
                current_cash_balance: branch.current_cash_balance,
                minimum_cash_balance: branch.minimum_cash_balance,
                cash_utilization_percentage: cash_utilization,
            },
            contact_info: ContactInfo {
                primary_phone: branch.primary_phone.to_string(),
                secondary_phone: branch.secondary_phone.as_ref().map(|s| s.to_string()),
                email: branch.email.as_ref().map(|s| s.to_string()),
            },
            security_features: SecurityFeaturesView {
                has_security_guard: branch.security_features.has_security_guard,
                has_cctv: branch.security_features.has_cctv,
                has_panic_button: branch.security_features.has_panic_button,
                has_safe: branch.security_features.has_safe,
                has_biometric_verification: branch.security_features.has_biometric_verification,
                police_station_distance_km: branch.security_features.police_station_distance_km,
            },
            accessibility_features: AccessibilityFeaturesView {
                wheelchair_accessible: branch.accessibility_features.wheelchair_accessible,
                has_ramp: branch.accessibility_features.has_ramp,
                has_braille_signage: branch.accessibility_features.has_braille_signage,
                has_audio_assistance: branch.accessibility_features.has_audio_assistance,
                has_sign_language_support: branch.accessibility_features.has_sign_language_support,
                parking_available: branch.accessibility_features.parking_available,
                public_transport_nearby: branch.accessibility_features.public_transport_nearby,
            },
            risk_rating: format!("{:?}", branch.risk_rating),
            is_cash_pickup_enabled: branch.is_cash_pickup_enabled(),
            created_at: branch.created_at,
            last_updated_at: branch.last_updated_at,
        }
    }
}