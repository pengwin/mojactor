//! Tokio based runtime for virtual-actor

mod address;
mod context;
mod error_handling;
mod executor;
mod messaging;
mod runtime;
mod utils;

pub use address::{LocalAddr, VirtualAddr, WeakLocalAddr, WeakVirtualAddr};
pub use context::{RuntimeContext, RuntimeContextFactory};
pub use executor::{ExecutorPreferences, Handle as ExecutorHandle, TokioRuntimePreferences};
pub use utils::GracefulShutdown;

pub mod errors {
    //! Virtual actor errors
    pub use crate::address::{ActorStartError, LocalAddrError, VirtualAddrError};
    pub use crate::executor::{ActorTaskError, LocalExecutorError};
    pub use crate::runtime::ActorSpawnError;
    pub use crate::utils::waiter::WaitError;
}

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
