use libp2p::SwarmBuilder;
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{PeerId, identity};

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
        .with_behaviour(|_| libp2p::swarm::dummy::Behaviour)?
        .build();

    swarm.listen_on("/ip4/127.0.0.1/tcp/8080".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
            }
            SwarmEvent::Behaviour(event) => {
                println!("Behaviour event: {:?}", event);
            }
            other => {
                // Handle other SwarmEvent
                println!("Swarm event: {:?}", other);
            }
        }
    }
}
