use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use dashmap::DashMap;
use virtual_actor::{
    Actor, ActorAddr, ActorContext, ActorFactory, AddrError, VirtualActor, VirtualActorFactory,
};

use crate::{
    address::ActorHandle,
    context::ActorContextFactory,
    executor::{Handle, LocalExecutorError},
    runtime::runtime_preferences::RuntimePreferences,
    Addr, LocalExecutor, WaitError,
};

use super::{
    housekeeping::{
        GarbageCollectActors, HousekeepingActor, HousekeepingActorFactory,
        HousekeepingContextFactory,
    },
    virtual_actor_registration::{VirtualActorRegistration, VirtualActorSpawner},
};

#[derive(Debug, thiserror::Error)]
pub enum StartHousekeepingError {
    #[error("WaitDispatcherError {0:?}")]
    WaitDispatcherError(#[from] WaitError),
    #[error("StartGarbageCollectError {0:?}")]
    StartGarbageCollectError(#[from] AddrError),
}

#[derive(Debug, thiserror::Error)]
pub enum ActorSpawnError {
    #[error("StartHousekeepingError {0:?}")]
    StartHousekeeping(#[from] StartHousekeepingError),
    #[error("WaitDispatcherError {0:?}")]
    WaitDispatcher(#[from] WaitError),
    #[error("ExecutorError {0:?}")]
    ExecutorError(#[from] LocalExecutorError),
}

pub struct ActorActivator<A: VirtualActor> {
    registration: Box<dyn VirtualActorSpawner<A>>,
    cache: Arc<DashMap<A::ActorId, Arc<ActorHandle<A>>>>,
    housekeeping_actor: Arc<ActorHandle<HousekeepingActor<A>>>,
    /// Indicates that housekeeping has started
    /// Housekeeping is lazy started when first actor is spawned
    house_keeping_started: Arc<AtomicBool>,
}

impl<A: VirtualActor> ActorActivator<A> {
    pub fn new<AF, CF>(
        factory: AF,
        context_factory: Arc<CF>,
        executor: &LocalExecutor,
        housekeeping_executor: &Handle,
        preferences: Arc<RuntimePreferences>,
    ) -> Result<Self, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext:
            ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
        AF: VirtualActorFactory + 'static,
        AF: ActorFactory<Actor = A> + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        let cache = Arc::new(DashMap::new());
        let housekeeping_actor_factory = Arc::new(HousekeepingActorFactory::new(
            cache.clone(),
            Duration::from_millis(100),
            preferences,
        ));
        let housekeeping_context_factory =
            Arc::new(HousekeepingContextFactory::<<AF as ActorFactory>::Actor>::default());
        let housekeeping_actor = housekeeping_executor.spawn_local_actor_no_wait(
            &housekeeping_actor_factory,
            &housekeeping_context_factory,
        )?;
        Ok(Self {
            registration: Box::new(VirtualActorRegistration::new(
                factory,
                context_factory,
                executor,
            )),
            cache,
            housekeeping_actor,
            house_keeping_started: Arc::new(AtomicBool::new(false)),
        })
    }

    pub async fn spawn(
        &self,
        id: A::ActorId,
        wait_timeout: Duration,
    ) -> Result<Arc<ActorHandle<A>>, ActorSpawnError> {
        if let Some(handle) = self.cache.get(&id) {
            return Ok(handle.value().clone());
        }
        self.start_housekeeping().await?;
        let handle = self.registration.spawn_no_wait(id.clone())?;
        handle.wait_for_dispatcher(wait_timeout).await?;
        self.cache.insert(id, handle.clone());
        Ok(handle)
    }

    async fn start_housekeeping(&self) -> Result<(), StartHousekeepingError> {
        if self.house_keeping_started.load(Ordering::Relaxed) {
            return Ok(());
        }
        self.housekeeping_actor
            .wait_for_dispatcher(Duration::from_millis(100))
            .await?;

        let addr = self.housekeeping_actor.addr();
        addr.dispatch(GarbageCollectActors)?;

        self.house_keeping_started.store(true, Ordering::Relaxed);

        Ok(())
    }
}
