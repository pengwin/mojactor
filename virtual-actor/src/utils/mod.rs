//! This module contains utility functions

mod cancellation_token_trait;
mod unwind_panic;

pub use cancellation_token_trait::CancellationToken;
pub use unwind_panic::unwind_panic;
