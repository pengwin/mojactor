//! Runtime context for actor.

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorAddr, ActorContext, VirtualActor};

use crate::{
    address::{LocalAddr, VirtualAddr},
    runtime::{ActivateActorError, WeakActorRegistry},
    utils::cancellation_token_wrapper::CancellationTokenWrapper,
    WeakLocalAddr,
};

/// Runtime context for actor.
pub struct RuntimeContext<A: Actor> {
    /// Actor weak address
    self_addr_weak: WeakLocalAddr<A>,
    /// Cancellation token
    cancellation_token: CancellationTokenWrapper,
    /// Mailbox cancellation token
    mailbox_cancellation_token: CancellationToken,
    /// Actor registry
    registry: WeakActorRegistry,
}

impl<A: Actor> RuntimeContext<A> {
    pub(crate) fn new(
        registry: WeakActorRegistry,
        self_addr_weak: WeakLocalAddr<A>,
        mailbox_cancellation_token: &CancellationToken,
        cancellation_token: &CancellationToken,
    ) -> Self {
        Self {
            self_addr_weak,
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
    #[allow(clippy::unused_async)]
    pub async fn get_or_create<VA: VirtualActor>(
        &self,
        id: &VA::ActorId,
    ) -> Result<VirtualAddr<VA>, ActivateActorError> {
        self.registry.get_or_create(id)
    }
}

impl<A> Clone for RuntimeContext<A>
where
    A: Actor,
{
    fn clone(&self) -> Self {
        Self {
            self_addr_weak: self.self_addr_weak.clone(),
            mailbox_cancellation_token: self.mailbox_cancellation_token.clone(),
            cancellation_token: self.cancellation_token.clone(),
            registry: self.registry.clone(),
        }
    }
}

impl<A: Actor> ActorContext<A> for RuntimeContext<A> {
    type Addr = LocalAddr<A>;
    type CancellationToken = CancellationTokenWrapper;

    fn self_addr(&self) -> &<Self::Addr as ActorAddr<A>>::WeakRef {
        &self.self_addr_weak
    }

    fn stop(&self) {
        self.mailbox_cancellation_token.cancel();
    }

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &self.cancellation_token
    }
}
