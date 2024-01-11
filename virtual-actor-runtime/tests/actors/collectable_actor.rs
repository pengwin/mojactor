use serde::Deserialize;
use serde::Serialize;
use virtual_actor_runtime::prelude::*;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(())]
pub struct Ping;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(u32)]
pub struct GetCounter;

#[derive(Actor, VirtualActor)]
#[message(Ping)]
#[message(GetCounter)]
pub struct CollectableActor {
    id: u32,
    counter: u32,
}

impl MessageHandler<Ping> for CollectableActor {
    async fn handle(&mut self, _msg: Ping, _ctx: &Self::ActorContext) -> <Ping as Message>::Result {
        self.counter += 1;
    }
}

impl MessageHandler<GetCounter> for CollectableActor {
    async fn handle(
        &mut self,
        _msg: GetCounter,
        _ctx: &Self::ActorContext,
    ) -> <GetCounter as Message>::Result {
        self.counter
    }
}

impl VirtualActorConstructor for CollectableActor {
    fn new(id: &u32) -> Self {
        Self {
            id: *id,
            counter: 0,
        }
    }
}
