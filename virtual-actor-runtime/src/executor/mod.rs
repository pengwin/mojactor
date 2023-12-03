//! Local executor implementation

mod actor_registry;
mod error;
mod executor_preferences;
mod local_actor;
mod local_executor;
mod local_set_wrapper;
mod spawner;

pub use executor_preferences::ExecutorPreferences;
pub use executor_preferences::TokioRuntimePreferences;
pub use local_executor::LocalExecutor;
