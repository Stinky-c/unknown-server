pub use kameo_actors::pool::{Broadcast, Dispatch};
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Greet(pub String);
impl Message for Greet {}

/// Adds `left` and `right` together
#[cfg_attr(feature = "server", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Add(pub u32, pub u32);
impl Message for Add {}

#[derive(Debug, Clone)]
pub struct Shutdown;
impl Message for Shutdown {}

pub trait Message {}
