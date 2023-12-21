//! Local executor implementation

mod error;
mod executor_preferences;
mod handle;
mod local_actor;
mod local_executor;
mod local_set_wrapper;
mod spawner;

pub use error::LocalExecutorError;
pub use executor_preferences::ExecutorPreferences;
pub use executor_preferences::TokioRuntimePreferences;
pub use handle::Handle;
pub use local_actor::ActorTaskError;
pub use local_executor::LocalExecutor;
