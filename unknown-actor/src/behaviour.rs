use kameo::remote;
use libp2p::{Swarm, kad, mdns, noise, swarm::NetworkBehaviour, tcp, uds, yamux};

#[derive(NetworkBehaviour)]
pub(crate) struct ActorNetBehaviour {
    kameo: remote::Behaviour,
    mdns: mdns::tokio::Behaviour,
    kademlia: kad::Behaviour<kad::store::MemoryStore>,
}

pub(crate) type SwarmBehavior = Swarm<ActorNetBehaviour>;
pub(crate) fn behavior() -> Result<SwarmBehavior, Box<dyn std::error::Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            let peer_id = key.public().to_peer_id();
            let kameo = remote::Behaviour::new(
                key.public().to_peer_id(),
                remote::messaging::Config::default(),
            );
            let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;

            let kademlia = kad::Behaviour::new(peer_id, kad::store::MemoryStore::new(peer_id));
            Ok(ActorNetBehaviour {
                kameo,
                mdns,
                kademlia,
            })
        })?
        .build();

    swarm.behaviour().kameo.init_global();
    Ok(swarm)
}
