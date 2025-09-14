use serde::{Deserialize, Serialize};

/// Type of messaging/communication method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessagingType {
    /// Email location
    Email,
    /// Phone number (mobile or landline)
    Phone,
    /// SMS/Text messaging
    Sms,
    /// WhatsApp messaging
    WhatsApp,
    /// Telegram messaging
    Telegram,
    /// Skype
    Skype,
    /// Microsoft Teams
    Teams,
    /// Signal messaging
    Signal,
    /// WeChat
    WeChat,
    /// Viber
    Viber,
    /// Facebook Messenger
    Messenger,
    /// LinkedIn messaging
    LinkedIn,
    /// Slack
    Slack,
    /// Discord
    Discord,
    /// Other messaging type (specify in other_type field)
    Other,
}

/// Type of person being referenced in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersonType {
    /// Natural person (human individual)
    Natural,
    /// Legal entity (corporation, institution)
    Legal,
    /// System or automated process
    System,
    /// External integration or API
    Integration,
    /// Unknown or unspecified
    Unknown,
}

/// Type of entity that the person is referenced as
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipRole {
    /// Customer reference
    Customer,
    /// Employee reference
    Employee,
    /// Shareholder reference
    Shareholder,
    /// Director or board member
    Director,
    /// Beneficial owner
    BeneficialOwner,
    /// Agent or representative
    Agent,
    /// Vendor or supplier
    Vendor,
    /// Partner organization
    Partner,
    /// Regulatory contact
    RegulatoryContact,
    /// Emergency contact
    EmergencyContact,
    /// System administrator
    SystemAdmin,
    /// Other entity type
    Other,
}