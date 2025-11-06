use crate::actor::ServerActor;
use kameo::actor::ActorRef;
use kameo::{Actor, mailbox};
use kameo_actors::pool::ActorPool;

pub use kameo_actors::pool::{Broadcast, Dispatch};

pub type ActorPoolRef = ActorRef<ActorPool<ServerActor>>;

pub async fn pool(
    pool_size: Option<usize>,
    mailbox_size: Option<usize>,
) -> Result<ActorPoolRef, Box<dyn std::error::Error>> {
    let pool = ActorPool::spawn(ActorPool::new(pool_size.unwrap_or(4), move || {
        ServerActor::spawn_with_mailbox(ServerActor::new(), mailbox::bounded(mailbox_size.unwrap_or(16)))
    }));

    pool.wait_for_startup_result().await?;
    Ok(pool)
}
