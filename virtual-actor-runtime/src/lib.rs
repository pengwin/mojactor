//! Tokio based runtime for virtual-actor

mod actor_handle;
mod addr;
mod context_factory_trait;
mod executor;
mod message_dispatcher;
mod one_shot_responder;
mod runtime_context;
mod utils;

pub use addr::{Addr, WeakRef};
pub use executor::LocalExecutor;
pub use runtime_context::{RuntimeContext, RuntimeContextFactory};
pub use utils::waiter::WaitError;
pub use utils::GracefulShutdown;

pub mod prelude {
    //! Virtual actor prelude
    pub use virtual_actor::*;

    // Export derive macros
    pub use virtual_actor_derive::Actor;
    pub use virtual_actor_derive::Message;
    pub use virtual_actor_derive::VirtualActor;
    pub use virtual_actor_derive::VirtualMessage;
}


