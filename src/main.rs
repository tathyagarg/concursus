use libp2p::{
    Multiaddr, PeerId, Swarm, Transport, identity, mdns, noise,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp, yamux,
};
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Local identity
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {peer_id}");

    // Transport stack: TCP + Noise + Yamux
    let transport = libp2p::tokio_development_transport(id_keys.clone())?;

    // mDNS for LAN discovery
    let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;
    let mut swarm = SwarmBuilder::with_tokio_executor(transport, mdns, peer_id).build();

    // stdin task for sending messages
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    tokio::spawn(async move {
        while let Some(Ok(line)) = stdin.next_line().await {
            println!("(not implemented yet) Would send: {line}");
        }
    });

    // Event loop
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(mdns::Event::Discovered(peers)) => {
                for (peer, _) in peers {
                    println!("Discovered: {peer}");
                }
            }
            SwarmEvent::Behaviour(mdns::Event::Expired(peers)) => {
                for (peer, _) in peers {
                    println!("Expired: {peer}");
                }
            }
            _ => {}
        }
    }
}
