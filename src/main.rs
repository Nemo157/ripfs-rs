extern crate maddr;
extern crate libp2p;
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate msgio;
extern crate bytes;

mod identity;

use std::str::FromStr;
use maddr::{MultiAddr, Segment};
use libp2p::{ PeerInfo, Swarm };
use libp2p::identity::HostId;
use tokio_core::reactor::Core;
use futures::{Future, Sink, Stream};
use libp2p::identity::PeerId;
use tokio_io::codec::Framed;

use identity::Identity;

fn main() {
    let host = HostId::from_der(
        include_bytes!("private_key.der").as_ref().to_owned(),
        include_bytes!("public_key.der").as_ref().to_owned()).unwrap();

    println!("host: {:?}", host);

    let arg = std::env::args().skip(1).next();
    println!("arg: {:?}", arg);

    let mut core = Core::new().unwrap();

    fn ping(mut core: Core, mut swarm: Swarm, peer: PeerInfo) {
        println!("going to ping {:?}", peer);
        core.run(swarm.add_peers(vec![peer.clone()])).unwrap();
        println!("added peer");
        let id = peer.id().clone();
        let parts = core.run(swarm.open_stream(id, b"/ipfs/ping/1.0.0")).unwrap();
        println!("opened stream");
        let stream = Framed::from_parts(parts, Identity);
        let stream = core.run(stream.send(b"1234567890ABCDEF1234567890ABCDEF"[..].into())).unwrap();
        println!("sent ping");
        let (result, stream) = core.run(stream.into_future()).unwrap();
        println!("ping result {:?}", result.map(|b| String::from_utf8(b.to_vec())));
    }

    if arg == Some("listen".to_owned()) {
        let addr = MultiAddr::from_str("/ip4/127.0.0.1/tcp/4002").unwrap();
        println!("listening at: {}", addr.clone() + Segment::Ipfs(host.hash().clone()));
        let swarm = Swarm::new(host, "ripfs/0.0.0".to_owned(), vec![addr], core.handle()).unwrap();
        println!("result: {:?}", core.run(swarm));
    } else if arg == Some("self".to_owned()) {
        let swarm = Swarm::new(host, "ripfs/0.0.0".to_owned(), vec![], core.handle()).unwrap();
        core.handle().spawn(swarm.clone().map_err(|err| println!("Swarm error {:?}", err)));
        let addr = MultiAddr::from_str("/ip4/127.0.0.1/tcp/4002/ipfs/QmdxDB3FVu9JoUtSgaVTaCQpswH7ghfRsM3wGYWFSqW7Gb").unwrap();
        let peer = PeerInfo::from_addr(addr).unwrap();
        ping(core, swarm, peer);
    } else {
        let swarm = Swarm::new(host, "ripfs/0.0.0".to_owned(), vec![], core.handle()).unwrap();
        core.handle().spawn(swarm.clone().map_err(|err| println!("Swarm error {:?}", err)));
        let addr = MultiAddr::from_str("/ip4/127.0.0.1/tcp/4001/ipfs/QmcD3Pzo3kwvuZYNcxwEbefhmhR8s2ftd7zMkAWBwMhjax").unwrap();
        let peer = PeerInfo::from_addr(addr).unwrap();
        ping(core, swarm, peer);
    }
}
