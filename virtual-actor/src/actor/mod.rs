//! Module with actor traits and types

mod actor_addr;
mod actor_context_trait;
mod actor_factory_trait;
mod actor_id_trait;
mod actor_name;
mod actor_trait;
mod weak_actor_add_trait;

pub use actor_addr::ActorAddr;
pub use actor_context_trait::ActorContext;
pub use actor_factory_trait::ActorFactory;
pub use actor_id_trait::ActorId;
pub use actor_name::ActorName;
pub use actor_trait::Actor;
pub use weak_actor_add_trait::WeakActorAddr;
