//! Tokio based runtime for virtual-actor

mod address;
mod context;
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
    pub use crate::address::errors::*;
    pub use crate::executor::errors::*;
    pub use crate::runtime::errors::*;
    pub use crate::utils::waiter::WaitError;

    pub use virtual_actor::errors::*;
}

pub mod prelude {
    //! Virtual actor prelude
    pub use virtual_actor::actor::*;
    pub use virtual_actor::local_actor::*;
    pub use virtual_actor::message::*;
    pub use virtual_actor::virtual_actor::*;

    pub use crate::runtime::Runtime;
    pub use crate::runtime::RuntimePreferences;

    // Export derive macros
    pub use virtual_actor_derive::Actor;
    pub use virtual_actor_derive::LocalActor;
    pub use virtual_actor_derive::Message;
    pub use virtual_actor_derive::VirtualActor;
    pub use virtual_actor_derive::VirtualMessage;
}
