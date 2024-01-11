//! # Virtual Actor Persistence library

mod actor_state;
mod actor_with_state_trait;
mod actor_persistence_trait;
mod inmemory_persistence;

pub mod prelude {
    //! Virtual actor persistence prelude
    
    pub use super::actor_state::ActorState;
    pub use super::actor_with_state_trait::ActorWithState;
    pub use super::actor_persistence_trait::ActorPersistence;
}

pub use inmemory_persistence::InmemoryPersistence;