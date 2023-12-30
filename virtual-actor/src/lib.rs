//! Virtual actors traits and types

pub mod errors;
pub mod utils;

pub mod actor;
pub mod local_actor;
pub mod message;
pub mod virtual_actor;

/// Reexport of uuid
pub use uuid::Uuid;
