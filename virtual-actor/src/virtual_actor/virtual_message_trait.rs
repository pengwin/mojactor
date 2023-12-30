//! Virtual message trait

use serde::{Deserialize, Serialize};

use crate::message::{Message, MessageName};


/// A message can be sent to virtual actor
/// Message can be serialized and send through network or processed as is by local actor
pub trait VirtualMessage: Serialize + Deserialize<'static> + Send + 'static + Message
where
    Self::Result: Serialize + Deserialize<'static> + Send + 'static,
{
    /// Name of the message
    fn name() -> MessageName;
}
