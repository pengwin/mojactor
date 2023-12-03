//! This benchmark tests the performance of the `LocalExecutor` by spawning an actor and sending it a message.
#![allow(unused_must_use)]
#![allow(missing_docs)]

use std::sync::Arc;

use bench_actor::{BenchActorFactory, DispatchMessage, EchoMessage};
use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use virtual_actor::ActorAddr;
use virtual_actor_runtime::{LocalExecutor, RuntimeContextFactory};

#[allow(clippy::missing_errors_doc)]
pub fn criterion_benchmark(c: &mut Criterion) -> Result<(), Box<dyn std::error::Error>> {
    let mut executor = LocalExecutor::new()?;

    let actor_factory = Arc::new(BenchActorFactory {});
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let benchmark_runtime = Runtime::new()?;

    let actor = benchmark_runtime
        .block_on(async { executor.spawn_actor(&actor_factory, &context_factory).await })?;

    let addr = actor.addr();

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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/// This module contains the actor and actor factory used for the benchmark.
mod bench_actor {
    use virtual_actor_runtime::prelude::*;

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

    #[derive(Actor)]
    #[message(EchoMessage)]
    #[message(DispatchMessage)]
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

    #[derive(Default)]
    pub struct BenchActorFactory {}

    impl ActorFactory<BenchActor> for BenchActorFactory {
        async fn create_actor(&self) -> BenchActor {
            BenchActor
        }
    }
}
