pub mod actor;
pub mod error;
pub mod message;
pub mod pool;

pub static NAME: &str = "unknown-actor";

#[allow(unused_imports)]
pub mod prelude {
    pub use super::NAME;
    pub use crate::actor::ServerActor;
    pub use crate::message::*;
    pub use crate::pool::{ActorPoolRef, Broadcast, Dispatch, pool};
}
