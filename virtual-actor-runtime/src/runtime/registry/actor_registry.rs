use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use virtual_actor::{
    names::ActorName, Actor, ActorContext, ActorFactory, VirtualActor, VirtualActorFactory,
};

use crate::{
    context::ActorContextFactory, executor::LocalExecutorError,
    runtime::runtime_preferences::RuntimePreferences, Addr, ExecutorPreferences, LocalExecutor,
    TokioRuntimePreferences,
};

use super::actor_activator::{ActorActivator, ActorSpawnError};

#[derive(Debug, thiserror::Error)]
pub enum ActivateActorError {
    #[error("Actor {0:?} not found")]
    ActorNotFound(ActorName),
    #[error("Unexpected activator registered for actor {0:?}")]
    UnexpectedActivator(ActorName),
    #[error("ActorSpawnError: {0:?}")]
    SpawnError(#[from] ActorSpawnError),
}

pub struct ActorRegistry {
    activators: DashMap<ActorName, Box<dyn std::any::Any + Send + Sync>>,
    housekeeping_executor: LocalExecutor,
}

impl ActorRegistry {
    pub fn new() -> Result<Self, LocalExecutorError> {
        Ok(Self {
            activators: DashMap::new(),
            housekeeping_executor: LocalExecutor::with_preferences(ExecutorPreferences {
                tokio_runtime_preferences: TokioRuntimePreferences {
                    enable_io: false,
                    enable_time: true,
                    ..Default::default()
                },
                ..Default::default()
            })?,
        })
    }

    pub fn register_actor<AF, CF>(
        &self,
        factory: AF,
        context_factory: Arc<CF>,
        executor: &LocalExecutor,
        preferences: Arc<RuntimePreferences>,
    ) -> Result<(), LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext:
            ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
        AF: VirtualActorFactory + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        let name = <AF as ActorFactory>::Actor::name();
        let activator = ActorActivator::new(
            factory,
            context_factory,
            executor,
            self.housekeeping_executor.handle(),
            preferences,
        )?;
        self.activators.insert(name, Box::new(activator));
        Ok(())
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
