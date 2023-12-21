//! Local actor spawner implementation and trait

use std::{any::Any, panic::AssertUnwindSafe, sync::Arc};

use super::actor_loop::ActorLoop;
use super::ActorSpawnError;
use crate::address::ActorTask;
use crate::context::ActorContextFactory;
use crate::utils::atomic_counter::AtomicCounter;
use crate::utils::notify_once::NotifyOnce;
use crate::{address::ActorHandle, address::LocalAddr};
use futures::FutureExt;
use virtual_actor::{Actor, ActorContext, ActorFactory};

use super::{
    error::ActorTaskError, local_spawned_actor_trait::LocalSpawnedActor, mailbox::Mailbox,
};

/// Local actor implementation
pub struct LocalSpawnedActorImpl<AF, CF, AL>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    AL: ActorLoop<AF, CF> + 'static,
{
    /// Actor factory
    actor_factory: Arc<AF>,
    /// Actor context factory
    context_factory: Arc<CF>,
    /// Actor handle
    handle: ActorHandle<<AF as ActorFactory>::Actor>,
    /// Actor loop
    actor_loop: AL,
    /// Counter of messages dispatched to actor
    dispatched_msg_counter: AtomicCounter,
}

impl<AF, CF, AL> LocalSpawnedActorImpl<AF, CF, AL>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    AL: ActorLoop<AF, CF> + 'static,
{
    /// Creates new local actor
    pub fn new(
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
        handle: &ActorHandle<<AF as ActorFactory>::Actor>,
        actor_loop: AL,
        dispatched_msg_counter: AtomicCounter,
    ) -> Self
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: ActorFactory + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
        AL: ActorLoop<AF, CF> + 'static,
    {
        Self {
            actor_factory: actor_factory.clone(),
            context_factory: context_factory.clone(),
            handle: handle.clone(),
            actor_loop,
            dispatched_msg_counter,
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

    fn finish_actor(result: &Result<(), ActorTaskError>, notify: &NotifyOnce) {
        if let Err(e) = result {
            eprintln!("Actor task error: {e:?}");
        }
        notify.notify();
    }

    fn spawn_actor(&self, mailbox: Mailbox<<AF as ActorFactory>::Actor>) -> ActorTask {
        let handle = self.handle.clone();
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
            .inspect(move |x| Self::finish_actor(x, &stop_notify)),
        )
    }
}

impl<AF, CF, AL> LocalSpawnedActor for LocalSpawnedActorImpl<AF, CF, AL>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    AL: ActorLoop<AF, CF> + 'static,
{
    fn spawn(&self) -> Result<(), ActorSpawnError> {
        let mailbox_preferences = self.actor_factory.mailbox_preferences();
        let (dispatcher, mailbox) = Mailbox::<<AF as ActorFactory>::Actor>::new(
            mailbox_preferences,
            self.handle.mailbox_cancellation(),
            &self.dispatched_msg_counter,
        );

        self.handle
            .set_dispatcher(dispatcher)
            .map_err(ActorSpawnError::DispatcherAlreadySet)?;

        let task = self.spawn_actor(mailbox);

        self.handle
            .set_task(task)
            .map_err(ActorSpawnError::ActorTaskAlreadySet)?;

        Ok(())
    }
}
