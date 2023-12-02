mod actor_loop;
mod error;
mod factory;
mod handle;
mod local_actor_impl;
mod local_actor_trait;
mod mailbox;

pub use factory::create;
pub use handle::{ActorId, LocalActorHandle};
pub use local_actor_trait::LocalActor;
