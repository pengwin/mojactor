use virtual_actor::{
    actor::ActorAddr,
    message::{Message, MessageEnvelopeFactory, MessageHandler},
    virtual_actor::VirtualActor,
};

use crate::runtime::ActorActivator;

use super::{weak_virtual_addr::WeakVirtualAddr, ActorHandle};

/// Virtual actor address
pub struct VirtualAddr<A: VirtualActor> {
    id: A::ActorId,
    activator: ActorActivator<A>,
}

impl<A: VirtualActor> VirtualAddr<A> {
    pub(crate) fn new(id: &A::ActorId, activator: &ActorActivator<A>) -> Self {
        VirtualAddr {
            id: id.clone(),
            activator: activator.clone(),
        }
    }

    async fn get_addr(&self) -> Result<ActorHandle<A>, super::errors::VirtualAddrError> {
        let handle = self.activator.get_or_spawn(&self.id).await?;

        Ok(handle)
    }
}

impl<A: VirtualActor> ActorAddr<A> for VirtualAddr<A> {
    type Error = super::errors::VirtualAddrError;

    type WeakRef = WeakVirtualAddr<A>;

    async fn send<M>(&self, msg: M) -> Result<M::Result, Self::Error>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        let addr = self.get_addr().await?;
        addr.send(msg)
            .await
            .map_err(super::errors::VirtualAddrError::LocalAddrError)
    }

    async fn dispatch<M>(&self, msg: M) -> Result<(), Self::Error>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        let addr = self.get_addr().await?;

        addr.dispatch(msg)?;

        Ok(())
    }

    fn weak_ref(&self) -> Self::WeakRef {
        WeakVirtualAddr::new(&self.id, &self.activator)
    }
}
