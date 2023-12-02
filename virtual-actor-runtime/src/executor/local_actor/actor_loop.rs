use std::sync::Arc;

use std::future::Future;

use tokio::select;
use virtual_actor::{Actor, ActorContext, ActorFactory};

use crate::{address::ActorHandle, context::ActorContextFactory, Addr};

use super::{error::ActorTaskError, mailbox::Mailbox};

/// Actor loop trait
pub trait ActorLoop<A, AF, CF>
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    /// Message processing loop for an actor.
    fn actor_loop(
        mailbox: Mailbox<A>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: Arc<ActorHandle<A>>,
    ) -> impl Future<Output = Result<(), ActorTaskError>>;
}

impl<A, AF, CF> ActorLoop<A, AF, CF> for A
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    async fn actor_loop(
        mut mailbox: Mailbox<A>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: Arc<ActorHandle<A>>,
    ) -> Result<(), ActorTaskError> {
        let mut actor = actor_factory.create_actor().await;

        let context = context_factory.create_context(&handle);

        let task_ct = handle.cancellation_token();
        while let Some(envelope) = mailbox.recv(task_ct).await {
            select! {
                r = actor.handle_envelope(envelope, &context) => r.map_err(ActorTaskError::ResponderError),
                () = task_ct.cancelled() => Err(ActorTaskError::Cancelled),
            }?;
        }
        Ok(())
    }
}
