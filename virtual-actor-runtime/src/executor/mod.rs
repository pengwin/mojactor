//! Local executor implementation

mod actor;
pub mod errors;
mod executor_preferences;
mod handle;
mod local_executor;
mod local_set_wrapper;
mod spawner;

pub use executor_preferences::ExecutorPreferences;
pub use executor_preferences::TokioRuntimePreferences;
pub use handle::Handle;
pub use local_executor::LocalExecutor;
