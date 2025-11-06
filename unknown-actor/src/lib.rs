pub mod actor;
pub mod error;
pub mod message;
#[cfg(feature = "pool")]
pub mod pool;

pub static NAME: &str = "unknown-actor";

#[allow(unused_imports)]
pub mod prelude {
    pub use super::NAME;
    pub use crate::actor::ServerActor;
    pub use crate::message::{Add, Greet};

    // Server only
    #[cfg(feature = "server")]
    pub use crate::message::Shutdown;

    // Pool only
    #[cfg(feature = "pool")]
    pub use crate::pool::{ActorPoolRef, Broadcast, Dispatch, pool};
}
