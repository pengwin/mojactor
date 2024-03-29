use serde::{Deserialize, Serialize};
use virtual_actor_runtime::prelude::*;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(Result<String, String>)]
pub struct HelloVirtualMessage {
    msg: String,
}

impl HelloVirtualMessage {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

#[derive(Actor, VirtualActor)]
#[message(HelloVirtualMessage)]
pub struct HelloVirtualActor {
    id: u32,
    counter: u32,
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

impl VirtualActorConstructor for HelloVirtualActor {
    fn new(id: &u32) -> Self {
        Self {
            id: *id,
            counter: 0,
        }
    }
}
