mod default_virtual_actor_factory;
mod virtual_actor_constructor_trait;
mod virtual_actor_factory_trait;
mod virtual_actor_trait;
mod virtual_message_trait;

pub use default_virtual_actor_factory::DefaultVirtualActorFactory;
pub use virtual_actor_constructor_trait::VirtualActorConstructor;
pub use virtual_actor_factory_trait::VirtualActorFactory;
pub use virtual_actor_trait::VirtualActor;
pub use virtual_message_trait::VirtualMessage;
