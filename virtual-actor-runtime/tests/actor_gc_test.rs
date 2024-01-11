use std::time::Duration;

use virtual_actor_runtime::{prelude::*, GracefulShutdown, VirtualAddr};

use crate::actors::collectable_actor::{CollectableActor, GetCounter, Ping};

mod actors;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(6);

#[tokio::test]
async fn actor_gc_test() -> Result<(), Box<dyn std::error::Error>> {
    let gc_interval = Duration::from_millis(100);
    let idle = gc_interval * 5;
    let mut runtime = Runtime::with_preferences(RuntimePreferences {
        actor_idle_timeout: idle,
        garbage_collect_interval: gc_interval,
        ..Default::default()
    })?;
    let executor = runtime.create_executor()?;

    runtime.register_actor::<CollectableActor>(&executor)?;

    let id = 10;
    let addr: VirtualAddr<CollectableActor> = runtime.spawn_virtual(&id).await?;
    for _ in 0..10 {
        addr.send(Ping).await?;
    }

    let counter = addr.send(GetCounter).await?;
    assert_eq!(counter, 10, "Counter should be 10");

    tokio::time::sleep(idle + gc_interval * 2).await;

    let counter = addr.send(GetCounter).await?;
    assert_eq!(counter, 0, "Counter should be 0 after gc");

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
async fn actor_gc_refresh_test() -> Result<(), Box<dyn std::error::Error>> {
    let gc_interval = Duration::from_millis(100);
    let idle = gc_interval * 5;
    let mut runtime = Runtime::with_preferences(RuntimePreferences {
        actor_idle_timeout: idle,
        garbage_collect_interval: gc_interval,
        ..Default::default()
    })?;
    let executor = runtime.create_executor()?;

    runtime.register_actor::<CollectableActor>(&executor)?;

    let id = 10;
    let addr: VirtualAddr<CollectableActor> = runtime.spawn_virtual(&id).await?;
    for _ in 0..10 {
        addr.send(Ping).await?;
    }

    let counter = addr.send(GetCounter).await?;
    assert_eq!(counter, 10, "Counter should be 10");

    tokio::time::sleep(gc_interval * 4).await;
    addr.send(Ping).await?;
    tokio::time::sleep(gc_interval * 4).await;

    let counter = addr.send(GetCounter).await?;
    assert_eq!(counter, 11, "Counter should be 11 after refresh");

    tokio::time::sleep(gc_interval * 6).await;

    let counter = addr.send(GetCounter).await?;
    assert_eq!(counter, 0, "Counter should be 0 after gc");

    tokio::select! {
        biased;
        () = tokio::time::sleep(Duration::from_millis(100)) => {
            panic!("Runtime shutdown timeout")
        }
        e = runtime.graceful_shutdown(SHUTDOWN_TIMEOUT) => e
    }?;

    Ok(())
}
