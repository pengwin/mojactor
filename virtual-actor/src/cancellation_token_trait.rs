use std::future::Future;

/// Cancellation token, used to signal actor about execution cancellation.
pub trait CancellationToken {
    /// Returns a `Future` that gets fulfilled when cancellation is requested.
    fn cancelled(&self) -> impl Future<Output = ()>;
}
