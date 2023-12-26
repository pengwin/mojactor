use std::sync::{Arc, Mutex};

use super::ActorTask;

#[derive(thiserror::Error, Debug)]
#[error("Actor task container error: {message}")]
pub struct ActorTaskContainerError {
    pub message: String,
}

pub struct ActorTaskContainer {
    inner: Arc<Mutex<Option<ActorTask>>>,
}

impl Default for ActorTaskContainer {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }
}

impl ActorTaskContainer {
    pub fn set(&self, task: ActorTask) -> Result<(), ActorTaskContainerError> {
        let mut value = self.inner.lock().map_err(|e| ActorTaskContainerError {
            message: format!("Actor task lock error: {e:?}"),
        })?;

        if value.is_some() {
            return Err(ActorTaskContainerError {
                message: "Actor task already set".to_string(),
            });
        }

        *value = Some(task);
        Ok(())
    }

    pub fn take(&self) -> Result<Option<ActorTask>, ActorTaskContainerError> {
        let mut task_guard = self.inner.lock().map_err(|e| ActorTaskContainerError {
            message: format!("Actor task lock error: {e:?}"),
        })?;

        let task = task_guard.take();

        Ok(task)
    }
}
