use crate::errors::MessageProcessingError;

use super::Message;

/// Message processing result
pub type MessageProcessingResult<M> = Result<<M as Message>::Result, MessageProcessingError>;
