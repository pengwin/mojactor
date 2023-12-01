//! `ActorAddr` implementation

use std::sync::{Arc, Weak};

use virtual_actor::{
    Actor, ActorAddr, AddrError, Message, MessageEnvelopeFactory, MessageHandler, WeakActorRef,
};

use crate::actor_handle::ActorHandle;

/// Actor address
pub struct Addr<A: Actor> {
    /// Actor handler
    handle: Arc<ActorHandle<A>>,
}

impl<A: Actor> Addr<A> {
    /// Creates new actor address
    pub(crate) fn new(handle: &Arc<ActorHandle<A>>) -> Self {
        Self {
            handle: handle.clone(),
        }
    }

    /// Creates private clone
    pub(crate) fn create_clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<A: Actor> ActorAddr<A> for Addr<A> {
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
            handle: Arc::downgrade(&self.handle),
        }
    }
}

/// Weak actor address
pub struct WeakRef<A: Actor> {
    /// Actor weak handler
    handle: Weak<ActorHandle<A>>,
}

impl<A: Actor> Clone for WeakRef<A> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<A: Actor> WeakActorRef<A, Addr<A>> for WeakRef<A> {
    fn upgrade(&self) -> Option<Addr<A>> {
        match self.handle.upgrade() {
            Some(handle) => {
                if handle.is_cancelled() {
                    None
                } else {
                    Some(Addr::new(&handle))
                }
            }
            None => None,
        }
    }
}
