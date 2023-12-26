use serde::Deserialize;
use serde::Serialize;
use virtual_actor_runtime::prelude::*;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(())]
pub struct UnhandledMessage;

#[derive(Message, VirtualMessage, Serialize, Deserialize)]
#[result(())]
pub struct PanicMessage {
    pub message: String,
}

#[derive(Actor, VirtualActor)]
#[message(UnhandledMessage)]
#[message(PanicMessage)]
pub struct ErrorHandlingActor {
    id: String,
}

impl MessageHandler<UnhandledMessage> for ErrorHandlingActor {
    async fn handle(
        &mut self,
        _msg: UnhandledMessage,
        _ctx: &Self::ActorContext,
    ) -> <UnhandledMessage as Message>::Result {
    }
}

impl MessageHandler<PanicMessage> for ErrorHandlingActor {
    async fn handle(
        &mut self,
        msg: PanicMessage,
        _ctx: &Self::ActorContext,
    ) -> <PanicMessage as Message>::Result {
        panic!("{}", msg.message);
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Factory error {message}")]
pub struct FactoryError {
    pub message: String,
}

pub struct FactoryErrorActorFactory;

impl ActorFactory for FactoryErrorActorFactory {
    type Actor = ErrorHandlingActor;
}

impl VirtualActorFactory for FactoryErrorActorFactory {
    type Error = FactoryError;

    async fn create_actor(&self, id: &String) -> Result<ErrorHandlingActor, Self::Error> {
        match id.as_str() {
            "panic_str" => {
                panic!("panic_str");
            }
            "panic_string" => {
                let panic_string = "panic_string".to_owned();
                panic!("{}", panic_string);
            }
            "error_str" => {
                Err(FactoryError {
                    message: id.clone(),
                })?;
            }
            _ => {}
        };

        Ok(ErrorHandlingActor { id: id.clone() })
    }
}
