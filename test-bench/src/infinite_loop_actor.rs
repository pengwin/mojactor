use std::time::Duration;

use virtual_actor_runtime::prelude::*;

#[derive(Message)]
#[result(())]
pub struct PendingTask;

#[derive(Message)]
#[result(())]
pub struct ThreadSleepTask;

#[derive(Actor, LocalActor, Default)]
#[message(PendingTask)]
#[message(ThreadSleepTask)]
pub struct InfiniteLoopActor;

impl MessageHandler<PendingTask> for InfiniteLoopActor {
    async fn handle(
        &mut self,
        _msg: PendingTask,
        _ctx: &Self::ActorContext,
    ) -> <PendingTask as Message>::Result {
        futures::future::pending::<()>().await;
    }
}

impl MessageHandler<ThreadSleepTask> for InfiniteLoopActor {
    async fn handle(
        &mut self,
        _msg: ThreadSleepTask,
        _ctx: &Self::ActorContext,
    ) -> <ThreadSleepTask as Message>::Result {
        std::thread::sleep(Duration::from_secs(3600));
    }
}
