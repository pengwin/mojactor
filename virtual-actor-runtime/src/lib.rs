//! Tokio based runtime for virtual-actor

mod address;
mod context;
mod executor;
mod messaging;
mod runtime;
mod utils;

pub use address::{Addr, WeakRef};
pub use context::{RuntimeContext, RuntimeContextFactory};
pub use executor::{ExecutorPreferences, LocalExecutor, TokioRuntimePreferences};
pub use utils::waiter::WaitError;
pub use utils::GracefulShutdown;

pub mod prelude {
    //! Virtual actor prelude
    pub use virtual_actor::*;

    pub use crate::runtime::Runtime;

    // Export derive macros
    pub use virtual_actor_derive::Actor;
    pub use virtual_actor_derive::LocalActor;
    pub use virtual_actor_derive::Message;
    pub use virtual_actor_derive::VirtualActor;
    pub use virtual_actor_derive::VirtualMessage;
}
