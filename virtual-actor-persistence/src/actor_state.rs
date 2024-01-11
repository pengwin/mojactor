//! State trait

use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use virtual_actor_runtime::errors::BoxedActorError;

use super::actor_with_state_trait::ActorWithState;
use super::actor_persistence_trait::ActorPersistence;

/// Container for actor state
pub struct ActorState<A>
where
    A: ActorWithState,
{
    actor_id: A::ActorId,
    persistence: Arc<dyn ActorPersistence<A>>,
    state: A::State,
}

impl<A> ActorState<A>
where
    A: ActorWithState,
{
    /// Create a new actor state
    ///
    /// # Errors
    ///
    /// Returns error from persistence layer
    pub async fn load(
        persistence: &Arc<dyn ActorPersistence<A>>,
        id: &A::ActorId,
    ) -> Result<Self, BoxedActorError> {
        let state = persistence.load(id).await?.unwrap_or_default();
        Ok(Self {
            actor_id: id.clone(),
            persistence: persistence.clone(),
            state,
        })
    }

    /// Save state
    ///
    /// # Errors
    ///
    /// Returns error from persistence layer
    pub async fn save(&self) -> Result<(), BoxedActorError> {
        self.persistence.save(&self.actor_id, &self.state).await
    }

    /// Clear state
    ///
    /// # Errors
    ///
    /// Returns error from persistence layer
    pub async fn clear(&self) -> Result<(), BoxedActorError> {
        self.persistence.clear(&self.actor_id).await
    }
}

impl<A> Deref for ActorState<A>
where
    A: ActorWithState,
{
    type Target = A::State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<A> DerefMut for ActorState<A>
where
    A: ActorWithState,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
