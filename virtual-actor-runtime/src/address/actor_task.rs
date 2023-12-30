use tokio::task::JoinHandle;

use crate::executor::errors::ActorTaskError;

pub type ActorTask = JoinHandle<Result<(), ActorTaskError>>;
