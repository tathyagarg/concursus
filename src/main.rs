use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{PeerId, identity};
use libp2p::{SwarmBuilder, mdns};
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let mdns = libp2p::mdns::tokio::Behaviour::new(libp2p::mdns::Config::default(), peer_id)?;
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            Default::default(),
            (libp2p::tls::Config::new, libp2p::noise::Config::new),
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| mdns)?
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {:?}", address);
                }
                SwarmEvent::Behaviour(mdns::Event::Discovered(list)) => {
                    for (peer_id, multiaddr) in list {
                        println!("Discovered peer {:?} at {:?}", peer_id, multiaddr);
                    }
                }
                SwarmEvent::Behaviour(mdns::Event::Expired(list)) => {
                    for (peer_id, multiaddr) in list {
                        println!("Expired peer {:?} at {:?}", peer_id, multiaddr);
                    }
                }
                _ => {}
            }
        }
    }
}
