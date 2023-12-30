//! Module with local actor traits and types

mod default_local_actor_factory;
mod local_actor_constructor_trait;
mod local_actor_factory_trait;
mod local_actor_trait;

pub use default_local_actor_factory::DefaultLocalActorFactory;
pub use local_actor_constructor_trait::LocalActorConstructor;
pub use local_actor_factory_trait::LocalActorFactory;
pub use local_actor_trait::LocalActor;
