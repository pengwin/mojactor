use virtual_actor_runtime::prelude::*;

#[derive(Message)]
#[result(Result<String, std::io::Error>)]
pub struct HelloMessage {
    msg: &'static str,
}

impl HelloMessage {
    pub fn new(msg: &'static str) -> Self {
        Self { msg }
    }
}

#[derive(Actor, LocalActor)]
#[message(HelloMessage)]
pub struct HelloActor {
    counter: u32,
}

impl HelloActor {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl MessageHandler<HelloMessage> for HelloActor {
    async fn handle(
        &mut self,
        msg: HelloMessage,
        _ctx: &Self::ActorContext,
    ) -> <HelloMessage as Message>::Result {
        self.counter += 1;
        let result = format!("Hello {} {}", msg.msg, self.counter);
        Ok(result.to_string())
    }
}

#[derive(Default)]
pub struct HelloActorFactory {}

impl ActorFactory<HelloActor> for HelloActorFactory {}

impl LocalActorFactory<HelloActor> for HelloActorFactory {
    async fn create_actor(&self) -> HelloActor {
        HelloActor::new()
    }
}
