use std::sync::Arc;

use dashmap::DashMap;
use futures::future::BoxFuture;
use virtual_actor_runtime::errors::BoxedActorError;

use super::actor_with_state_trait::ActorWithState;
use super::actor_persistence_trait::ActorPersistence;

/// Inmemory persistence error
#[derive(Debug, thiserror::Error)]
pub enum InmemoryPersistenceError {
    /// Failed to serialize id
    #[error("Failed to serialize id: {0}")]
    SerializeId(String),
    /// Failed to serialize state
    #[error("Failed to serialize state: {0}")]
    SerializeState(String),
    /// Failed to deserialize state
    #[error("Failed to deserialize state: {0}")]
    DeserializeState(String),
}

impl InmemoryPersistenceError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn failed_to_serialize_id(e: bincode::Error) -> BoxedActorError {
        BoxedActorError::new(Self::SerializeId(e.to_string()))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn failed_to_serialize_state(e: bincode::Error) -> BoxedActorError {
        BoxedActorError::new(Self::SerializeState(e.to_string()))
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn failed_to_deserialize_state(e: bincode::Error) -> BoxedActorError {
        BoxedActorError::new(Self::DeserializeState(e.to_string()))
    }
}

type ActorStateStorage = Arc<DashMap<Vec<u8>, Vec<u8>>>;

/// Inmemory actor state persistence
/// 
/// Test purposes implementation of `ActorPersistence` trait
pub struct InmemoryPersistence {
    storages: Arc<DashMap<String, ActorStateStorage>>,
}

impl InmemoryPersistence {
    /// Create a new inmemory persistence
    #[must_use]
    pub fn new() -> Self {
        Self {
            storages: Arc::new(DashMap::new()),
        }
    }
}

impl Default for InmemoryPersistence {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> ActorPersistence<A> for InmemoryPersistence
where
    A: ActorWithState,
{
    fn load(&self, id: &A::ActorId) -> BoxFuture<Result<Option<A::State>, BoxedActorError>> {
        let storage = self
            .storages
            .entry(A::name().to_string())
            .or_insert_with(|| Arc::new(DashMap::new()))
            .clone();

        let id_bytes = bincode::serialize(id);

        Box::pin(async move {
            let id_bytes = id_bytes.map_err(InmemoryPersistenceError::failed_to_serialize_id)?;
            let opt = storage.get(&id_bytes);

            match opt {
                Some(a) => match bincode::deserialize::<A::State>(&a) {
                    Ok(state) => Ok(Some(state)),
                    Err(e) => Err(InmemoryPersistenceError::failed_to_deserialize_state(e)),
                },
                None => Ok(None),
            }
        })
    }

    fn save(&self, id: &A::ActorId, state: &A::State) -> BoxFuture<Result<(), BoxedActorError>> {
        let storage = self
            .storages
            .entry(A::name().to_string())
            .or_insert_with(|| Arc::new(DashMap::new()))
            .clone();

        let id_bytes = bincode::serialize(id);
        let state_bytes = bincode::serialize(state);

        Box::pin(async move {
            let id_bytes = id_bytes.map_err(InmemoryPersistenceError::failed_to_serialize_id)?;
            let state_bytes =
                state_bytes.map_err(InmemoryPersistenceError::failed_to_serialize_state)?;
            storage.insert(id_bytes, state_bytes);
            Ok(())
        })
    }

    fn clear(&self, id: &A::ActorId) -> BoxFuture<Result<(), BoxedActorError>> {
        let storage = self
            .storages
            .entry(A::name().to_string())
            .or_insert_with(|| Arc::new(DashMap::new()))
            .clone();

        let id_bytes = bincode::serialize(id);

        Box::pin(async move {
            let id_bytes = id_bytes.map_err(InmemoryPersistenceError::failed_to_serialize_id)?;
            storage.remove(&id_bytes);
            Ok(())
        })
    }
}
