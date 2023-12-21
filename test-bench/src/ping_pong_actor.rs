use virtual_actor_runtime::prelude::*;
use virtual_actor_runtime::WeakLocalAddr;

#[derive(Message)]
#[result(())]
pub struct Ping {
    addr: WeakLocalAddr<PingPongActor>,
}

impl Ping {
    pub fn new(addr: WeakLocalAddr<PingPongActor>) -> Self {
        Self { addr }
    }
}

#[derive(Message)]
#[result(())]
pub struct Pong {
    addr: WeakLocalAddr<PingPongActor>,
}

impl Pong {
    pub fn new(addr: WeakLocalAddr<PingPongActor>) -> Self {
        Self { addr }
    }
}

#[derive(Message)]
#[result(u32)]
pub struct GetCounter;

#[derive(Actor, LocalActor)]
#[message(Ping)]
#[message(Pong)]
#[message(GetCounter)]
pub struct PingPongActor {
    counter: u32,
}

impl MessageHandler<Ping> for PingPongActor {
    async fn handle(&mut self, msg: Ping, ctx: &Self::ActorContext) -> <Ping as Message>::Result {
        let pong = Pong::new(ctx.self_addr().clone());
        if let Some(addr) = msg.addr.upgrade() {
            addr.dispatch(pong)
                .await
                .expect("Failed to dispatch message");
        }
        self.counter += 1;
    }
}

impl MessageHandler<Pong> for PingPongActor {
    async fn handle(&mut self, msg: Pong, ctx: &Self::ActorContext) -> <Pong as Message>::Result {
        let ping = Ping::new(ctx.self_addr().clone());
        if let Some(addr) = msg.addr.upgrade() {
            addr.dispatch(ping)
                .await
                .expect("Failed to dispatch message");
        }
        self.counter += 1;
    }
}

impl MessageHandler<GetCounter> for PingPongActor {
    async fn handle(
        &mut self,
        _msg: GetCounter,
        _ctx: &Self::ActorContext,
    ) -> <GetCounter as Message>::Result {
        self.counter
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct PingPongActorFactory;

impl ActorFactory for PingPongActorFactory {
    type Actor = PingPongActor;
}

impl LocalActorFactory for PingPongActorFactory {
    async fn create_actor(&self) -> PingPongActor {
        PingPongActor { counter: 0 }
    }
}
