use serde::Deserialize;
use serde::Serialize;
use virtual_actor_runtime::prelude::*;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(Result<(), String>)]
pub struct VirtualPing;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(())]
pub struct VirtualPong;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(u32)]
pub struct VirtualGetCounter;

#[derive(Actor, VirtualActor)]
#[message(VirtualPing)]
#[message(VirtualGetCounter)]
pub struct VirtualPingActor {
    id: u32,
    counter: u32,
}

#[derive(Actor, VirtualActor)]
#[message(VirtualPong)]
#[message(VirtualGetCounter)]
pub struct VirtualPongActor {
    id: u32,
    counter: u32,
}

impl MessageHandler<VirtualPing> for VirtualPingActor {
    async fn handle(
        &mut self,
        _msg: VirtualPing,
        ctx: &Self::ActorContext,
    ) -> <VirtualPing as Message>::Result {
        let addr = ctx
            .get_or_create::<VirtualPongActor>(&self.id)
            .await
            .map_err(|err| err.to_string())?;
        addr.dispatch(VirtualPong)
            .await
            .map_err(|err| err.to_string())?;
        self.counter += 1;
        Ok(())
    }
}

impl MessageHandler<VirtualPong> for VirtualPongActor {
    async fn handle(
        &mut self,
        _msg: VirtualPong,
        _ctx: &Self::ActorContext,
    ) -> <VirtualPong as Message>::Result {
        self.counter += 1;
    }
}

impl MessageHandler<VirtualGetCounter> for VirtualPingActor {
    async fn handle(
        &mut self,
        _msg: VirtualGetCounter,
        _ctx: &Self::ActorContext,
    ) -> <VirtualGetCounter as Message>::Result {
        self.counter
    }
}

impl MessageHandler<VirtualGetCounter> for VirtualPongActor {
    async fn handle(
        &mut self,
        _msg: VirtualGetCounter,
        _ctx: &Self::ActorContext,
    ) -> <VirtualGetCounter as Message>::Result {
        self.counter
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct VirtualPingActorFactory;

impl ActorFactory for VirtualPingActorFactory {
    type Actor = VirtualPingActor;
}

impl VirtualActorFactory for VirtualPingActorFactory {
    async fn create_actor(&self, id: &u32) -> VirtualPingActor {
        VirtualPingActor {
            id: *id,
            counter: 0,
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct VirtualPongActorFactory;

impl ActorFactory for VirtualPongActorFactory {
    type Actor = VirtualPongActor;
}

impl VirtualActorFactory for VirtualPongActorFactory {
    async fn create_actor(&self, id: &u32) -> VirtualPongActor {
        VirtualPongActor {
            id: *id,
            counter: 0,
        }
    }
}
