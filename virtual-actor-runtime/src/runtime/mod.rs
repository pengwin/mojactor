mod registry;
mod runtime_impl;
mod runtime_preferences;

pub use registry::WeakActorRegistry;
pub use registry::{ActorActivator, WeakActorActivator};
pub use runtime_impl::Runtime;

pub mod errors {
    pub use super::registry::errors::*;
}
