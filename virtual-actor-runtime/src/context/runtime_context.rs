//! Runtime context for actor.

use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorContext, VirtualActor};

use crate::{
    address::Addr,
    runtime::{ActivateActorError, ActorRegistry},
};

use super::cancellation_token_wrapper::CancellationTokenWrapper;

/// Runtime context for actor.
pub struct RuntimeContext<A: Actor> {
    /// Actor address
    self_addr: Addr<A>,
    /// Cancellation token
    cancellation_token: CancellationTokenWrapper,
    /// Mailbox cancellation token
    mailbox_cancellation_token: CancellationToken,
    /// Actor registry
    registry: Arc<ActorRegistry>,
}

impl<A: Actor> RuntimeContext<A> {
    pub(crate) fn new(
        registry: Arc<ActorRegistry>,
        self_addr: Addr<A>,
        mailbox_cancellation_token: &CancellationToken,
        cancellation_token: &CancellationToken,
    ) -> Self {
        Self {
            self_addr,
            mailbox_cancellation_token: mailbox_cancellation_token.clone(),
            cancellation_token: CancellationTokenWrapper::new(cancellation_token.clone()),
            registry,
        }
    }

    /// Gets or creates virtual actor
    ///
    /// # Errors
    ///
    /// Returns error if actor cannot be activated
    pub async fn get_or_create<VA: VirtualActor>(
        &self,
        id: VA::ActorId,
    ) -> Result<Addr<VA>, ActivateActorError> {
        self.registry.get_or_create(id).await
    }
}

impl<A> Clone for RuntimeContext<A>
where
    A: Actor,
{
    fn clone(&self) -> Self {
        Self {
            self_addr: self.self_addr.create_clone(),
            mailbox_cancellation_token: self.mailbox_cancellation_token.clone(),
            cancellation_token: self.cancellation_token.clone(),
            registry: self.registry.clone(),
        }
    }
}

impl<A: Actor> ActorContext<A> for RuntimeContext<A> {
    type Addr = Addr<A>;
    type CancellationToken = CancellationTokenWrapper;

    fn self_addr(&self) -> &Self::Addr {
        &self.self_addr
    }

    fn stop(&self) {
        self.mailbox_cancellation_token.cancel();
    }

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &self.cancellation_token
    }
}
