use virtual_actor::{VirtualActor, WeakActorAddr};

use crate::runtime::{ActorActivator, WeakActorActivator};

use super::virtual_addr::VirtualAddr;

/// Weak reference to `VirtualAddr`
pub struct WeakVirtualAddr<A: VirtualActor> {
    id: A::ActorId,
    weak: WeakActorActivator<A>,
}

impl<A: VirtualActor> WeakVirtualAddr<A> {
    /// Creates new weak actor address
    pub(crate) fn new(id: &A::ActorId, activator: &ActorActivator<A>) -> Self {
        Self {
            id: id.clone(),
            weak: activator.weak_ref(),
        }
    }
}

impl<A: VirtualActor> Clone for WeakVirtualAddr<A> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            weak: self.weak.clone(),
        }
    }
}

impl<A: VirtualActor> WeakActorAddr<A> for WeakVirtualAddr<A> {
    type Upgraded = VirtualAddr<A>;

    fn upgrade(&self) -> Option<Self::Upgraded> {
        self.weak
            .upgrade()
            .map(|activator| VirtualAddr::new(&self.id, &activator))
    }
}
