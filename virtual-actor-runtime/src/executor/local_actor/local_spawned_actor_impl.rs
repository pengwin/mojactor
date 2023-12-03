//! Local actor spawner implementation and trait

use std::{any::Any, marker::PhantomData, panic::AssertUnwindSafe, sync::Arc};

use super::actor_loop::ActorLoop;
use crate::context::ActorContextFactory;
use crate::executor::actor_registry::ActorRegistry;
use crate::{address::ActorHandle, address::Addr};
use futures::FutureExt;
use tokio::sync::Notify;
use virtual_actor::{Actor, ActorContext, ActorFactory};

use super::handle::{generate_actor_id, ActorId, ActorTaskJoinHandle, LocalActorHandle};
use super::{
    error::ActorTaskError, local_spawned_actor_trait::LocalSpawnedActor, mailbox::Mailbox,
};

/// Local actor implementation
pub struct LocalSpawnedActorImpl<A, AF, CF, AL>
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
    AL: ActorLoop<A, AF, CF> + 'static,
{
    /// Phantom data
    _a: PhantomData<fn(A) -> A>,
    /// Actor factory
    actor_factory: Arc<AF>,
    /// Actor context factory
    context_factory: Arc<CF>,
    /// Actor handle
    handle: Arc<ActorHandle<A>>,
    /// Actor loop
    actor_loop: AL,
}

impl<A, AF, CF, AL> LocalSpawnedActorImpl<A, AF, CF, AL>
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
    AL: ActorLoop<A, AF, CF> + 'static,
{
    /// Creates new local actor
    pub fn new(
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
        handle: &Arc<ActorHandle<A>>,
        actor_loop: AL,
    ) -> Self
    where
        A: Actor + 'static,
        A::ActorContext: ActorContext<A, Addr = Addr<A>>,
        AF: ActorFactory<A> + 'static,
        CF: ActorContextFactory<A> + 'static,
    {
        Self {
            _a: PhantomData,
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
        actor_id: &ActorId,
        registry: &ActorRegistry,
        notify: &Notify,
    ) {
        if let Err(e) = result {
            eprintln!("Actor task error: {e:?}");
        }
        registry.remove_actor(actor_id);
        notify.notify_one();
    }

    fn spawn_actor(
        &self,
        actor_id: ActorId,
        mailbox: Mailbox<A>,
        actor_registry: &ActorRegistry,
    ) -> ActorTaskJoinHandle {
        let handle = self.handle.clone();
        let actor_registry = actor_registry.clone();
        let stop_notify = handle.stop_notify().clone();
        let actor_loop = self.actor_loop.clone();
        tokio::task::spawn_local(
            AssertUnwindSafe(actor_loop.actor_loop(
                mailbox,
                self.actor_factory.clone(),
                self.context_factory.clone(),
                handle.clone(),
            ))
            .catch_unwind()
            .map(Self::unwind_panic)
            .inspect(move |x| Self::finish_actor(x, &actor_id, &actor_registry, &stop_notify)),
        )
    }
}

impl<A, AF, CF, AL> LocalSpawnedActor for LocalSpawnedActorImpl<A, AF, CF, AL>
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
    AL: ActorLoop<A, AF, CF> + 'static,
{
    fn spawn(&self, actor_registry: &ActorRegistry) {
        let mailbox_preferences = self.actor_factory.mailbox_preferences();
        let (dispatcher, mailbox) =
            Mailbox::<A>::new(mailbox_preferences, self.handle.mailbox_cancellation());

        self.handle
            .set_dispatcher(dispatcher)
            .expect("Dispatcher already set");

        let actor_id = generate_actor_id();
        let stop_notify_handler = self.handle.stop_notify().clone();

        let task = self.spawn_actor(actor_id, mailbox, actor_registry);

        let spawned_actor = LocalActorHandle::new(
            actor_id,
            task,
            &stop_notify_handler,
            self.handle.cancellation_token(),
        );
        actor_registry.add_actor(spawned_actor);
    }
}
