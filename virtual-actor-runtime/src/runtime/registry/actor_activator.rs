use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Weak,
};

use tokio::sync::Mutex;
use virtual_actor::{
    Actor, ActorAddr, ActorContext, ActorFactory, VirtualActor, VirtualActorFactory,
};

use crate::{
    address::{ActorHandle, ActorStartError, LocalAddrError},
    context::ActorContextFactory,
    errors::WaitError,
    executor::{Handle, LocalExecutorError},
    runtime::runtime_preferences::RuntimePreferences,
    ExecutorHandle, LocalAddr,
};

use super::{
    actors_cache::ActorsCache,
    housekeeping::{
        GarbageCollectActors, HousekeepingActor, HousekeepingActorFactory,
        HousekeepingContextFactory,
    },
    virtual_actor_registration::{VirtualActorRegistration, VirtualActorSpawner},
};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum StartHousekeepingError {
    #[error("WaitDispatcherError {0:?}")]
    WaitDispatcherError(#[from] WaitError),
    #[error("StartGarbageCollectError {0:?}")]
    StartGarbageCollectError(#[from] LocalAddrError),
    /// Actor start error
    #[error("Actor start error {0:?}")]
    ActorStartError(#[from] ActorStartError),
}

/// Actor spawn error
#[derive(Debug, thiserror::Error)]
pub enum ActorSpawnError {
    /// Start housekeeping error
    #[error("StartHousekeepingError {0:?}")]
    StartHousekeeping(#[from] StartHousekeepingError),
    /// Wait dispatcher error
    #[error("WaitDispatcherError {0:?}")]
    WaitDispatcher(#[from] WaitError),
    /// Executor error
    #[error("ExecutorError {0:?}")]
    ExecutorError(#[from] LocalExecutorError),
    /// Actor start error
    #[error("Actor start error {0:?}")]
    ActorStartError(#[from] ActorStartError),
}

pub struct ActorActivator<A: VirtualActor> {
    inner: Arc<Inner<A>>,
}

impl<A: VirtualActor> Clone for ActorActivator<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct Inner<A: VirtualActor> {
    registration: Box<dyn VirtualActorSpawner<A>>,
    cache: ActorsCache<A>,
    housekeeping_actor: LocalAddr<HousekeepingActor<A>>,
    /// Indicates that housekeeping has started
    /// Housekeeping is lazy started when first actor is spawned
    house_keeping_started: Arc<AtomicBool>,
    /// Housekeeping sync primitive, to prevent starting multiple garbage collection
    housekeeping_lock: Arc<Mutex<bool>>,
    /// Preferences
    preferences: Arc<RuntimePreferences>,
}

impl<A: VirtualActor> ActorActivator<A> {
    pub fn new<AF, CF>(
        factory: AF,
        context_factory: Arc<CF>,
        executor: &ExecutorHandle,
        housekeeping_executor: &Handle,
        preferences: Arc<RuntimePreferences>,
    ) -> Result<Self, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: VirtualActorFactory + 'static,
        AF: ActorFactory<Actor = A> + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        let cache = ActorsCache::new();
        let housekeeping_actor_factory = Arc::new(HousekeepingActorFactory::new(
            housekeeping_executor.mailbox_cancellation().child_token(),
            cache.clone(),
            &preferences,
        ));
        let context_cancellation = housekeeping_executor.executor_cancellation().child_token();
        let housekeeping_context_factory =
            HousekeepingContextFactory::<<AF as ActorFactory>::Actor>::new(context_cancellation);
        let housekeeping_context_factory = Arc::new(housekeeping_context_factory);
        let housekeeping_actor = housekeeping_executor
            .spawn_local_actor_no_wait(&housekeeping_actor_factory, &housekeeping_context_factory)?
            .addr();
        Ok(Self {
            inner: Arc::new(Inner {
                registration: Box::new(VirtualActorRegistration::new(
                    factory,
                    context_factory,
                    executor,
                )),
                cache,
                housekeeping_actor,
                house_keeping_started: Arc::new(AtomicBool::new(false)),
                housekeeping_lock: Arc::new(Mutex::new(false)),
                preferences,
            }),
        })
    }

    pub fn weak_ref(&self) -> WeakActorActivator<A> {
        WeakActorActivator {
            inner: Arc::downgrade(&self.inner),
        }
    }

    pub async fn get_or_spawn(&self, id: &A::ActorId) -> Result<ActorHandle<A>, ActorSpawnError> {
        if let Some(handle) = self.inner.cache.get(id) {
            return Ok(handle);
        }
        self.start_housekeeping().await?;
        let handle = self.inner.registration.spawn_no_wait(id.clone())?;
        handle
            .wait_for_ready(self.inner.preferences.actor_activation_timeout)
            .await?;
        self.inner.cache.insert(id.clone(), handle.clone());
        Ok(handle)
    }

    async fn start_housekeeping(&self) -> Result<(), StartHousekeepingError> {
        if self.inner.house_keeping_started.load(Ordering::Relaxed) {
            return Ok(());
        }

        let mut lock = self.inner.housekeeping_lock.lock().await;

        // double check
        if *lock {
            return Ok(());
        }

        self.inner
            .housekeeping_actor
            .wait_for_ready(self.inner.preferences.actor_activation_timeout)
            .await?;

        self.inner
            .housekeeping_actor
            .dispatch(GarbageCollectActors)
            .await?;

        self.inner
            .house_keeping_started
            .store(true, Ordering::Relaxed);

        *lock = true;
        drop(lock);

        Ok(())
    }
}

pub struct WeakActorActivator<A: VirtualActor> {
    inner: Weak<Inner<A>>,
}

impl<A: VirtualActor> Clone for WeakActorActivator<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<A: VirtualActor> WeakActorActivator<A> {
    pub fn upgrade(&self) -> Option<ActorActivator<A>> {
        let inner = self.inner.upgrade()?;
        Some(ActorActivator { inner })
    }
}
