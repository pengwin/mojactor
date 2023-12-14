//! `ActorAddr` implementation

use virtual_actor::{
    Actor, ActorAddr, AddrError, Message, MessageEnvelopeFactory, MessageHandler, WeakActorRef,
};

use super::actor_handle::{ActorHandle, WeakActorHandle};

/// Actor address
pub struct LocalAddr<A: Actor> {
    /// Actor handler
    handle: ActorHandle<A>,
}

impl<A: Actor> LocalAddr<A> {
    /// Creates new actor address
    pub(crate) fn new(handle: &ActorHandle<A>) -> Self {
        Self {
            handle: handle.clone(),
        }
    }
}

impl<A: Actor> Clone for LocalAddr<A> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<A: Actor> ActorAddr<A> for LocalAddr<A> {
    type WeakRef = WeakRef<A>;

    async fn send<M>(&self, msg: M) -> Result<M::Result, AddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        self.handle.send(msg).await
    }

    fn dispatch<M>(&self, msg: M) -> Result<(), AddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        self.handle.dispatch(msg)
    }

    fn weak_ref(&self) -> Self::WeakRef {
        WeakRef {
            weak_handle: self.handle.weak_ref(),
        }
    }
}

/// Weak actor address
pub struct WeakRef<A: Actor> {
    /// Actor weak handler
    weak_handle: WeakActorHandle<A>,
}

impl<A: Actor> Clone for WeakRef<A> {
    fn clone(&self) -> Self {
        Self {
            weak_handle: self.weak_handle.clone(),
        }
    }
}

impl<A: Actor> WeakActorRef<A, LocalAddr<A>> for WeakRef<A> {
    fn upgrade(&self) -> Option<LocalAddr<A>> {
        match self.weak_handle.upgrade() {
            Some(handle) => {
                if handle.is_cancelled() {
                    None
                } else {
                    Some(LocalAddr::new(&handle))
                }
            }
            None => None,
        }
    }
}
