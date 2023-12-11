//! Utility functions

pub mod atomic_timestamp;
mod graceful_shutdown;
pub mod notify_once;
pub mod waiter;
pub use graceful_shutdown::GracefulShutdown;
pub use graceful_shutdown::GracefulShutdownHandle;
