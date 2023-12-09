//! Local actor spawner implementation and trait

use std::{any::Any, panic::AssertUnwindSafe, sync::Arc};

use super::actor_loop::ActorLoop;
use crate::context::ActorContextFactory;
use crate::executor::actor_tasks_registry::{
    ActorTaskJoinHandle, ActorTasksRegistry, SpawnedActorId,
};
use crate::{address::ActorHandle, address::Addr};
use futures::FutureExt;
use tokio::sync::Notify;
use virtual_actor::{Actor, ActorContext, ActorFactory};

use super::{
    error::ActorTaskError, local_spawned_actor_trait::LocalSpawnedActor, mailbox::Mailbox,
};

/// Local actor implementation
pub struct LocalSpawnedActorImpl<AF, CF, AL>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    AL: ActorLoop<AF, CF> + 'static,
{
    id: SpawnedActorId,
    /// Actor factory
    actor_factory: Arc<AF>,
    /// Actor context factory
    context_factory: Arc<CF>,
    /// Actor handle
    handle: Arc<ActorHandle<<AF as ActorFactory>::Actor>>,
    /// Actor loop
    actor_loop: AL,
}

impl<AF, CF, AL> LocalSpawnedActorImpl<AF, CF, AL>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    AL: ActorLoop<AF, CF> + 'static,
{
    /// Creates new local actor
    pub fn new(
        id: SpawnedActorId,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
        handle: &Arc<ActorHandle<<AF as ActorFactory>::Actor>>,
        actor_loop: AL,
    ) -> Self
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext:
            ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
        AF: ActorFactory + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
        AL: ActorLoop<AF, CF> + 'static,
    {
        Self {
            id,
            actor_factory: actor_factory.clone(),
            context_factory: context_factory.clone(),
            handle: handle.clone(),
            actor_loop,
        }
    }

    fn unwind_panic(
        result: Result<Result<(), ActorTaskError>, Box<dyn Any + Send>>,
    ) -> Result<(), ActorTaskError> {
        match result {
            Ok(r) => r,
            Err(e) => match e.downcast_ref::<&str>() {
                Some(s) => Err(ActorTaskError::ActorPanic((*s).to_string())),
                None => Err(ActorTaskError::ActorPanic("Unknown panic".to_string())),
            },
        }
    }

    fn finish_actor(
        result: &Result<(), ActorTaskError>,
        id: &SpawnedActorId,
        registry: &ActorTasksRegistry,
        notify: &Notify,
    ) {
        if let Err(e) = result {
            eprintln!("Actor task error: {e:?}");
        }
        notify.notify_one();
        registry.unregister_actor(id);
    }

    fn spawn_actor(
        &self,
        mailbox: Mailbox<<AF as ActorFactory>::Actor>,
        registry: Arc<ActorTasksRegistry>,
    ) -> ActorTaskJoinHandle {
        let handle = self.handle.clone();
        let stop_notify = handle.stop_notify().clone();
        let actor_loop = self.actor_loop.clone();
        let id = self.id;
        tokio::task::spawn_local(
            AssertUnwindSafe(actor_loop.actor_loop(
                mailbox,
                self.actor_factory.clone(),
                self.context_factory.clone(),
                handle.clone(),
            ))
            .catch_unwind()
            .map(Self::unwind_panic)
            .inspect(move |x| Self::finish_actor(x, &id, &registry, &stop_notify)),
        )
    }
}

impl<AF, CF, AL> LocalSpawnedActor for LocalSpawnedActorImpl<AF, CF, AL>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    AL: ActorLoop<AF, CF> + 'static,
{
    fn spawn(&self, registry: Arc<ActorTasksRegistry>) -> ActorTaskJoinHandle {
        let mailbox_preferences = self.actor_factory.mailbox_preferences();
        let (dispatcher, mailbox) = Mailbox::<<AF as ActorFactory>::Actor>::new(
            mailbox_preferences,
            self.handle.mailbox_cancellation(),
        );

        self.handle
            .set_dispatcher(dispatcher)
            .expect("Dispatcher already set");

        self.spawn_actor(mailbox, registry)
    }

    fn id(&self) -> SpawnedActorId {
        self.id
    }
}
