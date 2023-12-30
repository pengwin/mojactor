use virtual_actor::{
    actor::{Actor, ActorAddr, WeakActorAddr},
    message::{Message, MessageEnvelopeFactory, MessageHandler},
};

use super::{actor_handle::WeakActorHandle, ActorHandle};

/// Weak actor address
pub struct WeakLocalAddr<A: Actor> {
    /// Actor weak handler
    weak_handle: WeakActorHandle<A>,
}

impl<A: Actor> WeakLocalAddr<A> {
    /// Creates new weak actor address
    pub(crate) fn new(handle: &ActorHandle<A>) -> Self {
        Self {
            weak_handle: handle.weak_ref(),
        }
    }
}

impl<A: Actor> Clone for WeakLocalAddr<A> {
    fn clone(&self) -> Self {
        Self {
            weak_handle: self.weak_handle.clone(),
        }
    }
}

impl<A: Actor> WeakActorAddr<A> for WeakLocalAddr<A> {
    type Upgraded = UpgradedWeakLocalAddr<A>;

    fn upgrade(&self) -> Option<Self::Upgraded> {
        match self.weak_handle.upgrade() {
            Some(handle) => {
                if handle.is_cancelled() {
                    None
                } else {
                    Some(UpgradedWeakLocalAddr::new(&handle))
                }
            }
            None => None,
        }
    }
}

/// Weak upgraded actor address
pub struct UpgradedWeakLocalAddr<A: Actor> {
    handle: ActorHandle<A>,
}

impl<A: Actor> UpgradedWeakLocalAddr<A> {
    /// Creates new upgraded weak actor address
    pub(crate) fn new(handle: &ActorHandle<A>) -> Self {
        Self {
            handle: handle.clone(),
        }
    }
}

impl<A: Actor> ActorAddr<A> for UpgradedWeakLocalAddr<A> {
    type Error = super::errors::LocalAddrError;
    type WeakRef = WeakLocalAddr<A>;

    async fn send<M>(&self, msg: M) -> Result<M::Result, super::errors::LocalAddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        self.handle.send(msg).await
    }

    async fn dispatch<M>(&self, msg: M) -> Result<(), super::errors::LocalAddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        self.handle.dispatch(msg)
    }

    fn weak_ref(&self) -> Self::WeakRef {
        WeakLocalAddr::new(&self.handle)
    }
}
