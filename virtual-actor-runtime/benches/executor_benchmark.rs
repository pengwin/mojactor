//! This benchmark tests the performance of the `LocalExecutor` by spawning an actor and sending it a message.
#![allow(unused_must_use)]
#![allow(missing_docs)]
#![allow(clippy::missing_errors_doc)]

use bench_actor::{AksMessage, BenchActor, DispatchMessage, EchoMessage};
use criterion::{criterion_group, criterion_main, Criterion};
use virtual_actor::ActorAddr;
use virtual_actor_runtime::{
    prelude::Runtime, ExecutorHandle, ExecutorPreferences, LocalAddr, TokioRuntimePreferences,
};

fn create_runtime() -> Result<tokio::runtime::Runtime, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(100)
        .enable_time()
        .build()?;

    Ok(rt)
}

fn create_executor(runtime: &mut Runtime) -> Result<ExecutorHandle, Box<dyn std::error::Error>> {
    let executor = runtime.create_executor_with_preferences(&ExecutorPreferences {
        tokio_runtime_preferences: TokioRuntimePreferences {
            enable_io: false,
            enable_time: false,
        },
        thread_name: "benchmark".to_string(),
        ..Default::default()
    })?;

    Ok(executor)
}

pub fn messaging_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    let benchmark_runtime = create_runtime()?;

    let mut runtime = Runtime::new()?;
    let executor = create_executor(&mut runtime)?;

    let addr: LocalAddr<BenchActor> =
        benchmark_runtime.block_on(async { runtime.spawn_local(&executor).await })?;

    c.bench_function("send_wait", |b| {
        b.to_async(&benchmark_runtime).iter(|| async {
            if let Err(e) = addr.send(EchoMessage::new("test")).await {
                println!("Error: {e:?}");
            }
        });
    });

    c.bench_function("dispatch", |b| {
        b.to_async(&benchmark_runtime).iter(|| async {
            if let Err(e) = addr.send(DispatchMessage).await {
                println!("Error: {e:?}");
            }
        });
    });

    Ok(())
}

pub fn inter_thread_messaging_benchmark(
    c: &mut Criterion,
) -> Result<(), Box<dyn std::error::Error>> {
    let benchmark_runtime = create_runtime()?;
    let mut runtime = Runtime::new()?;

    let executor_1 = create_executor(&mut runtime)?;
    let executor_2 = create_executor(&mut runtime)?;

    let addr_1_1: LocalAddr<BenchActor> =
        benchmark_runtime.block_on(async { runtime.spawn_local(&executor_1).await })?;

    let addr_1_2: LocalAddr<BenchActor> =
        benchmark_runtime.block_on(async { runtime.spawn_local(&executor_1).await })?;

    let addr_2_2: LocalAddr<BenchActor> =
        benchmark_runtime.block_on(async { runtime.spawn_local(&executor_2).await })?;

    c.bench_function("single thread communication", |b| {
        b.to_async(&benchmark_runtime).iter(|| async {
            if let Err(e) = addr_1_1.send(AksMessage::new(addr_1_2.weak_ref())).await {
                println!("Error: {e:?}");
            }
        });
    });

    c.bench_function("inter thread communication", |b| {
        b.to_async(&benchmark_runtime).iter(|| async {
            if let Err(e) = addr_1_1.send(AksMessage::new(addr_2_2.weak_ref())).await {
                println!("Error: {e:?}");
            }
        });
    });

    Ok(())
}

criterion_group!(
    benches,
    messaging_benchmark,
    inter_thread_messaging_benchmark
);
criterion_main!(benches);

/// This module contains the actor and actor factory used for the benchmark.
mod bench_actor {
    use virtual_actor_runtime::{prelude::*, WeakLocalAddr};

    #[derive(Message)]
    #[result(())]
    pub struct AksMessage {
        recipient: WeakLocalAddr<BenchActor>,
    }

    impl AksMessage {
        pub fn new(recipient: WeakLocalAddr<BenchActor>) -> Self {
            Self { recipient }
        }
    }

    #[derive(Message)]
    #[result(&'static str)]
    pub struct EchoMessage {
        msg: &'static str,
    }

    impl EchoMessage {
        pub fn new(msg: &'static str) -> Self {
            Self { msg }
        }
    }

    #[derive(Message)]
    #[result(())]
    pub struct DispatchMessage;

    #[derive(Actor, LocalActor, Default)]
    #[message(EchoMessage)]
    #[message(DispatchMessage)]
    #[message(AksMessage)]
    pub struct BenchActor;

    impl MessageHandler<EchoMessage> for BenchActor {
        async fn handle(
            &mut self,
            msg: EchoMessage,
            _ctx: &Self::ActorContext,
        ) -> <EchoMessage as Message>::Result {
            msg.msg
        }
    }

    impl MessageHandler<DispatchMessage> for BenchActor {
        async fn handle(
            &mut self,
            _msg: DispatchMessage,
            _ctx: &Self::ActorContext,
        ) -> <DispatchMessage as Message>::Result {
        }
    }

    impl MessageHandler<AksMessage> for BenchActor {
        async fn handle(
            &mut self,
            msg: AksMessage,
            _ctx: &Self::ActorContext,
        ) -> <AksMessage as Message>::Result {
            let r = msg.recipient.upgrade();
            if let Some(r) = r {
                if let Err(e) = r.send(EchoMessage::new("test")).await {
                    println!("Error: {e:?}");
                }
            }
        }
    }
}
