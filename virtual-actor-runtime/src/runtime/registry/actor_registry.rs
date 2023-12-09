use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use virtual_actor::{
    names::ActorName, Actor, ActorContext, ActorFactory, VirtualActor, VirtualActorFactory,
};

use crate::{context::ActorContextFactory, executor::LocalExecutorError, Addr, LocalExecutor};

use super::virtual_actor_registration::ActorActivator;

#[derive(Debug, thiserror::Error)]
pub enum ActivateActorError {
    #[error("Actor {0:?} not found")]
    ActorNotFound(ActorName),
    #[error("Unexpected activator registered for actor {0:?}")]
    UnexpectedActivator(ActorName),
    #[error("Local executor error: {0:?}")]
    ExecutorError(#[from] LocalExecutorError),
}

pub struct ActorRegistry {
    activators: DashMap<ActorName, Box<dyn std::any::Any + Send + Sync>>,
}

impl Default for ActorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ActorRegistry {
    pub fn new() -> Self {
        Self {
            activators: DashMap::new(),
        }
    }

    pub fn register_actor<AF, CF>(
        &self,
        factory: AF,
        context_factory: Arc<CF>,
        executor: &LocalExecutor,
    ) where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext:
            ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
        AF: VirtualActorFactory + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        let name = <AF as ActorFactory>::Actor::name();
        let activator = ActorActivator::new(factory, context_factory, executor);
        self.activators.insert(name, Box::new(activator));
    }

    /// Gets or creates virtual actor
    pub async fn get_or_create<A: VirtualActor>(
        &self,
        id: A::ActorId,
    ) -> Result<Addr<A>, ActivateActorError> {
        let name = A::name();
        let activator = self
            .activators
            .get(&name)
            .ok_or(ActivateActorError::ActorNotFound(name))?;
        let activator = activator
            .downcast_ref::<ActorActivator<A>>()
            .ok_or(ActivateActorError::UnexpectedActivator(name))?;
        let handle = activator.spawn(id, Duration::from_secs(1)).await?;
        Ok(handle.addr())
    }
}
