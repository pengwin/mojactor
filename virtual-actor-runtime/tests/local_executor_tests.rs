use std::{sync::Arc, thread::ThreadId};

use virtual_actor_runtime::{prelude::*, LocalExecutor, RuntimeContextFactory};

#[derive(Message)]
#[result(ThreadId)]
pub struct GetThreadId;

#[derive(Actor, LocalActor)]
#[message(GetThreadId)]
pub struct TestActor;

pub struct TestActorFactory;

impl ActorFactory<TestActor> for TestActorFactory {}

impl LocalActorFactory<TestActor> for TestActorFactory {
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
    let mut executor = LocalExecutor::new()?;

    let actor_factory = Arc::new(TestActorFactory);
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let actor_one = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let actor_two = executor
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

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
    let mut executor_one = LocalExecutor::new()?;
    let mut executor_two = LocalExecutor::new()?;

    let actor_factory = Arc::new(TestActorFactory);
    let context_factory = Arc::new(RuntimeContextFactory::default());

    let actor_one = executor_one
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

    let actor_two = executor_two
        .spawn_local_actor(&actor_factory, &context_factory)
        .await?;

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
