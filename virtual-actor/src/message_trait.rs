//! Message trait

/// Marker trait for actor messages
pub trait Message: Send + 'static {
    /// Type of result returned by message handler
    type Result: Send + 'static;
}
