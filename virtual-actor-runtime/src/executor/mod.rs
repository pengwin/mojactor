//! Local executor implementation

mod error;
mod executor_preferences;
mod handle;
mod actor;
mod local_executor;
mod local_set_wrapper;
mod spawner;

pub use error::LocalExecutorError;
pub use executor_preferences::ExecutorPreferences;
pub use executor_preferences::TokioRuntimePreferences;
pub use handle::Handle;
pub use actor::ActorTaskError;
pub use local_executor::LocalExecutor;
