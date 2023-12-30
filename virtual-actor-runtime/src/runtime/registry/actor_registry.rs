use std::sync::{Arc, Weak};

use dashmap::DashMap;
use virtual_actor::{
    actor::{Actor, ActorContext, ActorFactory, ActorName},
    virtual_actor::{VirtualActor, VirtualActorFactory},
};

use crate::{
    address::VirtualAddr, context::ActorContextFactory, executor::LocalExecutorError,
    runtime::runtime_preferences::RuntimePreferences, ExecutorHandle, LocalAddr,
};

use super::actor_activator::{ActorActivator, ActorSpawnError};

#[derive(Debug, thiserror::Error)]
pub enum ActivateActorError {
    #[error("Actor {0:?} not found")]
    ActorNotFound(ActorName),
    #[error("Actor registry dropped")]
    ActorRegistryDropped,
    #[error("Unexpected activator registered for actor {0:?}")]
    UnexpectedActivator(ActorName),
    #[error("ActorSpawnError: {0:?}")]
    SpawnError(#[from] ActorSpawnError),
}

pub struct ActorRegistry {
    inner: Arc<Inner>,
}

#[derive(Clone)]
pub struct WeakActorRegistry {
    inner: Weak<Inner>,
}

impl WeakActorRegistry {
    /// Gets or creates virtual actor
    pub fn get_or_create<A: VirtualActor>(
        &self,
        id: &A::ActorId,
    ) -> Result<VirtualAddr<A>, ActivateActorError> {
        let inner = self
            .inner
            .upgrade()
            .ok_or(ActivateActorError::ActorRegistryDropped)?;
        let reg = ActorRegistry { inner };
        reg.get_or_create(id)
    }
}

struct Inner {
    activators: DashMap<ActorName, Box<dyn std::any::Any + Send + Sync>>,
    housekeeping_executor: ExecutorHandle,
}

impl ActorRegistry {
    pub fn new(housekeeping_executor: &ExecutorHandle) -> Self {
        let inner = Inner {
            activators: DashMap::new(),
            housekeeping_executor: housekeeping_executor.clone(),
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn weak_ref(&self) -> WeakActorRegistry {
        WeakActorRegistry {
            inner: Arc::downgrade(&self.inner),
        }
    }

    pub fn register_actor<AF, CF>(
        &self,
        factory: AF,
        context_factory: Arc<CF>,
        executor: &ExecutorHandle,
        preferences: Arc<RuntimePreferences>,
    ) -> Result<(), LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: VirtualActorFactory + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        let name = <AF as ActorFactory>::Actor::name();
        let activator = ActorActivator::new(
            factory,
            context_factory,
            executor,
            &self.inner.housekeeping_executor,
            preferences,
        )?;
        self.inner.activators.insert(name, Box::new(activator));
        Ok(())
    }

    /// Gets or creates virtual actor
    pub fn get_or_create<A: VirtualActor>(
        &self,
        id: &A::ActorId,
    ) -> Result<VirtualAddr<A>, ActivateActorError> {
        let name = A::name();
        let activator = self
            .inner
            .activators
            .get(&name)
            .ok_or(ActivateActorError::ActorNotFound(name))?;
        let activator = activator
            .downcast_ref::<ActorActivator<A>>()
            .ok_or(ActivateActorError::UnexpectedActivator(name))?;

        let addr = VirtualAddr::new(id, activator);
        Ok(addr)
    }
}
