use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountOwnership {
    pub ownership_id: Uuid,
    pub account_id: Uuid,
    pub customer_id: Uuid,
    pub ownership_type: OwnershipType,
    pub ownership_percentage: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRelationship {
    pub relationship_id: Uuid,
    pub account_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: EntityType,
    pub relationship_type: RelationshipType,
    pub status: RelationshipStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AccountMandate {
    pub mandate_id: Uuid,
    pub account_id: Uuid,
    pub grantee_customer_id: Uuid,
    pub permission_type: PermissionType,
    pub transaction_limit: Option<Decimal>,
    pub approval_group_id: Option<Uuid>,
    pub status: MandateStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UltimateBeneficiary {
    pub ubo_link_id: Uuid,
    pub corporate_customer_id: Uuid,
    pub beneficiary_customer_id: Uuid,
    pub ownership_percentage: Option<Decimal>,
    pub control_type: ControlType,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub status: UboStatus,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OwnershipType {
    Single,
    Joint,
    Corporate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Branch,
    Agent,
    RiskManager,
    ComplianceOfficer,
    CustomerService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    PrimaryHandler,
    BackupHandler,
    RiskOversight,
    ComplianceOversight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermissionType {
    ViewOnly,
    LimitedWithdrawal,
    JointApproval,
    FullAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MandateStatus {
    Active,
    Suspended,
    Revoked,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType { 
    DirectOwnership, 
    IndirectOwnership, 
    SignificantInfluence, 
    SeniorManagement 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationStatus { 
    Pending, 
    Verified, 
    Rejected, 
    RequiresUpdate 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UboStatus {
    Active,
    Inactive,
    UnderReview,
}