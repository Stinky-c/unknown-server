#[cfg(feature = "server")]
mod behaviour;

use kameo::prelude::*;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use unknown_actor_lib::message::Shutdown;
use unknown_actor_lib::prelude::*;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fmt_layer = {
        let format = tracing_subscriber::fmt::format().with_source_location(false);

        tracing_subscriber::fmt::layer().event_format(format)
    };
    let filter_layer =
        { EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("debug"))? };
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter_layer)
        .init();

    // let swarm = behavior()?;
    // let peer_id = swarm.local_peer_id();

    let peer_id = remote::bootstrap()?;
    info!("bootstrapped at {:?}", peer_id);

    let actor = ServerActor::spawn_with_mailbox(ServerActor, mailbox::bounded(16));

    actor.register(NAME).await?;
    actor.tell("Hello World!".to_string()).await?;
    tokio::signal::ctrl_c().await?;
    actor.tell(Shutdown).await?;
    actor.wait_for_shutdown().await;

    Ok(())
}
