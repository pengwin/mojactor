//! Local executor for actor

use std::sync::Arc;
use std::thread::JoinHandle;

use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tokio::sync::Notify;

use crate::utils::waiter::waiter;
use crate::utils::waiter::WaitError;
use crate::utils::GracefulShutdown;
use crate::utils::GracefulShutdownHandle;
use tokio_util::sync::CancellationToken;

use super::errors::LocalExecutorError;
use super::executor_preferences::TokioRuntimePreferences;
use super::handle::Handle;
use super::local_set_wrapper::LocalSetWrapper;
use super::spawner::LocalSpawner;
use super::ExecutorPreferences;

/// Std thread handle
type ThreadHandle = std::thread::JoinHandle<()>;

/// Executor based on `tokio::task::LocalSet`
pub struct LocalExecutor {
    /// Name
    name: String,
    /// Thread handle
    thread_handle: ThreadHandle,
    self_handle: Handle,
    /// Graceful shutdown handle for spawner
    spawner_gs: GracefulShutdownHandle,
    /// Graceful shutdown handle for local set
    local_set_gs: GracefulShutdownHandle,
    /// Thread stopped notify
    thread_stopped_notify: Arc<Notify>,
}

impl GracefulShutdown for LocalExecutor {
    async fn graceful_shutdown(self, timeout: std::time::Duration) -> Result<(), WaitError> {
        // cancel message receiving
        self.self_handle.mailbox_cancellation().cancel();
        // wait for spawn task to finish processing all pending tasks
        // to ensure that no more actors will be spawned
        match self.spawner_gs.wait(timeout).await {
            Err(WaitError::Timeout(w)) => {
                eprintln!("{w} wait timeout on {}", self.name);
                self.spawner_gs.shutdown();
                // wait for spawn task to finish
                // to ensure that no actors will be spawned
                self.spawner_gs.wait(timeout).await
            }
            Err(e) => Err(e),
            _ => Ok(()),
        }?;

        // wait for local set to finish
        // all actors should be stopped
        match self.local_set_gs.wait(timeout).await {
            Ok(()) => Ok(()),
            Err(WaitError::Timeout(w)) => {
                eprintln!("{w} wait timeout on {}", self.name);
                // if local set is not finished, then shutdown actor execution
                self.self_handle.executor_cancellation().cancel();
                Ok(())
            }
            Err(e) => Err(e),
        }?;
        // wait for thread to finish
        self.wait_thread(timeout).await?;

        Ok(())
    }
}

impl LocalExecutor {
    /// Returns clonable handle for this executor to spawn actors
    #[must_use]
    pub(crate) fn handle(&self) -> &Handle {
        &self.self_handle
    }

    /// Starts executor thread
    /// with preferences
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub fn new(preferences: &ExecutorPreferences) -> Result<Self, LocalExecutorError> {
        let name = preferences.thread_name.clone();
        let executor_cancellation = CancellationToken::new();
        let mailbox_cancellation = CancellationToken::new();

        let thread_stopped_notify = Arc::new(Notify::new());

        let spawner = LocalSpawner::new(
            &preferences.mailbox_preferences,
            &mailbox_cancellation.child_token(),
            &executor_cancellation.child_token(),
        );
        let spawner_dispatcher = spawner.dispatcher().clone();
        let spawner_gs = spawner.graceful_shutdown_handle();

        let local_set_cancellation = CancellationToken::new();
        let local_set_stopped = Arc::new(Notify::new());
        let local_set_gs = GracefulShutdownHandle::new(
            "LocalSet",
            local_set_stopped.clone(),
            local_set_cancellation.clone(),
        );
        let thread_handle = Self::start_thread(
            preferences,
            spawner,
            local_set_stopped.clone(),
            thread_stopped_notify.clone(),
            local_set_cancellation.clone(),
        )?;

        let self_handle = Handle::new(
            spawner_dispatcher,
            executor_cancellation,
            mailbox_cancellation,
        );

        Ok(Self {
            name,
            thread_handle,
            self_handle,
            spawner_gs,
            local_set_gs,
            thread_stopped_notify,
        })
    }

    /// Waits for finish of executor thread
    async fn wait_thread(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        let res = waiter(
            stringify!(wait_thread),
            &self.thread_stopped_notify,
            timeout,
            None,
        )
        .await;
        match res {
            Ok(()) => Ok(()),
            Err(WaitError::Timeout(s)) => {
                eprintln!("Thread waiting timeout");
                if !self.thread_handle.is_finished() {
                    eprintln!("Thread is not finished");
                }
                Err(WaitError::Timeout(s))
            }
            Err(e) => Err(e),
        }
    }

    fn start_thread(
        executor_preferences: &ExecutorPreferences,
        spawner: LocalSpawner,
        local_set_stopped: Arc<Notify>,
        thread_stopped: Arc<Notify>,
        local_set_cancellation: CancellationToken,
    ) -> Result<JoinHandle<()>, LocalExecutorError> {
        let rt = Self::build_runtime(&executor_preferences.tokio_runtime_preferences)?;

        let mut thread_builder =
            std::thread::Builder::new().name(executor_preferences.thread_name.clone());

        if let Some(stack_size) = executor_preferences.thread_stack_size {
            thread_builder = thread_builder.stack_size(stack_size);
        }

        let handle = thread_builder
            .spawn(move || {
                let local = LocalSetWrapper::new();
                local.spawn_local(spawner.run());
                local.run(&rt, &local_set_stopped, &local_set_cancellation);

                thread_stopped.notify_one();
            })
            .map_err(LocalExecutorError::ThreadSpawnError)?;

        Ok(handle)
    }

    fn build_runtime(preferences: &TokioRuntimePreferences) -> Result<Runtime, LocalExecutorError> {
        let mut builder = Builder::new_current_thread();
        if preferences.enable_io {
            builder.enable_io();
        }
        if preferences.enable_time {
            builder.enable_time();
        }
        /* Disabled since it's current thread runtime
        if let Some(stack_size) = preferences.thread_stack_size {
            builder.thread_stack_size(stack_size);
        }
        */
        let rt = builder.build()?;
        Ok(rt)
    }
}
