
use std::sync::Arc;

use virtual_actor_runtime::{prelude::*, errors::BoxedActorError};
use virtual_actor_persistence::prelude::*;

use super::hello_virtual_actor::HelloVirtualMessage;

#[derive(Actor, VirtualActor)]
#[message(HelloVirtualMessage)]
pub struct HelloActorWithState {
    id: u32,
    state: ActorState<Self>,
}

impl MessageHandler<HelloVirtualMessage> for HelloActorWithState {
    async fn handle(
        &mut self,
        msg: HelloVirtualMessage,
        _ctx: &Self::ActorContext,
    ) -> <HelloVirtualMessage as Message>::Result {
        *self.state += 1;
        let result = format!("Hello {} {}", msg.msg(), &*self.state);
        self.state.save().await.map_err(|_| "unable to save state")?;
        Ok(result.to_string())
    }
}

impl ActorWithState for HelloActorWithState {
    type State = u32;

    fn state(&self) -> &ActorState<Self> {
        &self.state
    }
}

pub struct HelloActorWithStateFactory {
    persistence: Arc<dyn ActorPersistence<HelloActorWithState>>,
}

impl HelloActorWithStateFactory {
    pub fn new(persistence: Arc<dyn ActorPersistence<HelloActorWithState>>) -> Self {
        Self { persistence }
    }
}

impl ActorFactory for HelloActorWithStateFactory {
    type Actor = HelloActorWithState;
}

impl VirtualActorFactory for HelloActorWithStateFactory {
    type Error = BoxedActorError;

    async fn create_actor(
        &self,
        id: &<Self::Actor as VirtualActor>::ActorId,
    ) -> Result<Self::Actor, Self::Error> {
        let state = ActorState::load(&self.persistence, id).await?;
        Ok(Self::Actor { id: *id, state })
    }
}
