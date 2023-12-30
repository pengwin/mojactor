use std::time::Duration;

use virtual_actor::errors::MessageProcessingError;
use virtual_actor_runtime::{
    errors::{
        ActorStartError, ActorTaskError, LocalAddrError, RuntimeSpawnError, VirtualAddrError,
    },
    prelude::*,
    GracefulShutdown, VirtualAddr,
};

use crate::actors::error_handling_virtual_actor::{
    ErrorHandlingActor, FactoryError, FactoryErrorActorFactory, PanicMessage, UnhandledMessage,
};

mod actors;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(6);

#[tokio::test]
async fn factory_error_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor_with_factory(FactoryErrorActorFactory, &executor)?;

    let id = "error_str".to_string();
    let addr: VirtualAddr<ErrorHandlingActor> = runtime.spawn_virtual(&id).await?;
    let send_res = addr.send(UnhandledMessage).await;

    match send_res {
        Ok(()) => panic!("Should not be Ok"),
        Err(e) => match e {
            VirtualAddrError::SpawnError(RuntimeSpawnError::ActorStartError(
                ActorStartError::ActorTaskError(ActorTaskError::ActorFactoryError(e)),
            )) => {
                if let Ok(e) = e.downcast::<FactoryError>() {
                    assert_eq!(e.message, id, "Error message should be equal");
                } else {
                    panic!("Should be FactoryError")
                }
            }
            _ => panic!("Should be SpawnError(ActorStartError)"),
        },
    }

    tokio::select! {
        biased;
        () = tokio::time::sleep(Duration::from_millis(100)) => {
            panic!("Runtime shutdown timeout")
        }
        e = runtime.graceful_shutdown(SHUTDOWN_TIMEOUT) => e
    }?;

    Ok(())
}

#[tokio::test]
async fn factory_panic_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor_with_factory(FactoryErrorActorFactory, &executor)?;

    let id = "panic_str".to_string();
    let addr: VirtualAddr<ErrorHandlingActor> = runtime.spawn_virtual(&id).await?;
    let send_res = addr.send(UnhandledMessage).await;

    match send_res {
        Ok(()) => panic!("Should not be Ok"),
        Err(e) => match e {
            VirtualAddrError::SpawnError(RuntimeSpawnError::ActorStartError(
                ActorStartError::ActorTaskError(ActorTaskError::ActorPanic(e)),
            )) => {
                assert_eq!(e, id, "Error message should be equal");
            }
            _ => panic!("Should be SpawnError(ActorStartError)"),
        },
    }

    tokio::select! {
        biased;
        () = tokio::time::sleep(Duration::from_millis(100)) => {
            panic!("Runtime shutdown timeout")
        }
        e = runtime.graceful_shutdown(SHUTDOWN_TIMEOUT) => e
    }?;

    Ok(())
}

#[tokio::test]
async fn message_panic_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor_with_factory(FactoryErrorActorFactory, &executor)?;

    let id = "message_panic".to_string();
    let addr: VirtualAddr<ErrorHandlingActor> = runtime.spawn_virtual(&id).await?;
    let send_res = addr
        .send(PanicMessage {
            message: id.clone(),
        })
        .await;

    match send_res {
        Ok(()) => panic!("Should not be Ok"),
        Err(err) => match err {
            VirtualAddrError::LocalAddrError(LocalAddrError::MessageProcessingError(
                MessageProcessingError::Panic(e),
            )) => {
                assert_eq!(e, id, "Error message should be equal");
            }
            _ => panic!("{err} Should be LocalAddrError::MessageProcessingError"),
        },
    }

    tokio::select! {
        biased;
        () = tokio::time::sleep(Duration::from_millis(100)) => {
            panic!("Runtime shutdown timeout")
        }
        e = runtime.graceful_shutdown(SHUTDOWN_TIMEOUT) => e
    }?;

    Ok(())
}
