//! Module contains error types

/// Boxed error
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct BoxedActorError(#[from] Box<dyn std::error::Error + Send + Sync>);

impl BoxedActorError {
    /// Creates new boxed error
    #[must_use]
    pub fn new<E: std::error::Error + 'static + Send + Sync>(e: E) -> Self {
        Self(Box::new(e))
    }

    /// Returns inner error
    ///
    /// # Errors
    ///
    /// Returns error if inner error is not of type `T`
    pub fn downcast_error<T: std::error::Error + 'static>(self) -> Result<Box<T>, String> {
        self.0
            .downcast::<T>()
            .map_err(|e| format!("Failed to downcast error: {e:?}"))
    }
}

/// Message processing error
#[derive(Debug, thiserror::Error)]
pub enum MessageProcessingError {
    /// Message handling panic
    #[error("Message processing panic {0:?}")]
    Panic(String),
}

/// Responder error
#[derive(Debug, thiserror::Error)]
pub enum ResponderError {
    /// Communication channel error
    #[error("ResponderChannelError {0:?}")]
    ChannelError(&'static str),
    /// Response was already used
    #[error("AlreadyRespond {0}")]
    AlreadyRespond(&'static str),
}
