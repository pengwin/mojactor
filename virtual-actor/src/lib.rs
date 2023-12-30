//! Virtual actors traits and types

pub mod errors;
pub mod utils;

mod actor;
mod local_actor;
mod message;
mod virtual_actor;

/// Reexport of uuid
pub use uuid::Uuid;

// Export actor traits
pub use actor::*;

// Export message traits
pub use message::*;

// Export local actor traits
pub use local_actor::*;

// Export virtual actor traits
pub use virtual_actor::*;
