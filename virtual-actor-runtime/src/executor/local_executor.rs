//! Local executor for actor

use std::sync::Arc;
use std::thread::JoinHandle;

use tokio::runtime::Builder;
use tokio::sync::Notify;

use tokio_util::sync::CancellationToken;
use virtual_actor::Actor;
use virtual_actor::ActorContext;
use virtual_actor::ActorFactory;

use crate::address::ActorHandle;
use crate::address::Addr;
use crate::context::ActorContextFactory;
use crate::utils::waiter::waiter;
use crate::utils::waiter::WaitError;
use crate::utils::GracefulShutdown;
use crate::utils::GracefulShutdownHandle;

use super::actor_registry::ActorRegistry;
use super::error::LocalExecutorError;
use super::local_actor;
use super::local_set_wrapper::LocalSetWrapper;
use super::spawner::LocalSpawner;
use super::spawner::SpawnerDispatcher;
use super::ExecutorPreferences;

/// Std thread handle
type ThreadHandle = std::thread::JoinHandle<()>;

/// Executor based on `tokio::task::LocalSet`
pub struct LocalExecutor {
    /// Thread handle
    thread_handle: ThreadHandle,
    /// Spawner dispatcher
    spawner_dispatcher: SpawnerDispatcher,
    /// Cancellation actor execution
    executor_cancellation: CancellationToken,
    /// Cancellation actor message processing
    mailbox_cancellation: CancellationToken,
    /// Graceful shutdown handle for spawner
    spawner_gs: GracefulShutdownHandle,
    /// Graceful shutdown handle for local set
    local_set_gs: GracefulShutdownHandle,
    /// Thread stopped notify
    thread_stopped_notify: Arc<Notify>,
    /// Actor registry
    registry: ActorRegistry,
}

impl GracefulShutdown for LocalExecutor {
    async fn graceful_shutdown(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        // cancel message receiving
        self.mailbox_cancellation.cancel();
        // wait for spawn task to finish processing all pending tasks
        // to ensure that no more actors will be spawned
        match self.spawner_gs.wait(timeout).await {
            Err(WaitError::Timeout(w)) => {
                eprintln!("{w} wait timeout");
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
                eprintln!("{w} wait timeout");
                // if local set is not finished, then shutdown actor execution
                self.executor_cancellation.cancel();
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
    /// Starts executor thread
    /// with preferences
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub fn new() -> Result<Self, LocalExecutorError> {
        let preferences = ExecutorPreferences::default();
        Self::with_preferences(preferences)
    }

    /// Starts executor thread
    /// with preferences
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    #[allow(clippy::needless_pass_by_value)]
    pub fn with_preferences(preferences: ExecutorPreferences) -> Result<Self, LocalExecutorError> {
        let registry = ActorRegistry::new();

        let executor_cancellation = CancellationToken::new();
        let mailbox_cancellation = CancellationToken::new();

        let thread_stopped_notify = Arc::new(Notify::new());

        let spawner = LocalSpawner::new(
            &preferences.mailbox_preferences,
            registry.clone(),
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
            spawner,
            local_set_stopped.clone(),
            thread_stopped_notify.clone(),
            local_set_cancellation.clone(),
        )?;

        Ok(Self {
            thread_handle,
            spawner_dispatcher,
            executor_cancellation,
            mailbox_cancellation,
            spawner_gs,
            local_set_gs,
            thread_stopped_notify,
            registry,
        })
    }

    /// Spawns actor on thread, without waiting for dispatcher to be set
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub fn spawn_actor_no_wait<A, AF, CF>(
        &mut self,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
    ) -> Result<Arc<ActorHandle<A>>, LocalExecutorError>
    where
        A: Actor + 'static,
        A::ActorContext: ActorContext<A, Addr = Addr<A>>,
        AF: ActorFactory<A> + 'static,
        CF: ActorContextFactory<A> + 'static,
    {
        let execution_ct = self.executor_cancellation.child_token();
        let mailbox_ct = self.mailbox_cancellation.child_token();
        let (local_actor, handle) =
            local_actor::create(actor_factory, context_factory, execution_ct, mailbox_ct);

        self.spawner_dispatcher
            .send(local_actor)
            .map_err(|e| LocalExecutorError::SpawnerSendError(format!("{e:?}")))?;

        Ok(handle)
    }

    /// Spawns actor on thread, without waiting for dispatcher to be set
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub async fn spawn_actor<A, AF, CF>(
        &mut self,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
    ) -> Result<Arc<ActorHandle<A>>, LocalExecutorError>
    where
        A: Actor + 'static,
        A::ActorContext: ActorContext<A, Addr = Addr<A>>,
        AF: ActorFactory<A> + 'static,
        CF: ActorContextFactory<A> + 'static,
    {
        let handle = self.spawn_actor_no_wait(actor_factory, context_factory)?;
        handle
            .wait_for_dispatcher(std::time::Duration::from_millis(100))
            .await?;
        Ok(handle)
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
                if !self.registry.is_empty() {
                    eprintln!("Registry is not empty");
                }
                Err(WaitError::Timeout(s))
            }
            Err(e) => Err(e),
        }
    }

    fn start_thread(
        spawner: LocalSpawner,
        local_set_stopped: Arc<Notify>,
        thread_stopped: Arc<Notify>,
        local_set_cancellation: CancellationToken,
    ) -> Result<JoinHandle<()>, LocalExecutorError> {
        let rt = Builder::new_current_thread().enable_all().build()?;

        let handle = std::thread::spawn(move || {
            let local = LocalSetWrapper::new();

            local.spawn_local(spawner.run());

            local.run(&rt, &local_set_stopped, &local_set_cancellation);
            thread_stopped.notify_one();
        });

        Ok(handle)
    }
}
