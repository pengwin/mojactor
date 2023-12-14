use std::{sync::Arc, thread::ThreadId};

use virtual_actor_runtime::prelude::*;

#[derive(Message)]
#[result(ThreadId)]
pub struct GetThreadId;

#[derive(Actor, LocalActor)]
#[message(GetThreadId)]
pub struct TestActor;

pub struct TestActorFactory;

impl ActorFactory for TestActorFactory {
    type Actor = TestActor;
}

impl LocalActorFactory for TestActorFactory {
    async fn create_actor(&self) -> TestActor {
        TestActor
    }
}

impl MessageHandler<GetThreadId> for TestActor {
    async fn handle(
        &mut self,
        _msg: GetThreadId,
        _ctx: &Self::ActorContext,
    ) -> <GetThreadId as Message>::Result {
        std::thread::current().id()
    }
}

#[tokio::test]
async fn test_local_executor_actor_threads_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor = runtime.create_executor()?;

    let actor_factory = Arc::new(TestActorFactory);

    let actor_one = runtime.spawn_local(&actor_factory, &executor).await?;

    let actor_two = runtime.spawn_local(&actor_factory, &executor).await?;

    let current_thread_id = std::thread::current().id();

    let actor_one_thread_id = actor_one.addr().send(GetThreadId).await?;
    let actor_two_thread_id = actor_two.addr().send(GetThreadId).await?;

    assert_ne!(
        current_thread_id, actor_one_thread_id,
        "Actor threads and current thread must be different"
    );
    assert_ne!(
        current_thread_id, actor_two_thread_id,
        "Actor threads and current thread must be different"
    );
    assert_eq!(
        actor_one_thread_id, actor_two_thread_id,
        "Actor threads must be equal"
    );

    Ok(())
}

#[tokio::test]
async fn test_local_executors_threads_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = Runtime::new()?;
    let executor_one = runtime.create_executor()?;
    let executor_two = runtime.create_executor()?;

    let actor_factory = Arc::new(TestActorFactory);

    let actor_one = runtime.spawn_local(&actor_factory, &executor_one).await?;

    let actor_two = runtime.spawn_local(&actor_factory, &executor_two).await?;

    let current_thread_id = std::thread::current().id();

    let actor_one_thread_id = actor_one.addr().send(GetThreadId).await?;
    let actor_two_thread_id = actor_two.addr().send(GetThreadId).await?;

    assert_ne!(
        current_thread_id, actor_one_thread_id,
        "Actor threads and current thread must be different"
    );
    assert_ne!(
        current_thread_id, actor_two_thread_id,
        "Actor threads and current thread must be different"
    );
    assert_ne!(
        actor_one_thread_id, actor_two_thread_id,
        "Actor threads of different executors must be different"
    );

    Ok(())
}
