pub mod country;
pub mod country_subdivision;
pub mod entity_reference;
pub mod locality;
pub mod location;
#[allow(clippy::module_inception)]
pub mod person;

pub use self::country::*;
pub use self::country_subdivision::*;
pub use self::entity_reference::*;
pub use self::locality::*;
pub use self::location::*;
pub use self::person::*;