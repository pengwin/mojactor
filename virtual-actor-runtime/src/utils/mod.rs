//! Utility functions

mod graceful_shutdown;
pub mod waiter;
pub use graceful_shutdown::GracefulShutdown;
pub use graceful_shutdown::GracefulShutdownHandle;
