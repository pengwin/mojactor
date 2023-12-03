//! Test bench to run simple actors
#![allow(clippy::missing_docs_in_private_items)]

mod hello_actor;
mod infinite_loop_actor;
mod ping_pong_actor;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use virtual_actor_runtime::prelude::*;
use virtual_actor_runtime::{GracefulShutdown, WaitError};
use virtual_actor_runtime::{LocalExecutor, RuntimeContextFactory};

use crate::{
    hello_actor::{HelloActorFactory, HelloMessage},
    infinite_loop_actor::{InfiniteLoopActorFactory, PendingTask, ThreadSleepTask},
    ping_pong_actor::{Ping, PingPongActorFactory},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bench_send_wait().await?;
    bench_spawn_wait_shutdown().await?;
    bench_same_thread_ping_pong().await?;
    bench_2_executors_ping_pong().await?;
    bench_infinite_loop_pending().await?;
    bench_infinite_loop_thread_sleep().await?;

    Ok(())
}

async fn bench_spawn_wait_shutdown() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_spawn_wait_shutdown");

    let actor_factory = Arc::new(HelloActorFactory::default());
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let mut executor = LocalExecutor::new()?;

    let num_iteration = 10_000;
    let start = Instant::now();
    for _ in 0..num_iteration {
        let actor = executor
            .spawn_local_actor(&actor_factory, &context_factory)
            .await?;
        let addr = actor.addr();
        let _res = addr.send(HelloMessage::new("world")).await??;
        actor.graceful_shutdown(Duration::from_millis(100)).await?;
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!(
        "Per iteration: {elapsed:.2?}",
        elapsed = elapsed / num_iteration
    );

    executor
        .graceful_shutdown(Duration::from_millis(100))
        .await?;

    Ok(())
}

async fn bench_send_wait() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_send_wait");

    let actor_factory = Arc::new(HelloActorFactory {});
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let mut executor = LocalExecutor::new()?;

    let actor = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let addr = actor.addr();

    let num_iteration = 10_000;
    let start = Instant::now();
    for _ in 0..num_iteration {
        let _res = addr.send(HelloMessage::new("world")).await??;
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!(
        "Per iteration: {elapsed:.2?}",
        elapsed = elapsed / num_iteration
    );

    executor
        .graceful_shutdown(Duration::from_millis(100))
        .await?;

    Ok(())
}

#[allow(clippy::similar_names)]
async fn bench_same_thread_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_same_thread_ping_pong");

    let actor_factory = Arc::new(PingPongActorFactory {});
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let mut executor = LocalExecutor::new()?;

    let ping = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let pong = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let ping_addr = ping.addr();
    let pong_addr = pong.addr();

    ping_addr.send(Ping::new(pong_addr.weak_ref())).await?;
    pong_addr.send(Ping::new(ping_addr.weak_ref())).await?;

    let duration = Duration::from_secs(1);
    tokio::time::sleep(duration).await;

    let ping_counter = ping_addr.send(ping_pong_actor::GetCounter).await?;
    let pong_counter = pong_addr.send(ping_pong_actor::GetCounter).await?;

    let ping_duration = duration / ping_counter as u32;
    let pong_duration = duration / pong_counter as u32;

    println!("Ping counter: {ping_counter} Average time for message {ping_duration:.2?}");
    println!("Pong counter: {pong_counter} Average time for message {pong_duration:.2?}");

    executor
        .graceful_shutdown(Duration::from_millis(100))
        .await?;

    Ok(())
}

#[allow(clippy::similar_names)]
async fn bench_2_executors_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_2_executors_ping_pong");

    let actor_factory = Arc::new(PingPongActorFactory {});
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let mut executor_ping = LocalExecutor::new()?;
    let mut executor_pong = LocalExecutor::new()?;

    let ping = executor_ping
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let pong = executor_pong
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let ping_addr = ping.addr();
    let pong_addr = pong.addr();

    ping_addr.send(Ping::new(pong_addr.weak_ref())).await?;
    pong_addr.send(Ping::new(ping_addr.weak_ref())).await?;

    let duration = Duration::from_secs(1);
    tokio::time::sleep(duration).await;

    let ping_counter = ping_addr.send(ping_pong_actor::GetCounter).await?;
    let pong_counter = pong_addr.send(ping_pong_actor::GetCounter).await?;

    let ping_duration = duration / ping_counter as u32;
    let pong_duration = duration / pong_counter as u32;

    println!("Ping counter: {ping_counter} Average time for message {ping_duration:.2?}");
    println!("Pong counter: {pong_counter} Average time for message {pong_duration:.2?}");

    let timeout = Duration::from_millis(100);
    tokio::try_join!(
        executor_ping.graceful_shutdown(timeout),
        executor_pong.graceful_shutdown(timeout)
    )?;

    Ok(())
}

async fn bench_infinite_loop_pending() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_infinite_loop_pending");

    let actor_factory = Arc::new(InfiniteLoopActorFactory {});
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let mut executor = LocalExecutor::new()?;

    let actor = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let addr = actor.addr();

    addr.dispatch(PendingTask)?;

    let duration = Duration::from_millis(10);
    tokio::time::sleep(duration).await;

    executor
        .graceful_shutdown(Duration::from_millis(100))
        .await?;

    Ok(())
}

async fn bench_infinite_loop_thread_sleep() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_infinite_loop_thread_sleep");

    let actor_factory = Arc::new(InfiniteLoopActorFactory);
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let mut executor = LocalExecutor::new()?;

    let actor = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let addr = actor.addr();

    addr.dispatch(ThreadSleepTask)?;

    let duration = Duration::from_millis(10);
    tokio::time::sleep(duration).await;

    match executor.graceful_shutdown(Duration::from_millis(100)).await {
        Err(WaitError::Timeout(s)) => {
            if s != "LocalSpawner" {
                return Err(WaitError::Timeout(s).into());
            }
            Ok(())
        }
        r => r,
    }?;

    Ok(())
}
