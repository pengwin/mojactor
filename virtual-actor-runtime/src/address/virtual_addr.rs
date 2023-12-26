use virtual_actor::{ActorAddr, Message, MessageEnvelopeFactory, MessageHandler, VirtualActor};

use crate::runtime::{ActorActivator, ActorSpawnError};

use super::{
    actor_task_container::ActorTaskContainerError, weak_virtual_addr::WeakVirtualAddr, ActorHandle,
    LocalAddrError,
};

/// Actor handler error
#[derive(thiserror::Error, Debug)]
pub enum VirtualAddrError {
    /// Actor spawn error
    #[error("ActorSpawnError {0:?}")]
    SpawnError(#[from] ActorSpawnError),
    /// Dispatcher error
    #[error("LocalAddrError {0:?}")]
    LocalAddrError(#[from] LocalAddrError),
    /// Actor task error
    #[error("ActorTaskContainerError {0:?}")]
    ActorTaskContainerError(#[from] ActorTaskContainerError),
}

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

    async fn get_addr(&self) -> Result<ActorHandle<A>, VirtualAddrError> {
        let handle = self.activator.get_or_spawn(&self.id).await?;

        Ok(handle)
    }
}

impl<A: VirtualActor> ActorAddr<A> for VirtualAddr<A> {
    type Error = VirtualAddrError;

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
            .map_err(VirtualAddrError::LocalAddrError)
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
