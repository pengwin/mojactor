//! Virtual message trait

use super::names::MessageName;
use serde::{Deserialize, Serialize};

use super::message_trait::Message;

/// A message can be sent to virtual actor
/// Message can be serialized and send through network or processed as is by local actor
pub trait VirtualMessage: Serialize + Deserialize<'static> + Send + 'static + Message
where
    Self::Result: Serialize + Deserialize<'static> + Send + 'static,
{
    /// Name of the message
    fn name() -> MessageName;
}
