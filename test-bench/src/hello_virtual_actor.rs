use serde::{Deserialize, Serialize};
use virtual_actor_runtime::prelude::*;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(Result<String, String>)]
pub struct HelloVirtualMessage {
    msg: &'static str,
}

impl HelloVirtualMessage {
    pub fn new(msg: &'static str) -> Self {
        Self { msg }
    }
}

#[derive(Actor, VirtualActor)]
#[message(HelloVirtualMessage)]
pub struct HelloVirtualActor {
    id: u32,
    counter: u32,
}

impl HelloVirtualActor {
    pub fn new(id: u32) -> Self {
        Self { id, counter: 0 }
    }
}

impl MessageHandler<HelloVirtualMessage> for HelloVirtualActor {
    async fn handle(
        &mut self,
        msg: HelloVirtualMessage,
        _ctx: &Self::ActorContext,
    ) -> <HelloVirtualMessage as Message>::Result {
        self.counter += 1;
        let result = format!("Hello {} {}", msg.msg, self.counter);
        Ok(result.to_string())
    }
}

#[derive(Default)]
pub struct HelloVirtualActorFactory;

impl ActorFactory for HelloVirtualActorFactory {
    type Actor = HelloVirtualActor;
}

impl VirtualActorFactory for HelloVirtualActorFactory {
    async fn create_actor(&self, id: &u32) -> HelloVirtualActor {
        HelloVirtualActor::new(*id)
    }
}
