use std::time::Duration;

use virtual_actor::{ActorAddr, ActorContext, Message, MessageHandler, VirtualActor, WeakActorRef};

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
            let (id, handle) = e.pair();
            let last_received = handle.last_received_msg_timestamp().elapsed();
            let last_processed = handle.last_processed_msg_timestamp().elapsed();
            let is_idle =
                last_processed < last_received && last_processed > self.actor_idle_timeout;
            if handle.is_finished() {
                println!("Actor {id} is finished");
                self.cache.remove(id);
                continue;
            }
            if is_idle {
                idle_actors.push(id.clone());
            }
        }

        let actor_name = A::name();

        // remove idle actors
        for id in idle_actors {
            println!("Shutting down actor {actor_name}::{id}");
            if let Some(handle) = self.cache.get(&id) {
                let shutdown = handle.graceful_shutdown(Duration::from_millis(100)).await;
                if let Err(e) = shutdown {
                    eprintln!("Failed to gracefully shutdown actor {id}: {e:?}");
                }
                
                println!("Actor {actor_name}::{id} is idle and has been successfully shutdown");
            }

            self.cache.remove(&id);
        }

        // schedule next garbage collection
        let addr = ctx.self_addr().weak_ref();
        let interval = self.interval;
        tokio::task::spawn_local(async move {
            tokio::time::sleep(interval).await;
            if let Some(addr) = addr.upgrade() {
                addr.dispatch(GarbageCollectActors)
                    .expect("Failed to dispatch message");
            };
        });
    }
}
