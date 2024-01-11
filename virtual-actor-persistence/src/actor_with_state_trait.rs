use serde::{de::DeserializeOwned, Serialize};
use virtual_actor_runtime::prelude::VirtualActor;

use super::actor_state::ActorState;

/// Actor with state trait
pub trait ActorWithState: VirtualActor {
    /// Type of actor state
    type State: Serialize + DeserializeOwned + Default;

    /// Accessor for actor state
    fn state(&self) -> &ActorState<Self>;
}
