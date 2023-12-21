use virtual_actor::{
    ActorAddr, ActorContext, Message, MessageHandler, VirtualActor, WeakActorAddr,
};

use crate::GracefulShutdown;

use super::HousekeepingActor;

#[derive(Debug)]
pub struct GarbageCollectActors;

impl Message for GarbageCollectActors {
    type Result = ();
}

impl<A: VirtualActor> MessageHandler<GarbageCollectActors> for HousekeepingActor<A> {
    async fn handle(
        &mut self,
        _msg: GarbageCollectActors,
        ctx: &Self::ActorContext,
    ) -> <GarbageCollectActors as Message>::Result {
        let mut idle_actors = Vec::new();
        // find all finished or idle actors
        for e in self.cache.iter() {
            let (actor_id, handle) = e.pair();
            self.actor_counters.update(actor_id, handle);
            let is_idle = self
                .actor_counters
                .is_idle(actor_id, self.preferences.actor_idle_timeout);
            if handle.is_finished() {
                println!("Actor {actor_id} is finished");
                self.cache.remove(actor_id);
                continue;
            }
            if is_idle {
                idle_actors.push(actor_id.clone());
            }
        }

        let actor_name = A::name();

        // remove idle actors
        for actor_id in idle_actors {
            println!("Shutting down actor {actor_name}::{actor_id}");
            let handle = self.cache.remove(&actor_id);
            if let Some(handle) = handle {
                let shutdown = handle
                    .graceful_shutdown(self.preferences.actor_shutdown_interval)
                    .await;
                if let Err(e) = shutdown {
                    eprintln!("Failed to gracefully shutdown actor {actor_id}: {e:?}");
                }

                println!(
                    "Actor {actor_name}::{actor_id} is idle and has been successfully shutdown"
                );
            }

            self.cache.remove(&actor_id);
            self.actor_counters.remove(&actor_id);
        }

        // schedule next garbage collection
        let addr = ctx.self_addr().clone();
        let interval = self.preferences.garbage_collect_interval;
        tokio::task::spawn_local(async move {
            tokio::time::sleep(interval).await;
            if let Some(addr) = addr.upgrade() {
                addr.dispatch(GarbageCollectActors)
                    .await
                    .expect("Failed to dispatch message");
            } else {
                println!("Housekeeping stopped");
            }
        });
    }
}
