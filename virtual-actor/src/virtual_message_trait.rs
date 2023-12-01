//! Virtual message trait

use super::names::MessageName;
use serde::{de::DeserializeOwned, Serialize};

use super::message_trait::Message;

/// A message can be sent to virtual actor
/// Mesage can be serialized and send through network or processed as is by local actor
pub trait VirtualMessage: Serialize + DeserializeOwned + Send + 'static + Message
where
    Self::Result: Serialize + DeserializeOwned + Send + 'static,
{
    /// Name of the message
    fn name() -> MessageName;
}
