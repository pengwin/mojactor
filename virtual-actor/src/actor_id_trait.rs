//! Identifier of actor and blanket implementations

use std::{fmt::Display, sync::Arc};

use serde::{de::DeserializeOwned, Serialize};

/// Trait for identifier of actor
pub trait ActorId: Serialize + DeserializeOwned + Clone + Display + Send + Sync + 'static {
    /* Empty */
}

impl ActorId for usize {}
impl ActorId for u8 {}
impl ActorId for u16 {}
impl ActorId for u32 {}
impl ActorId for u64 {}
impl ActorId for String {}
impl ActorId for Arc<str> {}
impl ActorId for uuid::Uuid {}
