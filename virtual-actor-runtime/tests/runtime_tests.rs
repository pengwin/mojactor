use std::time::{Duration, Instant};

use virtual_actor_runtime::{prelude::*, GracefulShutdown};

use crate::actors::ping_pong_virtual_actor::{
    VirtualGetCounter, VirtualPing, VirtualPingActor, VirtualPongActor,
};

mod actors;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_millis(10000);

#[allow(clippy::similar_names)]
#[tokio::test]
async fn bench_virtual_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor::<VirtualPingActor>(&executor)?;
    runtime.register_actor::<VirtualPongActor>(&executor)?;

    let num_iteration: u32 = 10_000;
    let id = 42;
    let start = Instant::now();
    for _ in 0..num_iteration {
        let addr = runtime.spawn_virtual::<VirtualPingActor>(&id).await?;
        addr.send(VirtualPing).await??;
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!(
        "Per iteration: {elapsed:.2?}",
        elapsed = elapsed / num_iteration
    );

    let ping_addr = runtime.spawn_virtual::<VirtualPingActor>(&id).await?;
    let ping_counter = ping_addr.send(VirtualGetCounter).await?;
    let ping_duration = elapsed / ping_counter as u32;

    println!("Ping counter: {ping_counter} Average time for message {ping_duration:.2?}");

    let pong_addr = runtime.spawn_virtual::<VirtualPongActor>(&id).await?;
    let pong_counter = pong_addr.send(VirtualGetCounter).await?;
    let pong_duration = elapsed / pong_counter as u32;

    println!("Pong counter: {pong_counter} Average time for message {pong_duration:.2?}");

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}
