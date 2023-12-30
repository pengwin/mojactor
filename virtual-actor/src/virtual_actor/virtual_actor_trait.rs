//! Virtual actor trait

use crate::{Actor, ActorId};

/// Virtual Actor trait
/// Actor instance can be reached by name through network
/// using pair `name` and `id` of the actor
pub trait VirtualActor: Actor + 'static {
    /// Type of actor id
    type ActorId: ActorId;

    /// Id of the actor
    fn id(&self) -> &Self::ActorId;
}
