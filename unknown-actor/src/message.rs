pub use kameo_actors::pool::{Broadcast, Dispatch};

#[derive(Debug, Clone)]
pub struct Greet(pub String);
impl Message for Greet {}

/// Adds `left` and `right` together
#[derive(Debug, Clone)]
pub struct Add(pub u32, pub u32);
impl Message for Add {}

#[derive(Debug, Clone)]
pub struct Shutdown;
impl Message for Shutdown {}

pub trait Message {}
