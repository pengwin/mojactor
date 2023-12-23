//! Test bench to run simple actors
#![allow(clippy::missing_docs_in_private_items)]

mod hello_actor;
mod hello_virtual_actor;
mod infinite_loop_actor;
mod ping_pong_actor;
mod ping_pong_virtual_actor;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::StreamExt;
use tokio_util::codec::{FramedRead, LinesCodec};
use virtual_actor_runtime::prelude::*;
use virtual_actor_runtime::{GracefulShutdown, WaitError};

use crate::{
    hello_actor::{HelloActorFactory, HelloMessage},
    hello_virtual_actor::{HelloVirtualActor, HelloVirtualActorFactory, HelloVirtualMessage},
    infinite_loop_actor::{InfiniteLoopActorFactory, PendingTask, ThreadSleepTask},
    ping_pong_actor::{Ping, PingPongActorFactory},
    ping_pong_virtual_actor::{
        VirtualGetCounter, VirtualPing, VirtualPingActor, VirtualPingActorFactory,
        VirtualPongActor, VirtualPongActorFactory,
    },
};

const SHUTDOWN_TIMEOUT: Duration = Duration::from_millis(10000);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //test_qwe().await?;
    bench_send_wait().await?;
    bench_spawn_wait_shutdown().await?;
    bench_same_thread_ping_pong().await?;
    bench_2_executors_ping_pong().await?;
    bench_infinite_loop_pending().await?;
    bench_virtual_actor_spawn_send_wait().await?;
    bench_virtual_ping_pong().await?;

    Ok(())
}

async fn bench_spawn_wait_shutdown() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_spawn_wait_shutdown");

    let actor_factory = Arc::new(HelloActorFactory::default());

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    let num_iteration = 10_000;
    let start = Instant::now();
    for _ in 0..num_iteration {
        let addr = runtime.spawn_local(&actor_factory, &executor).await?;
        let _res = addr.send(HelloMessage::new("world")).await??;
        addr.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!(
        "Per iteration: {elapsed:.2?}",
        elapsed = elapsed / num_iteration
    );

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}

async fn bench_send_wait() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_send_wait");

    let actor_factory = Arc::new(HelloActorFactory {});

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    let addr = runtime.spawn_local(&actor_factory, &executor).await?;

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

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}

#[allow(clippy::similar_names)]
async fn bench_same_thread_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_same_thread_ping_pong");

    let actor_factory = Arc::new(PingPongActorFactory {});

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    let ping_addr = runtime.spawn_local(&actor_factory, &executor).await?;
    let pong_addr = runtime.spawn_local(&actor_factory, &executor).await?;

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

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}

#[allow(clippy::similar_names)]
async fn bench_2_executors_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_2_executors_ping_pong");

    let actor_factory = Arc::new(PingPongActorFactory {});

    let mut runtime = Runtime::new()?;

    let executor_ping = runtime.create_executor()?;
    let executor_pong = runtime.create_executor()?;

    let runtime = Runtime::new()?;

    let ping_addr = runtime.spawn_local(&actor_factory, &executor_ping).await?;
    let pong_addr = runtime.spawn_local(&actor_factory, &executor_pong).await?;

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

    let timeout = SHUTDOWN_TIMEOUT;
    runtime.graceful_shutdown(timeout).await?;

    Ok(())
}

async fn bench_infinite_loop_pending() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_infinite_loop_pending");

    let actor_factory = Arc::new(InfiniteLoopActorFactory {});

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    let addr = runtime.spawn_local(&actor_factory, &executor).await?;

    addr.dispatch(PendingTask).await?;

    let duration = Duration::from_millis(10);
    tokio::time::sleep(duration).await;

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}

async fn bench_virtual_actor_spawn_send_wait() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_virtual_actor_spawn_send_wait");

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor(HelloVirtualActorFactory, &executor)?;

    let num_iteration: u32 = 10_000;
    let start = Instant::now();
    for id in 0..num_iteration {
        let addr = runtime.spawn_virtual::<HelloVirtualActor>(&id).await?;
        let _res = addr.send(HelloVirtualMessage::new("world")).await??;
    }
    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!(
        "Per iteration: {elapsed:.2?}",
        elapsed = elapsed / num_iteration
    );

    tokio::time::sleep(Duration::from_secs(10)).await;

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}

#[allow(clippy::similar_names)]
async fn bench_virtual_ping_pong() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_virtual_ping_pong");

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor(VirtualPingActorFactory, &executor)?;
    runtime.register_actor(VirtualPongActorFactory, &executor)?;

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

#[allow(dead_code)]
async fn bench_infinite_loop_thread_sleep() -> Result<(), Box<dyn std::error::Error>> {
    println!("bench_infinite_loop_thread_sleep");

    let actor_factory = Arc::new(InfiniteLoopActorFactory);

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    let addr = runtime.spawn_local(&actor_factory, &executor).await?;

    addr.dispatch(ThreadSleepTask).await?;

    let duration = Duration::from_millis(10);
    tokio::time::sleep(duration).await;

    match runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await {
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

#[allow(dead_code)]
async fn test_qwe() -> Result<(), Box<dyn std::error::Error>> {
    println!("test_qwe");

    let stdin = tokio::io::stdin();
    let mut reader = FramedRead::new(stdin, LinesCodec::new());

    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    runtime.register_actor(HelloVirtualActorFactory, &executor)?;

    let num_iteration: u32 = 10_000;
    let start = Instant::now();
    let id = 42;
    let addr = runtime.spawn_virtual::<HelloVirtualActor>(&id).await?;

    let mut line = reader.next().await.transpose()?;
    while let Some(l) = line {
        if l == "quit" {
            break;
        }
        let res = addr.send(HelloVirtualMessage::new(&l)).await??;
        println!("res: {res}");
        line = reader.next().await.transpose()?;
    }

    let elapsed = start.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!(
        "Per iteration: {elapsed:.2?}",
        elapsed = elapsed / num_iteration
    );

    tokio::time::sleep(Duration::from_secs(10)).await;

    runtime.graceful_shutdown(SHUTDOWN_TIMEOUT).await?;

    Ok(())
}
