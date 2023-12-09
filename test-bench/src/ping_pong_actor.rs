use virtual_actor_runtime::prelude::*;
use virtual_actor_runtime::WeakRef;

#[derive(Message)]
#[result(())]
pub struct Ping {
    addr: WeakRef<PingPongActor>,
}

impl Ping {
    pub fn new(addr: WeakRef<PingPongActor>) -> Self {
        Self { addr }
    }
}

#[derive(Message)]
#[result(())]
pub struct Pong {
    addr: WeakRef<PingPongActor>,
}

impl Pong {
    pub fn new(addr: WeakRef<PingPongActor>) -> Self {
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
        let pong = Pong::new(ctx.self_addr().weak_ref());
        if let Some(addr) = msg.addr.upgrade() {
            addr.dispatch(pong).expect("Failed to dispatch message");
        }
        self.counter += 1;
    }
}

impl MessageHandler<Pong> for PingPongActor {
    async fn handle(&mut self, msg: Pong, ctx: &Self::ActorContext) -> <Pong as Message>::Result {
        let ping = Ping::new(ctx.self_addr().weak_ref());
        if let Some(addr) = msg.addr.upgrade() {
            addr.dispatch(ping).expect("Failed to dispatch message");
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
