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

#[derive(Actor, LocalActor, Default)]
#[message(HelloMessage)]
pub struct HelloActor {
    counter: u32,
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
