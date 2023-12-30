use crate::actor::Actor;

/// Marker trait for local actors
pub trait LocalActor: Actor + 'static {}
