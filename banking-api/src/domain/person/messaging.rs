use heapless::{String as HeaplessString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::person::common_enums::MessagingType;

/// # Service Trait
/// - FQN: banking-api/src/service/person/messaging_service.rs/MessagingService
/// # Documentation
/// - Messaging/communication identifier for a person
/// # Nature
/// - Immutable: 
///     - A messaging information can be fixed if eroneous, but does not change base on the holder. 
///     - A customer changing phone number receives an association with a new record.
///     - Many customers can share the same messaging object 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messaging {
    /// # Trait method
    /// - find_messaging_by_id
    /// # Nature
    /// - primary index
    pub id: Uuid,
    /// # Documentation
    /// - Type of messaging/communication method
    pub messaging_type: MessagingType,
    /// # Trait method
    /// - find_messaging_by_value
    /// # Documentation
    /// - The actual messaging identifier/location (email, phone, username, etc.)
    pub value: HeaplessString<100>,
    /// # Documentation
    /// - Description of the messaging type when MessagingType::Other is used
    pub other_type: Option<HeaplessString<20>>,
}