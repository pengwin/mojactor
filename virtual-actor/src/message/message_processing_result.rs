use crate::{errors::MessageProcessingError, Message};

/// Message processing result
pub type MessageProcessingResult<M> = Result<<M as Message>::Result, MessageProcessingError>;
