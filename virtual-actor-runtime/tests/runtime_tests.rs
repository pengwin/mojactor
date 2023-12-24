use std::time::Duration;

use virtual_actor_runtime::{prelude::*, GracefulShutdown, VirtualAddr};

use crate::actors::ping_pong_virtual_actor::{
    VirtualGetCounter, VirtualPing, VirtualPingActor, VirtualPongActor,
};

mod actors;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(6);

#[allow(clippy::similar_names)]
#[tokio::test]
async fn bench_virtual_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor::<VirtualPingActor>(&executor)?;
    runtime.register_actor::<VirtualPongActor>(&executor)?;

    let num_iteration: u32 = 100;
    let id = 42;
    for _ in 0..num_iteration {
        let addr: VirtualAddr<VirtualPingActor> = runtime.spawn_virtual(&id).await?;
        addr.send(VirtualPing).await??;
    }

    let ping_addr: VirtualAddr<VirtualPingActor> = runtime.spawn_virtual(&id).await?;
    let ping_counter = ping_addr.send(VirtualGetCounter).await?;

    let pong_addr: VirtualAddr<VirtualPongActor> = runtime.spawn_virtual(&id).await?;
    let pong_counter = pong_addr.send(VirtualGetCounter).await?;

    assert_eq!(ping_counter, pong_counter);

    tokio::select! {
        biased;
        () = tokio::time::sleep(Duration::from_millis(100)) => {
            panic!("Runtime shutdown timeout")
        }
        e = runtime.graceful_shutdown(SHUTDOWN_TIMEOUT) => e
    }?;

    Ok(())
}
