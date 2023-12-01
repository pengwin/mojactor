//! Virtual actor trait

use crate::actor_id_trait::ActorId;

use super::actor_trait::Actor;
use super::names::ActorName;

/// Virtual Actor trait
/// Actor instance can be reached by name through network
/// using pair `name` and `id` of the actor
pub trait VirtualActor: Actor {
    /// Type of actor id
    type ActorId: ActorId;

    /// Id of the actor
    fn id(&self) -> &Self::ActorId;

    /// Name of the actor
    fn name() -> ActorName;
}
