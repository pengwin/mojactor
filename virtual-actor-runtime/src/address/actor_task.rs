use tokio::task::JoinHandle;

use crate::executor::ActorTaskError;

pub type ActorTask = JoinHandle<Result<(), ActorTaskError>>;
